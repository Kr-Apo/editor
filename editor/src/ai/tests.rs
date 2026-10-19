use super::*;
use crate::app_state::AppState;
use ankhimate_document::commands::key_cmds::CreateAnimation;

#[test]
fn expert_result_never_rewrites_an_older_call_with_the_same_provider_id() {
    let mut s = state();
    let mut p = Panel {
        settings: Settings {
            tools_enabled: true,
            editing: true,
            ..Default::default()
        },
        share_context: true,
        ..Default::default()
    };
    p.messages = vec![
        Message::result("reused", json!({"status":"invalid","proposal_id":"older"})),
        Message::result(
            "reused",
            json!({"status":"awaiting_user_approval","proposal_id":"newer"}),
        ),
    ];
    p.pending = Some(Pending {
        edit: tools::prepare(&s, &breathing("torso")).unwrap(),
        ids: vec!["reused".into()],
        proposal_id: "newer".into(),
    });
    p.resolve(&mut s, true).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&p.messages[0].text).unwrap()["status"],
        "invalid"
    );
    assert_eq!(
        serde_json::from_str::<Value>(&p.messages[1].text).unwrap()["status"],
        "applied"
    );
}

#[test]
fn expert_invalid_set_is_corrected_inside_the_original_tool_budget() {
    let s = state();
    let bad = json!({"summary":"Set a missing clip","actions":[{"op":"animation.set","animation":"ai_breath","duration":2,"looping":true}]});
    let (base, _) = server(vec![
        (200, native_call("ankhimate_animate", bad), Duration::ZERO),
        (
            200,
            native_call(
                "ankhimate_animate",
                serde_json::to_value(breathing("torso")).unwrap(),
            ),
            Duration::ZERO,
        ),
    ]);
    let turn = wait(transport::start_at(
        Settings {
            tools_enabled: true,
            editing: true,
            ..Default::default()
        },
        key(),
        vec![Message::text("user", "Add breathing")],
        Some(tools::snapshot(&s)),
        false,
        egui::Context::default(),
        base,
    ))
    .unwrap();
    assert!(
        matches!(turn.plan.unwrap().actions[0], tools::Action::Create { .. }),
        "An invalid set must return an actionable native error and allow a bounded correction"
    );
}
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::TcpListener,
    time::{Duration, Instant},
};

#[path = "expert_tests.rs"]
mod expert;
#[path = "live_tests.rs"]
mod live;

fn state() -> AppState {
    let mut state = AppState::default();
    let loaded=ankhimate_formats::convert::from_schema(&serde_json::from_value(json!({"version":1,"name":"agent-test","fps":30,"bones":[{"name":"root","length":1},{"name":"torso","parent":"root","length":1}]})).unwrap());
    state.replace_document(ankhimate_document::Document::from_loaded(loaded));
    state
}
fn snapshot(state: &AppState) -> Value {
    serde_json::to_value(ankhimate_formats::convert::to_schema(
        &state.doc.as_project_ref(),
    ))
    .unwrap()
}
fn breathing(bone: &str) -> tools::EditPlan {
    serde_json::from_value(json!({"summary":"Create gentle looping breathing on the actual torso", "actions":[
        {"op":"animation.create","name":"ai_breath","duration":2.0,"looping":true},
        {"op":"key.add","animation":"ai_breath","bone":bone,"channel":"scale_y","time":0.0,"value":1.0,"interpolation":"smooth"},
        {"op":"key.add","animation":"ai_breath","bone":bone,"channel":"scale_y","time":1.0,"value":1.03,"interpolation":"smooth"},
        {"op":"key.add","animation":"ai_breath","bone":bone,"channel":"scale_y","time":2.0,"value":1.0,"interpolation":"smooth"}
    ]})).unwrap()
}

#[test]
fn defaults_are_local_read_only_and_keys_are_not_serialized() {
    let p = Panel::default();
    assert!(!p.settings.provider.cloud() && !p.settings.editing && !p.settings.tools_enabled);
    assert!(p.job.is_none() && p.pending.is_none() && !p.share_context);
    let data = serde_json::to_string(&p.settings).unwrap();
    assert!(!data.contains("api_key") && !data.contains("secret"));
}

