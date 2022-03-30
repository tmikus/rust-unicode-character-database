use lazy_static::lazy_static;
use std::collections::HashMap;

// DO NOT MODIFY:
// This file was generated with ucd_generate_alphabets

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Alphabet {
    NonAlphabet,
    Whitespace,
    Arabic,
    CJK,
    Cyrillic,
    Greek,
    Hangul,
    Hebrew,
    Hiragana,
    Katakana,
    Latin,
    Thai,
    Turkic,
}

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
        (Alphabet::NonAlphabet, "NonAlphabet"),
        (Alphabet::Whitespace, "Whitespace"),
        (Alphabet::Arabic, "Arabic"),
        (Alphabet::CJK, "CJK"),
        (Alphabet::Cyrillic, "Cyrillic"),
        (Alphabet::Greek, "Greek"),
        (Alphabet::Hangul, "Hangul"),
        (Alphabet::Hebrew, "Hebrew"),
        (Alphabet::Hiragana, "Hiragana"),
        (Alphabet::Katakana, "Katakana"),
        (Alphabet::Latin, "Latin"),
        (Alphabet::Thai, "Thai"),
        (Alphabet::Turkic, "Turkic"),
]);

    static ref STR_TO_ENUM: HashMap<&'static str, Alphabet> = HashMap::from([
        ("NonAlphabet", Alphabet::NonAlphabet),
        ("Whitespace", Alphabet::Whitespace),
        ("Arabic", Alphabet::Arabic),
        ("CJK", Alphabet::CJK),
        ("Cyrillic", Alphabet::Cyrillic),
        ("Greek", Alphabet::Greek),
        ("Hangul", Alphabet::Hangul),
        ("Hebrew", Alphabet::Hebrew),
        ("Hiragana", Alphabet::Hiragana),
        ("Katakana", Alphabet::Katakana),
        ("Latin", Alphabet::Latin),
        ("Thai", Alphabet::Thai),
        ("Turkic", Alphabet::Turkic),
]);
}
    