// Goal is to take a string of SLP1 transliterated Sanskrit text
// and clean it (Remove Spaces, Remove tags, Remove Special Characters)
use crate::scheme::Set;
use lazy_static::lazy_static;

/// Copied from sounds.rs in vidyut-sandhi crate
/// Returns whether the given character is a Sanskrit sound or not.
/// Non-Sanskrit sounds include:
/// s - avagraha
/// - spaces
/// - other punctuation characters (|, ||, numbers)
/// - characters or symbols from non-SLP1 encodings
pub fn is_sanskrit(c: char) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from("aAiIuUfFxXeEoOMHkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL");
    }
    CHARS.contains(c)
}

//// Remove all non-sanskrit sounds from the input strings.
/// Note: This may break when reading verses from html pages and kind where text
/// is enclosed within tags
pub fn clean(mut raw: String) -> String {
    let mut tmp = String::new();
    // Remove all non-sanskrit characters (including avagraha and pluta)
    for i in raw.chars() {
        if !is_sanskrit(i) {
            tmp.push(i);
        }
    }
    for i in tmp.chars() {
        raw = raw.replace(i, "");
    }
    raw
}
