// ... (existing code)

pub fn get_slp1_to_devanagari(c: char) -> Option<&'static str> {
    match c {
        // Vowels
        'a' => Some("अ"),
        'A' => Some("आ"),
        'i' => Some("इ"),
        'I' => Some("ई"),
        'u' => Some("उ"),
        'U' => Some("ऊ"),
        'f' => Some("ऋ"),
        'F' => Some("ॠ"),
        'x' => Some("ऌ"),
        'X' => Some("ॡ"),
        'e' => Some("ए"),
        'E' => Some("ऐ"),
        'o' => Some("ओ"),
        'O' => Some("औ"),
        // Consonants
        'k' => Some("क"),
        'K' => Some("ख"),
        'g' => Some("ग"),
        'G' => Some("घ"),
        'N' => Some("ङ"),
        'c' => Some("च"),
        'C' => Some("छ"),
        'j' => Some("ज"),
        'J' => Some("झ"),
        'Y' => Some("ञ"),
        'w' => Some("ट"),
        'W' => Some("ठ"),
        'q' => Some("ड"),
        'Q' => Some("ढ"),
        'R' => Some("ण"),
        't' => Some("त"),
        'T' => Some("थ"),
        'd' => Some("द"),
        'D' => Some("ध"),
        'n' => Some("न"),
        'p' => Some("प"),
        'P' => Some("फ"),
        'b' => Some("ब"),
        'B' => Some("भ"),
        'm' => Some("म"),
        'y' => Some("य"),
        'r' => Some("र"),
        'l' => Some("ल"),
        'v' => Some("व"),
        'S' => Some("श"),
        'z' => Some("ष"),
        's' => Some("स"),
        'h' => Some("ह"),
        // Others
        'M' => Some("ं"),
        'H' => Some("ः"),
        '~' => Some("ँ"),
        '\'' => Some("ऽ"),
        _ => None,
    }
}

pub fn get_slp1_matra_devanagari(c: char) -> Option<&'static str> {
    match c {
        'a' => Some(""), // Inherent 'a' has no matra
        'A' => Some("ा"),
        'i' => Some("ि"),
        'I' => Some("ी"),
        'u' => Some("ु"),
        'U' => Some("ू"),
        'f' => Some("ृ"),
        'F' => Some("ॄ"),
        'x' => Some("ॢ"),
        'X' => Some("ॣ"),
        'e' => Some("े"),
        'E' => Some("ै"),
        'o' => Some("ो"),
        'O' => Some("ौ"),
        _ => None,
    }
}

pub fn is_slp1_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
}

pub fn is_slp1_consonant(c: char) -> bool {
    matches!(
        c,
        'k' | 'K'
            | 'g'
            | 'G'
            | 'N'
            | 'c'
            | 'C'
            | 'j'
            | 'J'
            | 'Y'
            | 'w'
            | 'W'
            | 'q'
            | 'Q'
            | 'R'
            | 't'
            | 'T'
            | 'd'
            | 'D'
            | 'n'
            | 'p'
            | 'P'
            | 'b'
            | 'B'
            | 'm'
            | 'y'
            | 'r'
            | 'l'
            | 'v'
            | 'S'
            | 'z'
            | 's'
            | 'h'
    )
}

// Reverse mapping for IAST to SLP1 (ordered by length descending for greedy match)
pub static IAST_TO_SLP1: &[(&str, &str)] = &[
    ("ai", "E"),
    ("au", "O"),
    ("kh", "K"),
    ("gh", "G"),
    ("ch", "C"),
    ("jh", "J"),
    ("ṭh", "W"),
    ("ḍh", "Q"),
    ("th", "T"),
    ("dh", "D"),
    ("ph", "P"),
    ("bh", "B"),
    ("ṛ", "f"),
    ("ṝ", "F"),
    ("ḷ", "x"),
    ("ḹ", "X"),
    ("ṅ", "N"),
    ("ñ", "Y"),
    ("ṭ", "w"),
    ("ḍ", "q"),
    ("ṇ", "R"),
    ("ś", "S"),
    ("ṣ", "z"),
    ("ṃ", "M"),
    ("ḥ", "H"),
    ("m̐", "~"),
    ("ā", "A"),
    ("ī", "I"),
    ("ū", "U"),
    ("a", "a"),
    ("i", "i"),
    ("u", "u"),
    ("e", "e"),
    ("o", "o"),
    ("k", "k"),
    ("g", "g"),
    ("c", "c"),
    ("j", "j"),
    ("t", "t"),
    ("d", "d"),
    ("n", "n"),
    ("p", "p"),
    ("b", "b"),
    ("m", "m"),
    ("y", "y"),
    ("r", "r"),
    ("l", "l"),
    ("v", "v"),
    ("s", "s"),
    ("h", "h"),
];

pub fn get_iast_to_slp1_map() -> &'static [(&'static str, &'static str)] {
    IAST_TO_SLP1
}

pub fn get_devanagari_consonant_to_slp1(c: char) -> Option<char> {
    match c {
        'क' => Some('k'),
        'ख' => Some('K'),
        'ग' => Some('g'),
        'घ' => Some('G'),
        'ङ' => Some('N'),
        'च' => Some('c'),
        'छ' => Some('C'),
        'ज' => Some('j'),
        'झ' => Some('J'),
        'ञ' => Some('Y'),
        'ट' => Some('w'),
        'ठ' => Some('W'),
        'ड' => Some('q'),
        'ढ' => Some('Q'),
        'ण' => Some('R'),
        'त' => Some('t'),
        'थ' => Some('T'),
        'द' => Some('d'),
        'ध' => Some('D'),
        'न' => Some('n'),
        'प' => Some('p'),
        'फ' => Some('P'),
        'ब' => Some('b'),
        'भ' => Some('B'),
        'म' => Some('m'),
        'य' => Some('y'),
        'र' => Some('r'),
        'ल' => Some('l'),
        'व' => Some('v'),
        'श' => Some('S'),
        'ष' => Some('z'),
        'स' => Some('s'),
        'ह' => Some('h'),
        _ => None,
    }
}

pub fn get_devanagari_matra_to_slp1(c: char) -> Option<char> {
    match c {
        'ा' => Some('A'),
        'ि' => Some('i'),
        'ी' => Some('I'),
        'ु' => Some('u'),
        'ू' => Some('U'),
        'ृ' => Some('f'),
        'ॄ' => Some('F'),
        'ॢ' => Some('x'),
        'ॣ' => Some('X'),
        'े' => Some('e'),
        'ै' => Some('E'),
        'ो' => Some('o'),
        'ौ' => Some('O'),
        _ => None,
    }
}

pub fn get_devanagari_vowel_to_slp1(c: char) -> Option<char> {
    match c {
        'अ' => Some('a'),
        'आ' => Some('A'),
        'इ' => Some('i'),
        'ई' => Some('I'),
        'उ' => Some('u'),
        'ऊ' => Some('U'),
        'ऋ' => Some('f'),
        'ॠ' => Some('F'),
        'ऌ' => Some('x'),
        'ॡ' => Some('X'),
        'ए' => Some('e'),
        'ऐ' => Some('E'),
        'ओ' => Some('o'),
        'औ' => Some('O'),
        _ => None,
    }
}

pub fn get_devanagari_other_to_slp1(c: char) -> Option<char> {
    match c {
        'ं' => Some('M'),
        'ः' => Some('H'),
        'ँ' => Some('~'),
        'ऽ' => Some('\''),
        _ => None,
    }
}
