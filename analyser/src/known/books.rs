use super::KnownContext;

pub(super) fn apply_known_books(c: &mut KnownContext) {
    c.open_key("13fd", "Book: William's diary", |_c| {});
}
