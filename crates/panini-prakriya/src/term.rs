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
    /// The dhātu belongs to adādi (gaṇa 2), the aluk gaṇa. Read by 2.4.72,
    /// which luks the śap that 3.1.68 inserts. Mirrors Divadi/Tudadi.
    Adadi,
    /// The dhātu belongs to kryādi (gaṇa 9), whose vikaraṇa is śnā. Read by
    /// 3.1.81 alone. Mirrors Divadi/Tudadi/Adadi.
    Kryadi,
    /// The dhātu belongs to svādi (gaṇa 5), whose vikaraṇa is śnu. Read by
    /// 3.1.73 alone. Mirrors Divadi/Tudadi/Adadi/Kryadi.
    Svadi,
    /// The dhātu belongs to rudhādi (gaṇa 7), whose vikaraṇa is śnam. Read
    /// by 3.1.78 alone. Mirrors Divadi/Tudadi/Adadi/Kryadi/Svadi.
    Rudhadi,
    /// The vikaraṇa (at `SHAP`) is a-final: śap (3.1.68), śyan (3.1.69) or
    /// śa (3.1.77), after their own it-lopa leaves them "a"/"ya"/"a". Set at
    /// the same three insertion points, cleared by 2.4.72 when it luks śap
    /// (the tag tracks SHAP's live shape, not śap's grammatical identity —
    /// `Tag::Vikarana` covers that), and read in `adesha.rs` wherever the
    /// grammar's real question is "is the vikaraṇa thematic śap" rather than
    /// "does SHAP's text currently end in `a`" — the two agree for śap/śyan/śa
    /// but not for rudhādi's śnam, whose infix split (3.1.78) leaves SHAP as
    /// `"na"`, an a-final string produced by a non-thematic vikaraṇa.
    Thematic,
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
    pub fn remove(&mut self, tag: Tag) {
        self.tags.remove(&tag);
    }
}
