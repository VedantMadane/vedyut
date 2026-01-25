use crate::term::Term;
use crate::tag::Tag;

#[derive(Debug, Clone)]
pub struct Prakriya {
    pub terms: Vec<Term>,
    pub history: Vec<String>,
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
        self.history.push(rule.to_string());
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

    pub fn insert_after(&mut self, index: usize, term: Term) {
        if index + 1 <= self.terms.len() {
            self.terms.insert(index + 1, term);
        }
    }
}
