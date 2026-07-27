//! Aṅga operations: 6.4.71 … 7.3.101.
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
use crate::tinanta::terms::{ANGA, ENDING, SHAP, following_sarvadhatuka};
use panini_data::{Lakara, Pada};

pub(crate) static ANGA_RULES: &[Rule] = &[
    // 6.4.71 luṅlaṅlṛṅkṣvaḍudāttaḥ: the aṭ-āgama is prefixed to the aṅga in laṅ.
    //
    // Modelled as a prefix on the aṅga's text rather than as a separate term,
    // so the ANGA/SHAP/ENDING indices stay stable for every later rule. The
    // trace still cites 6.4.71, which is what the reader checks.
    Rule {
        id: "6.4.71",
        name: "luNlaNlfNkzvaqudAttaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            if !matches!(p.ctx.lakara, Lakara::Lan) || is_vowel(first) {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("a{}", p.terms[ANGA].text);
            p.record("6.4.71", "luNlaNlfNkzvaqudAttaH", before);
            true
        },
    },
    // 6.4.72 āḍ ajādīnām: vowel-initial aṅgas take the āṭ-āgama in laṅ
    // (apavāda to 6.4.71's aṭ). The A then merges with the root's initial
    // vowel by 6.1.90 āṭaś ca into vṛddhi: a+eD → ED, a+Ikz → Ekz, a+ad → Ad.
    Rule {
        id: "6.4.72",
        name: "Aq ajAdInAm",
        kind: RuleKind::Vidhi,
        apply: |p| {
            let first = p.terms[ANGA].text.chars().next().unwrap();
            // Only apply to true vowel-initial roots, not to an aṅga that
            // already carries 6.4.71's aṭ augment. 6.4.71's augment is
            // itself the character `a`, which is indistinguishable from a
            // genuinely a-initial root (√ad) by first-char alone — so check
            // whether 6.4.71 actually fired in this derivation (the trace)
            // rather than sniffing the character. Roots 6.4.71 augmented are
            // consonant-initial by its own guard, so this never double-fires;
            // genuinely a-initial roots (√ad) never trigger 6.4.71 (their
            // first char is already a vowel), so they reach here untouched
            // and correctly take āṭ.
            let already_augmented = p.log.iter().any(|s| s.sutra == "6.4.71");
            if !matches!(p.ctx.lakara, Lakara::Lan) || !is_vowel(first) || already_augmented {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text = format!("A{}", p.terms[ANGA].text);
            p.record("6.4.72", "Aq ajAdInAm", before);
            true
        },
    },
    // 7.3.100 adaH sarvezAm: √ad prefixes aṭ (`a`) to a laṅ singular
    // consonant ending (2sg s, 3sg t). Without it, Ad+s / Ad+t are word-final
    // conjuncts that 8.2.23 saṃyogāntasya lopaḥ would strip to bare Ad,
    // collapsing 2sg=3sg=1sg-stem. The inserted `a` makes the word
    // vowel-final: 8.2.23 declines, and cartva (8.4.55) skips the `d` (now
    // before `a`, not a khar) → Adat, Adas→AdaH. Guarded structurally
    // (Tag::Adadi ∧ laṅ ∧ consonant-final aṅga ∧ single-char s/t ending); in
    // the current root set that is exactly √ad, and √vas landing (5e) adds
    // no new case here (its ātmanepada endings never collapse to a bare
    // single-char ending at the point this rule runs).
    //
    // The `||`→`&&` mutant on the guard line below is killed by the
    // `akupyat_trace_shows_7_3_100_declines_for_non_adadi_roots` pin in
    // `crates/panini/tests/trace.rs`: the mutant fires for laṅ non-adādi
    // derivations and 6.1.97 repairs the surface form, so only the ordered
    // trace exposes it. (Slice 5e parked this mutant as unkillable on a case
    // analysis that slice 5f corrected.)
    Rule {
        id: "7.3.100",
        name: "adaH sarvezAm",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::Lan) || !p.terms[ANGA].has(Tag::Adadi) {
                return false;
            }
            // Consonant-final aṅga only (ā-final √yā/√vā never insert).
            let Some(anga_last) = p.terms[ANGA].text.chars().last() else {
                return false;
            };
            if is_vowel(anga_last) {
                return false;
            }
            // Single-consonant ending: 2sg `s` / 3sg `t` (not the multi-char
            // tam/tAm/ta of dual/plural).
            let e = &p.terms[ENDING].text;
            if e.chars().count() != 1 || !matches!(e.as_str(), "s" | "t") {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = format!("a{e}");
            p.record("7.3.100", "adaH sarvezAm", before);
            true
        },
    },
    // 7.1.5 ātmanepadeṣv anataḥ: in ātmanepada, the leading `J` (jh) of the
    // ending becomes `at` — not the `ant` of 7.1.3 — when the segment the
    // ending attaches to does not end in short `a`. Apavāda to 7.1.3, ordered
    // before it; 7.1.3 then declines on its own (ending no longer starts `J`).
    // The "anataḥ" test reads the last non-empty char BEFORE the ending: for a
    // thematic root that is the śap vikaraṇa `a` (rule declines → laBante); for
    // adādi √ās the śap is luk'd/empty, so it is the root-final `s` (rule fires
    // → Asate). By this point 3.4.79 has already turned `Ja` → `Je` (laṭ/loṭ),
    // so 7.1.5 strips the leading `J` and prepends `at`: Je → ate, Ja → ata,
    // JAm → atAm. First non-a-final ātmanepadī aṅga in the engine.
    Rule {
        id: "7.1.5",
        name: "AtmanepadezvanataH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.pada, Pada::Atmanepada) {
                return false;
            }
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            // "anataḥ": the segment before the ending must NOT end in short `a`.
            // Scan the terms before ENDING (skipping the luk'd/empty śap) for
            // the last non-empty char.
            let prev = p.terms[..ENDING]
                .iter()
                .rev()
                .find_map(|t| t.text.chars().last());
            let Some(prev) = prev else {
                return false;
            };
            if prev == 'a' {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("at{rest}");
            p.record("7.1.5", "AtmanepadezvanataH", before);
            true
        },
    },
    // 7.1.6 śīṅo ruṭ: the *jha* of √śī takes the ruṭ augment. 7.1.5 has just
    // replaced the ending's leading `J` with `at` (Je → ate, Ja → ata,
    // JAm → atAm); ruṭ's `r` prefixes that, giving Se + r + ate → Serate.
    //
    // Guarded on 7.1.5 having FIRED IN THIS DERIVATION rather than on the
    // ending's surface shape: the ruṭ attaches to the `at` that 7.1.5
    // produced, so that is the condition itself and not a proxy for it.
    // Reading the log for a prior rule is the idiom 6.4.72 already uses to
    // test whether 6.4.71 augmented the aṅga.
    //
    // This is why vidhiliṅ needs no special case: 3.4.105 jhasya ran has
    // already replaced the jha with `ran` far earlier in the array, so 7.1.5
    // never fires there and ruṭ cannot attach → SayIran, not *SayIraran.
    Rule {
        id: "7.1.6",
        name: "SINo ruw",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ANGA].text.ends_with("SI") || !p.log.iter().any(|s| s.sutra == "7.1.5") {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = format!("r{}", p.terms[ENDING].text);
            p.record("7.1.6", "SINo ruw", before);
            true
        },
    },
    // 7.1.3 jho'ntaḥ: a leading `J` of the ending → `ant`.
    Rule {
        id: "7.1.3",
        name: "Jo'ntaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[ENDING].text.starts_with('J') {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("ant{rest}");
            p.record("7.1.3", "Jo'ntaH", before);
            true
        },
    },
    // 7.2.79 liṅaḥ salopo 'nantyasya: the non-final s of sārvadhātuka liṅ's
    // ending is elided. yAst → yAt, yAss → yAs (madhyama-eka: only the first
    // s is non-final!), yAsus → yAus. MUST precede 7.2.80: only after the s
    // goes does the ending start with the `yA` shape 7.2.80 rewrites.
    // Every non-final s reaching this rule is yāsuṭ- or sīyuṭ-derived; the
    // invariant is that the only non-final s is āgama-initial.
    Rule {
        id: "7.2.79",
        name: "liNaH salopo'nantyasya",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin) {
                return false;
            }
            let text = &p.terms[ENDING].text;
            let n = text.chars().count();
            let reduced: String = text
                .chars()
                .enumerate()
                .filter(|&(i, c)| c != 's' || i + 1 == n)
                .map(|(_, c)| c)
                .collect();
            if reduced == *text {
                return false;
            }
            let before = p.snapshot();
            p.terms[ENDING].text = reduced;
            p.record("7.2.79", "liNaH salopo'nantyasya", before);
            true
        },
    },
    // 7.2.80 ato yeyaḥ: after an a-final aṅga (here: the śap), the yA of the
    // yāsuṭ is replaced by iy. yAt → iyt, yAus → iyus.
    Rule {
        id: "7.2.80",
        name: "ato yeyaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !matches!(p.ctx.lakara, Lakara::VidhiLin)
                || !p.terms[SHAP].text.ends_with('a')
                || !p.terms[ENDING].text.starts_with("yA")
            {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(2).collect();
            p.terms[ENDING].text = format!("iy{rest}");
            p.record("7.2.80", "ato yeyaH", before);
            true
        },
    },
    // 7.2.81 āto ṅitaḥ: after an a-final aṅga (the śap), the initial ā of a
    // ṅit ending → iy. Ate→iyte (laṭ), AtAm→iytAm (laṅ/loṭ), ATe→iyTe.
    // The ṅit condition is the TERM tag from 1.2.4 (laṭ/loṭ are ṭit lakāras,
    // yet their apit ātmanepada endings behave as ṅit) — NOT ctx.is_ngit_like.
    // The tag also keeps this rule off parasmaipada loṭ uttama's āṭ (Ani),
    // which 1.2.4 never tags (pic ca) and which belongs to 6.1.101.
    // MUST precede 6.1.101, which would otherwise dīrgha-merge the tagged
    // A-initial endings (laṭ 3du would surface as laBAte, not laBete).
    Rule {
        id: "7.2.81",
        name: "Ato NitaH",
        kind: RuleKind::Vidhi,
        apply: |p| {
            if !p.terms[SHAP].text.ends_with('a')
                || !p.terms[ENDING].has(Tag::Ngit)
                || !p.terms[ENDING].text.starts_with('A')
            {
                return false;
            }
            let before = p.snapshot();
            let rest: String = p.terms[ENDING].text.chars().skip(1).collect();
            p.terms[ENDING].text = format!("iy{rest}");
            p.record("7.2.81", "Ato NitaH", before);
            true
        },
    },
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
        apply: |p| {
            // 1.1.5 kṅiti ca, exactly as at 7.3.84 above — same follower
            // lookup, same ṅit-only narrowness.
            if following_sarvadhatuka(p).is_some_and(|t| t.has(Tag::Ngit)) {
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
    // 6.1.78 eco'yavāyāvaḥ: e/o before a vowel → ay/av. The sūtra also covers
    // E/O → Ay/Av, but those two arms are dropped here: within the current
    // 30-root × 4-lakāra grammar, ANGA can never end in a vṛddhi vowel (E/O)
    // at the point this rule runs. `vrddhi_of` (the only source of E/O in
    // this engine) is called from three places, all in 6.1.90 — the aṅga arm
    // writes the vṛddhi vowel at *position 0* of the aṅga (replacing the āṭ
    // augment + the root's first vowel), never at the aṅga's last character;
    // the other two arms write into SHAP/ENDING, not ANGA. No curated root is
    // a single SLP1 character, so the aṅga arm's tail slice is never empty
    // either. And the order is decisive on its own: 6.1.90 is the only caller
    // of `vrddhi_of`, and it runs *after* 6.1.78 in the single-pass rule
    // array, so any E/O it produces can never be seen by 6.1.78 at all. Per
    // the mutation gate's own rule (same rationale as 8.4.53's removal
    // below), unexecutable arms cannot be kept under the mutation gate.
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
        apply: |p| {
            let anga_last = p.terms[ANGA].text.chars().last().unwrap();
            let sub = match anga_last {
                'e' => "ay",
                'o' => "av",
                _ => return false,
            };
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
            // √śī vidhiliṅ 3pl: guṇa (7.4.21) has already made the aṅga `Se`,
            // and 3.4.102/7.2.79 have left the ending leading with `I`
            // (Iyran, after sīyuṭ's salopa strips the non-final `s`); this
            // arm reads only that leading `I` and turns Se + Iyran →
            // Say + Iyran. 6.1.66 (later in the array) then elides the
            // surviving `y` before the val `r` → SayIran.
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
            false
        },
    },
    // 7.3.101 ato dīrgho yañi: aṅga-final `a` (śap) → `A` before a yañ-initial
    // sārvadhātuka ending (here: mi/vas/mas).
    Rule {
        id: "7.3.101",
        name: "ato dIrGo yaYi",
        kind: RuleKind::Vidhi,
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
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::term::Term;
    use crate::tinanta::form_g;
    use crate::tinanta::rules;
    use panini_data::{Purusha, Vacana};

    #[test]
    fn salopa_elides_only_the_non_final_s() {
        // Madhyama-eka is the trap: yAs + s = yAss, and only the FIRST s is
        // non-final. Eliding both would derive *Bave for BaveH.
        for (ending, want) in [("yAst", "yAt"), ("yAss", "yAs"), ("yAsus", "yAus")] {
            let mut p = Prakriya {
                terms: vec![Term::new("Bav"), Term::new("a"), Term::new(ending)],
                log: vec![],
                ctx: Context::new(
                    Lakara::VidhiLin,
                    Pada::Parasmaipada,
                    Purusha::Prathama,
                    Vacana::Eka,
                ),
                blocked: false,
            };
            let rule = rules().find(|r| r.id == "7.2.79").unwrap();
            assert!((rule.apply)(&mut p), "{ending}");
            assert_eq!(p.terms[ENDING].text, want, "{ending}");
        }
    }

    #[test]
    fn ato_yeyah_rewrites_the_ya_prefix_after_shap_a() {
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
        assert!((rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "iyt");
    }

    #[test]
    fn ato_yeyah_requires_vidhilin_lakara() {
        // shap == "a" and ending starts_with "yA" are both satisfied, but the
        // lakara isn't vidhilin: the guard's first `||` must still short-
        // circuit to false. Kills the `||` -> `&&` mutant at the first
        // operator, which would otherwise let this fire whenever the other
        // two conditions hold regardless of lakara.
        let mut p = Prakriya {
            terms: vec![Term::new("Bav"), Term::new("a"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lan,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAt");
    }

    #[test]
    fn ato_yeyah_requires_shap_a() {
        // lakara is vidhilin and ending starts_with "yA", but shap isn't
        // "a": the guard's second `||` must still short-circuit to false.
        // Kills the `||` -> `&&` mutant at the second operator, which would
        // otherwise let this fire whenever lakara is vidhilin regardless of
        // shap.
        let mut p = Prakriya {
            terms: vec![Term::new("i"), Term::new("i"), Term::new("yAt")],
            log: vec![],
            ctx: Context::new(
                Lakara::VidhiLin,
                Pada::Parasmaipada,
                Purusha::Prathama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.80").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "yAt");
    }

    #[test]
    fn ato_nitah_requires_the_ngit_tag() {
        // Parasmaipada lot uttama Ani starts with A but is NOT Nid-vat
        // (1.2.4 pic-ca exclusion) — it belongs to 6.1.101, not 7.2.81.
        let mut anga = Term::new("Bav");
        anga.add(Tag::Anga);
        let mut p = Prakriya {
            terms: vec![anga, Term::new("a"), Term::new("Ani")],
            log: vec![],
            ctx: Context::new(
                Lakara::Lot,
                Pada::Parasmaipada,
                Purusha::Uttama,
                Vacana::Eka,
            ),
            blocked: false,
        };
        let rule = rules().find(|r| r.id == "7.2.81").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "Ani");
    }

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
        // Mutating `chars[n - 2]` (line 806) to `chars[n / 2]` would guNa
        // 'i' instead ("e"), yielding "aBier". Mutating `chars[..n - 2]`
        // (line 815) to `chars[..n / 2]` would prefix with "aB" instead
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
    fn shings_guna_leaves_every_other_adadi_root_alone() {
        // 7.4.21 is root-specific. The other five adādi roots must be
        // untouched by it: their finals (`A`, `d`, `s`) are outside the guard,
        // and their shipped forms are the proof.
        assert_eq!(
            form_g("yA", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "yAti"
        );
        assert_eq!(
            form_g("ad", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "atti"
        );
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
            "Aste"
        );
        assert_eq!(
            form_g("vas", Lakara::Lat, Purusha::Prathama, Vacana::Eka),
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

    #[test]
    fn rut_requires_both_shing_and_a_fired_seven_one_five() {
        // Both clauses of 7.1.6's guard must hold. Dropping either one is a
        // live mutant, and each half is pinned here.
        //
        // (a) 7.1.5 fired, but the aṅga is √ās, not √śī: no ruṭ (Asate, not
        //     *Asrate). This is the clause an `||` → `&&` mutant drops.
        assert_eq!(
            form_g("As", Lakara::Lat, Purusha::Prathama, Vacana::Bahu),
            "Asate"
        );
        // (b) The aṅga IS √śī, but 7.1.5 never fired (empty log): the rule
        //     must decline and leave the ending untouched.
        let mut p = Prakriya {
            terms: vec![Term::new("SI"), Term::new(""), Term::new("ate")],
            log: vec![],
            ..Default::default()
        };
        let rule = rules().find(|r| r.id == "7.1.6").unwrap();
        assert!(!(rule.apply)(&mut p));
        assert_eq!(p.terms[ENDING].text, "ate");
    }
}
