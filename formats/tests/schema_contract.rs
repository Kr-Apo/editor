//! Contract documentation is reviewed data, not a second serializer.
use ankhimate_formats::{container, schema};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .into()
}
fn inventory() -> Value {
    serde_json::from_str(include_str!("../../docs/format-schema.json")).unwrap()
}
fn canonical(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .flat_map(|line| line.chars().filter(|c| !c.is_whitespace()))
        .collect()
}

#[test]
fn every_schema_field_and_variant_matches_the_documented_inventory() {
    let expected = inventory();
    let mut actual = json!({});
    let mut active: Option<String> = None;
    let mut variant: Option<String> = None;
    let mut attrs = Vec::<String>::new();
    for raw in include_str!("../src/schema.rs").lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with("#[derive") {
            continue;
        }
        if line.starts_with("#[") {
            attrs.push(line.into());
            continue;
        }
        if line.starts_with("pub struct ") || line.starts_with("pub enum ") {
            let parts: Vec<_> = line.split_whitespace().collect();
            active = Some(parts[2].into());
            actual[parts[2]] =
                json!({"kind":parts[1],"attributes":attrs,"fields":{},"variants":{}});
            attrs = Vec::new();
            continue;
        }
        let Some(name) = active.as_deref() else {
            attrs.clear();
            continue;
        };
        if line == "}" || line == "}," {
            if variant.take().is_none() {
                active = None;
            }
            attrs.clear();
            continue;
        }
        if let Some((field, ty)) = line
            .trim_start_matches("pub ")
            .strip_suffix(',')
            .unwrap_or(line)
            .split_once(": ")
        {
            let fields = match variant.as_deref() {
                Some(v) => &mut actual[name]["variants"][v]["fields"],
                None => &mut actual[name]["fields"],
            };
            fields[field] = json!({"type":ty,"attributes":attrs});
            attrs = Vec::new();
            continue;
        }
        assert_eq!(actual[name]["kind"], "enum", "unrecognized {name}: {line}");
        let label = line.trim_end_matches(',').trim_end_matches(" {");
        let (label, payload) = label
            .split_once('(')
            .map_or((label, None), |(a, b)| (a, Some(b.trim_end_matches(')'))));
        let mut item = json!({"attributes":attrs,"fields":{}});
        if let Some(p) = payload {
            item["payload"] = json!(p);
        }
        actual[name]["variants"][label] = item;
        variant = line.ends_with(" {").then(|| label.into());
        attrs = Vec::new();
    }
    // Per-field assertions name the declaration that changed, including serde defaults.
    for (name, ty) in expected["types"].as_object().unwrap() {
        assert_eq!(
            actual[name]["attributes"], ty["attributes"],
            "{name} serde container attributes"
        );
        for (field, definition) in ty["fields"].as_object().unwrap() {
            assert_eq!(actual[name]["fields"][field], *definition, "{name}.{field}");
        }
        for (v, definition) in ty["variants"].as_object().unwrap() {
            assert_eq!(
                actual[name]["variants"][v], *definition,
                "{name}.{v} variant/fields"
            );
        }
        assert_eq!(actual[name], *ty, "{name}: added/removed field or variant");
    }
    assert_eq!(actual, expected["types"], "added or removed schema type");
}

#[test]
fn normative_tables_cover_each_field_and_serde_attribute() {
    let spec = include_str!("../../docs/format-spec.md");
    let data = inventory();
    for (name, ty) in data["types"].as_object().unwrap() {
        for (scope, fields) in std::iter::once((name.clone(), &ty["fields"])).chain(
            ty["variants"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(v, x)| (format!("{name}.{v}"), &x["fields"])),
        ) {
            for (field, def) in fields.as_object().unwrap() {
                let prefix = format!(
                    "| {scope} | `{field}` | `{}` |",
                    def["type"].as_str().unwrap()
                );
                let row = spec
                    .lines()
                    .find(|l| l.starts_with(&prefix))
                    .unwrap_or_else(|| panic!("spec missing {scope}.{field}"));
                for attr in def["attributes"].as_array().unwrap() {
                    assert!(
                        row.contains(attr.as_str().unwrap()),
                        "spec missing {scope}.{field} serde rule {attr}"
                    );
                }
            }
        }
    }
    let digest = format!(
        "{:x}",
        Sha256::digest(canonical(include_str!("../src/schema.rs")))
    );
    assert_eq!(
        digest, data["schema_code_sha256"],
        "schema/default helper changed: review fields and default table"
    );
}

