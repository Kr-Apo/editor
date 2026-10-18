//! Constant-camera, art-only sequence rendering. No filesystem or UI state.
use super::*;
use ankhimate_core::constraints::Constraint;

#[derive(Clone, Debug)]
pub struct SequenceRequest {
    pub animation: String,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
}

/// Decoded textures and a camera fitted to every sampled pose, not per frame.
/// Memory scales with the document and one frame, not the duration of the clip.
pub struct Sequence<'a> {
    doc: &'a Document,
    request: SequenceRequest,
    count: u32,
    camera: ResolvedCamera,
    focus: ResolvedFocus,
    textures: HashMap<String, Texture>,
}

impl SequenceRequest {
    pub fn frame_count(&self, doc: &Document) -> Result<u32, Error> {
        validate_size(self.width, self.height)?;
        if !(1..=240).contains(&self.fps) {
            return Err(Error::Invalid("FPS must be between 1 and 240".into()));
        }
        let clip = animation(doc, Some(&self.animation))?.unwrap();
        if !clip.duration.is_finite() || clip.duration <= 0.0 {
            return Err(Error::Invalid(
                "clip duration must be finite and greater than zero".into(),
            ));
        }
        // Round only representation noise at integer boundaries (e.g. 0.1f32
        // at 30 FPS). Non-integral durations retain their last t < duration.
        let frames = f64::from(clip.duration) * f64::from(self.fps);
        let rounded = frames.round();
        let frames = if (frames - rounded).abs() <= frames.abs() * f64::from(f32::EPSILON) {
            rounded
        } else {
            frames.ceil()
        };
        if !(1.0..=100_000.0).contains(&frames) {
            return Err(Error::Invalid(
                "sequence must contain between 1 and 100000 frames".into(),
            ));
        }
        if doc
            .skeleton
            .constraints
            .values()
            .any(|c| matches!(c, Constraint::Physics(_)))
        {
            return Err(Error::Invalid("PNG sequences do not yet simulate physics; remove physics constraints or use a project without them".into()));
        }
        // Existing non-normal blend equations target an opaque canvas. Refuse
        // them here rather than silently losing alpha on a transparent export.
        if doc
            .skeleton
            .slots
            .values()
            .any(|s| s.blend_mode != BlendMode::Normal)
        {
            return Err(Error::Invalid(
                "transparent PNG sequences currently support Normal slot blending only".into(),
            ));
        }
        Ok(frames as u32)
    }
}

impl<'a> Sequence<'a> {
    /// `progress` returns false to cancel, including before the first pose.
    pub fn prepare(
        doc: &'a Document,
        request: &SequenceRequest,
        mut progress: impl FnMut(u32, u32) -> bool,
    ) -> Result<Self, Error> {
        let count = request.frame_count(doc)?;
        let focus = ResolvedFocus::new(
            doc,
            Some(&Focus {
                mode: FocusMode::ArtOnly,
                ..Default::default()
            }),
        )?;
        if !progress(0, count) {
            return Err(Error::Invalid("export cancelled".into()));
        }
        let textures = decode_textures(doc);
        // Unlike preview, export must not silently omit undecodable/missing art.
        if textures.len() != doc.assets.images.len() {
            return Err(Error::Image(
                "one or more project images cannot be decoded".into(),
            ));
        }
        let mut union = Bounds::empty();
        for i in 0..count {
            if !progress(i, count) {
                return Err(Error::Invalid("export cancelled".into()));
            }
            let pose = pose_at(doc, Some(&request.animation), i as f32 / request.fps as f32)?;
            for &slot in &pose.draw_order {
                if let Some(attachment) =
                    doc.skeleton
                        .resolve_posed(&[doc.skeleton.default_skin], &pose, slot)
                {
                    let name = match attachment {
                        Attachment::Region(r) => r
                            .sequence
                            .as_ref()
                            .and_then(|s| {
                                pose.slot_sequence_frames
                                    .get(slot)
                                    .and_then(|f| s.frame(*f))
                            })
                            .unwrap_or(&r.texture),
                        Attachment::Mesh(m) => m
                            .sequence
                            .as_ref()
                            .and_then(|s| {
                                pose.slot_sequence_frames
                                    .get(slot)
                                    .and_then(|f| s.frame(*f))
                            })
                            .unwrap_or(&m.texture),
                        _ => continue,
                    };
                    if !textures.contains_key(name) {
                        return Err(Error::Image(format!("missing texture `{name}`")));
                    }
                }
            }
            let draws = build_draws(doc, &pose, &focus)?;
            union.include_bounds(bounds_for(doc, &pose, &draws, &focus));
        }
        if !progress(count, count) {
            return Err(Error::Invalid("export cancelled".into()));
        }
        let camera = resolve_camera(&Camera::default(), union, request.width, request.height)?;
        Ok(Self {
            doc,
            request: request.clone(),
            count,
            camera,
            focus,
            textures,
        })
    }

    pub fn frame_count(&self) -> u32 {
        self.count
    }
    pub fn camera(&self) -> ResolvedCamera {
        self.camera
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.request.width, self.request.height)
    }
    pub fn time(&self, index: u32) -> Result<f32, Error> {
        if index >= self.count {
            return Err(Error::Invalid("frame index outside sequence".into()));
        }
        Ok(index as f32 / self.request.fps as f32)
    }
    pub fn render(&self, index: u32) -> Result<RenderedPng, Error> {
        let image = self.render_rgba(index)?;
        let mut bytes = Vec::new();
        PngEncoder::new(&mut bytes)
            .write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|e| Error::Image(e.to_string()))?;
        Ok(RenderedPng {
            bytes,
            width: image.width(),
            height: image.height(),
        })
    }

    /// The same evaluated frame before PNG encoding, for page-at-a-time packing.
    pub fn render_rgba(&self, index: u32) -> Result<RgbaImage, Error> {
        let time = self.time(index)?;
        let pose = pose_at(self.doc, Some(&self.request.animation), time)?;
        let draws = build_draws(self.doc, &pose, &self.focus)?;
        let mut canvas = Canvas::new(self.request.width, self.request.height, [0; 4]);
        let trail_poses = [(time, &pose)];
        let scene = PaintContext {
            doc: self.doc,
            focus: &self.focus,
            camera: self.camera,
            trail_poses: &trail_poses,
            textures: &self.textures,
        };
        paint_scene(
            &mut canvas,
            Viewport::new(0, 0, self.request.width, self.request.height),
            &pose,
            &draws,
            &scene,
        );
        Ok(canvas.image)
    }
}
