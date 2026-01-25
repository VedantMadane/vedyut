use crate::prakriya::Prakriya;
use crate::tag::Tag;

/// 8.4.40 stoḥ ścunā ścuḥ
pub fn rule_8_4_40(p: &mut Prakriya) -> bool {
    let mut changed = false;
    let mut i = 0;
    while i < p.terms.len().saturating_sub(1) {
        let left_text = p.terms[i].text.clone();
        let right_text = p.terms[i+1].text.clone();

        let left_last = left_text.chars().last().unwrap();
        let right_first = right_text.chars().next().unwrap();

        // s/tu + sh/cu -> sh/cu
        // Sarkar/Tavarga + Sakar/Cavarga -> Sakar/Cavarga
        let t_varga = "tdn"; // Simplified check
        let c_varga = "cjY";

        if (left_last == 's' || t_varga.contains(left_last)) && (right_first == 'S' || c_varga.contains(right_first)) {
             let replacement = match left_last {
                 's' => 'S',
                 't' => 'c',
                 'd' => 'j',
                 'n' => 'Y',
                 _ => left_last,
             };

             if replacement != left_last {
                 let new_left = format!("{}{}", &left_text[..left_text.len()-left_last.len_utf8()], replacement);
                 p.terms[i].text = new_left;
                 p.add_rule("8.4.40 stoḥ ścunā ścuḥ");
                 changed = true;
             }
        }
        i += 1;
    }
    changed
}

/// 8.2.39 jhalāṃ jaśo'nte
pub fn rule_8_2_39(p: &mut Prakriya) -> bool {
    // Padanta Jhal -> Jas
    // k, kh, g, gh -> g
    // c, ch, j, jh -> j
    // t, th, d, dh -> d (retroflex)
    // t, th, d, dh -> d (dental)
    // p, ph, b, bh -> b
    let mut changed = false;
    for term in p.terms.iter_mut() {
        if term.has_tag(Tag::Pada) {
            let last = term.text.chars().last().unwrap();
            let replacement = match last {
                'k' | 'K' | 'g' | 'G' => 'g',
                'c' | 'C' | 'j' | 'J' => 'j',
                'w' | 'W' | 'q' | 'Q' => 'q',
                't' | 'T' | 'd' | 'D' => 'd',
                'p' | 'P' | 'b' | 'B' => 'b',
                _ => last,
            };

            if replacement != last {
                term.text.pop();
                term.text.push(replacement);
                // Note: Rule ID is actually handled by caller mostly, but here we can add
                // Ideally `p.add_rule` needs `&mut self`
                // We're iterating mutable references, so `p` is borrowed.
                // We can't call `p.add_rule` inside this loop directly easily without index iteration.
                // So we'll skip adding history here for now or switch to index loop.
                changed = true;
            }
        }
    }
    if changed {
        // p.add_rule("8.2.39 jhalāṃ jaśo'nte");
        // Can't add rule cleanly here without refactor, but operation done.
    }
    changed
}

// Re-implement rule_8_2_39 with index to allow rule logging
pub fn rule_8_2_39_indexed(p: &mut Prakriya) -> bool {
    let mut changed = false;
    for i in 0..p.terms.len() {
        if p.terms[i].has_tag(Tag::Pada) {
            let last = p.terms[i].text.chars().last().unwrap();
            let replacement = match last {
                'k' | 'K' | 'g' | 'G' => 'g',
                'c' | 'C' | 'j' | 'J' => 'j',
                'w' | 'W' | 'q' | 'Q' => 'q',
                't' | 'T' | 'd' | 'D' => 'd',
                'p' | 'P' | 'b' | 'B' => 'b',
                _ => last,
            };

            if replacement != last {
                p.terms[i].text.pop();
                p.terms[i].text.push(replacement);
                p.add_rule("8.2.39 jhalāṃ jaśo'nte");
                changed = true;
            }
        }
    }
    changed
}
