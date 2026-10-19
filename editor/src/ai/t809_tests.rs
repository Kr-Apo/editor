//! Synthetic pixels and local HTTP only: no inference or GUI.
use super::*;
fn image() -> vision::ImageInput {
    let source = egui::ColorImage::filled([4, 4], egui::Color32::GREEN);
    let spec = vision::FrameSpec {
        rect: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(4., 4.)),
        pixels_per_point: 1.,
        revision: 1,
        camera: Default::default(),
        playhead: 0.,
        setup: true,
    };
    vision::crop(&source, &spec).unwrap().1
}
#[test]
fn all_six_providers_encode_real_image_in_native_wire_contract() {
    for provider in Provider::ALL {
        let settings = Settings {
            provider,
            model: "synthetic-vision".into(),
            ..Default::default()
        };
        let mut m = Message::text("user", "Describe this pose");
        m.image = Some(image());
        let body = protocol::request(&settings, &[m], &[]);
        match provider {
            Provider::OpenAI => assert_eq!(
                body["input"][0]["content"][1]["type"], "input_image",
                "OpenAI image missing"
            ),
            Provider::Anthropic => assert_eq!(
                body["messages"][0]["content"][0]["source"]["media_type"],
                "image/png"
            ),
            Provider::Ollama => assert!(
                body["messages"][1]["images"][0]
                    .as_str()
                    .is_some_and(|s| !s.starts_with("data:"))
            ),
            _ => assert!(
                body["messages"][1]["content"][1]["image_url"]["url"]
                    .as_str()
                    .unwrap()
                    .starts_with("data:image/png;base64,")
            ),
        }
    }
}

