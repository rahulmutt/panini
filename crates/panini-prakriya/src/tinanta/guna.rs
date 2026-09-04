//! Vowel gradation and vikaraṇa reshaping: 7.4.21 … 6.4.113.
//!
//! Split out of `anga.rs` (which had reached 1110 lines) ahead of svādi.
//! The cut falls after 7.2.81: `anga.rs` keeps the augments and the rules
//! that reshape the *ending*, this file takes the rules that reshape a
//! *vowel* — the aṅga's, or the vikaraṇa's.
//!
//! Ordered AFTER 3.1.68 — ending at `ENDING`, śap at `SHAP`, and
//! `terms[SHAP].text` may be empty (2.4.72). See `super::terms`.
//!
//! 6.1.78 *eco'yavāyāvaḥ* sits in this stage rather than with the other
//! 6.1.x rules in `super::adesha` because that is where the pipeline order
//! puts it, between 7.3.86 and 7.3.101. Order outranks sūtra family: the
//! flattened sequence is the grammar.

use crate::rule::{Rule, RuleKind};
use crate::term::Tag;
use crate::tinanta::sound::{guna_of, is_vowel};
use crate::tinanta::terms::{ANGA, ENDING, SHAP, following_sarvadhatuka, vikarana_u_asamyogapurva};
use panini_data::Lakara;