#[test]
fn revoking_context_clears_old_wire_history_before_another_send() {
    let s = state();
    let (base, rx) = server(vec![(
        200,
        text_stream(Provider::Ollama, "Conversation only"),
        Duration::ZERO,
    )]);
    let mut p = Panel {
        settings: Settings {
            model: "test-model".into(),
            local_url: base,
            ..Default::default()
        },
        key: Zeroizing::new("mock-only".into()),
        input: "Hello".into(),
        route: "earlier-route-with-context".into(),
        messages: vec![Message::text("assistant", "PRIVATE_PREVIOUS_METADATA")],
        display: vec![("Assistant".into(), "Earlier local history".into())],
        ..Default::default()
    };
    p.start(&s, &egui::Context::default(), false).unwrap();
    let request = rx.recv_timeout(Duration::from_secs(3)).unwrap();
    assert!(!request.to_string().contains("PRIVATE_PREVIOUS_METADATA"));
    assert!(!request.to_string().contains("torso"));
    assert!(request.get("tools").is_none());
    assert!(
        p.display
            .iter()
            .any(|(_, text)| text == "Earlier local history")
    );
    wait(p.job.take().unwrap()).unwrap();
}

#[test]
fn deepseek_reasoning_content_survives_a_native_tool_roundtrip() {
    let mut decoder = protocol::Decoder::default();
    decoder.frame(Provider::DeepSeek,&json!({"choices":[{"delta":{"reasoning_content":"opaque reasoning","tool_calls":[{"index":0,"id":"c1","function":{"name":"ankhimate_read","arguments":"{\"section\":\"bones\"}"}}]}}]}).to_string()).unwrap();
    decoder.frame(Provider::DeepSeek, "[DONE]").unwrap();
    let response = decoder.finish().unwrap();
    let request = protocol::request(
        &Settings {
            provider: Provider::DeepSeek,
            ..Default::default()
        },
        &[response, Message::result("c1", json!([]))],
        &[],
    );
    assert_eq!(
        request["messages"][1]["reasoning_content"],
        "opaque reasoning"
    );
    assert_eq!(request["messages"][2]["tool_call_id"], "c1");
}

#[test]
fn multiple_streamed_calls_preserve_numeric_execution_order() {
    let mut decoder = protocol::Decoder::default();
    for index in 0..12 {
        decoder.frame(Provider::LmStudio,&json!({"choices":[{"delta":{"tool_calls":[{"index":index,"id":format!("c{index}"),"function":{"name":"ankhimate_read","arguments":"{\"section\":\"bones\"}"}}]}}]}).to_string()).unwrap();
    }
    decoder.frame(Provider::LmStudio, "[DONE]").unwrap();
    assert_eq!(
        decoder
            .finish()
            .unwrap()
            .calls
            .iter()
            .map(|c| c.id.clone())
            .collect::<Vec<_>>(),
        (0..12).map(|i| format!("c{i}")).collect::<Vec<_>>()
    );
}

#[test]
fn native_batch_is_atomic_and_three_undo_redo_cycles_restore_complete_document() {
    let mut s = state();
    let before = snapshot(&s);
    let rev = s.revision;
    let proposal = tools::prepare(&s, &breathing("torso")).unwrap();
    assert_eq!(snapshot(&s), before);
    proposal.approve(&mut s).unwrap();
    let after = snapshot(&s);
    assert_ne!(before, after);
    assert!(s.revision > rev);
    assert_eq!(s.history.undo_label(), Some("AI Animation Edit"));
    let clip = s
        .doc
        .animations
        .values()
        .find(|a| a.name == "ai_breath")
        .unwrap();
    assert!(clip.looping);
    assert_eq!(clip.timelines.len(), 1);
    assert_eq!(
        crate::app_state::key_times(&clip.timelines[0]),
        vec![0.0, 1.0, 2.0]
    );
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), before);
        assert!(!s.history.can_undo());
        s.redo();
        assert_eq!(snapshot(&s), after);
    }
    s.session.playhead = 1.0;
    s.refresh_pose();
    let bone = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "torso")
        .unwrap()
        .0;
    assert!((s.pose.locals[bone].scale.y - 1.03).abs() < 0.001);
    s.session.playhead = 0.0;
    s.refresh_pose();
    assert!((s.pose.locals[bone].scale.y - 1.0).abs() < 0.001);
}

#[test]
fn rejected_or_invalid_proposals_preserve_document_redo_selection_and_assets() {
    let mut s = state();
    s.dispatch(Box::new(CreateAnimation::new("existing", 1.0)));
    s.undo();
    let before = snapshot(&s);
    let redo = s.history.redo_label().map(str::to_owned);
    let selected = s.session.active_animation;
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
        edit: tools::prepare(&s, &breathing("torso")).unwrap(),
        ids: vec![],
        proposal_id: "test-proposal".into(),
    });
    p.resolve(&mut s, false).unwrap();
    assert_eq!(snapshot(&s), before);
    assert_eq!(s.session.active_animation, selected);
    assert_eq!(s.history.redo_label(), redo.as_deref());
    let mut bad = breathing("torso");
    if let tools::Action::Key { bone, .. } = &mut bad.actions[2] {
        *bone = "unknown".into();
    }
    assert!(tools::prepare(&s, &bad).is_err());
    assert_eq!(snapshot(&s), before);
    assert_eq!(s.history.redo_label(), redo.as_deref());
    s.redo();
    assert!(s.doc.animations.values().any(|a| a.name == "existing"));
}

