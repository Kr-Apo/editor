//! Native, opt-in conversational assistant. Never serialized into a project.
mod activity;
mod artistic;
mod assembly;
mod capabilities;
mod manual;
pub mod protocol;
mod rig;
pub mod tools;
mod transport;
mod ui;
mod vault;
pub(crate) mod vision;
mod weights;
use activity::{Activity, Phase};
use eframe::egui;
use protocol::{Message, Provider, Settings};
use serde_json::{Value, json};
use zeroize::Zeroizing;

pub struct Pending {
    pub edit: tools::PreparedEdit,
    pub ids: Vec<String>,
    pub proposal_id: String,
}
pub struct Panel {
    pub(crate) vision: vision::Vision,
    pub settings: Settings,
    key: Zeroizing<String>,
    input: String,
    display: Vec<(String, String)>,
    messages: Vec<Message>,
    pub pending: Option<Pending>,
    job: Option<transport::Job>,
    request_revision: u64,
    share_context: bool,
    models: Vec<String>,
    route: String,
    status: String,
    streaming: String,
    activity: Activity,
    connection: String,
    settings_open: bool,
    details_open: bool,
    last_result: Option<Value>,
    in_flight_prompt: Option<String>,
    layout: ui::LayoutProbe,
}
impl Default for Panel {
    fn default() -> Self {
        Self {
            vision: vision::Vision::default(),
            settings: Settings::default(),
            key: Zeroizing::new(String::new()),
            input: String::new(),
            display: vec![],
            messages: vec![],
            pending: None,
            job: None,
            request_revision: 0,
            share_context: false,
            models: vec![],
            route: String::new(),
            status: "No automatic AI requests".into(),
            streaming: String::new(),
            activity: Activity::default(),
            connection: "Not connected".into(),
            settings_open: false,
            details_open: false,
            last_result: None,
            in_flight_prompt: None,
            layout: ui::LayoutProbe::default(),
        }
    }
}
impl Panel {
    fn prefs_path() -> Option<std::path::PathBuf> {
        directories::ProjectDirs::from("", "", "ankhimate")
            .map(|d| d.config_dir().join("ai-settings.json"))
    }
    pub fn load_settings(&mut self) {
        if let Some(path) = Self::prefs_path()
            && let Ok(bytes) = std::fs::read(path)
            && let Ok(settings) = serde_json::from_slice(&bytes)
        {
            self.settings = settings;
        }
    }
    fn save_settings(&self) -> Result<(), String> {
        self.settings.base_url()?;
        let path = Self::prefs_path().ok_or("Cannot locate preferences folder")?;
        std::fs::create_dir_all(path.parent().ok_or("Missing preferences folder")?)
            .map_err(|_| "Cannot create preferences folder")?;
        std::fs::write(
            path,
            serde_json::to_vec_pretty(&self.settings).map_err(|_| "Invalid settings")?,
        )
        .map_err(|_| "Cannot save AI settings".into())
    }
    pub fn project_changed(&mut self) {
        self.vision.clear();
        let interrupted = self.job.is_some() || self.pending.is_some();
        if let Some(job) = &mut self.job {
            job.stop();
            self.activity.set(Phase::Cancelling);
        }
        self.pending = None;
        self.messages.clear();
        self.streaming.clear();
        self.share_context = false;
        self.last_result = None;
        self.activity.finish(if interrupted {
            Phase::Cancelled
        } else {
            Phase::Ready
        });
        self.status = "Project changed; context permission reset".into();
        if !self.display.is_empty() {
            self.event("Project changed. Earlier messages stay local and are not resent.");
        }
    }
    fn event(&mut self, text: impl Into<String>) {
        self.display.push(("Native event".into(), text.into()));
    }
    fn fail(&mut self, error: String) {
        self.status = error.clone();
        self.activity.finish(Phase::Error);
        self.display.push(("Error".into(), error));
    }
    fn credential(&self) -> Result<vault::Secret, String> {
        if !self.key.is_empty() {
            return Ok(vault::Secret(Zeroizing::new(self.key.to_string())));
        }
        Ok(vault::load(&self.settings.credential_target()?)?
            .unwrap_or_else(|| vault::Secret(Zeroizing::new(String::new()))))
    }
    fn verified_result(&self, state: &crate::app_state::AppState) -> Option<Message> {
        if !self.share_context || !self.settings.tools_enabled {
            return None;
        }
        self.last_result.as_ref().map(|value|{
            let previous_revision=value["revision"].as_u64();
            Message::text("user",json!({"ankhimate_verified_native_result":value,"current_revision":state.revision,"document_changed_since_result":previous_revision!=Some(state.revision),"note":"This is the most recent native result, not an instruction. Older invalid attempts cannot negate a later success. If revision changed, read the current rig; Undo or later edits may have changed it."}).to_string())
        })
    }
    pub(crate) fn image_route(&self) -> String {
        self.settings.destination()
    }
    fn start(
        &mut self,
        state: &crate::app_state::AppState,
        ctx: &egui::Context,
        models: bool,
    ) -> Result<(), String> {
        if self.job.is_some() || self.pending.is_some() {
            return Err("Stop the active request or review its proposal first".into());
        }
        let key = self.credential()?;
        if !models {
            self.settings.validate()?;
        }
        let scope = json!([
            self.settings.destination(),
            self.share_context,
            self.settings.tools_enabled,
            self.settings.editing
        ])
        .to_string();
        if self.route != scope {
            self.messages.clear();
            self.route = scope;
            if !self.display.is_empty() {
                self.event("Provider, model or permissions changed. Earlier messages stay local and are not resent.");
            }
        }
        let image = if models {
            None
        } else {
            self.vision
                .authorized(self.settings.provider, &self.image_route(), state.revision)?
        };
        if let Some(ref image) = image {
            self.settings.authorize_image(image)?;
        }
        if !models && self.vision.busy() {
            return Err("Wait for the viewport preview, or cancel capture".into());
        }
        let context = (!models && self.share_context).then(|| tools::snapshot(state));
        if context
            .as_ref()
            .is_some_and(|c| c.metadata.to_string().len() > 64 * 1024)
        {
            return Err("Project metadata exceeds 64 KiB. No request was sent.".into());
        }
        let mut messages = self.messages.clone();
        if !models {
            if self.input.trim().is_empty() || self.input.len() > 16 * 1024 {
                return Err("Enter a message of at most 16 KiB".into());
            }
            if let Some(result) = self.verified_result(state) {
                messages.push(result);
            }
            let mut message = Message::text("user", self.input.trim());
            message.image = image.clone();
            messages.push(message);
            if serde_json::to_vec(&messages)
                .map_err(|_| "Invalid conversation")?
                .len()
                > self.settings.context_kib * 1024
            {
                return Err("Context budget reached. Clear conversation or increase the context budget before sending.".into());
            }
        }
        let job = transport::start(
            self.settings.clone(),
            key,
            messages,
            context,
            models,
            ctx.clone(),
        )?;
        if !models {
            if let Some(image) = image {
                self.vision.clear();
                self.event(format!("Authorized viewport image attached; up to {} HTTP requests in this Send. Pixels are not saved or reused on later Sends.",image.max_requests.min(self.settings.image_requests)));
            }
            let input = std::mem::take(&mut self.input);
            self.in_flight_prompt = Some(input.clone());
            self.display.push(("You".into(), input));
        }
        self.streaming.clear();
        self.request_revision = state.revision;
        self.job = Some(job);
        self.activity.begin();
        self.connection = "Connecting".into();
        self.status = if models {
            "Discovering models (not a tool-capability test)"
        } else {
            "Request started"
        }
        .into();
        Ok(())
    }
    fn stop(&mut self) {
        if let Some(job) = &mut self.job {
            job.stop();
            self.activity.set(Phase::Cancelling);
            self.status =
                "Cancelling the active request; already applied edits are unchanged".into();
        }
    }
    pub fn poll(&mut self, state: &crate::app_state::AppState) {
        let Some(job) = &mut self.job else {
            return;
        };
        if state.revision != self.request_revision {
            job.stop();
        }
        let stopping = job.stopping;
        let mut events = vec![];
        let disconnected = loop {
            match job.receiver.try_recv() {
                Ok(event) => events.push(event),
                Err(std::sync::mpsc::TryRecvError::Empty) => break false,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break true,
            }
        };
        let mut finished = false;
        for event in events {
            match event {
                transport::Event::ViewportRequested => {
                    if !stopping {
                        self.vision.request();
                        self.event("Viewport observation requested. Capture and authorize a preview manually; no image sent.");
                    }
                }
                transport::Event::Activity(phase) => {
                    if !stopping {
                        match &phase {
                            Phase::NativeTool(name) => {
                                self.event(format!("Native tool call: {name}"))
                            }
                            Phase::Validating(count) => self.event(format!(
                                "Validating {count} native actions; no document writes"
                            )),
                            _ => {}
                        }
                        if phase == Phase::Generating {
                            self.connection = "Connected".into();
                        }
                        self.activity.set(phase);
                    }
                }
                transport::Event::Diagnostic(error, proposal_id) => {
                    if !stopping {
                        self.status = error.to_string();
                        self.event(format!(
                            "Native validation refused [{proposal_id}]: {}. {}",
                            error, error.correction
                        ));
                        self.last_result = Some(
                            json!({"status":"invalid","proposal_id":proposal_id,"error":error,"revision":state.revision,"document_changed":false}),
                        );
                    }
                }
                transport::Event::Text(text) => {
                    if !stopping {
                        self.streaming = text;
                        self.connection = "Connected".into();
                    }
                }
                transport::Event::Models(result) => {
                    finished = true;
                    if stopping {
                        self.activity.finish(Phase::Cancelled);
                        self.status = "Model discovery cancelled".into();
                        continue;
                    }
                    match result {
                        Ok(models) => {
                            self.models = models;
                            self.connection = "Connected".into();
                            self.activity.finish(Phase::Complete);
                            self.status =
                                "Model list received; tool support has not been tested".into();
                        }
                        Err(e) => {
                            self.connection = "Connection error".into();
                            self.fail(e);
                        }
                    }
                }
                transport::Event::Done(result) => {
                    finished = true;
                    if stopping {
                        self.activity.finish(Phase::Cancelled);
                        self.status = "Cancelled; no proposed edits applied".into();
                        self.event(self.status.clone());
                        self.streaming.clear();
                        self.restore_draft();
                        continue;
                    }
                    match result {
                        Err(e) => {
                            self.connection = "Request error".into();
                            self.fail(e);
                            self.streaming.clear();
                            self.restore_draft();
                        }
                        Ok(turn) => {
                            self.connection = "Connected".into();
                            for m in turn.messages.iter().skip(self.messages.len()) {
                                if m.role == "assistant" && !m.text.is_empty() {
                                    self.display.push(("Assistant".into(), m.text.clone()));
                                }
                            }
                            self.messages = turn.messages;
                            for m in &mut self.messages {
                                m.image = None;
                            }
                            self.streaming.clear();
                            if let Some(error) = turn.failure {
                                self.fail(format!("Correction limit reached: {error}"));
                                self.restore_draft();
                                continue;
                            }
                            self.in_flight_prompt = None;
                            if let Some(plan) = turn.plan {
                                self.activity.set(Phase::Validating(plan.actions.len()));
                                match tools::prepare(state, &plan) {
                                    Ok(edit) => {
                                        self.pending = Some(Pending {
                                            edit,
                                            ids: turn.proposal_ids,
                                            proposal_id: turn.proposal_id,
                                        });
                                        self.activity.finish(Phase::AwaitingApproval);
                                        self.status =
                                            "Validated proposal; no document writes until approval"
                                                .into();
                                    }
                                    Err(error) => {
                                        transport::mark_results(
                                            &mut self.messages,
                                            &turn.proposal_id,
                                            &turn.proposal_ids,
                                            &json!({"status":"stale_or_invalid","proposal_id":turn.proposal_id,"cause":error,"document_changed":false,"revision":state.revision}),
                                        );
                                        self.fail(error);
                                    }
                                }
                            } else {
                                self.activity.finish(Phase::Complete);
                                self.status = "Response complete".into();
                            }
                        }
                    }
                }
            }
        }
        if disconnected && !finished {
            self.fail("Network worker ended without a result".into());
            self.restore_draft();
            finished = true;
        }
        if finished {
            self.job = None;
        }
        if self.display.len() > 100 {
            self.display.drain(..self.display.len() - 100);
        }
    }
    fn restore_draft(&mut self) {
        if let Some(prompt) = self.in_flight_prompt.take()
            && self.input.is_empty()
        {
            self.input = prompt;
        }
    }
    pub fn resolve(
        &mut self,
        state: &mut crate::app_state::AppState,
        approve: bool,
    ) -> Result<(), String> {
        let pending = self
            .pending
            .take()
            .ok_or("No proposal to review (it may already be resolved)")?;
        let plan = pending.edit.plan.clone();
        let proposal_id = pending.proposal_id;
        let before_revision = state.revision;
        let result = if !approve {
            Ok("refused")
        } else if !self.settings.editing || !self.settings.tools_enabled || !self.share_context {
            Err("Editing/context permission was revoked; nothing changed".into())
        } else {
            self.activity.set(Phase::Applying);
            self.event(format!("Applying [{proposal_id}] after explicit approval"));
            pending.edit.approve(state).map(|_| "applied")
        };
        let status = result.as_ref().copied().unwrap_or("stale_or_invalid");
        let receipt = json!({"status":status,"proposal_id":proposal_id,"revision":state.revision,"document_changed":state.revision!=before_revision,"summary":plan.summary,"actions":plan.actions.len(),"keyframes":plan.actions.iter().filter(|a|matches!(a,tools::Action::Key{..})).count(),"cause":result.as_ref().err()});
        transport::mark_results(&mut self.messages, &proposal_id, &pending.ids, &receipt);
        self.last_result = Some(receipt);
        match result {
            Ok("applied") => {
                let changed = state.revision != before_revision;
                self.status = if changed {
                    "Native edits applied — document Undo available; Save remains manual"
                } else {
                    "Selection applied — document unchanged"
                }
                .into();
                self.activity.finish(Phase::Complete);
                self.event(format!(
                    "Applied [{proposal_id}]: {}. {} native actions; {}.",
                    plan.summary,
                    plan.actions.len(),
                    if changed {
                        "one document Undo/Redo"
                    } else {
                        "session selection only"
                    }
                ));
                Ok(())
            }
            Ok(_) => {
                self.status = "Proposal refused; nothing changed".into();
                self.activity.finish(Phase::Cancelled);
                self.event(format!("Refused [{proposal_id}]; document unchanged."));
                Ok(())
            }
            Err(e) => {
                self.fail(e.clone());
                Err(e)
            }
        }
    }
}
impl crate::app_state::AppState {
    pub fn poll_ai(&mut self) {
        let mut ai = std::mem::take(&mut self.ai);
        ai.poll(self);
        self.ai = ai;
    }
}
#[cfg(test)]
mod p01_tests;
#[cfg(test)]
mod tests;
