use crate::{patches::Patcher, xor::{dexor, XOR_KEY}};

pub struct AdbIndexEntry {
    idx: usize,
    key: String,
    size: usize,
}

#[derive(Debug)]
pub enum AdbEntryKind {
    Dummy,
    Code(Vec<u8>),
    String {
        raw: Vec<u8>,
        decoded: String,
        _ignore_last: bool,
    },
    //SmallString(String),
    //Raw(Vec<u8>),
    Raw(Vec<u8>),
    // Empty,
}

#[derive(Debug)]
pub struct AdbEntry {
    pub name: Option<String>,
    pub open_key: bool,
    pub kind: AdbEntryKind,
    pub region: Option<AdbEntryRegion>,
}

#[derive(Default, Debug)]
pub struct AdbEntryRegion {
    pub width: Option<usize>,
    pub bg_reference: Vec<(i32, i32, String)>,
}

impl AdbEntry {
    pub fn new(kind: AdbEntryKind) -> Self {
        Self {
            name: None,
            open_key: true,
            kind,
            region: None,
        }
    }

    pub fn describe(&self, key: &str) -> &'static str {
        match &self.kind {
            &AdbEntryKind::Code(_) => "code",
            AdbEntryKind::String { .. } => "string",
            AdbEntryKind::Raw(_) if self.region.is_some() || key.ends_with(".rp") || key.ends_with(".r") => "reg",
            AdbEntryKind::Raw(_) => "raw",
            AdbEntryKind::Dummy => "dummy",
        }
    }

    pub fn raw(&self) -> &[u8] {
        match &self.kind {
            AdbEntryKind::Code(v)
            | AdbEntryKind::String { raw: v, .. }
            | AdbEntryKind::Raw(v, ..) => v,
            _ => unreachable!(),
        }
    }

    pub fn size(&self) -> usize {
        match &self.kind {
            AdbEntryKind::Dummy => 0,
            AdbEntryKind::Code(raw)
            | AdbEntryKind::String { raw, .. }
            | AdbEntryKind::Raw(raw) => raw.len(),
        }
    }
}

pub fn create_patched(mut db: Vec<u8>, patcher: Patcher) -> Vec<u8> {
    let count = u32::from_le_bytes(db[8..12].try_into().unwrap()) as usize;
    for i in 0..count {
        let (idx, key, size) = {
            let pos = 0x14 + 0x28 * i;
            let idx = u32::from_le_bytes(db[pos..pos + 4].try_into().unwrap()) as usize;
            let nullbyte = db[pos + 4..pos + 36].iter().position(|b| *b == 0).unwrap_or(32) + 4;
            let key = std::str::from_utf8(&db[pos + 4..pos + nullbyte]).unwrap().to_string();
            let size = u32::from_le_bytes(db[pos + 36..pos + 40].try_into().unwrap()) as usize;
            (idx, key, size)
        };
        if size == 0 {
            unreachable!("empty entry");
            //return (key, AdbEntry::Empty);
        }
        let pos = 0x14 + 0x28 * count + idx;
        let entry = db[pos..pos + size].to_vec();
        patcher.with_data(&key, &entry, |patched, patches| {
            if !patches.is_empty() {
                println!("  patching {key}");
                db[pos..pos + size].copy_from_slice(patched);
            }
        });
    }
    db
}

pub fn extract(db: Vec<u8>) -> impl Iterator<Item = (String, AdbEntry)> {
    assert_eq!(&db[0..4], b"\x9A\x02\x00\x00", "magic mismatch");
    assert_eq!(&db[4..8], b"\x00\x00\x00\x00", "magic mismatch");
    assert_eq!(&db[8..12], &db[12..16], "count mismatch");
    let count = u32::from_le_bytes(db[8..12].try_into().unwrap()) as usize;
    assert_eq!(&db[16..20], b"\x1F\x00\x00\x00", "header size mismatch");
    (0..count)
        .map(|i| {
            let pos = 0x14 + 0x28 * i;
            let idx = u32::from_le_bytes(db[pos..pos + 4].try_into().unwrap()) as usize;
            let nullbyte = db[pos + 4..pos + 36].iter().position(|b| *b == 0).unwrap_or(32) + 4;
            let key = std::str::from_utf8(&db[pos + 4..pos + nullbyte]).unwrap().to_string();
            let size = u32::from_le_bytes(db[pos + 36..pos + 40].try_into().unwrap()) as usize;
            AdbIndexEntry { idx, key, size }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(move |AdbIndexEntry { idx, key, size }| {
            if size == 0 {
                unreachable!("empty entry");
                //return (key, AdbEntry::Empty);
            }
            let pos = 0x14 + 0x28 * count + idx;
            let mut entry = db[pos..pos + size].to_vec();
            //(key, if size >= 20 && &db[pos + 6..pos + 16] == b"\x01\x05\xAD\xDE\x0C\x00\x01\x00\x00\x00" {
            if size >= 12 && &entry[8..12] == b"\xAD\xDE\x0C\x00" {
                return (key, AdbEntry::new(AdbEntryKind::Code(entry)));
            }
            let entry_sub = &entry[0..entry.len() - 1];
            let ascii = entry_sub.iter().filter(|b| 0x0A <= **b && **b < 0x7F).count();
            let ascii_decodable = entry_sub.iter().zip(XOR_KEY.iter().cycle()).filter(|(b, k)| (*b ^ *k ^ 0xFF) <= 0x7F).count();
            let majority_ascii = ascii as f32 / (size - 1) as f32 >= 0.9;
            let majority_decoded_ascii = ascii_decodable as f32 / (size - 1) as f32 >= 0.9;
            let last_null = *entry.last().unwrap() == 0;
            let last_decoded_null = entry.last().unwrap() ^ XOR_KEY[(size - 1) % XOR_KEY.len()] ^ 0xFF == 0;
            if majority_decoded_ascii {
                dexor(&mut entry[..]);
                if last_null || last_decoded_null {
                    let decoded = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&entry[0..entry.len() - 1]).unwrap().to_string();
                    return (key, AdbEntry::new(AdbEntryKind::String {
                        raw: entry,
                        decoded,
                        _ignore_last: true,
                    }));
                }
                let decoded = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&entry[..]).unwrap().to_string();
                return (key, AdbEntry::new(AdbEntryKind::String {
                    raw: entry,
                    decoded,
                    _ignore_last: false,
                }));
            }
            if majority_ascii {
                if last_null {
                    let decoded = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&entry[0..entry.len() - 1]).unwrap().to_string();
                    return (key, AdbEntry::new(AdbEntryKind::String {
                        raw: entry,
                        decoded,
                        _ignore_last: true,
                    }));
                }
                let decoded = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&entry[..]).unwrap().to_string();
                return (key, AdbEntry::new(AdbEntryKind::String {
                    raw: entry,
                    decoded,
                    _ignore_last: false,
                }));
            }
            //let decoded = encoding_rs::WINDOWS_1250.decode_without_bom_handling_and_without_replacement(&entry[..]).unwrap().to_string();
            (key, AdbEntry::new(AdbEntryKind::Raw(entry)))//, decoded)));
            //return (key, AdbEntry::Raw(entry));
        })
}
