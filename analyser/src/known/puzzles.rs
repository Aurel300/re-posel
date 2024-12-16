use super::KnownContext;

pub(super) fn apply_known_puzzles(c: &mut KnownContext) {
    c.open_key("108d", "Puzzle: James' photo", |c| {
        c.close_key("1070", "Music", |_| {});
        c.close_key("10af", "Exit (back)", |_| {});
        c.close_key("118d", "Solution check", |_| {});
        c.close_key("192b", "BG", |_| {});
        c.open_key("1291", "Pieces", |c| {
            c.close_key("p", "Pictures", |_| {});
            c.close_key("r", "Places", |_| {});
            c.close_key("rp", "Positions", |_| {});
        });

        c.region_reference(&[
            &["10af", "r"],
            &["1291", "r", "1"], &["1291", "rp", "1"],
            &["1291", "r", "2"], &["1291", "rp", "2"],
            &["1291", "r", "3"], &["1291", "rp", "3"],
            &["1291", "r", "4"], &["1291", "rp", "4"],
            &["1291", "r", "5"], &["1291", "rp", "5"],
            &["1291", "r", "6"], &["1291", "rp", "6"],
            &["1291", "r", "7"], &["1291", "rp", "7"],
            &["1291", "r", "8"], &["1291", "rp", "8"],
            &["1291", "r", "9"], &["1291", "rp", "9"],
            &["1291", "r", "10"], &["1291", "rp", "10"],
            &["1291", "r", "11"], &["1291", "rp", "11"],
            &["1291", "r", "12"], &["1291", "rp", "12"],
            &["1291", "r", "13"], &["1291", "rp", "13"],
            &["1291", "r", "14"], &["1291", "rp", "14"],
            &["1291", "r", "15"], &["1291", "rp", "15"],
            &["1291", "r", "16"], &["1291", "rp", "16"],
            &["1291", "r", "17"], &["1291", "rp", "17"],
            &["1291", "r", "18"], &["1291", "rp", "18"],
            &["1291", "r", "19"], &["1291", "rp", "19"],
            &["1291", "r", "20"], &["1291", "rp", "20"],
            &["1291", "r", "21"], &["1291", "rp", "21"],
            &["1291", "r", "22"], &["1291", "rp", "22"],
            &["1291", "r", "23"], &["1291", "rp", "23"],
            &["1291", "r", "24"], &["1291", "rp", "24"],
            &["1291", "r", "25"], &["1291", "rp", "25"],
            &["1291", "r", "26"], &["1291", "rp", "26"],
            &["1291", "r", "27"], &["1291", "rp", "27"],
            &["1291", "r", "28"], &["1291", "rp", "28"],
            &["1291", "r", "29"], &["1291", "rp", "29"],
            &["1291", "r", "30"], &["1291", "rp", "30"],
            &["1291", "r", "31"], &["1291", "rp", "31"],
            &["1291", "r", "32"], &["1291", "rp", "32"],
            &["1291", "r", "33"], &["1291", "rp", "33"],
            &["1291", "r", "34"], &["1291", "rp", "34"],
            &["1291", "r", "35"], &["1291", "rp", "35"],
        ], &[
            (0, 45, "gfx1.grp/puzzledement.bmp"),
        ], None);
    });

    c.open_key("1547", "puzzle_stoky", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1b48", "puzzledraty_pozadi", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1f8a", "puzzleglobus", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("20b7", "puzzle-hodiny-background", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1716", "zverokruh-background2", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("192a", "Puzzle: swap stones", |_c| {
        //c.close_key("100f", "BG", |_| {});
    });


    c.open_key("1175", "Puzzle: word pillars", |_c| {});
    c.open_key("1189", "Puzzle: mines generator", |_c| {});
    c.open_key("1796", "Puzzle: knight moves", |c| {
        c.close_key("100f", "BG", |_| {});
        c.close_key("10af", "Exit (back)", |_| {});
        c.close_key("kun", "Knight piece", |_| {});

        c.region_reference(&[
            &["10af", "r"],
            &["17af", "r"],
            &["kun", "17b5"],
            &["kun", "17b8"],
            &["kun", "17bb"],
            &["kun", "17be"],
            &["kun", "17c1"],
            &["kun", "17c4"],
            &["kun", "17c7"],
            &["kun", "17ca"],
            &["kun", "r1a"],
            &["kun", "r1b"],
            &["kun", "r1c"],
            &["kun", "r2a"],
            &["kun", "r2b"],
            &["kun", "r2c"],
            &["kun", "r3a"],
            &["kun", "r3b"],
            &["kun", "r3c"],
        ], &[
            (0, 45, "gfx1.grp/konepozadi.bmp"),
        ], None);
    });
}