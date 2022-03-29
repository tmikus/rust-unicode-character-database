#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Alphabet {
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
    NonAlphabet,
    Whitespace,
}

impl Default for Alphabet {
    fn default() -> Self {
        Self::NonAlphabet
    }
}
