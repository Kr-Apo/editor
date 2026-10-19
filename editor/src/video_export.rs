//! MP4/GIF worker: existing CPU sequence, bounded streaming, supervised external executable.
use ankhimate_document::Document;
use ankhimate_render::sequence::{Sequence, SequenceRequest};
use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};
pub const DOWNLOAD: &str = "https://ffmpeg.org/download.html";
const LOG_LIMIT: usize = 256 * 1024;
const STALL: Duration = Duration::from_secs(60);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Format {
    #[default]
    Mp4,
    Gif,
}
impl Format {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Mp4 => "mp4",
            Self::Gif => "gif",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Mp4 => "MP4",
            Self::Gif => "Animated GIF",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QualityMode {
    #[default]
    Crf,
    TargetBitrate,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Encoder {
    #[default]
    Libx264,
    Libx264Rgb,
}
impl Encoder {
    pub fn name(self) -> &'static str {
        match self {
            Self::Libx264 => "libx264",
            Self::Libx264Rgb => "libx264rgb",
        }
    }
    pub fn pixel_format(self) -> &'static str {
        match self {
            Self::Libx264 => "yuv420p",
            Self::Libx264Rgb => "rgb24",
        }
    }
}
pub const MIN_BITRATE_KBPS: u32 = 64;
pub const MAX_BITRATE_KBPS: u32 = 100_000;
#[derive(Debug, Clone)]
pub struct Settings {
    pub quality: QualityMode,
    pub bitrate_kbps: u32,
    pub encoder: Encoder,
    pub open_after_export: bool,
    pub format: Format,
    /// Session ID; absent only for unambiguous legacy name-based callers.
    pub clip: Option<ankhimate_core::ids::AnimationId>,
    pub request: SequenceRequest,
    pub background: [u8; 3],
    pub crf: u8,
    pub cycles: u32,
    pub executable: String,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            format: Format::Mp4,
            quality: QualityMode::Crf,
            bitrate_kbps: 2000,
            encoder: Encoder::Libx264,
            open_after_export: false,
            clip: None,
            request: SequenceRequest {
                animation: String::new(),
                fps: 30,
                width: 512,
                height: 512,
            },
            background: [255; 3],
            crf: 23,
            cycles: 1,
            executable: String::new(),
        }
    }
}
impl Settings {
    pub fn mp4_description(&self) -> String {
        let quality = match self.quality {
            QualityMode::Crf => format!("CRF {}", self.crf),
            QualityMode::TargetBitrate => format!(
                "target {} kbit/s (average not guaranteed)",
                self.bitrate_kbps
            ),
        };
        format!(
            "{} / {} / {quality} / preset medium",
            self.encoder.name(),
            self.encoder.pixel_format()
        )
    }
    fn selected<'a>(
        &self,
        doc: &'a Document,
    ) -> Result<
        (
            ankhimate_core::ids::AnimationId,
            &'a ankhimate_core::animation::Animation,
        ),
        String,
    > {
        if let Some(id) = self.clip {
            return doc
                .animations
                .get(id)
                .map(|a| (id, a))
                .ok_or_else(|| "Selected animation no longer exists; select it again".into());
        }
        let mut matches = doc
            .animations
            .iter()
            .filter(|(_, a)| a.name == self.request.animation);
        let first = matches.next().ok_or("Selected animation does not exist")?;
        if matches.next().is_some() {
            return Err("Ambiguous animation name; select a clip by ID".into());
        }
        Ok(first)
    }
    /// Same one-clip snapshot principle as T-601.3, never mutate author data.
    pub fn snapshot(&self, doc: &Document) -> Result<Document, String> {
        let (id, _) = self.selected(doc)?;
        let mut snapshot = doc.clone();
        snapshot.animations.retain(|key, _| key == id);
        Ok(snapshot)
    }
    pub fn request_for(&self, doc: &Document) -> Result<SequenceRequest, String> {
        let mut request = self.request.clone();
        request.animation = self.selected(doc)?.1.name.clone();
        Ok(request)
    }
    pub fn count(&self, doc: &Document) -> Result<u32, String> {
        if self.format == Format::Mp4 && self.quality == QualityMode::Crf && self.crf > 51 {
            return Err("H.264 CRF must be 0..51 (lower is higher quality)".into());
        }
        if self.format == Format::Mp4
            && self.quality == QualityMode::TargetBitrate
            && !(MIN_BITRATE_KBPS..=MAX_BITRATE_KBPS).contains(&self.bitrate_kbps)
        {
            return Err(format!(
                "Target bitrate must be {MIN_BITRATE_KBPS}..{MAX_BITRATE_KBPS} kbit/s"
            ));
        }
        if self.cycles == 0 {
            return Err("Loop cycles must be positive".into());
        }
        if self.format == Format::Mp4
            && (!self.request.width.is_multiple_of(2) || !self.request.height.is_multiple_of(2))
        {
            return Err(
                "H.264 yuv420p requires even width and height; no automatic resizing".into(),
            );
        }
        if self.format == Format::Gif && self.request.fps > 100 {
            return Err("GIF supports at most 100 source FPS (centisecond delays); no silent frame dropping".into());
        }
        let (_, clip) = self.selected(doc)?;
        // No asset-byte clone during UI repaint. SequenceRequest remains authoritative
        // for timing, dimensions, physics and blending validation.
        let mut timing = Document::new();
        timing.skeleton = doc.skeleton.clone();
        timing
            .animations
            .insert(ankhimate_core::animation::Animation::new(
                &clip.name,
                clip.duration,
            ));
        self.request_for(doc)?
            .frame_count(&timing)
            .map_err(|e| e.to_string())?
            .checked_mul(self.cycles)
            .filter(|n| *n <= 1_000_000)
            .ok_or_else(|| "Video exceeds 1,000,000 frames".into())
    }
}
/// Resolve only a regular executable file; a custom path never falls back to PATH.
pub fn locate(custom: &str) -> Result<PathBuf, String> {
    locate_in(custom, std::env::var_os("PATH").as_deref())
}
fn locate_in(custom: &str, path: Option<&OsStr>) -> Result<PathBuf, String> {
    let candidates = if custom.is_empty() {
        path.map(std::env::split_paths)
            .into_iter()
            .flatten()
            .filter(|p| !p.as_os_str().is_empty())
            .map(|p| {
                p.join(if cfg!(windows) {
                    "ffmpeg.exe"
                } else {
                    "ffmpeg"
                })
            })
            .collect::<Vec<_>>()
    } else {
        vec![PathBuf::from(custom)]
    };
    candidates.into_iter()
        .filter(|p|p.is_file())
        .filter_map(|p|fs::canonicalize(p).ok())
        // Command may implicitly invoke cmd.exe for .bat/.cmd on Windows.
        .find(|p|!cfg!(windows) || p.extension().and_then(|e|e.to_str()).is_some_and(|e|e.eq_ignore_ascii_case("exe")))
        .ok_or_else(||format!("FFmpeg executable {} is missing or unusable (Windows requires .exe). Configure Settings → Video, or install it yourself: {DOWNLOAD}",if custom.is_empty(){"in PATH"}else{custom}))
}
fn command(exe: &Path) -> Command {
    let mut cmd = Command::new(exe);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    cmd
}
pub fn arguments(settings: &Settings, count: u32, output: &Path) -> Vec<OsString> {
    let r = &settings.request;
    let mut args: Vec<OsString> = [
        "-hide_banner",
        "-loglevel",
        "warning",
        "-nostdin",
        "-n",
        "-f",
        "rawvideo",
        "-pixel_format",
        "rgb24",
        "-video_size",
    ]
    .into_iter()
    .map(Into::into)
    .collect();
    args.extend([
        format!("{}x{}", r.width, r.height).into(),
        "-framerate".into(),
        r.fps.to_string().into(),
        "-i".into(),
        "pipe:0".into(),
    ]);
    args.extend(
        ["-an", "-c:v", settings.encoder.name(), "-preset", "medium"]
            .into_iter()
            .map(OsString::from),
    );
    match settings.quality {
        QualityMode::Crf => {
            args.push("-crf".into());
            args.push(settings.crf.to_string().into());
        }
        QualityMode::TargetBitrate => {
            args.push("-b:v".into());
            args.push(format!("{}k", settings.bitrate_kbps).into());
        }
    }
    args.extend(
        ["-pix_fmt", settings.encoder.pixel_format(), "-frames:v"]
            .into_iter()
            .map(OsString::from),
    );
    args.push(count.to_string().into());
    args.extend(
        ["-movflags", "+faststart", "-f", "mp4"]
            .into_iter()
            .map(OsString::from),
    );
    args.push(output.as_os_str().to_owned());
    args
}
fn composite(rgba: &image::RgbaImage, bg: [u8; 3]) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(rgba.len() / 4 * 3);
    for p in rgba.pixels() {
        let a = u32::from(p[3]);
        for c in 0..3 {
            rgb.push(((u32::from(p[c]) * a + u32::from(bg[c]) * (255 - a) + 127) / 255) as u8);
        }
    }
    rgb
}
fn drain(mut stream: impl Read) -> Result<String, String> {
    let mut data = Vec::new();
    let mut buf = [0; 8192];
    let mut truncated = false;
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                data.extend_from_slice(&buf[..n]);
                if data.len() > LOG_LIMIT {
                    data.drain(..data.len() - LOG_LIMIT);
                    truncated = true;
                }
            }
            Err(e) => return Err(format!("Cannot read FFmpeg log: {e}")),
        }
    }
    Ok(format!(
        "{}{}",
        if truncated {
            "[FFmpeg log truncated; last 256 KiB follow]\n"
        } else {
            ""
        },
        String::from_utf8_lossy(&data)
    ))
}
/// Last-resort ownership guard, also covering spawn/pump setup failures and unwinding.
struct Process(Child);
impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn stopped(cancel: &AtomicBool) -> Result<(), String> {
    if cancel.load(Ordering::Relaxed) {
        Err("Video cancelled".into())
    } else {
        Ok(())
    }
}

