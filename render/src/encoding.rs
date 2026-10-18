//! Frame encoding only. Sampling, geometry and camera stay in Sequence.
use crate::{Error, validate_size};
use image::{
    ImageEncoder, RgbaImage,
    codecs::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder},
};
use std::io::Write;

pub const JPEG_QUALITY: u8 = 95;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FrameEncoding {
    #[default]
    Png,
    /// An opaque background is required by construction; alpha cannot be dropped silently.
    Jpeg {
        background: [u8; 3],
    },
    WebpLossless,
}
impl FrameEncoding {
    pub fn label(self) -> &'static str {
        match self {
            Self::Png => "PNG",
            Self::Jpeg { .. } => "JPEG",
            Self::WebpLossless => "WebP lossless",
        }
    }
    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg { .. } => "jpg",
            Self::WebpLossless => "webp",
        }
    }
    pub fn frame_name(self, index: u32) -> String {
        format!("frame_{index:06}.{}", self.extension())
    }
    pub fn filename_pattern(self) -> String {
        format!("frame_%06d.{}", self.extension())
    }
    pub fn alpha(self) -> &'static str {
        if matches!(self, Self::Jpeg { .. }) {
            "none"
        } else {
            "straight"
        }
    }
    pub fn background(self) -> [u8; 4] {
        match self {
            Self::Jpeg {
                background: [r, g, b],
            } => [r, g, b, 255],
            _ => [0; 4],
        }
    }
    pub fn encode(self, image: &RgbaImage) -> Result<Vec<u8>, Error> {
        let mut out = vec![];
        self.encode_into(image, &mut out)?;
        Ok(out)
    }
    pub fn encode_into(self, image: &RgbaImage, writer: impl Write) -> Result<(), Error> {
        validate_size(image.width(), image.height())?;
        let result = match self {
            Self::Png => PngEncoder::new(writer).write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgba8,
            ),
            Self::WebpLossless => WebPEncoder::new_lossless(writer).write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ExtendedColorType::Rgba8,
            ),
            Self::Jpeg { background } => {
                let rgb = flatten(image, background);
                JpegEncoder::new_with_quality(writer, JPEG_QUALITY).encode(
                    &rgb,
                    image.width(),
                    image.height(),
                    image::ExtendedColorType::Rgb8,
                )
            }
        };
        result.map_err(|e| Error::Image(format!("{} encoding failed: {e}", self.label())))
    }
}
/// Straight RGBA composited onto explicit RGB in encoded 8-bit colour space.
fn flatten(image: &RgbaImage, background: [u8; 3]) -> Vec<u8> {
    image
        .pixels()
        .flat_map(|p| {
            background.into_iter().zip(p.0).map(move |(bg, src)| {
                let a = u32::from(p.0[3]);
                ((u32::from(src) * a + u32::from(bg) * (255 - a) + 127) / 255) as u8
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lossless_formats_preserve_every_rgba_byte_even_beneath_zero_alpha() {
        let image = RgbaImage::from_raw(
            3,
            2,
            vec![
                17, 93, 201, 0, 10, 20, 30, 128, 250, 30, 9, 255, 1, 2, 3, 1, 0, 255, 100, 254, 77,
                88, 99, 0,
            ],
        )
        .unwrap();
        for format in [FrameEncoding::Png, FrameEncoding::WebpLossless] {
            let bytes = format.encode(&image).unwrap();
            assert_eq!(image::load_from_memory(&bytes).unwrap().to_rgba8(), image);
            assert_eq!(bytes, format.encode(&image).unwrap());
        }
    }
    #[test]
    fn jpeg_flattens_straight_alpha_onto_the_requested_background() {
        let rgba =
            RgbaImage::from_raw(3, 1, vec![255, 0, 0, 0, 255, 0, 0, 128, 5, 6, 7, 255]).unwrap();
        assert_eq!(
            flatten(&rgba, [20, 40, 60]),
            vec![20, 40, 60, 138, 20, 30, 5, 6, 7]
        );
        let uniform = RgbaImage::from_pixel(16, 16, image::Rgba([220, 15, 99, 0]));
        let bytes = FrameEncoding::Jpeg {
            background: [24, 80, 160],
        }
        .encode(&uniform)
        .unwrap();
        for p in image::load_from_memory(&bytes).unwrap().to_rgb8().pixels() {
            for (a, b) in p.0.into_iter().zip([24u8, 80, 160]) {
                assert!(a.abs_diff(b) <= 2);
            }
        }
    }
    #[test]
    fn encoder_errors_and_zero_dimensions_are_reported_for_each_format() {
        struct Broken;
        impl Write for Broken {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("simulated encoder output failure"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        for f in [
            FrameEncoding::Png,
            FrameEncoding::WebpLossless,
            FrameEncoding::Jpeg {
                background: [255; 3],
            },
        ] {
            assert!(
                f.encode_into(&RgbaImage::new(8, 8), Broken)
                    .unwrap_err()
                    .to_string()
                    .contains("encoding failed")
            );
            assert!(f.encode(&RgbaImage::new(0, 0)).is_err());
        }
    }
}
