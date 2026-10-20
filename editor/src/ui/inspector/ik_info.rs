//! Read-only IK presentation. Evaluation and authoring commands remain unchanged.
use crate::app_state::AppState;
use ankhimate_core::constraints::IkConstraint;
pub(super) fn summary(
    state: &AppState,
    ik: &IkConstraint,
) -> Result<Vec<(&'static str, String)>, String> {
    let skel = &state.doc.skeleton;
    let Some(&root) = ik.bones.first() else {
        return Err("Invalid IK: empty chain".into());
    };
    if !skel.bones.contains_key(ik.target) {
        return Err("Invalid IK: missing target".into());
    }
    if ik.bones.contains(&ik.target) {
        return Err("Invalid IK: target belongs to the chain".into());
    }
    for (i, bone) in ik.bones.iter().enumerate() {
        if !skel.bones.contains_key(*bone) {
            return Err("Invalid IK: missing chain bone".into());
        }
        if ik.bones[..i].contains(bone) {
            return Err("Invalid IK: repeated chain bone".into());
        }
        if i > 0 && !skel.is_descendant(*bone, ik.bones[i - 1]) {
            return Err(
                "Invalid IK presentation: chain must be ordered ancestor to descendant".into(),
            );
        }
        if !state.pose.worlds.contains_key(*bone) {
            return Err("IK span unavailable: pose not evaluated".into());
        }
    }
    // The analytic/FABRIK joints may skip an authored bone. Sum actual gaps,
    // not authored lengths. This is the current evaluated span, not a predicted
    // maximum reach or the core's separate softness/stretch natural-reach input.
    let joints: Vec<_> = ik
        .bones
        .iter()
        .map(|b| state.pose.world_position(*b))
        .chain(std::iter::once(
            state.pose.world_tip(skel, *ik.bones.last().unwrap()),
        ))
        .collect();
    let lengths: Vec<_> = joints.windows(2).map(|p| (p[1] - p[0]).length()).collect();
    if lengths.iter().any(|n| !n.is_finite() || *n <= 1e-6) {
        return Err("IK span unavailable: non-finite or zero-length segment".into());
    }
    let span: f32 = lengths.iter().sum();
    if !span.is_finite() {
        return Err("IK span unavailable: non-finite total".into());
    }
    Ok(vec![
        ("Chain root", skel.bones[root].name.clone()),
        (
            "Chain order",
            ik.bones
                .iter()
                .map(|b| skel.bones[*b].name.as_str())
                .collect::<Vec<_>>()
                .join(" → "),
        ),
        (
            "Solver",
            match ik.bones.len() {
                1 => "Aim (1 bone)",
                2 => "Analytic (2 bones)",
                _ => "FABRIK (3+ bones)",
            }
            .into(),
        ),
        (
            "Evaluated span",
            format!("{span:.3} world units (current pose)"),
        ),
    ])
}