#[test]
fn stale_or_newly_locked_proposals_never_apply() {
    let mut s = state();
    let edit = tools::prepare(&s, &breathing("torso")).unwrap();
    s.dispatch(Box::new(CreateAnimation::new("newer", 1.0)));
    let before = snapshot(&s);
    assert!(edit.approve(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
    let edit = tools::prepare(&s, &breathing("torso")).unwrap();
    let bone = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "torso")
        .unwrap()
        .0;
    s.session.locked_bones.insert(bone, true);
    assert!(edit.approve(&mut s).is_err());
    assert_eq!(snapshot(&s), before);
}

#[test]
fn action_limits_unknown_fields_nonfinite_and_duration_errors_are_rejected() {
    let s = state();
    let before = snapshot(&s);
    assert!(
        serde_json::from_value::<tools::EditPlan>(
            json!({"summary":"bad","actions":[],"script":"code"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<tools::EditPlan>(
            json!({"summary":"bad","actions":[{"op":"file.write","path":"x"}]})
        )
        .is_err()
    );
    for value in [f32::NAN, f32::INFINITY, -1.0, 4000.0] {
        let mut p = breathing("torso");
        if let tools::Action::Create { duration, .. } = &mut p.actions[0] {
            *duration = value;
        }
        assert!(tools::prepare(&s, &p).is_err());
    }
    for time in [f32::NAN, f32::INFINITY, -0.1, 3.0] {
        let mut p = breathing("torso");
        if let tools::Action::Key { time: t, .. } = &mut p.actions[2] {
            *t = time;
        }
        assert!(tools::prepare(&s, &p).is_err());
    }
    let mut p = breathing("torso");
    p.actions = vec![p.actions[0].clone(); 129];
    assert!(tools::prepare(&s, &p).is_err());
    assert_eq!(snapshot(&s), before);
}

#[test]
fn duplicate_keys_update_and_native_angle_units_and_meta_are_preserved() {
    let mut s = state();
    let mut plan = breathing("torso");
    let mut key = plan.actions[2].clone();
    if let tools::Action::Key { value, .. } = &mut key {
        *value = 1.02;
    }
    plan.actions.push(key);
    plan.actions.push(tools::Action::Key {
        animation: "ai_breath".into(),
        bone: "torso".into(),
        channel: tools::Channel::Rotate,
        time: 1.0,
        value: 45.0,
        interpolation: tools::Curve::Linear,
    });
    tools::prepare(&s, &plan).unwrap().approve(&mut s).unwrap();
    let a = s.doc.animations.values().next().unwrap();
    assert_eq!(crate::app_state::key_times(&a.timelines[0]).len(), 3);
    s.session.playhead = 1.0;
    s.refresh_pose();
    let bone = s
        .doc
        .skeleton
        .bones
        .iter()
        .find(|(_, b)| b.name == "torso")
        .unwrap()
        .0;
    assert!((s.pose.locals[bone].rotation.to_degrees() - 45.0).abs() < 0.001);
    let meta = tools::EditPlan {
        summary: "Shorten".into(),
        actions: vec![tools::Action::Meta {
            animation: "ai_breath".into(),
            duration: 0.5,
            looping: false,
        }],
    };
    let before = snapshot(&s);
    assert!(tools::prepare(&s, &meta).is_err());
    assert_eq!(snapshot(&s), before);
}

#[test]
fn save_reopen_native_export_and_assets_survive_agent_edit() {
    let mut s = state();
    tools::prepare(&s, &breathing("torso"))
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let expected = snapshot(&s);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("édition-agent.ankh");
    assert!(matches!(
        crate::fileops::write_to(&s, &path),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let mut reopened = AppState::default();
    crate::fileops::open_path(&mut reopened, &path);
    assert_eq!(snapshot(&reopened), expected);
    let schema = ankhimate_formats::convert::to_schema(&reopened.doc.as_project_ref());
    let plan = ankhimate_export::run::plan(
        &schema,
        &reopened.doc.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let data: Value = serde_json::from_str(
        &plan
            .files
            .iter()
            .find(|f| f.path == "skeleton.json")
            .unwrap()
            .contents,
    )
    .unwrap();
    assert_eq!(data["format_version"], 2);
    assert!(data["animations"]["ai_breath"].is_object());
}

#[test]
fn local_url_policy_never_sends_credentials_to_remote_or_credential_urls() {
    for url in [
        "http://example.com",
        "http://127.0.0.1:1234?key=private",
        "http://name:secret@127.0.0.1:1234",
        "file:///tmp/project",
        "https://localhost/#private",
    ] {
        let s = Settings {
            local_url: url.into(),
            ..Default::default()
        };
        assert!(s.base_url().is_err());
    }
    for url in [
        "http://localhost:1234/v1",
        "http://127.0.0.1:11434",
        "http://[::1]:1234",
    ] {
        assert!(
            Settings {
                local_url: url.into(),
                ..Default::default()
            }
            .base_url()
            .is_ok()
        );
    }
}

#[test]
fn providers_use_distinct_requests_and_manual_model_ids() {
    for provider in Provider::ALL {
        let s = Settings {
            provider,
            model: "my-installed-qwen-or-other-model".into(),
            ..Default::default()
        };
        let body = protocol::request(
            &s,
            &[Message::text("user", "Hello")],
            &tools::definitions(true),
        );
        assert_eq!(body["model"], s.model);
        assert_eq!(body["stream"], true);
        match provider {
            Provider::OpenAI => {
                assert_eq!(body["store"], false);
                assert!(body["input"].is_array());
                assert_eq!(body["tools"][0]["type"], "function");
            }
            Provider::Anthropic => {
                assert!(body["system"].is_string());
                assert!(body["tools"][0]["input_schema"].is_object());
            }
            Provider::Ollama => assert_eq!(body["options"]["num_predict"], s.max_tokens),
            _ => {
                assert_eq!(body["max_tokens"], s.max_tokens);
                assert!(body["tools"][0]["function"]["parameters"].is_object());
            }
        }
        let conversation = protocol::request(&s, &[Message::text("user", "Hi")], &[]);
        assert!(conversation.get("tools").is_none());
    }
}

fn text_stream(provider: Provider, text: &str) -> String {
    match provider {
        Provider::OpenAI => format!(
            "data: {}\n\ndata: {}\n\n",
            json!({"type":"response.output_text.delta","delta":text}),
            json!({"type":"response.completed"})
        ),
        Provider::Anthropic => format!(
            "data: {}\n\ndata: {}\n\ndata: {}\n\n",
            json!({"type":"content_block_start","index":0,"content_block":{"type":"text","text":""}}),
            json!({"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":text}}),
            json!({"type":"message_stop"})
        ),
        Provider::Ollama => format!(
            "{}\n{}\n",
            json!({"message":{"content":text}}),
            json!({"done":true})
        ),
        _ => format!(
            "data: {}\n\ndata: [DONE]\n\n",
            json!({"choices":[{"delta":{"content":text}}]})
        ),
    }
}
#[test]
fn all_provider_streams_handle_split_utf8_bytes_and_truncation() {
    for provider in Provider::ALL {
        let data = text_stream(provider, "Respiration douce — prête 🐸");
        let mut f = transport::Framer::new();
        let mut d = protocol::Decoder::default();
        for b in data.as_bytes() {
            f.push(&[*b], provider, &mut d).unwrap();
        }
        f.end(provider, &mut d).unwrap();
        assert_eq!(d.finish().unwrap().text, "Respiration douce — prête 🐸");
        let mut d = protocol::Decoder::default();
        assert!(d.frame(provider, "not-json").is_err());
        assert!(protocol::Decoder::default().finish().is_err());
    }
}
#[test]
fn tool_argument_deltas_and_opaque_reasoning_replay_are_preserved() {
    let mut d = protocol::Decoder::default();
    for v in [
        json!({"type":"response.output_item.added","output_index":1,"item":{"type":"function_call","call_id":"c1","name":"ankhimate_read"}}),
        json!({"type":"response.function_call_arguments.delta","output_index":1,"delta":"{\"section\":"}),
        json!({"type":"response.function_call_arguments.delta","output_index":1,"delta":"\"bones\"}"}),
        json!({"type":"response.completed","response":{"output":[{"type":"reasoning","id":"r1","encrypted_content":"opaque"},{"type":"function_call","call_id":"c1","name":"ankhimate_read","arguments":"{\"section\":\"bones\"}"}]}}),
    ] {
        d.frame(Provider::OpenAI, &v.to_string()).unwrap();
    }
    let m = d.finish().unwrap();
    assert_eq!(m.calls[0].arguments["section"], "bones");
    let body = protocol::request(
        &Settings {
            provider: Provider::OpenAI,
            ..Default::default()
        },
        &[m, Message::result("c1", json!([]))],
        &[],
    );
    assert_eq!(body["input"][0]["encrypted_content"], "opaque");
    assert_eq!(body["input"][2]["type"], "function_call_output");
    for provider in [Provider::DeepSeek, Provider::LmStudio] {
        let mut d = protocol::Decoder::default();
        for args in ["{\"section\":", "\"selection\"}"] {
            d.frame(provider,&json!({"choices":[{"delta":{"tool_calls":[{"index":0,"id":"c1","function":{"name":if args.starts_with('{'){"ankhimate_read"}else{""},"arguments":args}}]}}]}).to_string()).unwrap();
        }
        d.frame(provider, "[DONE]").unwrap();
        assert_eq!(
            d.finish().unwrap().calls[0].arguments["section"],
            "selection"
        );
    }
}

fn server(responses: Vec<(u16, String, Duration)>) -> (String, std::sync::mpsc::Receiver<Value>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let (sender, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for (status, body, delay) in responses {
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
            let size = headers
                .lines()
                .find_map(|l| {
                    l.strip_prefix("content-length:")
                        .and_then(|s| s.trim().parse::<usize>().ok())
                })
                .unwrap_or(0);
            while bytes.len() < end + size {
                let n = socket.read(&mut chunk).unwrap();
                if n == 0 {
                    return;
                }
                bytes.extend_from_slice(&chunk[..n]);
            }
            let request: Value =
                serde_json::from_slice(&bytes[end..end + size]).unwrap_or(Value::Null);
            let _ = sender.send(request);
            std::thread::sleep(delay);
            let header = format!(
                "HTTP/1.1 {status} Test\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = socket.write_all(header.as_bytes());
            let _ = socket.write_all(body.as_bytes());
        }
    });
    (base, rx)
}
fn wait(job: transport::Job) -> Result<transport::Turn, String> {
    let deadline = Instant::now() + Duration::from_secs(8);
    loop {
        match job.receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(transport::Event::Done(result)) => return result,
            Ok(_) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => assert!(Instant::now() < deadline),
            Err(e) => panic!("worker ended without result: {e}"),
        }
    }
}
fn key() -> vault::Secret {
    vault::Secret(Zeroizing::new("mock-key-not-a-real-credential".into()))
}
#[test]
fn five_http_adapters_really_stream_against_loopback_fixtures() {
    for provider in Provider::ALL {
        let (base, rx) = server(vec![(
            200,
            text_stream(provider, "Native streaming works"),
            Duration::ZERO,
        )]);
        let s = Settings {
            provider,
            model: "manual-model".into(),
            ..Default::default()
        };
        let turn = wait(transport::start_at(
            s,
            key(),
            vec![Message::text("user", "hello")],
            None,
            false,
            egui::Context::default(),
            base,
        ))
        .unwrap();
        assert_eq!(turn.messages.last().unwrap().text, "Native streaming works");
        let req = rx.recv().unwrap();
        assert!(req.get("tools").is_none());
    }
}
#[test]
fn http_auth_quota_timeout_and_cancel_never_change_the_project() {
    for code in [401, 429] {
        let (base, _) = server(vec![(
            code,
            "server echoes mock-key-not-a-real-credential".into(),
            Duration::ZERO,
        )]);
        let error = wait(transport::start_at(
            Settings::default(),
            key(),
            vec![],
            None,
            false,
            egui::Context::default(),
            base,
        ))
        .err()
        .unwrap();
        assert!(error.contains(if code == 401 { "401" } else { "429" }));
        assert!(!error.contains("mock-key"));
    }
    let (base, _) = server(vec![(200, String::new(), Duration::from_secs(2))]);
    let error = wait(transport::start_at(
        Settings {
            timeout_seconds: 1,
            ..Default::default()
        },
        key(),
        vec![],
        None,
        false,
        egui::Context::default(),
        base,
    ))
    .err()
    .unwrap();
    assert!(error.contains("timed out"));
    let (base, rx) = server(vec![(200, String::new(), Duration::from_secs(2))]);
    let mut job = transport::start_at(
        Settings::default(),
        key(),
        vec![],
        None,
        false,
        egui::Context::default(),
        base,
    );
    rx.recv_timeout(Duration::from_secs(2)).unwrap();
    let start = Instant::now();
    job.stop();
    assert!(wait(job).err().unwrap().contains("Stopped"));
    assert!(start.elapsed() < Duration::from_millis(600));
}

#[test]
fn metadata_contains_no_paths_image_bytes_or_psd_provenance() {
    let mut s = state();
    s.doc
        .psd_layer_paths
        .insert("private".into(), "DO_NOT_SEND_THIS_PATH".into());
    s.doc
        .export_presets
        .push(json!({"secret":"DO_NOT_SEND_PRESET"}));
    let context = tools::context(&s).to_string();
    assert!(context.contains("torso"));
    assert!(!context.contains("DO_NOT_SEND"));
    assert!(!format!("{:?}", key()).contains("mock-key"));
}

#[cfg(windows)]
#[test]
fn windows_credential_vault_real_roundtrip_uses_unique_test_target() {
    let target = format!(
        "Ankhimate.Community.AI/test/{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
    );
    assert!(vault::load(&target).unwrap().is_none());
    vault::store(&target, "test-key-sentinel-only").unwrap();
    assert_eq!(
        vault::load(&target).unwrap().unwrap().0.as_str(),
        "test-key-sentinel-only"
    );
    vault::remove(&target).unwrap();
    assert!(vault::load(&target).unwrap().is_none());
}

fn native_call(name: &str, args: Value) -> String {
    format!(
        "{}\n",
        json!({"message":{"content":"","tool_calls":[{"function":{"name":name,"arguments":args}}]},"done":true})
    )
}
#[test]
fn model_reads_real_metadata_then_proposes_without_mutating_until_confirmation() {
    let mut s = state();
    let before = snapshot(&s);
    let (base, rx) = server(vec![
        (
            200,
            native_call("ankhimate_read", json!({"section":"bones"})),
            Duration::ZERO,
        ),
        (
            200,
            native_call(
                "ankhimate_animate",
                serde_json::to_value(breathing("torso")).unwrap(),
            ),
            Duration::ZERO,
        ),
    ]);
    let settings = Settings {
        tools_enabled: true,
        editing: true,
        model: "test-model".into(),
        ..Default::default()
    };
    let turn = wait(transport::start_at(
        settings,
        key(),
        vec![Message::text("user", "Inspect and add breathing")],
        Some(tools::snapshot(&s)),
        false,
        egui::Context::default(),
        base,
    ))
    .unwrap();
    assert_eq!(snapshot(&s), before);
    assert!(turn.plan.is_some());
    rx.recv().unwrap();
    let second = rx.recv().unwrap();
    let result = second["messages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["role"] == "tool")
        .unwrap();
    assert_eq!(result["tool_name"], "ankhimate_read");
    assert!(result["content"].as_str().unwrap().contains("torso"));
    tools::prepare(&s, &turn.plan.unwrap())
        .unwrap()
        .approve(&mut s)
        .unwrap();
    assert!(s.doc.animations.values().any(|a| a.name == "ai_breath"));
}
#[test]
fn tool_budget_read_only_and_unknown_tool_refuse_without_writes() {
    for (settings, reply) in [
        (
            Settings {
                tools_enabled: true,
                editing: true,
                tool_budget: 1,
                ..Default::default()
            },
            native_call("ankhimate_read", json!({"section":"bones"})),
        ),
        (
            Settings {
                tools_enabled: true,
                editing: false,
                ..Default::default()
            },
            native_call(
                "ankhimate_animate",
                serde_json::to_value(breathing("torso")).unwrap(),
            ),
        ),
        (
            Settings {
                tools_enabled: true,
                editing: true,
                ..Default::default()
            },
            native_call("execute_script", json!({"code":"bad"})),
        ),
        (
            Settings {
                tools_enabled: true,
                editing: true,
                ..Default::default()
            },
            native_call(
                "ankhimate_read",
                json!({"section":"bones","path":"forbidden"}),
            ),
        ),
    ] {
        let s = state();
        let before = snapshot(&s);
        let (base, _) = server(vec![(200, reply, Duration::ZERO)]);
        assert!(
            wait(transport::start_at(
                settings,
                key(),
                vec![],
                Some(tools::snapshot(&s)),
                false,
                egui::Context::default(),
                base
            ))
            .is_err()
        );
        assert_eq!(snapshot(&s), before);
    }
}
#[test]
fn models_are_discovered_without_chat_or_cloud_account_for_local_servers() {
    for provider in Provider::ALL {
        let body = if provider == Provider::Ollama {
            json!({"models":[{"name":"qwen-local"},{"name":"another-installed-model"}]})
        } else {
            json!({"data":[{"id":"manual-compatible-model"}]})
        };
        let (base, _) = server(vec![(200, body.to_string(), Duration::ZERO)]);
        let job = transport::start_at(
            Settings {
                provider,
                ..Default::default()
            },
            key(),
            vec![],
            None,
            true,
            egui::Context::default(),
            base,
        );
        match job.receiver.recv_timeout(Duration::from_secs(3)).unwrap() {
            transport::Event::Models(Ok(models)) => assert!(!models.is_empty()),
            _ => panic!("model discovery failed"),
        }
    }
}
#[test]
fn disconnected_local_server_and_truncated_stream_surface_errors() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    assert!(
        wait(transport::start_at(
            Settings::default(),
            key(),
            vec![],
            None,
            false,
            egui::Context::default(),
            base
        ))
        .err()
        .unwrap()
        .contains("connect")
    );
    let (base, _) = server(vec![(
        200,
        json!({"message":{"content":"unfinished"}}).to_string() + "\n",
        Duration::ZERO,
    )]);
    assert!(
        wait(transport::start_at(
            Settings::default(),
            key(),
            vec![],
            None,
            false,
            egui::Context::default(),
            base
        ))
        .err()
        .unwrap()
        .contains("completion")
    );
}
#[test]
fn anthropic_thinking_signature_and_json_fragments_survive_tool_roundtrip() {
    let mut d = protocol::Decoder::default();
    for v in [
        json!({"type":"content_block_start","index":0,"content_block":{"type":"thinking","thinking":"","signature":""}}),
        json!({"type":"content_block_delta","index":0,"delta":{"type":"thinking_delta","thinking":"opaque thought"}}),
        json!({"type":"content_block_delta","index":0,"delta":{"type":"signature_delta","signature":"opaque-signature"}}),
        json!({"type":"content_block_start","index":1,"content_block":{"type":"tool_use","id":"tool1","name":"ankhimate_read","input":{}}}),
        json!({"type":"content_block_delta","index":1,"delta":{"type":"input_json_delta","partial_json":"{\"section\":\"bones\"}"}}),
        json!({"type":"message_stop"}),
    ] {
        d.frame(Provider::Anthropic, &v.to_string()).unwrap();
    }
    let m = d.finish().unwrap();
    let b = protocol::request(
        &Settings {
            provider: Provider::Anthropic,
            ..Default::default()
        },
        &[
            Message::text("user", "Read"),
            m,
            Message::result("tool1", json!([])),
        ],
        &[],
    );
    assert_eq!(
        b["messages"][1]["content"][0]["signature"],
        "opaque-signature"
    );
    assert_eq!(b["messages"][1]["content"][1]["input"]["section"], "bones");
    assert_eq!(b["messages"][2]["content"][0]["tool_use_id"], "tool1");
}

fn ui_frame(
    ctx: &egui::Context,
    p: &mut Panel,
    s: &mut AppState,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1100.0, 1200.0),
            )),
            events,
            ..Default::default()
        },
        |ui| p.ui(ui, s),
    )
}
fn text_center(shapes: &[egui::epaint::ClippedShape], label: &str) -> egui::Pos2 {
    fn find(shape: &egui::Shape, label: &str) -> Option<egui::Pos2> {
        match shape {
            egui::Shape::Text(t) if t.galley.job.text == label => {
                Some(t.pos + t.galley.rect.center().to_vec2())
            }
            egui::Shape::Vec(shapes) => shapes.iter().find_map(|s| find(s, label)),
            _ => None,
        }
    }
    shapes
        .iter()
        .find_map(|s| find(&s.shape, label))
        .unwrap_or_else(|| panic!("Missing UI control {label}"))
}
#[test]
fn egui_approve_and_refuse_buttons_really_resolve_native_proposals() {
    for approve in [false, true] {
        let mut s = state();
        let before = snapshot(&s);
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
            edit: tools::prepare(&s, &breathing("torso")).unwrap(),
            ids: vec![],
            proposal_id: "test-proposal".into(),
        });
        let ctx = egui::Context::default();
        let frame = ui_frame(&ctx, &mut p, &mut s, vec![]);
        let pos = text_center(
            &frame.shapes,
            if approve {
                "Approve native edits"
            } else {
                "Refuse changes"
            },
        );
        ui_frame(
            &ctx,
            &mut p,
            &mut s,
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
        assert!(p.pending.is_none());
        assert_eq!(snapshot(&s) == before, !approve);
        assert_eq!(s.history.can_undo(), approve);
    }
}

