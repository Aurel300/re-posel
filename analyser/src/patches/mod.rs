use std::{collections::HashMap, ops::Range};

mod chapter_select;
mod check_again;
mod skip_intros;

#[derive(Clone)]
pub enum PatchChange<'a> {
    DataModify {
        key: &'a str,
        range: Range<usize>,
        content: &'a [u8],
    },
    DataZero {
        key: &'a str,
        range: Range<usize>,
    },
}

impl<'a> PatchChange<'a> {
    pub fn range(&self) -> Range<usize> {
        match self {
            Self::DataModify { range, .. }
            | Self::DataZero { range, .. } => range.clone(),
        }
    }
}

#[allow(dead_code)]
pub struct Patch<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub changes: &'a [PatchChange<'a>],
}

/*
impl<'a> Patch<'a> {
    pub const fn new_data(description: &str, data: &str, changes: &[(Range<usize>, &[u8])]) -> Self {
        Self {
            description: description.to_string(),
            changes: changes.iter()
                .map(|(r, b)| PatchChange::DataModify {
                    key: data.to_string(),
                    range: r.clone(),
                    content: b.to_vec(),
                })
                .collect(),
        }
    }
}
*/

pub const ACTIVE_PATCHES: &[&Patch<'static>] = &[
    &chapter_select::PATCH,
    &check_again::PATCH,
    &skip_intros::PATCH,
];

pub struct Patcher<'a> {
    //patches: &'a [&'a Patch<'a>],
    data_affected: HashMap<String, Vec<(&'a Patch<'a>, &'a PatchChange<'a>)>>,
}

impl<'a> Patcher<'a> {
    pub fn new() -> Self {
        Self {
            data_affected:  HashMap::new(),
        }
    }

    pub fn add_patch(&mut self, patch: &'a Patch<'a>) {
        for change in patch.changes {
            match change {
                PatchChange::DataModify { key, .. }
                | PatchChange::DataZero { key, .. } => {
                    self.data_affected.entry(key.to_string())
                        .or_default()
                        .push((patch, change));
                }
            }
        }
    }

    pub fn with_data<R>(&self, key: &str, original: &[u8], f: impl FnOnce(&[u8], &[(&Patch<'a>, &PatchChange<'a>)]) -> R) -> R {
        if let Some(changes) = self.data_affected.get(key) {
            let mut patched = original.to_vec();
            for (_, change) in changes {
                match change {
                    PatchChange::DataModify { range, content, .. } => {
                        patched[range.clone()].copy_from_slice(content);
                    }
                    PatchChange::DataZero { range, .. } => {
                        patched[range.clone()].fill(0);
                    }
                }
            }
            f(&patched, changes)
        } else {
            f(original, &[])
        }
    }

    pub fn clear(&mut self) {
        self.data_affected.clear();
    }
}
