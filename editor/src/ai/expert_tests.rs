//! T-805.1: behavioral regressions over real HTTP mocks and real egui events.
use super::*;

#[test]
fn send_stop_widget_identity_survives_proposals_and_native_result_banners() {
    let mut s = state();
    let mut p = Panel::default();
    let ctx = egui::Context::default();
    let size = egui::vec2(430.0, 600.0);
    for _ in 0..3 {
        sized_frame(&ctx, &mut p, &mut s, size, vec![]);
    }
    let send = p.layout.send_id;
    let stop = p.layout.stop_id;
    p.pending = Some(Pending {
        edit: tools::prepare(&s, &breathing("torso")).unwrap(),
        ids: vec![],
        proposal_id: "identity".into(),
    });
    sized_frame(&ctx, &mut p, &mut s, size, vec![]);
    assert_eq!(
        send, p.layout.send_id,
        "Adding approval controls changed Send's identity"
    );
    assert_eq!(stop, p.layout.stop_id);
    p.pending = None;
    p.last_result = Some(json!({"status":"applied","revision":s.revision}));
    sized_frame(&ctx, &mut p, &mut s, size, vec![]);
    assert_eq!(send, p.layout.send_id);
    assert_eq!(stop, p.layout.stop_id);
}

#[test]
fn opening_a_project_is_ready_unless_it_really_interrupts_a_proposal() {
    let s = state();
    let mut p = Panel::default();
    p.project_changed();
    assert_eq!(p.activity.phase, Phase::Ready);
    assert!(p.job.is_none());
    p.pending = Some(Pending {
        edit: tools::prepare(&s, &breathing("torso")).unwrap(),
        ids: vec![],
        proposal_id: "interrupted".into(),
    });
    p.project_changed();
    assert_eq!(p.activity.phase, Phase::Cancelled);
    assert!(p.pending.is_none());
}

#[test]
fn long_drafts_keep_send_and_stop_at_the_same_fixed_position() {
    let mut s = state();
    let mut p = Panel::default();
    let ctx = egui::Context::default();
    let size = egui::vec2(430.0, 600.0);
    for _ in 0..3 {
        sized_frame(&ctx, &mut p, &mut s, size, vec![]);
    }
    let send = p.layout.send;
    let stop = p.layout.stop;
    p.input = "A longer multiline draft that must scroll inside its own fixed area.\n".repeat(100);
    for _ in 0..3 {
        sized_frame(&ctx, &mut p, &mut s, size, vec![]);
    }
    assert!(
        (p.layout.send.top() - send.top()).abs() < 1.0,
        "Send moved when the draft grew"
    );
    assert!((p.layout.stop.top() - stop.top()).abs() < 1.0);
    assert!(p.layout.panel.contains_rect(p.layout.send));
}

#[test]
fn embedded_manual_selects_relevant_sections_with_a_hard_byte_budget() {
    for prompt in [
        "Inspect the rig then add breathing",
        "une attaque avec anticipation",
        "rigging poids et articulations",
        "hello",
        &"rig mesh key attack breathing weight ".repeat(1000),
    ] {
        let a = manual::select(prompt, true);
        let b = manual::select(prompt, true);
        assert_eq!(a.text, b.text);
        assert!(a.text.len() <= manual::MAX_BYTES);
        assert!(a.sections.contains(&"TOOLS"));
        assert!(a.text.contains("NEVER creates"));
        assert!(a.sections.len() < 7);
    }
    assert!(
        manual::select("rig respiration", true)
            .sections
            .contains(&"Breathing")
    );
    assert!(manual::select("attaque", true).sections.contains(&"Attack"));
    let rig = manual::select("rigging poids", true);
    assert!(rig.sections.contains(&"RIGGING"));
    assert!(!rig.sections.contains(&"Breathing"));
    let no_tools = manual::select("hello", false);
    assert_eq!(no_tools.sections, vec!["ANKHIMATE-AI"]);
    for provider in Provider::ALL {
        let body = protocol::request(
            &Settings {
                provider,
                ..Default::default()
            },
            &[Message::text("user", "respiration")],
            &tools::definitions(true),
        );
        let text = match provider {
            Provider::OpenAI => body["instructions"].as_str().unwrap(),
            Provider::Anthropic => body["system"].as_str().unwrap(),
            _ => body["messages"][0]["content"].as_str().unwrap(),
        };
        assert!(text.contains(manual::VERSION) && text.contains("Breathing"));
    }
}

