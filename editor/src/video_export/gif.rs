//! GIF-specific two-pass palette workflow; process ownership stays in video_export.
use super::*;

pub(super) fn probe(exe: &Path, cancel: &AtomicBool) -> Result<(), String> {
    let encoders = query(exe, "-encoders", cancel)?;
    let filters = query(exe, "-filters", cancel)?;
    for (listing, name) in [
        (&encoders, "gif"),
        (&encoders, "png"),
        (&filters, "palettegen"),
        (&filters, "paletteuse"),
    ] {
        if !listed(listing, name) {
            return Err(format!(
                "FFmpeg GIF capability missing: {name}. libx264 is not required. {DOWNLOAD}"
            ));
        }
    }
    Ok(())
}
fn input(settings: &Settings) -> Vec<OsString> {
    let r = &settings.request;
    [
        "-hide_banner".into(),
        "-loglevel".into(),
        "warning".into(),
        "-nostdin".into(),
        "-n".into(),
        "-f".into(),
        "rawvideo".into(),
        "-pixel_format".into(),
        "rgb24".into(),
        "-video_size".into(),
        format!("{}x{}", r.width, r.height).into(),
        "-framerate".into(),
        r.fps.to_string().into(),
        "-i".into(),
        "pipe:0".into(),
    ]
    .into()
}
/// Nearest centisecond boundary, half up. The last delay completes this grid.
fn boundary(index: u32, fps: u32) -> u64 {
    (u64::from(index) * 200 + u64::from(fps)) / (u64::from(fps) * 2)
}
pub fn final_delay(count: u32, fps: u32) -> u64 {
    boundary(count, fps) - boundary(count - 1, fps)
}
pub fn palette_arguments(settings: &Settings, palette: &Path) -> Vec<OsString> {
    let mut a = input(settings);
    a.extend(
        [
            "-an",
            "-vf",
            "palettegen=max_colors=256:reserve_transparent=0:stats_mode=full",
            "-frames:v",
            "1",
            "-c:v",
            "png",
            "-threads",
            "1",
            "-update",
            "1",
            "-f",
            "image2",
        ]
        .into_iter()
        .map(OsString::from),
    );
    a.push(palette.as_os_str().to_owned());
    a
}
pub fn arguments(settings: &Settings, count: u32, palette: &Path, output: &Path) -> Vec<OsString> {
    let mut a = input(settings);
    a.push("-i".into());
    a.push(palette.as_os_str().to_owned());
    a.extend(
        [
            "-filter_complex",
            "[0:v][1:v]paletteuse=dither=sierra2_4a:diff_mode=rectangle",
            "-an",
            "-c:v",
            "gif",
            "-gifflags",
            "0",
            "-fps_mode",
            "passthrough",
            "-frames:v",
        ]
        .into_iter()
        .map(OsString::from),
    );
    a.push(count.to_string().into());
    a.extend(
        ["-loop", "-1", "-final_delay"]
            .into_iter()
            .map(OsString::from),
    );
    a.push(final_delay(count, settings.request.fps).to_string().into());
    a.extend(["-f", "gif"].into_iter().map(OsString::from));
    a.push(output.as_os_str().to_owned());
    a
}
fn check_palette(path: &Path) -> Result<(), String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() > 1024 * 1024 {
        return Err("Invalid oversized GIF palette".into());
    }
    let mut bytes = Vec::new();
    file.take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    let dimensions =
        image::ImageReader::with_format(std::io::Cursor::new(&bytes), image::ImageFormat::Png)
            .into_dimensions()
            .map_err(|e| e.to_string())?;
    if dimensions != (16, 16) {
        return Err("Expected a 16 × 16 GIF palette".into());
    }
    let palette = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)
        .map_err(|e| format!("Invalid GIF palette PNG: {e}"))?
        .to_rgba8();
    if palette.dimensions() != (16, 16) || palette.pixels().any(|p| p[3] != 255) {
        return Err("Expected an opaque 16 × 16 GIF palette".into());
    }
    Ok(())
}
pub(super) fn render(
    exe: &Path,
    sequence: &Sequence<'_>,
    settings: &Settings,
    count: u32,
    stage: &ankhimate_export::new_file::PreparedFile,
    cancel: &AtomicBool,
    progress: &mut impl FnMut(&'static str, u32, u32),
) -> Result<(), String> {
    let palette = stage
        .auxiliary("palette.png")
        .ok_or("GIF palette not owned by staging")?;
    for (phase, args) in [
        ("Palette generation", palette_arguments(settings, palette)),
        (
            "GIF encoding",
            arguments(settings, count, palette, &stage.path),
        ),
    ] {
        progress(phase, 0, count);
        stopped(cancel)?;
        let mut cmd = command(exe);
        cmd.args(args);
        encode(
            &mut cmd,
            count,
            cancel,
            STALL,
            |i| {
                sequence
                    .render_rgba(i % sequence.frame_count())
                    .map(|image| composite(&image, settings.background))
                    .map_err(|e| e.to_string())
            },
            |done| progress(phase, done, count),
        )
        .map_err(|e| format!("{phase}: {e}"))?;
        stopped(cancel)?;
        if phase == "Palette generation" {
            check_palette(palette)?;
            progress("Palette ready", count, count);
            stopped(cancel)?;
        }
    }
    let info = inspect(&stage.path)?;
    if info.width != settings.request.width
        || info.height != settings.request.height
        || info.frames != count
        || info.repeat.is_some()
        || info.min_delay == 0
        || info.transparent
        || info
            .total_delay_cs
            .abs_diff(boundary(count, settings.request.fps))
            > 1
    {
        return Err(format!(
            "Unexpected GIF structure: {info:?}; expected {count} opaque frames, requested dimensions and no container repeat"
        ));
    }
    Ok(())
}
#[derive(Debug)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub frames: u32,
    pub total_delay_cs: u64,
    pub min_delay: u16,
    pub max_delay: u16,
    pub repeat: Option<u16>,
    pub transparent: bool,
}
/// Bounded block parsing only. Does not validate LZW decompression; integration tests do.
pub fn inspect(path: &Path) -> Result<Info, String> {
    use std::io::{BufReader, Seek, SeekFrom};
    fn read<const N: usize>(r: &mut impl Read) -> Result<[u8; N], String> {
        let mut b = [0; N];
        r.read_exact(&mut b).map_err(|_| "Truncated GIF block")?;
        Ok(b)
    }
    fn skip(r: &mut (impl Read + Seek), n: u64, len: u64) -> Result<(), String> {
        let end = r
            .stream_position()
            .map_err(|e| e.to_string())?
            .checked_add(n)
            .filter(|end| *end <= len)
            .ok_or("Truncated GIF data")?;
        r.seek(SeekFrom::Start(end)).map_err(|e| e.to_string())?;
        Ok(())
    }
    fn blocks(r: &mut impl Read) -> Result<Vec<u8>, String> {
        let mut first = Vec::new();
        loop {
            let n = read::<1>(r)?[0];
            if n == 0 {
                break;
            }
            let mut data = [0; 255];
            r.read_exact(&mut data[..usize::from(n)])
                .map_err(|_| "Truncated GIF sub-block")?;
            if first.is_empty() {
                first.extend_from_slice(&data[..usize::from(n)]);
            }
        }
        Ok(first)
    }
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let len = file.metadata().map_err(|e| e.to_string())?.len();
    let mut r = BufReader::new(file);
    let header = read::<6>(&mut r)?;
    if &header != b"GIF87a" && &header != b"GIF89a" {
        return Err("Invalid GIF signature".into());
    }
    let logical = read::<7>(&mut r)?;
    let mut info = Info {
        width: u32::from(u16::from_le_bytes([logical[0], logical[1]])),
        height: u32::from(u16::from_le_bytes([logical[2], logical[3]])),
        frames: 0,
        total_delay_cs: 0,
        min_delay: u16::MAX,
        max_delay: 0,
        repeat: None,
        transparent: false,
    };
    if info.width == 0 || info.height == 0 {
        return Err("Empty GIF logical screen".into());
    }
    let global = logical[4] & 0x80 != 0;
    if global {
        skip(&mut r, 3 * (2u64 << (logical[4] & 7)), len)?;
    }
    let mut delay = 0;
    let mut transparent = false;
    loop {
        match read::<1>(&mut r)?[0] {
            0x21 => match read::<1>(&mut r)?[0] {
                0xf9 => {
                    if read::<1>(&mut r)?[0] != 4 {
                        return Err("Invalid GIF control extension".into());
                    }
                    let gce = read::<4>(&mut r)?;
                    if read::<1>(&mut r)?[0] != 0 {
                        return Err("Invalid GIF control terminator".into());
                    }
                    delay = u16::from_le_bytes([gce[1], gce[2]]);
                    transparent = gce[0] & 1 != 0;
                }
                0xff => {
                    let n = read::<1>(&mut r)?[0];
                    let mut application = vec![0; usize::from(n)];
                    r.read_exact(&mut application)
                        .map_err(|_| "Truncated GIF application extension")?;
                    let data = blocks(&mut r)?;
                    if application == b"NETSCAPE2.0" || application == b"ANIMEXTS1.0" {
                        if data.len() != 3 || data[0] != 1 {
                            return Err("Invalid GIF repeat extension".into());
                        }
                        info.repeat = Some(u16::from_le_bytes([data[1], data[2]]));
                    }
                }
                _ => {
                    blocks(&mut r)?;
                }
            },
            0x2c => {
                let d = read::<9>(&mut r)?;
                let x = u32::from(u16::from_le_bytes([d[0], d[1]]));
                let y = u32::from(u16::from_le_bytes([d[2], d[3]]));
                let w = u32::from(u16::from_le_bytes([d[4], d[5]]));
                let h = u32::from(u16::from_le_bytes([d[6], d[7]]));
                if w == 0 || h == 0 || x + w > info.width || y + h > info.height {
                    return Err("GIF image rectangle exceeds logical screen".into());
                }
                if d[8] & 0x80 != 0 {
                    skip(&mut r, 3 * (2u64 << (d[8] & 7)), len)?;
                } else if !global {
                    return Err("GIF image has no colour table".into());
                }
                let code = read::<1>(&mut r)?[0];
                if !(2..=8).contains(&code) || blocks(&mut r)?.is_empty() {
                    return Err("Invalid GIF LZW blocks".into());
                }
                info.frames += 1;
                if info.frames > 1_000_000 {
                    return Err("Too many GIF frames".into());
                }
                info.total_delay_cs += u64::from(delay);
                info.min_delay = info.min_delay.min(delay);
                info.max_delay = info.max_delay.max(delay);
                info.transparent |= transparent;
                delay = 0;
                transparent = false;
            }
            0x3b => {
                if info.frames == 0 || r.stream_position().map_err(|e| e.to_string())? != len {
                    return Err("Empty GIF or trailing data".into());
                }
                return Ok(info);
            }
            _ => return Err("Unexpected GIF block".into()),
        }
    }
}
#[cfg(test)]
#[path = "gif/tests.rs"]
mod tests;
