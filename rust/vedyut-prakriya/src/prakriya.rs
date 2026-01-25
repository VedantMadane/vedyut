use crate::term::Term;
use crate::tag::Tag;

#[derive(Debug, Clone)]
pub struct Step {
    pub rule: String,
    pub result: String,
}

#[derive(Debug, Clone)]
pub struct Prakriya {
    pub terms: Vec<Term>,
    pub history: Vec<Step>,
}

impl Prakriya {
    pub fn new() -> Self {
        Self {
            terms: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn with_terms(terms: Vec<Term>) -> Self {
        Self {
            terms,
            history: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: &str) {
        self.history.push(Step {
            rule: rule.to_string(),
            result: self.get_text(),
        });
    }

    pub fn get_text(&self) -> String {
        self.terms.iter().map(|t| t.text.clone()).collect::<Vec<_>>().join("")
    }

    pub fn find_first(&self, tag: Tag) -> Option<usize> {
        self.terms.iter().position(|t| t.has_tag(tag))
    }

    pub fn find_last(&self, tag: Tag) -> Option<usize> {
        self.terms.iter().rposition(|t| t.has_tag(tag))
    }

    pub fn get(&self, index: usize) -> Option<&Term> {
        self.terms.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Term> {
        self.terms.get_mut(index)
    }

    pub fn has(&self, index: usize, text_pattern: Option<&str>, tag_pattern: Option<Tag>) -> bool {
        if let Some(term) = self.terms.get(index) {
            let text_match = match text_pattern {
                Some(p) => term.text == p,
                None => true,
            };
            let tag_match = match tag_pattern {
                Some(t) => term.has_tag(t),
                None => true,
            };
            text_match && tag_match
        } else {
            false
        }
    }

    pub fn insert_after(&mut self, index: usize, term: Term) {
        if index < self.terms.len() {
            self.terms.insert(index + 1, term);
        } else if index == self.terms.len() {
             self.terms.push(term);
        }
    }

    pub fn set(&mut self, index: usize, text: &str) {
        if let Some(term) = self.terms.get_mut(index) {
            term.set_text(text);
        }
    }
}
