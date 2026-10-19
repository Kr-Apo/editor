use super::*;

#[test]
fn t8105_absolute_sampling_counts_endpoints_and_repeats() {
    for (duration, fps, cycles, count) in [
        (1.0, 24, 1, 24),
        (2.0, 30, 1, 60),
        (1.01, 30, 1, 31),
        (0.1, 30, 1, 3),
        (1.0, 60, 1, 60),
        (1.0, 60, 3, 180),
    ] {
        let mut doc = tests::walker();
        let id = doc
            .animations
            .iter()
            .find(|(_, a)| a.name == "walk")
            .unwrap()
            .0;
        doc.animations[id].duration = duration;
        let settings = Settings {
            clip: Some(id),
            cycles,
            request: SequenceRequest {
                animation: "walk".into(),
                fps,
                width: 64,
                height: 48,
            },
            ..Default::default()
        };
        assert_eq!(settings.count(&doc).unwrap(), count);
        let focused = settings.snapshot(&doc).unwrap();
        let sequence = Sequence::prepare(&focused, &settings.request, |_, _| true).unwrap();
        assert_eq!(sequence.time(0).unwrap(), 0.0);
        assert!(sequence.time(sequence.frame_count() - 1).unwrap() < duration);
        assert!(sequence.time(sequence.frame_count()).is_err());
        for global_index in 0..count {
            let index = global_index % sequence.frame_count();
            assert_eq!(sequence.time(index).unwrap(), index as f32 / fps as f32);
        }
    }
}

#[test]
fn t8105_png_cpu_pixels_and_streamed_rgb_have_the_same_order_and_camera() {
    let doc = tests::walker();
    let settings = Settings {
        request: SequenceRequest {
            animation: "walk".into(),
            fps: 24,
            width: 64,
            height: 48,
        },
        background: [31, 99, 173],
        ..Default::default()
    };
    let sequence = Sequence::prepare(&doc, &settings.request, |_, _| true).unwrap();
    let camera = sequence.camera();
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("été & spaces.mp4");
    let exe = tests::helper("receipt-before-eof");
    let mut cmd = command(&exe);
    cmd.args(arguments(&settings, 24, &output));
    let mut expected = Vec::new();
    encode(
        &mut cmd,
        24,
        &AtomicBool::new(false),
        Duration::from_secs(3),
        |index| {
            let rgba = sequence.render_rgba(index).map_err(|e| e.to_string())?;
            let png = sequence.render(index).unwrap();
            let decoded = image::load_from_memory(&png.bytes).unwrap().to_rgba8();
            assert_eq!(rgba, decoded);
            assert_eq!(rgba.dimensions(), (64, 48));
            assert_eq!(rgba.len(), 64 * 48 * 4);
            assert_eq!(sequence.camera().center, camera.center);
            assert_eq!(sequence.camera().zoom, camera.zoom);
            let rgb = composite(&rgba, settings.background);
            assert_eq!(rgb.len(), 64 * 48 * 3);
            for (source, actual) in rgba.pixels().zip(rgb.chunks_exact(3)) {
                for channel in 0..3 {
                    let alpha = u32::from(source[3]);
                    let expected = (u32::from(source[channel]) * alpha
                        + u32::from(settings.background[channel]) * (255 - alpha)
                        + 127)
                        / 255;
                    assert_eq!(u32::from(actual[channel]), expected);
                }
            }
            expected.extend_from_slice(&rgb);
            Ok(rgb)
        },
        |_| {},
    )
    .unwrap();
    let receipt = fs::read(output.with_extension("receipt")).unwrap();
    let start = receipt.windows(8).position(|p| p == b"\nPIXELS\n").unwrap() + 8;
    assert_eq!(&receipt[start..], expected.as_slice());
    check_mp4(&output).unwrap();
    fs::remove_file(exe).unwrap();
}

#[test]
fn t8105_finalization_is_distinct_from_completion_and_cancellable() {
    let doc = tests::walker();
    for cancel_at_eof in [false, true] {
        let executable = tests::helper("ok");
        let settings = Settings {
            executable: executable.to_string_lossy().into_owned(),
            request: SequenceRequest {
                animation: "walk".into(),
                fps: 24,
                width: 32,
                height: 24,
            },
            ..Default::default()
        };
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("film.mp4");
        let cancel = AtomicBool::new(false);
        let mut phases = Vec::new();
        let result = export(&doc, &settings, &out, &cancel, |phase, done, total| {
            phases.push((phase, done, total));
            if phase == "Finalizing" && cancel_at_eof {
                cancel.store(true, Ordering::Relaxed);
            }
        });
        assert!(phases.contains(&("Finalizing", 24, 24)));
        if cancel_at_eof {
            assert!(result.unwrap_err().contains("cancelled"));
            assert!(!out.exists());
            assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 0);
            assert!(!phases.iter().any(|p| p.0 == "Complete"));
        } else {
            assert_eq!(result.unwrap(), 24);
            assert_eq!(phases.last(), Some(&("Complete", 24, 24)));
            check_mp4(&out).unwrap();
        }
        fs::remove_file(executable).unwrap();
    }
}
