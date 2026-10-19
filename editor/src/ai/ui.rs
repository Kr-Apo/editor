//! Docked and detached views share one bounded layout and the same session state.
use super::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

pub(super) struct LayoutProbe {
    pub panel: egui::Rect,
    pub composer: egui::Rect,
    pub send: egui::Rect,
    pub stop: egui::Rect,
    pub conversation: egui::Rect,
    pub scroll_offset: f32,
    #[cfg(test)]
    pub send_id: Option<egui::Id>,
    #[cfg(test)]
    pub stop_id: Option<egui::Id>,
}
impl Default for LayoutProbe {
    fn default() -> Self {
        Self {
            panel: egui::Rect::NOTHING,
            composer: egui::Rect::NOTHING,
            send: egui::Rect::NOTHING,
            stop: egui::Rect::NOTHING,
            conversation: egui::Rect::NOTHING,
            scroll_offset: 0.0,
            #[cfg(test)]
            send_id: None,
            #[cfg(test)]
            stop_id: None,
        }
    }
}

fn toolbar(ui: &mut egui::Ui, name: &str, contents: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        let id = egui::Id::new((name, ui.ctx().viewport_id()));
        ui.scope_builder(egui::UiBuilder::new().id(id), contents);
    });
}

impl Panel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut crate::app_state::AppState) {
        self.layout.panel = ui.available_rect_before_wrap();
        let compact = self.layout.panel.height() < 300.0;
        let phase = if self.job.is_some() && matches!(self.activity.phase, Phase::Complete) {
            Phase::Generating
        } else {
            self.activity.phase.clone()
        };
        let color = match phase {
            Phase::Error => ui.visuals().error_fg_color,
            Phase::AwaitingApproval => ui.visuals().warn_fg_color,
            Phase::Complete => egui::Color32::from_rgb(84, 189, 125),
            _ => ui.visuals().text_color(),
        };
        let activity = if self.activity.elapsed().as_secs() > 0 {
            format!(
                "{} · {} s",
                phase.label(),
                self.activity.elapsed().as_secs()
            )
        } else {
            phase.label()
        };
        let permissions =
            if self.share_context && self.settings.tools_enabled && self.settings.editing {
                "Edit + approval"
            } else {
                "Read only"
            };
        let model = if self.settings.model.is_empty() {
            "Choose a model"
        } else {
            &self.settings.model
        };
        let applied = self
            .last_result
            .as_ref()
            .is_some_and(|r| r["status"] == "applied");
        ui.horizontal(|ui| {
            ui.strong(format!("{} AI Assistant", crate::ui::icons::PLUGIN));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Settings").clicked() {
                    self.settings_open = !self.settings_open;
                }
                if compact {
                    ui.add(
                        egui::Label::new(egui::RichText::new(&activity).small().color(color))
                            .truncate(),
                    )
                    .on_hover_text(format!(
                        "{}\n{}\n{}",
                        activity, self.connection, self.status
                    ));
                }
            });
        });
        if compact {
            let summary = format!(
                "{} · {model} · {permissions}{}",
                self.settings.provider.label(),
                if applied { " · Last edit applied" } else { "" }
            );
            ui.add(egui::Label::new(egui::RichText::new(&summary).small()).truncate())
                .on_hover_text(summary);
        } else {
            ui.horizontal_wrapped(|ui| {
                ui.small(format!("{} · {model}", self.settings.provider.label()));
                ui.small(permissions);
            });
            ui.colored_label(color, &activity)
                .on_hover_text(format!("{}\n{}", self.connection, self.status));
            if let Some(result) = &self.last_result
                && applied
            {
                let current = result["revision"].as_u64() == Some(state.revision);
                ui.colored_label(
                    egui::Color32::from_rgb(84, 189, 125),
                    if current {
                        "Native result: applied"
                    } else {
                        "Last native result: applied; document changed since"
                    },
                )
                .on_hover_text(result.to_string());
            }
        }
        if self.job.is_some() {
            ui.ctx().request_repaint_after(Duration::from_secs(1));
        }
        ui.separator();
        let remaining = ui.available_rect_before_wrap();
        let show_send = !(compact && self.pending.is_some());
        let rows = if self.pending.is_some() {
            34.0_f32
        } else {
            0.0
        } + if show_send { 34.0 } else { 0.0 };
        let input_height = (remaining.height() - rows - 20.0).clamp(20.0, 48.0);
        let footer_height = (rows + input_height + 6.0).min(remaining.height());
        let body = egui::Rect::from_min_max(
            remaining.min,
            egui::pos2(
                remaining.max.x,
                (remaining.max.y - footer_height).max(remaining.min.y),
            ),
        );
        let footer =
            egui::Rect::from_min_max(egui::pos2(remaining.min.x, body.max.y), remaining.max);
        self.layout.composer = footer;
        self.layout.conversation = body;
        if body.height() > 1.0 {
            ui.scope_builder(egui::UiBuilder::new().id(egui::Id::new(("ai_conversation_region",ui.ctx().viewport_id()))).max_rect(body),|ui|{
                ui.set_clip_rect(body.intersect(ui.clip_rect()));
                let output=egui::ScrollArea::vertical().id_salt("ai_messages").auto_shrink([false,false]).max_height(body.height()).stick_to_bottom(true).show(ui,|ui|{
                    if self.display.is_empty(){ui.label("Ask about your rig or propose an animation. Configure a provider in Settings. Project sharing is opt-in.");}
                    for(index,(role,text))in self.display.iter().enumerate(){ui.push_id(index,|ui|{
                        let native=role=="Native event";let error=role=="Error";
                        let fill=if role=="You"{ui.visuals().selection.bg_fill.gamma_multiply(0.35)}else{ui.visuals().faint_bg_color};
                        egui::Frame::new().fill(fill).inner_margin(8.0).corner_radius(5.0).show(ui,|ui|{
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui|{ui.colored_label(if error{ui.visuals().error_fg_color}else if native{ui.visuals().warn_fg_color}else{ui.visuals().strong_text_color()},role);if ui.small_button("Copy").clicked(){ui.ctx().copy_text(text.clone());}});
                            ui.add(egui::Label::new(text).wrap().selectable(true));
                        });ui.add_space(5.0);
                    });}
                    if !self.streaming.is_empty(){ui.strong("Assistant · streaming");ui.add(egui::Label::new(&self.streaming).wrap().selectable(true));}
                    if let Some(pending)=&self.pending&&Self::proposal_card(ui,state,pending){self.details_open=true;}
                });
                self.layout.scroll_offset=output.state.offset.y;
            });
        }
        ui.scope_builder(
            egui::UiBuilder::new()
                .id(egui::Id::new(("ai_footer", ui.ctx().viewport_id())))
                .max_rect(footer),
            |ui| {
                ui.set_clip_rect(footer.intersect(ui.clip_rect()));
                if self.pending.is_some() {
                    toolbar(ui, "ai_approval_toolbar", |ui| {
                        if ui.button("Approve native edits").clicked()
                            && let Err(e) = self.resolve(state, true)
                        {
                            self.status = e;
                        }
                        if ui.button("Refuse changes").clicked() {
                            let _ = self.resolve(state, false);
                        }
                    });
                }
                let id = ui.make_persistent_id("ai_composer");
                let shortcut = ui.memory(|m| m.has_focus(id))
                    && ui.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Enter));
                egui::ScrollArea::vertical()
                    .id_salt("ai_draft_scroll")
                    .min_scrolled_height(20.0)
                    .max_height(input_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.input)
                                .id(id)
                                .hint_text("Ask about the rig…")
                                .desired_width(f32::INFINITY)
                                .desired_rows(2),
                        );
                    });
                if show_send {
                    toolbar(ui, "ai_send_toolbar", |ui| {
                        let enabled = self.job.is_none() && self.pending.is_none();
                        let send = ui.add_enabled(enabled, egui::Button::new("Send · Ctrl+Enter"));
                        self.layout.send = send.rect;
                        #[cfg(test)]
                        {
                            self.layout.send_id = Some(send.id);
                        }
                        if (send.clicked() || (shortcut && enabled))
                            && let Err(e) = self.start(state, ui.ctx(), false)
                        {
                            self.fail(e);
                        }
                        let stop = ui.add_enabled(self.job.is_some(), egui::Button::new("Stop"));
                        self.layout.stop = stop.rect;
                        #[cfg(test)]
                        {
                            self.layout.stop_id = Some(stop.id);
                        }
                        if stop.clicked() {self.stop();}
                        if ui.add_enabled(self.job.is_none()&&self.pending.is_none(),egui::Button::new(if self.vision.preview.is_some(){"Image…"}else{"View…"})).on_hover_text("Capture / review viewport image. No transmission until authorized Send.").clicked(){self.vision.request();}
                    });
                } else {
                    self.layout.send = egui::Rect::NOTHING;
                    self.layout.stop = egui::Rect::NOTHING;
                }
            },
        );
        ui.advance_cursor_after_rect(remaining);
        self.settings_window(ui.ctx(), state);
        self.vision_window(ui.ctx(), state);
        if self.details_open {
            let mut open = true;
            egui::Window::new("Native proposal details")
                .id(egui::Id::new("ai_proposal_details"))
                .open(&mut open)
                .vscroll(true)
                .default_width(520.0)
                .max_height(ui.ctx().content_rect().height() - 40.0)
                .show(ui.ctx(), |ui| {
                    if let Some(pending) = &self.pending {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(
                                    serde_json::to_string_pretty(&pending.edit.plan)
                                        .unwrap_or_default(),
                                )
                                .monospace(),
                            )
                            .wrap()
                            .selectable(true),
                        );
                    } else {
                        ui.label(
                            "Proposal resolved; see the native result in conversation history.",
                        );
                    }
                });
            self.details_open = open;
        }
    }
    fn proposal_card(
        ui: &mut egui::Ui,
        state: &crate::app_state::AppState,
        pending: &Pending,
    ) -> bool {
        let mut clips = BTreeMap::new();
        let mut bones = BTreeSet::new();
        let mut channels = BTreeSet::new();
        let mut keys = 0;
        for action in &pending.edit.plan.actions {
            match action {
                tools::Action::Create {
                    name,
                    duration,
                    looping,
                } => {
                    clips.insert(name.clone(), (*duration, *looping));
                }
                tools::Action::Meta {
                    animation,
                    duration,
                    looping,
                } => {
                    clips.insert(animation.clone(), (*duration, *looping));
                }
                tools::Action::Key {
                    animation,
                    bone,
                    channel,
                    ..
                } => {
                    keys += 1;
                    bones.insert(bone.clone());
                    channels.insert(
                        serde_json::to_value(channel)
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    );
                    if !clips.contains_key(animation)
                        && let Some(a) =
                            state.doc.animations.values().find(|a| a.name == *animation)
                    {
                        clips.insert(animation.clone(), (a.duration, a.looping));
                    }
                }
                a if a.is_weights() => {
                    channels.insert(format!("{} (mesh weights / Setup)", a.operation()));
                }
                tools::Action::MeshSet { .. }
                | tools::Action::MeshNormalize { .. }
                | tools::Action::MeshSmooth { .. } => unreachable!(),
                a if a.is_assembly() => {
                    channels.insert(format!("{} (existing regions / Setup)", a.operation()));
                }
                tools::Action::RegionTransform { .. }
                | tools::Action::SlotBone { .. }
                | tools::Action::DrawOrder { .. } => unreachable!(),
                tools::Action::BoneCreate {
                    name,
                    parent,
                    position,
                    rotation_degrees,
                    length,
                } => {
                    bones.insert(name.clone());
                    channels.insert(format!(
                        "Create {name}, parent {}, local ({}, {}), {}°, length {}",
                        parent.as_deref().unwrap_or("ROOT"),
                        position[0],
                        position[1],
                        rotation_degrees,
                        length
                    ));
                }
                tools::Action::BoneSetup {
                    bone,
                    position,
                    rotation_degrees,
                    scale,
                } => {
                    bones.insert(bone.clone());
                    channels.insert(format!(
                        "Setup {bone}: local {:?}, {}°, scale {:?}",
                        position, rotation_degrees, scale
                    ));
                }
                tools::Action::BoneRename { bone, name } => {
                    bones.insert(bone.clone());
                    channels.insert(format!("Rename {bone} → {name} (stable references)"));
                }
                tools::Action::Select { animation } => {
                    if let Some(a) = state.doc.animations.values().find(|a| a.name == *animation) {
                        clips
                            .entry(animation.clone())
                            .or_insert((a.duration, a.looping));
                    }
                }
            }
        }
        egui::Frame::new()
            .stroke(egui::Stroke::new(1.0, ui.visuals().warn_fg_color))
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.strong("NATIVE EDIT PROPOSAL — not applied");
                ui.label(&pending.edit.plan.summary);
                for (name, (duration, looping)) in clips {
                    ui.label(format!(
                        "{name}: {duration} s · {}",
                        if looping { "loop" } else { "one-shot" }
                    ));
                }
                ui.label(format!(
                    "{} · {}",
                    if pending.edit.plan.actions.iter().any(tools::Action::is_setup){format!("{} Setup actions",pending.edit.plan.actions.len())}else{format!("{keys} keyframes")},
                    channels.into_iter().collect::<Vec<_>>().join(", ")
                ));
                for change in &pending.edit.review {
                    ui.group(|ui|{
                        ui.strong(change["op"].as_str().unwrap_or("Assembly"));
                        let before=&change["before"];let after=&change["after"];
                        if before.get("draw_order").is_some(){ui.label(format!("Before draw order: {}",before["draw_order"]));ui.label(format!("After draw order: {}",after["draw_order"]));}
                        else if before["type"]=="mesh" {
                            ui.label(format!("{} / {} / {} — weighted mesh",before["skin"],before["slot"],before["attachment"]));
                            ui.label(format!("{} selected vertices; existing influences preserved",before["affected_vertices"]));
                            ui.collapsing("Weight values before / after",|ui|{for (old,new) in before["vertices"].as_array().into_iter().flatten().zip(after["vertices"].as_array().into_iter().flatten()){ui.label(format!("Vertex {}: {} → {}",old["index"],old["influences"],new["influences"]));}});
                        }
                        else{ui.label(format!("{} / {} / {} — region",before["skin"].as_str().unwrap_or("?"),before["slot"].as_str().unwrap_or("?"),before["attachment"].as_str().unwrap_or("?")));ui.label(format!("Bone: {} → {}",before["bone"],after["bone"]));ui.label(format!("Before local transform: {}",before["local"]));ui.label(format!("After local transform: {}",after["local"]));ui.small(format!("Setup draw index: {} → {}",before["draw_index"],after["draw_index"]));}
                        ui.colored_label(ui.visuals().warn_fg_color,change["warning"].as_str().unwrap_or("Review Setup changes"));
                    });
                }
                if !bones.is_empty() {
                    ui.label(format!(
                        "Bones: {}",
                        bones.into_iter().collect::<Vec<_>>().join(", ")
                    ));
                }
                ui.small(format!(
                    "ID: {} · See action units above · Save stays manual",
                    pending.proposal_id
                ));
                if pending.edit.plan.actions.iter().any(tools::Action::is_setup){ui.colored_label(ui.visuals().warn_fg_color,"Setup only. Existing art/animations keep their references, but changing setup can change their appearance. Bone positions are parent-local; region positions are bone-local. Image pixels require conversion.");}
                ui.small("Approve / Refuse stays below the chat.");
                ui.button("Inspect proposed actions").clicked()
            })
            .inner
    }
    fn settings_window(&mut self, ctx: &egui::Context, state: &crate::app_state::AppState) {
        if !self.settings_open {
            return;
        }
        let previous_destination = self.image_route();
        let mut open = true;
        egui::Window::new("AI Settings").id(egui::Id::new("ai_settings")).open(&mut open).vscroll(true).default_width(460.0).max_height((ctx.content_rect().height()-48.0).max(120.0)).show(ctx,|ui|{
            ui.label(&self.connection);ui.label(&self.status);
            let busy=self.job.is_some();
            ui.add_enabled_ui(!busy&&self.pending.is_none(),|ui|{
                let old=self.settings.provider;
                egui::ComboBox::from_id_salt("ai_provider").selected_text(old.label()).show_ui(ui,|ui|{for provider in Provider::ALL{ui.selectable_value(&mut self.settings.provider,provider,provider.label());}});
                if self.settings.provider!=old{self.key.clear();self.models.clear();self.share_context=false;self.settings.local_url=self.settings.provider.default_url().into();self.connection="Not connected".into();}
                ui.label("Model ID (manual IDs are allowed)");ui.text_edit_singleline(&mut self.settings.model);
                if !self.models.is_empty(){egui::ComboBox::from_id_salt("ai_model_list").selected_text("Discovered models").show_ui(ui,|ui|{for model in &self.models{ui.selectable_value(&mut self.settings.model,model.clone(),model);}});}
                if !self.settings.provider.cloud(){ui.label("Loopback server URL");if ui.text_edit_singleline(&mut self.settings.local_url).changed(){self.key.clear();self.share_context=false;self.models.clear();self.connection="Not connected".into();}}
                if self.settings.provider==Provider::NInfer{ui.small("NInfer profile: local OpenAI-compatible HTTP. Backend and model capabilities depend on your server; discovering a model does not test tools.");}
                if ui.button("Fetch models / test connection").clicked()&&let Err(e)=self.start(state,ctx,true){self.fail(e);}
                ui.collapsing("API key (optional for local servers)",|ui|{
                    ui.add(egui::TextEdit::singleline(&mut *self.key).password(true));
                    ui.small("Session-only until Store is clicked. Never saved in a project or chat.");
                    if ui.button("Store key securely").clicked(){self.status=self.settings.credential_target().and_then(|target|vault::store(&target,&self.key)).map(|_|{self.key.clear();"Stored in Windows Credential Manager".into()}).unwrap_or_else(|e|e);}
                    if ui.button("Remove stored key").clicked(){self.status=self.settings.credential_target().and_then(|target|vault::remove(&target)).map(|_|{self.key.clear();"Stored key removed".into()}).unwrap_or_else(|e|e);}
                });
                ui.separator();ui.strong("Permissions");
                if ui.button("Viewport image / preview / consent…").clicked(){self.vision.request();}
                ui.small("Image transport: all six profiles. Model vision and tool capabilities require explicit qualification; model discovery sends no image.");
                ui.checkbox(&mut self.settings.tools_enabled,"Enable model tools");
                ui.add_enabled(self.settings.tools_enabled,egui::Checkbox::new(&mut self.settings.editing,"Allow native edits with explicit approval"));
                ui.checkbox(&mut self.share_context,if self.settings.provider.cloud(){"Allow project metadata to this cloud provider on Send"}else{"Allow project metadata to this local server on Send"});
                ui.small("Metadata permission is separate from per-image consent. No files, paths or automatic saves. Chat text is sent to the selected provider.");
                ui.collapsing("Exact metadata available to tools",|ui|{if self.share_context{ui.add(egui::Label::new(serde_json::to_string_pretty(&tools::context(state)).unwrap_or_default()).wrap().selectable(true));}else{ui.label("Metadata sharing is disabled.");}});
                ui.collapsing("Model capabilities (user declarations)",|ui|{
                    let mut caps=self.settings.capabilities();caps.destination=self.settings.destination();
                    ui.small("Applies only to this exact provider / endpoint / model. Discovery is not qualification. Unknown vision needs per-image permission.");
                    for (name,value) in [("Text",&mut caps.text),("Images",&mut caps.images),("Tools",&mut caps.tools),("Streaming",&mut caps.streaming),("Separate reasoning",&mut caps.reasoning)] {
                        egui::ComboBox::from_id_salt(("ai_caps",name)).selected_text(format!("{}: {}",name,value.label())).show_ui(ui,|ui|{for support in super::capabilities::Support::ALL {ui.selectable_value(value,support,support.label());}});
                    }
                    self.settings.capabilities=caps;
                });
                ui.collapsing("Request limits / profiles",|ui|{
                    ui.horizontal(|ui|{if ui.button("Small local model").clicked(){self.settings.profile(true);}
                    if ui.button("Balanced budgets").clicked(){self.settings.profile(false);}});
                    ui.small("Byte budgets are not a model token-window or 8 GB VRAM guarantee. Required tool schemas are always retained; reduce context if the server rejects it.");
                    ui.horizontal(|ui|{ui.label("Context (KiB, excluding PNG base64)");ui.add(egui::DragValue::new(&mut self.settings.context_kib).range(32..=2048));});
                    ui.horizontal(|ui|{ui.label("Embedded manual bytes");ui.add(egui::DragValue::new(&mut self.settings.manual_bytes).range(2048..=manual::MAX_BYTES));});
                    ui.horizontal(|ui|{ui.label("HTTP requests carrying this image / Send");ui.add(egui::DragValue::new(&mut self.settings.image_requests).range(1..=17));});
                    egui::ComboBox::from_id_salt("ai_image_quality").selected_text(self.settings.image_quality.label()).show_ui(ui,|ui|{for q in vision::Quality::ALL {ui.selectable_value(&mut self.settings.image_quality,q,q.label());}});
                    ui.horizontal(|ui|{ui.label("Timeout (s)");ui.add(egui::DragValue::new(&mut self.settings.timeout_seconds).range(1..=600));});
                    ui.horizontal(|ui|{ui.label("Output tokens / response");ui.add(egui::DragValue::new(&mut self.settings.max_tokens).range(128..=32768));});
                    ui.horizontal(|ui|{ui.label("Tool calls / Send");ui.add(egui::DragValue::new(&mut self.settings.tool_budget).range(1..=16));});
                    ui.small("At most two invalid-proposal correction rounds inside this budget. No automatic retry after approval.");
                });
                if ui.button("Save non-secret settings").clicked(){self.status=self.save_settings().map(|_|"Settings saved without keys".into()).unwrap_or_else(|e|e);}
                if ui.button("Clear conversation").clicked(){self.messages.clear();self.display.clear();self.streaming.clear();}
            });
            if self.pending.is_some()&&ui.button("Pending proposal details").clicked(){self.details_open=true;}
            ui.collapsing("Diagnostics and embedded manual",|ui|{
                let guide=manual::select_with_budget(&self.input,self.settings.tools_enabled&&self.share_context,self.settings.manual_bytes);
                ui.label(format!("{} · draft sections: {} · {} / {} bytes",manual::VERSION,guide.sections.join(", "),guide.text.len(),manual::MAX_BYTES));
                ui.small("Selection is deterministic. Current user prompt selects public manual sections; project data never supplies instructions. No internal reasoning state is inferred.");
                if let Some(result)=&self.last_result{ui.add(egui::Label::new(serde_json::to_string_pretty(result).unwrap_or_default()).wrap().selectable(true));}
            });
        });
        if previous_destination != self.image_route() {
            self.vision.clear();
        }
        self.settings_open = open;
    }
}

