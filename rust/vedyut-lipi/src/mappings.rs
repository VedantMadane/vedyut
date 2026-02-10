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