#[test]
#[ignore = "Qualification on a complete isolated Jade copy, supplied via ANKHIMATE_AI_DEMO_PROJECT"]
fn jade_copy_native_demo_save_reopen_export() {
    let path = std::path::PathBuf::from(
        std::env::var("ANKHIMATE_AI_DEMO_PROJECT").expect("Set the path to an isolated Jade copy"),
    );
    let mut s = AppState::default();
    assert!(matches!(
        crate::fileops::open_path(&mut s, &path),
        crate::fileops::FileOutcome::Opened(_)
    ));
    let bone = s
        .doc
        .skeleton
        .bones
        .values()
        .find(|b| b.name == "chest")
        .expect("Jade's actual torso bone is named chest")
        .name
        .clone();
    let before = snapshot(&s);
    let asset_bytes: Vec<_> = s
        .doc
        .assets
        .images
        .iter()
        .map(|(_, a)| a.bytes.clone())
        .collect();
    tools::prepare(&s, &breathing(&bone))
        .unwrap()
        .approve(&mut s)
        .unwrap();
    let after = snapshot(&s);
    assert_ne!(before, after);
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), before);
        s.redo();
        assert_eq!(snapshot(&s), after);
    }
    let output = path.parent().unwrap().join("Jade-AI-Breathing.ankh");
    assert!(!output.exists(), "Refuse overwrite of demo evidence");
    assert!(matches!(
        crate::fileops::write_to(&s, &output),
        crate::fileops::FileOutcome::Saved(_)
    ));
    let mut reopened = AppState::default();
    crate::fileops::open_path(&mut reopened, &output);
    assert_eq!(snapshot(&reopened), after);
    assert_eq!(
        reopened
            .doc
            .assets
            .images
            .iter()
            .map(|(_, a)| a.bytes.clone())
            .collect::<Vec<_>>(),
        asset_bytes
    );
    let schema = ankhimate_formats::convert::to_schema(&reopened.doc.as_project_ref());
    let plan = ankhimate_export::run::plan(
        &schema,
        &reopened.doc.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let destination = path.parent().unwrap().join("AI-native-export");
    assert!(!destination.exists());
    std::fs::create_dir(&destination).unwrap();
    ankhimate_export::run::write(&plan, &destination).unwrap();
    let data: Value =
        serde_json::from_slice(&std::fs::read(destination.join("skeleton.json")).unwrap()).unwrap();
    assert_eq!(data["format_version"], 2);
    assert!(data["animations"]["ai_breath"].is_object());
}

