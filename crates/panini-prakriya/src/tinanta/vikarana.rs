//! Vikaraṇa selection and luk: 3.1.69, 3.1.73, 3.1.77, 3.1.78, 3.1.81,
//! 3.1.68, 2.4.72, 3.1.83, 1.2.4.
//!
//! **This stage contains the 3.1.68 boundary.** Rules before 3.1.68 in this
//! file address the ending as `ENDING_PRE_SHAP` (index 1); rules after it use
//! `ENDING` (index 2) and may use `SHAP`. Get this wrong and a rule mutates
//! śap while believing it is mutating the ending, or panics indexing a slot
//! that does not exist yet. See `super::terms`.
//!
//! 2.4.72 luks śap by emptying its text in place rather than removing the
//! term, which is what keeps every later index valid — and what makes
//! `terms[SHAP].text` possibly empty for the rest of the pipeline. 3.1.78
//! carries a second SHAP hazard of its own — `terms[SHAP].text` may hold the
//! root's tail, not just the vikaraṇa — see `super::terms`'s second caveat.

use crate::it_samjna::run_it_samjna;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::sound::is_vowel;
use crate::tinanta::terms::{ANGA, ENDING, SHAP, sound_before_ending};
use panini_data::Lakara;

pub(crate) static VIKARANA: &[Rule] = &[
    // 3.1.69 divādibhyaḥ śyan: divādi (gaṇa 4) takes śyan, not śap. Apavāda
    // to the utsarga 3.1.68, ordered before it (as 6.4.72 precedes 6.4.71).
    // śyan is apit; the second 1.2.4 makes it ṅit and 1.1.5 then blocks guṇa.
    Rule {
        id: "3.1.69",
        name: "divAdiByaH Syan",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Divadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Syan");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            s.add(Tag::Thematic); // a-final after it-lopa: Syan -> ya
            p.terms.insert(SHAP, s);
            p.record("3.1.69", "divAdiByaH Syan", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S, 1.3.3 strips n → ya
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.73 svādibhyaḥ śnuḥ: svādi (gaṇa 5) takes śnu, not śap. Apavāda to
    // 3.1.68, ordered before it, exactly as 3.1.69, 3.1.77 and 3.1.81 are.
    //
    // śnu is apit, so the second 1.2.4 below tags it ṅit with no change of
    // its own — which is what blocks the FIRST 7.3.84 on the ik-final roots
    // (hi, ri): hinoti, not *henoti. The guṇa svādi IS famous for lands on
    // śnu's own `u` and belongs to 7.3.84's SECOND application (`guna.rs`),
    // because by 1.4.13 the aṅga for the tiṅ ending is root + vikaraṇa.
    //
    // Unlike śnā, śnu's text never changes shape here — 6.4.87 and 6.4.77
    // rewrite its `u` later, in `guna.rs`, and only before a vowel.
    Rule {
        id: "3.1.73",
        name: "svAdiByaH SnuH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Svadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Snu");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.73", "svAdiByaH SnuH", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → nu
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.77 tudādibhyaḥ śaḥ: tudādi (gaṇa 6) takes śa, not śap. Apavāda to
    // 3.1.68, same shape as 3.1.69. śa is apit → ṅit (1.2.4) → guṇa blocked.
    Rule {
        id: "3.1.77",
        name: "tudAdiByaH SaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Tudadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Sa");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            s.add(Tag::Thematic); // a-final after it-lopa: Sa -> a
            p.terms.insert(SHAP, s);
            p.record("3.1.77", "tudAdiByaH SaH", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → a
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.78 rudhādibhyaḥ śnam: rudhādi (gaṇa 7) takes śnam, not śap.
    // Apavāda to 3.1.68, ordered before it exactly as 3.1.69, 3.1.73,
    // 3.1.77 and 3.1.81 are.
    //
    // Unlike every other vikaraṇa, śnam is NOT a suffix. It is **mit**, and
    // 1.1.47 mid aco'ntyāt paraḥ places a mit affix after the last vowel of
    // what it attaches to: kft + śnam is `kfnat`, not kft + na. 1.1.47 is a
    // paribhāṣā and is cited here rather than implemented as its own Rule —
    // the treatment 1.4.13 and 1.1.5 already get, and what vidyut-prakriya's
    // trace does (it emits 3.1.78 and never 1.1.47).
    //
    // REPRESENTATION, load-bearing. The pipeline's three fixed slots
    // [ANGA, SHAP, ENDING] have nowhere to put an infix, so the root is
    // split across the first two: ANGA keeps the head through its last
    // vowel, SHAP holds śnam followed by whatever the root had after it.
    // kft → [kf, nat, ti]; hins → [hi, nans, ti].
    //
    // The consequence — recorded in `super::terms` too — is that
    // terms[SHAP].text is no longer purely the vikaraṇa for this gaṇa.
    // 6.4.23 deletes a nasal that came from the ROOT but now lives in SHAP,
    // and 6.4.111 deletes śnam's own `a` from the same term.
    //
    // The alternative — ANGA holding the whole infixed stem with SHAP empty,
    // the adādi śap-luk shape — was rejected: it forces 6.4.23 and 6.4.111
    // to locate a character by position inside a merged string, which is the
    // failure mode this file's header exists to warn about.
    //
    // ORDERING WITHIN THE RULE: the OPERATION order is unchanged from a
    // plain apavāda — the it-saṁjñā still runs BEFORE the root's tail is
    // appended. With the tail already in place, 1.3.3 halantyam would strip
    // the ROOT's final consonant instead of śnam's mit `m`.
    //
    // But `run_it_samjna` is not silent: it takes its own snapshot and
    // records its own `1.3.9 tasya lopaH` step. Recording 3.1.78 only once,
    // after it-saṁjñā and the 1.1.47 placement both ran, would invert that
    // step against 3.1.78 in the trace (1.3.9 would appear to precede the
    // rule that introduces what it elides) and rewind the before/after
    // chain. So the RECORD is split into two 3.1.78 entries around the
    // it-saṁjñā call, even though the operation itself is not: the first
    // records the insertion (kftti → kftSnamti), then it-saṁjñā logs its own
    // 1.3.9 (kftSnamti → kftnati), then the second 3.1.78 entry records the
    // 1.1.47 placement (kftnati → kfnatti). The trace then reads 3.1.78,
    // 1.3.9, 3.1.78 — in sūtra order, chain continuous — with the placement
    // attributed to 3.1.78 rather than to 1.1.47, matching what
    // vidyut-prakriya's trace does.
    Rule {
        id: "3.1.78",
        name: "ruDAdiByaH Snam",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Rudhadi) {
                return false;
            }
            let root: Vec<char> = p.terms[ANGA].text.chars().collect();
            let Some(last_vowel) = root.iter().rposition(|c| is_vowel(*c)) else {
                return false;
            };
            let head: String = root[..=last_vowel].iter().collect();
            let tail: String = root[last_vowel + 1..].iter().collect();
            let before = p.snapshot();
            let mut s = Term::new("Snam");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.78", "ruDAdiByaH Snam", before);
            // 1.3.8 laSakvataddhite strips S; 1.3.3 halantyam strips the
            // mit m. Leaves `na`. Logs its own 1.3.9 step.
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP);
            p.terms[SHAP] = s;
            let before = p.snapshot();
            // 1.1.47, cited above.
            p.terms[ANGA].text = head;
            p.terms[SHAP].text.push_str(&tail);
            p.terms[ANGA].add(Tag::Anga);
            p.record("3.1.78", "ruDAdiByaH Snam", before);
            true
        },
    },
    // 3.1.81 kryādibhyaḥ śnā: kryādi (gaṇa 9) takes śnā, not śap. Apavāda to
    // 3.1.68, ordered before it, exactly as 3.1.69 and 3.1.77 are. śnā is
    // apit; the second 1.2.4 makes it ṅit and 1.1.5 then blocks guṇa — which
    // is what keeps kliS from guṇating to kleS under 7.3.86.
    //
    // Unlike adādi's śap, śnā is never luk'd: its text goes nA → nI (6.4.113)
    // or nA → n (6.4.112), and never to empty. But a rule that guards on
    // `SHAP.is_empty()` to detect "the thematic coalescence rules didn't
    // apply" still silently declines for kryādi: its SHAP is non-empty but
    // also non-`a`-final, so `is_empty()` misses it exactly where an
    // athematic arm is needed. 6.1.66 (`adesha.rs`) learned this the hard
    // way — its old emptiness guard produced *vfRIyta instead of vfRIta
    // until it was widened to `!SHAP.ends_with('a')`.
    //
    // That text test was itself later found unsound, but not simply
    // "replace it with the tag" unsound — the two are different questions,
    // and a rule must pick the one it actually means:
    //   - "IS SHAP one of the four a-final vikaraṇas (śap/śyan/śa/śānac)":
    //     `Tag::Thematic` (its own comment in `term.rs`). This is what an
    //     athematic-vs-thematic PATH GUARD wants — deciding WHICH of a
    //     rule's arms applies — and it is what 6.1.101, 6.1.97, 6.1.87,
    //     6.1.66's athematic arm and 6.4.105 (`adesha.rs`) all guard on
    //     now. rudhādi's śnam-infix split (3.1.78) leaves SHAP as the
    //     a-final `"na"`, which is not one of the four; the text test could
    //     not tell the difference and silently mistreated śnam as śap.
    //   - "DOES SHAP's text currently end in short `a`": plain
    //     `SHAP.text.ends_with('a')`, unchanged, still the guard for
    //     7.3.101 (`super::guna`) — a rule doing vowel SANDHI on that `a`,
    //     which needs the current surface shape, not the vikaraṇa's
    //     identity, and for which the two CAN differ (a thematic vikaraṇa
    //     whose `a` an earlier rule already rewrote is still thematic by
    //     identity, but is no longer `a`-final by shape).
    //   6.1.101, 6.1.97 and 6.1.87 each ALSO mutate SHAP's last character
    //   once their (now identity-gated) guard passes — but that mutation
    //   code is not itself a second guard; it runs only after the identity
    //   check already confirmed this cell's SHAP is thematic. Identity and
    //   shape do NOT stay coincident once that stage is running: √bhū loṭ
    //   uttama eka takes `Bav + a + Ani` to `Bav + A + ni` at 6.1.101's
    //   bhvādi arm, so 6.1.97 three rules later reads this tag on a SHAP
    //   already drifted from `a`-final (7.3.101 drifts it the same way one
    //   stage earlier, before an m/v-initial ending). What keeps those
    //   mutations sound is the ENDING test each of the five carries
    //   alongside the tag — an `A`-, guṇa- or `i`/`I`-initial ending, or
    //   the bare `hi` — which the leftover ending never satisfies once the
    //   drift has happened (`ni`/`va`/`ma`/`vahE`/`mahE`, `m`/`v`). See `Tag::Thematic`'s
    //   own comment in `crate::term` for the worked trace.
    Rule {
        id: "3.1.81",
        name: "kryAdiByaH SnA",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Kryadi) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("SnA");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            p.terms.insert(SHAP, s);
            p.record("3.1.81", "kryAdiByaH SnA", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S → nA
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 3.1.68 kartari śap: insert śap between dhātu and ending, run it-samjña
    // on it (Sap → a), and mark the dhātu an aṅga.
    Rule {
        id: "3.1.68",
        name: "kartari Sap",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            // Utsarga: fires only when no apavāda vikaraṇa (śyan 3.1.69 / śa
            // 3.1.77) is already present. Guarding on the vikaraṇa's presence
            // keeps śap the default without hard-coding a gaṇa, so curādi can
            // reuse śap later.
            if p.terms.len() > SHAP && p.terms[SHAP].has(Tag::Vikarana) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("Sap");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            s.add(Tag::Pit); // p-anubandha: śap is pit, so 1.2.4 leaves it alone
            s.add(Tag::Thematic); // a-final after it-lopa: Sap -> a
            p.terms.insert(SHAP, s);
            p.record("3.1.68", "kartari Sap", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP);
            p.terms[SHAP] = s;
            p.terms[ANGA].add(Tag::Anga);
            true
        },
    },
    // 2.4.72 adiprabhṛtibhyaḥ śapaḥ: adādi (gaṇa 2) luks the śap that 3.1.68
    // inserts, so the tiṅ ending attaches directly to the root. Modelled by
    // emptying the śap term's text (the term stays, keeping ENDING at index 2
    // and text() = root + "" + ending). Guarded on Tag::Adadi and on a real
    // śap being present, so it never touches divādi/tudādi (śyan/śa) or bhvādi
    // that has already been processed differently.
    //
    // Also drops Tag::Thematic, which 3.1.68 just set: even though
    // `Tag::Thematic` is an IDENTITY tag ("this term IS one of the four
    // a-final vikaraṇas"), not a live-shape one — see its own comment —
    // luk (1.1.61 pratyayasya lopa ādarśanam) removes the vikaraṇa ITSELF,
    // so after this rule the term is no longer śap at any level, identity
    // included. `Tag::Vikarana` stays: it marks that a vikaraṇa-shaped term
    // occupies the slot at all (needed for sthānivadbhāva / apavāda
    // bookkeeping elsewhere), which luk does not undo. Leaving
    // `Tag::Thematic` set here would let the five `adesha.rs` rules that
    // read it treat adādi's empty, luk'd śap as still one of the thematic
    // four, which the text test they replaced never did
    // (`"".ends_with('a')` is false).
    Rule {
        id: "2.4.72",
        name: "adipraBftiByaH SapaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Adadi) {
                return false;
            }
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].text.is_empty())
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].text = String::new();
            p.terms[SHAP].remove(Tag::Thematic);
            p.record("2.4.72", "adipraBftiByaH SapaH", before);
            true
        },
    },
    // 3.4.110 ātaḥ / 3.4.111 laṅaḥ śākaṭāyanasyaiva: after an ā-final aṅga,
    // jhi is replaced by jus — and in laṅ that replacement is Śākaṭāyana's,
    // i.e. OPTIONAL. One rule implements the pair, cited under 3.4.111,
    // because 3.4.110 supplies only the condition and is never separately
    // observable here; vidyut-prakriya records the single step the same way.
    // Its witnesses are the two ā-final adādi roots: ayAn / ayuH, avAn /
    // avuH.
    //
    // `J`, not `Ji`: 3.4.100 itaś ca has already dropped jhi's final `i` in
    // the tiṅ stage (laṅ is ṅit-like and this is parasmaipada). The term is
    // still jhi — 3.4.110/111 replace the whole of it — but its text is not.
    //
    // The ā is read AFFIX-RELATIVELY via sound_before_ending, rather than by
    // hardcoding a term position, so it keeps working if a future gaṇa's
    // vikaraṇa is luk'd the same way adādi's śap is. Placing the rule after
    // 2.4.72 is what makes that reading available at all — and it is also
    // what forces the `J` guard above.
    //
    // But sound_before_ending alone is not enough to gate this to adādi: it
    // reports whatever sound truly precedes the ending, and kryādi's śnā
    // vikaraṇa is itself `A`-final ("nA", reduced to n/nI only later by
    // 6.4.112/6.4.113) — so it reports the same `A` the adādi witnesses do,
    // even though the DHĀTU there (kliS) is a consonant-final root with no
    // claim on 3.4.110 at all. The brief's literal guard, lacking the extra
    // conjunct below, forked `akliSnan` into a spurious `akliSnan`/`akliSnuH`
    // for exactly this reason. `SHAP.text.is_empty()` is what excludes it: it
    // holds only when nothing but the aṅga itself could be the sound
    // sound_before_ending found — true for adādi (śap luk'd by 2.4.72),
    // false for any live vikaraṇa, kryādi's śnā included. The guard test's
    // 4th case (a live vikaraṇa standing between a thematic ā-final aṅga and
    // the ending) is what witnesses sound_before_ending's own contribution —
    // it must decline on the character check, not merely on this conjunct,
    // which is why the character check is ordered first below.
    //
    // This reads as `is_empty()` rather than the 3.1.81 comment's own
    // `!ends_with('a')` advice (90 lines above) — that is not an oversight.
    // This guard needs "śap was luk'd" (true only for adādi), not
    // "athematic" (also true of kryādi's ā-final śnā, which is exactly the
    // over-generation this conjunct exists to rule out); `!ends_with('a')`
    // would let kryādi's ā-final `nA` through unchanged, reintroducing the
    // spurious fork this rule was written to prevent.
    //
    // Must sit above 7.1.3 jho'ntaḥ, which turns a surviving `J` into `ant`.
    Rule {
        id: "3.4.111",
        name: "laNaH SAkawAyanasyEva",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) || p.terms[ENDING].text != "J" {
                return false;
            }
            if sound_before_ending(p) != Some('A') || !p.terms[SHAP].text.is_empty() {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = "jus".into();
            p.record("3.4.111", "laNaH SAkawAyanasyEva", before);
            let before = p.snapshot();
            p.terms[ENDING].text = "us".into();
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
    // 3.1.83 halaḥ śnaḥ śānac: after a CONSONANT-final root, with `hi`
    // following, śnā is replaced wholesale by śānac. it-samjña strips the
    // leading S (1.3.8) and the final c (1.3.3), leaving `Ana`; the existing
    // 6.4.105 ato heḥ then elides the hi after śāna's short `a`, giving
    // kliSAna. No new rule is needed for the hi-lopa. śānac is the FOURTH
    // a-final vikaraṇa (with śap, śyan, śa) and carries `Tag::Thematic` for
    // exactly that reason — 6.4.105 reads the tag, not śāna's text, to
    // decide whether it applies.
    //
    // Placement carries two constraints, both failing visibly:
    //   - BEFORE 6.4.113 (anga stage, later): that rule would otherwise turn
    //     śnā's ā into ī before the consonant-initial ṅit `hi` and give
    //     *kliSnIhi. As an apavāda, 3.1.83 must remove śnā first.
    //   - BEFORE the second 1.2.4, immediately below: śānac is apit and must
    //     be tagged ṅit, or 7.3.86 guṇates kliS's laghu upadhā and the form
    //     surfaces as *kleSAna.
    //
    // Vowel-final roots fall outside "halaḥ" and keep śnā, taking 6.4.113 to
    // vrIRIhi. That pair — kliSAna against vrIRIhi — is the rule's pin.
    //
    // Its id is 3.1.x but it lives after the 3.1.68 boundary, so it addresses
    // the ending as ENDING (index 2). Stage placement is by pipeline position,
    // not sūtra family; see `super::terms`. The `hi` it reads already exists:
    // 3.4.87 ser hyapic ca runs in the earlier `tin` stage.
    Rule {
        id: "3.1.83",
        name: "halaH SnaH SAnajJO",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        apply: |p| {
            if p.terms.len() <= ENDING || p.terms[SHAP].text != "nA" {
                return false;
            }
            if p.terms[ENDING].text != "hi" {
                return false;
            }
            let Some(last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if is_vowel(last) {
                return false;
            }
            let before = p.snapshot();
            let mut s = Term::new("SAnac");
            s.add(Tag::Vikarana);
            s.add(Tag::Sarvadhatuka);
            s.add(Tag::Thematic); // a-final after it-lopa: SAnac -> Ana
            p.terms[SHAP] = s;
            p.record("3.1.83", "halaH SnaH SAnajJO", before);
            let mut s = p.terms[SHAP].clone();
            run_it_samjna(&mut s, p, SHAP); // 1.3.8 strips S, 1.3.3 strips c → Ana
            p.terms[SHAP] = s;
            true
        },
    },
    // 1.2.4 sārvadhātukam apit — second application, on the vikaraṇa. The
    // first application (above the boundary) tags apit ātmanepada endings;
    // this one tags the apit sārvadhātuka VIKARAṆA ṅit once it exists. śyan
    // and śa are apit (no p-anubandha); śap carries Tag::Pit (3.1.68) and is
    // skipped — so bhvādi is untouched. NOT pada-gated: śyan/śa are apit in
    // parasmaipada derivations too, which is what blocks guṇa in dīvyati /
    // kupyati / tudati.
    Rule {
        id: "1.2.4",
        name: "sArvaDAtukam apit",
        kind: RuleKind::Atidesha,
        vikalpa: false,
        apply: |p| {
            if !(p.terms.len() > SHAP
                && p.terms[SHAP].has(Tag::Vikarana)
                && !p.terms[SHAP].has(Tag::Pit)
                && !p.terms[SHAP].has(Tag::Ngit))
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[SHAP].add(Tag::Ngit);
            p.record("1.2.4", "sArvaDAtukam apit", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tinanta::derive;
    use crate::tinanta::rules;
    use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};

    #[test]
    fn svadibhyah_shnu_inserts_nu_for_svadi_only() {
        let mut p = Prakriya {
            terms: vec![Term::new("Ap"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Svadi);
        let rule = rules().find(|r| r.id == "3.1.73").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nu");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(p.terms[SHAP].has(Tag::Sarvadhatuka));
        assert_eq!(p.terms[ENDING].text, "ti");
    }

    #[test]
    fn svadibhyah_shnu_declines_without_the_gana_tag() {
        // bhvādi: no Tag::Svadi, so the apavāda must not fire and 3.1.68 keeps
        // its utsarga job.
        let mut p = Prakriya {
            terms: vec![Term::new("BU"), Term::new("ti")],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.73").unwrap();
        assert!(!(rule.apply)(&mut p));
    }

    #[test]
    fn shnu_is_tagged_ngit_by_the_second_1_2_4_without_change() {
        // śnu carries no p-anubandha, so the existing second 1.2.4 must tag it
        // ṅit with no edit. This is what blocks the FIRST 7.3.84 on ik-final
        // roots (hinoti, not *henoti).
        let mut p = Prakriya {
            terms: vec![Term::new("hi"), Term::new("ti")],
            ..Default::default()
        };
        p.terms[ANGA].add(Tag::Svadi);
        let shnu = rules().find(|r| r.id == "3.1.73").unwrap();
        assert!((shnu.apply)(&mut p));
        assert_eq!(rules().filter(|r| r.id == "1.2.4").count(), 2);
        let second = rules().filter(|r| r.id == "1.2.4").nth(1).unwrap();
        assert!((second.apply)(&mut p));
        assert!(p.terms[SHAP].has(Tag::Ngit));
    }

    // --- 3.1.68 / second 1.2.4: `len() > SHAP` boundary pins --------------
    //
    // Both guards read `p.terms.len() > SHAP && p.terms[SHAP]. ...` to
    // avoid indexing the not-yet-inserted vikaraNa slot. Every real
    // derivation always has an ending term present (terms.len() >= 2)
    // before either rule runs, so `> SHAP` (i.e. `> 1`) and `>= SHAP`
    // never diverge on any golden or negative derivation: len() is never
    // exactly 1 there. Pin the boundary directly with a single-term
    // Prakriya (aGga only, no ending) so the two outcomes diverge: the
    // original short-circuits before indexing terms[SHAP]; the `>` -> `>=`
    // mutant does not, and panics indexing out of bounds on a 1-element
    // Vec (an unexpected panic still fails the test).
    #[test]
    fn kartari_sap_single_term_anga_does_not_panic() {
        let mut p = Prakriya {
            terms: vec![Term::new("kf")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.68").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
        assert_eq!(p.terms[SHAP].text, "a");
    }

    #[test]
    fn sarvadhatukam_apit_second_application_single_term_does_not_panic() {
        // The SECOND "1.2.4" rule in TINANTA_RULES (the vikaraNa-Girit
        // application, ordered after 3.1.68) is targeted here, not the
        // first (ENDING_PRE_SHAP) application above the 3.1.68 boundary.
        let mut p = Prakriya {
            terms: vec![Term::new("kf")],
            log: vec![],
            ..Default::default()
        };
        assert_eq!(
            rules().filter(|r| r.id == "1.2.4").count(),
            2,
            "expected exactly two 1.2.4 rule entries; nth(1) locator assumes this"
        );
        let rule = rules().filter(|r| r.id == "1.2.4").nth(1).unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
    }

    // --- 2.4.72: `len() > SHAP` boundary + guard-order pins ----------------
    //
    // 2.4.72's guard is `len() > SHAP && has(Vikarana) && !text.is_empty()`,
    // each conjunct short-circuiting before the next would index the
    // not-yet-inserted vikaraNa slot. Every real derivation reaches this
    // rule only after 3.1.68 has already inserted Sap (terms.len() >= 2),
    // so `> SHAP` vs `>= SHAP`, and `&&` vs `||` at either join, never
    // diverge on any golden or negative derivation. Pin the boundary
    // directly: a single-term Prakriya (aGga only, tagged Adadi so the
    // outer gana guard passes) makes `len() > SHAP` (1 > 1) false, so the
    // original short-circuits before ever touching terms[SHAP]. Each of
    // the three mutants below removes a different short-circuit and
    // indexes terms[SHAP] out of bounds on this 1-element Vec, panicking:
    //   - `>` -> `>=`: `1 >= 1` is true, so `has(Vikarana)` is evaluated.
    //   - first `&&` -> `||`: `len() > SHAP` (false) forces evaluation of
    //     `has(Vikarana)` to resolve the OR.
    //   - second `&&` -> `||`: `(len() > SHAP && has(Vikarana))` (false)
    //     forces evaluation of `!text.is_empty()` to resolve the OR.
    // One construction catches all three.
    #[test]
    fn adiprabhrtibhyah_sapah_single_term_anga_does_not_panic() {
        let mut anga = Term::new("kf");
        anga.add(Tag::Adadi);
        let mut p = Prakriya {
            terms: vec![anga],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "2.4.72").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "kf");
    }

    #[test]
    fn kryadibhyah_shna_inserts_shna_for_kryadi_only() {
        // 3.1.81 is an apavAda to 3.1.68, same shape as 3.1.69/3.1.77.
        // it-samjNa strips the S (1.3.8), leaving nA. No Tag::Pit: SnA is
        // apit, so the second 1.2.4 makes it Nit and 1.1.5 then blocks guNa
        // -- which is why kliS gives kliSnAti and not *kleSnAti.
        let mut anga = Term::new("kliS");
        anga.add(Tag::Kryadi);
        let mut p = Prakriya {
            terms: vec![anga, Term::new("ti")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.81").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(p.terms[SHAP].has(Tag::Sarvadhatuka));
        assert!(!p.terms[SHAP].has(Tag::Pit));
        assert!(p.terms[ANGA].has(Tag::Anga));
    }

    #[test]
    fn kryadibhyah_shna_declines_for_every_other_gana() {
        // bhvAdi carries no gana tag at all; the other three carry their own.
        // A mutant that drops the tag guard would give every root SnA.
        for tag in [None, Some(Tag::Divadi), Some(Tag::Tudadi), Some(Tag::Adadi)] {
            let mut anga = Term::new("BU");
            if let Some(t) = tag {
                anga.add(t);
            }
            let mut p = Prakriya {
                terms: vec![anga, Term::new("ti")],
                log: vec![],
                ..Default::default()
            };
            let rule = rules().find(|r| r.id == "3.1.81").unwrap();
            assert!(!(rule.apply)(&mut p), "fired for {tag:?}");
        }
    }

    /// `[anga, SnA, ending]`, the shape 3.1.83 inspects.
    fn shna_before(anga: &str, ending: &str) -> Prakriya {
        let mut vik = Term::new("nA");
        vik.add(Tag::Vikarana);
        vik.add(Tag::Sarvadhatuka);
        Prakriya {
            terms: vec![Term::new(anga), vik, Term::new(ending)],
            log: vec![],
            ..Default::default()
        }
    }

    #[test]
    fn halah_shnah_shanac_replaces_shna_after_a_consonant_final_root() {
        // kliS + nA + hi -> kliS + Ana + hi; 6.4.105 ato heH (adesha stage)
        // then drops the hi, giving kliSAna.
        let mut p = shna_before("kliS", "hi");
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "Ana");
        assert!(p.terms[SHAP].has(Tag::Vikarana));
        assert!(!p.terms[SHAP].has(Tag::Pit)); // apit: the next 1.2.4 tags it
    }

    #[test]
    fn halah_shnah_shanac_declines_after_a_vowel_final_root() {
        // "halaH" is the whole condition. vrI is vowel-final, so it keeps SnA
        // and takes 6.4.113 instead: vrIRIhi, not *vrIRAna. This pair is the
        // rule's shape guard.
        let mut p = shna_before("vrI", "hi");
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[SHAP].text, "nA");
    }

    #[test]
    fn halah_shnah_shanac_declines_for_endings_other_than_hi() {
        // The sutra is conditioned on hi alone. A mutant dropping this would
        // rewrite the entire consonant-final paradigm as *kliSAnati.
        for ending in ["ti", "taH", "anti", "tAt"] {
            let mut p = shna_before("kliS", ending);
            let rule = rules().find(|r| r.id == "3.1.83").unwrap();
            assert!(!(rule.apply)(&mut p), "fired on {ending}");
            assert_eq!(p.terms[SHAP].text, "nA");
        }
    }

    #[test]
    fn halah_shnah_shanac_ignores_other_vikaranas_and_short_prakriyas() {
        for vikarana in ["a", "ya", ""] {
            let mut p = shna_before("kliS", "hi");
            p.terms[SHAP].text = vikarana.to_string();
            let rule = rules().find(|r| r.id == "3.1.83").unwrap();
            assert!(!(rule.apply)(&mut p), "fired on {vikarana:?}");
        }
        // A one-term prakriya must not panic indexing SHAP or ENDING.
        let mut p = Prakriya {
            terms: vec![Term::new("kliS")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "3.1.83").unwrap();
        assert!(!(rule.apply)(&mut p));
    }

    /// A laṅ prakriyā `[ANGA, SHAP, ENDING]` with the given texts. The ctx
    /// matters here: 3.4.111 is one of the few vikaraṇa-stage rules that
    /// reads the lakāra.
    fn lan_prakriya(anga: &str, shap: &str, ending: &str) -> Prakriya {
        Prakriya {
            terms: vec![Term::new(anga), Term::new(shap), Term::new(ending)],
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Bahu,
            ),
            ..Default::default()
        }
    }

    /// 3.4.111 replaces jhi with jus in laṅ after an ā — optionally, per
    /// Śākaṭāyana. The ending's text at this point is `J`, not `Ji`: 3.4.100
    /// itaś ca has already dropped the final `i` in the tiṅ stage.
    #[test]
    fn shakatayana_jus_needs_lan_a_and_jhi() {
        let rule = rules().find(|r| r.id == "3.4.111").unwrap();

        let mut p = lan_prakriya("yA", "", "J");
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "us");

        // not ā-final
        let mut p = lan_prakriya("Bava", "", "J");
        assert!(!(rule.apply)(&mut p));

        // not jhi
        let mut p = lan_prakriya("yA", "", "t");
        assert!(!(rule.apply)(&mut p));

        // a live vikaraṇa stands between the ā and the ending, so the ā is
        // not what precedes the ending — the affix-relative reading
        let mut p = lan_prakriya("yA", "a", "J");
        assert!(!(rule.apply)(&mut p));

        // kryādi's śnā vikaraṇa is itself `A`-final ("nA"), so
        // sound_before_ending alone would find the same character the aṅga
        // arm looks for — even though the DHĀTU (kliS) is a consonant-final
        // root and 3.4.110's ātaḥ has nothing to do with this cell. The
        // `SHAP.text.is_empty()` conjunct is what still declines here.
        let mut p = lan_prakriya("kliS", "nA", "J");
        assert!(!(rule.apply)(&mut p));
    }

    #[test]
    fn shnam_lands_after_the_roots_last_vowel() {
        // 1.1.47's placement, enumerated. `RuleStep` records only the word
        // before and after, so assert on the stem the step produces: a
        // suffix model would give kftnati, not kfnatti.
        //
        // √hiṃs is the row that matters: its tail is TWO consonants, so a
        // rule that assumed a one-character tail passes on kft and Kid and
        // fails only here.
        //
        // Adjusted from the brief: this cannot use `sole()`, because Task 7
        // (8.4.65) forks Kid's laṭ prathama eka cell into an optional pair.
        // Asserting the 3.1.78 step over every branch is strictly stronger
        // than asserting it on a single derivation and stays valid once that
        // fork exists.
        for (number, stem) in [
            ("07.0010", "kfnat"),
            ("07.0019", "hinans"),
            ("07.0012", "Kinad"),
        ] {
            let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
            let branches = derive(
                d,
                Lakara::Lat,
                d.pada.padas()[0],
                Purusha::Prathama,
                Vacana::Eka,
            );
            assert!(!branches.is_empty());
            for p in &branches {
                // 3.1.78 is recorded TWICE, around 1.3.9 (see the rule's
                // ORDERING comment): once for the insertion, once for the
                // 1.1.47 placement that actually produces the stem. Take the
                // LAST 3.1.78 entry, not the first — the first's `after` is
                // still Snam-shaped (e.g. kftSnamti), not the stem.
                let step = p
                    .log
                    .iter()
                    .rev()
                    .find(|s| s.sutra == "3.1.78")
                    .unwrap_or_else(|| panic!("{}: 3.1.78 never fired", d.code));
                assert!(
                    step.after.starts_with(stem),
                    "{}: expected stem {stem}, got {}",
                    d.code,
                    step.after
                );
                // The chain stays continuous and in sūtra order: 1.3.9 sits
                // strictly between the two 3.1.78 entries, not before both.
                let first = p.log.iter().position(|s| s.sutra == "3.1.78").unwrap();
                let last = p.log.iter().rposition(|s| s.sutra == "3.1.78").unwrap();
                assert!(
                    first < last,
                    "{}: expected two 3.1.78 entries (insertion, placement)",
                    d.code
                );
                assert!(
                    p.log[first + 1..last].iter().any(|s| s.sutra == "1.3.9"),
                    "{}: 1.3.9 does not fall between the two 3.1.78 entries",
                    d.code
                );
            }
        }
    }

    #[test]
    fn shnam_declines_outside_rudhadi() {
        // The guard is a gaṇa tag, not a shape test. √kliś would split
        // perfectly well after its `i`, and must not.
        for number in ["01.0001", "09.0058", "05.0016", "02.0001"] {
            let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
            let branches = derive(
                d,
                Lakara::Lat,
                d.pada.padas()[0],
                Purusha::Prathama,
                Vacana::Eka,
            );
            for p in &branches {
                assert!(
                    !p.log.iter().any(|s| s.sutra == "3.1.78"),
                    "{}: 3.1.78 fired outside rudhādi",
                    d.code
                );
            }
        }
    }
}
