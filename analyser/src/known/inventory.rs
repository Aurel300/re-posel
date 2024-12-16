use super::KnownContext;

pub(super) fn apply_known_inventory(c: &mut KnownContext) {
    c.open_key("inv", "Inventory", |c| {
        c.close_key("11f6", "Item popup", |_| {});
        c.close_key("1ddb", "Item popup: jewel box", |_| {});
        c.close_key("1208", "Create cursors", |_| {});

        c.close_key("1005", "Item: film", |_| {});
        c.close_key("1092", "Item: sacred dagger", |_| {});
        c.close_key("112b", "Item: hammer", |_| {});
        c.close_key("112d", "Item: box", |_| {});
        c.close_key("1133", "Item: planets", |_| {});
        c.close_key("1134", "Item: map", |_| {});
        c.close_key("113b", "Item: the key to the drawer", |_| {});
        c.close_key("1144", "Item: camera", |_| {});
        c.close_key("116d", "Item: a coin", |_| {});
        c.close_key("1180", "Item: firm wire", |_| {});
        c.close_key("1181", "Item: wire cutters", |_| {});
        c.close_key("1191", "Item: fuse", |_| {});
        c.close_key("119f", "Item: William's diary", |_| {});
        c.close_key("11a3", "Item: revolver", |_| {});
        c.close_key("11a4", "Item: revolver loaded with one bullet", |_| {});
        c.close_key("11a5", "Item: nail", |_| {});
        c.close_key("11ac", "Item: newspaper", |_| {});
        c.close_key("11ca", "Item: Henry's bill of exchange", |_| {});
        c.close_key("11d3", "Item: blood", |_| {});
        c.close_key("11f8", "Item: the third sacred key", |_| {});
        c.close_key("1201", "Item: the fifth sacred key", |_| {});
        c.close_key("1202", "Item: the fourth sacred key", |_| {});
        c.close_key("1203", "Item: the second sacred key", |_| {});
        c.close_key("1204", "Item: the first sacred key", |_| {});
        c.close_key("1214", "Item: black sphere", |_| {});
        c.close_key("1215", "Item: alcohol cigarette lighter", |_| {});
        c.close_key("1216", "Item: ring", |_| {});
        c.close_key("121a", "Item: gardening scissors", |_| {});
        c.close_key("121c", "Item: wallet", |_| {});
        c.close_key("121d", "Item: small knife", |_| {});
        c.close_key("121e", "Item: map of the manor", |_| {});
        c.close_key("121f", "Item: jewel box", |_| {});
        c.close_key("1243", "Item: a strange object", |_| {});
        c.close_key("124b", "Item: picture", |_| {});
        c.close_key("1254", "Item: sweets", |_| {});
        c.close_key("1272", "Item: toner", |_| {});
        c.close_key("128a", "Item: urn", |_| {});
        c.close_key("12b8", "Item: screw", |_| {});
        c.close_key("1333", "Item: wet handkerchief", |_| {});
        c.close_key("133b", "Item: hammer", |_| {});
        c.close_key("1345", "Item: syringe with amobarbital", |_| {});
        c.close_key("136f", "Item: syringe", |_| {});
        c.close_key("13a6", "Item: cogwheel", |_| {});
        c.close_key("13a7", "Item: cogwheel", |_| {});
        c.close_key("1430", "Item: pin", |_| {});
        c.close_key("1433", "Item: small key", |_| {});
        c.close_key("1434", "Item: duty schedule", |_| {});
        c.close_key("1442", "Item: rag", |_| {});
        c.close_key("1443", "Item: rag", |_| {});
        c.close_key("1447", "Item: damp rag", |_| {});
        c.close_key("144c", "Item: a pair of rubber boots", |_| {});
        c.close_key("1459", "Item: the key to the chapel", |_| {});
        c.close_key("145f", "Item: body of a doll", |_| {});
        c.close_key("149f", "Item: shard", |_| {});
        c.close_key("14a1", "Item: shard", |_| {});
        c.close_key("14a4", "Item: broken gardening scissors", |_| {});
        c.close_key("14a9", "Item: James's hair", |_| {});
        c.close_key("14b1", "Item: vase", |_| {});
        c.close_key("14b2", "Item: small empty bottle", |_| {});
        c.close_key("14b4", "Item: small key", |_| {});
        c.close_key("14b6", "Item: sedatives", |_| {});
        c.close_key("14c1", "Item: handkerchief", |_| {});
        c.close_key("14ce", "Item: head of a doll", |_| {});
        c.close_key("14d8", "Item: keys to the cells", |_| {});
        c.close_key("14e6", "Item: head and body of a doll", |_| {});
        c.close_key("14eb", "Item: doll", |_| {});
        c.close_key("14fe", "Item: keys", |_| {});
        c.close_key("150c", "Item: James's diary", |_| {});
        c.close_key("1546", "Item: small chest", |_| {});
        c.close_key("157b", "Item: helmet", |_| {});
        c.close_key("158c", "Item: talisman", |_| {});
        c.close_key("15b2", "Item: candle", |_| {});
        c.close_key("15c6", "Item: acid", |_| {});
        c.close_key("15cd", "Item: rod", |_| {});
        c.close_key("15cf", "Item: rod", |_| {});
        c.close_key("15d0", "Item: rope", |_| {});
        c.close_key("15d2", "Item: hook on a rope", |_| {});
        c.close_key("15d5", "Item: sharpened rod", |_| {});
        c.close_key("15d9", "Item: pills", |_| {});
        c.close_key("15da", "Item: old photograph", |_| {});
        c.close_key("161c", "Item: small bottle with an oxidant in it", |_| {});
        c.close_key("163f", "Item: the key to the mansion", |_| {});
        c.close_key("164f", "Item: kettle", |_| {});
        c.close_key("1655", "Item: logs", |_| {});
        c.close_key("1657", "Item: poker", |_| {});
        c.close_key("1669", "Item: label", |_| {});
        c.close_key("167d", "Item: key", |_| {});
        c.close_key("169d", "Item: alcohol cigarette lighter without a wick", |_| {});
        c.close_key("16ae", "Item: kettle with water in it", |_| {});
        c.close_key("16e6", "Item: William's mourning-card", |_| {});
        c.close_key("16f4", "Item: a key", |_| {});
        c.close_key("170b", "Item: a key", |_| {});
        c.close_key("1712", "Item: soil", |_| {});
        c.close_key("1721", "Item: fountain pen", |_| {});
        c.close_key("1727", "Item: little rocks", |_| {});
        c.close_key("173b", "Item: small bottle with water in it", |_| {});
        c.close_key("1795", "Item: amulet", |_| {});
        c.close_key("17de", "Item: wire", |_| {});
        c.close_key("17ef", "Item: the key to the tomb", |_| {});
        c.close_key("17f7", "Item: a key with a skull", |_| {});
        c.close_key("185a", "Item: glue", |_| {});
        c.close_key("18a2", "Item: rope", |_| {});
        c.close_key("18ab", "Item: hook", |_| {});
        c.close_key("19d8", "Item: Marcus Gordon's chronicle", |_| {});
        c.close_key("1a0f", "Item: the letter for William", |_| {});
        c.close_key("1a13", "Item: paper", |_| {});
        c.close_key("1a1c", "Item: piece of imprint plastic", |_| {});
        c.close_key("1a1e", "Item: small plastic bags", |_| {});
        c.close_key("1a23", "Item: medical forceps", |_| {});
        c.close_key("1a42", "Item: the hair of the murderer", |_| {});
        c.close_key("1a4e", "Item: Morris's hair", |_| {});
        c.close_key("1a4f", "Item: William's last will", |_| {});
        c.close_key("1a50", "Item: Robert's diary", |_| {});
        c.close_key("1a51", "Item: the undelivered letter from William", |_| {});
        c.close_key("1a5a", "Item: imprint of a key", |_| {});
        c.close_key("1a90", "Item: Henry's clothes", |_| {});
        c.close_key("1a9d", "Item: torn-up letter", |_| {});
        c.close_key("1a9e", "Item: small key to the garbage container", |_| {});
        c.close_key("1b00", "Item: revolver loaded with two bullets", |_| {});
        c.close_key("1b02", "Item: bullets", |_| {});
        c.close_key("1b3a", "Item: plans to a machine", |_| {});
        c.close_key("1b3c", "Item: small key", |_| {});
        c.close_key("1b6c", "Item: old book", |_| {});
        c.close_key("1b84", "Item: wet rag", |_| {});
        c.close_key("1b98", "Item: the letter from R.", |_| {});
        c.close_key("1bb8", "Item: a key", |_| {});
        c.close_key("1bcc", "Item: camera with film in it", |_| {});
        c.close_key("1bd4", "Item: torn-up photograph", |_| {});
        c.close_key("1bfc", "Item: the key to the attic", |_| {});
        c.close_key("1cbe", "Item: Henry's letter", |_| {});
        c.close_key("1cf8", "Item: cap", |_| {});
        c.close_key("1d51", "Item: rod", |_| {});
        c.close_key("1d5b", "Item: the key to my room", |_| {});
        c.close_key("1da5", "Item: Robert's key", |_| {});
        c.close_key("1db2", "Item: William's key", |_| {});
        c.close_key("1dcf", "Item: a key", |_| {});
        c.close_key("1deb", "Item: untitled book", |_| {});
        c.close_key("1e1a", "Item: the key from the fountain", |_| {});
        c.close_key("1f0c", "Item: the key to the cellar", |_| {});
        c.close_key("1f4f", "Item: diamond", |_| {});
        c.close_key("1f71", "Item: flashlight", |_| {});
        c.close_key("1f77", "Item: shovel", |_| {});
        c.close_key("20a5", "Item: paper", |_| {});
        c.close_key("20ae", "Item: a key", |_| {});
        c.close_key("2188", "Item: wick", |_| {});
        c.close_key("219b", "Item: chock", |_| {});
        c.close_key("21a7", "Item: pin with a thread", |_| {});
        c.close_key("21b4", "Item: small bottle with dissolved ink", |_| {});
        c.close_key("21b6", "Item: cluster of keys", |_| {});
        c.close_key("21bb", "Item: label painted with glue", |_| {});
        c.close_key("21c1", "Item: William's watch", |_| {});
        c.close_key("21c7", "Item: William's watch", |_| {});
        c.close_key("21c8", "Item: small piece of paper", |_| {});
        c.close_key("mec", "Item: sword", |_| {});
        c.close_key("nit", "Item: thread", |_| {});
        c.close_key("tyc", "Item: sharp rod", |_| {});
        c.close_key("vez", "Item: black rook", |_| {});

        /*
        let mut inv_names = c.tree.children.iter()
            .filter(|(_, e)| e.children.contains_key("t"))
            .filter_map(|(k, _)| c.entries.get(&format!("inv.{k}.t")).map(|e| (k, &e.kind)))
            .filter_map(|(k, kind)| match &kind {
                AdbEntryKind::String { decoded, .. } => Some((k, decoded)),
                _ => None,
            })
            .collect::<Vec<_>>();
        inv_names.sort();
        for (k, name) in inv_names {
            println!("inv {k}: {name}");
        }
        */
        //for (k, entry) in c.tree.children.iter() {
        //    if let Some(child) = entry.children.get("t") {
        //        if let Some(AdbEntryKind::String { decoded, .. }) = c.entries.get(&format!("inv.{k}.t")).map(|e| &e.kind) {
        //            println!("inv {k}: {decoded}");
        //        }
        //    }
        //}
    });
}