#[cfg(test)]
fn probe(exe: &Path, cancel: &AtomicBool) -> Result<(), String> {
    probe_encoder(exe, Encoder::Libx264, cancel)
}
fn probe_encoder(exe: &Path, encoder: Encoder, cancel: &AtomicBool) -> Result<(), String> {
    let out = query(exe, "-encoders", cancel)?;
    if !listed(&out, encoder.name()) {
        return Err(format!(
            "FFmpeg does not provide the requested {} H.264 encoder. No codec fallback.\n{DOWNLOAD}",
            encoder.name()
        ));
    }
    Ok(())
}
fn listed(output: &str, name: &str) -> bool {
    output
        .lines()
        .any(|l| l.split_whitespace().nth(1) == Some(name))
}
fn query(exe: &Path, option: &str, cancel: &AtomicBool) -> Result<String, String> {
    thread::scope(|scope| {
        stopped(cancel)?;
        let mut child = Process(
            command(exe)
                .args(["-hide_banner", option])
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Cannot run FFmpeg {}: {e}. {DOWNLOAD}", exe.display()))?,
        );
        let out = child.0.stdout.take().unwrap();
        let err = child.0.stderr.take().unwrap();
        let stdout = scope.spawn(move || drain(out));
        let stderr = scope.spawn(move || drain(err));
        let start = Instant::now();
        let result = loop {
            if let Err(e) = stopped(cancel) {
                break Err(e);
            }
            match child.0.try_wait() {
                Ok(Some(s)) => break Ok(s),
                Ok(None) => {}
                Err(e) => break Err(e.to_string()),
            }
            if start.elapsed() > Duration::from_secs(10) {
                break Err("FFmpeg detection timed out".into());
            }
            thread::sleep(Duration::from_millis(10));
        };
        if result.is_err() {
            let _ = child.0.kill();
            let _ = child.0.wait();
        }
        let out = stdout.join().map_err(|_| "FFmpeg stdout reader failed")??;
        let err = stderr.join().map_err(|_| "FFmpeg stderr reader failed")??;
        let status = result.map_err(|e| format!("{e}\n{err}\n{DOWNLOAD}"))?;
        if !status.success() {
            return Err(format!(
                "FFmpeg capability query failed ({status}).\n{err}\n{DOWNLOAD}"
            ));
        }
        Ok(out)
    })
}

