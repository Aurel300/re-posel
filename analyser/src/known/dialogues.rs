use super::KnownContext;

pub(super) fn apply_known_dialogues(c: &mut KnownContext) {
    // dialogue system
    c.open_key("105f", "Dialogue choice", |_c| {});
    c.open_key("12c2", "Dialogue starts?", |_c| {});

    // people
    c.open_key("1228", "Dialogues: boilerman", |_c| {});
    c.open_key("122c", "Dialogues: boilerman", |_c| {});
    c.open_key("122e", "Dialogues: Hermann", |_c| {});
    c.open_key("122f", "Dialogues: Bates", |_c| {});
    c.open_key("1230", "Dialogues: Morris", |_c| {});
    c.open_key("1231", "Dialogues: Victoria", |_c| {});
    c.open_key("1232", "Dialogues: Henry", |_c| {});
    c.open_key("1233", "Dialogues: Robert", |_c| {});
    c.open_key("1234", "Dialogues: fisherman", |_c| {});
    c.open_key("1235", "Dialogues: Vick", |_c| {});
    c.open_key("1236", "Dialogues: Harry", |c| {
        c.close_key("105c", "Topic icons", |_| {});
        for (key, name) in [
            ("12cc", "Topic name: what's new"), // "105f.var.1"
            ("1fdf", "Topic name: debt"), // "105f.var.2"
            ("tbm", "Topic name: Black Mirror"), // "105f.var.3"
            ("1ff8", "Topic name: about the pub"), // "105f.var.4"
            ("1ff8", "Topic name: about the pub"), // "105f.var.5"
            ("1c72", "Topic name: pawn shop"), // "105f.var.6"
            ("1ff9", "Topic name: pay off the debt"), // "105f.var.7"
            ("13eb", "Topic name: Henry"), // "105f.var.8"
            ("1fe0", "Topic name: Mark"), // "105f.var.9"
            ("13c3", "Topic name: making a key"), // "105f.var.10"
            ("1fe0", "Topic name: Mark"), // "105f.var.11"
            ("1ff8", "Topic name: about the pub"), // "105f.var.12"
        ] {
            c.close_key(key, name, |_| {});
        }
        c.close_key("114d", "Topic: pawn shop", |_| {});
    });
    c.open_key("1237", "Dialogues: Tom", |_c| {});
    c.open_key("1238", "Dialogues: gravedigger", |_c| {});
    c.open_key("1239", "Dialogues: Mark", |_c| {});
    c.open_key("123a", "Dialogues: Murray", |_c| {});
    c.open_key("123b", "Dialogues: Father Frederick", |_c| {});
    c.open_key("123c", "Dialogues: nurse", |_c| {});
    c.open_key("123d", "Dialogues: Ralph", |_c| {});
    c.open_key("123f", "Dialogues: monologues", |_c| {
        // 1095 = monologue selection
        //   16 = intro
    });
}
