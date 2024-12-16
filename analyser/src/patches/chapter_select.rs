use super::{Patch, PatchChange};

pub(super) const PATCH: Patch<'static> = Patch {
    name: "chapter_select",
    description: "enables chapter selection when starting new game",
    changes: &[
        PatchChange::DataModify {
            key: "1006.1058",
            range: 0x24..0x25,
            content: b"\x17",
        },
    ],
};
