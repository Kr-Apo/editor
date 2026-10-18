use super::*;
use std::sync::OnceLock;
pub(crate) fn helper(mode: &str) -> PathBuf {
    static BUILD: OnceLock<tempfile::TempDir> = OnceLock::new();
    let dir = BUILD.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        let output = dir
            .path()
            .join(if cfg!(windows) { "fake.exe" } else { "fake" });
        let result = Command::new("rustc")
            .args(["--edition=2024", "--crate-name", "fake_ffmpeg"])
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/video_export/fake_ffmpeg.rs"))
            .arg("-o")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        dir
    });
    // Unique names avoid parallel tests replacing an executable in use on Windows.
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let file = dir.path().join(format!(
        "été fake {mode}-{}{}",
        NEXT.fetch_add(1, Ordering::Relaxed),
        if cfg!(windows) { ".exe" } else { "" }
    ));
    fs::copy(
        dir.path()
            .join(if cfg!(windows) { "fake.exe" } else { "fake" }),
        &file,
    )
    .unwrap();
    file
}
pub(crate) fn walker() -> Document {
    Document::from_loaded(
        ankhimate_formats::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples/walker.ankh"),
        )
        .unwrap()
        .0,
    )
}
fn settings(mode: &str) -> Settings {
    Settings {
        request: SequenceRequest {
            animation: "walk".into(),
            fps: 30,
            width: 64,
            height: 48,
        },
        cycles: 2,
        background: [80, 150, 210],
        executable: helper(mode).to_string_lossy().into_owned(),
        ..Settings::default()
    }
}
#[test]
fn detection_missing_custom_path_and_empty_path_never_fall_back() {
    assert!(locate_in("", None).unwrap_err().contains(DOWNLOAD));
    assert!(
        locate("not-a-real-custom-ffmpeg.exe")
            .unwrap_err()
            .contains("not-a-real-custom")
    );
    let exe = helper("ok");
    assert_eq!(
        locate(exe.to_str().unwrap()).unwrap(),
        fs::canonicalize(exe).unwrap()
    );
}
#[test]
fn detection_searches_only_supplied_path() {
    let d = tempfile::tempdir().unwrap();
    fs::copy(
        helper("ok"),
        d.path().join(if cfg!(windows) {
            "ffmpeg.exe"
        } else {
            "ffmpeg"
        }),
    )
    .unwrap();
    assert!(
        locate_in(
            "",
            Some(std::env::join_paths([d.path()]).unwrap().as_os_str())
        )
        .is_ok()
    );
}
#[test]
fn argument_vector_is_explicit_and_filename_is_one_literal_argument() {
    let s = settings("ok");
    let output = Path::new("C:/été & unsafe $(name)/film.mp4");
    let args = arguments(&s, 60, output);
    assert_eq!(args.last().unwrap(), output.as_os_str());
    let text: Vec<_> = args.iter().map(|s| s.to_str().unwrap()).collect();
    for pair in [
        ["-pixel_format", "rgb24"],
        ["-pix_fmt", "yuv420p"],
        ["-c:v", "libx264"],
        ["-frames:v", "60"],
        ["-framerate", "30"],
        ["-video_size", "64x48"],
    ] {
        assert!(text.windows(2).any(|w| w == pair));
    }
    assert!(text.contains(&"-n"));
    assert!(text.contains(&"-an"));
    assert!(!text.contains(&"cmd"));
    assert!(!text.contains(&"-y"));
}
#[test]
fn exact_count_cycles_and_invalid_parameters() {
    let doc = walker();
    let mut s = settings("ok");
    assert_eq!(s.count(&doc).unwrap(), 60);
    for field in ["fps", "width", "height", "cycles", "crf", "clip"] {
        let mut invalid = s.clone();
        match field {
            "fps" => invalid.request.fps = 0,
            "width" => invalid.request.width = 63,
            "height" => invalid.request.height = 0,
            "cycles" => invalid.cycles = 0,
            "crf" => invalid.crf = 52,
            _ => invalid.request.animation = "missing".into(),
        };
        assert!(invalid.count(&doc).is_err(), "{field}");
    }
    s.cycles = u32::MAX;
    assert!(s.count(&doc).is_err());
}
#[test]
fn alpha_is_composited_onto_explicit_background() {
    let pixels = image::RgbaImage::from_raw(
        3,
        1,
        vec![200, 100, 50, 0, 200, 100, 50, 255, 200, 100, 50, 128],
    )
    .unwrap();
    assert_eq!(
        composite(&pixels, [20, 40, 60]),
        vec![20, 40, 60, 200, 100, 50, 110, 70, 55]
    );
}
#[test]
fn unavailable_h264_and_unusable_executable_report_without_fallback() {
    let cancel = AtomicBool::new(false);
    assert!(
        probe(&helper("no-codec"), &cancel)
            .unwrap_err()
            .contains("libx264")
    );
    let d = tempfile::tempdir().unwrap();
    let file = d.path().join("not executable.exe");
    fs::write(&file, b"text").unwrap();
    assert!(probe(&file, &cancel).unwrap_err().contains(DOWNLOAD));
}
#[test]
fn process_receives_exact_order_and_frame_bytes_and_reaps() {
    let d = tempfile::tempdir().unwrap();
    let output = d.path().join("résultat & literal.mp4");
    let s = settings("receipt-before-eof");
    let mut cmd = command(Path::new(&s.executable));
    cmd.args(arguments(&s, 60, &output));
    let mut progress = vec![];
    encode(
        &mut cmd,
        60,
        &AtomicBool::new(false),
        Duration::from_secs(3),
        |i| Ok(vec![i as u8; 64 * 48 * 3]),
        |n| progress.push(n),
    )
    .unwrap();
    assert_eq!(progress, (1..=60).collect::<Vec<_>>());
    check_mp4(&output).unwrap();
    let receipt = fs::read(output.with_extension("receipt")).unwrap();
    let start = receipt.windows(8).position(|p| p == b"\nPIXELS\n").unwrap() + 8;
    let raw = &receipt[start..];
    assert_eq!(raw.len(), 60 * 64 * 48 * 3);
    for (i, frame) in raw.chunks_exact(64 * 48 * 3).enumerate() {
        assert!(frame.iter().all(|b| *b == i as u8));
    }
    // Removing a launched executable also succeeds once our child has been reaped.
    fs::remove_file(&s.executable).unwrap();
}
#[test]
fn stderr_nonzero_early_exit_and_truncation_are_reported() {
    for mode in ["early", "stderr", "fail"] {
        let d = tempfile::tempdir().unwrap();
        let s = settings(mode);
        let output = d.path().join("out.mp4");
        let mut cmd = command(Path::new(&s.executable));
        cmd.args(arguments(&s, 3, &output));
        let error = encode(
            &mut cmd,
            3,
            &AtomicBool::new(false),
            Duration::from_secs(3),
            |_| Ok(vec![0; 64 * 48 * 3]),
            |_| {},
        )
        .unwrap_err();
        assert!(
            error.contains(if mode == "fail" {
                "Encoding rejected"
            } else {
                "FAKE ENCODER ERROR 37"
            }),
            "{error}"
        );
        if mode == "stderr" {
            assert!(error.contains("truncated"));
            assert!(error.contains("END OF DIAGNOSTIC"));
        }
        fs::remove_file(s.executable).unwrap();
    }
}
#[test]
fn blocked_stdin_is_killed_reaped_and_pipes_joined() {
    let d = tempfile::tempdir().unwrap();
    let s = settings("block");
    let mut cmd = command(Path::new(&s.executable));
    cmd.args(arguments(&s, 2, &d.path().join("out.mp4")));
    let start = Instant::now();
    let error = encode(
        &mut cmd,
        2,
        &AtomicBool::new(false),
        Duration::from_millis(150),
        |_| Ok(vec![0; 4 * 1024 * 1024]),
        |_| {},
    )
    .unwrap_err();
    assert!(error.contains("stalled"), "{error}");
    assert!(start.elapsed() < Duration::from_secs(4));
    fs::remove_file(s.executable).unwrap();
}
#[test]
fn cancellation_during_blocked_encoding_kills_and_reaps() {
    let d = tempfile::tempdir().unwrap();
    let s = settings("block");
    let cancel = AtomicBool::new(false);
    thread::scope(|scope| {
        scope.spawn(|| {
            thread::sleep(Duration::from_millis(100));
            cancel.store(true, Ordering::Relaxed);
        });
        let mut cmd = command(Path::new(&s.executable));
        cmd.args(arguments(&s, 2, &d.path().join("out.mp4")));
        assert!(
            encode(
                &mut cmd,
                2,
                &cancel,
                Duration::from_secs(3),
                |_| Ok(vec![0; 4 * 1024 * 1024]),
                |_| {}
            )
            .unwrap_err()
            .contains("cancelled")
        );
    });
    fs::remove_file(s.executable).unwrap();
}
#[test]
fn cancellation_before_launch_and_during_framing_writes_nothing() {
    let doc = walker();
    let d = tempfile::tempdir().unwrap();
    let s = settings("ok");
    let output = d.path().join("film.mp4");
    let cancel = AtomicBool::new(true);
    assert!(
        export(&doc, &s, &output, &cancel, |_, _, _| {})
            .unwrap_err()
            .contains("cancelled")
    );
    cancel.store(false, Ordering::Relaxed);
    assert!(
        export(&doc, &s, &output, &cancel, |phase, _, _| {
            if phase == "Framing" {
                cancel.store(true, Ordering::Relaxed);
            }
        })
        .is_err()
    );
    assert_eq!(fs::read_dir(d.path()).unwrap().count(), 0);
}
#[test]
fn cancellation_during_frame_production_preserves_existing_output() {
    let doc = walker();
    let d = tempfile::tempdir().unwrap();
    let s = settings("ok");
    let output = d.path().join("film.mp4");
    let cancel = AtomicBool::new(false);
    let result = export(&doc, &s, &output, &cancel, |phase, done, _| {
        if phase == "Encoding" && done == 1 {
            cancel.store(true, Ordering::Relaxed);
        }
    });
    assert!(result.unwrap_err().contains("cancelled"));
    assert!(!output.exists());
    assert_eq!(fs::read_dir(d.path()).unwrap().count(), 0);
    fs::remove_file(s.executable).unwrap();
}
#[test]
fn successful_fake_export_preserves_author_data_and_samples_each_cycle_identically() {
    let doc = walker();
    let before = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    let s = settings("ok");
    let d = tempfile::tempdir().unwrap();
    let out = d.path().join("été film.mp4");
    assert_eq!(
        export(&doc, &s, &out, &AtomicBool::new(false), |_, _, _| {}).unwrap(),
        60
    );
    check_mp4(&out).unwrap();
    assert_eq!(
        before,
        ankhimate_formats::convert::to_schema(&doc.as_project_ref())
    );
    assert_eq!(fs::read_dir(d.path()).unwrap().count(), 1);
    let sequence = Sequence::prepare(&doc, &s.request, |_, _| true).unwrap();
    let samples: (Vec<_>, Vec<_>) = (0..60)
        .map(|i| {
            composite(
                &sequence.render_rgba(i % sequence.frame_count()).unwrap(),
                s.background,
            )
        })
        .enumerate()
        .partition(|(i, _)| *i < 30);
    assert_eq!(
        samples.0.into_iter().map(|(_, p)| p).collect::<Vec<_>>(),
        samples.1.into_iter().map(|(_, p)| p).collect::<Vec<_>>()
    );
}
#[test]
fn publication_failure_existing_file_and_race_never_replace() {
    let doc = walker();
    let d = tempfile::tempdir().unwrap();
    let out = d.path().join("film.mp4");
    let s = settings("ok");
    fs::write(&out, b"old").unwrap();
    assert!(
        export(&doc, &s, &out, &AtomicBool::new(false), |_, _, _| {})
            .unwrap_err()
            .contains("exists")
    );
    assert_eq!(fs::read(&out).unwrap(), b"old");
    fs::remove_file(&out).unwrap();
    let s = settings("race");
    assert!(export(&doc, &s, &out, &AtomicBool::new(false), |_, _, _| {}).is_err());
    assert_eq!(fs::read(out).unwrap(), b"race winner");
}
#[test]
fn empty_invalid_and_failed_encodes_never_publish() {
    let doc = walker();
    for mode in ["empty", "invalid", "fail"] {
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("film.mp4");
        let s = settings(mode);
        assert!(export(&doc, &s, &out, &AtomicBool::new(false), |_, _, _| {}).is_err());
        assert!(!out.exists());
    }
}
#[test]
fn real_walker_mp4_if_available() {
    let Ok(exe) = locate("") else {
        eprintln!("REAL MP4 NOT RUN: FFmpeg missing");
        return;
    };
    if let Err(e) = probe(&exe, &AtomicBool::new(false)) {
        eprintln!("REAL MP4 NOT RUN: {e}");
        return;
    }
    let d = tempfile::tempdir().unwrap();
    let output = d.path().join("walker.mp4");
    let mut s = Settings {
        request: SequenceRequest {
            animation: "walk".into(),
            fps: 30,
            width: 64,
            height: 48,
        },
        cycles: 2,
        executable: exe.to_string_lossy().into_owned(),
        ..Default::default()
    };
    s.background = [210, 220, 230];
    assert_eq!(
        export(
            &walker(),
            &s,
            &output,
            &AtomicBool::new(false),
            |_, _, _| {}
        )
        .unwrap(),
        60
    );
    // Decode with the same external installation, no ffprobe dependency required.
    let decoded = command(&exe)
        .args(["-v", "error", "-i"])
        .arg(&output)
        .args(["-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1"])
        .output()
        .unwrap();
    assert!(
        decoded.status.success(),
        "{}",
        String::from_utf8_lossy(&decoded.stderr)
    );
    assert_eq!(decoded.stdout.len(), 60 * 64 * 48 * 3);
    let probe_path = exe.with_file_name(if cfg!(windows) {
        "ffprobe.exe"
    } else {
        "ffprobe"
    });
    if probe_path.is_file() {
        let p = command(&probe_path)
            .args([
                "-v",
                "error",
                "-show_streams",
                "-show_format",
                "-of",
                "json",
            ])
            .arg(&output)
            .output()
            .unwrap();
        assert!(p.status.success());
        let data: serde_json::Value = serde_json::from_slice(&p.stdout).unwrap();
        let v = &data["streams"][0];
        assert_eq!(v["codec_name"], "h264");
        assert_eq!(v["width"], 64);
        assert_eq!(v["height"], 48);
        let duration = data["format"]["duration"]
            .as_str()
            .unwrap()
            .parse::<f64>()
            .unwrap();
        assert!((duration - 2.0).abs() <= 1.0 / 30.0);
        assert_eq!(data["streams"].as_array().unwrap().len(), 1);
    }
}
#[test]
fn stalled_finalization_is_killed_after_eof() {
    let d = tempfile::tempdir().unwrap();
    let s = settings("finish-stall");
    let mut cmd = command(Path::new(&s.executable));
    cmd.args(arguments(&s, 1, &d.path().join("out.mp4")));
    let error = encode(
        &mut cmd,
        1,
        &AtomicBool::new(false),
        Duration::from_millis(150),
        |_| Ok(vec![0; 64 * 48 * 3]),
        |_| {},
    )
    .unwrap_err();
    assert!(error.contains("finalization stalled"), "{error}");
    fs::remove_file(s.executable).unwrap();
}
#[test]
fn cancellation_during_dependency_probe_reaps_process() {
    let exe = helper("probe-block");
    let cancel = AtomicBool::new(false);
    thread::scope(|scope| {
        scope.spawn(|| {
            thread::sleep(Duration::from_millis(100));
            cancel.store(true, Ordering::Relaxed);
        });
        assert!(probe(&exe, &cancel).unwrap_err().contains("cancelled"));
    });
    fs::remove_file(exe).unwrap();
}
#[test]
fn renderer_error_kills_process_and_does_not_hide_failure() {
    let d = tempfile::tempdir().unwrap();
    let s = settings("ok");
    let mut cmd = command(Path::new(&s.executable));
    cmd.args(arguments(&s, 2, &d.path().join("out.mp4")));
    let error = encode(
        &mut cmd,
        2,
        &AtomicBool::new(false),
        Duration::from_secs(3),
        |_| Err("controlled render failure".into()),
        |_| {},
    )
    .unwrap_err();
    assert!(error.contains("controlled render failure"));
    fs::remove_file(s.executable).unwrap();
}
#[test]
fn physics_blending_and_missing_assets_keep_existing_refusals() {
    let doc = walker();
    let s = settings("ok");
    let mut blend = doc.clone();
    blend.skeleton.slots.values_mut().next().unwrap().blend_mode =
        ankhimate_core::slot::BlendMode::Additive;
    assert!(s.count(&blend).unwrap_err().contains("Normal"));
    let mut physics = doc.clone();
    let bone = physics.skeleton.bones.keys().next().unwrap();
    physics
        .skeleton
        .constraints
        .insert(ankhimate_core::constraints::Constraint::Physics(
            ankhimate_core::constraints::PhysicsConstraint::sway("physics", bone),
        ));
    assert!(s.count(&physics).unwrap_err().contains("physics"));
    let mut missing = doc;
    missing.assets.images.clear();
    let d = tempfile::tempdir().unwrap();
    assert!(
        export(
            &missing,
            &s,
            &d.path().join("film.mp4"),
            &AtomicBool::new(false),
            |_, _, _| {}
        )
        .is_err()
    );
    assert_eq!(fs::read_dir(d.path()).unwrap().count(), 0);
}
#[test]
fn video_job_preserves_author_history_selection_and_cancels_on_new_document() {
    use ankhimate_core::{math::Transform, skeleton::Bone};
    let mut state = crate::app_state::AppState {
        doc: walker(),
        ..Default::default()
    };
    state.dispatch(Box::new(
        ankhimate_document::commands::bone_cmds::CreateBone::new(Bone {
            name: "history".into(),
            parent: None,
            length: 1.0,
            local_transform: Transform::default(),
            inherit: Default::default(),
            color: Bone::default_color(),
        }),
    ));
    let project = ankhimate_formats::convert::to_schema(&state.doc.as_project_ref());
    let assets = serde_json::to_vec(&state.doc.assets).unwrap();
    let selection = state.session.selection.clone();
    let history_depth = state.history.undo_depth();
    let d = tempfile::tempdir().unwrap();
    let s = settings("block");
    state.session.export.png.start_video(
        state.doc.clone(),
        s.clone(),
        d.path().join("film.mp4"),
        eframe::egui::Context::default(),
    );
    // Ordinary project edits/history are unaffected while a snapshot exports.
    assert_eq!(
        project,
        ankhimate_formats::convert::to_schema(&state.doc.as_project_ref())
    );
    assert_eq!(assets, serde_json::to_vec(&state.doc.assets).unwrap());
    assert_eq!(selection, state.session.selection);
    assert_eq!(history_depth, state.history.undo_depth());
    assert!(state.history.can_undo());
    state.reset_rig_session();
    assert!(state.session.export.png.is_running());
    let deadline = Instant::now() + Duration::from_secs(4);
    while state.session.export.png.is_running() {
        state.session.export.png.poll();
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(5));
    }
    assert!(
        state
            .session
            .export
            .png
            .message
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap_err()
            .contains("cancelled")
    );
    assert!(!d.path().join("film.mp4").exists());
}
#[test]
#[cfg(windows)]
fn script_custom_path_cannot_implicitly_launch_a_shell() {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("ffmpeg.cmd");
    fs::write(&path, b"exit /b 0").unwrap();
    assert!(locate(path.to_str().unwrap()).is_err());
}
#[test]
fn invalid_extension_and_empty_clip_refuse_before_output() {
    let mut doc = walker();
    let d = tempfile::tempdir().unwrap();
    let mut s = settings("ok");
    assert!(
        export(
            &doc,
            &s,
            &d.path().join("movie.gif"),
            &AtomicBool::new(false),
            |_, _, _| {}
        )
        .unwrap_err()
        .contains(".mp4")
    );
    doc.animations.clear();
    assert!(s.count(&doc).is_err());
    doc.animations
        .insert(ankhimate_core::animation::Animation::new("empty", 0.0));
    s.request.animation = "empty".into();
    assert!(s.count(&doc).is_err());
    assert_eq!(fs::read_dir(d.path()).unwrap().count(), 0);
}
#[test]
fn quality_modes_bounds_and_no_conflicting_encoder_arguments() {
    let doc = walker();
    let mut s = settings("ok");
    assert_eq!(s.quality, QualityMode::Crf);
    assert_eq!(s.encoder, Encoder::Libx264);
    assert_eq!(s.crf, 23);
    assert!(!s.open_after_export);
    for crf in [0, 51] {
        s.crf = crf;
        assert!(s.count(&doc).is_ok());
    }
    s.crf = 52;
    assert!(s.count(&doc).is_err());
    s.quality = QualityMode::TargetBitrate;
    for bitrate in [MIN_BITRATE_KBPS, MAX_BITRATE_KBPS] {
        s.bitrate_kbps = bitrate;
        assert!(s.count(&doc).is_ok());
    }
    for bitrate in [0, MIN_BITRATE_KBPS - 1, MAX_BITRATE_KBPS + 1, u32::MAX] {
        s.bitrate_kbps = bitrate;
        assert!(s.count(&doc).is_err());
    }
    s.bitrate_kbps = 2000;
    let args = arguments(&s, 60, Path::new("out.mp4"));
    assert!(!args.iter().any(|v| v == "-crf"));
    assert!(
        args.windows(2)
            .any(|w| w == [OsString::from("-b:v"), OsString::from("2000k")])
    );
    s.quality = QualityMode::Crf;
    let args = arguments(&s, 60, Path::new("out.mp4"));
    assert!(!args.iter().any(|v| v == "-b:v"));
}
#[test]
fn default_profile_arguments_are_exactly_the_accepted_historical_vector() {
    let s = Settings {
        request: SequenceRequest {
            animation: "walk".into(),
            fps: 30,
            width: 256,
            height: 256,
        },
        cycles: 2,
        ..Default::default()
    };
    let expected = [
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
        "256x256",
        "-framerate",
        "30",
        "-i",
        "pipe:0",
        "-an",
        "-c:v",
        "libx264",
        "-preset",
        "medium",
        "-crf",
        "23",
        "-pix_fmt",
        "yuv420p",
        "-frames:v",
        "60",
        "-movflags",
        "+faststart",
        "-f",
        "mp4",
        "output.mp4",
    ];
    assert_eq!(
        arguments(&s, 60, Path::new("output.mp4")),
        expected.into_iter().map(OsString::from).collect::<Vec<_>>()
    );
}
#[test]
fn selected_rgb_encoder_requires_its_own_capability_and_never_falls_back() {
    let cancel = AtomicBool::new(false);
    assert!(
        probe_encoder(&helper("no-rgb"), Encoder::Libx264Rgb, &cancel)
            .unwrap_err()
            .contains("libx264rgb")
    );
    assert!(probe_encoder(&helper("no-codec"), Encoder::Libx264Rgb, &cancel).is_ok());
    let mut s = settings("ok");
    s.encoder = Encoder::Libx264Rgb;
    let args = arguments(&s, 60, Path::new("rgb.mp4"));
    assert!(
        args.windows(2)
            .any(|w| w == [OsString::from("-c:v"), OsString::from("libx264rgb")])
    );
    assert!(
        args.windows(2)
            .any(|w| w == [OsString::from("-pix_fmt"), OsString::from("rgb24")])
    );
}
#[test]
fn real_bitrate_and_rgb_profiles_encode_and_decode_when_available() {
    let Ok(exe) = locate("") else {
        eprintln!("REAL EXTRA PROFILES NOT RUN: FFmpeg absent");
        return;
    };
    let doc = walker();
    for (i, encoder, quality) in [
        (0, Encoder::Libx264, QualityMode::TargetBitrate),
        (1, Encoder::Libx264Rgb, QualityMode::Crf),
        (2, Encoder::Libx264Rgb, QualityMode::TargetBitrate),
    ] {
        if let Err(e) = probe_encoder(&exe, encoder, &AtomicBool::new(false)) {
            eprintln!("REAL PROFILE NOT RUN: {e}");
            continue;
        }
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join(format!("profile{i}.mp4"));
        let s = Settings {
            encoder,
            quality,
            bitrate_kbps: 200,
            request: SequenceRequest {
                animation: "walk".into(),
                fps: 30,
                width: 64,
                height: 48,
            },
            cycles: 2,
            executable: exe.to_string_lossy().into_owned(),
            ..Default::default()
        };
        assert_eq!(
            export(&doc, &s, &out, &AtomicBool::new(false), |_, _, _| {}).unwrap(),
            60
        );
        let decoded = command(&exe)
            .args(["-v", "error", "-i"])
            .arg(&out)
            .args(["-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1"])
            .output()
            .unwrap();
        assert!(
            decoded.status.success(),
            "{}",
            String::from_utf8_lossy(&decoded.stderr)
        );
        assert_eq!(decoded.stdout.len(), 60 * 64 * 48 * 3);
    }
}
