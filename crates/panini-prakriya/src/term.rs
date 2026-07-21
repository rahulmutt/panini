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
    /// The dhatu takes atmanepada (the data-layer stand-in for the anudatta
    /// it-marker that 1.3.12 reads; see the spec's pada-sanction section).
    Atmanepadin,
    /// The term behaves as Nit (set by the atidesha 1.2.4 sarvadhatukam apit;
    /// consumed by 7.2.81 Ato NitaH).
    Ngit,
    /// The pratyaya carries the p-anubandha (pit). Set on śap by 3.1.68; the
    /// second 1.2.4 application reads it to leave śap alone (only apit
    /// vikaraṇas — śyan, śa — become ṅit).
    Pit,
    /// The dhātu belongs to divādi (gaṇa 4) / tudādi (gaṇa 6). Data-layer
    /// stand-ins mirroring Atmanepadin, read by 3.1.69 / 3.1.77. bhvādi
    /// carries neither tag.
    Divadi,
    Tudadi,
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