#[test]
fn manual_examples_match_typed_actions_wire_schema_and_native_results() {
    use std::collections::BTreeSet;
    let rows: Vec<Value> =
        serde_json::from_str(include_str!("../../../docs/ai/examples.json")).unwrap();
    let defs = tools::definitions(true);
    let variants = defs[1]["parameters"]["properties"]["actions"]["items"]["oneOf"]
        .as_array()
        .unwrap();
    let mut used = BTreeSet::new();
    for row in rows {
        let mut s = state();
        s.dispatch(Box::new(CreateAnimation::new("existing", 2.0)));
        let before = snapshot(&s);
        let plan: tools::EditPlan = serde_json::from_value(row["plan"].clone()).unwrap();
        for action in &plan.actions {
            let value = serde_json::to_value(action).unwrap();
            let op = action.operation();
            used.insert(op);
            let schema = variants
                .iter()
                .find(|v| v["properties"]["op"]["const"] == op)
                .unwrap();
            let fields: value_set::Fields = value.as_object().unwrap().keys().cloned().collect();
            let required: value_set::Fields = schema["required"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect();
            assert_eq!(fields, required);
            assert_eq!(schema["additionalProperties"], false);
            for (field, v) in value.as_object().unwrap() {
                let rule = &schema["properties"][field];
                if let Some(values) = rule["enum"].as_array() {
                    assert!(values.contains(v));
                }
                match rule["type"].as_str() {
                    Some("string") => assert!(v.is_string()),
                    Some("number") => assert!(v.is_number()),
                    Some("boolean") => assert!(v.is_boolean()),
                    _ => {}
                }
                if let Some(min) = rule["minimum"].as_f64()
                    && row["expected"] == "valid"
                {
                    assert!(v.as_f64().unwrap() >= min);
                }
                if let Some(max) = rule["maximum"].as_f64()
                    && row["expected"] == "valid"
                {
                    assert!(v.as_f64().unwrap() <= max);
                }
            }
        }
        let result = tools::validate(&tools::snapshot(&s), &plan);
        if row["expected"] == "valid" {
            result.unwrap().approve(&mut s).unwrap();
        } else {
            assert_eq!(
                result.err().unwrap().code,
                row["expected"].as_str().unwrap()
            );
            assert_eq!(snapshot(&s), before);
        }
    }
    assert_eq!(
        used,
        BTreeSet::from([
            "animation.create",
            "animation.set",
            "animation.select",
            "key.add"
        ])
    );
    assert_eq!(
        defs[1]["parameters"]["properties"]["actions"]["maxItems"],
        tools::MAX_ACTIONS
    );
    for variant in variants {
        if variant["properties"].get("duration").is_some() {
            assert_eq!(
                variant["properties"]["duration"]["maximum"],
                tools::MAX_DURATION
            );
        }
    }
    // Exact units remain covered by the native 45-degree and 1.03 multiplier tests.
}
mod value_set {
    pub type Fields = std::collections::BTreeSet<String>;
}

pub(super) fn compatible_call(name: &str, args: Value) -> String {
    format!(
        "data: {}\n\ndata: [DONE]\n\n",
        json!({"choices":[{"delta":{"tool_calls":[{"index":0,"id":"reused","function":{"name":name,"arguments":args.to_string()}}]},"finish_reason":"tool_calls"}]})
    )
}

#[test]
fn ninfer_correction_carries_exact_error_proposal_identity_and_success_on_next_send() {
    let mut s = state();
    let before = snapshot(&s);
    let bad = json!({"summary":"Set missing animation","actions":[{"op":"animation.set","animation":"ai_breath","duration":2,"looping":true}]});
    let (base, rx) = server(vec![
        (
            200,
            compatible_call("ankhimate_animate", bad),
            Duration::ZERO,
        ),
        (
            200,
            compatible_call(
                "ankhimate_animate",
                serde_json::to_value(breathing("torso")).unwrap(),
            ),
            Duration::ZERO,
        ),
        (
            200,
            text_stream(Provider::NInfer, "The latest native proposal was applied."),
            Duration::ZERO,
        ),
    ]);
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            model: "qwen-compatible".into(),
            local_url: base,
            tools_enabled: true,
            editing: true,
            ..Default::default()
        },
        key: Zeroizing::new("mock-only".into()),
        share_context: true,
        input: "Add breathing".into(),
        ..Default::default()
    };
    p.start(&s, &egui::Context::default(), false).unwrap();
    poll_until_idle(&mut p, &s);
    assert!(p.pending.is_some());
    assert_eq!(snapshot(&s), before);
    assert_eq!(p.activity.phase, Phase::AwaitingApproval);
    rx.recv().unwrap();
    let second = rx.recv().unwrap();
    let wire = second["messages"].as_array().unwrap();
    let result: Value = serde_json::from_str(
        wire.iter().find(|m| m["role"] == "tool").unwrap()["content"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(result["code"], "animation_not_found");
    assert_eq!(result["operation"], "animation.set");
    assert_eq!(result["field"], "animation");
    assert_eq!(result["document_changed"], false);
    assert!(result["correction"].as_str().unwrap().contains("Create"));
    let approved_id = p.pending.as_ref().unwrap().proposal_id.clone();
    assert_ne!(result["proposal_id"], approved_id);
    p.resolve(&mut s, true).unwrap();
    assert_eq!(p.last_result.as_ref().unwrap()["status"], "applied");
    assert!(p.job.is_none());
    let after = snapshot(&s);
    assert!(p.resolve(&mut s, true).is_err());
    assert_eq!(snapshot(&s), after);
    let old: Value = serde_json::from_str(
        &p.messages
            .iter()
            .find(|m| m.role == "tool" && m.text.contains("animation_not_found"))
            .unwrap()
            .text,
    )
    .unwrap();
    assert_eq!(old["status"], "invalid");
    p.input = "What actually happened?".into();
    p.start(&s, &egui::Context::default(), false).unwrap();
    poll_until_idle(&mut p, &s);
    let next = rx.recv().unwrap();
    let messages = next["messages"].as_array().unwrap();
    let verified = messages
        .iter()
        .find_map(|m| {
            serde_json::from_str::<Value>(m["content"].as_str()?)
                .ok()
                .filter(|v| v.get("ankhimate_verified_native_result").is_some())
        })
        .unwrap();
    assert_eq!(
        verified["ankhimate_verified_native_result"]["status"],
        "applied"
    );
    assert_eq!(
        verified["ankhimate_verified_native_result"]["proposal_id"],
        approved_id
    );
    assert_eq!(verified["document_changed_since_result"], false);
    assert!(
        p.display
            .iter()
            .any(|(role, text)| role == "Native event" && text.contains("Applied"))
    );
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), before);
        s.redo();
        assert_eq!(snapshot(&s), after);
    }
    s.undo();
    let verified = p.verified_result(&s).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&verified.text).unwrap()["document_changed_since_result"],
        true
    );
    p.share_context = false;
    assert!(p.verified_result(&s).is_none());
}

