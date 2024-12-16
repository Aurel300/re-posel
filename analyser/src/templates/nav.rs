use std::collections::HashMap;

use crate::adb::{AdbEntry, AdbEntryKind};


pub struct NavTree {
    pub key: String,
    pub kind: &'static str,
    pub children: HashMap<String, NavTree>,
}

impl NavTree {
    pub fn get(&self, key: &str) -> &Self {
        self.children.get(key).unwrap()
    }

    pub fn get_mut(&mut self, key: &str) -> &mut Self {
        self.children.get_mut(key).unwrap()
    }

    pub fn flatten(&self) -> NavEntry {
        let mut children = self.children.values().map(|t| t.flatten()).collect::<Vec<_>>();
        children.sort_by_key(|t| t.key.clone());
        NavEntry {
            key: self.key.to_string(),
            kind: self.kind,
            children,
        }
    }

    pub fn add<'a>(&mut self, mut comps: impl Iterator<Item = &'a str>, kind: &'static str) {
        if let Some(comp) = comps.next() {
            self.children.entry(comp.to_string())
                .or_insert_with(|| NavTree {
                    key: comp.to_string(),
                    kind: "dummy",
                    children: HashMap::new(),
                })
                .add(comps, kind);
        } else {
            self.kind = kind;
        }
    }

    pub fn add_dummies(&self, prefix: &mut Vec<String>, entries: &mut HashMap<String, AdbEntry>) {
        let prefix_full = prefix.iter().cloned().collect::<String>();
        let full_name = format!("{prefix_full}{}", self.key);
        if !full_name.is_empty() && !entries.contains_key(&full_name) {
            entries.insert(full_name, AdbEntry::new(AdbEntryKind::Dummy));
        }
        if !self.key.is_empty() {
            prefix.push(self.key.clone());
            prefix.push(".".to_string());
        }
        for child in self.children.values() {
            child.add_dummies(prefix, entries);
        }
        if !self.key.is_empty() {
            prefix.pop();
            prefix.pop();
        }
    }
}

pub struct NavEntry {
    key: String,
    kind: &'static str,
    children: Vec<NavEntry>,
}

impl NavEntry {
    fn render_into(&self, active_key: &str, prefix: &mut Vec<String>, buf: &mut String, entries: &HashMap<String, AdbEntry>) {
        let prefix_full = prefix.iter().cloned().collect::<String>();
        let full_name = format!("{prefix_full}{}", self.key);
        let selected = if full_name == active_key { " nav-selected" } else { "" };
        let mut open_key = true;
        let mut rev_name = "".to_string();
        if let Some(entry) = entries.get(&full_name) {
            open_key = entry.open_key;
            rev_name = entry.name.as_ref().map(|s| format!(" ({s})")).unwrap_or_default();
        }
        if self.children.is_empty() {
            buf.push_str(&format!("<a href=\"{full_name}.html\" class=\"nav-leaf nav-{}{selected}\">{}{}</a>", self.kind, self.key, rev_name));
            return;
        }
        let open = if open_key { " open" } else { "" };
        buf.push_str(&format!("<details{open}><summary><a href=\"{full_name}.html\" class=\"nav-{}{selected}\">{}</a> ({}){}</summary>", self.kind, self.key, self.children.len(), rev_name));
        if !self.key.is_empty() {
            prefix.push(self.key.clone());
            prefix.push(".".to_string());
        }
        self.children.iter().for_each(|e| e.render_into(active_key, prefix, buf, entries));
        if !self.key.is_empty() {
            prefix.pop();
            prefix.pop();
        }
        buf.push_str("</details>");
    }

    pub fn render(&self, active_key: &str, entries: &HashMap<String, AdbEntry>) -> String {
        let mut ret = String::new();
        let mut prefix = Vec::new();
        self.render_into(active_key, &mut prefix, &mut ret, entries);
        ret
    }
}
