//! svadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.

use crate::helpers::trace_for;

#[test]
fn apnoti_trace_pins_the_vikarana_guna() {
    // Ap prathama eka. The second 7.3.84 guṇates śnu's `u`; the first
    // declines on the root's `p`. 3.1.68 never fires -- 3.1.73 is its
    // apavāda.
    assert_eq!(
        trace_for("Apnoti"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.1.73", "1.3.9", "1.2.4", "7.3.84"
        ]
    );
}

#[test]
fn apnavani_trace_pins_the_guna_before_6_1_78_order() {
    // Ap loT uttama eka. The second 7.3.84 must run before 6.1.78 so the
    // vikaraṇa's guṇated `o` (not the root's) gets the ay-ādeśa: Apnu + Ani
    // -> Apno + Ani (7.3.84) -> Apnav + Ani (6.1.78). Ordered the other way
    // round gives *ApnoAni; run before 6.4.77/6.4.87 it would give
    // *ApnuvAni instead.
    assert_eq!(
        trace_for("ApnavAni"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.85", "3.4.89", "3.4.92", "3.1.73", "1.3.9", "1.2.4",
            "7.3.84", "6.1.78"
        ]
    );
}

#[test]
fn apnuvanti_trace_is_the_conjunct_uvang_path() {
    // Ap prathama bahu. `p` before śnu's `n` makes the `u` saṁyogapūrva, so
    // 6.4.87 (yaṇ) declines and 6.4.77 (uvaṅ) fires instead.
    assert_eq!(
        trace_for("Apnuvanti"),
        vec![
            "1.3.78", "3.4.78", "1.2.4", "3.1.73", "1.3.9", "1.2.4", "7.1.3", "6.4.77"
        ]
    );
}

#[test]
fn apnuhi_trace_is_the_conjunct_hi_luk_block() {
    // Ap loT madhyama eka. 6.4.106 declines (conjunct-preceded `u`), so `hi`
    // survives to where 6.4.101 would rewrite it -- but the nearest
    // non-empty term before the ending is śnu's `u`, not the root's jhal
    // `p`, so 6.4.101 declines too and `hi` surfaces unchanged.
    assert_eq!(
        trace_for("Apnuhi"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.85", "3.4.87", "3.1.73", "1.3.9", "1.2.4"
        ]
    );
}

#[test]
fn hinvanti_trace_is_the_asamyogapurva_yan_path() {
    // hi prathama bahu. The `i` before śnu's `n` is asaṁyogapūrva, so 6.4.87
    // (yaṇ) fires and its apavāda-ordering pre-empts 6.4.77 (uvaṅ), which
    // never runs.
    assert_eq!(
        trace_for("hinvanti"),
        vec![
            "1.3.78", "3.4.78", "1.2.4", "3.1.73", "1.3.9", "1.2.4", "7.1.3", "6.4.87"
        ]
    );
}

#[test]
fn hinu_trace_pins_the_hi_luk() {
    // hi loT madhyama eka. Asaṁyogapūrva `u` lets 6.4.106 elide `hi`
    // outright.
    assert_eq!(
        trace_for("hinu"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.4.85", "3.4.87", "3.1.73", "1.3.9", "1.2.4", "6.4.106"
        ]
    );
}

#[test]
fn rinoti_trace_ends_in_natva_over_the_new_stem() {
    // ri prathama eka: guṇa (7.3.84) creates rinoti, and ṇatva then reaches
    // across the vowel to retroflex the ending's `n` -- the tripādī rule
    // seeing the new (vikaraṇa-bearing) stem correctly.
    assert_eq!(
        trace_for("riRoti"),
        vec![
            "1.3.78", "3.4.78", "1.3.9", "3.1.73", "1.3.9", "1.2.4", "7.3.84", "8.4.2"
        ]
    );
}

#[test]
fn ashnuvate_trace_is_7_1_5_then_6_4_77() {
    // aS prathama bahu, atmanepada: 7.1.5 turns jha into ate (the aṅga is not
    // a-final), then the vowel-initial ending puts śnu's u before a vowel and
    // 6.4.77 (uvaṅ) fires -- the gaṇa's only ātmanepadī reuse of both rules.
    assert_eq!(
        trace_for("aSnuvate"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.73", "1.3.9", "1.2.4", "7.1.5", "6.4.77"
        ]
    );
}

#[test]
fn ashnushva_trace_reaches_8_3_59_through_the_vikarana() {
    // aS loT madhyama eka: TAs -> se (3.4.80) -> sva (3.4.91), then the
    // existing 8.3.59 retroflexes the s after śnu's u -- no code change, it
    // just sees the vikaraṇa's vowel instead of the root's.
    assert_eq!(
        trace_for("aSnuzva"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.85", "3.4.80", "3.4.91", "3.1.73", "1.3.9", "1.2.4",
            "8.3.59"
        ]
    );
}