#[test]
fn compact_wire_tags_are_never_silently_reordered() {
    let src = include_str!("../src/compact.rs");
    let table = src
        .split("const FIELDS: &[&str] = &[")
        .nth(1)
        .unwrap()
        .split("];")
        .next()
        .unwrap();
    let tags: Vec<_> = table
        .lines()
        .filter_map(|l| l.trim().strip_prefix('"'))
        .map(|l| l.split('"').next().unwrap())
        .collect();
    assert_eq!(
        json!(tags),
        inventory()["compact_fields"],
        "compact wire tags changed; do not reorder/reuse published entries"
    );
}

fn version_gate(author: u32, envelope: u16, manifest: &Value) -> Result<(), String> {
    if manifest["author_version"] != author {
        return Err(format!(
            "author version {author} needs a format identity, fixture, spec and migration tests"
        ));
    }
    if manifest["container_version"] != envelope {
        return Err(format!(
            "envelope version {envelope} requires a fixture and specification"
        ));
    }
    let spec = include_str!("../../docs/format-spec.md");
    let anchor = manifest["spec_anchor"]
        .as_str()
        .ok_or("missing format identity")?;
    if anchor != format!("author-v{author}") || !spec.contains(&format!("id=\"{anchor}\"")) {
        return Err("missing current format specification anchor".into());
    }
    if !root()
        .join(
            manifest["migration_test"]
                .as_str()
                .ok_or("missing migration tests")?,
        )
        .is_file()
    {
        return Err("migration test source absent".into());
    }
    let fixtures = manifest["fixtures"].as_array().ok_or("missing fixtures")?;
    if !fixtures.iter().any(|f| f["version"] == author) {
        return Err("missing current-version reference fixture".into());
    }
    for f in fixtures {
        let path = f["path"].as_str().ok_or("missing fixture path")?;
        let bytes = fs::read(root().join(path)).map_err(|e| e.to_string())?;
        if format!("{:x}", Sha256::digest(&bytes)) != f["sha256"].as_str().unwrap() {
            return Err(format!("fixture changed: {path}"));
        }
        if !bytes.starts_with(container::MAGIC) || bytes[4..6] != envelope.to_le_bytes() {
            return Err(format!("wrong envelope in {path}"));
        }
        let (loaded, _) = ankhimate_formats::load(&root().join(path))
            .map_err(|e| format!("historical fixture unreadable {path}: {e}"))?;
        if loaded.name.is_empty() || f["source_commit"].as_str().unwrap_or("").len() != 40 {
            return Err(format!("missing representative data/provenance: {path}"));
        }
    }
    Ok(())
}

#[test]
fn current_versions_require_reference_fixtures_spec_and_migration_tests() {
    let path = std::env::var_os("ANKH_T605_VERSION_MANIFEST").map_or_else(
        || root().join("formats/tests/fixtures/format-versions.json"),
        PathBuf::from,
    );
    let manifest: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    version_gate(schema::CURRENT_VERSION, container::VERSION, &manifest).unwrap();
}

#[test]
fn future_author_or_envelope_version_and_missing_evidence_fail_closed() {
    let manifest: Value =
        serde_json::from_str(include_str!("fixtures/format-versions.json")).unwrap();
    assert!(
        version_gate(schema::CURRENT_VERSION + 1, container::VERSION, &manifest)
            .unwrap_err()
            .contains("author version")
    );
    assert!(
        version_gate(schema::CURRENT_VERSION, container::VERSION + 1, &manifest)
            .unwrap_err()
            .contains("envelope version")
    );
    for field in ["fixtures", "spec_anchor", "migration_test"] {
        let mut missing = manifest.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(
            version_gate(schema::CURRENT_VERSION, container::VERSION, &missing).is_err(),
            "missing {field}"
        );
    }
}

