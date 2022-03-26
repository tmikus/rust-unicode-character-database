use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::attributes::Attributes;
use crate::CharacterInfo;

fn character_info_from_xml(attributes: Attributes) -> Option<CharacterInfo> {
    let mut codepoint = None;
    let mut name = None;
    for attr in attributes {
        let attr = attr.unwrap();
        if attr.key == b"cp" {
            codepoint = Some(
                u32::from_str_radix(
                    std::str::from_utf8(attr.value.as_ref()).unwrap(),
                    16,
                ).unwrap(),
            );
            continue;
        }
        if attr.key == b"na" {
            name = Some(std::str::from_utf8(attr.value.as_ref()).unwrap().to_owned());
        }
    }
    match (codepoint, name) {
        (Some(cp), Some(n)) => Some(CharacterInfo {
            codepoint: cp,
            name: n,
        }),
        _ => None
    }
}

pub fn read_character_info(dir: &str) -> Vec<CharacterInfo> {
    let file = File::open(Path::new(dir).join("ucd.all.flat.xml")).unwrap();
    let file = BufReader::new(file);
    let mut reader = Reader::from_reader(file);
    let mut character_info = vec![];
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) if e.name() == b"char" => {
                match character_info_from_xml(e.attributes()) {
                    Some(ci) => character_info.push(ci),
                    _ => (),
                }
            },
            Ok(Event::Start(ref e)) if e.name() == b"char" => {
                match character_info_from_xml(e.attributes()) {
                    Some(ci) => character_info.push(ci),
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }
    // for e in parser {
    //     match e {
    //         Ok(XmlEvent::StartElement { attributes, name, .. }) if name.local_name == "char" => {
    //             match character_info_from_xml(attributes) {
    //                 Some(ci) => character_info.push(ci),
    //                 _ => {},
    //             }
    //         }
    //         _ => {},
    //     }
    // }
    character_info
}


// use std::fs::File;
// use std::io::BufReader;
// use std::path::Path;
// use xml::{ EventReader };
// use xml::attribute::OwnedAttribute;
// use xml::reader::XmlEvent;
// use crate::CharacterInfo;
//
// fn character_info_from_xml(attributes: Vec<OwnedAttribute>) -> Option<CharacterInfo> {
//     let mut codepoint = None;
//     let mut name = None;
//     for attr in attributes {
//         if attr.name.local_name == "cp" {
//             codepoint = Some(u32::from_str_radix(&attr.value, 16).unwrap());
//             continue;
//         }
//         if attr.name.local_name == "na" {
//             name = Some(attr.value);
//         }
//     }
//     match (codepoint, name) {
//         (Some(cp), Some(n)) => Some(CharacterInfo {
//             codepoint: cp,
//             name: n,
//         }),
//         _ => None
//     }
// }
//
// pub fn read_character_info(dir: &str) -> Vec<CharacterInfo> {
//     let file = File::open(Path::new(dir).join("ucd.all.flat.xml")).unwrap();
//     let file = BufReader::new(file);
//     let parser = EventReader::new(file);
//     let mut character_info = vec![];
//     for e in parser {
//         match e {
//             Ok(XmlEvent::StartElement { attributes, name, .. }) if name.local_name == "char" => {
//                 match character_info_from_xml(attributes) {
//                     Some(ci) => character_info.push(ci),
//                     _ => {},
//                 }
//             }
//             _ => {},
//         }
//     }
//     character_info
// }
