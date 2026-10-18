use ankhimate_export::{
    run::{self, Plan},
    template::RenderedFile,
};
use image::{ImageEncoder, ImageFormat};
use std::sync::atomic::{AtomicBool, Ordering};
fn plan() -> Plan {
    Plan {
        files: vec![RenderedFile {
            path: "sequence.json".into(),
            contents: "{}".into(),
        }],
        binaries: ["frame_000000.png", "frame_000001.jpg", "frame_000002.webp"]
            .into_iter()
            .map(|n| (n.into(), vec![]))
            .collect(),
    }
}
fn bytes(index: usize) -> Vec<u8> {
    let mut out = vec![];
    match index {
        0 => image::codecs::png::PngEncoder::new(&mut out)
            .write_image(&[10, 20, 30, 128], 1, 1, image::ExtendedColorType::Rgba8)
            .unwrap(),
        1 => image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 95)
            .encode(&[10, 20, 30], 1, 1, image::ExtendedColorType::Rgb8)
            .unwrap(),
        _ => image::codecs::webp::WebPEncoder::new_lossless(&mut out)
            .write_image(&[10, 20, 30, 128], 1, 1, image::ExtendedColorType::Rgba8)
            .unwrap(),
    };
    out
}
#[test]
fn generated_images_use_the_existing_transaction_and_match_their_extensions() {
    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("Égypte-蛙");
    let p = plan();
    run::write_generated_images(&p, &dest, |i, _| Ok(bytes(i)), || false).unwrap();
    for (i, (name, _)) in p.binaries.iter().enumerate() {
        let b = std::fs::read(dest.join(name)).unwrap();
        assert_eq!(b, bytes(i));
        assert_eq!(
            image::guess_format(&b).unwrap(),
            [ImageFormat::Png, ImageFormat::Jpeg, ImageFormat::WebP][i]
        );
    }
}
#[test]
fn encoding_failure_corrupt_bytes_or_wrong_format_never_publish_a_partial_set() {
    for kind in 0..3 {
        let temp = tempfile::tempdir().unwrap();
        let sentinel = temp.path().join("older.backup");
        std::fs::write(&sentinel, b"preserve backup").unwrap();
        let result = run::write_generated_images(
            &plan(),
            temp.path(),
            |i, _| {
                if i == 1 {
                    return match kind {
                        0 => Err("simulated JPEG encoding failure".into()),
                        1 => Ok(b"bad JPEG bytes".to_vec()),
                        _ => Ok(bytes(0)),
                    };
                }
                Ok(bytes(i))
            },
            || false,
        );
        assert!(result.is_err());
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"preserve backup");
        assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
    }
}
#[test]
fn cancellation_and_collisions_preserve_prior_images_and_orphans() {
    let temp = tempfile::tempdir().unwrap();
    let cancel = AtomicBool::new(false);
    std::fs::write(temp.path().join("orphan.webp"), bytes(2)).unwrap();
    assert!(
        run::write_generated_images(
            &plan(),
            temp.path(),
            |i, _| {
                if i == 1 {
                    cancel.store(true, Ordering::Relaxed);
                }
                Ok(bytes(i))
            },
            || cancel.load(Ordering::Relaxed)
        )
        .is_err()
    );
    assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
    std::fs::write(temp.path().join("frame_000001.jpg"), b"old JPG").unwrap();
    assert!(
        run::write_generated_images(&plan(), temp.path(), |i, _| Ok(bytes(i)), || false).is_err()
    );
    assert_eq!(
        std::fs::read(temp.path().join("frame_000001.jpg")).unwrap(),
        b"old JPG"
    );
    assert_eq!(
        std::fs::read(temp.path().join("orphan.webp")).unwrap(),
        bytes(2)
    );
    assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 2);
}
#[test]
fn unsupported_suffix_unsafe_path_or_precomputed_data_is_refused_before_generation() {
    for kind in 0..3 {
        let temp = tempfile::tempdir().unwrap();
        let dest = temp.path().join("unused");
        let mut p = plan();
        match kind {
            0 => p.binaries[0].0 = "frame.gif".into(),
            1 => p.binaries[0].0 = "../outside.jpg".into(),
            _ => p.binaries[0].1 = bytes(0),
        };
        let result = run::write_generated_images(
            &p,
            &dest,
            |_, _| panic!("invalid plan must not generate"),
            || false,
        );
        assert!(result.is_err());
        assert!(!dest.exists());
    }
}

#[test]
fn cancellation_after_first_publication_rolls_back_generated_images_and_metadata() {
    let temp = tempfile::tempdir().unwrap();
    let sentinel = temp.path().join("prior.backup");
    std::fs::write(&sentinel, b"last valid export").unwrap();
    let metadata = temp.path().join("sequence.json");
    let error = run::write_generated_images(
        &plan(),
        temp.path(),
        |i, _| Ok(bytes(i)),
        || metadata.exists(),
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("publication failed") && error.contains("rollback completed"),
        "{error}"
    );
    assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
    assert_eq!(std::fs::read(sentinel).unwrap(), b"last valid export");
}
