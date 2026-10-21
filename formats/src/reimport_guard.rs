//! Fail-closed gate for the PSD reimport workflow, not a format migration.
use serde_json::Value;
use std::path::Path;

/// Refuse a source project whose supplied fields/values cannot survive the current
/// loaded model. Comparison is deliberately conservative (array order included).
/// It does not recover fields already lost by an earlier save through another path.
pub fn verify_source(path: &Path) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("Cannot verify project source: {e}"))?;
    let raw: Value = if bytes.starts_with(crate::container::MAGIC) {
        let payload = crate::container::decode(&bytes).map_err(|e| e.to_string())?;
        crate::compact::expand(
            rmp_serde::from_slice(&payload).map_err(|e| format!("Project payload: {e}"))?,
        )
    } else {
        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|e| format!("Native author project required for safe reimport: {e}"))?;
        if value.get("version").is_some() {
            value
        } else {
            crate::compact::expand(value)
        }
    };
    let (p, _) = crate::load(path).map_err(|e| e.to_string())?;
    if !p.report.dangling.is_empty() || !p.report.lossy.is_empty() {
        return Err("Project has unresolved or lossy data; reimport refused".into());
    }
    let view = crate::ProjectRef {
        animation_order: &[],
        skeleton: &p.skeleton,
        animations: &p.animations,
        assets: &p.assets,
        name: &p.name,
        fps: p.fps,
        export_presets: &p.export_presets,
        psd_layer_paths: &p.psd_layer_paths,
    };
    let roundtrip =
        serde_json::to_value(crate::convert::to_schema(&view)).map_err(|e| e.to_string())?;
    fn supplied(a: &Value, b: &Value, at: &str) -> Result<(), String> {
        match (a, b) {
            (Value::Object(x), Value::Object(y)) => {
                for (key, v) in x {
                    let next = format!("{at}.{key}");
                    supplied(
                        v,
                        y.get(key).ok_or_else(|| {
                            format!("{next}: field not retained by loaded model; reimport refused")
                        })?,
                        &next,
                    )?;
                }
            }
            (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
                for (i, (a, b)) in x.iter().zip(y).enumerate() {
                    supplied(a, b, &format!("{at}[{i}]"))?;
                }
            }
            (Value::Number(x), Value::Number(y))
                if (x.as_f64().unwrap() - y.as_f64().unwrap()).abs() <= 0.0001 => {}
            _ if a == b => {}
            _ => {
                return Err(format!(
                    "{at}: data/order not preserved by current model; reimport refused"
                ));
            }
        }
        Ok(())
    }
    supplied(&raw, &roundtrip, "project")
}
