//! Optional pixel-space crops over the unchanged common-camera Sequence.
//! No resampling, evaluator changes or whole-sequence RGBA retention.
use crate::{Error, sequence::Sequence, spritesheet::PixelRect};
use image::RgbaImage;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundsMode {
    #[default]
    Historical,
    GlobalPixels,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct FrameOptions {
    pub bounds: BoundsMode,
    pub trim: bool,
    pub padding: u32,
}
impl FrameOptions {
    pub fn crops(self) -> bool {
        self.trim || self.bounds != BoundsMode::Historical
    }
    pub fn validate(self, width: u32, height: u32) -> Result<(), Error> {
        if self.padding > 32 {
            return Err(Error::Invalid(
                "image padding must be between 0 and 32 px".into(),
            ));
        }
        // Conservative: even the full source canvas plus guard band must fit.
        crate::validate_size(width, height)?;
        crate::validate_size(width + self.padding * 2, height + self.padding * 2)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FrameMapping {
    pub index: u32,
    pub source_rect: PixelRect,
    pub content_rect: PixelRect,
    pub image_size: Size,
    pub empty: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct PreparedFrames {
    pub version: u32,
    pub required: bool,
    pub options: FrameOptions,
    pub source_size: Size,
    pub global_rect: PixelRect,
    pub frames: Vec<FrameMapping>,
}
impl PreparedFrames {
    pub fn prepare(
        sequence: &Sequence<'_>,
        options: FrameOptions,
        mut progress: impl FnMut(u32, u32) -> bool,
    ) -> Result<Self, Error> {
        let (w, h) = sequence.dimensions();
        options.validate(w, h)?;
        let count = sequence.frame_count();
        let mut visible = Vec::with_capacity(count as usize);
        for i in 0..count {
            if !progress(i, count) {
                return Err(Error::Invalid("export cancelled".into()));
            }
            let rgba = sequence.render_rgba(i)?;
            visible.push((pixel_rect(&rgba), rgba.pixels().all(|p| p.0[3] == 0)));
        }
        if !progress(count, count) {
            return Err(Error::Invalid("export cancelled".into()));
        }
        Ok(Self::from_visible(Size { w, h }, options, visible))
    }
    fn from_visible(
        source_size: Size,
        options: FrameOptions,
        visible: Vec<(Option<PixelRect>, bool)>,
    ) -> Self {
        let full = PixelRect {
            x: 0,
            y: 0,
            w: source_size.w,
            h: source_size.h,
        };
        let union = visible
            .iter()
            .filter_map(|(rect, _)| *rect)
            .reduce(union_rect);
        let global_rect = if options.bounds == BoundsMode::GlobalPixels {
            union.unwrap_or(PixelRect {
                x: 0,
                y: 0,
                w: 1,
                h: 1,
            })
        } else {
            full
        };
        let frames = visible
            .into_iter()
            .enumerate()
            .map(|(index, (rect, empty))| {
                let source_rect = if options.trim {
                    rect.unwrap_or(PixelRect {
                        x: global_rect.x,
                        y: global_rect.y,
                        w: 1,
                        h: 1,
                    })
                } else {
                    global_rect
                };
                FrameMapping {
                    index: index as u32,
                    source_rect,
                    content_rect: PixelRect {
                        x: options.padding,
                        y: options.padding,
                        w: source_rect.w,
                        h: source_rect.h,
                    },
                    image_size: Size {
                        w: source_rect.w + 2 * options.padding,
                        h: source_rect.h + 2 * options.padding,
                    },
                    empty,
                }
            })
            .collect();
        Self {
            version: 1,
            required: true,
            options,
            source_size,
            global_rect,
            frames,
        }
    }
    pub fn crop(&self, index: u32, rgba: &RgbaImage) -> Result<RgbaImage, Error> {
        if rgba.dimensions() != (self.source_size.w, self.source_size.h) {
            return Err(Error::Invalid(
                "crop canvas does not match source size".into(),
            ));
        }
        let r = self
            .frames
            .get(index as usize)
            .ok_or_else(|| Error::Invalid("crop index outside sequence".into()))?
            .source_rect;
        Ok(image::imageops::crop_imm(rgba, r.x, r.y, r.w, r.h).to_image())
    }
}
/// Preserve every nonzero RGBA byte, including RGB under rounded-zero alpha.
/// Removing those pixels would prevent exact decoded PNG/WebP reconstruction.
pub fn pixel_rect(image: &RgbaImage) -> Option<PixelRect> {
    let mut min_x = image.width();
    let mut min_y = image.height();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut any = false;
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel.0 != [0; 4] {
            any = true;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }
    any.then(|| PixelRect {
        x: min_x,
        y: min_y,
        w: max_x - min_x + 1,
        h: max_y - min_y + 1,
    })
}
fn union_rect(a: PixelRect, b: PixelRect) -> PixelRect {
    let x = a.x.min(b.x);
    let y = a.y.min(b.y);
    PixelRect {
        x,
        y,
        w: (a.x + a.w).max(b.x + b.w) - x,
        h: (a.y + a.h).max(b.y + b.h) - y,
    }
}
/// Guard pixels repeat nearest straight RGBA; consumers discard the guard band.
pub fn padded(image: RgbaImage, padding: u32) -> Result<RgbaImage, Error> {
    FrameOptions {
        padding,
        ..Default::default()
    }
    .validate(image.width(), image.height())?;
    if padding == 0 {
        return Ok(image);
    }
    let mut out = RgbaImage::new(image.width() + padding * 2, image.height() + padding * 2);
    for (x, y, pixel) in out.enumerate_pixels_mut() {
        *pixel = *image.get_pixel(
            x.saturating_sub(padding).min(image.width() - 1),
            y.saturating_sub(padding).min(image.height() - 1),
        );
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn static_margins_partial_alpha_and_empty_frames_have_unambiguous_offsets() {
        let mut image = RgbaImage::new(16, 12);
        image.put_pixel(3, 4, image::Rgba([5, 6, 7, 1]));
        image.put_pixel(10, 8, image::Rgba([100, 120, 150, 128]));
        let r = pixel_rect(&image).unwrap();
        assert_eq!(
            r,
            PixelRect {
                x: 3,
                y: 4,
                w: 8,
                h: 5
            }
        );
        let options = FrameOptions {
            trim: true,
            padding: 2,
            ..Default::default()
        };
        let prepared = PreparedFrames::from_visible(
            Size { w: 16, h: 12 },
            options,
            vec![(Some(r), false), (None, true)],
        );
        let cropped = prepared.crop(0, &image).unwrap();
        let bytes = padded(cropped, 2).unwrap();
        assert_eq!(bytes.dimensions(), (12, 9));
        assert_eq!(bytes.get_pixel(0, 0).0, [5, 6, 7, 1]);
        let mut restored = RgbaImage::new(16, 12);
        let c = prepared.frames[0].content_rect;
        for y in 0..r.h {
            for x in 0..r.w {
                restored.put_pixel(x + r.x, y + r.y, *bytes.get_pixel(x + c.x, y + c.y));
            }
        }
        assert_eq!(restored, image);
        assert!(prepared.frames[1].empty);
        assert_eq!(prepared.frames[1].image_size, Size { w: 5, h: 5 });
    }
    #[test]
    fn global_union_is_constant_and_all_transparent_clips_use_one_pixel() {
        let a = PixelRect {
            x: 2,
            y: 3,
            w: 4,
            h: 2,
        };
        let b = PixelRect {
            x: 12,
            y: 5,
            w: 2,
            h: 6,
        };
        let options = FrameOptions {
            bounds: BoundsMode::GlobalPixels,
            ..Default::default()
        };
        let p = PreparedFrames::from_visible(
            Size { w: 16, h: 12 },
            options,
            vec![(Some(a), false), (None, true), (Some(b), false)],
        );
        assert_eq!(
            p.global_rect,
            PixelRect {
                x: 2,
                y: 3,
                w: 12,
                h: 8
            }
        );
        assert!(p.frames.iter().all(|f| f.source_rect == p.global_rect));
        let empty = PreparedFrames::from_visible(
            Size { w: 16, h: 12 },
            options,
            vec![(None, true), (None, true)],
        );
        assert!(empty.frames.iter().all(|f| f.empty
            && f.source_rect
                == PixelRect {
                    x: 0,
                    y: 0,
                    w: 1,
                    h: 1
                }));
    }
    #[test]
    fn padding_and_size_limits_are_checked_without_overflow() {
        assert!(
            FrameOptions {
                padding: 33,
                ..Default::default()
            }
            .validate(8, 8)
            .is_err()
        );
        assert!(
            FrameOptions {
                padding: 1,
                ..Default::default()
            }
            .validate(8192, 8)
            .is_err()
        );
        assert!(
            FrameOptions::default()
                .validate(u32::MAX, u32::MAX)
                .is_err()
        );
        assert!(FrameOptions::default().validate(0, 8).is_err());
    }
    #[test]
    fn rgb_beneath_zero_alpha_is_retained_for_exact_byte_reconstruction() {
        let mut image = RgbaImage::new(8, 8);
        image.put_pixel(1, 2, image::Rgba([40, 160, 220, 0]));
        image.put_pixel(5, 6, image::Rgba([40, 160, 220, 1]));
        assert_eq!(
            pixel_rect(&image),
            Some(PixelRect {
                x: 1,
                y: 2,
                w: 5,
                h: 5
            })
        );
        image.put_pixel(5, 6, image::Rgba([0; 4]));
        let p = PreparedFrames::from_visible(
            Size { w: 8, h: 8 },
            FrameOptions {
                trim: true,
                ..Default::default()
            },
            vec![(pixel_rect(&image), true)],
        );
        assert!(p.frames[0].empty);
        assert_eq!(
            p.crop(0, &image).unwrap().get_pixel(0, 0).0,
            [40, 160, 220, 0]
        );
    }
    #[test]
    fn cropped_images_propagate_encoding_failures_without_altering_source() {
        use std::io::Write;
        struct Broken;
        impl Write for Broken {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("injected encoding failure"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut image = RgbaImage::new(8, 8);
        image.put_pixel(3, 4, image::Rgba([100, 50, 25, 128]));
        let before = image.clone();
        let p = PreparedFrames::from_visible(
            Size { w: 8, h: 8 },
            FrameOptions {
                trim: true,
                padding: 2,
                ..Default::default()
            },
            vec![(pixel_rect(&image), false)],
        );
        for codec in [
            crate::encoding::FrameEncoding::Png,
            crate::encoding::FrameEncoding::WebpLossless,
            crate::encoding::FrameEncoding::Jpeg {
                background: [255; 3],
            },
        ] {
            let cropped = padded(p.crop(0, &image).unwrap(), 2).unwrap();
            assert!(
                codec
                    .encode_into(&cropped, Broken)
                    .unwrap_err()
                    .to_string()
                    .contains("encoding failed")
            );
        }
        assert_eq!(image, before);
    }
}
