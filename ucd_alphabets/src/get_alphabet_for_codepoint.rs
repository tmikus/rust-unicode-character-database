use crate::Alphabet;
use crate::CODEPOINT_TO_ALPHABET;

pub fn get_alphabet_for_codepoint(codepoint: u32) -> Option<Alphabet> {
    if codepoint as usize >= CODEPOINT_TO_ALPHABET.len() {
        return None;
    }
    Some(CODEPOINT_TO_ALPHABET[codepoint as usize].clone())
}
