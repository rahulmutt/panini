use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    Dhatu,
    Pratyaya,
    Anga,
    Vikarana,
    Tin,
    Sarvadhatuka,
    Ardhadhatuka,
    It,
    Abhyasa,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub text: String,
    pub tags: HashSet<Tag>,
}

impl Term {
    pub fn new(text: &str) -> Term {
        Term {
            text: text.to_string(),
            tags: HashSet::new(),
        }
    }
    pub fn has(&self, tag: Tag) -> bool {
        self.tags.contains(&tag)
    }
    pub fn add(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }
}