#[test]
#[ignore = "Validate the manually saved GUI copy via ANKHIMATE_AI_DEMO_PROJECT"]
fn jade_gui_saved_copy_has_native_keys_and_exports_without_asset_changes() {
    let path = std::path::PathBuf::from(std::env::var("ANKHIMATE_AI_DEMO_PROJECT").unwrap());
    // The manual Graph Editor check intentionally changes the middle key.
    // Supply its independently observed value; never derive the expected value
    // from the file under test or alter a project file to make it pass.
    let peak: f32 = std::env::var("ANKHIMATE_AI_DEMO_PEAK")
        .unwrap_or_else(|_| "1.03".into())
        .parse()
        .unwrap();
    assert!(peak.is_finite() && (1.0..=1.1).contains(&peak));
    let mut s = AppState::default();
    assert!(matches!(
        crate::fileops::open_path(&mut s, &path),
        crate::fileops::FileOutcome::Opened(_)
    ));
    let clip = s
        .doc
        .animations
        .values()
        .find(|a| a.name == "ai_breath")
        .unwrap();
    assert!(clip.looping);
    assert_eq!(clip.duration, 2.0);
    let track = clip
        .timelines
        .iter()
        .find(|t| matches!(t, ankhimate_core::animation::Timeline::BoneScale { .. }))
        .unwrap();
    if let ankhimate_core::animation::Timeline::BoneScale { bone, axis, keys } = track {
        assert_eq!(s.doc.skeleton.bones[*bone].name, "chest");
        assert_eq!(*axis, ankhimate_core::animation::Axis::Y);
        assert_eq!(
            keys.iter().map(|k| (k.time, k.value)).collect::<Vec<_>>(),
            vec![(0.0, 1.0), (1.0, peak), (2.0, 1.0)]
        );
    }
    let mut reference = AppState::default();
    crate::fileops::open_path(
        &mut reference,
        &path.parent().unwrap().join("Jade-AI-Breathing.ankh"),
    );
    let manual_reference = tools::EditPlan {
        summary: "Expected native manual middle-key edit".into(),
        actions: vec![tools::Action::Key {
            animation: "ai_breath".into(),
            bone: "chest".into(),
            channel: tools::Channel::ScaleY,
            time: 1.0,
            value: peak,
            interpolation: tools::Curve::Smooth,
        }],
    };
    tools::prepare(&reference, &manual_reference)
        .unwrap()
        .approve(&mut reference)
        .unwrap();
    assert_eq!(snapshot(&s), snapshot(&reference));
    assert_eq!(
        s.doc
            .assets
            .images
            .iter()
            .map(|(_, a)| &a.bytes)
            .collect::<Vec<_>>(),
        reference
            .doc
            .assets
            .images
            .iter()
            .map(|(_, a)| &a.bytes)
            .collect::<Vec<_>>()
    );
    let schema = ankhimate_formats::convert::to_schema(&s.doc.as_project_ref());
    let plan = ankhimate_export::run::plan(
        &schema,
        &s.doc.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let destination = path.parent().unwrap().join("GUI-native-export");
    assert!(!destination.exists());
    std::fs::create_dir(&destination).unwrap();
    ankhimate_export::run::write(&plan, &destination).unwrap();
    let json: Value =
        serde_json::from_slice(&std::fs::read(destination.join("skeleton.json")).unwrap()).unwrap();
    assert_eq!(json["format_version"], 2);
    assert!(json["animations"]["ai_breath"].is_object());
}

#[path = "t806_tests.rs"]
mod t806;

#[path = "t807_tests.rs"]
mod t807;

#[path = "t808_tests.rs"]
mod t808;

#[path = "t809_tests.rs"]
mod t809;
