use crate::prakriya::Prakriya;
use crate::tag::Tag;
use crate::term::Term;

/// 6.1.101 akaḥ savarṇe dīrghaḥ
pub fn rule_6_1_101(p: &mut Prakriya) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < p.terms.len().saturating_sub(1) {
        let left = &p.terms[i];
        let right = &p.terms[i + 1];

        let result = match (left.text.as_str(), right.text.as_str()) {
            ("a", "a") | ("a", "A") | ("A", "a") | ("A", "A") => Some("A"),
            ("i", "i") | ("i", "I") | ("I", "i") | ("I", "I") => Some("I"),
            ("u", "u") | ("u", "U") | ("U", "u") | ("U", "U") => Some("U"),
            _ => None,
        };

        if let Some(res) = result {
             p.terms[i].text = res.to_string();
             p.terms[i].add_tag(Tag::Guna);
             p.terms.remove(i + 1);
             p.add_rule("6.1.101 akaḥ savarṇe dīrghaḥ");
             changed = true;
        } else {
            i += 1;
        }
    }
    changed
}

/// 6.1.77 iko yaṇaci
pub fn rule_6_1_77(p: &mut Prakriya) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < p.terms.len().saturating_sub(1) {
        let left_text = p.terms[i].text.clone();
        let right_text = p.terms[i+1].text.clone();

        let left_last = left_text.chars().last().unwrap();
        let right_first = right_text.chars().next().unwrap();

        if "iIuUfFxX".contains(left_last) && "aAiIuUfFxXeEoO".contains(right_first) {
             let replacement = match left_last {
                 'i' | 'I' => "y",
                 'u' | 'U' => "v",
                 'f' | 'F' => "r",
                 'x' | 'X' => "l",
                 _ => "",
             };

             if !replacement.is_empty() {
                 let new_left = format!("{}{}", &left_text[..left_text.len()-left_last.len_utf8()], replacement);
                 p.terms[i].text = new_left;
                 p.add_rule("6.1.77 iko yaṇaci");
                 changed = true;
             }
        }
        i += 1;
    }
    changed
}

/// 8.2.66 sasajuṣo ruḥ
pub fn rule_8_2_66(p: &mut Prakriya) -> bool {
    let mut changed = false;
    // Iterate indices to avoid borrowing conflict
    for i in 0..p.terms.len() {
        let term = &p.terms[i];
        if term.text.ends_with('s') && term.has_tag(Tag::Pada) {
             p.terms[i].text.pop();
             p.terms[i].text.push_str("ru~");
             p.add_rule("8.2.66 sasajuṣo ruḥ");
             changed = true;
        }
    }
    changed
}

/// 6.1.78 eco'yavāyāvaḥ
pub fn rule_6_1_78(p: &mut Prakriya) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < p.terms.len().saturating_sub(1) {
        let left_text = p.terms[i].text.clone();
        let right_text = p.terms[i+1].text.clone();

        // Simplified check: if left ends in e/o/ai/au and right starts with vowel
        let last = left_text.chars().last().unwrap();
        let first = right_text.chars().next().unwrap();

        if "eEoO".contains(last) && "aAiIuUfFxXeEoO".contains(first) {
            let replacement = match last {
                'e' => "ay",
                'o' => "av",
                'E' => "Ay",
                'O' => "Av",
                _ => "",
            };

            if !replacement.is_empty() {
                let new_left = format!("{}{}", &left_text[..left_text.len()-last.len_utf8()], replacement);
                p.terms[i].text = new_left;
                p.add_rule("6.1.78 eco'yavāyāvaḥ");
                changed = true;
            }
        }
        i += 1;
    }
    changed
}

/// 8.3.15 kharavasānayorvisarjanīyaḥ
pub fn rule_8_3_15(p: &mut Prakriya) -> bool {
    let mut changed = false;
    let len = p.terms.len();
    if len > 0 {
        let last_idx = len - 1;
        let text = &p.terms[last_idx].text;
        if text.ends_with("r") || text.ends_with("ru~") {
             if text.ends_with("ru~") {
                 let new_text = text.trim_end_matches("ru~").to_string();
                 p.terms[last_idx].text = new_text;
             } else {
                 p.terms[last_idx].text.pop();
             }
             p.terms[last_idx].text.push('H');
             p.add_rule("8.3.15 kharavasānayorvisarjanīyaḥ");
             changed = true;
        }
    }
    changed
}
