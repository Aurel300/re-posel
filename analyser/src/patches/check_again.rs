use super::{Patch, PatchChange};

pub(super) const PATCH: Patch<'static> = Patch {
    name: "check_again",
    description: "don't waste players' time on repeated dialogue checks",
    changes: &[
        PatchChange::DataModify {
            key: "1236",
            range: 0x3F6..0x3F7,
            content: b"\x1A",
        },
    ],
};
