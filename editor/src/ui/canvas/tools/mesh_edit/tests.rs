use super::*;
use crate::{app_state::AppState, session::Session};
use ankhimate_core::{
    animation::{Animation, Key, Timeline},
    attachment::{Rect, RegionAttachment, VertexWeight},
    math::Transform,
    skeleton::Bone,
    slot::Slot,
};
use ankhimate_document::{WorkMode, commands::History};
use glam::{Vec2, vec2};

fn bone(name: &str, transform: Transform) -> Bone {
    Bone {
        name: name.into(),
        parent: None,
        length: 1.0,
        local_transform: transform,
        inherit: Default::default(),
        color: Bone::default_color(),
    }
}

fn fixture(weighted: bool, initial: Vec2) -> AppState {
    let mut state = AppState::default();
    let root = state
        .doc
        .skeleton
        .add_bone(bone("root", Transform::default()));
    let a = state.doc.skeleton.add_bone(bone("a", Transform::default()));
    let b = state.doc.skeleton.add_bone(bone("b", Transform::default()));
    let slot = state.doc.skeleton.add_slot(Slot {
        attachment: Some("art".into()),
        ..Slot::new("art".into(), root)
    });
    let mut mesh = MeshAttachment::from_region(&RegionAttachment {
        texture: "art".into(),
        local_offset: Vec2::ZERO,
        local_rotation: 0.0,
        local_scale: Vec2::ONE,
        width: 40.0,
        height: 40.0,
        uv_rect: Rect {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        },
        pivot: Vec2::splat(0.5),
        sequence: None,
    });
    if weighted {
        mesh.weights = vec![
            vec![
                VertexWeight {
                    bone: a,
                    weight: 0.25
                },
                VertexWeight {
                    bone: b,
                    weight: 0.75
                }
            ];
            4
        ];
    }
    let skin = state.doc.skeleton.default_skin;
    state.doc.skeleton.skins[skin].set(slot, "art", Attachment::Mesh(mesh));
    state.rebind_meshes();
    let mut clip = Animation::new("probe", 3.0);
    if weighted {
        clip.timelines.push(Timeline::BoneRotate {
            bone: a,
            keys: vec![Key::linear(0.0, 30.0)],
        });
        clip.timelines.push(Timeline::BoneRotate {
            bone: b,
            keys: vec![Key::linear(0.0, -20.0)],
        });
    }
    let offsets = if weighted {
        vec![initial; 8]
    } else {
        vec![initial; 4]
    };
    if initial != Vec2::ZERO {
        clip.timelines.push(Timeline::Deform {
            slot,
            attachment: "art".into(),
            keys: vec![Key::linear(0.0, offsets)],
        });
    }
    let anim = state.doc.animations.insert(clip);
    state.session = Session::new(skin);
    state.session.select_slot(Some(slot));
    state.session.mesh_edit = true;
    state.session.work_mode = WorkMode::Animate;
    state.session.active_animation = Some(anim);
    state.session.camera.position = vec2(7.0, -9.0);
    state.session.camera.zoom = 2.5;
    state.history = History::default();
    state.refresh_pose();
    state
}

fn drawn(state: &AppState, index: usize) -> Vec2 {
    let target = target(state).unwrap();
    let offsets = state.pose.deforms.get(&(target.slot, target.name.clone()));
    let slice = match offsets {
        Some(o) if !target.mesh.weights.is_empty() => o
            .get(target.mesh.influence_range(index))
            .unwrap_or_default(),
        Some(o) => o.get(index..index + 1).unwrap_or_default(),
        None => &[],
    };
    target
        .mesh
        .skin_vertex_with_deform(index, slice, &state.pose, &target.world)
}

