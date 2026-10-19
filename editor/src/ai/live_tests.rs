//! Explicit opt-in only. Never probes or calls a cloud provider.
use super::*;

#[test]
#[ignore = "Requires the maintainer-authorized loopback Qwen server and an isolated Jade copy"]
fn ninfer_live_jade_conversation_tools_approval_and_verified_result() {
    let path = std::path::PathBuf::from(
        std::env::var("ANKHIMATE_AI_LIVE_PROJECT").expect("Set an isolated Jade copy"),
    );
    let proof = path.parent().unwrap().join("NInfer-live-proof.json");
    assert!(!proof.exists(), "Never overwrite live evidence");
    let mut s = AppState::default();
    assert!(matches!(
        crate::fileops::open_path(&mut s, &path),
        crate::fileops::FileOutcome::Opened(_)
    ));
    let before = snapshot(&s);
    let assets: Vec<_> = s
        .doc
        .assets
        .images
        .iter()
        .map(|(_, a)| a.bytes.clone())
        .collect();
    let mut p = Panel {
        settings: Settings {
            provider: Provider::NInfer,
            model: "qwen3.8-27b".into(),
            local_url: "http://127.0.0.1:8080/v1".into(),
            timeout_seconds: 120,
            max_tokens: 4096,
            tool_budget: 8,
            ..Default::default()
        },
        ..Default::default()
    };
    let ctx = egui::Context::default();
    let mut trace = vec![];
    fn finish(p: &mut Panel, s: &AppState, trace: &mut Vec<Value>) {
        let start = Instant::now();
        let mut previous = String::new();
        while p.job.is_some() {
            p.poll(s);
            let phase = p.activity.phase.label();
            if phase != previous {
                println!("Live NInfer: {phase}");
                trace.push(json!({"seconds":start.elapsed().as_secs_f32(),"phase":phase}));
                previous = phase;
            }
            assert!(start.elapsed() < Duration::from_secs(125));
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_ne!(p.activity.phase, Phase::Error, "{}", p.status);
    }
    p.start(&s, &ctx, true).unwrap();
    finish(&mut p, &s, &mut trace);
    assert!(p.models.iter().any(|m| m == "qwen3.8-27b"));
    p.input = "Bonjour. Réponds en une phrase française, sans outil.".into();
    p.start(&s, &ctx, false).unwrap();
    finish(&mut p, &s, &mut trace);
    assert!(
        p.display
            .iter()
            .any(|(r, t)| r == "Assistant" && !t.is_empty())
    );
    p.share_context = true;
    p.settings.tools_enabled = true;
    p.settings.editing = true;
    p.input="Examine cette copie de Jade Sentinel avec ankhimate_read section overview. Puis crée exclusivement une nouvelle respiration appelée ai_expert_live, durée 2 secondes, boucle vraie, sur le bone réel chest : scale_y aux temps 0:1, 1:1.03, 2:1, et rotate aux temps 0:0, 1:2, 2:0 ; interpolation smooth. Propose ces six opérations key.add après animation.create dans un seul lot, sans modifier les clips existants, puis attends l'approbation native.".into();
    p.start(&s, &ctx, false).unwrap();
    finish(&mut p, &s, &mut trace);
    assert_eq!(snapshot(&s), before);
    assert_eq!(p.activity.phase, Phase::AwaitingApproval, "{}", p.status);
    assert!(p.messages.iter().any(|m| m.role == "tool"
        && serde_json::from_str::<Value>(&m.text).is_ok_and(|v| v.get("bones").is_some())));
    let proposed = serde_json::to_value(&p.pending.as_ref().unwrap().edit.plan).unwrap();
    assert!(
        p.pending
            .as_ref()
            .unwrap()
            .edit
            .plan
            .actions
            .iter()
            .any(|a| matches!(a,tools::Action::Create{name,..}if name=="ai_expert_live"))
    );
    // Explicit test-user approval; production never calls this automatically.
    p.resolve(&mut s, true).unwrap();
    let after = snapshot(&s);
    assert_ne!(after, before);
    assert!(p.resolve(&mut s, true).is_err());
    assert_eq!(snapshot(&s), after);
    let receipt = p.last_result.clone().unwrap();
    assert_eq!(receipt["status"], "applied");
    let clip = s
        .doc
        .animations
        .values()
        .find(|a| a.name == "ai_expert_live")
        .unwrap();
    assert!(clip.looping);
    assert_eq!(clip.duration, 2.0);
    assert!(
        clip.timelines
            .iter()
            .map(|t| crate::app_state::key_times(t).len())
            .sum::<usize>()
            >= 6
    );
    p.input="Quel est le dernier résultat natif vérifié ? Confirme le nom du clip et son application uniquement si le résultat le prouve. Réponds brièvement, sans nouvel appel d'outil.".into();
    p.start(&s, &ctx, false).unwrap();
    finish(&mut p, &s, &mut trace);
    assert!(p.pending.is_none());
    assert_eq!(p.last_result.as_ref().unwrap()["status"], "applied");
    assert_eq!(snapshot(&s), after);
    for _ in 0..3 {
        s.undo();
        assert_eq!(snapshot(&s), before);
        s.redo();
        assert_eq!(snapshot(&s), after);
    }
    let output = path.parent().unwrap().join("Jade-Expert-Live.ankh");
    assert!(!output.exists());
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
        assets
    );
    let schema = ankhimate_formats::convert::to_schema(&reopened.doc.as_project_ref());
    let plan = ankhimate_export::run::plan(
        &schema,
        &reopened.doc.assets,
        &ankhimate_export::presets::default_preset(),
    )
    .unwrap();
    let destination = path.parent().unwrap().join("Native-export");
    assert!(!destination.exists());
    std::fs::create_dir(&destination).unwrap();
    ankhimate_export::run::write(&plan, &destination).unwrap();
    let json: Value =
        serde_json::from_slice(&std::fs::read(destination.join("skeleton.json")).unwrap()).unwrap();
    assert_eq!(json["format_version"], 2);
    assert!(json["animations"]["ai_expert_live"].is_object());
    std::fs::write(proof,serde_json::to_vec_pretty(&json!({"real_inference":true,"provider":"NInfer profile / OpenAI-compatible HTTP","model":p.settings.model,"endpoint":p.settings.local_url,"cloud_calls":0,"trace":trace,"proposed_plan":proposed,"native_receipt":receipt,"display":p.display,"undo_redo_cycles":3,"save_reopen_export":true})).unwrap()).unwrap();
}
