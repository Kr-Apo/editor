//! Make modified copies of historical RAW-channel PSDs, never overwrite fixtures.
#[derive(Clone)]
pub struct Layer {
    pub bounds: [i32; 4],
    pub channels: Vec<(i16, Vec<u8>)>,
    suffix: Vec<u8>,
}
fn u32_at(b: &[u8], p: usize) -> usize {
    u32::from_be_bytes(b[p..p + 4].try_into().unwrap()) as usize
}
impl Layer {
    pub fn name(&self) -> &str {
        let n = self.suffix[24] as usize;
        std::str::from_utf8(&self.suffix[25..25 + n]).unwrap()
    }
    pub fn rename(&mut self, name: &str) {
        let old = 1 + self.suffix[24] as usize;
        let old = (old + 3) & !3;
        let mut pascal = vec![name.len() as u8];
        pascal.extend(name.as_bytes());
        while !pascal.len().is_multiple_of(4) {
            pascal.push(0);
        }
        self.suffix.splice(24..24 + old, pascal);
        let len = (self.suffix.len() - 16) as u32;
        self.suffix[12..16].copy_from_slice(&len.to_be_bytes());
    }
    pub fn recolor(&mut self) {
        let (_, red) = self.channels.iter_mut().find(|(id, _)| *id == 0).unwrap();
        assert_eq!(&red[..2], &[0, 0]);
        for byte in &mut red[2..] {
            *byte ^= 0x35;
        }
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        self.bounds[2] = self.bounds[0] + h as i32;
        self.bounds[3] = self.bounds[1] + w as i32;
        for (_, plane) in &mut self.channels {
            assert_eq!(&plane[..2], &[0, 0]);
            plane.resize(2 + w * h, plane[2]);
        }
    }
    pub fn hide(&mut self) {
        self.suffix[10] |= 2;
    }
}
pub fn edit(source: &[u8], change: impl FnOnce(&mut Vec<Layer>)) -> Vec<u8> {
    assert_eq!(&source[..4], b"8BPS");
    let mut p = 26;
    for _ in 0..2 {
        p += 4 + u32_at(source, p);
    }
    let prefix = p;
    let end = p + 4 + u32_at(source, p);
    p += 8;
    let count = i16::from_be_bytes(source[p..p + 2].try_into().unwrap()).unsigned_abs() as usize;
    p += 2;
    let mut layers = vec![];
    let mut lengths = vec![];
    for _ in 0..count {
        let bounds = std::array::from_fn(|i| {
            i32::from_be_bytes(source[p + 4 * i..p + 4 * i + 4].try_into().unwrap())
        });
        p += 16;
        let n = u16::from_be_bytes(source[p..p + 2].try_into().unwrap()) as usize;
        p += 2;
        let mut channels = vec![];
        let mut lens = vec![];
        for _ in 0..n {
            channels.push((
                i16::from_be_bytes(source[p..p + 2].try_into().unwrap()),
                vec![],
            ));
            lens.push(u32_at(source, p + 2));
            p += 6;
        }
        let stop = p + 16 + u32_at(source, p + 12);
        layers.push(Layer {
            bounds,
            channels,
            suffix: source[p..stop].to_vec(),
        });
        lengths.push(lens);
        p = stop;
    }
    for (layer, lens) in layers.iter_mut().zip(lengths) {
        for ((_, data), len) in layer.channels.iter_mut().zip(lens) {
            *data = source[p..p + len].to_vec();
            p += len;
        }
    }
    change(&mut layers);
    let mut info = (layers.len() as i16).to_be_bytes().to_vec();
    let mut pixels = vec![];
    for layer in layers {
        for v in layer.bounds {
            info.extend(v.to_be_bytes());
        }
        info.extend((layer.channels.len() as u16).to_be_bytes());
        for (id, data) in layer.channels {
            info.extend(id.to_be_bytes());
            info.extend((data.len() as u32).to_be_bytes());
            pixels.extend(data);
        }
        info.extend(layer.suffix);
    }
    info.extend(pixels);
    if !info.len().is_multiple_of(2) {
        info.push(0);
    }
    let mut mask = (info.len() as u32).to_be_bytes().to_vec();
    mask.extend(info);
    mask.extend(0u32.to_be_bytes());
    let mut out = source[..prefix].to_vec();
    out.extend((mask.len() as u32).to_be_bytes());
    out.extend(mask);
    out.extend(&source[end..]);
    out
}
