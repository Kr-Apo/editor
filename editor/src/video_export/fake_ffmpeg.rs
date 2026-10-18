// Standalone deterministic child used only by tests; never built into the editor.
use std::{
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
};
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let exe = std::env::current_exe().unwrap();
    let mode = exe.file_stem().unwrap().to_string_lossy();
    if args.iter().any(|s| s == "-encoders") {
        if mode.contains("probe-block") {
            std::thread::sleep(Duration::from_secs(300));
        }
        if !mode.contains("no-codec") {
            println!(" V....D libx264 fake test encoder");
        }
        if !mode.contains("no-rgb") {
            println!(" V..... libx264rgb fake test encoder");
        }
        if !mode.contains("no-gif-encoder") {
            println!(" V..... gif fake test encoder");
        }
        if !mode.contains("no-png") {
            println!(" V..... png fake test encoder");
        }
        return;
    }
    if args.iter().any(|s| s == "-filters") {
        if !mode.contains("no-palettegen") {
            println!(" ... palettegen V->V fake filter");
        }
        if !mode.contains("no-paletteuse") {
            println!(" ... paletteuse VV->V fake filter");
        }
        return;
    }
    let path = PathBuf::from(args.last().unwrap());

    let palette_pass = path.file_name().unwrap() == "palette.png";
    let gif_pass = path.extension().is_some_and(|s| s == "gif");
    let active = if mode.contains("palette-") {
        palette_pass
    } else if mode.contains("gif-") {
        gif_pass
    } else {
        true
    };
    // Sidecars go OUTSIDE the owned staging directory in process-only tests.
    if active && mode.contains("stderr") {
        let bytes = vec![b'x'; 1024 * 1024];
        std::io::stderr().write_all(&bytes).unwrap();
        eprintln!(" END OF DIAGNOSTIC");
    }
    if active && (mode.contains("early") || mode.contains("stderr")) {
        eprintln!("FAKE ENCODER ERROR 37");
        std::process::exit(37);
    }
    if active && mode.contains("block") {
        std::thread::sleep(Duration::from_secs(300));
        return;
    }
    let mut pixels = Vec::new();
    if mode.contains("before-eof") {
        let value = |name: &str| args[args.iter().position(|s| s == name).unwrap() + 1].clone();
        let size = value("-video_size");
        let (w, h) = size.split_once('x').unwrap();
        let bytes = w.parse::<usize>().unwrap()
            * h.parse::<usize>().unwrap()
            * 3
            * value("-frames:v").parse::<usize>().unwrap();
        pixels.resize(bytes, 0);
        std::io::stdin().read_exact(&mut pixels).unwrap();
    } else {
        std::io::stdin().read_to_end(&mut pixels).unwrap();
    }
    if active && mode.contains("fail") {
        eprintln!("Encoding rejected after reading stdin");
        std::process::exit(23);
    }
    if active && mode.contains("empty") {
        std::fs::write(&path, []).unwrap();
        return;
    }
    if active && mode.contains("invalid") {
        std::fs::write(&path, b"not an MP4").unwrap();
        return;
    }
    if active && mode.contains("race") {
        std::fs::write(
            path.parent().unwrap().parent().unwrap().join(if gif_pass {
                "film.gif"
            } else {
                "film.mp4"
            }),
            b"race winner",
        )
        .unwrap();
    }
    if active && mode.contains("finish-stall") {
        std::thread::sleep(Duration::from_secs(300));
    }
    if palette_pass {
        let mut png = PALETTE[..PALETTE.len() - 12].to_vec();
        let text = format!("input\0{:016x}", hash(&pixels));
        chunk(&mut png, b"tEXt", text.as_bytes());
        png.extend_from_slice(&PALETTE[PALETTE.len() - 12..]);
        std::fs::write(&path, png).unwrap();
        return;
    }
    if gif_pass {
        let inputs: Vec<_> = args
            .windows(2)
            .filter(|w| w[0] == "-i")
            .map(|w| &w[1])
            .collect();
        let palette = std::fs::read(inputs[1]).expect("palette must exist before GIF encoding");
        let expected = format!("input\0{:016x}", hash(&pixels));
        assert!(
            palette
                .windows(expected.len())
                .any(|w| w == expected.as_bytes()),
            "the two passes must receive identical RGB samples in identical order"
        );
        let value = |name: &str| args[args.iter().position(|s| s == name).unwrap() + 1].clone();
        let size = value("-video_size");
        let (w, h) = size.split_once('x').unwrap();
        let (w, h) = (w.parse::<u16>().unwrap(), h.parse::<u16>().unwrap());
        let count = (pixels.len() / (usize::from(w) * usize::from(h) * 3)) as u32;
        let fps = value("-framerate").parse::<u32>().unwrap();
        let mut gif = b"GIF89a".to_vec();
        gif.extend(w.to_le_bytes());
        gif.extend(h.to_le_bytes());
        gif.extend([0x80, 0, 0, 0, 0, 0, 255, 255, 255]);
        for i in 0..count {
            let boundary = |i: u32| (i * 200 + fps) / (fps * 2);
            let delay = (boundary(i + 1) - boundary(i)) as u16;
            gif.extend([0x21, 0xf9, 4, 0]);
            gif.extend(delay.to_le_bytes());
            gif.extend([0, 0, 0x2c, 0, 0, 0, 0]);
            gif.extend(w.to_le_bytes());
            gif.extend(h.to_le_bytes());
            gif.extend([0, 2, 2, 0x44, 1, 0]);
        }
        gif.push(0x3b);
        std::fs::write(path, gif).unwrap();
        return;
    }
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(b"\x00\x00\x00\x10ftypisom\0\0\0\0\0\0\0\x09moovx\0\0\0\x09mdatx")
        .unwrap();

    // Process-only tests use .mp4 outside staging and inspect this receipt.
    if mode.contains("receipt") {
        let mut f = std::fs::File::create(path.with_extension("receipt")).unwrap();
        writeln!(f, "{}", args.join("\n")).unwrap();
        f.write_all(b"\nPIXELS\n").unwrap();
        f.write_all(&pixels).unwrap();
    }
}

fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    })
}
fn chunk(png: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    png.extend((data.len() as u32).to_be_bytes());
    png.extend(kind);
    png.extend(data);
    let mut crc = 0xffff_ffffu32;
    for b in kind.iter().chain(data) {
        crc ^= u32::from(*b);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb88320
            } else {
                crc >> 1
            };
        }
    }
    png.extend((!crc).to_be_bytes());
}
const PALETTE: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 16, 0, 0, 0, 16, 8, 2,
    0, 0, 0, 144, 145, 104, 54, 0, 0, 0, 17, 73, 68, 65, 84, 120, 156, 99, 96, 24, 5, 163, 96, 20,
    48, 64, 1, 0, 3, 16, 0, 1, 124, 139, 255, 101, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];
