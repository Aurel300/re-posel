use super::KnownContext;

pub(super) fn apply_known_zooms(c: &mut KnownContext) {
    // chapter 1
    c.open_key("19d5", "Zoom: Marcus Gordon picture", |c| {
        c.close_key("100f", "BG", |_| {});
        c.close_key("1070", "Music", |_| {});
        c.close_key("10af", "Exit: back", |_| {});
    });
    c.open_key("10bc", "1pohrebiste_zoom", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1403", "1zoom_kronika", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1409", "1zoom_symbolvez", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1e22", "1zoom_obraz", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("201f", "1zoom_obrazMordred", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("208e", "1zoom_dopisHermann", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    // chapter 2
    c.open_key("1168", "2zoom_suplesklenikp", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("19d4", "2zoom_marcus", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("19fa", "2marnice_zoomsuple", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1e1d", "2zoom_dopisvyderac", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1e1f", "2zoom_suplesklenikl", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    // chapter 3
    c.open_key("1661", "3zoom_stulstodola", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1664", "3zoom_kanal", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("16c0", "3zoom_krb", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1825", "3zoom_kostra", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    // chapter 4
    c.open_key("12b1", "4zoom_termostat", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("142c", "4zoom_nastenka", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("14af", "4zoom_lekarna", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("214b", "4zoom_symbol_druidi", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    // chapter 5
    c.open_key("1a3a", "Zoom: Morgue body Hermann", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("122a", "Zoom: Lothar Gordon picture", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1401", "5zoom_zavet", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1405", "5zoom_dopisWilliam", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1408", "5zoom_symbol_majak", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1a35", "5marnice_zoom_symbol", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1a66", "5zoom_doktorhlava", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1cea", "5zoom_dopisMorris", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("1d56", "5zoom_hrobka_prazdna", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    c.open_key("20b5", "5zoom_kodtrezor", |c| {
        c.close_key("100f", "BG", |_| {});
    });

    // others?
    c.open_key("1a5e", "Zoom: Morgue body Harry", |_c| {});
    c.open_key("1221", "Zoom: William's grave", |_c| {});

    c.open_key("1513", "Zoom: James' diary", |c| {
        c.close_key("100f", "BG", |_| {});
    });
    c.open_key("216e", "Zoom: James' letter", |c| {
        c.close_key("100f", "BG", |_| {});
    });
}