fn poll_until_idle(p: &mut Panel, s: &AppState) {
    let deadline = Instant::now() + Duration::from_secs(8);
    while p.job.is_some() {
        p.poll(s);
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn invalid_corrections_stop_without_changing_intent_or_document() {
    let s = state();
    let before = snapshot(&s);
    let bad = json!({"summary":"Still invalid","actions":[{"op":"animation.set","animation":"missing","duration":2,"looping":true}]});
    let (base, rx) = server(
        (0..3)
            .map(|_| {
                (
                    200,
                    compatible_call("ankhimate_animate", bad.clone()),
                    Duration::ZERO,
                )
            })
            .collect(),
    );
    let turn = wait(transport::start_at(
        Settings {
            provider: Provider::NInfer,
            tools_enabled: true,
            editing: true,
            tool_budget: 16,
            ..Default::default()
        },
        key(),
        vec![],
        Some(tools::snapshot(&s)),
        false,
        egui::Context::default(),
        base,
    ))
    .unwrap();
    assert!(turn.plan.is_none());
    assert_eq!(turn.failure.unwrap().code, "animation_not_found");
    assert_eq!(rx.try_iter().count(), 3);
    assert_eq!(snapshot(&s), before);
}

#[test]
fn selecting_only_reports_no_document_write_and_has_no_fake_undo() {
    let mut s = state();
    s.dispatch(Box::new(CreateAnimation::new("existing", 2.0)));
    let before = snapshot(&s);
    let rev = s.revision;
    let label = s.history.undo_label().map(str::to_owned);
    let plan = tools::EditPlan {
        summary: "Select existing".into(),
        actions: vec![tools::Action::Select {
            animation: "existing".into(),
        }],
    };
    let mut p = Panel {
        settings: Settings {
            tools_enabled: true,
            editing: true,
            ..Default::default()
        },
        share_context: true,
        ..Default::default()
    };
    p.pending = Some(Pending {
        edit: tools::prepare(&s, &plan).unwrap(),
        ids: vec![],
        proposal_id: "selection".into(),
    });
    p.resolve(&mut s, true).unwrap();
    assert_eq!(p.last_result.as_ref().unwrap()["document_changed"], false);
    assert_eq!(s.revision, rev);
    assert_eq!(snapshot(&s), before);
    assert_eq!(s.history.undo_label(), label.as_deref());
}

fn sized_frame(
    ctx: &egui::Context,
    p: &mut Panel,
    s: &mut AppState,
    size: egui::Vec2,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            events,
            ..Default::default()
        },
        |ui| p.ui(ui, s),
    )
}

