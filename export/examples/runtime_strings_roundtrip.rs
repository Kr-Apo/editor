//! M001b: actual native JSON strings for the TypeScript reader's round-trip gate.
//! Prints references to stdout; never writes files or authored projects.
use ankhimate_export::{
    context::{Context, ExportInfo},
    presets,
    template::Engine,
};
use ankhimate_formats::schema::Project;
use serde_json::{Value, json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let controls: String = (0..=31).map(char::from).collect();
    let suffix = format!("\"\\ é漢字🦴\u{2028}\u{2029} &<>'/:*?{controls}");
    let name = format!("rig{suffix}");
    let bone = format!("bone{suffix}");
    let slot = format!("slot{suffix}");
    let skin = format!("skin{suffix}");
    let attachment = format!("attachment{suffix}");
    let texture = format!("texture{suffix}");
    let event = format!("event{suffix}");
    let audio = format!("audio{suffix}");
    let mut cases = Vec::new();
    for animation in [
        format!("clip{suffix}"),
        "__proto__".into(),
        "constructor".into(),
        "toString".into(),
    ] {
        let project: Project = serde_json::from_value(json!({
            "version": 1, "name": name, "fps": 60,
            "bones": [{"name": bone, "length": 0, "tx": 0, "ty": 0, "rotation": 0, "sx": 1, "sy": 1}],
            "slots": [{"name": slot, "bone": bone, "attachment": attachment}],
            "skins": [{"name": skin, "entries": [{"slot": slot, "name": attachment, "attachment": {"type": "region", "texture": texture, "width": 16, "height": 16}}]}],
            "default_skin": skin,
            "animations": [{"name": animation, "duration": 1, "looping": false,
                "timelines": [{"kind": "bone_translate", "bone": bone, "axis": "x", "keys": [{"time": 0, "value": 0, "curve": "linear"}, {"time": 1, "value": 10, "curve": "linear"}]}],
                "events": [{"time": 0.25, "name": event, "string_value": suffix, "audio": audio}]
            }]
        }))?;
        let context = Context::build(&project, None, ExportInfo::default());
        let text = Engine::new().render(
            "native strings roundtrip",
            &presets::default_preset().templates[0].body,
            &context.to_value(),
        )?;
        let exported: Value = serde_json::from_str(&text)?;
        assert_eq!(exported["name"], name);
        assert_eq!(
            exported["animations"][&animation]["events"][0]["string"],
            suffix
        );
        cases.push(json!({"text": text, "expected": {"name": name, "bone": bone, "slot": slot, "skin": skin, "attachment": attachment, "texture": texture, "animation": animation, "event": event, "payload": suffix, "audio": audio}}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"source": "editor/export/examples/runtime_strings_roundtrip.rs", "cases": cases})
        )?
    );
    Ok(())
}
