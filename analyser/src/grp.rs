use std::collections::HashSet;

use crate::xor::dexor;

fn trimnull(data: &[u8]) -> &[u8] {
    if let Some(idx) = data.iter().position(|e| *e == 0) {
        return &data[..idx];
    }
    data
}

pub fn extract(
    data: Vec<u8>,
    filter: Option<HashSet<String>>,
) -> impl Iterator<Item = (String, Vec<u8>)> {
    const MAGIC1: &[u8] = b"AGDS group file\x1A";
    const MAGIC2: &[u8] = b"\xE6\xC9\x03\x1A";
    const VERSION1: u32 = 0x2C;
    const VERSION2: u32 = 0x02;

    let header = data[0..0x2C].to_vec();
    let mut xor_buf = data[0..0x10].to_vec();
    dexor(&mut xor_buf);
    assert!((&header[0..0x10] == MAGIC1) || (xor_buf == MAGIC1), "magic mismatch");
    let encrypted = xor_buf == MAGIC1;
    let version1 = u32::from_le_bytes(header[0x10..0x14].try_into().unwrap());
    assert_eq!(version1, VERSION1, "version mismatch");
    assert_eq!(&header[0x14..0x18], MAGIC2, "magic mismatch");
    let version2 = u32::from_le_bytes(header[0x18..0x1C].try_into().unwrap());
    assert_eq!(version2, VERSION2, "version mismatch");
    let count = u32::from_le_bytes(header[0x1C..0x20].try_into().unwrap()) as usize;

    (0..count)
        .filter_map(move |file_idx| {
            let fhpos = header.len() + file_idx * 0x31;
            let file_header = &data[fhpos..fhpos + 0x31];
            let mut name_buf = trimnull(&file_header[0..0x21]).to_vec();
            if encrypted {
                dexor(&mut name_buf);
            }
            let name = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&name_buf).unwrap().to_string();
            if filter.as_ref().map(|f| !f.contains(&name)).unwrap_or(false) {
                return None;
            }
            assert!(!name.contains(".."));
            assert!(!name.contains("/"));
            assert!(!name.contains("\\"));
            let offset = u32::from_le_bytes(file_header[0x21..0x25].try_into().unwrap()) as usize;
            let size = u32::from_le_bytes(file_header[0x25..0x29].try_into().unwrap()) as usize;
            Some((name, data[offset..offset + size].to_vec()))
        })
}
