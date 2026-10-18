use super::*;
use crate::video_export::tests::{helper, walker};
fn settings(mode: &str) -> Settings {
    Settings {
        format: Format::Gif,
        request: SequenceRequest {
            animation: "walk".into(),
            fps: 30,
            width: 64,
            height: 48,
        },
        cycles: 2,
        executable: helper(mode).to_string_lossy().into_owned(),
        ..Default::default()
    }
}
#[test]
fn gif_capabilities_do_not_require_h264_and_report_missing_filters() {
    let cancel = AtomicBool::new(false);
    let exe = helper("no-codec");
    probe(&exe, &cancel).unwrap();
    assert!(super::super::probe(&exe, &cancel).is_err());
    for mode in ["no-palettegen", "no-paletteuse", "no-gif-encoder", "no-png"] {
        assert!(
            probe(&helper(mode), &cancel)
                .unwrap_err()
                .contains("capability missing")
        );
    }
    assert!(
        probe(Path::new("nonexistent-ffmpeg.exe"), &cancel)
            .unwrap_err()
            .contains(DOWNLOAD)
    );
}
#[test]
fn gif_has_no_mp4_flags_and_explicit_no_repeat() {
    let s = settings("ok");
    let p = Path::new("C:/été & files/palette.png");
    let output = Path::new("C:/été & files/movie.gif");
    let a = arguments(&s, 60, p, output);
    let text: Vec<_> = a.iter().map(|s| s.to_str().unwrap()).collect();
    for pair in [
        ["-c:v", "gif"],
        ["-loop", "-1"],
        ["-gifflags", "0"],
        ["-final_delay", "3"],
    ] {
        assert!(text.windows(2).any(|w| w == pair));
    }
    for v in ["libx264", "-crf", "yuv420p", "-y"] {
        assert!(!text.contains(&v));
    }
    assert_eq!(a.last().unwrap(), output.as_os_str());
    assert!(a.contains(&p.as_os_str().to_owned()));
    let a = palette_arguments(&s, p);
    assert!(
        a.iter()
            .any(|s| s == "palettegen=max_colors=256:reserve_transparent=0:stats_mode=full")
    );
}
#[test]
fn gif_allows_odd_dimensions_and_rejects_unrepresentable_fps() {
    let doc = walker();
    let mut s = settings("ok");
    s.request.width = 63;
    s.request.height = 47;
    s.crf = 255;
    assert_eq!(s.count(&doc).unwrap(), 60);
    s.request.fps = 101;
    assert!(s.count(&doc).unwrap_err().contains("100"));
    s.request.fps = 0;
    assert!(s.count(&doc).is_err());
}
#[test]
fn boundary_quantization_preserves_total_with_varying_delays() {
    for fps in [1, 24, 25, 30, 50, 60, 100] {
        let count = fps * 2;
        let delays: Vec<_> = (0..count)
            .map(|i| boundary(i + 1, fps) - boundary(i, fps))
            .collect();
        assert_eq!(delays.iter().sum::<u64>(), 200);
        assert!(delays.iter().all(|d| *d > 0));
        assert_eq!(delays[count as usize - 1], final_delay(count, fps));
    }
    assert_eq!(
        (0..6)
            .map(|i| boundary(i + 1, 30) - boundary(i, 30))
            .collect::<Vec<_>>(),
        vec![3, 4, 3, 3, 4, 3]
    );
}
#[test]
fn both_passes_use_identical_ordered_samples_and_clean_owned_files() {
    let d = tempfile::tempdir().unwrap();
    let output = d.path().join("été film.gif");
    let s = settings("no-codec");
    let doc = walker();
    let project = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    let assets = serde_json::to_vec(&doc.assets).unwrap();
    let mut phases = vec![];
    assert_eq!(
        export(&doc, &s, &output, &AtomicBool::new(false), |p, _, _| phases
            .push(p))
        .unwrap(),
        60
    );
    let info = inspect(&output).unwrap();
    assert_eq!(info.frames, 60);
    assert_eq!(info.total_delay_cs, 200);
    assert_eq!((info.min_delay, info.max_delay), (3, 4));
    assert_eq!(info.repeat, None);
    assert!(!info.transparent);
    assert!(
        phases.iter().position(|p| *p == "Palette ready").unwrap()
            < phases.iter().position(|p| *p == "GIF encoding").unwrap()
    );
    assert_eq!(fs::read_dir(d.path()).unwrap().count(), 1);
    assert_eq!(
        project,
        ankhimate_formats::convert::to_schema(&doc.as_project_ref())
    );
    assert_eq!(assets, serde_json::to_vec(&doc.assets).unwrap());
}
#[test]
fn failures_in_each_pass_preserve_diagnostics_and_never_publish() {
    for mode in [
        "palette-early",
        "gif-early",
        "palette-stderr",
        "gif-stderr",
        "palette-fail",
        "gif-fail",
        "palette-empty",
        "gif-empty",
        "palette-invalid",
        "gif-invalid",
    ] {
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("film.gif");
        let s = settings(mode);
        let error = export(&walker(), &s, &out, &AtomicBool::new(false), |_, _, _| {}).unwrap_err();
        if mode.contains("stderr") {
            assert!(error.contains("truncated"));
            assert!(error.contains("FAKE ENCODER ERROR 37"));
        }
        if mode.contains("early") || mode.contains("fail") {
            assert!(error.contains("FFmpeg"), "{error}");
        }
        assert!(!out.exists(), "{mode}");
        assert_eq!(
            fs::read_dir(d.path()).unwrap().count(),
            0,
            "{mode}: {error}"
        );
        fs::remove_file(s.executable).unwrap();
    }
}
#[test]
fn cancel_during_each_pass_and_between_passes_cleans_palette_and_output() {
    for phase in ["Palette generation", "Palette ready", "GIF encoding"] {
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("film.gif");
        let s = settings("ok");
        let cancel = AtomicBool::new(false);
        let error = export(&walker(), &s, &out, &cancel, |p, n, _| {
            if p == phase && n > 0 {
                cancel.store(true, Ordering::Relaxed);
            }
        })
        .unwrap_err();
        assert!(error.contains("cancelled"), "{error}");
        assert_eq!(fs::read_dir(d.path()).unwrap().count(), 0);
        fs::remove_file(s.executable).unwrap();
    }
}
#[test]
fn blocked_or_stalled_process_in_both_passes_can_be_killed_and_reaped() {
    for mode in [
        "palette-block",
        "gif-block",
        "palette-finish-stall",
        "gif-finish-stall",
    ] {
        let d = tempfile::tempdir().unwrap();
        let s = settings(mode);
        let palette = d.path().join("palette.png");
        let gif = d.path().join("film.gif");
        let args = if mode.starts_with("palette-") {
            palette_arguments(&s, &palette)
        } else {
            arguments(&s, 1, &palette, &gif)
        };
        let mut cmd = command(Path::new(&s.executable));
        cmd.args(args);
        let start = Instant::now();
        let error = encode(
            &mut cmd,
            1,
            &AtomicBool::new(false),
            Duration::from_millis(200),
            |_| Ok(vec![0; 4 * 1024 * 1024]),
            |_| {},
        )
        .unwrap_err();
        assert!(error.contains("stalled"), "{mode}: {error}");
        assert!(start.elapsed() < Duration::from_secs(4));
        fs::remove_file(s.executable).unwrap();
    }
}
#[test]
fn gif_existing_and_racing_destinations_are_preserved() {
    let d = tempfile::tempdir().unwrap();
    let out = d.path().join("film.gif");
    fs::write(&out, b"old GIF").unwrap();
    let s = settings("ok");
    assert!(export(&walker(), &s, &out, &AtomicBool::new(false), |_, _, _| {}).is_err());
    assert_eq!(fs::read(&out).unwrap(), b"old GIF");
    fs::remove_file(&out).unwrap();
    let s = settings("gif-race");
    assert!(
        export(&walker(), &s, &out, &AtomicBool::new(false), |_, _, _| {})
            .unwrap_err()
            .contains("exists")
    );
    assert_eq!(fs::read(&out).unwrap(), b"race winner");
}
#[test]
fn homonyms_use_selected_id_for_both_formats_and_legacy_names_refuse() {
    let mut doc = walker();
    let first = doc
        .animations
        .iter()
        .find(|(_, a)| a.name == "walk")
        .unwrap()
        .0;
    let mut duplicate = doc.animations[first].clone();
    duplicate.duration = 2.0;
    let chosen = doc.animations.insert(duplicate);
    let before = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    for format in [Format::Mp4, Format::Gif] {
        let mut s = settings("ok");
        s.format = format;
        assert!(s.count(&doc).unwrap_err().contains("Ambiguous"));
        s.clip = Some(chosen);
        assert_eq!(s.count(&doc).unwrap(), 120);
        let snapshot = s.snapshot(&doc).unwrap();
        assert_eq!(snapshot.animations.len(), 1);
        assert!(snapshot.animations.contains_key(chosen));
        assert!(!snapshot.animations.contains_key(first));
        assert_eq!(snapshot.animations[chosen].duration, 2.0);
    }
    assert_eq!(
        before,
        ankhimate_formats::convert::to_schema(&doc.as_project_ref())
    );
    doc.animations.remove(chosen);
    let mut removed = settings("ok");
    removed.clip = Some(chosen);
    assert!(
        removed
            .count(&doc)
            .unwrap_err()
            .contains("no longer exists")
    );
}
#[test]
fn gif_signature_is_insufficient_and_truncated_blocks_refuse() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("truncated.gif");
    for bytes in [
        b"GIF89a".as_slice(),
        b"GIF89a\x01\0\x01\0\0\0\0\x3b",
        b"GIF89a\x01\0\x01\0\0\0\0\x21\xf9\x04\0",
    ] {
        fs::write(&p, bytes).unwrap();
        assert!(inspect(&p).is_err());
    }
}
#[test]
fn real_palette_contains_late_frame_colour_if_ffmpeg_available() {
    let Ok(exe) = locate("") else {
        eprintln!("REAL GIF PALETTE NOT RUN: FFmpeg missing");
        return;
    };
    if let Err(e) = probe(&exe, &AtomicBool::new(false)) {
        eprintln!("REAL GIF PALETTE NOT RUN: {e}");
        return;
    }
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("palette.png");
    let s = Settings {
        format: Format::Gif,
        request: SequenceRequest {
            animation: "unused".into(),
            fps: 2,
            width: 16,
            height: 16,
        },
        ..Default::default()
    };
    let mut cmd = command(&exe);
    cmd.args(palette_arguments(&s, &p));
    encode(
        &mut cmd,
        2,
        &AtomicBool::new(false),
        Duration::from_secs(5),
        |i| Ok(if i == 0 { [255, 0, 0] } else { [0, 0, 255] }.repeat(256)),
        |_| {},
    )
    .unwrap();
    check_palette(&p).unwrap();
    let palette = image::open(&p).unwrap().to_rgb8();
    assert!(palette.pixels().any(|p| p.0 == [255, 0, 0]));
    assert!(palette.pixels().any(|p| p.0 == [0, 0, 255]));
}
#[test]
fn real_walker_gif_decodes_all_frames_with_quantized_timing_if_available() {
    let Ok(exe) = locate("") else {
        eprintln!("REAL GIF NOT RUN: FFmpeg missing");
        return;
    };
    if let Err(e) = probe(&exe, &AtomicBool::new(false)) {
        eprintln!("REAL GIF NOT RUN: {e}");
        return;
    }
    let d = tempfile::tempdir().unwrap();
    let out = d.path().join("walker.gif");
    let s = Settings {
        executable: exe.to_string_lossy().into_owned(),
        ..settings("ok")
    };
    assert_eq!(
        export(&walker(), &s, &out, &AtomicBool::new(false), |_, _, _| {}).unwrap(),
        60
    );
    let info = inspect(&out).unwrap();
    assert_eq!(info.frames, 60);
    assert_eq!((info.width, info.height), (64, 48));
    assert_eq!(info.total_delay_cs, 200);
    assert_eq!((info.min_delay, info.max_delay), (3, 4));
    assert!(!info.transparent);
    assert_eq!(info.repeat, None);
    let decoded = command(&exe)
        .args(["-v", "error", "-i"])
        .arg(&out)
        .args([
            "-fps_mode",
            "passthrough",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "pipe:1",
        ])
        .output()
        .unwrap();
    assert!(
        decoded.status.success(),
        "{}",
        String::from_utf8_lossy(&decoded.stderr)
    );
    assert_eq!(decoded.stdout.len(), 60 * 64 * 48 * 3);
}
#[test]
fn selected_homonym_is_actually_exported_without_losing_other_clips() {
    let mut doc = walker();
    let mut clip = doc
        .animations
        .values()
        .find(|a| a.name == "walk")
        .unwrap()
        .clone();
    clip.duration = 2.0;
    let id = doc.animations.insert(clip);
    let mut s = settings("ok");
    s.clip = Some(id);
    let d = tempfile::tempdir().unwrap();
    let out = d.path().join("chosen.gif");
    let before = ankhimate_formats::convert::to_schema(&doc.as_project_ref());
    assert_eq!(
        export(&doc, &s, &out, &AtomicBool::new(false), |_, _, _| {}).unwrap(),
        120
    );
    assert_eq!(inspect(&out).unwrap().frames, 120);
    assert_eq!(inspect(&out).unwrap().total_delay_cs, 400);
    assert_eq!(
        before,
        ankhimate_formats::convert::to_schema(&doc.as_project_ref())
    );
}
