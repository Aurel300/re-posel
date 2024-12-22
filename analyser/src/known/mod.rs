use std::collections::HashMap;

use crate::{adb::AdbEntry, templates::nav::NavTree};
#[allow(unused_imports)]
use crate::AdbEntryKind;

mod books;
mod dialogues;
mod inventory;
mod locations;
mod puzzles;
mod zooms;

struct KnownContext<'a> {
    prefix: Vec<&'static str>,
    tree: &'a mut NavTree,
    entries: &'a mut HashMap<String, AdbEntry>,
}

impl<'a> KnownContext<'a> {
    fn new(root: &'a mut NavTree, entries: &'a mut HashMap<String, AdbEntry>) -> Self {
        Self {
            prefix: Vec::new(),
            tree: root,
            entries,
        }
    }

    fn current(&mut self) -> &mut AdbEntry {
        let key = self.prefix.join(".");
        self.entries.get_mut(&key).unwrap()
    }

    fn with_key_multi(&mut self, keys: &[&'static str], f: impl FnOnce(&mut KnownContext)) {
        if keys.len() < 2 {
            return self.with_key(keys[0], f);
        }
        self.with_key(keys[0], |c| {
            c.with_key_multi(&keys[1..], f);
        });
    }
    fn with_key(&mut self, key: &'static str, f: impl FnOnce(&mut KnownContext)) {
        let child = self.tree.get_mut(key);
        let mut prefix = std::mem::take(&mut self.prefix);
        prefix.push(key);
        let mut kc = KnownContext {
            prefix,
            tree: child,
            entries: self.entries,
        };
        f(&mut kc);
        self.prefix = kc.prefix;
        self.prefix.pop().unwrap();
    }
    fn key(&mut self, key: &'static str, f: impl FnOnce(&mut AdbEntry)) {
        self.with_key(key, |c| {
            f(c.current());
        });
    }
    fn open_key(&mut self, key: &'static str, name: &'static str, f: impl FnOnce(&mut KnownContext)) {
        self.with_key(key, |c| {
            c.current().name = Some(name.to_string());
            f(c);
        });
    }
    fn close_key(&mut self, key: &'static str, name: &'static str, f: impl FnOnce(&mut KnownContext)) {
        self.with_key(key, |c| {
            c.current().name = Some(name.to_string());
            c.current().open_key = false;
            f(c);
        });
    }

    fn region_reference(
        &mut self,
        keys: &[&[&'static str]],
        bg_reference: &[(i32, i32, &'static str)],
        width: Option<usize>,
    ) {
        for k in keys {
            self.with_key_multi(k, |k| {
                let region = k.current().region.get_or_insert_default();
                region.bg_reference.extend(bg_reference.iter().map(|(x, y, s)| (*x, *y, s.to_string())));
                region.width = width;
            });
        }
    }
}

pub fn apply_known(root: &mut NavTree, entries: &mut HashMap<String, AdbEntry>) {
    let mut c = KnownContext::new(root, entries);

    c.open_key("main", "Entrypoint", |c| {
        c.close_key("1025", "Create fonts", |_| {});
        c.close_key("1088", "Globals reset 1", |_| {});
        c.close_key("1089", "Globals reset 2", |_| {});
        c.close_key("108a", "Globals reset 3", |_| {});
        c.close_key("11d2", "Back arrow", |_| {});
        c.close_key("1381", "Scroll left", |_| {});
        c.close_key("1382", "Scroll right", |_| {});
        c.close_key("1384", "Storm?", |_| {});
        c.close_key("21e0", "The Adventure Company logo", |_| {});

        //for k in [
        //    vec!["11d2", "r"],
        //] {
        //    c.with_key_multi(&k, |k| {
        //        k.current().region.get_or_insert_default().bg_reference.push((0, 0, "gfx1.grp/menu.bmp".to_string()));
        //    });
        //}
    });

    c.open_key("1006", "Main menu", |c| {
        c.with_key("1026", |c| {
            c.key("1030", |k| { k.region.get_or_insert_default(); });
            c.key("1032", |k| { k.region.get_or_insert_default(); });
            c.key("1034", |k| { k.region.get_or_insert_default(); });
            c.key("1036", |k| { k.region.get_or_insert_default(); });
            c.key("1039", |k| { k.region.get_or_insert_default(); });
            c.key("103c", |k| { k.region.get_or_insert_default(); });
        });
        c.with_key("100e", |c| {
            c.close_key("100b", "Button: Save game", |_| {});
            c.close_key("100f", "BG", |_| {});
            c.close_key("1020", "Button: Load game", |_| {});
            c.close_key("1022", "Button: Quit", |_| {});
            c.close_key("1037", "Button: Options", |_| {});
            c.close_key("103a", "Button: Credits", |_| {});
            c.close_key("21e1", "Version display", |_| {});
            c.close_key("min", "Motto", |_| {});
            c.close_key("new", "Button: New game", |_| {});
            c.close_key("ps", "Title", |_| {});
            c.close_key("psm", "Title (mask)", |_| {});
        });
        c.close_key("1014", "BG with text", |_| {});
        c.close_key("1015", "Music", |_| {});
        c.close_key("1017", "Intro music sting", |_| {});
        c.open_key("101b", "Submenu: Save game", |c| {
            c.close_key("1053", "Positions", |_| {});
        });
        c.open_key("103a", "Submenu: Credits", |_| {});
        c.open_key("1037", "Submenu: Options", |_| {});
        c.close_key("1055", "Loading screen", |_| {});
        c.close_key("1058", "Chapter select", |_| {});

        c.region_reference(&[
            &["1016", "rp"],
            &["100e", "1020", "rp"],
            &["100e", "1022", "rp"],
            &["100e", "1037", "rp"],
            &["100e", "103a", "rp"],
            &["100e", "new", "rp"],
        ], &[
            (  0,   0, "gfx1.grp/menu.bmp"),
            (252,  61, "gfx1.grp/poselsmrti.bmp"),
            (276, 159, "gfx1.grp/cerny_pruh.bmp"),
            (178, 420, "gfx1.grp/minulost.bmp"),
        ], None);
    });

    c.open_key("1069", "Intro", |_c| {
    });

    c.close_key("1079", "Chapter", |_| {});

    c.open_key("1094", "6zoom_oltar", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1098", "1vyhorela", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("109b", "1dovesnice", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("109c", "1uhospody", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("109d", "1hospoda", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("109f", "1spolecenska", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10a0", "1uvetese", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10a1", "1hlavnijidelna", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10a3", "1brana", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10a4", "1schody", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10a5", "1pokoj_Sama", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10a6", "1uskleniku", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10a8", "1sklenik", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10a9", "1hlavnibalkon", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10aa", "1naschodech", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10ab", "1stromsmrti", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10ac", "1fontana", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10ad", "1hradvchod", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10ae", "1uveze", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10b2", "1studna", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10b3", "1staj", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10b4", "1hlavniven", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10b5", "1hlavnikrb", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10b6", "1knihovnavchod", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10b7", "1knihovna2", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10b8", "1kostel_scroll", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10bb", "1pohrebiste", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10bd", "1knihovnastul", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10bf", "1hlavni1patro", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10c0", "1chodba1", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10c1", "1chodba2", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10c2", "1jidelna", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10c3", "1kuchyn", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10c8", "1puda", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10c9", "1vez", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10eb", "2sklepeni", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10ec", "2mostarna", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10ef", "2doktor", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10f2", "2udoktora", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("10f4", "2marnice", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1102", "2vkostelvchod", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1103", "2vkostelcelo", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1104", "2vkostelzpoved", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1105", "2zvonice", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1106", "2marcushrobka_tma", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1107", "2skladiste", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1108", "2dolyscroll", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1109", "2vytah", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("110a", "2vodarna", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("110b", "2vytah_mezipatro", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("110c", "2strojovna_tma", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("110d", "2vytah_vychod", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("110e", "2vchodoly", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("110f", "3ubrany", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1110", "3rozcesti", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1112", "3walesdum", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1113", "3hrabenka", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1114", "3ruiny", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1115", "3stodola", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1119", "3vestodole", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("111a", "3alchymie", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("111b", "3kaple", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1142", "1pokoj_Sama_suple_dolni", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1175", "stin+ticho", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1189", "pakypozadi", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("11dd", "6peklo", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1219", "5sanatorvkapli", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1220", "1hradvchod", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1224", "4kotelna", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1225", "4kotelna_zadni", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1226", "4cela", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1227", "4majak", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1228", "4predkotelnou", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1229", "4sanatorhrbitov", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("12df", "4sanatorscrolling", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("12ff", "4druidi01", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1340", "4sanator_brana", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("137c", "4sanatorkaple", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("138c", "4stoky_scroll", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("138d", "4bazen", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("13b2", "4stoky_pruchod", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("13ba", "4hala", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1435", "4kodzamek", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1542", "4bazen_dole", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1568", "6labyrint1", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1572", "6labyrint3", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1573", "6labyrint6", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1574", "6labyrint8", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1575", "6labyrint4", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1576", "6labyrint2", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1577", "6labyrint12", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1578", "6labyrint11", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1580", "6labyrint7", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1583", "6labyrint5", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1584", "6labyrint9", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1586", "6labyrint10", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1588", "4majak_hrob", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("15b5", "6inv_kuze", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1613", "3vkapli", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("171e", "3umrize", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1796", "konepozadi", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("17fb", "3vhrobce", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1888", "vodotlakbez", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1a8b", "2kontejner", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1b6d", "denikRW_strana01", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1c09", "2vetes", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("1e68", "1pokoj_Sama_suple_horni", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("2092", "5pracovna", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("209e", "5trezor_open", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("20a2", "denik_roberta12", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("20a3", "trezor", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("20a6", "5pracovna_suplata", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("20f4", "3walesnoc_okno", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("21c9", "inv_papircibule", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    books::apply_known_books(&mut c);
    dialogues::apply_known_dialogues(&mut c);
    inventory::apply_known_inventory(&mut c);
    locations::apply_known_locations(&mut c);
    puzzles::apply_known_puzzles(&mut c);
    zooms::apply_known_zooms(&mut c);

    /*
    let mut loc_names = c.tree.children.iter()
        .filter(|(_, e)| e.children.contains_key("100f"))
        .filter_map(|(k, _)| c.entries.get(&format!("{k}.100f.p")).map(|e| (k, &e.kind)))
        .filter_map(|(k, kind)| match &kind {
            AdbEntryKind::String { decoded, .. } => Some((k, decoded)),
            _ => None,
        })
        .collect::<Vec<_>>();
    loc_names.sort();
    for (k, name) in loc_names {
        println!("loc {k}: {name}");
    }
    */
}
