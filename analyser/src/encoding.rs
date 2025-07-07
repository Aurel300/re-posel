use std::cell::Cell;

thread_local! {
    pub static ENCODING: Cell<Option<&'static encoding_rs::Encoding>> = const { Cell::new(None) };
}

pub fn set_encoding(label: &str) {
    ENCODING.with(|c| {
        c.set(Some(encoding_rs::Encoding::for_label(label.as_bytes())
            .expect("no such encoding")));
    });
}

pub fn decode(data: &[u8]) -> String {
    ENCODING.with(|c| {
        let encoding = c.get()
            .unwrap_or(encoding_rs::WINDOWS_1250);
        encoding
            .decode_without_bom_handling_and_without_replacement(data)
            .unwrap()
            .to_string()
    })
}