#[test]
fn ashnuvita_trace_is_7_2_79_then_6_4_77_then_6_1_66() {
    // aS viDiliN prathama eka: yāsuṭ's salopa (7.2.79) leaves a vowel-initial
    // ending, so 6.4.77 (uvaṅ) fires on śnu's u instead of 7.2.80's yA -> iy
    // arm (the aṅga isn't a/A-final), and 6.1.66 then drops the resulting y
    // before the following v.
    assert_eq!(
        trace_for("aSnuvIta"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.102", "3.1.73", "1.3.9", "1.2.4", "7.2.79", "6.4.77",
            "6.1.66"
        ]
    );
}

#[test]
fn ashnave_trace_pins_the_widened_6_1_90_athematic_arm() {
    // aS loT uttama eka: the second 7.3.84 guṇates śnu to no, 6.1.78 turns
    // that into nav, and SHAP is now nav -- neither empty nor a/A-final, so
    // only the widened 6.1.90 (SHAP ends in neither a nor A) fires to absorb
    // the stranded āṭ A. One of the slice's only two witnesses for that
    // widening; the other, stiGnavE, is pinned only as a PARADIGM golden, not
    // as an ordered trace here.
    assert_eq!(
        trace_for("aSnavE"),
        vec![
            "1.3.12", "3.4.78", "1.3.9", "3.4.85", "3.4.79", "3.4.93", "3.4.92", "3.1.73", "1.3.9",
            "1.2.4", "7.3.84", "6.1.78", "6.1.90"
        ]
    );
}

#[test]
fn stighnute_trace_has_no_6_1_64_substitution() {
    // stiG prathama eka. stiG is stored post-6.1.64 dhAtvAdeH zaH saH -- no
    // rule in the engine performs that z -> s substitution, so this trace
    // (unlike aSnute's) never mentions 6.1.64; it starts directly from the
    // pada sanction on the already-s-initial root.
    assert_eq!(
        trace_for("stiGnute"),
        vec![
            "1.3.12", "3.4.78", "1.2.4", "3.4.79", "3.1.73", "1.3.9", "1.2.4"
        ]
    );
}

#[test]
fn hinvah_trace_is_hinuvah_plus_exactly_the_optional_step() {
    // The strongest available statement that these two forms are one
    // derivation that forked, rather than two derivations that happen both
    // to be listed: identical rule sequences, differing by exactly the
    // 6.4.107 step. Asserting each trace separately would pass even if the
    // two forms reached the surface by unrelated paths.
    let declined = trace_for("hinuvaH");
    let applied = trace_for("hinvaH");

    let applied_without: Vec<String> = applied
        .iter()
        .filter(|s| *s != "6.4.107")
        .cloned()
        .collect();
    assert_eq!(
        applied_without, declined,
        "hinvaH's trace must be hinuvaH's trace plus the 6.4.107 step"
    );
    assert_eq!(
        applied.iter().filter(|s| *s == "6.4.107").count(),
        1,
        "6.4.107 fires exactly once on the applied branch"
    );
    assert!(
        !declined.contains(&"6.4.107".to_string()),
        "the declined branch must not record the optional step"
    );
}

#[test]
fn ahinma_trace_shows_the_optional_lopa_after_the_augment() {
    // The laṅ witness, and a second root: 6.4.107 is in adesha, downstream
    // of 6.4.71's aṭ-āgama, and ṇatva (8.4.1/8.4.2, tripādī) still reaches
    // the elided branch — ariRma, not *arinma.
    let t = trace_for("ahinma");
    assert!(t.contains(&"6.4.107".to_string()), "got {t:?}");
    let i71 = t
        .iter()
        .position(|r| r == "6.4.71")
        .expect("6.4.71 present");
    let i107 = t
        .iter()
        .position(|r| r == "6.4.107")
        .expect("6.4.107 present");
    assert!(i71 < i107, "the aṭ-āgama precedes the optional lopa");

    let r = trace_for("ariRma");
    assert!(r.contains(&"6.4.107".to_string()), "got {r:?}");
    assert!(r.contains(&"8.4.2".to_string()), "ṇatva reaches the fork");
}

#[test]
fn apnutat_trace_shows_tatan_blocking_the_vikarana_guna() {
    // Ap loṭ prathama eka, tātaṅ branch. `tu` is pit and guṇates śnu
    // (Apnotu); tātaṅ is ṅit and 1.1.5 blocks the same application, so the
    // vikaraṇa stays `nu`. 7.3.84 must be absent entirely: the first
    // (root-relative) application never fires in svādi either.
    let t = trace_for("ApnutAt");
    assert!(t.contains(&"7.1.35".to_string()), "got {t:?}");
    assert!(!t.contains(&"7.3.84".to_string()), "got {t:?}");
}
