use crate::adb::AdbEntryGlobal;

use super::KnownContext;

pub(super) fn apply_known_locations(c: &mut KnownContext) {
    // map
    c.open_key("10ca", "Map", |c| {
        c.close_key("101c", "Exit: back", |_| {});
        c.close_key("1070", "Music", |_| {});
        c.close_key("10f7", "Button: Hermann's house", |_| {});
        c.close_key("10f8", "Button: The Warmhill vicarage", |_| {});
        c.close_key("10fa", "Button: Willow Creek", |_| {});
        c.close_key("11c0", "Button: Black Mirror", |_| {});
        c.close_key("1247", "BG: The Ashburry sanatorium", |_| {});
        c.close_key("12aa", "Button: Sharp edge", |_| {});
        c.close_key("1536", "Button: Black Mirror grounds", |_| {});
        c.close_key("18b9", "BG", |_| {});
        c.close_key("18bb", "Regions", |_| {});
        c.close_key("18be", "Button: The old mine", |_| {});
        c.close_key("18c2", "Button: The Ashburry sanatorium", |_| {});
        c.close_key("18c5", "Button: Stonering", |_| {});
        c.close_key("18c7", "BG: Stonering", |_| {});
        c.close_key("18ca", "BG: Sharp edge", |_| {});
        c.close_key("18cd", "BG: The old mine", |_| {});
        c.close_key("18d0", "BG: Hermann's house", |_| {});
    });
    c.key("K2_Mapa", |k| {
        k.name = Some("Map".to_string());
        k.scene.get_or_insert_default().bg_reference.extend([
            (  0,   0, "gfx1.grp/mapa-cista.bmp".to_string()),
            (313, 460, "gfx1.grp/sanatorium.bmp".to_string()),
            ( 65, 372, "gfx1.grp/druidi.bmp".to_string()),
            (610, 386, "gfx1.grp/majak.bmp".to_string()),
            ( 65, 244, "gfx1.grp/doly.bmp".to_string()),
            (482, 236, "gfx1.grp/Hermann.bmp".to_string()),
        ]);
    });

    // locations
    c.open_key("109b", "Willow Creek: left", |c| {
        c.close_key("100f", "BG", |_| {});
        c.close_key("1056", "Door", |_| {});
        c.close_key("1099", "Exit (Pub)", |_| {});
        c.open_key("10af", "Exit", |_| {});
        c.close_key("1151", "Box", |_| {});
        c.close_key("1152", "Stairs", |_| {});
        c.close_key("1153", "Willow Creek sign", |_| {});
        c.with_key("1154", |c| {
            c.key("r", |k| k.region.get_or_insert_default().width = Some(1000));
        });
        c.close_key("1155", "Dry willow", |_| {});
        c.close_key("1341", "Fog", |_| {});
        c.close_key("13bf", "Inn smoke", |_| {});
        c.close_key("15a3", "FG pillar", |_| {});
        c.close_key("1bff", "FG chimney", |_| {});
        c.close_key("1c20", "Water", |_| {});
        c.close_key("1c23", "FG chains 1", |_| {});
        c.close_key("1c24", "FG chains 2", |_| {});
        c.close_key("1c26", "Dogs sound FX", |_| {});
        c.close_key("1c2a", "River sound FX", |_| {});
        c.close_key("1c5f", "FG chains 3", |_| {});
        c.close_key("1c60", "FG reeds", |_| {});
        c.close_key("1c61", "FG tree", |_| {});
        c.close_key("1c62", "Vick ball animation", |_| {});
        c.close_key("1c63", "Walk sounds", |_| {});
        c.close_key("1c64", "Vick sitting", |_| {});
        c.close_key("1c65", "Fisherman", |_| {});
        c.close_key("r", "Walkmap", |_| {});
        c.key("r4", |k| {
            k.name = Some("Walkmap (C4)".to_string());
            k.region.get_or_insert_default();
        });
    });
    c.key("K1_Vesnice", |k| {
        k.name = Some("Willow Creek: left".to_string());
        let s = k.scene.get_or_insert_default();
        s.width = Some(1000);
        s.bg_reference.extend([
            (0, 45, "gfx1.grp/1dovesnice.bmp".to_string()),
        ]);
    });

    c.open_key("109c", "Willow Creek: pub exterior", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.key("K1_Predhospodou", |k| {
        k.name = Some("Willow Creek: pub exterior".to_string());
        k.scene.get_or_insert_default().bg_reference.extend([
            (0, 45, "gfx1.grp/1uhospody.bmp".to_string()),
        ]);
    });

    c.open_key("109d", "Willow Creek: pub interior", |c| {
        c.close_key("100f", "BG", |_| {});
        c.close_key("10af", "Exit", |_| {});
        c.close_key("115a", "Poster", |_| {});
        c.close_key("148c", "FG counter", |_| {});
        c.close_key("1530", "MG table right", |_| {});
        c.close_key("1534", "MG chair", |_| {});
        c.close_key("1535", "FG chair", |_| {});
        c.close_key("15b2", "BG candle", |_| {});
        c.close_key("164e", "BG glass shards", |_| {});
        c.close_key("178c", "FG table left 1", |_| {});
        c.close_key("1be0", "Rain sound FX", |_| {});
        c.close_key("1fd1", "FG table right", |_| {});
        c.close_key("1fdb", "Tom's chair", |_| {});
        c.close_key("1fe6", "FG table left 2", |_| {});
        c.close_key("1fe7", "Tom", |_| {});
        c.key("r", |k| k.name = Some("Walkmap".to_string()));
    });

    c.open_key("109f", "Castle: sitting room", |c| {
        c.close_key("1007", "C1 intro cutscene", |_| {});
        c.close_key("100f", "BG", |_| {});
        c.close_key("10b1", "Exit (lobby)", |_| {});
        c.close_key("11ac", "Magazine", |_| {});
        c.close_key("124b", "Dergham Gordon picture", |_| {});
        c.close_key("134c", "FG left 1", |_| {});
        c.close_key("134d", "FG left 2", |_| {});
        c.close_key("134f", "Fog", |_| {});
        c.close_key("1415", "MG middle left", |_| {});
        c.close_key("1416", "MG middle right", |_| {});
        c.close_key("1418", "FG middle", |_| {});
        c.close_key("1419", "FG right", |_| {});
        c.close_key("145b", "Small cabinet", |_| {});
        c.close_key("151d", "Horse (sign)", |_| {});
        c.close_key("18b1", "Chair 2", |_| {});
        c.close_key("1d0a", "C2 BG", |_| {});
        c.close_key("1d66", "Chair 1", |_| {});
        c.close_key("1d67", "Horse (statue)", |_| {});
        c.key("r", |k| k.name = Some("Walkmap".to_string()));

        c.region_reference(&[
            &["1095", "1007", "r"],
            &["1095", "143d"],
            &["10b1", "r"],
            &["11ac", "k", "r"],
            &["11ac", "r"],
            &["11bd", "r"],
            &["124b", "k", "r"],
            &["124b", "r"],
            &["145b", "k", "r"],
            &["145b", "r"],
            &["151d", "k", "r"],
            &["151d", "r"],
            &["1d67", "r"],
            &["r"],
        ], &[
            (0, 45, "gfx1.grp/1spolecenska.bmp"),
        ], None);
    });

    c.open_key("10a0", "Willow Creek: right", |c| {
        c.close_key("100f", "BG", |_| {});
        c.close_key("1099", "Exit", |_| {});
        c.close_key("10af", "Exit (pub)", |_| {});
        c.close_key("1111", "Exit (pawn shop)", |_| {});
        c.close_key("114d", "Pawn shop door locked", |_| {});
        c.close_key("115b", "Big clock", |_| {});
        c.close_key("115c", "House", |_| {});
        c.close_key("115d", "Trash", |_| {});
        c.close_key("115e", "Shop window", |_| {});
        c.close_key("11cd", "Punt", |_| {});
        c.close_key("1341", "Fog", |_| {});
        c.close_key("13bf", "FG chimney active", |_| {});
        c.close_key("15a3", "FG streetlamp", |_| {});
        c.close_key("1bfe", "MG house 1", |_| {});
        c.close_key("1bff", "FG chimney", |_| {});
        c.close_key("1c20", "Water", |_| {});
        c.close_key("1c21", "FG bridge", |_| {});
        c.close_key("1c22", "FG chains 1", |_| {});
        c.close_key("1c23", "FG chains 2", |_| {});
        c.close_key("1c24", "FG chains 3", |_| {});
        c.close_key("1c25", "MG house 2", |_| {});
        c.close_key("1c27", "Building", |_| {});
        c.close_key("1c28", "Pawn shop sign", |_| {});
        c.close_key("1c29", "Streetlamp FX", |_| {});
        c.close_key("1c2a", "River sound FX", |_| {});
        c.close_key("1c2b", "Punt sound FX", |_| {});
        //c.close_key("1c48", "Punt", |_| {});
        c.close_key("dum", "Private house", |_| {});
        c.key("r", |k| k.name = Some("Walkmap".to_string()));

        c.region_reference(&[
            &["1099", "r"],
            &["10af", "r"],
            &["10af", "ven", "r"],
            &["1111", "r"],
            &["114d", "r"],
            &["115b", "r"],
            &["115c", "r"],
            &["115d", "r"],
            &["115e", "r"],
            &["1c27", "r"],
            &["1c28", "r"],
            &["1c28", "1093"],
            &["1c30"],
            &["1c48", "r"],
            &["dum", "r"],
            &["r"],
        ], &[
            (0, 45, "gfx1.grp/1uvetese.bmp"),
        ], None);
    });

    c.open_key("10a1", "Castle: main hall (dining room)", |c| {
        c.close_key("1002", "Closing door animation", |_| {});
        c.close_key("100f", "BG", |_| {});
        c.close_key("1095", "On entry", |_| {
            // glb 5 = after C1 intro
        });
        c.close_key("1099", "Exit (dining room)", |_| {});
        c.close_key("10b1", "Exit (towards staircase)", |_| {});
        c.close_key("1111", "Exit (towards library top)", |_| {});
        c.close_key("124b", "Marcus Gordon picture", |c| {
            c.close_key("132b", "Counter", |_| {});
        });
        c.close_key("2027", "Exit (towards library bottom)", |_| {});
        c.close_key("2028", "Monolith", |_| {});
        c.key("r", |k| k.name = Some("Walkmap".to_string()));

        c.region_reference(&[
            &["1002", "rp"],
            &["1099", "r"],
            &["1099", "z", "r"],
            &["10b1", "r"],
            &["10b1", "z", "r"],
            &["10c6", "r"],
            &["10c6", "z", "r"],
            &["1111", "r"],
            &["1111", "z", "r"],
            &["124b", "r"],
            &["143a", "k", "r"],
            &["143a", "r"],
            &["1653", "r"],
            &["1d8f", "r"],
            &["2026", "r"],
            &["2027", "r"],
            &["2027", "z", "r"],
            &["2028", "r"],
            &["krb", "r"],
            &["r"],
        ], &[
            (0, 45, "gfx1.grp/1hlavnijidelna.bmp"),
        ], None);
    });
    c.region_reference(&[
        &["2029", "r"],
        &["202a", "r"],
        &["202b", "r"],
        &["202c", "r"],
        &["202d", "r"],
        &["202e", "r"],
        &["202f", "r"],
    ], &[
        (0, 45, "gfx1.grp/1hlavnijidelna.bmp"),
    ], None);

    c.open_key("10a5", "Castle: Samuel's room", |c| {
        c.key("1095", |k| {
            k.name = Some("Case".to_string());
            k.global = Some(AdbEntryGlobal {
                values: [
                    (3, "fainted, C1 end".to_string()),
                ].into(),
            });
        });
    });
    c.key("K1_Pokoj", |k| {
        k.name = Some("Castle: Samuel's room".to_string());
        k.scene.get_or_insert_default().bg_reference.extend([
            (0, 45, "gfx1.grp/1pokoj_Sama.bmp".to_string()),
        ]);
    });

    c.open_key("10b4", "Castle: main hall (entry)", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("10b8", "Church: exterior", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("10c0", "Castle: corridor", |c| {
        c.close_key("100f", "BG", |_| {});
        c.close_key("1099", "Exit (staircase)", |_| {});
        c.close_key("10af", "Exit (room) left", |_| {});
        c.open_key("10c6", "Robert's study", |c| {
            c.key("1093", |k| { k.region.get_or_insert_default(); });
        });
        c.close_key("1111", "Exit (room) right", |_| {});
        c.close_key("1146", "Newspaper with note", |_| {});
        c.close_key("115b", "Clock sound FX", |_| {});
        c.close_key("124b", "Jennifer Gordon picture", |_| {});
        c.close_key("143a", "MG middle table", |_| {});
        c.close_key("148a", "MG back pillar", |_| {});
        c.close_key("159b", "Newspaper rack", |_| {});
        c.close_key("1909", "FG left", |_| {});
        c.close_key("1d11", "Victoria's room", |_| {});
        c.close_key("206d", "Meals", |_| {});
        c.key("r", |k| k.name = Some("Walkmap".to_string()));
    });

    c.open_key("10c8", "Castle: attic", |c| {
        c.close_key("100f", "BG", |_| {});
        c.close_key("1099", "Door", |_| {});
        c.close_key("10af", "Exit (old wing)", |_| {});
        c.close_key("115f", "Robert's chest", |_| {});
        c.close_key("1161", "Shelves", |_| {});
        c.close_key("1162", "Chest", |_| {});
        c.close_key("1653", "Walk sounds (FG carpet)", |_| {});
        c.close_key("1cad", "MG right entry", |_| {});
        c.close_key("1d66", "Rocking chair", |_| {});
        c.close_key("1d84", "MG middle left", |_| {});
        c.close_key("1d85", "MG middle right", |_| {});
        c.close_key("1d86", "MG right pillar", |_| {});
        c.close_key("1d87", "FG left 1", |_| {});
        c.close_key("1d88", "FG left 2", |_| {});
        c.close_key("1d89", "Door boards", |_| {});
        c.close_key("1d8a", "FG middle 1", |_| {});
        c.close_key("1d8b", "FG middle 2", |_| {});
        c.close_key("1d8c", "FG right 1", |_| {});
        c.close_key("1d8d", "FG right 2", |_| {});
        c.close_key("1d8e", "Pigeons sound FX", |_| {});
        c.close_key("1d8f", "Walk sounds (BG carpet)", |_| {});
        c.close_key("1d90", "Box", |_| {});
        c.close_key("1d91", "Corner", |_| {});
        c.key("r", |k| k.name = Some("Walkmap".to_string()));

        c.key("1095", |k| {
            k.name = Some("Case".to_string());
            k.global = Some(AdbEntryGlobal {
                values: [
                    (2, "done exploring William's study".to_string()),
                ].into(),
            });
        });

        c.region_reference(&[
            &["1002", "rp"],
            &["1099", "1dba"],
            &["1099", "1dbc"],
            &["1099", "r"],
            &["1099", "rp"],
            &["10af", "r"],
            &["115f", "r"],
            &["115f", "rp"],
            &["1161", "r"],
            &["1162", "r"],
            &["1653", "r"],
            &["1d66", "r"],
            &["1d85", "rp"],
            &["1d8f", "r"],
            &["1d90", "r"],
            &["1d91", "r"],
            &["1d92", "r"],
            &["1da2", "rp"],
            &["1dbe", "rp"],
            &["1dbf", "rp"],
            &["r"],
        ], &[
            (0, 45, "gfx1.grp/1puda.bmp"),
        ], Some(1400));
    });

    c.open_key("10c9", "Castle: William's tower", |c| {
        c.close_key("100f", "BG", |_| {});

        c.open_key("10af", "Door", |c| {
            c.key("11d7", |k| {
                k.name = Some("Exit lock".to_string());
                k.global = Some(AdbEntryGlobal {
                    values: [
                        (0, "locked".to_string()),
                        (1, "unlocked".to_string()),
                    ].into(),
                });
            });
        });
    });
    c.key("K1_Vez", |k| {
        k.name = Some("Castle: William's tower".to_string());
        k.scene.get_or_insert_default().bg_reference.extend([
            (0, 45, "gfx1.grp/1vez.bmp".to_string()),
        ]);
    });

    c.open_key("10eb", "Castle: cellar", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("10f4", "Morgue: basement", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1107", "Mines: storeroom", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    //c.open_key("1107", "Mines: generator room", |c| {
    //    c.close_key("100f", "BG", |_| {});
    //});

    //c.open_key("1107", "Mines: lift middle", |c| {
    //    c.close_key("100f", "BG", |_| {});
    //});

    c.open_key("1108", "Mines: crossroad", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("110c", "Mines: machine room", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("110c", "Mines: lift top", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1112", "Wales: house exterior", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1113", "Wales: house interior", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1114", "Wales: ruins exterior", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1115", "Wales: old house exterior", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1119", "Wales: old house interior", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("111a", "Wales: laboratory", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("111b", "Wales: chapel exterior", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1166", "Castle: Fountain symbol", |_c| {});
    c.open_key("1168", "Castle: Greenhouse drawers", |_c| {});
    c.open_key("1219", "Sanatorium: chapel", |_c| {});
    c.open_key("10ad", "Castle: entrance", |_c| {}); // ? for a cutscene?
    c.open_key("1220", "Castle: entrance", |_c| {}); // ? for a cutscene?

    c.open_key("1222", "Sanatorium: corridor guard", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1223", "Sanatorium: corridor", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1224", "Sanatorium: boiler room", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1225", "Sanatorium: boiler room back", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1226", "Sanatorium: cell", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("12df", "Sanatorium: exterior", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1227", "Lighthouse", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1229", "Sanatorium: graveyard", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("138c", "Sewers: crossroad", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("138d", "Sewers: tank", |c| {
        c.close_key("100f", "BG", |_| {});
    });
}