struct PointerHarness {
    context: egui::Context,
    rect: egui::Rect,
}
impl PointerHarness {
    fn new(state: &mut AppState) -> Self {
        let mut h = Self {
            context: egui::Context::default(),
            rect: egui::Rect::NOTHING,
        };
        h.frame(state, vec2(-1000.0, -1000.0), None);
        h
    }
    fn frame(&mut self, state: &mut AppState, world: Vec2, button: Option<bool>) {
        let screen = state
            .session
            .camera
            .world_to_screen(world, vec2(self.rect.width(), self.rect.height()));
        let pos = egui::pos2(self.rect.min.x + screen.x, self.rect.min.y + screen.y);
        let mut events = vec![egui::Event::PointerMoved(pos)];
        if let Some(pressed) = button {
            events.push(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
        }
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..Default::default()
        };
        let _ = self.context.run_ui(input, |ui| {
            let (rect, response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
            self.rect = rect;
            let mouse = ui
                .input(|i| i.pointer.hover_pos())
                .map(|p| vec2(p.x - rect.min.x, p.y - rect.min.y));
            update(
                &mut ToolContext {
                    ui,
                    response: &response,
                    rect,
                    state,
                },
                mouse,
            );
        });
    }
    fn drag(&mut self, state: &mut AppState, deltas: &[Vec2]) {
        let start = drawn(state, 0);
        self.frame(state, start, None);
        self.frame(state, start, Some(true));
        assert_eq!(
            state.session.dragging_vertex,
            Some(0),
            "press must pick the drawn vertex"
        );
        for delta in deltas {
            self.frame(state, start + *delta, None);
        }
        self.frame(state, start + *deltas.last().unwrap(), Some(false));
    }
}

#[test]
fn ffd_drag_does_not_accumulate_mouse_updates() {
    let mut failures = Vec::new();
    for (name, initial, path) in [
        ("one update", Vec2::ZERO, vec![vec2(10.0, 0.0)]),
        (
            "ten updates",
            Vec2::ZERO,
            (1..=10).map(|i| vec2(i as f32, 0.0)).collect(),
        ),
        (
            "return during drag",
            Vec2::ZERO,
            vec![vec2(10.0, 0.0), vec2(5.0, 0.0), Vec2::ZERO],
        ),
        (
            "existing offset",
            vec2(4.0, 3.0),
            vec![vec2(5.0, 0.0), vec2(10.0, 0.0)],
        ),
        ("held pointer", Vec2::ZERO, vec![vec2(10.0, 0.0); 5]),
    ] {
        let mut state = fixture(false, initial);
        let start = drawn(&state, 0);
        PointerHarness::new(&mut state).drag(&mut state, &path);
        let actual = drawn(&state, 0) - start;
        let expected = *path.last().unwrap();
        println!(
            "{name}: expected={expected:?}, actual={actual:?}, offsets={:?}",
            state.pose.deforms.values().next()
        );
        if actual.distance(expected) > 0.01 {
            failures.push(name);
        }
    }
    assert!(failures.is_empty(), "incorrect drag paths: {failures:?}");
}

#[test]
fn ffd_drag_moves_weighted_vertices_in_world_space() {
    let mut state = fixture(true, vec2(4.0, 3.0));
    let start = drawn(&state, 0);
    let others: Vec<_> = (1..4).map(|i| drawn(&state, i)).collect();
    let target_before = target(&state).unwrap();
    PointerHarness::new(&mut state).drag(&mut state, &[vec2(2.0, 1.0), vec2(10.0, 5.0)]);
    let actual = drawn(&state, 0) - start;
    println!(
        "weighted: expected=(10,5), actual={actual:?}, offsets={:?}",
        state.pose.deforms.values().next()
    );
    assert!(actual.distance(vec2(10.0, 5.0)) < 0.01);
    for (i, before) in others.iter().enumerate() {
        assert!(
            drawn(&state, i + 1).distance(*before) < 0.01,
            "unselected vertex moved"
        );
    }
    let after = target(&state).unwrap();
    assert_eq!(after.mesh.setup_vertices, target_before.mesh.setup_vertices);
    assert_eq!(after.mesh.triangles, target_before.mesh.triangles);
    assert_eq!(
        after
            .mesh
            .weights
            .iter()
            .flatten()
            .map(|w| (w.bone, w.weight))
            .collect::<Vec<_>>(),
        target_before
            .mesh
            .weights
            .iter()
            .flatten()
            .map(|w| (w.bone, w.weight))
            .collect::<Vec<_>>()
    );
}

#[test]
fn ffd_drag_weighted_non_identity_binds_follow_the_pointer() {
    let mut state = fixture(true, vec2(4.0, 3.0));
    for (i, (_, bone)) in state.doc.skeleton.bones.iter_mut().enumerate() {
        bone.local_transform = Transform {
            position: vec2(12.0 * i as f32, -7.0 * i as f32),
            rotation: 0.2 * (i + 1) as f32,
            scale: vec2(1.0 + 0.2 * i as f32, 0.8 + 0.1 * i as f32),
            shear: vec2(0.1, -0.15),
        };
    }
    let slot = state.session.active_slot().unwrap();
    let skin = state.session.active_skin;
    let Attachment::Mesh(mut mesh) = state.doc.skeleton.skins[skin]
        .get(slot, "art")
        .unwrap()
        .clone()
    else {
        panic!()
    };
    mesh.inverse_bind_matrices.clear();
    state.doc.skeleton.skins[skin].set(slot, "art", Attachment::Mesh(mesh));
    state.rebind_meshes();
    state.refresh_pose();
    let original = state.pose.deforms.values().next().unwrap().clone();
    let start = drawn(&state, 0);
    let mut h = PointerHarness::new(&mut state);
    h.drag(&mut state, &[vec2(2.0, 1.0), vec2(10.0, 5.0), Vec2::ZERO]);
    assert!(drawn(&state, 0).distance(start) < 0.01);
    assert_eq!(state.pose.deforms.values().next().unwrap(), &original);
    h.drag(&mut state, &[vec2(10.0, 5.0)]);
    let delta = drawn(&state, 0) - start;
    println!("Weighted non-identity binds: delta={delta:?}");
    assert!(delta.distance(vec2(10.0, 5.0)) < 0.01);
}

#[test]
fn ffd_drag_is_one_undo_step_and_separate_gestures_stay_separate() {
    let mut state = fixture(false, vec2(4.0, 3.0));
    let original = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
    let mut h = PointerHarness::new(&mut state);
    h.drag(&mut state, &[vec2(3.0, 0.0), vec2(10.0, 0.0)]);
    assert_eq!(state.history.undo_depth(), 1);
    let first = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
    h.drag(&mut state, &[vec2(2.0, 0.0), vec2(5.0, 0.0)]);
    assert_eq!(
        state.history.undo_depth(),
        2,
        "separate drags at the same time must not merge"
    );
    let second = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
    state.undo();
    assert_eq!(
        ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
        first
    );
    state.undo();
    assert_eq!(
        ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
        original
    );
    state.redo();
    state.redo();
    assert_eq!(
        ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
        second
    );
}

#[test]
fn ffd_drag_respects_rigid_bone_transforms_and_setup_mode() {
    for setup in [false, true] {
        let mut state = fixture(false, Vec2::ZERO);
        let root = state.doc.skeleton.slots[state.session.active_slot().unwrap()].bone;
        state.doc.skeleton.bones[root].local_transform = Transform {
            position: vec2(30.0, -20.0),
            rotation: 0.7,
            scale: vec2(-1.2, 0.6),
            shear: vec2(0.1, -0.2),
        };
        if setup {
            state.set_work_mode(WorkMode::Setup);
        }
        state.refresh_pose();
        let start = drawn(&state, 0);
        PointerHarness::new(&mut state).drag(&mut state, &[vec2(2.0, 1.0), vec2(10.0, 5.0)]);
        assert!(drawn(&state, 0).distance(start + vec2(10.0, 5.0)) < 0.01);
        assert_eq!(state.pose.deforms.is_empty(), setup);
    }
}

#[test]
fn ffd_drag_preserves_individual_influence_offsets_and_selection() {
    let mut state = fixture(true, vec2(4.0, 3.0));
    let slot = state.session.active_slot().unwrap();
    let skin = state.session.active_skin;
    let Attachment::Mesh(mut mesh) = state.doc.skeleton.skins[skin]
        .get(slot, "art")
        .unwrap()
        .clone()
    else {
        panic!()
    };
    mesh.weights[1][0].weight = 0.6;
    mesh.weights[1][1].weight = 0.4;
    state.doc.skeleton.skins[skin].set(slot, "art", Attachment::Mesh(mesh));
    let anim = state.session.active_animation.unwrap();
    let Timeline::Deform { keys, .. } = state.doc.animations[anim].timelines.last_mut().unwrap()
    else {
        panic!()
    };
    keys[0].value[1] = vec2(-2.0, 7.0);
    state.refresh_pose();
    state.session.selected_vertices = vec![0, 1];
    let before: Vec<_> = (0..4).map(|i| drawn(&state, i)).collect();
    let initial = state.pose.deforms.values().next().unwrap().clone();
    PointerHarness::new(&mut state).drag(&mut state, &[vec2(1.0, 2.0), vec2(10.0, 5.0)]);
    for (i, pos) in before.iter().enumerate() {
        assert!(
            drawn(&state, i).distance(*pos + if i < 2 { vec2(10.0, 5.0) } else { Vec2::ZERO })
                < 0.01
        );
    }
    let after = state.pose.deforms.values().next().unwrap();
    assert!((after[1] - after[0]).distance(initial[1] - initial[0]) < 0.0001);
    assert_eq!(&after[4..], &initial[4..]);
}

#[test]
fn ffd_drag_refuses_unrepresentable_or_singular_weighted_vertices() {
    for singular in [false, true] {
        let mut state = fixture(true, Vec2::ZERO);
        let slot = state.session.active_slot().unwrap();
        let skin = state.doc.skeleton.default_skin;
        let Attachment::Mesh(mut mesh) = state.doc.skeleton.skins[skin]
            .get(slot, "art")
            .unwrap()
            .clone()
        else {
            panic!()
        };
        if singular {
            for weights in &mut mesh.weights {
                weights[0].weight = 0.5;
                weights[1].weight = 0.5;
            }
            let anim = state.session.active_animation.unwrap();
            let Timeline::BoneRotate { keys, .. } = &mut state.doc.animations[anim].timelines[0]
            else {
                panic!()
            };
            keys[0].value = 0.0;
            let Timeline::BoneRotate { keys, .. } = &mut state.doc.animations[anim].timelines[1]
            else {
                panic!()
            };
            keys[0].value = 180.0;
        } else {
            mesh.weights[0].clear();
        }
        state.doc.skeleton.skins[skin].set(slot, "art", Attachment::Mesh(mesh));
        state.refresh_pose();
        let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
        let start = drawn(&state, 0);
        let mut h = PointerHarness::new(&mut state);
        h.frame(&mut state, start, None);
        h.frame(&mut state, start, Some(true));
        assert!(state.session.dragging_vertex.is_none());
        assert!(
            state
                .session
                .status
                .as_deref()
                .unwrap()
                .contains(if singular {
                    "not invertible"
                } else {
                    "no influences"
                })
        );
        assert_eq!(state.history.undo_depth(), 0);
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            before
        );
    }
}