#[test]
fn docked_narrow_and_detached_sized_views_keep_controls_visible_with_long_chat() {
    for size in [
        egui::vec2(410.0, 180.0),
        egui::vec2(310.0, 360.0),
        egui::vec2(460.0, 600.0),
        egui::vec2(900.0, 800.0),
    ] {
        for pending in [false, true] {
            let mut s = state();
            let mut p = Panel {
                display: vec![("Assistant".into(), "Long selectable reply\n".repeat(150))],
                streaming: "More streamed text…".into(),
                ..Default::default()
            };
            if pending {
                p.pending = Some(Pending {
                    edit: tools::prepare(&s, &breathing("torso")).unwrap(),
                    ids: vec![],
                    proposal_id: "layout".into(),
                });
            }
            let ctx = egui::Context::default();
            let output = sized_frame(&ctx, &mut p, &mut s, size, vec![]);
            if !(pending && size.y < 300.0) {
                assert!(p.layout.panel.contains_rect(p.layout.send));
                assert!(p.layout.panel.contains_rect(p.layout.stop));
                text_center(&output.shapes, "Send · Ctrl+Enter");
            }
            assert!(p.layout.conversation.height() > 10.0);
            assert!(!p.settings_open);
            if pending {
                text_center(&output.shapes, "Approve native edits");
                text_center(&output.shapes, "Refuse changes");
            }
        }
    }
}

#[test]
fn scrolling_up_is_preserved_when_new_messages_arrive() {
    let mut s = state();
    let mut p = Panel {
        display: vec![("Assistant".into(), "Long reply\n".repeat(300))],
        ..Default::default()
    };
    let ctx = egui::Context::default();
    let size = egui::vec2(430.0, 600.0);
    for _ in 0..3 {
        sized_frame(&ctx, &mut p, &mut s, size, vec![]);
    }
    let bottom = p.layout.scroll_offset;
    assert!(bottom > 1000.0);
    let pos = p.layout.conversation.center();
    sized_frame(
        &ctx,
        &mut p,
        &mut s,
        size,
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                phase: egui::TouchPhase::Move,
                delta: egui::vec2(0.0, 1000.0),
                modifiers: Default::default(),
            },
        ],
    );
    for _ in 0..60 {
        sized_frame(&ctx, &mut p, &mut s, size, vec![]);
    }
    let reading = p.layout.scroll_offset;
    assert!(reading < bottom - 100.0);
    p.display
        .push(("Assistant".into(), "A newly received answer\n".repeat(20)));
    sized_frame(&ctx, &mut p, &mut s, size, vec![]);
    assert!(
        (p.layout.scroll_offset - reading).abs() < 1.0,
        "Reading position jumped when a message arrived"
    );
}

