use super::{Patch, PatchChange};

pub(super) const PATCH: Patch<'static> = Patch {
    name: "skip_intros",
    description: "skip intro movies and main menu animation on launch",
    changes: &[
        PatchChange::DataModify {
            key: "main",
            range: 0x31..0x36,
            content: b"\x12\x0D\xAF\x4C\x0C",
        },
        PatchChange::DataModify {
            key: "1006.100e",
            range: 0x33..0x34,
            content: b"\x01",
        },
        PatchChange::DataZero {
            key: "main",
            range: 0x36..0x55,
        },
        PatchChange::DataModify {
            key: "1006.100e.1022",
            range: 0x4E..0x50,
            content: b"\x9E\x0C",
        },
        PatchChange::DataZero {
            key: "1006.100e.1022",
            range: 0x50..0x75,
        },
    ],
};
