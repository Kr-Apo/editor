//! Deterministic, page-at-a-time packing of the existing sequence renderer.
use crate::{Error, RenderedPng, sequence::Sequence};
use image::{ImageEncoder, RgbaImage, codecs::png::PngEncoder};
use serde::Serialize;

pub const MAX_PAGE: u32 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct PixelRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Frame {
    pub index: u32,
    pub page: u32,
    pub rect: PixelRect,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Page {
    pub index: u32,
    pub width: u32,
    pub height: u32,
    pub first_frame: u32,
    pub frame_count: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    pages: Vec<Page>,
    frames: Vec<Frame>,
    padding: u32,
    width: u32,
    height: u32,
}

pub fn page_name(index: u32) -> String {
    format!("sheet_{index:06}.png")
}

impl Layout {
    /// Constant-space preflight for reactive UI; full frame records are built
    /// only once, by the background export worker.
    pub fn page_count(
        count: u32,
        width: u32,
        height: u32,
        max_page: u32,
        padding: u32,
    ) -> Result<u32, Error> {
        Ok(count.div_ceil(grid(count, width, height, max_page, padding)?.capacity))
    }
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }
    /// Row-major fixed cells, with padding on all four sides. Page dimensions
    /// are the occupied cell grid, including unused cells in its final row.
    pub fn new(
        count: u32,
        width: u32,
        height: u32,
        max_page: u32,
        padding: u32,
    ) -> Result<Self, Error> {
        let Grid {
            cell_width,
            cell_height,
            columns,
            capacity,
        } = grid(count, width, height, max_page, padding)?;
        let mut pages = Vec::new();
        let mut frames = Vec::with_capacity(count as usize);
        for page in 0..count.div_ceil(capacity) {
            let first_frame = page * capacity;
            let page_count = (count - first_frame).min(capacity);
            let page_columns = columns.min(page_count);
            pages.push(Page {
                index: page,
                width: page_columns * cell_width,
                height: page_count.div_ceil(page_columns) * cell_height,
                first_frame,
                frame_count: page_count,
            });
            for local in 0..page_count {
                frames.push(Frame {
                    index: first_frame + local,
                    page,
                    rect: PixelRect {
                        x: (local % page_columns) * cell_width + padding,
                        y: (local / page_columns) * cell_height + padding,
                        w: width,
                        h: height,
                    },
                });
            }
        }
        Ok(Self {
            pages,
            frames,
            padding,
            width,
            height,
        })
    }

    /// Explicitly owned RGBA buffers: one page + one rendered frame. Decoded
    /// project textures, the document, encoded PNG and codec scratch are extra.
    pub fn max_rgba_bytes(&self) -> u64 {
        let page = self
            .pages
            .iter()
            .map(|p| u64::from(p.width) * u64::from(p.height))
            .max()
            .unwrap_or(0);
        (page + u64::from(self.width) * u64::from(self.height)) * 4
    }
}

struct Grid {
    cell_width: u32,
    cell_height: u32,
    columns: u32,
    capacity: u32,
}
fn grid(count: u32, width: u32, height: u32, max_page: u32, padding: u32) -> Result<Grid, Error> {
    if !(1..=100_000).contains(&count)
        || width == 0
        || height == 0
        || max_page == 0
        || max_page > MAX_PAGE
        || padding > 32
    {
        return Err(Error::Invalid("spritesheets require 1–100000 frames, a page limit of 1–4096 px and padding of 0–32 px".into()));
    }
    let cell_width = width
        .checked_add(padding * 2)
        .ok_or_else(|| Error::Invalid("frame width overflow".into()))?;
    let cell_height = height
        .checked_add(padding * 2)
        .ok_or_else(|| Error::Invalid("frame height overflow".into()))?;
    let columns = max_page / cell_width;
    let rows = max_page / cell_height;
    if columns == 0 || rows == 0 {
        return Err(Error::Invalid(
            "a frame plus its padding does not fit the maximum page size".into(),
        ));
    }
    Ok(Grid {
        cell_width,
        cell_height,
        columns,
        capacity: columns * rows,
    })
}