#[test]
fn compiled_serde_defaults_required_fields_and_curve_contract() {
    let p: schema::Project =
        serde_json::from_value(json!({"version":1,"name":"égyptien 🐸","fps":60})).unwrap();
    assert!(p.bones.is_empty() && p.export_presets.is_empty());
    for field in ["version", "name", "fps"] {
        let mut v = json!({"version":1,"name":"rig","fps":60});
        v.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<schema::Project>(v).is_err(),
            "Project.{field} must be required"
        );
    }
    let b: schema::Bone = serde_json::from_value(json!({"name":"root","length":10})).unwrap();
    assert_eq!((b.sx, b.sy, b.tx, b.rotation), (1., 1., 0., 0.));
    assert!(b.inherit_rotation && b.inherit_scale && b.inherit_reflect);
    let slot: schema::Slot = serde_json::from_value(json!({"name":"slot","bone":"root"})).unwrap();
    assert_eq!(slot.color, [1.; 4]);
    assert!(slot.dark_color.is_none());
    let a: schema::Animation = serde_json::from_value(json!({"name":"idle","duration":1})).unwrap();
    assert!(a.looping);
    let c: schema::Constraint =
        serde_json::from_value(json!({"name":"ik","type":"ik","target":"root"})).unwrap();
    assert_eq!((c.mix, c.bend_direction, c.stretch_limit), (1., 1., 1.1));
    assert!(
        serde_json::from_value::<schema::ScalarKey>(json!({"time":0,"value":1})).is_err(),
        "flatten Interp actually requires curve"
    );
    for curve in [
        json!({"curve":"linear"}),
        json!({"curve":"stepped"}),
        json!({"curve":"bezier","handles":[0.2,0.,0.8,1.]}),
    ] {
        let mut key = json!({"time":0,"value":10});
        key.as_object_mut()
            .unwrap()
            .extend(curve.as_object().unwrap().clone());
        let parsed: schema::ScalarKey = serde_json::from_value(key.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap()["curve"], key["curve"]);
    }
    assert!(
        serde_json::from_value::<schema::ScalarKey>(
            json!({"time":0,"value":"invalid","curve":"linear"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<schema::Slot>(json!({"name":"x","bone":"root","color":[1,1,1]}))
            .is_err()
    );
    assert!(serde_json::from_value::<schema::Timeline>(json!({"kind":"future_track"})).is_err());
}

#[test]
fn every_schema_type_and_enum_variant_is_exercised_by_compiled_serde() {
    fn check<T: serde::de::DeserializeOwned + serde::Serialize>(value: Value, name: &str) {
        let parsed: T = serde_json::from_value(value).unwrap_or_else(|e| panic!("{name}: {e}"));
        let encoded = serde_json::to_string(&parsed).unwrap();
        if name == "MixKey" {
            // Existing dual-flatten behavior: Extra also captures curve, producing
            // a duplicate on direct schema serialization. Document, do not fix here.
            assert_eq!(encoded.matches("\"curve\"").count(), 2);
            let error = serde_json::from_str::<T>(&encoded)
                .err()
                .expect("MixKey duplicate must be diagnosed");
            assert!(error.to_string().contains("duplicate field"));
        } else {
            serde_json::from_str::<T>(&encoded).unwrap_or_else(|e| panic!("{name} reopen: {e}"));
        }
    }
    macro_rules! dispatch {
        ($name:expr, $value:expr, $($ty:ident),+ $(,)?) => {
            match $name { $(stringify!($ty) => check::<schema::$ty>($value, $name),)+ other => panic!("uncovered schema type {other}") }
        };
    }
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/schema-cases.json")).unwrap();
    let data = inventory();
    for (name, ty) in data["types"].as_object().unwrap() {
        let matching: Vec<_> = cases.iter().filter(|c| c["type"] == *name).collect();
        assert_eq!(
            matching.len(),
            ty["variants"].as_object().unwrap().len().max(1),
            "{name}: missing schema/variant case"
        );
        if ty["kind"] == "enum" {
            let expected: std::collections::BTreeSet<_> = ty["variants"]
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            let found: std::collections::BTreeSet<_> = matching
                .iter()
                .map(|case| case["variant"].as_str().unwrap())
                .collect();
            assert_eq!(found, expected, "{name}: variant coverage");
        }
        for case in matching {
            dispatch!(
                name.as_str(),
                case["value"].clone(),
                Project,
                Group,
                Asset,
                Bone,
                Slot,
                Skin,
                SkinEntry,
                Attachment,
                BoundingBox,
                Point,
                Sequence,
                LinkedMesh,
                Path,
                Clipping,
                Region,
                Mesh,
                Constraint,
                Animation,
                BoneOffset,
                Marker,
                Event,
                Timeline,
                Interp,
                ScalarKey,
                Vec2Key,
                ColorKey,
                Axis,
                TransformMix,
                MixKey,
                VisibleKey,
                AttachmentKey,
                DrawOrderKey,
                DeformKey
            );
        }
    }
}

#[test]
fn native_projection_or_template_changes_require_contract_review() {
    for (path, expected) in inventory()["native_contract_sources"].as_object().unwrap() {
        let code = fs::read_to_string(root().join(path))
            .unwrap()
            .replace("\r\n", "\n");
        assert_eq!(
            format!("{:x}", Sha256::digest(code)),
            expected.as_str().unwrap(),
            "{path}: review native field tables, fixtures and compatibility tests before updating the inventory"
        );
    }
}