#[test]
fn ffd_drag_no_motion_is_a_noop_and_release_applies_last_position() {
    let mut state = fixture(false, Vec2::ZERO);
    let start = drawn(&state, 0);
    let mut h = PointerHarness::new(&mut state);
    h.drag(&mut state, &[Vec2::ZERO; 3]);
    assert_eq!(state.history.undo_depth(), 0);
    h.frame(&mut state, start, None);
    h.frame(&mut state, start, Some(true));
    h.frame(&mut state, start + vec2(10.0, 0.0), Some(false));
    assert!(drawn(&state, 0).distance(start + vec2(10.0, 0.0)) < 0.01);
    assert!(state.session.deform_drag.is_none());
    assert_eq!(state.history.undo_depth(), 1);
}

#[test]
fn ffd_drag_does_not_reapply_after_undo_or_mode_change() {
    for mode_change in [false, true] {
        let mut state = fixture(false, Vec2::ZERO);
        let start = drawn(&state, 0);
        let mut h = PointerHarness::new(&mut state);
        h.frame(&mut state, start, None);
        h.frame(&mut state, start, Some(true));
        h.frame(&mut state, start + vec2(5.0, 0.0), None);
        if mode_change {
            state.set_work_mode(WorkMode::Setup);
        } else {
            state.undo();
        }
        let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
        h.frame(&mut state, start + vec2(10.0, 0.0), None);
        h.frame(&mut state, start + vec2(10.0, 0.0), Some(false));
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            before
        );
    }
}

