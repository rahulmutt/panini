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
    /// The term at `SHAP` IS one of the four a-final vikaraṇas — śap
    /// (3.1.68), śyan (3.1.69), śa (3.1.77) or śānac (3.1.83) — each
    /// a-final once its own it-lopa runs ("a"/"ya"/"a"/"Ana"). This is an
    /// IDENTITY tag, not a live-shape one: it answers "is the term
    /// currently occupying SHAP one of these four vikaraṇas", not "does
    /// SHAP's text happen to end in `a` right now". The two questions
    /// coincide at the moment each of the four is inserted, and still
    /// coincide at every point this tag is read as a PATH GUARD (deciding
    /// which of a rule's arms applies) — but they are not the same
    /// question, and a rule whose job is vowel SANDHI on SHAP's own `a`
    /// must keep testing the text directly, not this tag: 7.3.101
    /// (`super::guna`) guards on `SHAP.text.ends_with('a')` for exactly
    /// that reason. 6.1.101, 6.1.97 and 6.1.87 (`super::adesha`) guard on
    /// this tag (a path decision) and THEN mutate SHAP's last character —
    /// that mutation runs only once the tag has already confirmed the
    /// path, and is safe because no rule between vikaraṇa insertion and
    /// any of these reaches the same SHAP with a shape already drifted
    /// from `a`-final (7.3.101 is disjoint with all of them on the
    /// ending's leading sound). Giving a rule like 7.3.101 this tag
    /// instead of the text test would be wrong the moment a prior rule has
    /// rewritten SHAP's ending away from `a`.
    ///
    /// Set at the four insertion points above. Cleared by 2.4.72, which
    /// luks śap for adādi: luk removes the vikaraṇa itself (1.1.61
    /// pratyayasya lopa ādarśanam), so the term is no longer śap at all —
    /// an identity fact, not a shape one; `Tag::Vikarana` still marks that
    /// a vikaraṇa-shaped term occupies the slot. NOT cleared or re-set by
    /// any rule that only rewrites SHAP's text (7.3.101, 6.1.101's bhvādi
    /// arm, 6.1.97, 6.1.87): those rules change what SHAP spells, never
    /// which vikaraṇa is there, so the identity this tag tracks is
    /// unaffected and doing so would be witness-free dead code.
    ///
    /// Read in `adesha.rs` (6.1.101's two arms, 6.1.97, 6.1.87, 6.1.66's
    /// athematic arm, 6.4.105) wherever the grammar's real question is "is
    /// the vikaraṇa one of the thematic four" — which is what distinguishes
    /// rudhādi's śnam from them: śnam is not in the four, even though its
    /// infix split (3.1.78) can leave SHAP `a`-final too, e.g. `"na"` for a
    /// vowel-final root.
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
