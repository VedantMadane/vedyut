use rustc_hash::FxHashMap;

pub struct SchemeData {
    pub name: &'static str,
    pub vowels: Vec<&'static str>,
    pub marks: Vec<&'static str>,
    pub consonants: Vec<&'static str>,
    pub others: Vec<&'static str>,
}

// Re-expose these helpers for direct use
pub fn get_slp1_swaras() -> Vec<&'static str> {
    vec![
        "a", "A", "i", "I", "u", "U", "f", "F", "x", "X", "e", "E", "o", "O",
    ]
}

pub fn get_slp1_vyanjanas() -> Vec<&'static str> {
    vec![
        "k", "K", "g", "G", "N", "c", "C", "j", "J", "Y", "w", "W", "q", "Q", "R", "t", "T", "d",
        "D", "n", "p", "P", "b", "B", "m", "y", "r", "l", "v", "S", "z", "s", "h",
    ]
}

pub fn get_devanagari_swaras() -> Vec<&'static str> {
    vec![
        "अ", "आ", "इ", "ई", "उ", "ऊ", "ऋ", "ॠ", "ऌ", "ॡ", "ए", "ऐ", "ओ", "औ",
    ]
}

pub fn get_devanagari_matras() -> Vec<&'static str> {
    vec!["", "ा", "ि", "ी", "ु", "ू", "ृ", "ॄ", "ॢ", "ॣ", "े", "ै", "ो", "ौ"]
}

pub fn get_devanagari_vyanjanas() -> Vec<&'static str> {
    vec![
        "क", "ख", "ग", "घ", "ङ", "च", "छ", "ज", "झ", "ञ", "ट", "ठ", "ड", "ढ", "ण", "त", "थ", "द",
        "ध", "न", "प", "फ", "ब", "भ", "म", "य", "र", "ल", "व", "श", "ष", "स", "ह",
    ]
}

pub fn get_slp1_scheme() -> SchemeData {
    SchemeData {
        name: "slp1",
        vowels: get_slp1_swaras(),
        marks: get_slp1_swaras(), // SLP1 doesn't distinguish marks
        consonants: get_slp1_vyanjanas(),
        others: vec!["M", "H", "", "'"],
    }
}

pub fn get_devanagari_marks() -> FxHashMap<String, String> {
    let mut map = FxHashMap::default();
    let slp1_v = get_slp1_swaras();
    let deva_m = get_devanagari_matras();

    for (s, d) in slp1_v.iter().zip(deva_m.iter()) {
        if !d.is_empty() {
            map.insert(s.to_string(), d.to_string());
        }
    }
    map
}

// IAST <-> SLP1
pub fn get_iast_to_slp1() -> Vec<(&'static str, &'static str)> {
    let mut map = vec![
        ("a", "a"),
        ("ā", "A"),
        ("i", "i"),
        ("ī", "I"),
        ("u", "u"),
        ("ū", "U"),
        ("ṛ", "f"),
        ("ṝ", "F"),
        ("ḷ", "x"),
        ("ḹ", "X"),
        ("e", "e"),
        ("ai", "E"),
        ("o", "o"),
        ("au", "O"),
        ("k", "k"),
        ("kh", "K"),
        ("g", "g"),
        ("gh", "G"),
        ("ṅ", "N"),
        ("c", "c"),
        ("ch", "C"),
        ("j", "j"),
        ("jh", "J"),
        ("ñ", "Y"),
        ("ṭ", "w"),
        ("ṭh", "W"),
        ("ḍ", "q"),
        ("ḍh", "Q"),
        ("ṇ", "R"),
        ("t", "t"),
        ("th", "T"),
        ("d", "d"),
        ("dh", "D"),
        ("n", "n"),
        ("p", "p"),
        ("ph", "P"),
        ("b", "b"),
        ("bh", "B"),
        ("m", "m"),
        ("y", "y"),
        ("r", "r"),
        ("l", "l"),
        ("v", "v"),
        ("ś", "S"),
        ("ṣ", "z"),
        ("s", "s"),
        ("h", "h"),
        ("ṃ", "M"),
        ("ḥ", "H"),
        ("'", "'"),
    ];
    // Sort by length of key descending
    map.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    map
}

pub fn get_hk_to_slp1() -> Vec<(&'static str, &'static str)> {
    let mut map = vec![
        ("a", "a"),
        ("A", "A"),
        ("i", "i"),
        ("I", "I"),
        ("u", "u"),
        ("U", "U"),
        ("R", "f"),
        ("RR", "F"),
        ("lR", "x"),
        ("lRR", "X"),
        ("e", "e"),
        ("ai", "E"),
        ("o", "o"),
        ("au", "O"),
        ("k", "k"),
        ("kh", "K"),
        ("g", "g"),
        ("gh", "G"),
        ("G", "N"),
        ("c", "c"),
        ("ch", "C"),
        ("j", "j"),
        ("jh", "J"),
        ("J", "Y"),
        ("T", "w"),
        ("Th", "W"),
        ("D", "q"),
        ("Dh", "Q"),
        ("N", "R"),
        ("t", "t"),
        ("th", "T"),
        ("d", "d"),
        ("dh", "D"),
        ("n", "n"),
        ("p", "p"),
        ("ph", "P"),
        ("b", "b"),
        ("bh", "B"),
        ("m", "m"),
        ("y", "y"),
        ("r", "r"),
        ("l", "l"),
        ("v", "v"),
        ("z", "S"),
        ("S", "z"),
        ("s", "s"),
        ("h", "h"),
        ("M", "M"),
        ("H", "H"),
        ("'", "'"),
    ];
    map.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    map
}

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
        'झ' => Some("झ"),
        'ञ' => Some("ञ"),
        'ट' => Some("ट"),
        'ठ' => Some("ठ"),
        'ड' => Some("ड"),
        'ढ' => Some("ढ"),
        'ण' => Some("ण"),
        'त' => Some("त"),
        'थ' => Some("थ"),
        'द' => Some("द"),
        'ध' => Some("ध"),
        'न' => Some("न"),
        'प' => Some("प"),
        'फ' => Some("फ"),
        'ब' => Some("ब"),
        'भ' => Some("भ"),
        'म' => Some("म"),
        'य' => Some("य"),
        'र' => Some("र"),
        'ल' => Some("ल"),
        'व' => Some("व"),
        'श' => Some("श"),
        'ष' => Some("ष"),
        'स' => Some("स"),
        'ह' => Some("ह"),
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