#[test]
fn ffd_drag_selection_change_ends_the_old_gesture() {
    let mut state = fixture(false, Vec2::ZERO);
    let start = drawn(&state, 0);
    let mut h = PointerHarness::new(&mut state);
    h.frame(&mut state, start, None);
    h.frame(&mut state, start, Some(true));
    h.frame(&mut state, start + vec2(5.0, 0.0), None);
    state.session.select_slot(None);
    assert!(state.session.deform_drag.is_none());
    assert!(state.session.dragging_vertex.is_none());
}

#[test]
fn ffd_drag_crapaud_copy_paths_and_save_round_trip() {
    use ankhimate_document::{Document, commands::mesh_cmds::ConvertToMesh};
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../demos/m0070-crapaud-pharaon/projet/Crapaud-Pharaon.ankh");
    for steps in [1, 10] {
        let (loaded, _) = ankhimate_formats::load(&path).unwrap();
        assert!(loaded.report.is_clean());
        let mut state = AppState::default();
        state.replace_document(Document::from_loaded(loaded));
        let slot = state
            .doc
            .skeleton
            .slots
            .iter()
            .find(|(_, s)| s.name == "corps_slot")
            .unwrap()
            .0;
        let skin = state.doc.skeleton.default_skin;
        state.dispatch(Box::new(ConvertToMesh::new(skin, slot, "corps")));
        state.session.select_slot(Some(slot));
        state.session.mesh_edit = true;
        state.set_work_mode(WorkMode::Animate);
        let before = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
        let start = drawn(&state, 0);
        let path: Vec<_> = (1..=steps)
            .map(|i| vec2(10.0 * i as f32 / steps as f32, 0.0))
            .collect();
        PointerHarness::new(&mut state).drag(&mut state, &path);
        let delta = drawn(&state, 0) - start;
        println!("Crapaud: steps={steps}, delta={delta:?}");
        assert!(delta.distance(vec2(10.0, 0.0)) < 0.01);
        let after = ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap();
        state.undo();
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            before
        );
        state.redo();
        assert_eq!(
            ankhimate_formats::to_json(&state.doc.as_project_ref()).unwrap(),
            after
        );
        let temp = tempfile::tempdir().unwrap();
        let saved = temp.path().join("Crapaud-test.ankh");
        ankhimate_formats::save(&saved, &state.doc.as_project_ref(), &[]).unwrap();
        let (loaded, _) = ankhimate_formats::load(&saved).unwrap();
        assert!(loaded.report.is_clean());
        let reopened = Document::from_loaded(loaded);
        assert_eq!(
            ankhimate_formats::to_json(&reopened.as_project_ref()).unwrap(),
            after
        );
    }
}

