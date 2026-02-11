/// Sandhi splitting - reverse sandhi to find original words

/// Split a sandhi-combined word into possible original components
///
/// Assumes SLP1 input.
/// Returns vector of (left, right) tuples.
pub fn split_sandhi(text: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    // Iterate over char boundaries, skipping first and last (trivial splits)
    for (i, _) in text.char_indices().skip(1) {
        let left = &text[..i];
        let right = &text[i..];
        results.push((left.to_string(), right.to_string()));
    }

    for j in 0..n {
        let c = chars[j];

        match c {
            'A' => {
                // Dirgha: a+a, a+A, A+a, A+A
                add_splits(&mut results, &chars, j, 1, &["a", "A"], &["a", "A"]);
            }
            'I' => {
                // Dirgha: i+i, i+I, I+i, I+I
                add_splits(&mut results, &chars, j, 1, &["i", "I"], &["i", "I"]);
            }
            'U' => {
                // Dirgha: u+u...
                add_splits(&mut results, &chars, j, 1, &["u", "U"], &["u", "U"]);
            }
            'F' => {
                // Dirgha: f+f...
                add_splits(&mut results, &chars, j, 1, &["f", "F"], &["f", "F"]);
            }
            'e' => {
                // Guna: a/A + i/I
                add_splits(&mut results, &chars, j, 1, &["a", "A"], &["i", "I"]);
            }
            'o' => {
                // Guna: a/A + u/U
                add_splits(&mut results, &chars, j, 1, &["a", "A"], &["u", "U"]);
            }
            'E' => {
                // Vriddhi: a/A + e/E
                add_splits(&mut results, &chars, j, 1, &["a", "A"], &["e", "E"]);
            }
            'O' => {
                // Vriddhi: a/A + o/O
                add_splits(&mut results, &chars, j, 1, &["a", "A"], &["o", "O"]);
            }
            'y' => {
                // Yan: i/I + dissimilar vowel
                // But we don't know the following vowel.
                // The pattern is `y` + `V`.
                // Original: `i/I` + `V`.
                // So if we see `y` followed by vowel, we can split BEFORE `y` and change `y` to `i/I`.
                // Example: `ityAdi`. `y` + `A`.
                // Split: `iti` + `Adi`.
                // Logic: `left` = `...i`, `right` = `A...`.
                if j + 1 < n {
                    let next = chars[j + 1];
                    if is_vowel(next) {
                        // Replace 'y' with 'i'/'I'
                        // Split is AFTER the new 'i'/'I' and BEFORE 'next'.
                        let pre = chars[..j].iter().collect::<String>();
                        let post = chars[j + 1..].iter().collect::<String>();

                        results.push((format!("{}i", pre), format!("{}{}", next, post))); // Actually post includes next?
                                                                                          // Wait, `post` here is `chars[j+1..]`. `next` is `chars[j+1]`.
                                                                                          // So `post` starts with `next`.
                                                                                          // `right` should be `post`.
                        results.push((format!("{}i", pre), post.clone()));
                        results.push((format!("{}I", pre), post.clone()));
                    }
                }
            }
            'v' => {
                // Yan: u/U + vowel
                if j + 1 < n && is_vowel(chars[j + 1]) {
                    let pre = chars[..j].iter().collect::<String>();
                    let post = chars[j + 1..].iter().collect::<String>();
                    results.push((format!("{}u", pre), post.clone()));
                    results.push((format!("{}U", pre), post.clone()));
                }
            }
            'r' => {
                // Yan: f/F + vowel
                // Or Guna: a/A + f/F -> ar
                // 'r' is ambiguous.
                // Check if preceded by 'a'? "ar".
                if j > 0 && chars[j - 1] == 'a' {
                    // "ar". Could be Guna `a/A` + `f/F`.
                    // Replace "ar" with ...
                    // Wait, `j` is index of 'r'. `j-1` is 'a'.
                    // Pattern length 2.
                    // Split point is between vowels of original.
                    // Original: `...a` + `f...`
                    // Check `add_splits` for multi-char pattern.
                    // But manual logic here:
                    let pre = chars[..j - 1].iter().collect::<String>(); // before 'a'
                    let post = chars[j + 1..].iter().collect::<String>(); // after 'r'
                                                                          // Pairs: (a, f), (a, F), (A, f), (A, F)
                    for v1 in ["a", "A"] {
                        for v2 in ["f", "F"] {
                            results.push((format!("{}{}", pre, v1), format!("{}{}", v2, post)));
                        }
                    }
                }

                // Yan case: `r` + vowel (not preceded by vowel? No, `f` + vowel -> `r` + vowel).
                // e.g. `pitrartha` -> `pitf` + `artha`.
                if j + 1 < n && is_vowel(chars[j + 1]) {
                    // Check if `r` is part of consonant cluster?
                    // Yan `r` usually follows a consonant.
                    let pre = chars[..j].iter().collect::<String>();
                    let post = chars[j + 1..].iter().collect::<String>();
                    results.push((format!("{}f", pre), post.clone()));
                    results.push((format!("{}F", pre), post.clone()));
                }
            }
            _ => {}
        }

        // Multi-char patterns: Ay, Av, ay, av (Ayadi)
        // Check if `ay`
        if c == 'a' && j + 1 < n && chars[j + 1] == 'y' {
            // "ay" -> e + vowel
            if j + 2 < n && is_vowel(chars[j + 2]) {
                let pre = chars[..j].iter().collect::<String>();
                let post = chars[j + 2..].iter().collect::<String>(); // starts with vowel
                results.push((format!("{}e", pre), post));
            }
        }
        if c == 'a' && j + 1 < n && chars[j + 1] == 'v' {
            // "av" -> o + vowel
            if j + 2 < n && is_vowel(chars[j + 2]) {
                let pre = chars[..j].iter().collect::<String>();
                let post = chars[j + 2..].iter().collect::<String>();
                results.push((format!("{}o", pre), post));
            }
        }
        if c == 'A' && j + 1 < n && chars[j + 1] == 'y' {
            // "Ay" -> E + vowel
            if j + 2 < n && is_vowel(chars[j + 2]) {
                let pre = chars[..j].iter().collect::<String>();
                let post = chars[j + 2..].iter().collect::<String>();
                results.push((format!("{}E", pre), post));
            }
        }
        if c == 'A' && j + 1 < n && chars[j + 1] == 'v' {
            // "Av" -> O + vowel
            if j + 2 < n && is_vowel(chars[j + 2]) {
                let pre = chars[..j].iter().collect::<String>();
                let post = chars[j + 2..].iter().collect::<String>();
                results.push((format!("{}O", pre), post));
            }
        }
        // Also "al" (Guna)
        if c == 'a' && j + 1 < n && chars[j + 1] == 'l' {
            // "al" -> a/A + x/X
            // Need to check context? usually followed by consonant?
            // Guna applies usually.
            let pre = chars[..j].iter().collect::<String>();
            let post = chars[j + 2..].iter().collect::<String>();
            for v1 in ["a", "A"] {
                for v2 in ["x", "X"] {
                    results.push((format!("{}{}", pre, v1), format!("{}{}", v2, post)));
                }
            }
        }
    }

    // Deduplicate?
    results.sort();
    results.dedup();

    results
}

