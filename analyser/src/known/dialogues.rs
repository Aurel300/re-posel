use super::KnownContext;

pub(super) fn apply_known_dialogues(c: &mut KnownContext) {
    // dialogue system
    c.open_key("105f", "Dialogue choice", |_c| {});

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
    c.open_key("1236", "Dialogues: Harry", |_c| {});
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
