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
        (Alphabet::NonAlphabet, "non-alphabet"),
        (Alphabet::Whitespace, "whitespace"),
        (Alphabet::Arabic, "arabic"),
        (Alphabet::CJK, "cjk"),
        (Alphabet::Cyrillic, "cyrillic"),
        (Alphabet::Greek, "greek"),
        (Alphabet::Hangul, "hangul"),
        (Alphabet::Hebrew, "hebrew"),
        (Alphabet::Hiragana, "hiragana"),
        (Alphabet::Katakana, "katakana"),
        (Alphabet::Latin, "latin"),
        (Alphabet::Thai, "thai"),
        (Alphabet::Turkic, "turkic"),
]);

    static ref STR_TO_ENUM: HashMap<&'static str, Alphabet> = HashMap::from([
        ("non-alphabet", Alphabet::NonAlphabet),
        ("whitespace", Alphabet::Whitespace),
        ("arabic", Alphabet::Arabic),
        ("cjk", Alphabet::CJK),
        ("cyrillic", Alphabet::Cyrillic),
        ("greek", Alphabet::Greek),
        ("hangul", Alphabet::Hangul),
        ("hebrew", Alphabet::Hebrew),
        ("hiragana", Alphabet::Hiragana),
        ("katakana", Alphabet::Katakana),
        ("latin", Alphabet::Latin),
        ("thai", Alphabet::Thai),
        ("turkic", Alphabet::Turkic),
]);
}
    