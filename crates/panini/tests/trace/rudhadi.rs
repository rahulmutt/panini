//! rudhadi's ordered-trace witnesses. Helpers live in
//! `crate::helpers`; the module doc governing this suite is in
//! `main.rs`.

use crate::helpers::{at, cell_trace, trace_for};
use panini::Panini;
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

#[test]
fn krnatti_trace_shows_the_infix_then_natva() {
    // 3.1.78 splits the root; 8.4.1 then fires across the ANGA/SHAP
    // junction it created, exactly as it does for kryādi's vf + nA.
    let t = trace_for("kfRatti");
    assert!(at(&t, "3.1.78") < at(&t, "8.4.1"), "got {t:?}");
}

#[test]
fn hindi_trace_shows_dhi_ca_bleeding_jashtva() {
    // 6.4.101 rewrites the ending BEFORE 6.4.111 strips śnam's `a`; 8.2.25
    // then ELIDES the stem-final `s` rather than voicing it, which is why
    // this cell reaches no 8.4.53 where its sibling kfndDi does.
    let t = trace_for("hinDi");
    assert!(at(&t, "6.4.101") < at(&t, "6.4.111"), "got {t:?}");
    assert!(at(&t, "6.4.111") < at(&t, "8.2.25"), "got {t:?}");
    assert!(!t.contains(&"8.4.53".to_string()), "got {t:?}");
}

#[test]
fn krntat_trace_shows_savarna_elision_above_pausal() {
    // Three optional rules on one branch. The reverse of 8.4.65 / 8.4.56
    // derives kfnttAt and kfntAd but never this form, so the surface alone
    // does not catch it.
    let t = trace_for("kfntAt");
    assert!(at(&t, "7.1.35") < at(&t, "8.4.65"), "got {t:?}");
    assert!(at(&t, "8.4.65") < at(&t, "8.4.56"), "got {t:?}");
}