pub(crate) static GUNA: &[Rule] = &[
    // 7.4.21 śīṅaḥ sārvadhātuke guṇaḥ: √śī takes guṇa (SI → Se) before a
    // sārvadhātuka ending, overriding the 1.1.5 block that the ṅit ātmanepada
    // endings would otherwise impose. This is the entire reason *śete* exists:
    // with śap luk'd (2.4.72) every other adādi root either has no ik to
    // guṇate (yā/vā/ās) or is consonant-final (ad/vas), so the gaṇa would show
    // no guṇa at all.
    //
    // Ordered immediately before 7.3.84, and now genuinely its apavāda: on
    // this śap-luk'd path the ṅit ātmanepada ending IS the immediate follower
    // (see `following_sarvadhatuka`), so 1.1.5 really does block 7.3.84 here
    // and 7.4.21 is the targeted override that licenses *śete*. The ordering
    // additionally covers the loṭ-uttama cells, whose endings 1.2.4's first
    // application deliberately leaves untagged: there nothing blocks 7.3.84,
    // but 7.4.21 has already reshaped the aṅga to `Se`, on which 7.3.84
    // declines by its own shape guard (`guna_of('e')` is `None`). Either way
    // the trace credits the guṇa to the sūtra that licenses it.
    //
    // The guard is the single `ends_with("SI")` test, deliberately with no
    // Tag::Adadi clause: √śī is the only SI-final root, so a gaṇa clause would
    // be redundant AND unkillable under mutation (with the clause dropped, the
    // other adādi roots still change nothing — guna_of returns None for their
    // `d`/`A`/`s` finals). `ends_with` rather than `==` because 6.4.71 has
    // already prefixed the laṅ aṭ-augment onto the aṅga (aSI) by this point.
    //
    // The sūtra's *sārvadhātuke* condition is structurally satisfied, not
    // guarded: every tiṅ ending in scope is tagged Sarvadhatuka when it is
    // introduced (3.4.78 / 3.4.113), across all four lakāras, so a guard
    // clause would be always-true — the same reason 7.3.84 omits it. It must
    // become a real guard the moment an ārdhadhātuka affix enters scope.
    Rule {
        id: "7.4.21",
        name: "SINaH sArvaDAtuke guRaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].text.ends_with("SI") {
                return false;
            }
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            let last = s.pop().expect("ends_with(\"SI\") implies a final char");
            let Some(g) = guna_of(last) else {
                return false;
            };
            let before = p.snapshot();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + g;
            p.record("7.4.21", "SINaH sArvaDAtuke guRaH", before);
            true
        },
    },
    // 7.3.84 sārvadhātukārdhadhātukayoḥ: guṇa of the aṅga's final ik.
    Rule {
        id: "7.3.84",
        name: "sArvaDAtukArDaDAtukayoH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            // 1.1.5 kṅiti ca: a following ṅit sārvadhātuka blocks guṇa. On
            // the thematic path that follower is the vikaraṇa, ṅit (1.2.4)
            // exactly when apit (śyan, śa); śap is pit and is not, so bhvādi
            // guṇa is unaffected. On the śap-luk'd path it is the ending —
            // see `following_sarvadhatuka`. Narrowness: the sūtra is *kṅiti*,
            // ṅit OR kit; this engine has no kit tag because no implemented
            // rule assigns or consumes one. Widen this test the moment a kit
            // sārvadhātuka enters scope.
            if following_sarvadhatuka(p).is_some_and(|t| t.has(Tag::Ngit)) {
                return false;
            }
            let last = p.terms[ANGA].text.chars().last().unwrap();
            let Some(g) = guna_of(last) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            s.pop();
            p.terms[ANGA].text = s.into_iter().collect::<String>() + g;
            p.record("7.3.84", "sArvaDAtukArDaDAtukayoH", before);
            true
        },
    },
    // 7.3.86 pugantalaghūpadhasya ca: guṇa of a light (short, pre-single-
    // consonant) penultimate ik before the sārvadhātuka. vft → vart. The
    // only curated root with an ik upadhā; final-ik roots (BU, smf…) are
    // 7.3.84's business and never reach this shape guard.
    Rule {
        id: "7.3.86",
        name: "pugantalaGUpaDasya ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            // 1.1.5 kṅiti ca, exactly as at 7.3.84 above — same follower
            // lookup, same ṅit-only narrowness.
            if following_sarvadhatuka(p).is_some_and(|t| t.has(Tag::Ngit)) {
                return false;
            }
            if p.terms[ANGA].has(Tag::Tanadi) {
                // Gaṇa 8 is the vikalpa arm's below (Kaumudī 2547.1).
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || is_vowel(chars[n - 1]) {
                return false; // final-vowel aṅgas are 7.3.84's business
            }
            let Some(g) = guna_of(chars[n - 2]) else {
                return false;
            };
            // laghu: the short ik vowels are exactly the lowercase ones our
            // guna_of accepts; long variants are guru and out of scope here.
            if !matches!(chars[n - 2], 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let before = p.snapshot();
            let mut s: String = chars[..n - 2].iter().collect();
            s.push_str(g);
            s.push(chars[n - 1]);
            p.terms[ANGA].text = s;
            p.record("7.3.86", "pugantalaGUpaDasya ca", before);
            true
        },
    },
    // 7.3.86 pugantalaghūpadhasya ca — VIKALPA ARM, gaṇa 8 only. The four
    // tanādi roots whose laghu upadhā is an ik guṇate OPTIONALLY before
    // the vikaraṇa `u`: kziRoti/kzeRoti, fRoti/arRoti, tfRoti/tarRoti,
    // GfRoti/GarRoti. The optionality is not the sūtra's own: it is the
    // tanādi gaṇasūtra the Siddhānta-kaumudī carries (vidyut-prakriya
    // applies it at Kaumudī 2547.1, an optional guṇa-apavāda tag on
    // exactly those four upadeśas). This engine keeps the Pāṇinian id on
    // the branch that applies guṇa and records the Kaumudī source here,
    // so ALTERNATES keys stay inside the Aṣṭādhyāyī.
    //
    // Guarded structurally — gaṇa 8, an ik upadhā, the `u` still standing
    // — not by a root list: within gaṇa 8 that selects exactly the
    // gaṇasūtra's four (a-upadhā roots have nothing to guṇate; √kṛ's ik is
    // FINAL, 7.3.84's business). The nitya entry declines the gaṇa on the
    // same tag, so the two entries partition and can never double-apply.
    //
    // NO 1.1.5 test, deliberately: the trigger is the ārdhadhātuka `u`
    // (never ṅit — see 3.1.79), not the tiṅ ending, which is why the guṇa
    // branch exists even before ṅit endings (tarRvanti). This is the
    // hardcoded-follower lesson of the adādi slices applied in advance.
    Rule {
        id: "7.3.86",
        name: "pugantalaGUpaDasya ca",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tanadi) {
                return false;
            }
            if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("u") {
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || is_vowel(chars[n - 1]) {
                return false;
            }
            if !matches!(chars[n - 2], 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let Some(g) = guna_of(chars[n - 2]) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: String = chars[..n - 2].iter().collect();
            s.push_str(g);
            s.push(chars[n - 1]);
            p.terms[ANGA].text = s;
            p.record("7.3.86", "pugantalaGUpaDasya ca", before);
            true
        },
    },
    // 7.3.92 tṛṇaha im: √tṛh takes the *im* āgama before a hal-initial pit
    // sārvadhātuka. tfnah + ti → tfnaih, which 6.1.87 ād guṇaḥ (in
    // `super::adesha`) then coalesces to tfneh → tfReQi.
    //
    // The āgama is **mit**, so 1.1.47 mid aco'ntyāt paraḥ places it after
    // the last vowel of what it attaches to. 1.1.47 is cited here, not
    // implemented as its own Rule — the treatment 3.1.78 already gives
    // śnam, and 1.4.13 and 1.1.5 get elsewhere. This is the engine's first
    // ĀGAMA placed that way; the placement itself is not new.
    //
    // REPRESENTATION. The aṅga is `tfnah`, but 3.1.78 splits the rudhādi
    // stem across ANGA and SHAP, so it is held as [tf, nah]. The last vowel
    // of the COMBINED stem is śnam's own `a`, which lives in SHAP — so both
    // this insertion and 6.1.87's coalescence are SHAP-internal and neither
    // touches ANGA. The guard reads the combined text rather than the two
    // slots separately on purpose: the split is an implementation artifact,
    // `tfnah` is what the sūtra names. `ends_with` rather than `==` because
    // 6.4.71 has already prefixed the laṅ aṭ-augment onto ANGA (atf) by
    // this point — the same allowance 7.4.21's guard makes.
    //
    // FOUR CONJUNCTS, but only THREE have a negative control among
    // √tṛh's own 36 golden cells:
    //   - the stem is tfnah        every other rudhādi root
    //   - hal-initial follower     `am` → atfRaham; loṭ uttama Ani/Ava/Ama
    //   - pit sārvadhātuka         NO CONTROL — see below
    //   - NOT ṅit                  tātaṅ (7.1.35) → tfRQAt; yāsuṭ → tfMhyAt
    //
    // EQUIVALENT IN THEORY, BUT NOT AS TESTED — corrected after the
    // rudhādi 7e mutation campaign (547 mutants, `-j 4 --timeout 4800`).
    // The theory still holds: literally removing the pit conjunct from the
    // guard (so the disjunction reads `Ngit || !Sarvadhatuka`, with no
    // reference to Pit at all) changes no derivation among all 238 tests.
    // Root cause is 1.2.4 sārvadhātukam apit (`samjna.rs`), which tags
    // EVERY apit sārvadhātuka ending ṅit; the one exception, loṭ uttama, is
    // vowel-initial and already excluded by the hal conjunct above. So for
    // everything that reaches this guard, !Pit implies Ngit, and the ṅit
    // check alone already rejects tas/Ta/vas — the pit conjunct's own
    // CONTRIBUTION to the disjunction is redundant.
    //
    // But that is not the mutation cargo-mutants actually generates here,
    // and the prior version of this comment was wrong to predict survival
    // on that basis. cargo-mutants does not synthesize a "delete this
    // disjunct" mutant for a `||` chain; at this guard it only generates
    // negation flips and `||`↔`&&` swaps, and the 7e campaign caught every
    // one of them:
    // all four on 7.3.92's guard, `!ending.has(Tag::Pit) ||
    // ending.has(Tag::Ngit) || !ending.has(Tag::Sarvadhatuka)`:
    //   delete ! on the Pit check           (flips it to `ending.has(Tag::Pit)`)
    //   replace the first || with &&
    //   replace the second || with &&
    //   delete ! on the Sarvadhatuka check  (flips it to `ending.has(Tag::Sarvadhatuka)`)
    // Flipping `!ending.has(Tag::Pit)` to `ending.has(Tag::Pit)` does not
    // disable the conjunct, it inverts it: the guard then rejects exactly
    // the four cells that must fire (the ones where Pit IS true), which is
    // caught, correctly. The redundancy argument above only licenses
    // removing the conjunct outright — a mutation this repo has never
    // actually observed cargo-mutants produce at this site. If a future
    // mutation campaign reports a new surviving mutant on this guard, do
    // NOT assume this is the same documented case without first checking
    // that it is, in fact, a delete-the-disjunct mutation and not a flip.
    //
    // Kept anyway: it states the sūtra's own condition (7.3.92 IS a pit
    // rule), and this repo prefers guards faithful to the grammar over
    // guards minimised against the current engine's incidental behavior.
    // The redundancy is a property of how 1.2.4 is implemented today, not
    // a theorem — if that tagging ever changes, the pit check is what
    // keeps 7.3.92 correct. Same reasoning that keeps `is_shtu`'s
    // unreachable `R` arm.
    //
    // The fourth conjunct (not ṅit) is genuinely distinct from the third
    // and DOES have its own control: under yāsuṭ the ending's own `t` is
    // still pit, and it is the ĀGAMA that carries the ṅ — pit alone would
    // wrongly admit that cell, and only the ṅit check rejects it.
    //
    // The sārvadhātuka clause is a real guard here, not a structural
    // always-true as at 7.3.84: it is read off the ending directly, and
    // costs nothing to state.
    Rule {
        id: "7.3.92",
        name: "tfRaha im",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            let stem = format!("{}{}", p.terms[ANGA].text, p.terms[SHAP].text);
            if !stem.ends_with("tfnah") {
                return false;
            }
            let ending = &p.terms[ENDING];
            if !ending.has(Tag::Pit) || ending.has(Tag::Ngit) || !ending.has(Tag::Sarvadhatuka) {
                return false;
            }
            // *hali*: the ending must lead with a consonant. "Not a vowel"
            // is exact here — every ending in scope is vowel- or
            // consonant-initial, with no third case.
            let Some(first) = ending.text.chars().next() else {
                return false;
            };
            if is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            let last = s
                .pop()
                .expect("ends_with(\"tfnah\") implies a non-empty SHAP");
            s.push('i');
            s.push(last);
            p.terms[SHAP].text = s.into_iter().collect();
            p.record("7.3.92", "tfRaha im", before);
            true
        },
    },
    // 7.3.84 sārvadhātukārdhadhātukayoḥ — SECOND APPLICATION, on the
    // vikaraṇa. This is not a duplicate: by 1.4.13 yasmāt pratyayavidhis
    // tadādi pratyaye'ṅgam the aṅga is defined relative to the affix, so
    // the sūtra has two occasions in a single derivation. With respect to
    // the vikaraṇa the aṅga is the root — that is the entry above. With
    // respect to the tiṅ ending the aṅga is root + vikaraṇa, and its final
    // ik belongs to the vikaraṇa. Ap + nu + ti → Ap + no + ti.
    //
    // The pipeline already carries two applications of 1.2.4 for exactly
    // this reason (ending, then vikaraṇa); this is the same shape. Both
    // entries appear in `tinanta_rule_order_is_pinned`, and tests locate
    // this one with `.filter(id == "7.3.84").nth(1)`. Do not "deduplicate".
    //
    // Reads terms[ENDING] directly rather than `following_sarvadhatuka`:
    // that helper answers "what follows the aṅga", which for this
    // application is the vikaraṇa being operated on, not the trigger.
    //
    // NO DELTA on any pre-existing form, by guard rather than by argument.
    // The complete inventory of SHAP texts reaching this point is `a`
    // (śap/śa), `ya` (śyan), `` (adādi luk), `Ana` (śānac), `nA`/`n` (śnā,
    // 6.4.112), `nI` (śnā, 6.4.113), `u` (tanādi, 3.1.79), and — for
    // rudhādi, where SHAP holds śnam followed by the root's own tail
    // (3.1.78) — `nat`, `nah`, `nans` and their kin, plus `naih` once
    // 7.3.92 above has put the im in. `nI` and `u` are ik-final (`u`'s the
    // whole point of 3.1.79's ārdhadhātuka status: tanoti's guṇa runs
    // here); every rudhādi shape is consonant-final, so `guna_of` returns
    // None for all of them. 6.4.113 produces `nI` ONLY before a ṅit
    // ending — so the 1.1.5 test below declines there. Two tests pin both
    // halves.
    //
    // Ordered BEFORE 6.1.78: the loṭ uttama endings are vowel-initial and
    // pit, so guṇa leaves `no`, which 6.1.78 must then make `nav`. Ordered
    // after it, ApnavAni surfaces as *ApnoAni. Ordered BEFORE 6.4.87/6.4.77
    // for the same cells: those fire on a vowel-initial ending too, and
    // would take `nu` to `nuv` first, giving *ApnuvAni.
    Rule {
        id: "7.3.84",
        name: "sArvaDAtukArDaDAtukayoH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            // 1.1.5 kṅiti ca, as in the first application. Same ṅit-only
            // narrowness: no kit tag exists in this engine yet.
            if p.terms[ENDING].has(Tag::Ngit) {
                return false;
            }
            let Some(last) = p.terms[SHAP].text.chars().last() else {
                return false;
            };
            let Some(g) = guna_of(last) else {
                return false;
            };
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            s.pop();
            p.terms[SHAP].text = s.into_iter().collect::<String>() + g;
            p.record("7.3.84", "sArvaDAtukArDaDAtukayoH", before);
            true
        },
    },
    // ------------------------------------------------------------------
    // The √kṛ specials, 6.4.108–110. They live HERE, not with their
    // 6.4.10x siblings in adesha.rs, because 6.4.110 must precede 6.1.77
    // below: kar + u + anti must become kur + u + anti before the u goes
    // to v (kurvanti), and stage files are an organisational boundary,
    // not a grammatical one — the flattened order is the grammar (the
    // same argument that put 7.3.92 in this file). All three are keyed to
    // √kṛ by 6.4.108's *karoteḥ*, carried by anuvṛtti into 109 and 110.
    //
    // The guard is the aṅga-text tail (`kar`/`kur`) plus the tanādi `u`
    // SHAP alone, with NO `Tag::Tanadi` clause: no other aṅga in the
    // curated corpus ends in kar/kur while carrying a `u` vikaraṇa (only
    // 3.1.79 ever produces one), so a gaṇa tag would be redundant AND
    // unkillable under mutation — the same reasoning 7.4.21's comment
    // gives above for its bare `SI` guard. `ends_with` rather than `==`,
    // also as 7.4.21 and 7.3.92 do above: 6.4.71 has already prefixed
    // laṅ's aṭ-augment onto the aṅga's own text (`tinanta/anga.rs`) by
    // this point, so a laṅ derivation reads `akar`/`akur`, not the bare
    // root — `==` would silently decline for the whole lakāra.
    //
    // 8.2.77 hali ca (`tinanta/tripadi.rs`) IS implemented in this engine
    // — it lengthens div's upadhā, dīvyati — and its shape guard matches
    // `kur` just as readily (short `u` upadhā, `r` final); left alone it
    // would derive *kUrvanti here. 8.2.79 na BakurCurAm is modelled as a
    // named exclusion guard inside 8.2.77's own `apply`, rather than a
    // separate `Rule` entry (this engine has no niṣedha `RuleKind`, and
    // the exclusion never touches a cell 8.2.77 wasn't already about to
    // mutate) — see that rule's comment for the full argument. The
    // resulting forms are byte-identical to vidyut-prakriya's, which
    // records 8.2.79 on every kur cell for the same reason.
    // ------------------------------------------------------------------
    // 6.4.110 ata ut sārvadhātuke (kṅiti, anuvṛtti from 6.4.98/6.4.108's
    // context): kar's `a` becomes `u` before a ṅit sārvadhātuka —
    // kurutaH, kurvanti, kurute, and (via 6.4.106 next) kuru. Before pit
    // endings it declines and 7.3.84's guṇa run stands: karoti, karavAni.
    Rule {
        id: "6.4.110",
        name: "ata ut sArvaDAtuke",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].text.ends_with("kar") {
                return false;
            }
            if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("u") {
                return false;
            }
            if !p.terms[ENDING].has(Tag::Ngit) {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = s.len();
            s[n - 2] = 'u';
            p.terms[ANGA].text = s.into_iter().collect();
            p.record("6.4.110", "ata ut sArvaDAtuke", before);
            true
        },
    },
    // 6.4.108 nityaṁ karoteḥ: the lopa 6.4.107 makes optional is NITYA
    // for √kṛ before m/v — kurvaH, kurmaH, with no alternate. Ordered
    // before 6.4.107 (adesha.rs) by stage order; once this empties the
    // u, 6.4.107's helper declines on the empty text, so the vikalpa
    // machinery never sees √kṛ — the self-guarding 6.4.87/6.4.77 use.
    Rule {
        id: "6.4.108",
        name: "nityaM karoteH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].text.ends_with("kur") {
                return false;
            }
            if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("u") {
                return false;
            }
            if !p.terms[ENDING].text.starts_with(['m', 'v']) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.record("6.4.108", "nityaM karoteH", before);
            true
        },
    },
    // 6.4.109 ye ca: the same lopa before y — kuryAt and the rest of
    // vidhiliṅ parasmaipada (the ending term reads `yAt`/`yAtAm`/… here:
    // anga.rs has already fused yAsuṭ into it).
    Rule {
        id: "6.4.109",
        name: "ye ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].text.ends_with("kur") {
                return false;
            }
            if p.terms.get(SHAP).map(|t| t.text.as_str()) != Some("u") {
                return false;
            }
            if !p.terms[ENDING].text.starts_with('y') {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.record("6.4.109", "ye ca", before);
            true
        },
    },
    // 6.4.87 huśnuvoḥ sārvadhātuke: for √hu and śnu, before a sārvadhātuka,
    // yaṇ — `u` → `v` — rather than 6.4.77's uvaṅ. hi + nu + anti →
    // hinvanti; ri + nu + antu → riRvantu (ṇatva lands later, in tripadi).
    //
    // The *asaṁyogapūrva* restriction is anuvṛtti from 6.4.82 er anekāco'-
    // saṁyogapūrvasya; it is not visible in this sūtra's own words, which
    // is why the guard would otherwise look invented. It is what separates
    // hinvanti from Apnuvanti.
    //
    // The √hu arm is not implemented: √hu is juhotyādi, out of scope. Widen
    // when gaṇa 3 lands.
    //
    // APAVĀDA to 6.4.77 below, and ordered before it as the pipeline's other
    // apavāda pairs are (3.1.69 before 3.1.68; 6.4.72 before 6.4.71). It
    // self-guards: once this rule has written `nv`, 6.4.77's `nu` test no
    // longer matches, so no "did the apavāda fire?" check is needed.
    Rule {
        id: "6.4.87",
        name: "huSnuvoH sArvaDAtuke",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            // The sūtra names hu and śnu. Tanādi's bare `u` (which the
            // shared asaṁyogapūrva helper now also admits) is 6.1.77's
            // business below — without this test 6.4.87 would write śnu's
            // `nv` over a vikaraṇa that has no `n`.
            if p.terms[SHAP].text != "nu" {
                return false;
            }
            if !vikarana_u_asamyogapurva(p) {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "nv".into();
            p.record("6.4.87", "huSnuvoH sArvaDAtuke", before);
            true
        },
    },
    // 6.4.77 aci śnudhātubhruvāṁ yvor iyaṅuvaṅau: before a vowel, śnu's `u`
    // becomes uvaṅ. Ap + nu + anti → Apnuvanti; aS + nu + ate → aSnuvate;
    // aS + nu + Iyta → aSnuvIta (6.1.66 drops the y later, in `adesha`).
    //
    // Only the śnu arm is implemented. The *dhātu* arm (ī/ū-final roots) and
    // the *bhrū* arm have no root in scope — recorded rather than written,
    // as 6.4.112's *abhyasta* half and 6.4.113's *aghoḥ* are. Widen when a
    // root reaches either.
    //
    // Reads terms[ENDING] directly, NOT `following_sarvadhatuka`: that
    // helper answers "what follows the aṅga", which here is śnu itself —
    // this rule needs what follows śnu. Same reasoning as 6.4.112/6.4.113.
    Rule {
        id: "6.4.77",
        name: "aci SnuDAtuBruvAM yvoriyaNuvaNO",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms[SHAP].text != "nu" {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "nuv".into();
            p.record("6.4.77", "aci SnuDAtuBruvAM yvoriyaNuvaNO", before);
            true
        },
    },
    // 6.1.77 iko yaṇ aci: the tanādi vikaraṇa's `u` becomes `v` before a
    // vowel-initial ending. tan + u + anti → tanvanti; tan + u + ate →
    // tanvate; tan + u + Ita → tanvIta. This is the utsarga whose apavādas
    // the pipeline already carries for śnu — 6.4.87 (yaṇ, now self-guarded
    // to `nu`) and 6.4.77 (uvaṅ) — ordered above it as apavādas are
    // elsewhere; neither can contend here, since both test śnu's text and
    // this rule tests the bare `u`. vidyut-prakriya credits exactly this
    // sūtra for these cells.
    //
    // Only the vikaraṇa arm is written: no other ik-vowel hiatus survives
    // to this point in the pipeline, the same narrowness 6.1.78's three
    // arms and 6.4.77's śnu-only arm document. Widen by arm, with a
    // witness, when a root needs one.
    //
    // Ordered AFTER 7.3.84's second application: the loṭ uttama endings
    // are vowel-initial and pit, so guṇa takes `u` → `o` first and 6.1.78
    // then yields tanavAni — this rule's `u` test declines on the `o`, the
    // same self-guarding 6.4.87/6.4.77 rely on for ApnavAni.
    Rule {
        id: "6.1.77",
        name: "iko yaR aci",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms[SHAP].text != "u" || !p.terms[SHAP].has(Tag::Vikarana) {
                return false;
            }
            let Some(next) = p.terms.get(ENDING).and_then(|t| t.text.chars().next()) else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "v".into();
            p.record("6.1.77", "iko yaR aci", before);
            true
        },
    },
    // 6.1.78 eco'yavāyāvaḥ: e/o before a vowel → ay/av. The sūtra also covers
    // E/O → Ay/Av, but those two arms are dropped here: within the current
    // 49-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)
    // at the point this rule runs. `vrddhi_of` (the only source of E/O in
    // this engine) is called from three places, all in 6.1.90 — the aṅga arm
    // writes the vṛddhi vowel at *position 0* of the aṅga (replacing the āṭ
    // augment + the root's first vowel), never at the aṅga's last character;
    // the other two arms write into SHAP/ENDING, not ANGA. No curated root is
    // a single SLP1 character, so the aṅga arm's tail slice is never empty
    // either. And the order is decisive on its own: 6.1.90 is the only caller
    // of `vrddhi_of`, and it runs *after* 6.1.78 in the single-pass rule
    // array, so any E/O it produces can never be seen by 6.1.78 at all.
    // Unexecutable arms cannot be kept under the mutation gate — the same
    // discipline that removed 8.4.53 in `super::tripadi` as unreachable in
    // `9fa8e5f` (it was later RESTORED, once rudhādi supplied a witness —
    // see 8.4.53's own comment — so that removal is precedent for the
    // discipline, not a standing state of the code).
    // Restore the E/O arms (and re-add their coverage in the golden/mutation
    // suites) the moment a root lands whose aṅga can end in a vṛddhi vowel
    // before a vowel-initial ending. √śī (slice 5f) is NOT that root: 7.4.21
    // gives it guṇa (Se), never vṛddhi, and its `e` arm below is what carries
    // SayAte / SayIta / SayE. The trigger is a root that takes vṛddhi at the
    // aṅga-final position. Reaching the `e` arm for adādi at all depends on
    // the athematic follower lookup this slice added below (SHAP is luk'd
    // for adādi, so the arm falls back to ENDING's first character).
    Rule {
        id: "6.1.78",
        name: "eco'yavAyAvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            fn sub_for(c: char) -> Option<&'static str> {
                match c {
                    'e' => Some("ay"),
                    'o' => Some("av"),
                    _ => None,
                }
            }

            // The two arms below operate on the aṅga's own final ec. They
            // are reached only when the root has one; svādi's roots never
            // do, which is why the vikaraṇa arm at the bottom exists.
            if let Some(anga_last) = p.terms[ANGA].text.chars().last()
                && let Some(sub) = sub_for(anga_last)
            {
                // Thematic arm: the vikaraṇa (śap/śyan/śa) is a real, non-empty
                // buffer between the aṅga and the ending, so its own first
                // character is the "next" vowel this sūtra tests. Only reachable
                // when that first character exists AND is a vowel — a non-empty,
                // consonant-initial vikaraṇa (śyan's `ya`) correctly declines
                // here rather than firing on the wrong segment.
                if let Some(next_first) = p.terms[SHAP].text.chars().next()
                    && is_vowel(next_first)
                {
                    let before = p.snapshot();
                    let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
                    s.pop();
                    p.terms[ANGA].text = s.into_iter().collect::<String>() + sub;
                    p.record("6.1.78", "eco'yavAyAvaH", before);
                    return true;
                }
                // Athematic arm (śap luk'd, adādi, 2.4.72): with no vikaraṇa
                // buffer, the ending attaches directly to the aṅga, so the
                // ending's own first character is the "next" vowel instead.
                // Guarded on the śap being EMPTY, so this can never re-process
                // the thematic path above — a non-empty, non-vowel-initial śap
                // (śyan's `ya`, which fails the thematic arm's vowel check)
                // must decline here too, not fall through to test the ending.
                // The two arms' guards (SHAP vowel-initial vs. SHAP empty) are
                // mutually exclusive by construction, so at most one ever fires.
                // `is_empty()` (not `!ends_with('a')`) is still the right test
                // here and stays adādi-only: kryādi never guṇates its aṅga (the
                // ṅit śnā blocks 7.3.84/7.3.86 via 1.1.5), so an `e`/`o`-final
                // aṅga — this rule's whole precondition — never arises for it.
                // √śī vidhiliṅ 3pl: guṇa (7.4.21) has already made the aṅga `Se`,
                // and 3.4.102/7.2.79 have left the ending leading with `I`
                // (Iyran, after sīyuṭ's salopa strips the non-final `s`); this
                // arm reads only that leading `I` and turns Se + Iyran →
                // Say + Iyran. 6.1.66 (`super::adesha`, later in the pipeline)
                // then elides the surviving `y` before the val `r` → SayIran.
                if p.terms.len() > ENDING
                    && p.terms[SHAP].text.is_empty()
                    && let Some(next_first) = p.terms[ENDING].text.chars().next()
                    && is_vowel(next_first)
                {
                    let before = p.snapshot();
                    let mut s: Vec<char> = p.terms[ANGA].text.chars().collect();
                    s.pop();
                    p.terms[ANGA].text = s.into_iter().collect::<String>() + sub;
                    p.record("6.1.78", "eco'yavAyAvaH", before);
                    return true;
                }
            }

            // Vikaraṇa arm (svādi and tanādi): 7.3.84's second application
            // has just guṇated śnu's `u` to `o`, so the ec this sūtra
            // converts sits on the VIKARAṆA, not on the aṅga — Ap + no +
            // Ani → Apnav + Ani. Tanādi's bare `u` (3.1.79) reaches the same
            // arm the same way: tan + o + Ani → tanav + Ani (`tanavAni`).
            // Mutually exclusive with both arms above: those require the
            // aṅga to end in e/o, which no svādi or tanādi root does, and
            // this one requires SHAP to end in e/o, which none of śap `a`,
            // śyan `ya`, śa `a`, śnā `nA`/`n`/`nI`, śānac `Ana` or adādi's
            // empty śap ever does — only svādi's guṇated `nu` and tanādi's
            // guṇated `u` ever reach it.
            if p.terms.len() > ENDING
                && let Some(shap_last) = p.terms[SHAP].text.chars().last()
                && let Some(sub) = sub_for(shap_last)
                && let Some(next_first) = p.terms[ENDING].text.chars().next()
                && is_vowel(next_first)
            {
                let before = p.snapshot();
                let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
                s.pop();
                p.terms[SHAP].text = s.into_iter().collect::<String>() + sub;
                p.record("6.1.78", "eco'yavAyAvaH", before);
                return true;
            }
            false
        },
    },
    // 7.3.101 ato dīrgho yañi: aṅga-final `a` (śap) → `A` before a yañ-initial
    // sārvadhātuka ending (here: mi/vas/mas).
    Rule {
        id: "7.3.101",
        name: "ato dIrGo yaYi",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            // loṭ uttama gets its dīrgha from 3.4.92 āḍ + 6.1.101 instead.
            if matches!(p.ctx.lakara, Lakara::Lot) {
                return false;
            }
            let ending_first = p.terms[ENDING].text.chars().next().unwrap();
            // Ends in `a`, not equal to `a`: śyan's residue is `ya`, not `a`
            // (see 6.1.97's comment for why only the final vowel matters).
            if !matches!(ending_first, 'm' | 'v') || !p.terms[SHAP].text.ends_with('a') {
                return false;
            }
            let before = p.snapshot();
            let mut s: Vec<char> = p.terms[SHAP].text.chars().collect();
            s.pop();
            s.push('A');
            p.terms[SHAP].text = s.into_iter().collect();
            p.record("7.3.101", "ato dIrGo yaYi", before);
            true
        },
    },
    // --- śnā's alternation (6.4.112, 6.4.113) -----------------------------
    //
    // Placed at the END of this stage, not in sūtra order. Three constraints
    // fix the position and each fails visibly if broken:
    //   - AFTER 7.1.3 jho'ntaḥ, which makes `Ji` into `anti`/`ant`. Before it,
    //     the 3pl endings are not vowel-initial and 6.4.112 cannot see them.
    //   - AFTER 7.2.79 liṅaḥ salopo'nantyasya. The ātmanepada vidhiliṅ ending
    //     is `sIyta` until its s is elided; run earlier and 6.4.113 matches
    //     the s, giving *vfRIsIyta.
    //   - BEFORE adesha.rs, whose 6.1.87 ād guṇaḥ would coalesce nA + Iyta
    //     into ne and give *vfReta. This stage runs entirely before that one.
    //
    // Both read p.terms[ENDING] directly, NOT following_sarvadhatuka: the
    // helper answers "what follows the aṅga", which here is śnā itself — these
    // rules need what follows śnā.

    // 6.4.112 śnābhyastayor ātaḥ: śnā's `ā` is elided before a kṅit
    // sārvadhātuka beginning with a vowel. kliS + nA + anti → kliSnanti;
    // vf + nA + ate → vfRate; vf + nA + e → vfRe.
    //
    // The *abhyasta* half of the sūtra is out of scope — there is no
    // reduplication in this engine — so the guard is śnā's text alone. Widen
    // it when juhotyādi lands.
    Rule {
        id: "6.4.112",
        name: "SnA'ByastayorAtaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[SHAP].text != "nA" {
                return false;
            }
            if !p.terms[ENDING].has(Tag::Ngit) {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if !is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "n".into();
            p.record("6.4.112", "SnA'ByastayorAtaH", before);
            true
        },
    },
    // 6.4.113 ī halyaghoḥ: śnā's `ā` becomes `ī` before a kṅit sārvadhātuka
    // beginning with a consonant. kliS + nA + taH → kliSnItaH; kliS + nA +
    // yAt → kliSnIyAt; vrI + nA + hi → vrIRIhi.
    //
    // *aghoḥ* excludes the ghu roots (√dā, √dhā). They are juhotyādi, out of
    // scope, and no root that can reach this rule is one — so the exclusion is
    // recorded here rather than implemented. Implement it when gaṇa 3 lands.
    Rule {
        id: "6.4.113",
        name: "I halyaGoH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[SHAP].text != "nA" {
                return false;
            }
            if !p.terms[ENDING].has(Tag::Ngit) {
                return false;
            }
            let Some(next) = p.terms[ENDING].text.chars().next() else {
                return false;
            };
            if is_vowel(next) {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = "nI".into();
            p.record("6.4.113", "I halyaGoH", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tinanta::form_g;
    use crate::tinanta::rules;
    use panini_data::{Purusha, Vacana};

    // --- 7.3.86 pugantalaGUpaDasya ca: guard-edge pins -------------------
    //
    // The guard `n < 2 || is_vowel(chars[n - 1])` followed by index
    // arithmetic on `chars[n - 2]` / `chars[..n - 2]` is reachable-
    // equivalent to its mutants for every curated aGga except the 3-char
    // "vft" (where n-2 == n/2), so each case below is a constructed
    // Prakriya chosen to separate the mutant from the original at a
    // different edge.

    #[test]
    fn pugantalaghupadhasya_one_char_anga_returns_false_without_panic() {
        // n=1: `n < 2` alone is true, so `||` short-circuits and the body
        // never touches `chars[n - 2]`. The `<` -> `==` mutant makes
        // `n == 2` false for n=1; evaluating the right disjunct then needs
        // `chars[n - 1]` (fine, n-1=0) but the guard as a whole is now
        // false, so the mutant falls through to `chars[n - 2]` with n=1,
        // a usize underflow that panics. The original must return false
        // cleanly.
        let mut p = Prakriya {
            terms: vec![Term::new("d"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.3.86").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "d");
    }

    #[test]
    fn pugantalaghupadhasya_two_char_ik_penult_fires() {
        // n=2, final char 'd' is a consonant so the guard is false and the
        // rule fires: guNa of penult 'i' is "e", giving "ed". The
        // `<` -> `<=` mutant makes `n <= 2` true for n=2, so the mutant
        // guard short-circuits to true and wrongly returns false instead
        // of firing.
        let mut p = Prakriya {
            terms: vec![Term::new("id"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.3.86").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ed");
    }

    #[test]
    fn pugantalaghupadhasya_skips_vowel_final_anga() {
        // n=3, final char 'u' is a vowel, so the guard's `is_vowel` disjunct
        // is true and the rule must not fire (this shape is 7.3.84's
        // business). The `||` -> `&&` mutant makes the guard
        // `n < 2 && is_vowel(...)` = false && true = false, so the mutant
        // falls through and wrongly fires on the ik penult 'f'.
        let mut p = Prakriya {
            terms: vec![Term::new("Bfu"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.3.86").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bfu");
    }

    #[test]
    fn pugantalaghupadhasya_uses_n_minus_2_not_n_over_2() {
        // n=5 ("aBiur"): n-2=3 (penult 'u') but n/2=2 (chars[2]='i') --
        // these differ, so this case separates both `-` -> `/` mutants
        // from the original at once. By hand: guNa of chars[3]='u' is
        // "o"; laghu-check on chars[3]='u' passes; prefix is chars[..3]
        // = "aBi"; result = "aBi" + "o" + chars[4]='r' = "aBior".
        // Mutating the `chars[n - 2]` guard to `chars[n / 2]` would guNa
        // 'i' instead ("e"), yielding "aBier". Mutating the `chars[..n - 2]`
        // slice to `chars[..n / 2]` would prefix with "aB" instead
        // of "aBi", yielding "aBor". Both diverge from "aBior".
        let mut p = Prakriya {
            terms: vec![Term::new("aBiur"), Term::new("a")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.3.86").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "aBior");
    }

    #[test]
    fn pugantalaghupadhasya_single_term_still_applies_guna() {
        // 7.3.86 shares 7.3.84's 1.1.5 guard verbatim: both call
        // `following_sarvadhatuka` and block only when it finds a Ngit
        // follower. Unlike 7.3.84 (unreachable for divAdi/tudAdi, whose
        // aGgas are all consonant-final), 7.3.86's Ngit-true branch IS
        // reached by the curated corpus (div, tud, juz, ...), so most of the
        // helper's behaviour is already exercised there. What survives here
        // is the "no follower at all" edge: with len == 1 (no vikaraNa
        // term), `following_sarvadhatuka`'s `p.terms.get(SHAP)` is already
        // None, so the match's `None => None` arm returns None without
        // indexing anything -- nothing blocks, and guNa proceeds normally:
        // vft -> vart. `.get()` cannot panic regardless of arity, unlike the
        // old `p.terms[SHAP]` index it replaced.
        let mut p = Prakriya {
            terms: vec![Term::new("vft")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.3.86").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "vart");
    }

    #[test]
    fn eco_yavayavah_athematic_arm_requires_a_third_term() {
        // 6.1.78's ATHEMATIC arm (śap luk'd) reads p.terms[ENDING] (index 2)
        // once its guard passes. With only two terms (aGga + an empty śap,
        // no ending inserted yet), `p.terms.len() > ENDING` (2 > 2) is
        // false, so the guard short-circuits before indexing terms[2]. The
        // `>` -> `>=` mutant makes `2 >= 2` true; since the śap here is
        // empty, the mutant guard proceeds and indexes terms[ENDING], out of
        // bounds for a 2-term vector -> panics. The aGga ("Se") satisfies
        // the rule's own e/o-final precondition, isolating the athematic
        // arm's own third-term guard.
        let mut p = Prakriya {
            terms: vec![Term::new("Se"), Term::new("")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.78").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Se");
    }

    #[test]
    fn eco_yavayavah_vikarana_arm_requires_a_third_term() {
        // 6.1.78's VIKARAṆA arm (svādi) reads p.terms[ENDING] (index 2) once
        // its guard passes. With only two terms (aṅga + a guṇated śnu, no
        // ending inserted yet), `p.terms.len() > ENDING` (2 > 2) is false,
        // so the guard short-circuits before indexing terms[2]. The
        // `>` -> `>=` mutant makes `2 >= 2` true; since the śap here ends in
        // `o` (guṇated śnu, as 7.3.84's second application leaves it), the
        // mutant guard proceeds and indexes terms[ENDING], out of bounds for
        // a 2-term vector -> panics. The aṅga ("Ap") does not end in e/o, so
        // the thematic/athematic arms above decline, isolating the vikaraṇa
        // arm's own third-term guard.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("no")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.78").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "no");
    }

    #[test]
    fn eco_yavayavah_athematic_arm_requires_an_empty_shap() {
        // The athematic arm must fire ONLY when the śap is luk'd (empty) —
        // that is what confines it to the adADi (athematic) path; on the
        // thematic path the vikaraṇa itself supplies the "next" vowel. Here
        // the śap is the non-empty, consonant-initial "ya" (śyan) and the
        // ending is "Iran" (vowel-initial): the thematic arm declines (its
        // own guard reads SHAP's first char, 'y', which is not a vowel), and
        // the athematic arm must ALSO decline — not fall through to test the
        // vowel-initial ending — because the śap is not empty, leaving
        // "Iran" untouched. The mutant that drops the empty-śap conjunct
        // would let the athematic arm fire regardless — reading the ending's
        // vowel-initial "I" — and wrongly turn the aṅga "Se" into "Say" even
        // though the śap is a real (non-empty) buffer, not the śap-luk'd
        // adādi path this arm is for.
        let mut p = Prakriya {
            terms: vec![Term::new("Se"), Term::new("ya"), Term::new("Iran")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.78").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Se");
        assert_eq!(p.terms[ENDING].text, "Iran");
    }

    // --- 7.3.84 sArvaDAtukArDaDAtukayoH: 1.1.5 guard pins ------------------
    //
    // No curated divAdi/tudAdi root has a vowel-final aGga (they are all
    // consonant-final: div, naS, kup, man, yuD, vid, tud, liK, viS, juz,
    // vij, gur all end in a consonant), so 7.3.84's guNa-blocking business
    // — final-ik aGgas — is only ever reached by bhvAdi roots (BU, nI, ji,
    // smf), whose vikaraNa (Sap) is never Ngit and whose Sap is always
    // non-empty, so `following_sarvadhatuka` never falls through to ENDING
    // for them either. The helper's Ngit-true branch is therefore never
    // exercised by any golden or negative derivation, and mutants on it --
    // the whole helper body replaced by `None`, or its
    // `!shap.text.is_empty()` guard flipped to `true` or `false` -- are
    // invisible to the suite. Pin both edges directly: a constructed
    // two-term prakriya whose SHAP itself carries Ngit, and a bare one-term
    // prakriya with no follower at all.
    #[test]
    fn sarvadhatukardhadhatukayoh_blocks_guna_when_vikarana_is_ngit() {
        // Constructed vowel-final aGga ("nI") + a Ngit vikaraNa (as Syan/Sa
        // would be via the second 1.2.4), with SHAP carrying non-empty
        // text: `following_sarvadhatuka` must take its `Some(shap) if
        // !shap.text.is_empty()` arm and return SHAP itself rather than
        // fall through to ENDING (there isn't one on this two-term
        // prakriya). GuNa must be blocked. The `!shap.text.is_empty()`
        // guard flipped to `false` would fall through to
        // `p.terms.get(ENDING)`, which is None here, so the mutant sees no
        // follower at all and wrongly applies guNa ("nI" -> "ne").
        let mut p = Prakriya {
            terms: vec![Term::new("nI"), Term::new("ya")],
            log: vec![],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Ngit);
        let rule = rules().find(|r| r.id == "7.3.84").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "nI");
    }

    #[test]
    fn sarvadhatukardhadhatukayoh_single_term_anga_still_applies_guna() {
        // len == 1 (no vikaraNa term, no ending, no follower at all):
        // `following_sarvadhatuka`'s `p.terms.get(SHAP)` is already None, so
        // the match's `None => None` arm returns None without ever calling
        // `p.terms.get(ENDING)` or indexing anything -- nothing can block,
        // and guNa proceeds normally: "nI" -> "ne". This pins that `None`
        // arm and its no-panic guarantee: unlike the old `p.terms[SHAP]`
        // guard, which would have panicked indexing a 1-element Vec,
        // `.get()` never panics here regardless of arity.
        let mut p = Prakriya {
            terms: vec![Term::new("nI")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.3.84").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ne");
    }

    #[test]
    fn sarvadhatukardhadhatukayoh_blocks_guna_when_luk_shap_ending_is_ngit() {
        // The athematic (śap-luk'd) shape: an empty śap interposes nothing,
        // so the NGIT ending is what immediately follows the aGga and 1.1.5
        // must block guNa. Before this arm existed the guard read only
        // terms[SHAP] -- which on this path carries Sap's own Tag::Pit and
        // can never be Ngit -- so the block was silently inoperative.
        let mut p = Prakriya {
            terms: vec![Term::new("nI"), Term::new(""), Term::new("te")],
            log: vec![],
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        let rule = rules().find(|r| r.id == "7.3.84").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "nI");
    }

    #[test]
    fn sarvadhatukardhadhatukayoh_applies_guna_when_luk_shap_ending_is_not_ngit() {
        // Same athematic shape, non-Ngit ending: nothing blocks, guNa fires.
        // This is the "just outside the guard" half of the pair -- without it
        // a mutant that always blocks on the athematic path would survive.
        let mut p = Prakriya {
            terms: vec![Term::new("nI"), Term::new(""), Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.3.84").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "ne");
    }

    #[test]
    fn pugantalaghupadhasya_blocks_guna_when_luk_shap_ending_is_ngit() {
        // 7.3.86 carries the identical 1.1.5 guard, so it needs the identical
        // pair. "vft" is a light ik penult before a single consonant, which is
        // this rule's shape; the Ngit ending must still block it.
        let mut p = Prakriya {
            terms: vec![Term::new("vft"), Term::new(""), Term::new("te")],
            log: vec![],
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        let rule = rules().find(|r| r.id == "7.3.86").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "vft");
    }

    #[test]
    fn pugantalaghupadhasya_applies_guna_when_luk_shap_ending_is_not_ngit() {
        let mut p = Prakriya {
            terms: vec![Term::new("vft"), Term::new(""), Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.3.86").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "vart");
    }

    #[test]
    fn pugantalaghupadhasya_tanadi_arm_is_vikalpa_and_ngit_blind() {
        // tfR + u + anti: the vikalpa arm fires (its trigger is the
        // ārdhadhātuka u, on which 1.1.5 has no purchase — vidyut derives
        // tarRvanti), while the nitya entry declines the gaṇa entirely.
        let mut p = Prakriya {
            terms: vec![Term::new("tfR"), Term::new("u"), Term::new("anti")],
            ..Default::default()
        };
        p.terms[0].add(Tag::Dhatu);
        p.terms[0].add(Tag::Tanadi);
        p.terms[1].add(Tag::Vikarana);
        p.terms[1].add(Tag::Ardhadhatuka);
        p.terms[2].add(Tag::Ngit);
        let mut entries = rules().filter(|r| r.id == "7.3.86");
        let nitya = entries.next().unwrap();
        let vikalpa = entries.next().expect("the tanādi arm");
        assert!(!nitya.vikalpa);
        assert!(vikalpa.vikalpa);
        assert!(!(nitya.apply)(&mut p), "gaṇa 8 belongs to the vikalpa arm");
        assert!((vikalpa.apply)(&mut p));
        assert_eq!(p.terms[0].text, "tarR");
    }

    #[test]
    fn pugantalaghupadhasya_tanadi_arm_declines_a_upadha_and_final_ik() {
        // tan (a upadhā — nothing to guṇate) and kf (ik FINAL — 7.3.84's
        // business): both outside the gaṇasūtra's four.
        for root in ["tan", "kf"] {
            let mut p = Prakriya {
                terms: vec![Term::new(root), Term::new("u"), Term::new("ti")],
                ..Default::default()
            };
            p.terms[0].add(Tag::Dhatu);
            p.terms[0].add(Tag::Tanadi);
            p.terms[1].add(Tag::Vikarana);
            let vikalpa = rules().filter(|r| r.id == "7.3.86").nth(1).unwrap();
            assert!(!(vikalpa.apply)(&mut p), "{root}");
        }
    }

    #[test]
    fn pugantalaghupadhasya_tanadi_arm_declines_a_vowel_final_anga() {
        // The vikalpa arm's own guard, `n < 2 || is_vowel(chars[n - 1])`, has
        // an `||` that no curated tanādi root exercises: kziR/fR/tfR/GfR all
        // close on the consonant R, so `is_vowel(chars[n - 1])` is always
        // false for them and the guard's verdict comes entirely from `n < 2`.
        // A synthetic vowel-final aṅga ("fu") isolates the other disjunct:
        // `n < 2` is false (n=2) but `is_vowel(chars[n - 1] = 'u')` is true,
        // so `||` declines immediately. Under `&&` the guard would instead
        // fall through to the ik-penult check — `f` matches — and wrongly
        // guṇate, turning "fu" into "aru".
        let mut p = Prakriya {
            terms: vec![Term::new("fu"), Term::new("u"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[0].add(Tag::Dhatu);
        p.terms[0].add(Tag::Tanadi);
        p.terms[1].add(Tag::Vikarana);
        let vikalpa = rules().filter(|r| r.id == "7.3.86").nth(1).unwrap();
        assert!(!(vikalpa.apply)(&mut p));
        assert_eq!(
            p.terms[0].text, "fu",
            "guard must decline before any mutation"
        );
    }

    #[test]
    fn shings_guna_leaves_every_other_adadi_root_alone() {
        // 7.4.21 is root-specific. The other five adādi roots must be
        // untouched by it: their finals (`A`, `d`, `s`) are outside the guard,
        // and their shipped forms are the proof.
        assert_eq!(
            form_g("02.0044", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "yAti"
        );
        assert_eq!(
            form_g("02.0001", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "atti"
        );
        assert_eq!(
            form_g("02.0011", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "Aste"
        );
        assert_eq!(
            form_g("02.0013", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "vaste"
        );
        // And the rule declines outright on a prakriya whose aṅga is not √śī,
        // even when everything else about it looks like √śī's environment.
        let mut p = Prakriya {
            terms: vec![Term::new("nI"), Term::new(""), Term::new("te")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.4.21").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "nI");
    }

    /// Build `[anga, SnA, ending]` with the ending's ṅit-ness set explicitly.
    fn shna_prakriya(anga: &str, ending: &str, ngit: bool) -> Prakriya {
        let mut vik = Term::new("nA");
        vik.add(Tag::Vikarana);
        vik.add(Tag::Sarvadhatuka);
        vik.add(Tag::Ngit);
        let mut end = Term::new(ending);
        end.add(Tag::Tin);
        end.add(Tag::Sarvadhatuka);
        if ngit {
            end.add(Tag::Ngit);
        }
        Prakriya {
            terms: vec![Term::new(anga), vik, end],
            log: vec![],
            ..Default::default()
        }
    }

    #[test]
    fn shnabhyastayor_atah_elides_a_before_ajadi_ngit() {
        // kliS + nA + anti -> kliS + n + anti -> kliSnanti.
        let mut p = shna_prakriya("kliS", "anti", true);
        let rule = rules().find(|r| r.id == "6.4.112").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "n");
        assert_eq!(p.text(), "kliSnanti");
    }

    #[test]
    fn shnabhyastayor_atah_declines_on_halali_and_on_non_ngit() {
        // Consonant-initial: 6.4.113's case, not this rule's.
        let mut p = shna_prakriya("kliS", "taH", true);
        let rule = rules().find(|r| r.id == "6.4.112").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        // Vowel-initial but PIT (lot 3pl would be the only ajadi pit ending
        // if 1.2.4 misfired): the A must survive.
        let mut p = shna_prakriya("kliS", "anti", false);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
    }

    #[test]
    fn i_halyaghoh_replaces_a_with_i_before_halali_ngit() {
        // kliS + nA + taH -> kliS + nI + taH -> kliSnItaH.
        let mut p = shna_prakriya("kliS", "taH", true);
        let rule = rules().find(|r| r.id == "6.4.113").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nI");
        assert_eq!(p.text(), "kliSnItaH");
    }

    #[test]
    fn i_halyaghoh_declines_on_ajadi_and_on_non_ngit() {
        // Vowel-initial: 6.4.112's case.
        let mut p = shna_prakriya("kliS", "anti", true);
        let rule = rules().find(|r| r.id == "6.4.113").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        // The pit case is the whole paradigm split: kliSnAti, not *kliSnIti.
        let mut p = shna_prakriya("kliS", "ti", false);
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        assert_eq!(p.text(), "kliSnAti");
    }

    // --- 7.3.84 second application: the vikaraṇa-aṅga guṇa ------------------

    fn second_7_3_84() -> &'static Rule {
        assert_eq!(
            rules().filter(|r| r.id == "7.3.84").count(),
            2,
            "expected exactly two 7.3.84 entries; nth(1) locator assumes this"
        );
        rules().filter(|r| r.id == "7.3.84").nth(1).unwrap()
    }

    #[test]
    fn second_7_3_84_gunates_shnu_before_a_pit_ending() {
        // Ap + nu + ti → Ap + no + ti. `ti` is pit, so 1.1.5 does not block.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("ti")],
            ..Default::default()
        };
        assert!((second_7_3_84().apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "no");
        assert_eq!(p.terms[ANGA].text, "Ap", "the root must not be touched");
    }

    #[test]
    fn second_7_3_84_blocked_by_a_ngit_ending() {
        // Ap + nu + taH → ApnutaH. `tas` is apit, so the first 1.2.4 tagged it
        // ṅit and 1.1.5 blocks guṇa. This is the gaṇa's signature contrast.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("taH")],
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        assert!(!(second_7_3_84().apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nu");
    }

    #[test]
    fn second_7_3_84_declines_on_a_thematic_vikarana() {
        // bhvādi: SHAP is śap's `a`, not an ik. The no-delta guard, half one.
        let mut p = Prakriya {
            terms: vec![Term::new("Bo"), Term::new("a"), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(second_7_3_84().apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "a");
    }

    #[test]
    fn second_7_3_84_declines_on_kryadi_shni() {
        // kryādi: `nI` IS ik-final, so only the 1.1.5 guard keeps this rule off
        // it — and 6.4.113 only ever produces `nI` before a ṅit ending, so the
        // guard is always satisfied. The no-delta guard, half two. If this ever
        // fires, kryādi surfaces *kliSne and 1872 goldens move.
        let mut p = Prakriya {
            terms: vec![Term::new("kliS"), Term::new("nI"), Term::new("taH")],
            ..Default::default()
        };
        p.terms[ENDING].add(Tag::Ngit);
        assert!(!(second_7_3_84().apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nI");
    }

    #[test]
    fn second_7_3_84_declines_on_an_empty_shap() {
        // adādi: śap is luk'd to an empty string. Must not panic.
        let mut p = Prakriya {
            terms: vec![Term::new("ad"), Term::new(""), Term::new("ti")],
            ..Default::default()
        };
        assert!(!(second_7_3_84().apply)(&mut p));
    }

    #[test]
    fn shna_alternation_rules_ignore_other_vikaranas_and_short_prakriyas() {
        // The text guard is what keeps these off Sap/Syan/Sa and off the
        // Sanac that 3.1.83 substitutes ("Ana", not "nA").
        for vikarana in ["a", "ya", "Ana", ""] {
            let mut p = shna_prakriya("kliS", "taH", true);
            p.terms[SHAP].text = vikarana.to_string();
            for id in ["6.4.112", "6.4.113"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                assert!(!(rule.apply)(&mut p), "{id} fired on {vikarana:?}");
            }
        }
        // A one-term prakriya must not panic indexing SHAP or ENDING.
        let mut p = Prakriya {
            terms: vec![Term::new("kliS")],
            log: vec![],
            ..Default::default()
        };
        for id in ["6.4.112", "6.4.113"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p));
        }
    }

    // --- 6.1.78 third arm: the vikaraṇa-final ec ---------------------------

    #[test]
    fn eco_yavayavah_converts_the_vikaranas_o_before_a_vowel_ending() {
        // Ap + no + Ani → Ap + nav + Ani → ApnavAni.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("no"), Term::new("Ani")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.78").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nav");
        assert_eq!(p.terms[ANGA].text, "Ap");
    }

    #[test]
    fn eco_yavayavah_vikarana_arm_declines_before_a_consonant_ending() {
        // Apnoti: `ti` is consonant-initial, so nothing converts.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("no"), Term::new("ti")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.78").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "no");
    }

    #[test]
    fn eco_yavayavah_thematic_arm_still_wins_for_bhvadi() {
        // Bo + a + ti → Bav + a + ti. The root's `o`, not the vikaraṇa's.
        let mut p = Prakriya {
            terms: vec![Term::new("Bo"), Term::new("a"), Term::new("ti")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.78").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bav");
        assert_eq!(p.terms[SHAP].text, "a");
    }

    #[test]
    fn eco_yavayavah_athematic_arm_still_wins_for_adadi() {
        // Se + "" + Iyran → Say + "" + Iyran (√śī vidhiliṅ 3pl).
        let mut p = Prakriya {
            terms: vec![Term::new("Se"), Term::new(""), Term::new("Iyran")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "6.1.78").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Say");
    }

    // --- 6.4.87 / 6.4.77: śnu's u before a vowel ---------------------------

    #[test]
    fn hushnuvoh_yields_yan_for_a_vowel_final_root() {
        // hi + nu + anti → hi + nv + anti → hinvanti.
        let mut p = Prakriya {
            terms: vec![Term::new("hi"), Term::new("nu"), Term::new("anti")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        let rule = rules().find(|r| r.id == "6.4.87").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nv");
    }

    #[test]
    fn hushnuvoh_declines_on_a_conjunct_and_leaves_it_to_6_4_77() {
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("anti")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        let apavada = rules().find(|r| r.id == "6.4.87").unwrap();
        assert!(!(apavada.apply)(&mut p));
        let utsarga = rules().find(|r| r.id == "6.4.77").unwrap();
        assert!((utsarga.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nuv");
    }

    #[test]
    fn shnu_vowel_rules_decline_before_a_consonant_ending() {
        // ApnutaH: `taH` is consonant-initial, so neither fires.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("nu"), Term::new("taH")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        for id in ["6.4.87", "6.4.77"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} should decline");
        }
        assert_eq!(p.terms[SHAP].text, "nu");
    }

    #[test]
    fn shnu_vowel_rules_decline_once_guna_has_run() {
        // ApnavAni: 7.3.84's second application already made SHAP `no`, so
        // neither rule matches `nu` any more. This is what keeps the ordering
        // constraint honest — *ApnuvAni is the failure it prevents.
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("no"), Term::new("Ani")],
            ..Default::default()
        };
        p.terms[SHAP].add(Tag::Vikarana);
        for id in ["6.4.87", "6.4.77"] {
            let rule = rules().find(|r| r.id == id).unwrap();
            assert!(!(rule.apply)(&mut p), "{id} should decline");
        }
    }

    #[test]
    fn shnu_vowel_rules_never_touch_another_ganas_vikarana() {
        // kryādi's `nA` and bhvādi's `a` must be invisible to both rules.
        for shap in ["nA", "nI", "n", "a", "ya", "Ana", ""] {
            let mut p = Prakriya {
                terms: vec![Term::new("kliS"), Term::new(shap), Term::new("anti")],
                ..Default::default()
            };
            p.terms[SHAP].add(Tag::Vikarana);
            for id in ["6.4.87", "6.4.77"] {
                let rule = rules().find(|r| r.id == id).unwrap();
                assert!(!(rule.apply)(&mut p), "{id} fired on SHAP {shap:?}");
            }
        }
    }

    // --- 6.4.108/109/110: the √kṛ specials -------------------------------

    fn kr_prakriya(ending: &str, ngit: bool) -> Prakriya {
        // Post-first-7.3.84 shape: kar + u + ending.
        let mut p = Prakriya {
            terms: vec![Term::new("kar"), Term::new("u"), Term::new(ending)],
            ..Default::default()
        };
        p.terms[0].add(Tag::Dhatu);
        p.terms[0].add(Tag::Tanadi);
        p.terms[1].add(Tag::Vikarana);
        p.terms[1].add(Tag::Ardhadhatuka);
        if ngit {
            p.terms[2].add(Tag::Ngit);
        }
        p
    }

    #[test]
    fn ata_ut_fires_only_before_ngit_sarvadhatuka() {
        let r = rules().find(|r| r.id == "6.4.110").unwrap();
        let mut p = kr_prakriya("tas", true);
        assert!((r.apply)(&mut p));
        assert_eq!(p.terms[0].text, "kur");
        // karoti's pit ti: no ut.
        let mut p = kr_prakriya("ti", false);
        assert!(!(r.apply)(&mut p));
        // Another root's `a` is not karoti's: the text guard alone
        // (there is no Tag::Tanadi clause left to save this) rejects it.
        let mut p = kr_prakriya("tas", true);
        p.terms[0].text = "tan".into();
        assert!(!(r.apply)(&mut p));
        // laN's aT-augmented aGga (6.4.71 prefixes onto ANGA's own text):
        // akarutAm must become akurutAm, not decline on `!= "kar"`.
        let mut p = kr_prakriya("tas", true);
        p.terms[0].text = "akar".into();
        assert!((r.apply)(&mut p));
        assert_eq!(p.terms[0].text, "akur");
    }

    #[test]
    fn ata_ut_uses_n_minus_2_not_n_over_2() {
        // 6.4.110 writes the ut at `s[n - 2]`, the `a` of the aGga's final
        // `kar`. Both aGga shapes the golden corpus reaches -- "kar" (n=3,
        // n-2 = 1, n/2 = 1) and laN's aT-augmented "akar" (n=4, n-2 = 2,
        // n/2 = 2) -- give the SAME index for `n - 2` and `n / 2`, so no
        // paradigm cell can tell the two apart. This test exists solely to
        // distinguish them: an upasarga-prefixed "vikar" (vi + kf) has n=5,
        // where n-2 = 3 (the `a`) but n/2 = 2 (the `k`). The rule must
        // write "vikur"; the `-` -> `/` mutant would write "viuar".
        let r = rules().find(|r| r.id == "6.4.110").unwrap();
        let mut p = kr_prakriya("tas", true);
        p.terms[0].text = "vikar".into();
        assert!((r.apply)(&mut p));
        assert_eq!(p.terms[0].text, "vikur");
    }

    #[test]
    fn nityam_karoteh_empties_the_u_before_m_and_v() {
        let r = rules().find(|r| r.id == "6.4.108").unwrap();
        for ending in ["mas", "vas"] {
            let mut p = kr_prakriya(ending, true);
            p.terms[0].text = "kur".into();
            assert!((r.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[1].text, "", "{ending}");
        }
        // Not before tas — that u survives (kurutaH).
        let mut p = kr_prakriya("tas", true);
        p.terms[0].text = "kur".into();
        assert!(!(r.apply)(&mut p));
        // laN's akur must fire too (akurva, akurma — single branch, no
        // 6.4.107 fork): the guard reads the aGga tail, not its whole text.
        for ending in ["mas", "vas"] {
            let mut p = kr_prakriya(ending, true);
            p.terms[0].text = "akur".into();
            assert!((r.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[1].text, "", "{ending}");
        }
    }

    #[test]
    fn ye_ca_empties_the_u_before_y() {
        let r = rules().find(|r| r.id == "6.4.109").unwrap();
        let mut p = kr_prakriya("yAt", true);
        p.terms[0].text = "kur".into();
        assert!((r.apply)(&mut p));
        assert_eq!(p.terms[1].text, "");
        // Not before tas — kuryAt's lopa is y-specific, kurutaH keeps its u.
        let mut p = kr_prakriya("tas", true);
        p.terms[0].text = "kur".into();
        assert!(!(r.apply)(&mut p));
        // The guard reads the aGga tail, not its whole text — pin that
        // directly against an aT-prefixed akur, same as 6.4.108 above.
        let mut p = kr_prakriya("yAt", true);
        p.terms[0].text = "akur".into();
        assert!((r.apply)(&mut p));
        assert_eq!(p.terms[1].text, "");
    }
}