/// Stream at most one queued RGB frame; supervise blocked stdin and finalization.
fn encode(
    cmd: &mut Command,
    count: u32,
    cancel: &AtomicBool,
    timeout: Duration,
    mut frame: impl FnMut(u32) -> Result<Vec<u8>, String>,
    mut progress: impl FnMut(u32),
) -> Result<(), String> {
    thread::scope(|scope| {
        stopped(cancel)?;
        let mut child = Process(
            cmd.stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Cannot launch FFmpeg: {e}. {DOWNLOAD}"))?,
        );
        let mut stdin = child.0.stdin.take().unwrap();
        let stderr = child.0.stderr.take().unwrap();
        let reader = scope.spawn(move || drain(stderr));
        let (tx, rx) = mpsc::sync_channel::<Vec<u8>>(1);
        let (ack_tx, ack_rx) = mpsc::channel();
        let pump = scope.spawn(move || -> Result<(), String> {
            for bytes in rx {
                stdin
                    .write_all(&bytes)
                    .map_err(|e| format!("FFmpeg stdin: {e}"))?;
                let _ = ack_tx.send(());
            }
            drop(stdin);
            Ok(())
        });
        let attempt = (|| -> Result<(), String> {
            for index in 0..count {
                stopped(cancel)?;
                let bytes = frame(index)?;
                stopped(cancel)?;
                tx.send(bytes)
                    .map_err(|_| "FFmpeg closed stdin before all frames")?;
                // Only one frame in flight. Never block the controlling worker on a pipe write.
                let start = Instant::now();
                loop {
                    stopped(cancel)?;
                    match ack_rx.try_recv() {
                        Ok(()) => break,
                        Err(mpsc::TryRecvError::Disconnected) => {
                            return Err("FFmpeg stopped reading stdin".into());
                        }
                        Err(mpsc::TryRecvError::Empty) => {}
                    }
                    if let Some(status) = child.0.try_wait().map_err(|e| e.to_string())?
                        && (!status.success() || index + 1 < count)
                    {
                        return Err(format!("FFmpeg exited early: {status}"));
                    }
                    // With -frames:v, FFmpeg can finish the last frame before the
                    // pump is scheduled to acknowledge its completed pipe write.
                    // Exit 0 on this last frame must still await ACK (or pipe error).
                    if start.elapsed() > timeout {
                        return Err("FFmpeg stdin stalled; encoder killed".into());
                    }
                    thread::sleep(Duration::from_millis(10));
                }
                progress(index + 1);
            }
            Ok(())
        })();
        drop(tx);
        // Poll final flush separately after closing the frame queue / stdin.
        let result = match attempt {
            Err(e) => Err(e),
            Ok(_) => {
                let start = Instant::now();
                loop {
                    if let Err(e) = stopped(cancel) {
                        break Err(e);
                    }
                    match child.0.try_wait() {
                        Ok(Some(status)) => break Ok(status),
                        Ok(None) => {}
                        Err(e) => break Err(e.to_string()),
                    }
                    if start.elapsed() > timeout {
                        break Err("FFmpeg finalization stalled; encoder killed".into());
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            }
        };
        let termination = match &result {
            Ok(status) => status.to_string(),
            Err(_) => {
                let _ = child.0.kill();
                child
                    .0
                    .wait()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|e| format!("cannot reap FFmpeg: {e}"))
            }
        };
        let writer = pump.join().map_err(|_| "FFmpeg input worker panicked");
        let log = reader
            .join()
            .map_err(|_| "FFmpeg stderr worker panicked")??;
        let status =
            result.map_err(|e| format!("{e}\nFFmpeg exit status: {termination}\n{log}"))?;
        if !status.success() {
            return Err(format!("FFmpeg failed: {status}\n{log}"));
        }
        writer?.map_err(|e| format!("{e}\n{log}"))?;
        Ok(())
    })
}

/// Basic ISO-BMFF structure verification, not a substitute for a full decoder.
fn check_mp4(path: &Path) -> Result<(), String> {
    use std::io::{Seek, SeekFrom};
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let length = file.metadata().map_err(|e| e.to_string())?.len();
    let mut offset = 0u64;
    let (mut ftyp, mut moov, mut mdat) = (false, false, false);
    while offset < length {
        let mut h = [0; 8];
        file.read_exact(&mut h).map_err(|_| "Truncated MP4 box")?;
        let size = u32::from_be_bytes(h[..4].try_into().unwrap());
        let mut header = 8;
        let size = match size {
            0 => length - offset,
            1 => {
                let mut ext = [0; 8];
                file.read_exact(&mut ext)
                    .map_err(|_| "Truncated MP4 extended box")?;
                header = 16;
                u64::from_be_bytes(ext)
            }
            s => u64::from(s),
        };
        if size < header || size > length - offset {
            return Err("Invalid MP4 box size".into());
        }
        match &h[4..] {
            b"ftyp" => ftyp = size >= 16,
            b"moov" => moov = size > header,
            b"mdat" => mdat = size > header,
            _ => {}
        }
        offset += size;
        file.seek(SeekFrom::Start(offset))
            .map_err(|e| e.to_string())?;
    }
    if ftyp && moov && mdat {
        Ok(())
    } else {
        Err("FFmpeg output is not a nonempty MP4 (ftyp/moov/mdat required)".into())
    }
}

pub fn export(
    doc: &Document,
    settings: &Settings,
    destination: &Path,
    cancel: &AtomicBool,
    mut progress: impl FnMut(&'static str, u32, u32),
) -> Result<u32, String> {
    let count = settings.count(doc)?;
    if destination
        .extension()
        .and_then(|s| s.to_str())
        .is_none_or(|s| !s.eq_ignore_ascii_case(settings.format.extension()))
    {
        return Err(format!(
            "Choose a .{} destination",
            settings.format.extension()
        ));
    }
    ankhimate_export::new_file::inspect(destination)?;
    stopped(cancel)?;
    let focused = settings.snapshot(doc)?;
    let doc = &focused;
    let request = settings.request_for(doc)?;
    let project = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    let report = ankhimate_export::preflight::inspect(
        &project,
        &doc.assets,
        ankhimate_export::preflight::Target::Rendered,
        Some(&request.animation),
    );
    if !report.allowed(true) {
        return Err(serde_json::to_string_pretty(&report).unwrap());
    }
    let exe = locate(&settings.executable)?;
    progress("Checking FFmpeg", 0, count);
    match settings.format {
        Format::Mp4 => probe_encoder(&exe, settings.encoder, cancel)?,
        Format::Gif => gif::probe(&exe, cancel)?,
    }
    let sequence = Sequence::prepare(doc, &request, |done, total| {
        progress("Framing", done, total);
        !cancel.load(Ordering::Relaxed)
    })
    .map_err(|e| e.to_string())?;
    stopped(cancel)?;
    let stage = match settings.format {
        Format::Mp4 => ankhimate_export::new_file::PreparedFile::new(destination)?,
        Format::Gif => ankhimate_export::new_file::PreparedFile::with_auxiliary(
            destination,
            "encoded.gif",
            &["palette.png"],
        )?,
    };
    let result = (|| {
        match settings.format {
            Format::Mp4 => {
                let mut cmd = command(&exe);
                cmd.args(arguments(settings, count, &stage.path));
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
                    |done| {
                        progress(
                            if done == count {
                                "Finalizing"
                            } else {
                                "Encoding"
                            },
                            done,
                            count,
                        )
                    },
                )?;
                stopped(cancel)?;
                check_mp4(&stage.path)?;
            }
            Format::Gif => gif::render(
                &exe,
                &sequence,
                settings,
                count,
                &stage,
                cancel,
                &mut progress,
            )?,
        }
        stopped(cancel)?;
        stage.publish()?;
        Ok(count)
    })();
    match (result, stage.cleanup()) {
        (Ok(n), Ok(())) => {
            progress("Complete", n, n);
            Ok(n)
        }
        (Err(e), Ok(())) => Err(e),
        (Ok(_), Err(cleanup)) => Err(format!(
            "Video published successfully, but cleanup failed. {cleanup}"
        )),
        (Err(e), Err(cleanup)) => Err(format!("{e}\n{cleanup}")),
    }
}
#[path = "video_export/gif.rs"]
pub mod gif;
#[cfg(test)]
#[path = "video_export/t8105_tests.rs"]
mod t8105_tests;
#[cfg(test)]
pub(crate) mod tests;
