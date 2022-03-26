use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use ucd_parser::CharacterInfo;

fn write_names_map(characters: Vec<CharacterInfo>) {
    let mut file = File::create("../ucd_names/src/codepoint_to_name.rs").unwrap();
    file.write_all(b"pub const CODEPOINT_TO_NAME: &'static [&'static str] = &[\n").unwrap();
    let mut last_codepoint: u32 = 0;
    for character in characters.into_iter() {
        if character.codepoint > 0 {
            for _ in (last_codepoint + 1)..character.codepoint {
                file.write_all(format!("    \"\",\n").as_bytes());
            }
        }
        last_codepoint = character.codepoint;
        file.write_all(format!("    \"{}\",\n", character.name).as_bytes());
    }
    file.write_all(b"];\n");
}

fn main() {
    let characters = ucd_parser::read_character_info("../data");
    write_names_map(characters)
}