pub fn render_page(
    sequence: &Sequence<'_>,
    layout: &Layout,
    page_index: u32,
    progress: impl FnMut(u32) -> bool,
) -> Result<RenderedPng, Error> {
    render_page_cropped(sequence, layout, page_index, None, progress)
}

pub fn render_page_cropped(
    sequence: &Sequence<'_>,
    layout: &Layout,
    page_index: u32,
    prepared: Option<&crate::framing::PreparedFrames>,
    mut progress: impl FnMut(u32) -> bool,
) -> Result<RenderedPng, Error> {
    let page = layout
        .pages
        .get(page_index as usize)
        .ok_or_else(|| Error::Invalid("page index outside spritesheet".into()))?;
    if sequence.dimensions() != (layout.width, layout.height)
        || sequence.frame_count() != layout.frames.len() as u32
    {
        return Err(Error::Invalid(
            "spritesheet layout does not match the prepared sequence".into(),
        ));
    }
    if !progress(page.first_frame) {
        return Err(Error::Invalid("export cancelled".into()));
    }
    let mut image = RgbaImage::new(page.width, page.height);
    for frame in
        &layout.frames[page.first_frame as usize..(page.first_frame + page.frame_count) as usize]
    {
        if !progress(frame.index) {
            return Err(Error::Invalid("export cancelled".into()));
        }
        let rgba = sequence.render_rgba(frame.index)?;
        if let Some(prepared) = prepared {
            let rgba = prepared.crop(frame.index, &rgba)?;
            let rect = PixelRect {
                w: rgba.width(),
                h: rgba.height(),
                ..frame.rect
            };
            copy_padded(&mut image, &rgba, rect, layout.padding);
        } else {
            copy_padded(&mut image, &rgba, frame.rect, layout.padding);
        }
        if !progress(frame.index + 1) {
            return Err(Error::Invalid("export cancelled".into()));
        }
    }
    let mut bytes = Vec::new();
    PngEncoder::new(&mut bytes)
        .write_image(
            image.as_raw(),
            page.width,
            page.height,
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| Error::Image(e.to_string()))?;
    Ok(RenderedPng {
        bytes,
        width: page.width,
        height: page.height,
    })
}

