//! Fail closed before converting author data that the editable model would drop.
//! Serde remains the field authority; this is a reader guard, not another codec.
use crate::{Error, schema};
use serde_json::Value;

pub(crate) fn validate(raw: &Value, project: &schema::Project) -> Result<(), Error> {
    for (i, slot) in project.slots.iter().enumerate() {
        if !matches!(
            slot.blend_mode.as_str(),
            "" | "normal" | "additive" | "multiply" | "screen"
        ) {
            return Err(Error::UnsupportedMember {
                path: format!("/slots/{i}/blend_mode"),
            });
        }
    }
    for (i, constraint) in project.constraints.iter().enumerate() {
        if !matches!(
            constraint.kind.as_str(),
            "ik" | "transform" | "physics" | "path"
        ) {
            return Err(Error::UnsupportedMember {
                path: format!("/constraints/{i}/type"),
            });
        }
    }
    let mut known = project.clone();
    known.extra.clear();
    for v in &mut known.assets {
        v.extra.clear();
    }
    for v in &mut known.bones {
        v.extra.clear();
    }
    for v in &mut known.slots {
        v.extra.clear();
    }
    for v in &mut known.constraints {
        v.extra.clear();
    }
    for (skin_index, skin) in known.skins.iter_mut().enumerate() {
        skin.extra.clear();
        for (entry_index, entry) in skin.entries.iter_mut().enumerate() {
            let sequence = match &entry.attachment {
                schema::Attachment::Region(v) => v.sequence.as_ref(),
                schema::Attachment::Mesh(v) => v.sequence.as_ref(),
                _ => None,
            };
            if sequence.is_some_and(|v| {
                !matches!(
                    v.mode.as_str(),
                    "" | "hold"
                        | "once"
                        | "loop"
                        | "ping_pong"
                        | "once_reverse"
                        | "loop_reverse"
                        | "ping_pong_reverse"
                )
            }) {
                return Err(Error::UnsupportedMember {
                    path: format!(
                        "/skins/{skin_index}/entries/{entry_index}/attachment/sequence/mode"
                    ),
                });
            }
            match &mut entry.attachment {
                schema::Attachment::Region(v) => v.extra.clear(),
                schema::Attachment::Mesh(v) => v.extra.clear(),
                schema::Attachment::Clipping(v) => v.extra.clear(),
                schema::Attachment::Path(v) => v.extra.clear(),
                schema::Attachment::BoundingBox(v) => v.extra.clear(),
                schema::Attachment::Point(v) => v.extra.clear(),
            }
        }
    }
    for animation in &mut known.animations {
        animation.extra.clear();
        for timeline in &mut animation.timelines {
            match timeline {
                schema::Timeline::BoneTranslate { extra, .. }
                | schema::Timeline::BoneScale { extra, .. }
                | schema::Timeline::BoneShear { extra, .. } => extra.clear(),
                schema::Timeline::TransformConstraintMix { keys, .. } => {
                    for key in keys {
                        key.extra.clear();
                    }
                }
                _ => {}
            }
        }
    }
    let shape = serde_json::to_value(known)?;
    if let Some(path) = unknown(raw, &shape, "", raw) {
        return Err(Error::UnsupportedMember { path });
    }
    Ok(())
}

fn unknown(raw: &Value, known: &Value, path: &str, root: &Value) -> Option<String> {
    // Opaque export presets deliberately have no author-schema interpretation.
    if path == "/export_presets" {
        return None;
    }
    match (raw, known) {
        (Value::Object(input), Value::Object(shape)) => {
            for (key, value) in input {
                let child = format!("{path}/{}", key.replace('~', "~0").replace('/', "~1"));
                if let Some(expected) = shape.get(key) {
                    if let Some(found) = unknown(value, expected, &child, root) {
                        return Some(found);
                    }
                } else if !omitted_default(path, key, value, root) {
                    return Some(child);
                }
            }
        }
        (Value::Array(input), Value::Array(shape)) => {
            for (index, (value, expected)) in input.iter().zip(shape).enumerate() {
                if let Some(found) = unknown(value, expected, &format!("{path}/{index}"), root) {
                    return Some(found);
                }
            }
        }
        _ => {}
    }
    None
}

// Only declared skip_serializing_if defaults may disappear during typed Serde.
// Paths are scoped: e.g. an unknown `events: []` on a bone is still rejected.
fn omitted_default(path: &str, key: &str, value: &Value, root: &Value) -> bool {
    let scope = path
        .split('/')
        .map(|part| {
            if part.parse::<usize>().is_ok() {
                "*"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join("/");
    let empty_array = value.as_array().is_some_and(Vec::is_empty);
    let empty_string = value.as_str() == Some("");
    let zero = value.as_f64() == Some(0.0);
    match scope.as_str() {
        "" => {
            (matches!(key, "groups" | "export_presets") && empty_array)
                || (key == "psd_layer_paths"
                    && value.as_object().is_some_and(serde_json::Map::is_empty))
        }
        "/groups/*" => key == "parent" && empty_string,
        "/skins/*" => matches!(key, "bones" | "constraints") && empty_array,
        "/skins/*/entries/*/attachment" => {
            let kind = root
                .pointer(&format!("{path}/type"))
                .and_then(Value::as_str);
            (key == "edges" && kind == Some("mesh") && empty_array)
                || (key == "weights" && kind == Some("boundingbox") && empty_array)
                || (key == "linked" && kind == Some("mesh") && value.is_null())
                || (key == "sequence" && matches!(kind, Some("mesh" | "region")) && value.is_null())
        }
        "/skins/*/entries/*/attachment/linked" => key == "skin" && value.is_null(),
        "/constraints/*" => {
            (matches!(
                key,
                "transform_mix" | "offsets" | "physics" | "forces" | "channels" | "slot" | "path"
            ) && value.is_null())
                || (matches!(key, "local" | "relative") && value.as_bool() == Some(false))
        }
        "/constraints/*/transform_mix" => mix_channel(key) && zero,
        "/animations/*" => matches!(key, "events" | "markers" | "bone_offsets") && empty_array,
        "/animations/*/events/*" => {
            (matches!(key, "int_value" | "float_value" | "balance") && zero)
                || (key == "volume" && value.as_f64() == Some(1.0))
                || (matches!(key, "string_value" | "audio") && empty_string)
        }
        "/animations/*/timelines/*/keys/*" => {
            let timeline = path
                .rsplit_once("/keys/")
                .map(|(prefix, _)| prefix)
                .unwrap_or("");
            root.pointer(&format!("{timeline}/kind"))
                .and_then(Value::as_str)
                == Some("transform_constraint_mix")
                && mix_channel(key)
                && zero
        }
        _ => false,
    }
}
fn mix_channel(key: &str) -> bool {
    matches!(
        key,
        "rotate" | "translate_x" | "translate_y" | "scale_x" | "scale_y" | "shear_x" | "shear_y"
    )
}