fn add_splits(
    results: &mut Vec<(String, String)>,
    chars: &[char],
    index: usize,
    pattern_len: usize,
    left_options: &[&str],
    right_options: &[&str],
) {
    let pre = chars[..index].iter().collect::<String>();
    let post = chars[index + pattern_len..].iter().collect::<String>();

    for l in left_options {
        for r in right_options {
            results.push((format!("{}{}", pre, l), format!("{}{}", r, post)));
        }
    }
}

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'A' | 'i' | 'I' | 'u' | 'U' | 'f' | 'F' | 'x' | 'X' | 'e' | 'E' | 'o' | 'O'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_dirgha() {
        let splits = split_sandhi("devAlaya");
        // Expect "deva" + "Alaya" among results
        assert!(splits.contains(&("deva".to_string(), "Alaya".to_string())));
    }

    #[test]
    fn test_split_guna() {
        let splits = split_sandhi("devendra");
        // Expect "deva" + "indra"
        assert!(splits.contains(&("deva".to_string(), "indra".to_string())));
    }

    #[test]
    fn test_split_yan() {
        let splits = split_sandhi("ityAdi");
        // Expect "iti" + "Adi"
        assert!(splits.contains(&("iti".to_string(), "Adi".to_string())));
    }

    #[test]
    fn test_split_ayadi() {
        let splits = split_sandhi("nayanam");
        // Expect "ne" + "anam"
        assert!(splits.contains(&("ne".to_string(), "anam".to_string())));
    }
}
