//! Ephemeral app-render-target readback, never desktop capture or filesystem I/O.
use super::protocol::Provider;
use eframe::egui;
use serde_json::{Value, json};
use std::{io::Cursor, time::Instant};
pub const MAX_SIDE: u32 = 1024;
pub const MAX_BYTES: usize = 1024 * 1024;
pub fn supported(provider: Provider) -> bool {
    Provider::ALL.contains(&provider)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Quality {
    Light,
    #[default]
    Balanced,
    Detailed,
}
impl Quality {
    pub const ALL: [Self; 3] = [Self::Light, Self::Balanced, Self::Detailed];
    pub fn side(self) -> u32 {
        match self {
            Self::Light => 512,
            Self::Balanced => 768,
            Self::Detailed => MAX_SIDE,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Light => "Light (512)",
            Self::Balanced => "Balanced (768)",
            Self::Detailed => "Detailed (1024)",
        }
    }
}
#[derive(Clone)]
pub struct FrameSpec {
    pub rect: egui::Rect,
    pub pixels_per_point: f32,
    pub revision: u64,
    pub camera: crate::ui::canvas::camera::Camera2D,
    pub playhead: f32,
    pub setup: bool,
}
#[derive(Clone)]
pub struct ImageInput {
    pub data_url: String,
    pub mapping: Value,
    pub try_unknown: bool,
    pub max_requests: usize,
}
impl std::fmt::Debug for ImageInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageInput")
            .field("pixels", &"[redacted ephemeral PNG]")
            .field("max_requests", &self.max_requests)
            .finish()
    }
}
impl ImageInput {
    pub fn base64(&self) -> &str {
        self.data_url
            .strip_prefix("data:image/png;base64,")
            .unwrap_or("")
    }
    pub fn text(&self, text: &str) -> String {
        format!("{text}\nNative image mapping (DATA): {}", self.mapping)
    }
    pub fn validate(&self) -> Result<(), String> {
        use base64::Engine;
        if !self.data_url.starts_with("data:image/png;base64,")
            || self.data_url.len() > MAX_BYTES * 4 / 3 + 100
            || !(1..=17).contains(&self.max_requests)
        {
            return Err("Invalid image format or size/request limit (PNG viewport only)".into());
        }
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(self.base64())
            .map_err(|_| "Invalid PNG base64")?;
        if bytes.len() > MAX_BYTES || bytes.len() < 24 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" {
            return Err("Invalid PNG or image exceeds 1 MiB".into());
        }
        let w = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
        let h = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
        if w == 0
            || h == 0
            || w > MAX_SIDE
            || h > MAX_SIDE
            || self.mapping["width"] != w
            || self.mapping["height"] != h
        {
            return Err("Invalid viewport image dimensions or mapping (maximum 1024)".into());
        }
        let mut reader =
            image::ImageReader::with_format(Cursor::new(bytes), image::ImageFormat::Png);
        let mut limits = image::Limits::default();
        limits.max_image_width = Some(MAX_SIDE);
        limits.max_image_height = Some(MAX_SIDE);
        limits.max_alloc = Some(16 * 1024 * 1024);
        reader.limits(limits);
        reader.decode().map_err(|_| "Invalid PNG data")?;
        Ok(())
    }
}
pub struct Preview {
    pub image: egui::ColorImage,
    pub input: ImageInput,
    pub revision: u64,
    pub route: String,
    pub approved: bool,
    pub created: Instant,
    pub texture: Option<egui::TextureHandle>,
}
#[derive(Default)]
pub struct Vision {
    pub open: bool,
    pub quality: Quality,
    pub requested: bool,
    pub preview: Option<Preview>,
    pending: Option<(u64, FrameSpec, String, Instant)>,
    serial: u64,
    pub error: Option<String>,
    deadline: Option<Instant>,
    requester: Option<egui::ViewportId>,
}
impl Vision {
    pub fn request(&mut self) {
        self.open = true;
    }
    pub fn begin_capture(&mut self, ctx: &egui::Context) {
        self.clear();
        self.requester = Some(ctx.viewport_id());
        self.requested = true;
        self.open = false;
        self.deadline = Some(Instant::now() + std::time::Duration::from_secs(5));
        ctx.request_repaint_of(egui::ViewportId::ROOT);
        ctx.request_repaint_after_for(std::time::Duration::from_secs(5), egui::ViewportId::ROOT);
    }
    pub fn clear(&mut self) {
        self.deadline = None;
        self.preview = None;
        self.pending = None;
        self.requested = false;
        self.error = None;
    }
    pub fn busy(&self) -> bool {
        self.requested || self.pending.is_some()
    }
    pub fn cancel_capture(&mut self) {
        self.clear();
    }
    /// Called by the actual, visible ROOT canvas after its navigation/overlays.
    pub fn canvas(&mut self, ctx: &egui::Context, spec: FrameSpec, route: String) {
        if !self.requested || ctx.viewport_id() != egui::ViewportId::ROOT {
            return;
        }
        self.requested = false;
        if !spec.rect.is_finite() || spec.rect.width() < 2.0 || spec.rect.height() < 2.0 {
            self.error = Some("Open the viewport before capturing".into());
            return;
        }
        self.serial = self.serial.wrapping_add(1);
        self.pending = Some((self.serial, spec, route, Instant::now()));
        ctx.send_viewport_cmd_to(
            egui::ViewportId::ROOT,
            egui::ViewportCommand::Screenshot(egui::UserData::new(self.serial)),
        );
        ctx.request_repaint_after(std::time::Duration::from_secs(5));
    }
    pub fn receive(&mut self, ctx: &egui::Context, revision: u64) {
        if self.deadline.is_some_and(|end| Instant::now() >= end) {
            self.deadline = None;
            self.requested = false;
            self.pending = None;
            self.open = true;
            ctx.request_repaint_of(self.requester.unwrap_or(egui::ViewportId::ROOT));
            self.error =
                Some("Viewport capture timed out; open the viewport and retry manually".into());
        }
        if self
            .pending
            .as_ref()
            .is_some_and(|(_, _, _, t)| t.elapsed().as_secs() >= 5)
        {
            self.pending = None;
            self.error = Some("Viewport capture timed out; retry manually".into());
        }
        let events = ctx.input(|i| i.events.clone());
        for event in events {
            let egui::Event::Screenshot {
                viewport_id,
                user_data,
                image,
            } = event
            else {
                continue;
            };
            let Some((serial, _, _, _)) = &self.pending else {
                continue;
            };
            if viewport_id != egui::ViewportId::ROOT
                || user_data
                    .data
                    .as_ref()
                    .and_then(|d| d.downcast_ref::<u64>())
                    != Some(serial)
            {
                continue;
            }
            let (_, spec, route, _) = self.pending.take().unwrap();
            self.deadline = None;
            self.open = true;
            ctx.request_repaint_of(self.requester.unwrap_or(egui::ViewportId::ROOT));
            if revision != spec.revision {
                self.error = Some("Project changed during capture; recapture".into());
                continue;
            }
            let cropped = if self.quality == Quality::Detailed {
                crop(&image, &spec)
            } else {
                crop_with_quality(&image, &spec, self.quality)
            };
            match cropped {
                Ok((image, input)) => {
                    self.preview = Some(Preview {
                        image,
                        input,
                        revision,
                        route,
                        approved: false,
                        created: Instant::now(),
                        texture: None,
                    });
                    self.open = true;
                    self.error = None;
                }
                Err(e) => self.error = Some(e),
            }
        }
    }
    pub fn authorized(
        &self,
        provider: Provider,
        route: &str,
        revision: u64,
    ) -> Result<Option<ImageInput>, String> {
        let Some(p) = &self.preview else {
            return Ok(None);
        };
        if !p.approved {
            return Err("Review the viewport preview and explicitly authorize it, or remove it, before Send".into());
        }
        if !supported(provider) {
            return Err("Vision is not implemented for this profile. Remove the image to continue with text".into());
        }
        if p.route != route || p.revision != revision || p.created.elapsed().as_secs() > 120 {
            return Err("Image permission is stale (provider/model, document or two-minute limit). Capture and authorize again".into());
        }
        Ok(Some(p.input.clone()))
    }
}
/// Strictly inward-rounded crop: no pixel outside the actual canvas rectangle.
pub fn crop(
    source: &egui::ColorImage,
    spec: &FrameSpec,
) -> Result<(egui::ColorImage, ImageInput), String> {
    crop_with_quality(source, spec, Quality::Detailed)
}
pub fn crop_with_quality(
    source: &egui::ColorImage,
    spec: &FrameSpec,
    quality: Quality,
) -> Result<(egui::ColorImage, ImageInput), String> {
    let p = spec.pixels_per_point;
    if !p.is_finite()
        || p <= 0.0
        || !spec.rect.is_finite()
        || spec.rect.min.x < 0.0
        || spec.rect.min.y < 0.0
        || !spec.camera.zoom.is_finite()
        || spec.camera.zoom <= 0.0
        || !spec.camera.position.is_finite()
    {
        return Err("Invalid viewport geometry".into());
    }
    let x = (spec.rect.min.x * p).ceil() as u32;
    let y = (spec.rect.min.y * p).ceil() as u32;
    let right = (spec.rect.max.x * p).floor() as u32;
    let bottom = (spec.rect.max.y * p).floor() as u32;
    if right <= x
        || bottom <= y
        || right as usize > source.size[0]
        || bottom as usize > source.size[1]
        || source.pixels.len() != source.size[0] * source.size[1]
    {
        return Err("Canvas crop is outside the render target; retry after layout settles".into());
    }
    // Resize directly into a <=1024² buffer, without a second full-window copy.
    let width = right - x;
    let height = bottom - y;
    let ratio = (quality.side() as f64 / width.max(height) as f64).min(1.0);
    let w = (width as f64 * ratio).round().max(1.0) as u32;
    let h = (height as f64 * ratio).round().max(1.0) as u32;
    let mut rgba = image::RgbaImage::new(w, h);
    for (ox, oy, pixel) in rgba.enumerate_pixels_mut() {
        let sx = x + ((ox as f64 + 0.5) * width as f64 / w as f64).floor() as u32;
        let sy = y + ((oy as f64 + 0.5) * height as f64 / h as f64).floor() as u32;
        *pixel = image::Rgba(
            source.pixels[sy as usize * source.size[0] + sx as usize].to_srgba_unmultiplied(),
        );
    }
    let image = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba.as_raw());
    let mut bytes = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(rgba)
        .write_to(&mut bytes, image::ImageFormat::Png)
        .map_err(|_| "Could not encode viewport PNG")?;
    let bytes = bytes.into_inner();
    if bytes.len() > MAX_BYTES {
        return Err("Viewport PNG exceeds 1 MiB; reduce the viewport/detail and retry".into());
    }
    let top_left = spec.camera.screen_to_world(
        glam::vec2(
            x as f32 / p - spec.rect.min.x,
            y as f32 / p - spec.rect.min.y,
        ),
        glam::vec2(spec.rect.width(), spec.rect.height()),
    );
    let mapping = json!({"source":"Ankhimate canvas render target crop (may include editor overlays)","width":w,"height":h,"revision":spec.revision,"mode":if spec.setup{"setup"}else{"animate"},"playhead_seconds":spec.playhead,"pixel_origin":"top-left image edge; x right, y down","world_origin_at_image_edge":top_left.to_array(),"world_units_per_image_pixel":[width as f32/w as f32/p/spec.camera.zoom,-(height as f32/h as f32/p/spec.camera.zoom)],"warning":"Image shows the captured pose, not bone-local setup coordinates. Use native parent setup transforms; observations are estimates, never inferred precise joints. Metadata sharing is separate."});
    use base64::Engine;
    Ok((
        image,
        ImageInput {
            data_url: format!(
                "data:image/png;base64,{}",
                base64::engine::general_purpose::STANDARD.encode(bytes)
            ),
            mapping,
            try_unknown: false,
            max_requests: 2,
        },
    ))
}
pub fn definition() -> Value {
    json!({"name":"ankhimate_viewport","description":"Request a viewport observation. Opens the native capture/preview consent flow; does NOT capture or send an image. Ask the user to capture, review and authorize an image for a new Send. Never claim to have seen an image unless one is actually attached.","parameters":{"type":"object","properties":{},"additionalProperties":false}})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hidden_viewport_capture_deadline_and_cancel_do_not_keep_pending_pixels() {
        let ctx = egui::Context::default();
        let mut v = Vision::default();
        v.begin_capture(&ctx);
        assert!(v.busy());
        v.deadline = Some(Instant::now() - std::time::Duration::from_millis(1));
        v.receive(&ctx, 0);
        assert!(!v.busy());
        assert!(v.open && v.error.as_ref().unwrap().contains("timed out"));
        assert!(v.preview.is_none());
        v.begin_capture(&ctx);
        v.cancel_capture();
        assert!(!v.busy() && v.preview.is_none());
    }
}
