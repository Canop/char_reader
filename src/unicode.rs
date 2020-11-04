// Everyting in this file has been taken in std::str, where
// it hides behind an unstable feature flag.
// I hope to trash this file at some point.

/// assuming there's enough bytes in the buffer, read
/// the code point at current position, don't increment it.
///
/// This code mimics the one of str::next_code_point
/// (currently unstable)
/// As we already have the size it could be simplified.
pub fn read_code_point(buffer: &[u8], pos: usize, char_size: usize) -> u32 {
    let x = buffer[pos];
    if char_size == 1 {
        return x as u32;
    }
    // Multibyte case follows
    // Decode from a byte combination out of: [[[x y] z] w]
    let init = utf8_first_byte(x, 2);
    let y = buffer[pos + 1];
    let mut ch = utf8_acc_cont_byte(init, y);
    if x >= 0xE0 {
        // [[x y z] w] case
        // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
        let z = buffer[pos + 2];
        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            // [x y z w] case
            // use only the lower 3 bits of `init`
            let w = buffer[pos + 3];
            ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
        }
    }
    ch
}

/// Returns the initial codepoint accumulator for the first byte.
/// The first byte is special, only want bottom 5 bits for width 2, 4 bits
/// for width 3, and 3 bits for width 4.
///
/// taken in str unstable internals
#[inline]
fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

/// Returns the value of `ch` updated with continuation byte `byte`.
///
/// taken in str unstable internals
#[inline]
fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

// https://tools.ietf.org/html/rfc3629
static UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x1F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x3F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x5F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x7F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0xBF
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];

/// Given a first byte, determines how many bytes are in this UTF-8 character.
#[inline]
pub fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}

/// Mask of the value bits of a continuation byte.
const CONT_MASK: u8 = 0b0011_1111;
