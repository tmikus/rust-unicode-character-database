use crate::Alphabet;
use crate::get_alphabet_for_codepoint;

pub fn get_alphabet_for_char(ch: char) -> Option<Alphabet> {
    get_alphabet_for_codepoint(ch as u32)
}
