use crate::tag::Tag;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    pub text: String,
    pub tags: HashSet<Tag>,
}

impl Term {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            tags: HashSet::new(),
        }
    }

    pub fn make(text: &str, tag: Tag) -> Self {
        let mut t = Self::new(text);
        t.add_tag(tag);
        t
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(&tag)
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
