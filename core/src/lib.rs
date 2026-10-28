//! Ankhimate core — framework-free data model + `evaluate()`.
//!
//! No I/O, no image decoding, no egui/wgpu. Compiles for native + wasm32.
//! Deterministic: identical inputs produce bit-identical output (no
//! `std::time`, no global state).

#![forbid(unsafe_code)]

pub mod animation;
pub mod assets;
pub mod attachment;
pub mod clipping;
pub mod constraints;
pub mod ids;
pub mod math;
pub mod path;
pub mod physics;
pub mod pose;
pub mod replay_animation;
pub mod replay_clock;
pub mod replay_physics;
pub mod replay_skin;
pub mod replay_transport;
pub mod skeleton;
pub mod skin;
pub mod slot;
pub mod transforms;

pub use glam;
pub use slotmap;

pub mod replay_consumer;

pub mod mesh_pose;
