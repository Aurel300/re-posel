use std::collections::HashMap;

use crate::patches::Patcher;

pub struct AdbIndexEntry {
    idx: usize,
    key: String,
    size: usize,
}

#[derive(Debug)]
pub enum AdbEntryKind {
    Dummy,
    Global,
    Scene,
    Code(Vec<u8>),
    String {
        raw: Vec<u8>,
        decoded: String,
        _ignore_last: bool,
    },
    Raw(Vec<u8>),
    // Empty,
}

#[derive(Debug)]
pub struct AdbEntry {
    pub name: Option<String>,
    pub open_key: bool,
    pub kind: AdbEntryKind,
    pub region: Option<AdbEntryRegion>,
    pub global: Option<AdbEntryGlobal>,
    pub scene: Option<AdbEntryScene>,
    pub xrefs: Vec<AdbXref>,
}

#[derive(Debug, Clone)]
pub struct AdbXref {
    pub other_key: String,
    pub loc: Option<usize>,
    pub kind: AdbXrefKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdbXrefPathKind {
    Animation,
    Character,
    Cursor,
    Picture,
    Sound,

    Other,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdbXrefRegionKind {
    ScreenPos,
    ScreenRegion,
    Walkmap,

    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdbXrefTextKind {
    Dialogue,
    DisplayName,
    Var,

    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdbXrefKind {
    DialogueText,
    Scene,
    GlobalR,
    GlobalW,
    GlobalWConst(u32),
    Code,
    Item,
    Text(AdbXrefTextKind),
    Path(AdbXrefPathKind),
    Region(AdbXrefRegionKind),
    ParentOf(Box<AdbXrefKind>),
}

#[derive(Default, Debug)]
pub struct AdbEntryRegion {
    pub width: Option<usize>,
    pub bg_reference: Vec<(i32, i32, String)>,
}

#[derive(Debug)]
pub struct AdbEntryGlobal {
    pub values: HashMap<u32, String>,
}

#[derive(Default, Debug)]
pub struct AdbEntryScene {
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
            global: None,
            scene: None,
            xrefs: Vec::new(),
        }
    }

    pub fn is_region(&self, key: &str) -> bool {
        matches!(self.kind, AdbEntryKind::Raw(_))
            && (self.region.is_some()
                || key.ends_with(".rp")
                || key.ends_with(".r")
                || self.xrefs.iter().any(|xref| matches!(xref.kind, AdbXrefKind::Region(_))))
    }

    pub fn is_text(&self) -> bool {
        // TODO: treat paths differently
        self.xrefs.iter().any(|xref| matches!(xref.kind, AdbXrefKind::Text(_) | AdbXrefKind::Path(_)))
    }

    pub fn is_dialogue_text(&self) -> bool {
        self.xrefs.iter().any(|xref| xref.kind == AdbXrefKind::DialogueText)
    }

    pub fn describe(&self, key: &str) -> &'static str {
        match &self.kind {
            &AdbEntryKind::Code(_) => "code",
            AdbEntryKind::String { .. } => "string",
            AdbEntryKind::Raw(_) if self.is_region(key) => "reg",
            AdbEntryKind::Raw(_) if self.is_text() || self.is_dialogue_text() => "string",
            AdbEntryKind::Raw(_) => "raw",
            AdbEntryKind::Dummy => "dummy",
            AdbEntryKind::Global => "glb",
            AdbEntryKind::Scene => "scene",
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
            AdbEntryKind::Dummy
            | AdbEntryKind::Global
            | AdbEntryKind::Scene => 0,
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
            }
            let pos = 0x14 + 0x28 * count + idx;
            let entry = db[pos..pos + size].to_vec();
            //(key, if size >= 20 && &db[pos + 6..pos + 16] == b"\x01\x05\xAD\xDE\x0C\x00\x01\x00\x00\x00" {
            if size >= 12 && &entry[8..12] == b"\xAD\xDE\x0C\x00" {
                return (key, AdbEntry::new(AdbEntryKind::Code(entry)));
            }
            (key, AdbEntry::new(AdbEntryKind::Raw(entry)))
        })
}
