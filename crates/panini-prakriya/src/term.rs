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
    /// The dhatu is ubhayapadi: it takes EITHER pada, so both cells derive.
    ///
    /// Read only by 1.3.72 svaritaYitaH, and by 1.3.78's atmanepada arm,
    /// which declines rather than blocks when it is present.
    ///
    /// The tag is deliberately NOT named for 1.3.72's condition. It means:
    /// **1.3.72's condition holds AND 1.3.12's does not** -- the residue
    /// after 1.3.12, which is what the data layer's
    /// PadaAssignment::Ubhayapada stores.
    ///
    /// The counterexample that forces the distinction is Vindh: its upadesha
    /// `YiinDI~\` carries a Yi, and 1.3.72 reads Yit, so a marker-named tag
    /// (Svaritanit or similar) would have to be TRUE on Vindh -- and Vindh
    /// would grow a parasmaipada column it must not have. It must never
    /// reach 1.3.72 at all, because the anudatta `~\` on top of the Yi
    /// settles its pada by 1.3.12, and vidyut-prakriya derives it
    /// atmanepada-only. This tag is false on Vindh, which is the point.
    /// Pinned by `indh_is_atmanepada_only_despite_its_nit`.
    Ubhayapadin,
    /// The dhatu is the root 1.3.66 bhujo'navane names -- today exactly
    /// √bhuj (`07.0017`). Its ātmanepada is sanctioned by that root-keyed
    /// sūtra (in senses other than protecting, a restriction recorded but
    /// not modelled -- see the 1.3.66 comment block in `tinanta/samjna.rs`),
    /// its parasmaipada by 1.3.78's śeṣa.
    ///
    /// Deliberately distinct from Ubhayapadin even though the derivational
    /// behavior is identical: the tag keys the TRACE. Tagged Ubhayapadin,
    /// √bhuj would reach 1.3.72 and the trace would credit the wrong sūtra
    /// -- the mirror image of the √indh counterexample above, where a
    /// marker-named tag would have fired 1.3.72 on a root whose pada
    /// 1.3.12 had already settled. Read only by 1.3.66, and by 1.3.78's
    /// ātmanepada arm, which declines rather than blocks when it is
    /// present. Pinned by `svaritanit_declines_the_root_1_3_66_names`.
    Anavane,
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
    /// coincide at the moment each of the four is inserted, but they do
    /// NOT stay coincident, and this tag is read as a PATH GUARD at points
    /// where they have already diverged. √bhū loṭ uttama eka is the live
    /// case: 6.1.101's bhvādi arm rewrites `Bav + a + Ani` to
    /// `Bav + A + ni` (BavAni), and 6.1.96 and 6.1.90 both decline, so
    /// when 6.1.97 evaluates this tag — its `||` puts the tag first, ahead
    /// of the guṇa test — SHAP is `"A"`: thematic by identity, no longer
    /// `a`-final by shape. 7.3.101 (`super::guna`), which lengthens śap's
    /// `a` before an m/v-initial ending, drifts it the same way one stage
    /// earlier.
    ///
    /// A rule whose job is vowel SANDHI on SHAP's own `a` must therefore
    /// keep testing the text directly, not this tag: 7.3.101 guards on
    /// `SHAP.text.ends_with('a')` for exactly that reason. 6.1.101, 6.1.97
    /// and 6.1.87 (`super::adesha`) guard on this tag (a path decision)
    /// and THEN mutate SHAP's last character. That mutation is safe not
    /// because identity and shape coincide there — as BavAni shows, they
    /// need not — but because each of these rules ALSO tests the ENDING:
    /// 6.1.101 needs an `A`-initial ending, 6.1.97 an `a`/`e`/`o`-initial
    /// one, 6.1.87 an `i`/`I`-initial one, 6.4.105 the exact text `hi`.
    /// Whatever drifts SHAP has already consumed the ending's leading
    /// vowel, and what is left satisfies none of the later tests — after
    /// 6.1.101's bhvādi arm the ending is `ni`/`va`/`ma`/`vahE`/`mahE` (or,
    /// ātmanepada, the `E` that 6.1.90 absorbs before 6.1.97 looks), and after 7.3.101
    /// it begins `m`/`v`. So the pop-and-push only ever runs on a SHAP
    /// that is still `a`-final. Giving a rule like 7.3.101 this tag
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
