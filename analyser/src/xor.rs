pub const XOR_KEY: &[u8] = b"Vyvojovy tym AGDS varuje: Hackerovani skodi obchodu!";

pub fn dexor(buffer: &mut [u8]) {
    buffer.iter_mut()
        .zip(XOR_KEY.iter().cycle())
        .for_each(|(b, k)| *b ^= k ^ 0xFF);
}