impl Panel {
    fn vision_window(&mut self, ctx: &egui::Context, state: &crate::app_state::AppState) {
        if !self.vision.open {
            return;
        }
        let route = self.image_route();
        let mut capture_started = false;
        let mut open = true;
        egui::Window::new("Viewport image — review before Send").id(egui::Id::new("ai_vision")).open(&mut open).default_width(510.0).max_height((ctx.content_rect().height()-40.0).max(160.0)).vscroll(true).show(ctx,|ui|{
        ui.label(format!("Destination: {} · {} · {}",self.settings.provider.label(),self.settings.model,self.settings.base_url().unwrap_or_else(|_|"Invalid endpoint".into())));
        ui.small("Only the Ankhimate canvas. Image permission is separate from rig metadata. May include bones, overlays or an editor popup over the canvas: inspect the preview. No desktop, file upload or persistent image storage.");
        let image_cap=self.settings.capabilities().images;
        ui.label(format!("Model images: {}",image_cap.label()));
        ui.horizontal(|ui|{ui.label("Next capture quality");egui::ComboBox::from_id_salt("preview_quality").selected_text(self.settings.image_quality.label()).show_ui(ui,|ui|{for q in vision::Quality::ALL {ui.selectable_value(&mut self.settings.image_quality,q,q.label());}});});
        ui.small("Quality changes require a new capture. Perception differs between resolutions.");
        if let Some(e)=&self.vision.error{ui.colored_label(ui.visuals().error_fg_color,e);}
        if ui.add_enabled(!self.vision.busy()&&image_cap!=super::capabilities::Support::Unsupported&&self.job.is_none(),egui::Button::new("Capture viewport preview (no transmission)")).clicked(){
            self.settings_open=false;self.details_open=false;self.vision.quality=self.settings.image_quality;self.vision.begin_capture(ctx);capture_started=true;
        }
        if self.vision.busy(){ui.label("Waiting for the visible viewport render target…");if ui.button("Cancel capture").clicked(){self.vision.cancel_capture();}}
        if let Some(p)=&mut self.vision.preview {
            if p.texture.is_none(){p.texture=Some(ctx.load_texture("ai_viewport_preview",p.image.clone(),egui::TextureOptions::LINEAR));}
            ui.add(egui::Image::new(p.texture.as_ref().unwrap()).max_width(ui.available_width()).max_height(280.0).maintain_aspect_ratio(true));
            ui.small(format!("{} × {} · PNG · {} base64 bytes · revision {} · expires after 2 min",p.image.size[0],p.image.size[1],p.input.base64().len(),p.revision));
            if image_cap==super::capabilities::Support::Unknown && ui.checkbox(&mut p.input.try_unknown,"Attempt this image with an untested model (may reject or ignore it)").changed(){p.approved=false;}
            let max=self.settings.image_requests;
            if p.input.max_requests!=max {p.input.max_requests=max;p.approved=false;}
            ui.small(format!("Stateless continuation may resend this PNG: up to {max} HTTP requests in this Send, within the tool budget. No later Send reuses it; no automatic retry on refusal."));
            ui.collapsing("Image-to-world coordinates and limitations",|ui|{ui.label(p.input.mapping.to_string());});
            let fresh=p.route==route&&p.revision==state.revision&&p.created.elapsed().as_secs()<=120;
            let valid=fresh&&image_cap!=super::capabilities::Support::Unsupported&&(image_cap!=super::capabilities::Support::Unknown||p.input.try_unknown);
            if !valid{p.approved=false;ui.colored_label(ui.visuals().warn_fg_color,if !fresh {"Stale permission: capture again for this destination/document."} else {"Choose a compatible model or explicitly permit the untested attempt above."});}
            ui.add_enabled(valid,egui::Checkbox::new(&mut p.approved,"Authorize THIS image for the next explicit Send and its bounded tool rounds"));
            ui.small("Close this preview, enter your request, then Send. Later Sends will not reuse the image. A server listing models does not certify vision; a text-only model may reject the image.");
            if ui.button("Remove image without sending").clicked(){self.vision.clear();}
        }
    });
        self.vision.open = open && !capture_started;
    }
}
