use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::prelude::*;
use ucd_parser::CharacterInfo;

const ALPHABETS_JSON: &'static str = include_str!("alphabets.json");

fn get_alphabets() -> Vec<String> {
    serde_json::from_str(ALPHABETS_JSON).unwrap()
}

fn get_alphabet_for_name(alphabets: &Vec<String>, codepoint: u32, name: String) -> String {
    if is_white_space(codepoint) {
        return "Alphabet::Whitespace".to_owned()
    }
    let name = name.to_uppercase();
    for alphabet in alphabets {
        if name.contains(&alphabet.to_uppercase()) {
            return format!("Alphabet::{}", alphabet);
        }
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

fn write_alphabets_enum(alphabets: &Vec<String>) {
    let mut file = File::create("../ucd_alphabets/src/alphabet.rs").unwrap();
    file.write_all(b"\
use lazy_static::lazy_static;
use std::collections::HashMap;

// DO NOT MODIFY:
// This file was generated with ucd_generate_alphabets

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Alphabet {
    NonAlphabet,
    Whitespace,
");
    for alphabet in alphabets {
        file.write_all(format!("    {},\n", alphabet).as_bytes());
    }
    file.write_all(b"}\n");
    file.write_all(b"
impl Alphabet {
    pub fn from_str(value: &str) -> Option<Alphabet> {
        STR_TO_ENUM.get(value).map(|a| a.to_owned())
    }

    pub fn to_str(&self) -> &'static str {
        ENUM_TO_STR.get(self).unwrap()
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        Self::NonAlphabet
    }
}

impl From<String> for Alphabet {
    fn from(value: String) -> Self {
        Alphabet::from_str(&value).unwrap()
    }
}

impl Into<String> for Alphabet {
    fn into(self) -> String {
        self.to_str().to_owned()
    }
}

impl Into<String> for &Alphabet {
    fn into(self) -> String {
        self.to_str().to_owned()
    }
}

lazy_static! {
    static ref ENUM_TO_STR: HashMap<Alphabet, &'static str> = HashMap::from([
        (Alphabet::NonAlphabet, \"non-alphabet\"),
        (Alphabet::Whitespace, \"whitespace\"),
");
    for alphabet in alphabets {
        file.write_all(format!(
            "        (Alphabet::{}, \"{}\"),\n",
            alphabet,
            alphabet.to_lowercase(),
        ).as_bytes());
    }
    file.write_all(b"\
    ]);

    static ref STR_TO_ENUM: HashMap<&'static str, Alphabet> = HashMap::from([
        (\"non-alphabet\", Alphabet::NonAlphabet),
        (\"whitespace\", Alphabet::Whitespace),
");
    for alphabet in alphabets {
        file.write_all(format!(
            "        (\"{}\", Alphabet::{}),\n",
            alphabet.to_lowercase(),
            alphabet,
        ).as_bytes());
    }
    file.write_all(b"\
    ]);
}
    ");
}

fn write_names_map(alphabets: &Vec<String>, characters: Vec<CharacterInfo>) {
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
        file.write_all(format!(
            "    {},\n",
            get_alphabet_for_name(alphabets, character.codepoint, character.name),
        ).as_bytes());
    }
    file.write_all(b"];\n");
}

fn main() {
    let alphabets = get_alphabets();
    write_alphabets_enum(&alphabets);
    let characters = ucd_parser::read_character_info("../data");
    write_names_map(&alphabets, characters)
}