#[test]
fn ahinah_trace_shows_ru_fires_on_the_dhatus_own_final() {
    // 8.2.74 must act on `ahinas`. Below 8.2.73 it would find `ahinad` and
    // this branch would not exist at all.
    let t = trace_for("ahinaH");
    assert!(t.contains(&"8.2.74".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.73".to_string()), "got {t:?}");
}

#[test]
fn kndhi_trace_shows_jashtva_where_dhi_ca_declines() {
    // The counterpart to hinDi: kft's stem-final `t` is not an `s`, so
    // 8.2.25 declines and the junction is genuinely 8.4.53's.
    let t = trace_for("kfndDi");
    assert!(t.contains(&"8.4.53".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.2.25".to_string()), "got {t:?}");
}

#[test]
fn apinaq_trace_pins_8_2_23_above_8_2_41() {
    // THE LOAD-BEARING NEW ORDER in this slice, and BOTH laṅ eka cells
    // must be checked, not just one: `apinaq` is what prathama eka AND
    // madhyama eka both reduce to, and `trace_for`'s `.find()` would
    // silently settle for whichever analysis `candidates()` enumerates
    // first — prathama eka, whose ending is a bare `t`. That derivation
    // never presents an `s` for 8.2.41 ṣaḍhoḥ kaḥ si to see at all, so a
    // pin built on `trace_for` alone would hold "8.2.23 present, 8.2.41
    // absent" vacuously, for a reason unrelated to the order this pin
    // exists to guard, and would keep passing even after that order broke.
    // Go around `trace_for` and inspect every analysis `apinaq` produces
    // instead, asserting the COUNT along with each trace's content: at
    // laṅ madhyama eka the ending IS a bare `s`, and 8.2.23 saṃyogāntasya
    // lopaḥ elides it — as the second member of a word-final conjunct —
    // before 8.2.41 ever runs, so 8.2.41 finds no trigger and declines,
    // and the cell reduces exactly as laṅ prathama eka does. Reversed,
    // 8.2.41 would fire on the still-live `z`/`s` pair before the `s` is
    // elided; madhyama eka would surface `apinak` instead — a real-word-
    // looking form that no guard test would flag — and drop out of this
    // check entirely, so the analysis count below would no longer be 2.
    let r = Panini::new().check("apinaq");
    assert_eq!(r.analyses.len(), 2, "both laṅ eka cells reduce to apinaq");
    for a in &r.analyses {
        let t: Vec<String> = a.trace.iter().map(|s| s.sutra.clone()).collect();
        assert!(t.contains(&"8.2.23".to_string()), "got {t:?}");
        assert!(!t.contains(&"8.2.41".to_string()), "got {t:?}");
    }
}

#[test]
fn bhanakti_trace_shows_8_2_30_then_8_4_55() {
    // Banaj + ti -> Banag + ti (8.2.30 coḥ kuḥ turns the root-final `j`
    // into its ku counterpart `g` before the jhal `t`) -> Banakti (8.4.55
    // khari ca then devoices that `g` to `k` before the same khar `t`).
    let t = trace_for("Banakti");
    assert!(at(&t, "8.2.30") < at(&t, "8.4.55"), "got {t:?}");
}

#[test]
fn indhe_trace_shows_8_2_40_then_8_4_53() {
    // inD + te -> inD + De (8.2.40 jhaṣas tathor dho'dhaḥ turns the
    // ending's `t` into `D` after the stem's jhaṣ), then the anusvāra
    // round trip runs across it: iMDDe (8.3.24 naścāpadāntasya jhali
    // turns śnam's `n` into an anusvāra before the jhaṣ `D`) -> iMdDe
    // (8.4.53 jhalāṁ jaś jhaśi voices the stem's own `D` to `d` before
    // that `D`) -> indDe (8.4.58 anusvārasya yayi parasavarṇaḥ turns the
    // anusvāra back into the homorganic `n`).
    let t = trace_for("indDe");
    assert!(at(&t, "8.2.40") < at(&t, "8.4.53"), "got {t:?}");
}

#[test]
fn pinakshi_trace_shows_8_2_41_then_8_3_59() {
    // pinaz + si -> pinak + si (8.2.41 ṣaḍhoḥ kaḥ si turns the root-final
    // `z` into `k` before the `s`) -> pinakzi (8.3.59 ādeśapratyayayoḥ
    // retroflexes that same `s` back to `z` after the new `k`, a ku
    // sound).
    let t = trace_for("pinakzi");
    assert!(at(&t, "8.2.41") < at(&t, "8.3.59"), "got {t:?}");
}

#[test]
fn pimzwah_trace_is_the_round_trips_second_witness() {
    // 8.3.24 naścāpadāntasya jhali turns the śnam infix's `n` into an
    // anusvāra `M` before the jhal `z`. The return leg (8.4.58) DECLINES
    // here — its trigger needs a yay to follow, but what follows the
    // anusvāra is the root's own `z`, which is śal — so the anusvāra
    // survives to the surface in piMzwaH. √hiṃs's hiMstaH was the first
    // witness that 8.3.24/8.4.58 are not a no-op pair, in 7a; this is the
    // second.
    let t = trace_for("piMzwaH");
    assert!(t.contains(&"8.3.24".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.58".to_string()), "got {t:?}");
}

#[test]
fn pindhi_trace_is_the_full_8_4_41_chain() {
    // The deepest new chain in the slice, on top of the anusvāra 8.3.24
    // already gave śnam's `n`: piMzDi -> piMzQi (8.4.41 ṣṭunā ṣṭuḥ
    // retroflexes the ending's `D` to `Q` after the stem's `z`) -> piMqQi
    // (8.4.53 jhalāṁ jaś jhaśi voices that `z` to `q` before the new `Q`)
    // -> piRqQi (8.4.58 anusvārasya yayi parasavarṇaḥ turns the anusvāra
    // into the homorganic `R` before `Q`, completing this cell's declined
    // form) -> piRQi, this cell's alternate: optional 8.4.65 jharo jhari
    // savarṇe elides the penultimate `q` before the savarṇa `Q`.
    let t = trace_for("piRQi");
    assert!(at(&t, "8.4.41") < at(&t, "8.4.53"), "got {t:?}");
    assert!(at(&t, "8.4.53") < at(&t, "8.4.58"), "got {t:?}");
    assert!(at(&t, "8.4.58") < at(&t, "8.4.65"), "got {t:?}");
}

#[test]
fn rudh_lat_prathama_eka_credits_both_pada_sutras() {
    // One root, one cell, two padas -- the sharpest available test of the
    // whole three-rule pada-sanction table. ruD is Ubhayapadin: parasmaipada
    // `ruRadDi` must be sanctioned by 1.3.78 (which now DECLINES on the
    // ātmanepada arm rather than blocking, since the aṅga is Ubhayapadin),
    // and ātmanepada `rundDe` must be sanctioned by 1.3.72 svaritañitaḥ
    // instead. The negative halves are the load-bearing assertions: they are
    // what would catch 1.3.78 blocking outright instead of declining (which
    // would silently drop the ātmanepada cell entirely), or 1.3.72 firing on
    // the parasmaipada arm it must not touch. This single pair pins the
    // entire three-rule table (1.3.12 / 1.3.72 / 1.3.78) at once.
    let parasmaipada = trace_for("ruRadDi");
    assert!(
        parasmaipada.contains(&"1.3.78".to_string()),
        "got {parasmaipada:?}"
    );
    assert!(
        !parasmaipada.contains(&"1.3.72".to_string()),
        "got {parasmaipada:?}"
    );

    let atmanepada = trace_for("rundDe");
    assert!(
        atmanepada.contains(&"1.3.72".to_string()),
        "got {atmanepada:?}"
    );
    assert!(
        !atmanepada.contains(&"1.3.78".to_string()),
        "got {atmanepada:?}"
    );
}

#[test]
fn rudh_natva_follows_stem_strength_not_pada() {
    // The claim that √rudh needs no new phonology rests on this, so pin it
    // directly. `ruRadDi`'s trigger `r` and target `n` are separated by the
    // aṭ vowel `u`, so 8.4.2 (not the adjacent-trigger 8.4.1) fires.
    let ruraddhi = trace_for("ruRadDi");
    assert!(ruraddhi.contains(&"8.4.2".to_string()), "got {ruraddhi:?}");
    assert!(!ruraddhi.contains(&"8.4.1".to_string()), "got {ruraddhi:?}");

    // `ruRaDE` is what makes the test's name honest. Both witnesses above
    // and below are parasmaipada, so on their own they cannot distinguish
    // "ṇatva follows stem strength" from "ṇatva follows pada". This one is
    // ĀTMANEPADA (loṭ uttama eka, reached through 1.3.72, not 1.3.78) and
    // strong-stemmed, and ṇatva fires there too -- so the split really is
    // strong vs. weak, not pada vs. pada.
    let runadhai = trace_for("ruRaDE");
    assert!(runadhai.contains(&"1.3.72".to_string()), "got {runadhai:?}");
    assert!(runadhai.contains(&"8.4.2".to_string()), "got {runadhai:?}");

    // `runDanti` is the weak-stem witness: PARASMAIPADA like `ruRadDi`, but
    // with no ṇ at all (the `n` stays dental). 6.4.111 śnasor allopaḥ elides
    // śnam's `a`, which leaves the nasal directly before the jhal `D`, and
    // 8.3.24 naścāpadāntasya jhali -- a real rule here, gaṇa-guarded to
    // rudhādi and ordered above ṇatva in the tripādī -- then turns it into
    // an anusvāra. By the time 8.4.1 / 8.4.2 look there is no `n` left to
    // retroflex, so `is_natva_target` declines on the character itself, not
    // on its folded-8.3.24 jhal clause. That clause's own witness is
    // BAzante (√bhāṣ is not rudhādi, so the real 8.3.24 never fires for
    // it), pinned directly by
    // `natva_declines_before_a_jhal_because_8_3_24_bleeds_it` in
    // `tripadi.rs`; nothing in √rudh's paradigm exercises it -- `anti`'s
    // surviving `n` is the only one the clause still rejects here, and the
    // backward scan would break on the `D` before it anyway.
    let rundanti = trace_for("runDanti");
    assert!(!rundanti.contains(&"8.4.1".to_string()), "got {rundanti:?}");
    assert!(!rundanti.contains(&"8.4.2".to_string()), "got {rundanti:?}");
}

#[test]
fn runde_is_ambiguous_within_atmanepada() {
    // `runDe` is a genuine same-pada ambiguity: the appendix lists it in
    // both the ātmanepada laṭ prathama eka cell (as `rundDe`'s optional
    // 8.4.65 alternate) and the ātmanepada laṭ uttama eka cell (as that
    // cell's own base form). No existing root in this suite produces two
    // analyses that share BOTH lakāra and pada, only differing in purusha /
    // vacana -- every prior multi-analysis pin (e.g. `bhavatu_forks_...`,
    // `apinaq_trace_pins_...`) splits across padas, optional-rule forks, or
    // different cells that happen to reduce to the same surface form. This
    // exercises the multi-analysis path against that harder case.
    use panini_data::{Lakara, Pada, Purusha, Vacana};

    let r = Panini::new().check("runDe");
    assert_eq!(r.analyses.len(), 2, "exactly two ātmanepada readings");
    let cells: Vec<(Lakara, Pada, Purusha, Vacana)> = r
        .analyses
        .iter()
        .map(|a| (a.lakara, a.pada, a.purusha, a.vacana))
        .collect();
    assert!(
        cells.contains(&(
            Lakara::Lat,
            Pada::Atmanepada,
            Purusha::Prathama,
            Vacana::Eka
        )),
        "expected an ātmanepada laṭ prathama eka analysis, got {cells:?}"
    );
    assert!(
        cells.contains(&(Lakara::Lat, Pada::Atmanepada, Purusha::Uttama, Vacana::Eka)),
        "expected an ātmanepada laṭ uttama eka analysis, got {cells:?}"
    );
}

#[test]
fn kshud_natva_is_the_intervening_arm_under_a_sibilant_trigger() {
    // 07.0006 kzu\di~^r. The strong stem's trigger is the `z` of `kz`, the
    // target is Snam's `n`, and the root's own aw vowel `u` separates them,
    // so this is 8.4.2 awkupvANnumvyavAye'pi and NOT the adjacent 8.4.1.
    // That is √rudh's shape (ruRadDi, r-u-n) reached through a sibilant
    // rather than an r — 8.4.2's other curated witnesses (vrIRAti, muzARa)
    // are kryādi, where 8.3.24 never competes.
    let (strong, t) = cell_trace(
        "07.0006",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert!(t.contains(&"8.4.2".to_string()), "{strong}: got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "{strong}: got {t:?}");

    // The WEAK stem takes no ṇatva at all: 6.4.111 SnasorallopaH elides
    // Snam's `a`, leaving the nasal directly before a jhal, and 8.3.24
    // naScApadAntasya Jali — gaṇa-guarded to rudhādi and ordered above
    // ṇatva in the tripādī — turns it into an anusvāra before either ṇatva
    // rule looks. Same bleed √rudh shows at runDanti.
    let (weak, t) = cell_trace(
        "07.0006",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert!(!t.contains(&"8.4.1".to_string()), "{weak}: got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "{weak}: got {t:?}");
    assert!(t.contains(&"8.3.24".to_string()), "{weak}: got {t:?}");

    // An ĀTMANEPADA strong-stem cell, sanctioned by 1.3.72 rather than
    // 1.3.78, still retroflexes. Without this the two pins above could not
    // tell "ṇatva follows stem strength" from "ṇatva follows pada" — the
    // same reason rudh_natva_follows_stem_strength_not_pada includes ruRaDE.
    let (atma, t) = cell_trace(
        "07.0006",
        Lakara::Lot,
        Pada::Atmanepada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert!(t.contains(&"1.3.72".to_string()), "{atma}: got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "{atma}: got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "{atma}: got {t:?}");
}

#[test]
fn trd_natva_is_the_adjacent_arm_through_an_r_vowel_trigger() {
    // 07.0009 u~tfdi~^r. Structurally √kṛt, NOT √kṣud: the trigger `f` sits
    // directly against Snam's `n` with nothing intervening, so this is the
    // adjacent 8.4.1 razAByAM no RaH and not 8.4.2. It leans on
    // is_natva_trigger's `f | F` arm — the r-vowels counting as triggers by
    // 1.1.51 uraR raparaH, which until now existed for kryādi's √vṛ.
    let (strong, t) = cell_trace(
        "07.0009",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert!(t.contains(&"8.4.1".to_string()), "{strong}: got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "{strong}: got {t:?}");

    // Weak stem: 8.3.24 bleeds ṇatva, exactly as for √kṣud and √rudh.
    let (weak, t) = cell_trace(
        "07.0009",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Bahu,
    );
    assert!(!t.contains(&"8.4.1".to_string()), "{weak}: got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "{weak}: got {t:?}");
    assert!(t.contains(&"8.3.24".to_string()), "{weak}: got {t:?}");

    // Ātmanepada strong stem, sanctioned by 1.3.72: still retroflexes.
    let (atma, t) = cell_trace(
        "07.0009",
        Lakara::Lot,
        Pada::Atmanepada,
        Purusha::Uttama,
        Vacana::Eka,
    );
    assert!(t.contains(&"1.3.72".to_string()), "{atma}: got {t:?}");
    assert!(t.contains(&"8.4.1".to_string()), "{atma}: got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "{atma}: got {t:?}");
}

#[test]
fn rinakti_trace_reaches_k_in_one_step() {
    // THE pin that distinguishes a correct 8.2.30 from an accidentally
    // correct one. √bhañj's `j` takes the VOICED velar `g` and needs 8.4.55
    // Kari ca to devoice it afterwards -- that two-step path is pinned by
    // bhanakti_trace_shows_8_2_30_then_8_4_55 above. √ric's `c` is already
    // voiceless, so 1.1.50's nearest velar IS `k` and 8.2.30 reaches it in
    // one step, leaving 8.4.55 nothing to do.
    //
    // A substitute hardcoded to 'g' would produce riRagti here and let
    // 8.4.55 devoice it to the same riRakti surface. Every paradigm golden
    // would still pass; only this test fails. Do not weaken it to a
    // presence check on 8.2.30.
    let (text, t) = cell_trace(
        "07.0004",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "riRakti", "got {t:?}");
    assert!(t.contains(&"8.2.30".to_string()), "got {t:?}");
    assert!(
        !t.contains(&"8.4.55".to_string()),
        "8.4.55 must not fire: {t:?}"
    );
}

#[test]
fn rinakti_trace_takes_intervening_natva() {
    // r, then the aw vowel i, then śnam's n -> 8.4.2, not 8.4.1. Structurally
    // √rudh's arm (ruRadDi) reached through an `r` trigger, inside the one
    // gaṇa where 8.3.24 naScApadAntasya Jali competes on the weak stem.
    let (text, t) = cell_trace(
        "07.0004",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "riRakti", "got {t:?}");
    assert!(t.contains(&"8.4.2".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
}

#[test]
fn vinakti_trace_takes_no_natva_at_all() {
    // The minimal contrast to √ric: same gaṇa, same c-final shape, same
    // vikaraṇa, same 8.2.30 application -- and no r/z/f trigger anywhere, so
    // śnam's `n` stays dental. A NEGATIVE pin, and the point of the pair:
    // it is what stops a widened ṇatva guard passing unnoticed.
    let (text, t) = cell_trace(
        "07.0005",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "vinakti", "got {t:?}");
    assert!(t.contains(&"8.2.30".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.2".to_string()), "got {t:?}");
}

#[test]
fn unantas_trace_orders_6_4_23_before_6_4_111() {
    // und laT prathama dvi. The ORDER pin 6.4.23's own comment in anga.rs
    // asks for and nothing asserted until this slice: 6.4.23 SnAnnalopaH
    // takes the root's `n` out of unand, and only then does 6.4.111
    // SnasorallopaH take śnam's `a`. Reversed, 6.4.111 fires first and
    // 6.4.23 can no longer tell śnam's `n` from the root's.
    //
    // This is also where vidyut-prakriya credits 6.4.24 aniditAM hala
    // upaDAyAH kNiti for the same unad -> und step. It is the wrong credit
    // -- 6.4.24 deletes a nasal upadhā, and after 6.4.23 has run, unad's
    // upadhā is `a` -- so this engine does not implement 6.4.24 at all and
    // must not be "corrected" toward vidyut's history here.
    let (_text, t) = cell_trace(
        "07.0020",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert!(at(&t, "6.4.23") < at(&t, "6.4.111"), "got {t:?}");
    assert!(!t.contains(&"6.4.24".to_string()), "got {t:?}");
}

#[test]
fn aunat_trace_takes_the_u_vrddhi_arm() {
    // und laN prathama eka: AT (6.4.72) then 6.1.90 AwaS ca on a
    // vowel-initial aNga whose first vowel is `u`, so vrddhi_of returns
    // `O`. Every curated root before this slice drove 6.1.90 with e/I/E
    // only -- sound.rs's vrddhi_of_ac_vowels_all_arms says so in as many
    // words -- and this is the first golden derivation to reach the `u`
    // arm. A vrddhi table that mapped `u` to anything else would still
    // pass every unit test in sound.rs and fail here.
    let (text, t) = cell_trace(
        "07.0020",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "Onad", "got {t:?}");
    assert!(at(&t, "6.4.72") < at(&t, "6.1.90"), "got {t:?}");
}

#[test]
fn anaktas_trace_is_the_kutva_path_on_a_vowel_initial_root() {
    // aYj laT prathama dvi: 6.4.23 thins the root's nasal to anaj, 6.4.111
    // takes śnam's `a` to anj, 8.2.30 coH kuH substitutes the velar for the
    // `j`, 8.3.24 nasalises to aMg, 8.4.55 Kari ca devoices to aMk, and
    // 8.4.58 anusvArasya yayi parasavarRaH gives the velar nasal: aNktaH.
    // The whole tripAdi tail on a vowel-initial root, in order.
    let (text, t) = cell_trace(
        "07.0021",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Dvi,
    );
    assert_eq!(text, "aNktaH", "got {t:?}");
    assert!(at(&t, "6.4.23") < at(&t, "6.4.111"), "got {t:?}");
    assert!(at(&t, "6.4.111") < at(&t, "8.2.30"), "got {t:?}");
    assert!(at(&t, "8.2.30") < at(&t, "8.3.24"), "got {t:?}");
    assert!(at(&t, "8.3.24") < at(&t, "8.4.55"), "got {t:?}");
    assert!(at(&t, "8.4.55") < at(&t, "8.4.58"), "got {t:?}");
}

#[test]
fn trneddhi_trace_puts_8_3_13_below_8_4_41() {
    // tfh laT prathama eka, the whole im path in one cell: 7.3.92 inserts
    // the Agama, 6.1.87 coalesces it (tfnaih -> tfneh), 8.2.31 takes the
    // `h` to `Q`, 8.2.40 takes ti's `t` to `D`, 8.4.41 retroflexes that
    // `D` to `Q`, and only THEN can 8.3.13 elide the first of the two.
    //
    // The order assertion is the point. 8.3.13's second ḍh is 8.4.41's own
    // output, so in sūtra order the rule would see tfneQ + Di, decline,
    // and the cell would surface *tfReQQi.
    let (text, t) = cell_trace(
        "07.0018",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "tfReQi", "got {t:?}");
    assert!(at(&t, "7.3.92") < at(&t, "6.1.87"), "got {t:?}");
    assert!(at(&t, "8.2.31") < at(&t, "8.2.40"), "got {t:?}");
    assert!(at(&t, "8.4.41") < at(&t, "8.3.13"), "got {t:?}");
}

#[test]
fn trnaddhi_trace_has_8_3_13_and_no_8_4_65() {
    // tfh loT madhyama eka. Every other stop-final rudhAdi root makes this
    // cell a SIX-former: 8.4.53 voices, 8.4.65 Jaro Jari savarRe optionally
    // elides, and 7.1.35 and 8.4.56 multiply that by three. √tṛh's holds
    // three forms, because 8.3.13 obligatorily eats the very ḍh 8.4.65
    // would have forked on.
    //
    // The negative half is the pin: move 8.3.13 below 8.4.65 in
    // `tripadi.rs` and this cell silently grows to six forms, every one of
    // them a plausible word. The ALTERNATES count is the second alarm; this
    // is the one that says why.
    let (text, t) = cell_trace(
        "07.0018",
        Lakara::Lot,
        Pada::Parasmaipada,
        Purusha::Madhyama,
        Vacana::Eka,
    );
    assert_eq!(text, "tfRQi", "got {t:?}");
    assert!(t.contains(&"8.3.13".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.65".to_string()), "got {t:?}");
    // 6.4.101 huJalByo her DiH supplies the `Di` that 8.4.41 retroflexes;
    // without it there is no second ḍh and 8.3.13 has nothing to elide.
    assert!(at(&t, "6.4.101") < at(&t, "8.4.41"), "got {t:?}");
}

#[test]
fn atrned_trace_takes_the_im_before_8_2_23_eats_tips_t() {
    // tfh laN prathama eka. A cross-STAGE ordering fact that nothing else
    // records: 7.3.92 lives in the `guna` stage and 8.2.23 in `tripadi`,
    // so when 7.3.92 asks whether the following affix is hal-initial, laN
    // tip's apRkta `t` is STILL THERE. Let 8.2.23 saMyogAntasya lopaH run
    // first and ENDING is empty, the hal test fails, and the cell derives
    // *atfRah.
    //
    // vidyut-prakriya credits 6.1.68 hal NyAb Byo dIrGAt sutisyapfktaM hal
    // with that same deletion, here and for every curated rudhAdi root
    // (akfRat, aBinat, apinaw, aBanak). This engine has no 6.1.68 and
    // reaches the same surface by 8.2.23; the divergence predates √tṛh and
    // is audited clean, so it is not this slice's to correct.
    let (text, t) = cell_trace(
        "07.0018",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "atfReq", "got {t:?}");
    assert!(at(&t, "7.3.92") < at(&t, "8.2.23"), "got {t:?}");
    assert!(!t.contains(&"6.1.68".to_string()), "got {t:?}");
}

#[test]
fn acchinat_trace_orders_the_tuk_between_the_augment_and_shcutva() {
    // Cid laN prathama eka. The slice's central ordering fact, and it
    // spans three stages: 6.4.71 is in `anga`, 6.1.73 immediately below it
    // in the same stage, and 8.4.40 in `tripadi`. Each link is load-bearing
    // in a different way.
    //
    // 6.4.71 < 6.1.73: the aT-augment IS the short vowel 6.1.73 attaches
    // to. Run 6.1.73 first and the `C` is word-initial, the guard declines,
    // and the cell surfaces *aCinat.
    //
    // 6.1.73 < 8.4.40: the tuk IS the stu that Scutva palatalizes. Without
    // it there is nothing before the `C` for 8.4.40 to read, and the cell
    // surfaces *aCinat again -- by a different route, which is why both
    // links are pinned rather than just the surface.
    //
    // 8.4.56 vA'vasAne is optional, and `cell_trace` reads the DECLINED
    // branch (see its doc comment above): `acCinad` is definitionally the
    // reading on which 8.4.56 did not fire, so this cell can pin only its
    // absence, not its position relative to 8.4.40. The position itself --
    // that 8.4.56 sits last in the pipeline, below 8.4.40, whenever it DOES
    // fire -- is pinned exactly and more strongly by
    // `tinanta_rule_order_is_pinned` in
    // `crates/panini-prakriya/src/tinanta/derivation_tests.rs`, which
    // asserts the full flattened rule array those two ids sit in. A
    // relative-order check on one declined trace would only ever be a
    // weaker echo of that array; it would never catch an ordering
    // regression the array pin doesn't already catch, and it can't even be
    // written here in the first place. What CAN be written here, and is a
    // real invariant of this branch, is that a declined vikalpa rule
    // records no step at all -- so 8.4.56 must be absent from this trace,
    // and its absence is what this cell checks.
    let (text, t) = cell_trace(
        "07.0003",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "acCinad", "got {t:?}");
    assert!(at(&t, "6.4.71") < at(&t, "6.1.73"), "got {t:?}");
    assert!(at(&t, "6.1.73") < at(&t, "8.4.40"), "got {t:?}");
    assert!(!t.contains(&"8.4.56".to_string()), "got {t:?}");
}

#[test]
fn acchrnat_trace_runs_natva_and_shcutva_on_disjoint_sites() {
    // Cfd laN prathama eka -- a cell in the corpus that reaches both
    // Natva and Scutva (√chfd's laN parasmaipada PARADIGM block runs the
    // same pair at prathama, madhyama and uttama eka -- acCfRad, acCfRad
    // and acCfRadam all carry the R and are all laN -- so three cells reach
    // both; this is simply the cheapest of the three to pin). They touch
    // different characters of the same word:
    // 8.4.1 rewrites the `n` of Cfnad, whose trigger is the root's own `f`
    // directly before it, while 8.4.40 rewrites the tuk sitting IN FRONT of
    // that `f`.
    //
    // The negative half is the pin. If the tuk were ever placed between the
    // `f` and the `n`, 8.4.2's intervener test would decide the cell
    // instead -- `t` is not an aT member, so Natva would be blocked and the
    // cell would surface *acCfnad. That this test asserts both rules fired
    // is what says the tuk did not land there.
    let (text, t) = cell_trace(
        "07.0008",
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "acCfRad", "got {t:?}");
    assert!(t.contains(&"8.4.1".to_string()), "got {t:?}");
    assert!(t.contains(&"8.4.40".to_string()), "got {t:?}");
    assert!(at(&t, "6.1.73") < at(&t, "8.4.40"), "got {t:?}");
}

#[test]
fn chinatti_trace_cites_neither_new_sutra() {
    // Cid laT prathama eka. Both new sutras are laN-only, and for one
    // reason: outside laN there is no aT-augment, so the root's `C` is
    // word-initial and 6.1.73 has no short vowel to attach the tuk to.
    // 8.4.40 then has no stu to read.
    //
    // This is the cheapest guard against 6.1.73's `&&` collapsing to `||`
    // on the scan below: that mutant no longer requires the matched char to
    // be `C`, so it fires the moment it sees ANY hrasva vowel one position
    // back. For Cid that is always the root's own `i` at word index 1, so
    // every one of these 54 non-laN cells -- these are √chid's alone, 3
    // lakaras x 2 padas x 9 puruSa-vacana combinations; √chRd is a separate
    // count -- would grow a spurious `t` one slot later than the rule ever
    // places one (`Citnatti`, not `Cinatti`). The `is_hrasva` conjunct
    // itself is pinned elsewhere, by
    // `che_ca_inserts_tuk_only_after_a_short_vowel`'s dirgha `ACi` case in
    // this file's test module.
    let (text, t) = cell_trace(
        "07.0003",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "Cinatti", "got {t:?}");
    assert!(!t.contains(&"6.1.73".to_string()), "got {t:?}");
    assert!(!t.contains(&"8.4.40".to_string()), "got {t:?}");
}

#[test]
fn acchinat_has_exactly_two_forms() {
    // Cid laN prathama eka holds acCinad and acCinat, and nothing else --
    // the 8.4.56 vA'vasAne fork alone.
    //
    // The pin is that 8.4.65 Jaro Jari savarRe does NOT also fire. After
    // 8.4.40 the word carries `c` followed by `C`: same sthana, same
    // abhyantara prayatna, so savarna jhars, and the sutra read bare would
    // optionally elide the `c` and give a third form *aCinat. It declines
    // because 8.4.65 carries 8.4.64 halo yamAM yami lopaH's *halaH* by
    // anuvrtti -- implemented as `!is_vowel(w[i - 1])` -- and the sound
    // before that `c` is the aT-augment's own `a`.
    //
    // No previously curated root could put a savarna jhar pair directly
    // after a vowel, so this is the first cell to exercise that guard in
    // the direction that proves it necessary. Weaken it and the ALTERNATES
    // count is the second alarm; this is the one that says why.
    let d = dhatus()
        .iter()
        .find(|d| d.dhatupatha == "07.0003")
        .expect("07.0003 is curated");
    let forms: Vec<String> = derive(
        d,
        Lakara::Lan,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    )
    .iter()
    .filter(|p| !p.blocked)
    .map(|p| p.text())
    .collect();
    assert_eq!(forms, vec!["acCinad", "acCinat"], "got {forms:?}");
}

#[test]
fn bhunkte_trace_credits_1_3_66_not_1_3_72() {
    // Buj laT ātmanepada prathama eka — the slice's one new fact, read off
    // the trace. √bhuj's ātmanepada is sanctioned by the root-keyed 1.3.66
    // Bujo'navane, and the pada sanction opens every trace, so 1.3.66 is
    // at index 0. 1.3.72 must NOT appear: the root carries Tag::Anavane,
    // not Ubhayapadin, precisely so the trace cannot credit a svarita/ñit
    // sanction that `Bu\ja~`'s upadeśa does not carry. 1.3.78 must not
    // appear either — it declined this reading rather than blocking it.
    let (text, t) = cell_trace(
        "07.0017",
        Lakara::Lat,
        Pada::Atmanepada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "BuNkte", "got {t:?}");
    assert_eq!(
        at(&t, "1.3.66"),
        0,
        "the pada sanction opens the trace, got {t:?}"
    );
    assert!(!t.contains(&"1.3.72".to_string()), "got {t:?}");
    assert!(!t.contains(&"1.3.78".to_string()), "got {t:?}");
}

#[test]
fn bhunakti_trace_credits_the_shesa_1_3_78() {
    // The arm an UNCONDITIONAL 1.3.66 must leave open: the engine models
    // no sense, so the avane reading derives too, sanctioned by 1.3.78's
    // śeṣa exactly as for 1.3.72's roots — which is what "1.3.66 declines
    // rather than blocks" buys. 1.3.66 must not appear in this trace: it
    // declined without recording.
    let (text, t) = cell_trace(
        "07.0017",
        Lakara::Lat,
        Pada::Parasmaipada,
        Purusha::Prathama,
        Vacana::Eka,
    );
    assert_eq!(text, "Bunakti", "got {t:?}");
    assert_eq!(
        at(&t, "1.3.78"),
        0,
        "the pada sanction opens the trace, got {t:?}"
    );
    assert!(!t.contains(&"1.3.66".to_string()), "got {t:?}");
    assert!(!t.contains(&"1.3.72".to_string()), "got {t:?}");
}