fn copy_padded(page: &mut RgbaImage, frame: &RgbaImage, rect: PixelRect, padding: u32) {
    // Copy straight RGBA without blending. Extend the nearest edge pixel across
    // the guard band; every cell owns its full padding, including corners.
    for y in 0..rect.h + padding * 2 {
        for x in 0..rect.w + padding * 2 {
            let sx = x.saturating_sub(padding).min(rect.w - 1);
            let sy = y.saturating_sub(padding).min(rect.h - 1);
            page.put_pixel(
                rect.x - padding + x,
                rect.y - padding + y,
                *frame.get_pixel(sx, sy),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preflight_page_count_handles_long_sequences_without_frame_records() {
        assert_eq!(Layout::page_count(100000, 1, 1, 1, 0).unwrap(), 100000);
        assert_eq!(Layout::page_count(60, 512, 512, 2048, 2).unwrap(), 7);
        assert!(Layout::page_count(60, 512, 512, 512, 2).is_err());
    }
    use crate::sequence::{Sequence, SequenceRequest};
    use ankhimate_core::animation::Animation;
    use ankhimate_document::Document;

    #[test]
    fn layout_is_row_major_multi_page_and_never_exceeds_the_limit() {
        let layout = Layout::new(60, 16, 12, 64, 2).unwrap();
        assert_eq!(layout.pages.len(), 5);
        assert_eq!(
            layout.frames[0].rect,
            PixelRect {
                x: 2,
                y: 2,
                w: 16,
                h: 12
            }
        );
        assert_eq!(
            layout.frames[3].rect,
            PixelRect {
                x: 2,
                y: 18,
                w: 16,
                h: 12
            }
        );
        assert_eq!(layout.frames[12].page, 1);
        assert_eq!(layout.frames[59].index, 59);
        assert!(layout.pages.iter().all(|p| p.width <= 64 && p.height <= 64));
        assert_eq!(layout, Layout::new(60, 16, 12, 64, 2).unwrap());
        assert_eq!(layout.pages.iter().map(|p| p.frame_count).sum::<u32>(), 60);
    }

    #[test]
    fn impossible_pages_and_invalid_parameters_are_refused() {
        for args in [
            (0, 16, 16, 64, 2),
            (1, 0, 16, 64, 2),
            (1, 16, 16, 0, 2),
            (1, 16, 16, 4097, 2),
            (1, 16, 16, 64, 33),
            (1, 64, 16, 64, 1),
            (1, u32::MAX, 16, 64, 1),
            (100001, 1, 1, 64, 0),
        ] {
            assert!(Layout::new(args.0, args.1, args.2, args.3, args.4).is_err());
        }
        assert!(Layout::new(60, 512, 512, 512, 0).is_ok());
        let largest = Layout::new(1, 4096, 4096, 4096, 0).unwrap();
        assert_eq!(largest.max_rgba_bytes(), 128 * 1024 * 1024);
    }

    #[test]
    fn page_crop_matches_sequence_and_cancellation_stops_between_frames() {
        let mut doc = Document::new();
        doc.animations.insert(Animation::new("static", 1.0));
        let req = SequenceRequest {
            animation: "static".into(),
            fps: 60,
            width: 16,
            height: 12,
        };
        let sequence = Sequence::prepare(&doc, &req, |_, _| true).unwrap();
        let layout = Layout::new(60, 16, 12, 64, 2).unwrap();
        for page in &layout.pages {
            let png = render_page(&sequence, &layout, page.index, |_| true).unwrap();
            let image = image::load_from_memory(&png.bytes).unwrap().to_rgba8();
            for frame in layout.frames.iter().filter(|f| f.page == page.index) {
                let crop = image::imageops::crop_imm(&image, frame.rect.x, frame.rect.y, 16, 12)
                    .to_image();
                let expected =
                    image::load_from_memory(&sequence.render(frame.index).unwrap().bytes)
                        .unwrap()
                        .to_rgba8();
                assert_eq!(crop, expected);
            }
        }
        assert!(render_page(&sequence, &layout, 0, |done| done < 2).is_err());
        assert!(render_page(&sequence, &layout, 99, |_| true).is_err());
    }

    #[test]
    fn padding_extrudes_edge_rgba_without_mixing_adjacent_frames() {
        let mut page = image::RgbaImage::new(12, 12);
        let red = image::RgbaImage::from_pixel(2, 2, image::Rgba([255, 0, 0, 128]));
        let blue = image::RgbaImage::from_pixel(2, 2, image::Rgba([0, 0, 255, 64]));
        copy_padded(
            &mut page,
            &red,
            PixelRect {
                x: 2,
                y: 2,
                w: 2,
                h: 2,
            },
            2,
        );
        copy_padded(
            &mut page,
            &blue,
            PixelRect {
                x: 8,
                y: 2,
                w: 2,
                h: 2,
            },
            2,
        );
        for y in 0..6 {
            for x in 0..12 {
                assert_eq!(
                    page.get_pixel(x, y).0,
                    if x < 6 {
                        [255, 0, 0, 128]
                    } else {
                        [0, 0, 255, 64]
                    }
                );
            }
        }
        for y in 6..12 {
            for x in 0..12 {
                assert_eq!(
                    page.get_pixel(x, y).0,
                    [0; 4],
                    "unused cells remain transparent"
                );
            }
        }
    }
}