#[test]
fn ffd_drag_can_author_neutral_impact_neutral_without_a_curve_editor() {
    let mut state = fixture(false, Vec2::ZERO);
    let rest = drawn(&state, 0);
    let mut h = PointerHarness::new(&mut state);
    h.drag(&mut state, &[vec2(2.0, 0.0), Vec2::ZERO]);
    state.set_playhead(46.0 / 30.0);
    h.drag(&mut state, &[vec2(5.0, 0.0), vec2(10.0, 0.0)]);
    state.set_playhead(58.0 / 30.0);
    h.drag(&mut state, &[vec2(-5.0, 0.0), vec2(-10.0, 0.0)]);
    for (time, displacement) in [(0.0, 0.0), (46.0 / 30.0, 10.0), (58.0 / 30.0, 0.0)] {
        state.set_playhead(time);
        assert!(
            drawn(&state, 0).distance(rest + vec2(displacement, 0.0)) < 0.01,
            "pose t={time}"
        );
    }
    assert_eq!(state.history.undo_depth(), 3);
    let anim = state.session.active_animation.unwrap();
    let Timeline::Deform { keys, .. } = &state.doc.animations[anim].timelines[0] else {
        panic!()
    };
    assert_eq!(keys.len(), 3);
    assert_eq!(keys[0].value, vec![Vec2::ZERO; 4]);
    assert_eq!(keys[2].value, vec![Vec2::ZERO; 4]);
}
