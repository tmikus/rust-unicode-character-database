use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use ucd_parser::CharacterInfo;

fn get_alphabet_for_name(codepoint: u32, name: String) -> String {
    if is_white_space(codepoint) {
        return "Alphabet::Whitespace".to_owned()
    }
    let name = name.to_uppercase();
    if name.contains("ARABIC") {
        return "Alphabet::Arabic".to_owned();
    }
    if name.contains("CJK") {
        return "Alphabet::CJK".to_owned();
    }
    if name.contains("CYRILLIC") {
        return "Alphabet::Cyrillic".to_owned();
    }
    if name.contains("GREEK") {
        return "Alphabet::Greek".to_owned();
    }
    if name.contains("HANGUL") {
        return "Alphabet::Hangul".to_owned();
    }
    if name.contains("HEBREW") {
        return "Alphabet::Hebrew".to_owned();
    }
    if name.contains("HIRAGANA") {
        return "Alphabet::Hiragana".to_owned();
    }
    if name.contains("KATAKANA") {
        return "Alphabet::Katakana".to_owned();
    }
    if name.contains("LATIN") {
        return "Alphabet::Latin".to_owned();
    }
    if name.contains("THAI") {
        return "Alphabet::Thai".to_owned();
    }
    if name.contains("TURKIC") {
        return "Alphabet::Turkic".to_owned();
    }
    "Alphabet::NonAlphabet".to_owned()
}

/*
    Based on the PropList.txt
    0009..000D    ; White_Space # Cc   [5] <control-0009>..<control-000D>
    0020          ; White_Space # Zs       SPACE
    0085          ; White_Space # Cc       <control-0085>
    00A0          ; White_Space # Zs       NO-BREAK SPACE
    1680          ; White_Space # Zs       OGHAM SPACE MARK
    2000..200A    ; White_Space # Zs  [11] EN QUAD..HAIR SPACE
    2028          ; White_Space # Zl       LINE SEPARATOR
    2029          ; White_Space # Zp       PARAGRAPH SEPARATOR
    202F          ; White_Space # Zs       NARROW NO-BREAK SPACE
    205F          ; White_Space # Zs       MEDIUM MATHEMATICAL SPACE
    3000          ; White_Space # Zs       IDEOGRAPHIC SPACE
 */
fn is_white_space(codepoint: u32) -> bool {
   (codepoint >= 0x0009 && codepoint <= 0x000d)
   || codepoint == 0x0020
   || codepoint == 0x0085
   || codepoint == 0x00a0
   || codepoint == 0x1680
   || (codepoint >= 0x2000 && codepoint <= 0x200a)
   || codepoint == 0x2028
   || codepoint == 0x2029
   || codepoint == 0x202f
   || codepoint == 0x205f
   || codepoint == 0x3000
}

fn write_names_map(characters: Vec<CharacterInfo>) {
    let mut file = File::create("../ucd_alphabets/src/codepoint_to_alphabet.rs").unwrap();
    file.write_all(b"use crate::Alphabet;\n");
    file.write_all(b"\n");
    file.write_all(b"pub const CODEPOINT_TO_ALPHABET: &'static [Alphabet] = &[\n").unwrap();
    let mut last_codepoint: u32 = 0;
    for character in characters.into_iter() {
        if character.codepoint > 0 {
            for _ in (last_codepoint + 1)..character.codepoint {
                file.write_all(format!("    Alphabet::NonAlphabet,\n").as_bytes());
            }
        }
        last_codepoint = character.codepoint;
        file.write_all(format!("    {},\n", get_alphabet_for_name(character.codepoint, character.name)).as_bytes());
    }
    file.write_all(b"];\n");
}

fn main() {
    let characters = ucd_parser::read_character_info("../data");
    write_names_map(characters)
}