fn settings(provider: Provider) -> Settings {
    let mut s = Settings {
        provider,
        model: "synthetic-only".into(),
        tools_enabled: true,
        editing: true,
        ..Default::default()
    };
    s.capabilities.destination = s.destination();
    s.capabilities.images = super::super::capabilities::Support::Supported;
    s
}
fn message() -> Message {
    let mut m = Message::text("user", "Observe then read torso and propose breathing");
    m.image = Some(image());
    m
}
fn call(provider: Provider, id: &str, name: &str, args: Value) -> String {
    let frame = |v: Value| format!("data: {v}\n\n");
    match provider {
        Provider::OpenAI => {
            let item = json!({"type":"function_call","call_id":id,"name":name,"arguments":args.to_string()});
            frame(json!({"type":"response.output_item.added","output_index":1,"item":item}))
                + &frame(
                    json!({"type":"response.function_call_arguments.delta","output_index":1,"delta":args.to_string()}),
                )
                + &frame(
                    json!({"type":"response.output_item.done","output_index":0,"item":{"type":"reasoning","id":"r1","encrypted_content":"opaque-completed"}}),
                )
                + &frame(json!({"type":"response.output_item.done","output_index":1,"item":item}))
                + &frame(json!({"type":"response.completed"}))
        }
        Provider::Anthropic => {
            frame(
                json!({"type":"content_block_start","index":0,"content_block":{"type":"thinking","thinking":"","signature":""}}),
            ) + &frame(
                json!({"type":"content_block_delta","index":0,"delta":{"type":"thinking_delta","thinking":"opaque-thought"}}),
            ) + &frame(
                json!({"type":"content_block_delta","index":0,"delta":{"type":"signature_delta","signature":"opaque-signature"}}),
            ) + &frame(
                json!({"type":"content_block_start","index":1,"content_block":{"type":"tool_use","id":id,"name":name,"input":{}}}),
            ) + &frame(
                json!({"type":"content_block_delta","index":1,"delta":{"type":"input_json_delta","partial_json":args.to_string()}}),
            ) + &frame(json!({"type":"message_stop"}))
        }
        Provider::Ollama => format!(
            "{}\n",
            json!({"message":{"content":"","thinking":"opaque-thought","tool_calls":[{"function":{"name":name,"arguments":args}}]},"done":true})
        ),
        _ => {
            frame(
                json!({"choices":[{"delta":{"reasoning_content":"opaque-thought","tool_calls":[{"index":0,"id":id,"function":{"name":name,"arguments":args.to_string()}}]}}]}),
            ) + "data: [DONE]\n\n"
        }
    }
}
fn poll(p: &mut Panel, s: &AppState) {
    let end = Instant::now() + Duration::from_secs(8);
    while p.job.is_some() {
        p.poll(s);
        assert!(Instant::now() < end);
        std::thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn six_real_http_image_read_proposal_approval_receipt_and_no_later_pixels() {
    for provider in Provider::ALL {
        for approve in [false, true] {
            let mut s = state();
            let before = snapshot(&s);
            let (base, rx) = server(vec![
                (
                    200,
                    call(
                        provider,
                        "read1",
                        "ankhimate_read",
                        json!({"section":"bone","bone":"torso"}),
                    ),
                    Duration::ZERO,
                ),
                (
                    200,
                    call(
                        provider,
                        "edit1",
                        "ankhimate_animate",
                        serde_json::to_value(breathing("torso")).unwrap(),
                    ),
                    Duration::ZERO,
                ),
                (
                    200,
                    text_stream(provider, "Native receipt acknowledged"),
                    Duration::ZERO,
                ),
            ]);
            let settings = settings(provider);
            let job = transport::start_at(
                settings.clone(),
                key(),
                vec![message()],
                Some(tools::snapshot(&s)),
                false,
                egui::Context::default(),
                base.clone(),
            );
            let mut p = Panel {
                settings,
                share_context: true,
                request_revision: s.revision,
                job: Some(job),
                ..Default::default()
            };
            poll(&mut p, &s);
            assert!(p.pending.is_some(), "{provider:?}: {}", p.status);
            assert_eq!(snapshot(&s), before);
            let first = rx.recv().unwrap();
            let second = rx.recv().unwrap();
            // The actual HTTP body, including protocol-specific binary envelope.
            match provider {
                Provider::OpenAI => assert_eq!(
                    first["input"][0]["content"][1]["image_url"],
                    image().data_url
                ),
                Provider::Anthropic => assert_eq!(
                    first["messages"][0]["content"][0]["source"]["data"],
                    image().base64()
                ),
                Provider::Ollama => assert_eq!(first["messages"][1]["images"][0], image().base64()),
                _ => assert_eq!(
                    first["messages"][1]["content"][1]["image_url"]["url"],
                    image().data_url
                ),
            }
            assert!(second.to_string().contains("torso"));
            assert!(second.to_string().contains(image().base64()));
            match provider {
                Provider::OpenAI => assert!(second.to_string().contains("opaque-completed")),
                Provider::Anthropic => assert!(second.to_string().contains("opaque-signature")),
                Provider::LmStudio => {}
                _ => assert!(second.to_string().contains("opaque-thought")),
            }
            assert!(p.messages.iter().all(|m| m.image.is_none()));
            let local = serde_json::to_string(&p.messages).unwrap();
            assert!(!local.contains(image().base64()));
            assert!(!format!("{:?}", p.display).contains(image().base64()));
            p.resolve(&mut s, approve).unwrap();
            assert!(p.resolve(&mut s, true).is_err());
            assert_eq!(snapshot(&s) == before, !approve);
            let result = p.verified_result(&s).unwrap();
            assert!(
                result
                    .text
                    .contains(if approve { "applied" } else { "refused" })
            );
            let mut next = p.messages.clone();
            next.push(result);
            next.push(Message::text("user", "Continue text only"));
            wait(transport::start_at(
                p.settings.clone(),
                key(),
                next,
                Some(tools::snapshot(&s)),
                false,
                egui::Context::default(),
                base,
            ))
            .unwrap();
            let followup = rx.recv().unwrap().to_string();
            assert!(!followup.contains(image().base64()));
            assert!(followup.contains(if approve { "applied" } else { "refused" }));
            if approve {
                let after = snapshot(&s);
                for _ in 0..3 {
                    s.undo();
                    assert_eq!(snapshot(&s), before);
                    s.redo();
                    assert_eq!(snapshot(&s), after);
                }
            }
        }
    }
}
#[test]
fn six_http_image_text_responses_do_not_certify_models() {
    for provider in Provider::ALL {
        let (base, rx) = server(vec![(
            200,
            text_stream(provider, "synthetic answer 🐸"),
            Duration::ZERO,
        )]);
        let mut settings = Settings {
            provider,
            model: "unknown".into(),
            ..Default::default()
        };
        let mut m = message();
        m.image.as_mut().unwrap().try_unknown = true;
        let turn = wait(transport::start_at(
            settings.clone(),
            key(),
            vec![m],
            None,
            false,
            egui::Context::default(),
            base,
        ))
        .unwrap();
        assert_eq!(turn.messages.last().unwrap().text, "synthetic answer 🐸");
        rx.recv().unwrap();
        assert_eq!(
            settings.capabilities().images,
            super::super::capabilities::Support::Unknown
        );
        settings.capabilities.destination = settings.destination();
        settings.capabilities.images = super::super::capabilities::Support::Unsupported;
        let error = wait(transport::start_at(
            settings,
            key(),
            vec![message()],
            None,
            false,
            egui::Context::default(),
            "http://127.0.0.1:1".into(),
        ))
        .err()
        .unwrap();
        assert!(error.contains("incompatible"));
    }
}
#[test]
fn unknown_requires_choice_and_capabilities_never_leak_between_destinations() {
    use super::super::capabilities::Support;
    let mut s = Settings {
        model: "one".into(),
        ..Default::default()
    };
    assert!(s.authorize_image(&image()).unwrap_err().contains("unknown"));
    let mut img = image();
    img.try_unknown = true;
    assert!(s.authorize_image(&img).is_ok());
    s.capabilities.destination = s.destination();
    s.capabilities.images = Support::Supported;
    assert!(s.authorize_image(&image()).is_ok());
    for changed in [
        Settings {
            model: "two".into(),
            ..s.clone()
        },
        Settings {
            local_url: "http://127.0.0.1:9999".into(),
            ..s.clone()
        },
        Settings {
            provider: Provider::NInfer,
            ..s.clone()
        },
    ] {
        assert_eq!(changed.capabilities().images, Support::Unknown);
        assert!(changed.authorize_image(&image()).is_err());
    }
    s.capabilities.images = Support::Unsupported;
    assert!(s.authorize_image(&img).is_err());
    s.capabilities.tools = Support::Unsupported;
    s.tools_enabled = true;
    assert!(s.validate().unwrap_err().contains("tools"));
    s.tools_enabled = false;
    assert!(s.validate().is_ok());
    s.capabilities.streaming = Support::Unsupported;
    assert!(s.validate().unwrap_err().contains("streaming"));
}
#[test]
fn image_quality_is_deterministic_bounded_and_preview_matches_png() {
    use base64::Engine;
    let source = egui::ColorImage::filled(
        [2048, 1024],
        egui::Color32::from_rgba_unmultiplied(35, 87, 145, 123),
    );
    let spec = vision::FrameSpec {
        rect: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(2048., 1024.)),
        pixels_per_point: 1.,
        revision: 1,
        camera: Default::default(),
        playhead: 0.,
        setup: true,
    };
    for q in vision::Quality::ALL {
        let (preview, input) = vision::crop_with_quality(&source, &spec, q).unwrap();
        input.validate().unwrap();
        assert_eq!(preview.size, [q.side() as usize, q.side() as usize / 2]);
        assert_eq!(
            input.data_url,
            vision::crop_with_quality(&source, &spec, q)
                .unwrap()
                .1
                .data_url
        );
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(input.base64())
            .unwrap();
        let decoded = image::load_from_memory(&bytes).unwrap().to_rgba8();
        assert_eq!(
            preview,
            egui::ColorImage::from_rgba_unmultiplied(preview.size, decoded.as_raw())
        );
        assert!(!input.try_unknown);
    }
}
#[test]
fn invalid_oversized_non_user_images_fail_before_network() {
    let mut cases = vec![];
    let mut bad = image();
    bad.data_url = "file:///private/path.png".into();
    cases.push(bad);
    let mut bad = image();
    bad.data_url = "data:image/png;base64,invalid!".into();
    cases.push(bad);
    let mut bad = image();
    bad.mapping["width"] = json!(9999);
    cases.push(bad);
    let mut bad = image();
    bad.data_url.push_str(&"A".repeat(vision::MAX_BYTES * 2));
    cases.push(bad);
    let mut bad = image();
    bad.max_requests = 18;
    cases.push(bad);
    for bad in cases {
        let mut m = message();
        m.image = Some(bad);
        let error = wait(transport::start_at(
            settings(Provider::Ollama),
            key(),
            vec![m],
            None,
            false,
            egui::Context::default(),
            "http://127.0.0.1:1".into(),
        ))
        .err()
        .unwrap();
        assert!(!error.contains("connect"));
        assert!(!error.contains("private/path"));
    }
    let mut m = message();
    m.role = "assistant".into();
    assert!(
        wait(transport::start_at(
            settings(Provider::Ollama),
            key(),
            vec![m],
            None,
            false,
            egui::Context::default(),
            "http://127.0.0.1:1".into()
        ))
        .err()
        .unwrap()
        .contains("user")
    );
}
#[test]
fn image_budget_prevents_automatic_continuation_and_cached_reads_keep_context() {
    for budget in [1, 3] {
        let (base, rx) = server(vec![
            (
                200,
                call(
                    Provider::NInfer,
                    "r1",
                    "ankhimate_read",
                    json!({"section":"bones","offset":0,"limit":1}),
                ),
                Duration::ZERO,
            ),
            (
                200,
                call(
                    Provider::NInfer,
                    "r2",
                    "ankhimate_read",
                    json!({"section":"bones","offset":0,"limit":1}),
                ),
                Duration::ZERO,
            ),
            (
                200,
                text_stream(Provider::NInfer, "Read retained"),
                Duration::ZERO,
            ),
        ]);
        let mut settings = settings(Provider::NInfer);
        settings.image_requests = budget;
        let mut m = message();
        m.image.as_mut().unwrap().max_requests = 3;
        let result = wait(transport::start_at(
            settings,
            key(),
            vec![m],
            Some(tools::snapshot(&state())),
            false,
            egui::Context::default(),
            base,
        ));
        rx.recv().unwrap();
        if budget == 1 {
            assert!(result.err().unwrap().contains("image-request budget"));
            assert!(rx.try_recv().is_err());
        } else {
            result.unwrap();
            let second = rx.recv().unwrap();
            let third = rx.recv().unwrap();
            assert!(second.to_string().contains("next_offset"));
            let messages = third["messages"].as_array().unwrap();
            let results: Vec<_> = messages.iter().filter(|m| m["role"] == "tool").collect();
            assert_eq!(results.len(), 2);
            let cached: Value =
                serde_json::from_str(results[1]["content"].as_str().unwrap()).unwrap();
            assert_eq!(cached["cache_hit"], true);
            assert_eq!(cached["source_call_id"], "r1");
            assert!(!results[1]["content"].as_str().unwrap().contains("torso"));
        }
    }
}
#[test]
fn multimodal_errors_are_precise_sanitized_and_never_retry() {
    for provider in Provider::ALL {
        for (status, body, expected) in [
            (
                401,
                "private-path mock-key-not-a-real-credential".into(),
                "401/403",
            ),
            (429, "private-path".into(), "429"),
            (
                400,
                json!({"error":{"code":"context_length_exceeded","message":"private-path"}})
                    .to_string(),
                "context window",
            ),
            (422, "private-path".into(), "text-only"),
            (200, "".into(), "before completion"),
        ] {
            let (base, rx) = server(vec![(status, body, Duration::ZERO)]);
            let error = wait(transport::start_at(
                settings(provider),
                key(),
                vec![message()],
                None,
                false,
                egui::Context::default(),
                base,
            ))
            .err()
            .unwrap();
            assert!(error.contains(expected), "{provider:?}: {error}");
            assert!(!error.contains("private-path") && !error.contains("mock-key"));
            rx.recv().unwrap();
            assert!(rx.try_recv().is_err());
        }
    }
}
#[test]
fn output_limits_filter_and_malformed_stream_are_not_conflated() {
    for (provider, body, expected) in [
        (
            Provider::OpenAI,
            json!({"type":"response.incomplete","response":{"incomplete_details":{"reason":"max_output_tokens"}}}),
            "Output token",
        ),
        (
            Provider::OpenAI,
            json!({"type":"response.incomplete","response":{"incomplete_details":{"reason":"content_filter"}}}),
            "filter",
        ),
        (
            Provider::Anthropic,
            json!({"type":"message_delta","delta":{"stop_reason":"max_tokens"}}),
            "Output limit",
        ),
        (
            Provider::Ollama,
            json!({"done":true,"done_reason":"length"}),
            "Output limit",
        ),
        (
            Provider::DeepSeek,
            json!({"choices":[{"finish_reason":"length"}]}),
            "Output token",
        ),
        (
            Provider::NInfer,
            json!({"choices":[{"finish_reason":"content_filter"}]}),
            "filter",
        ),
    ] {
        assert!(
            protocol::Decoder::default()
                .frame(provider, &body.to_string())
                .unwrap_err()
                .contains(expected)
        );
    }
    assert!(
        protocol::Decoder::default()
            .frame(Provider::OpenAI, "bad JSON")
            .unwrap_err()
            .contains("Malformed")
    );
}
#[test]
fn settings_migrate_budgets_keep_all_tool_schemas_and_prioritize_business_guidance() {
    let old: Settings = serde_json::from_value(json!({"model":"old","provider":"NInfer"})).unwrap();
    assert_eq!(old.context_kib, 256);
    let mut s = old.clone();
    s.profile(true);
    s.validate().unwrap();
    let prompt = "Observe mesh weights and smooth influences";
    let guide = manual::select_with_budget(prompt, true, s.manual_bytes);
    assert!(guide.text.len() <= 3072);
    assert!(guide.sections.contains(&"WEIGHTS"));
    assert!(!guide.sections.contains(&"TOOLS")); // exact schemas are transmitted independently
    let defs = tools::definitions(true);
    let body = protocol::request(&s, &[Message::text("user", prompt)], &defs);
    assert_eq!(body["tools"].as_array().unwrap().len(), defs.len());
    assert!(body.to_string().contains("mesh.smooth_weights"));
    let mut s = settings(Provider::NInfer);
    s.context_kib = 32;
    let (base, rx) = server(vec![(
        200,
        text_stream(Provider::NInfer, "unused"),
        Duration::ZERO,
    )]);
    let error = wait(transport::start_at(
        s,
        key(),
        vec![Message::text("user", "a".repeat(40 * 1024))],
        None,
        false,
        egui::Context::default(),
        base,
    ))
    .err()
    .unwrap();
    assert!(error.contains("Context"));
    assert!(rx.try_recv().is_err());
}

#[test]
fn debug_and_serialization_never_log_png_or_permission() {
    let m = message();
    let debug = format!("{m:?}");
    assert!(debug.contains("redacted ephemeral PNG"));
    assert!(!debug.contains(image().base64()));
    let serialized = serde_json::to_string(&m).unwrap();
    assert!(!serialized.contains(image().base64()));
    assert!(!serialized.contains("try_unknown"));
}
#[test]
fn unknown_missing_revoked_expired_or_wrong_destination_permission_sends_nothing() {
    let s = state();
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            model: "unqualified".into(),
            local_url: "http://127.0.0.1:1".into(),
            ..Default::default()
        },
        input: "Observe".into(),
        ..Default::default()
    };
    let (_, input) = vision::crop(
        &egui::ColorImage::filled([4, 4], egui::Color32::GREEN),
        &vision::FrameSpec {
            rect: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(4., 4.)),
            pixels_per_point: 1.,
            revision: s.revision,
            camera: Default::default(),
            playhead: 0.,
            setup: true,
        },
    )
    .unwrap();
    let preview = || vision::Preview {
        image: egui::ColorImage::filled([4, 4], egui::Color32::GREEN),
        input: input.clone(),
        revision: s.revision,
        route: p.image_route(),
        approved: true,
        created: Instant::now(),
        texture: None,
    };
    p.vision.preview = Some(preview());
    assert!(
        p.start(&s, &egui::Context::default(), false)
            .unwrap_err()
            .contains("unknown")
    );
    assert!(p.job.is_none());
    p.vision.preview.as_mut().unwrap().input.try_unknown = true;
    p.vision.preview.as_mut().unwrap().approved = false;
    assert!(p.start(&s, &egui::Context::default(), false).is_err());
    p.vision.preview.as_mut().unwrap().approved = true;
    p.vision.preview.as_mut().unwrap().created = Instant::now() - Duration::from_secs(121);
    assert!(
        p.start(&s, &egui::Context::default(), false)
            .unwrap_err()
            .contains("stale")
    );
    p.vision.preview.as_mut().unwrap().created = Instant::now();
    p.settings.model = "changed".into();
    assert!(
        p.start(&s, &egui::Context::default(), false)
            .unwrap_err()
            .contains("stale")
    );
    p.settings.model = "unqualified".into();
    p.settings.local_url = "http://127.0.0.1:9999".into();
    assert!(
        p.start(&s, &egui::Context::default(), false)
            .unwrap_err()
            .contains("stale")
    );
    assert!(p.job.is_none());
}
#[test]
fn image_request_rejection_preserves_existing_conversation_and_restores_draft() {
    let s = state();
    let (base, rx) = server(vec![(
        400,
        "untrusted private server diagnostic".into(),
        Duration::ZERO,
    )]);
    let mut settings = settings(Provider::NInfer);
    settings.local_url = base.clone();
    settings.capabilities.destination = settings.destination();
    let mut p = Panel {
        settings,
        request_revision: s.revision,
        messages: vec![Message::text("user", "Earlier useful conversation")],
        input: "".into(),
        in_flight_prompt: Some("Observe".into()),
        ..Default::default()
    };
    let before = serde_json::to_value(&p.messages).unwrap();
    let mut messages = p.messages.clone();
    messages.push(message());
    p.job = Some(transport::start_at(
        p.settings.clone(),
        key(),
        messages,
        None,
        false,
        egui::Context::default(),
        base,
    ));
    poll(&mut p, &s);
    assert_eq!(serde_json::to_value(&p.messages).unwrap(), before);
    assert_eq!(p.input, "Observe");
    assert!(p.status.contains("text-only"));
    assert!(!format!("{:?}", p.display).contains("private server"));
    rx.recv().unwrap();
}
#[test]
fn every_provider_multimodal_stop_and_timeout_are_bounded() {
    for provider in Provider::ALL {
        for stop in [false, true] {
            let (base, rx) = server(vec![(
                200,
                text_stream(provider, "too late"),
                Duration::from_millis(1200),
            )]);
            let mut settings = settings(provider);
            settings.timeout_seconds = 1;
            let mut job = transport::start_at(
                settings,
                key(),
                vec![message()],
                None,
                false,
                egui::Context::default(),
                base,
            );
            rx.recv_timeout(Duration::from_secs(3)).unwrap();
            if stop {
                job.stop();
            }
            let error = wait(job).err().unwrap();
            assert!(error.contains(if stop { "Stopped" } else { "timed out" }));
        }
    }
}

#[test]
fn destination_binding_is_unambiguous_with_delimiters_in_endpoint_or_model() {
    let a = Settings {
        provider: Provider::NInfer,
        model: "c".into(),
        local_url: "http://127.0.0.1:8080/a|b".into(),
        ..Default::default()
    };
    let b = Settings {
        model: "b|c".into(),
        local_url: "http://127.0.0.1:8080/a".into(),
        ..a.clone()
    };
    assert_ne!(a.base_url().unwrap(), b.base_url().unwrap());
    assert_ne!(
        a.destination(),
        b.destination(),
        "Distinct endpoint/model tuples must never share image permission or capability declarations"
    );
}