#[test]
fn ctrl_enter_only_sends_from_the_composer_and_uses_the_real_send_path() {
    let s = state();
    let mut editable = state();
    let (base, rx) = server(vec![(
        200,
        text_stream(Provider::NInfer, "Bonjour"),
        Duration::ZERO,
    )]);
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            model: "mock".into(),
            local_url: base,
            ..Default::default()
        },
        key: Zeroizing::new("mock-only".into()),
        ..Default::default()
    };
    let ctx = egui::Context::default();
    let size = egui::vec2(450.0, 600.0);
    let control = egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::CTRL,
    };
    sized_frame(&ctx, &mut p, &mut editable, size, vec![control.clone()]);
    assert!(p.job.is_none());
    let output = sized_frame(&ctx, &mut p, &mut editable, size, vec![]);
    let pos = text_center(&output.shapes, "Ask about the rig…");
    sized_frame(
        &ctx,
        &mut p,
        &mut editable,
        size,
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            },
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Default::default(),
            },
        ],
    );
    sized_frame(
        &ctx,
        &mut p,
        &mut editable,
        size,
        vec![egui::Event::Text("Bonjour".into())],
    );
    sized_frame(&ctx, &mut p, &mut editable, size, vec![control]);
    assert!(p.job.is_some());
    assert_eq!(p.input, "");
    poll_until_idle(&mut p, &s);
    assert_eq!(p.activity.phase, Phase::Complete);
    rx.recv().unwrap();
}

#[test]
fn errors_restore_the_submitted_draft_and_settings_stay_outside_the_chat_layout() {
    let s = state();
    let (base, _) = server(vec![(
        401,
        "Never display this body or mock-key".into(),
        Duration::ZERO,
    )]);
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            model: "mock".into(),
            local_url: base,
            ..Default::default()
        },
        key: Zeroizing::new("mock-only".into()),
        input: "Keep this draft".into(),
        ..Default::default()
    };
    p.start(&s, &egui::Context::default(), false).unwrap();
    poll_until_idle(&mut p, &s);
    assert_eq!(p.input, "Keep this draft");
    assert_eq!(p.activity.phase, Phase::Error);
    assert!(!p.status.contains("mock-key"));
    assert!(p.display.iter().any(|(role, _)| role == "Error"));
}

#[test]
fn real_streaming_activity_is_cancellable_without_inventing_thinking() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut bytes = vec![];
        let mut chunk = [0; 4096];
        let end = loop {
            let n = socket.read(&mut chunk).unwrap();
            if n == 0 {
                return;
            }
            bytes.extend_from_slice(&chunk[..n]);
            if let Some(i) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                break i + 4;
            }
        };
        let headers = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
        let size: usize = headers
            .lines()
            .find_map(|l| l.strip_prefix("content-length:"))
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        while bytes.len() < end + size {
            let n = socket.read(&mut chunk).unwrap();
            if n == 0 {
                return;
            }
            bytes.extend_from_slice(&chunk[..n]);
        }
        let _ = socket.write_all(
            b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n",
        );
        let _ = socket.write_all(
            format!(
                "data: {}\n\n",
                json!({"choices":[{"delta":{"content":"Slow streaming"}}]})
            )
            .as_bytes(),
        );
        std::thread::sleep(Duration::from_millis(800));
        let _ = socket.write_all(b"data: [DONE]\n\n");
    });
    let s = state();
    let before = snapshot(&s);
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            model: "mock".into(),
            local_url: base,
            ..Default::default()
        },
        key: Zeroizing::new("mock-only".into()),
        input: "Slow response".into(),
        ..Default::default()
    };
    p.start(&s, &egui::Context::default(), false).unwrap();
    assert_eq!(p.activity.phase, Phase::Connecting);
    let deadline = Instant::now() + Duration::from_secs(3);
    while p.streaming.is_empty() {
        p.poll(&s);
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(p.job.is_some());
    assert_eq!(p.activity.phase, Phase::Generating);
    assert!(!p.activity.phase.label().to_lowercase().contains("think"));
    p.input = "My next draft".into();
    p.stop();
    assert_eq!(p.activity.phase, Phase::Cancelling);
    poll_until_idle(&mut p, &s);
    assert_eq!(p.activity.phase, Phase::Cancelled);
    assert_eq!(p.input, "My next draft");
    assert_eq!(snapshot(&s), before);
}
