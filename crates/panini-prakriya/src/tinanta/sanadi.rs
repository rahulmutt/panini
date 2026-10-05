//! The sanādi stage: ṇic and its folding into the dhātu — 3.1.25, 3.1.28's
//! āya where no ṇic is taken, ṇic's it-lopa (1.3.9), 3.4.114, 6.4.48,
//! 6.1.54, 7.2.116, the vārttika 7.3.37.2, 7.3.36, 7.2.115, 6.1.78, 6.4.92,
//! 7.2.114, 6.1.73, 7.3.86, 3.1.32 — opened by the dhātupāṭha gaṇasūtra
//! 10.0493, which credits a jñapādi root's mit-tva, then eight vikalpas that
//! fork a root whose ṇic is optional into its ṇic and ṇic-less branches (the
//! gaṇasūtras 10.0498 and 10.0499 and the Kaumudī's 2564, 2565, 2570, 2571,
//! 2573.1, 2573.3), a ninth (2573.2) that forks `pata`'s ṇic branch on its
//! final `a`, and two more gaṇasūtras, 10.0496 and 10.0497, which settle an
//! ākusmīya or ā-garvīya root's pada, all before ṇic is added.
//!
//! First in the pipeline, before any lakāra or tiṅ exists. The layout here
//! is `[AGAMA, ABHYASA, ANGA, ṇic]`, ṇic (or 3.1.28's āya) at `NIC`; 3.1.32
//! folds it into `ANGA` and removes it, so `super::samjna` starts on the
//! same `[AGAMA, ABHYASA, ANGA]` every other gaṇa does. From there on a
//! ṇijanta aṅga is an ordinary i-final dhātu (`cori`), and 7.3.84 then
//! 6.1.78 make `coray-` exactly as they make √nī's `nay-`; an āya aṅga is
//! a-final (`DUpAya`) and meets śap as an adanta root does. See
//! `super::terms`.
//!
//! Every rule self-guards: the eight optional-ṇic vikalpas on the row's
//! `OPTIONAL_NIC` entry, 2573.2 on `pata`'s, 10.0496 on `Tag::Akusmiya`,
//! 10.0497 on `Tag::AaGarviya`, 10.0493 on `Tag::Mit`, 3.1.25 on
//! `Tag::Curadi`, 3.1.28 on `Tag::Aya` with no ṇic taken, 6.1.73 on its own
//! saṁhitā condition, and the rest on the pratyaya at `NIC`: 3.4.114 and
//! 3.1.32 on ṇic or āya, 6.4.48, 7.2.114 and 7.3.86 on its being
//! ārdhadhātuka, the others on ṇic's ṇit (6.4.48 on an `a`-final aṅga as
//! well, 6.1.54 on √ci's row (`super::samjna::CISPHUR`), 6.4.92 on
//! `Tag::Mit`, 7.3.37.2 on √dhū's and √prī's text, 7.3.36 on an `A`-final
//! aṅga, 7.2.115 on an ac-final one, 6.1.78 on an ec-final one, 7.2.114 on
//! `Tag::Mrj`, and 7.2.116 and 7.3.86 decline on 6.4.48's `Tag::AtLopa`).
//! 6.1.73 and 3.1.28 carry no gaṇa guard, only their own conditions (3.1.28
//! reads the āya rows); on today's corpus the gaṇas 1–9 add nothing and
//! record nothing, and 6.1.73 fires only on √vich and √pich (slice 10k),
//! which `the_10i_aya_and_tuk_fire_only_on_their_rows` pins.

use crate::prakriya::Prakriya;
use crate::rule::{Rule, RuleKind};
use crate::term::{Tag, Term};
use crate::tinanta::anga::che_ca;
use crate::tinanta::samjna::CISPHUR;
use crate::tinanta::sound::{guna_of, hrasva_of, vrddhi_of};
use crate::tinanta::terms::{ANGA, NIC};
use panini_data::{Pada, optional_nic};

/// The ṇic-less branch of a root whose ṇic is optional: fires only when
/// `id` is the row's `OPTIONAL_NIC` entry. The root's ṇic-branch pada tag
/// goes, because ṇic is what 1.3.74, 10.0496 and 10.0497 hang on: with it
/// gone they decline on their own guards, and 1.3.78 finds a genuine śeṣa
/// (parasmaipada; ātmanepada blocks) — unless the row is
/// `PadaAssignment::NicUbhayapada`, whose svarita or ñit upadeśa makes the
/// ṇic-less branch 1.3.72's: `Tag::NicUbhayapada` adds `Tag::Ubhayapadin`
/// here, and 1.3.72 sanctions the ātmanepada. 3.1.25 is barred by the
/// caller's `bars`, so no ṇic is ever added and every ṇic-reading rule below
/// declines. Changes no text.
fn skip_nic(p: &mut Prakriya, id: &'static str, name: &'static str) -> bool {
    if optional_nic(p.ctx.dhatupatha) != Some(id) {
        return false;
    }
    let before = p.snapshot();
    for tag in [Tag::Nic, Tag::Akusmiya, Tag::AaGarviya] {
        p.terms[ANGA].remove(tag);
    }
    if p.terms[ANGA].has(Tag::NicUbhayapada) {
        p.terms[ANGA].add(Tag::Ubhayapadin);
    }
    p.record(id, name, before);
    true
}

/// The sanādi pratyaya at `NIC`, once it is ready to fold into the dhātu:
/// ṇic after its it-lopa (ṇit, `Tag::Rit`), or 3.1.28's āya, which has no it
/// to lose. Read by 3.4.114 and 3.1.32, the two rules both pratyayas meet.
/// āya is known by its text, as 1.3.9 knows ṇic by `Ric`.
fn sanadi_pratyaya(p: &Prakriya) -> Option<&Term> {
    p.terms
        .get(NIC)
        .filter(|t| t.has(Tag::Rit) || t.text == "Aya")
}

pub(crate) static SANADI: &[Rule] = &[
    // 10.0493, the gaṇasūtra closing the jñapādi (`10.0118 jYapa~` …
    // `10.0124 ciY`): these roots are mit. The verdict is `Tag::Mit`, which
    // `super::derive` sets from the data layer's `JNAPADI` range as it sets
    // `Tag::Ghu`; this entry credits it and changes no text, as 10.0496
    // credits `Tag::Akusmiya`. First in the stage because vidyut-prakriya
    // tags a jñapādi root mit there, as soon as the dhātu is identified and
    // before any optional-ṇic fork (`dhatu_karya.rs`): √ci (`10.0124`) is mit
    // and its ṇic is optional (2570), so 10.0493 is credited on both its
    // branches, ahead of 2570. 6.4.92 below is what the tag feeds.
    //
    // Not here: 01.0934 (am-final roots are mit) and 10.0494 *nānye mito
    // 'hetau* (no other curādi root is mit, outside the causative). With no
    // causative in this engine, 01.0934 could reach only curādi am-final
    // roots, where 10.0494 always blocks it: neither could fire or fail a
    // test. Both wait for a causative slice.
    Rule {
        id: "10.0493",
        name: "jYapAdayo mitaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mit) {
                return false;
            }
            let before = p.snapshot();
            p.record("10.0493", "jYapAdayo mitaH", before);
            true
        },
    },
    // 10.0498 ā dhṛṣād vā: the ādhṛṣīya — the curādi rows from `10.0338
    // yu\ja~` to `10.0388 Dfza~` — take ṇic optionally (*yojayati* /
    // *yojati*). A dhātupāṭha gaṇasūtra, numbered as vidyut-prakriya numbers
    // it (`DP("10.0498")`), like 10.0496 and 10.0497. First of the eight
    // optional-ṇic vikalpas because vidyut checks the ādhṛṣīya before idit or
    // udit (`dhatu_karya.rs`): `hisi~` and `kaWi~` are idit and `tanu~` and
    // `mfjU~` udit, and all four are 10.0498's. Keyed, like 2564 below, on
    // the row's `OPTIONAL_NIC` entry; the declined branch is the ṇic one and
    // stays index 0.
    Rule {
        id: "10.0498",
        name: "A DfzAd vA",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "10.0498", "A DfzAd vA"),
    },
    // 10.0499 ā svadaḥ sakarmakāt: the āsvadīya — the curādi rows from
    // `10.0279 grasa~` to `10.0337 svAda~` — take ṇic optionally
    // (*grāsayati* / *grasati*). A dhātupāṭha gaṇasūtra, numbered as
    // vidyut-prakriya numbers it (`DP("10.0499")`), like 10.0498. Its
    // *sakarmakāt* (only where the root takes an object) is not modelled:
    // vidyut makes ṇic optional on every row of the range, and so does this
    // engine. Second, right after 10.0498, because vidyut checks both
    // antargaṇas before any marker (`dhatu_karya.rs`): the idit `tuji~`,
    // `ahi~` and the rest, the udit `vftu~` and `vfDu~` and the īdit
    // `pUrI~` are all 10.0499's. Keyed on the row's `OPTIONAL_NIC` entry.
    Rule {
        id: "10.0499",
        name: "A svadaH sakarmakAt",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "10.0499", "A svadaH sakarmakAt"),
    },
    // Kaumudī 2564: an idit curādi root takes ṇic optionally (*cintayati* /
    // *cintati*). One of the eight vikalpa rules that decide ṇic before the
    // pada gaṇasūtras, as vidyut-prakriya does ("First decide Ric-pratyaya,
    // since this affects the scope of AkusmIya"): an ākusmīya root is
    // ātmanepadī only on its ṇic branch. Keyed on the row's `OPTIONAL_NIC`
    // entry, since idit-ness is a marker `code` no longer carries. The
    // declined branch is the ṇic one and stays index 0. Not a sūtra of the
    // Aṣṭādhyāyī: numbered as vidyut numbers it (`Kaumudi("2564")`).
    Rule {
        id: "2564",
        name: "iditkaraRaM RicaH pAkzikatve liNgam",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2564", "iditkaraRaM RicaH pAkzikatve liNgam"),
    },
    // Kaumudī 2565: `10.0022 pF`'s ṇic is optional (*pārayati* / *parati*).
    // The Kaumudī reads the long vowel of its upadeśa as the sign, as 2564
    // reads the idit marker. vidyut-prakriya keys it on an F-final root,
    // after idit (`dhatu_karya.rs`); `10.0346 jF` is ādhṛṣīya, so 10.0498
    // takes it first, and `pF` is the one row 2565 reaches. Keyed on the
    // row's `OPTIONAL_NIC` entry.
    Rule {
        id: "2565",
        name: "dIrGoccAraRaM RicaH pAkzikatve liNgam",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2565", "dIrGoccAraRaM RicaH pAkzikatve liNgam"),
    },
    // Kaumudī 2570: a ñit or udit curādi root takes ṇic optionally
    // (*vañcayate* / *vañcati*). 2564's twin, for its own rows.
    Rule {
        id: "2570",
        name: "YitkaraRasAmarTyAdasya RijvikalpaH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2570", "YitkaraRasAmarTyAdasya RijvikalpaH"),
    },
    // Kaumudī 2571: `10.0251 Guzi~r`'s ṇic is not nitya (*ghoṣayati* /
    // *ghoṣati*). The Kaumudī reads that off the exclusion in 7.2.23
    // *ghuṣir aviśabdane* (*niṣedhāl liṅgāt*), as 2564 reads the idit
    // marker. vidyut-prakriya keys it on this one upadeśa. Its `i~r` is the
    // irit marker, not idit, so 2564 never reaches it. Keyed on the row's
    // `OPTIONAL_NIC` entry.
    Rule {
        id: "2571",
        name: "nizeDAlliNgAdanityo'sya Ric",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2571", "nizeDAlliNgAdanityo'sya Ric"),
    },
    // Kaumudī 2573.1: `10.0400 pata` takes ṇic optionally (*patati*). Its
    // ṇic branch forks again at 2573.2 below.
    Rule {
        id: "2573.1",
        name: "vA RijantaH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2573.1", "vA RijantaH"),
    },
    // Kaumudī 2573.3: `mUtra`, `katra` and `garva` take ṇic optionally
    // (*mūtrati*, *garvati*). vidyut-prakriya keys it on those upadeśas (and
    // `karta`, which has no curādi row), not on their shape.
    Rule {
        id: "2573.3",
        name: "adantatvasAmarTyARRijvikalpaH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &["3.1.25", "2573.2"],
        apply: |p| skip_nic(p, "2573.3", "adantatvasAmarTyARRijvikalpaH"),
    },
    // Kaumudī 2573.2: on `pata`'s ṇic branch, some treat the root as not
    // adanta: its final `a` goes before ṇic exists, so 6.4.48 finds none and
    // sets no `Tag::AtLopa`, and 7.2.116 lengthens the upadhā (*pātayati*).
    // Declined, 6.4.48 deletes the `a` and 7.2.116 declines (*patayati*).
    // The ṇic-less branch bars it (2573.1's `bars`), as vidyut tries it only
    // once ṇic is taken.
    Rule {
        id: "2573.2",
        name: "vA'danta ityeke",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| {
            if optional_nic(p.ctx.dhatupatha) != Some("2573.1") {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('a') else {
                return false;
            };
            let stem = stem.to_string();
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.record("2573.2", "vA'danta ityeke", before);
            true
        },
    },
    // 10.0496 ā kusmād ātmanepadinaḥ: the curādi roots up to kusm are
    // ātmanepadī — the data layer's `AKUSMIYA` range, carried here as
    // `Tag::Akusmiya`. A gaṇasūtra of the dhātupāṭha, not a sūtra of the
    // Aṣṭādhyāyī, and the first such id in this engine: numbered as
    // vidyut-prakriya numbers it (`DP("10.0496")`), by its position in the
    // dhātupāṭha. Rule ids are opaque strings everywhere they are read.
    //
    // After the optional-ṇic vikalpas and before 3.1.25, because vidyut
    // credits it there, once ṇic is decided. It settles the pada outright, so
    // no pada sūtra in `super::samjna` is credited after it: 1.3.12, 1.3.66,
    // 1.3.72 and 1.3.74 decline on their own guards, 1.3.78 on this tag. The
    // wrong pada BLOCKS, as it does under 1.3.12 — derivation, not the
    // analyzer, is the source of truth for pada.
    Rule {
        id: "10.0496",
        name: "A kusmAd AtmanepadinaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Akusmiya) {
                return false;
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("10.0496", "A kusmAd AtmanepadinaH", before);
                    true
                }
                Pada::Parasmaipada => {
                    p.blocked = true;
                    false
                }
            }
        },
    },
    // 10.0497 ā garvād ātmanepadinaḥ: the curādi roots from `10.0440 pada` up
    // to `10.0449 garva` are ātmanepadī — the data layer's `AA_GARVIYA`
    // range, carried here as `Tag::AaGarviya`. 10.0496's twin in every
    // respect but the range: it settles the pada outright, so no pada sūtra
    // in `super::samjna` is credited after it, and the parasmaipada branch
    // BLOCKS. vidyut-prakriya credits it where it credits 10.0496, before
    // 3.1.25. The range includes `10.0449 garva`, whose ṇic is optional
    // (Kaumudī 2573.3): the gaṇasūtra applies only on its ṇic branch, and on
    // the ṇic-less one 2573.3 has removed `Tag::AaGarviya`, so this declines.
    Rule {
        id: "10.0497",
        name: "A garvAd AtmanepadinaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::AaGarviya) {
                return false;
            }
            match p.ctx.pada {
                Pada::Atmanepada => {
                    let before = p.snapshot();
                    p.record("10.0497", "A garvAd AtmanepadinaH", before);
                    true
                }
                Pada::Parasmaipada => {
                    p.blocked = true;
                    false
                }
            }
        },
    },
    // 3.1.25 satyāpapāśarūpavīṇātūlaślokasenālomatvacavarmavarṇacūrṇa-
    // curādibhyo ṇic: ṇic after a curādi root. The sūtra's nominal bases
    // (satya, pāśa, …) are not in scope; the gaṇa is.
    Rule {
        id: "3.1.25",
        name: "satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Curadi) {
                return false;
            }
            let before = p.snapshot();
            let mut nic = Term::new("Ric");
            nic.add(Tag::Pratyaya);
            p.terms.push(nic);
            p.record(
                "3.1.25",
                "satyApapASarUpavIRAtUlaSlokasenAlomatvacavarmavarRacUrRacurAdiByo Ric",
                before,
            );
            true
        },
    },
    // 3.1.28 gupūdhūpavicchipaṇipanibhya āyaḥ: √dhūp and √vich take the
    // pratyaya āya (*dhūpāyati*, *vicchāyati*). Only where ṇic was not
    // taken: on the ṇic branch 3.1.25 has put ṇic at `NIC`, and this
    // declines. vidyut-prakriya adds āya at this point, after its
    // optional-ṇic fork (trace `10.0499 → 3.1.28 → 3.4.114 → 3.1.32`).
    // Guarded on `Tag::Aya` (`panini_data::AYA`): a stored code cannot
    // decide it. Obligatory here: 3.1.31 *āyādaya ārdhadhātuke vā* makes
    // it optional only before an ārdhadhātuka ending, and no ārdhadhātuka
    // lakāra is in scope. āya has no it, so 1.3.9 passes it by; 3.4.114
    // makes it ārdhadhātuka and 3.1.32 folds it into the dhātu (`DUpAya`).
    Rule {
        id: "3.1.28",
        name: "gupUDUpavicCipaRipaniBya AyaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Aya) || p.terms.get(NIC).is_some() {
                return false;
            }
            let before = p.snapshot();
            let mut aya = Term::new("Aya");
            aya.add(Tag::Pratyaya);
            p.terms.push(aya);
            p.record("3.1.28", "gupUDUpavicCipaRipaniBya AyaH", before);
            true
        },
    },
    // 1.3.9 tasya lopaḥ, on ṇic: 1.3.7 cuṭū makes the initial ṇ an it, 1.3.3
    // halantyam the final c, and both go, leaving `i`. Local to ṇic, not
    // `it_samjna::run_it_samjna`: that helper has no 1.3.7 arm, and a
    // general one would strip the jh of the tiṅ ending `Ji`, which the
    // tradition exempts. The ṇ is recorded as `Tag::Rit` for 7.2.116. One
    // `1.3.9` step, the engine's it-lopa trace convention.
    Rule {
        id: "1.3.9",
        name: "tasya lopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            let Some(nic) = p.terms.get(NIC).filter(|t| t.text == "Ric") else {
                return false;
            };
            let before = p.snapshot();
            let mut nic = nic.clone();
            nic.text = "i".to_string();
            nic.add(Tag::Rit);
            p.terms[NIC] = nic;
            p.record("1.3.9", "tasya lopaH", before);
            true
        },
    },
    // 3.4.114 ārdhadhātukaṃ śeṣaḥ: ṇic and āya are neither tiṅ nor śit, so
    // they are ārdhadhātuka. Read by 7.3.86 below, whose
    // *sārvadhātukārdhadhātukayoḥ* is inherited from 7.3.84.
    Rule {
        id: "3.4.114",
        name: "ArDaDAtukaM SezaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if sanadi_pratyaya(p).is_none() {
                return false;
            }
            let before = p.snapshot();
            p.terms[NIC].add(Tag::Ardhadhatuka);
            p.record("3.4.114", "ArDaDAtukaM SezaH", before);
            true
        },
    },
    // 6.4.48 ato lopaḥ ārdhadhātuke: an aṅga's final `a` is deleted before an
    // ārdhadhātuka affix. The adanta curādi roots (`10.0389 kaTa` …) meet it
    // here, before ṇic: `kaTa` → `kaT`. vidyut-prakriya credits it at this
    // very point, after 3.4.114 and before 3.1.32.
    //
    // The lopa is not the end of the `a`. By 1.1.57 acaḥ parasmin
    // pūrvavidhau, a vowel replaced because of what follows still stands for
    // a rule about what precedes it: 7.2.116 and 7.3.86 below would read
    // `kaT`'s `a` and `kuh`'s `u` as the upadhā, but the deleted `a` is still
    // there for them, so *kathayati*, not *kāthayati*, and *kuhayate*, not
    // *kohayate*. `Tag::AtLopa` carries that, and both decline on it. 1.1.57
    // is a paribhāṣā and is not credited, as vidyut does not credit it. The
    // tag outlives 3.1.32, for later stages that must know the `a` was there.
    Rule {
        id: "6.4.48",
        name: "ato lopaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka)) {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('a') else {
                return false;
            };
            let stem = stem.to_string();
            let before = p.snapshot();
            p.terms[ANGA].text = stem;
            p.terms[ANGA].add(Tag::AtLopa);
            p.record("6.4.48", "ato lopaH", before);
            true
        },
    },
    // 6.1.54 cisphuror ṇau: before ṇic, √ci's `i` optionally becomes `A`
    // (`ci` → `cA`). Taken, 7.3.36 below adds puk (`cAp`) and 6.4.92 shortens
    // it (*capayati*); declined, 7.2.115 and 6.1.78 make `cAy` and 6.4.92
    // shortens that (*cayayati*). vidyut-prakriya credits it at this point,
    // right after 3.4.114. Keyed on the row number (`super::samjna::CISPHUR`),
    // as vidyut keys it on the upadeśa `ciY`: curādi `10.0325 ci` stores as
    // the same `ci` and takes no `A`. The sūtra's `sphur` meets ṇic only in a
    // causative.
    Rule {
        id: "6.1.54",
        name: "cisPuror RO",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| {
            if !CISPHUR.contains(&p.ctx.dhatupatha)
                || !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit))
            {
                return false;
            }
            let Some(stem) = p.terms[ANGA].text.strip_suffix('i') else {
                return false;
            };
            let text = format!("{stem}A");
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("6.1.54", "cisPuror RO", before);
            true
        },
    },
    // 7.2.116 ata upadhāyāḥ: vṛddhi of an `a` upadhā before a ñit or ṇit
    // affix. `laq` → `lAq` before ṇic. Only ṇit has a carrier in this engine
    // (`Tag::Rit`); no ñit affix is in scope.
    Rule {
        id: "7.2.116",
        name: "ata upaDAyAH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            // 1.1.57: 6.4.48's deleted `a` still stands; see 6.4.48 above.
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) || p.terms[ANGA].has(Tag::AtLopa)
            {
                return false;
            }
            let mut chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || chars[n - 2] != 'a' {
                return false;
            }
            let before = p.snapshot();
            chars[n - 2] = 'A';
            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("7.2.116", "ata upaDAyAH", before);
            true
        },
    },
    // Vārttika 7.3.37.2 dhūñprīñor nug vaktavyaḥ: √dhū and √prī take the
    // augment nuk before ṇic (*dhūnayati*, *prīṇayati*; 8.4.2 makes the ṇ).
    // Optional, as vidyut-prakriya applies it (per Haradatta, on prīñ in the
    // Siddhānta-kaumudī), and numbered as vidyut numbers it
    // (`Varttika("7.3.37.2")`): the first vārttika id in this engine. Before
    // 7.2.115, so the nuk branch leaves the aṅga no final ac to lengthen;
    // declined, 7.2.115 gives *dhāvayati*, *prāyayati*. nuk is kit, so it is
    // the aṅga's final part (1.1.46 ādyantau ṭakitau): its `n` is appended to
    // the aṅga's text, its it-lopa unrecorded, as no rule reads the augment
    // as a term. Keyed on the root's text, as vidyut keys it: in this stage
    // the aṅga is still the bare root.
    Rule {
        id: "7.3.37.2",
        name: "DUYprIYor nug vaktavyaH",
        kind: RuleKind::Vidhi,
        vikalpa: true,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit))
                || !matches!(p.terms[ANGA].text.as_str(), "DU" | "prI")
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text.push('n');
            p.record("7.3.37.2", "DUYprIYor nug vaktavyaH", before);
            true
        },
    },
    // 7.3.36 artihrīvlīrīknūyīkṣmāyyātāṃ puk ṇau: an ā-final aṅga takes the
    // augment puk before ṇic (*jñāpayati*; √ci's 6.1.54 branch `cA` →
    // `cAp`). puk is kit, so, like 7.3.37.2's nuk above, its `p` is appended
    // to the aṅga's text (1.1.46 ādyantau ṭakitau), its it-lopa unrecorded.
    // vidyut-prakriya inserts a term; one here, between `ANGA` and `NIC`,
    // would move ṇic off the index every rule in this stage reads. Before
    // 7.2.115: `vrddhi_of` maps `A` to itself, so on a bare `jYA` 7.2.115
    // would record a step vidyut does not credit; after puk it finds the `p`
    // and declines. The sūtra's named roots (`f`, `hrI`, `vlI`, `rI`,
    // `knUy`, `kzmAy`) meet ṇic only in a causative, so the ā-final arm is
    // the one here.
    Rule {
        id: "7.3.36",
        name: "artihrIvlIrIknUyIkzmAyyAtAM pug RO",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit))
                || !p.terms[ANGA].text.ends_with('A')
            {
                return false;
            }
            let before = p.snapshot();
            p.terms[ANGA].text.push('p');
            p.record("7.3.36", "artihrIvlIrIknUyIkzmAyyAtAM pug RO", before);
            true
        },
    },
    // 7.2.115 aco ñṇiti: vṛddhi of an ac-final aṅga before a ñit or ṇit
    // affix. Before ṇic: `lI` → `lE`, `BU` → `BO`, `vf` → `vAr`, `jF` →
    // `jAr` (1.1.51 uraṇ raparaḥ). Only ṇit has a carrier in this engine
    // (`Tag::Rit`), as for 7.2.116. vidyut-prakriya credits it here, before
    // 3.1.32. An `a`-final root never reaches it: 6.4.48 has deleted the `a`.
    // Nor does an `A`-final one: 7.3.36 has added its `p`.
    Rule {
        id: "7.2.115",
        name: "aco YRiti",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut text = p.terms[ANGA].text.clone();
            let Some(v) = text.pop().and_then(vrddhi_of) else {
                return false;
            };
            let before = p.snapshot();
            text.push_str(v);
            p.terms[ANGA].text = text;
            p.record("7.2.115", "aco YRiti", before);
            true
        },
    },
    // 6.1.78 eco 'yavāyāvaḥ, before ṇic's `i`: 7.2.115's `E` and `O` become
    // `Ay` and `Av` (`lE` → `lAy`, `BO` → `BAv`) while ṇic is still its own
    // term, where vidyut-prakriya credits it, before 3.1.32 folds ṇic in. A
    // second entry under the guṇa stage's id: that one reads the aṅga against
    // a vikaraṇa or an ending, never against ṇic, so the two never meet.
    Rule {
        id: "6.1.78",
        name: "eco'yavAyAvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut text = p.terms[ANGA].text.clone();
            let sub = match text.pop() {
                Some('e') => "ay",
                Some('o') => "av",
                Some('E') => "Ay",
                Some('O') => "Av",
                _ => return false,
            };
            let before = p.snapshot();
            text.push_str(sub);
            p.terms[ANGA].text = text;
            p.record("6.1.78", "eco'yavAyAvaH", before);
            true
        },
    },
    // 6.4.92 mitāṃ hrasvaḥ: a mit root's upadhā is shortened before ṇi.
    // `jYAp` → `jYap`: it undoes 7.2.116's vṛddhi, so √jñap makes
    // *jñapayati*, not *jñāpayati*. On √ci it undoes 7.2.115's and 6.1.78's
    // `cAy` (*cayayati*) and 6.1.54's and 7.3.36's `cAp` (*capayati*), so it
    // sits after all of them. Guarded on `Tag::Mit` (10.0493's verdict) and
    // on ṇit ṇic; a short upadhā declines.
    //
    // vidyut-prakriya credits 6.4.92 later, among its asiddhavat rules, after
    // 7.3.84 guṇates ṇic's `i`. It sits here, while root and ṇic are still
    // separate terms and the upadhā is one character read; after 3.1.32 the
    // vowel would be inside `jYApe`. The forms agree for every root curated
    // here: the upadhā-readers after it, 7.2.114 and 7.3.86, need √mṛj or a
    // laghu ik, and decline on the six `a` roots and on `cay` and `cap`.
    Rule {
        id: "6.4.92",
        name: "mitAM hrasvaH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mit) || !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Rit)) {
                return false;
            }
            let mut chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let Some(upadha) = chars.len().checked_sub(2) else {
                return false;
            };
            let Some(short) = hrasva_of(chars[upadha]) else {
                return false;
            };
            let before = p.snapshot();
            chars[upadha] = short;
            p.terms[ANGA].text = chars.into_iter().collect();
            p.record("6.4.92", "mitAM hrasvaH", before);
            true
        },
    },
    // 7.2.114 mṛjer vṛddhiḥ: √mṛj's ik takes vṛddhi where guṇa would come.
    // Before ārdhadhātuka ṇic, `mfj` → `mArj`, ahead of 7.3.86, which then
    // finds no laghu ik (*mārjayati*). Guarded on `Tag::Mrj`
    // (`super::samjna::MRJ`); the ṇic-less branch is the guṇa stage's entry.
    Rule {
        id: "7.2.114",
        name: "mfjer vfdDiH",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if !p.terms[ANGA].has(Tag::Mrj)
                || !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka))
            {
                return false;
            }
            let text = &p.terms[ANGA].text;
            let Some(i) = text.find(['i', 'u', 'f', 'x']) else {
                return false;
            };
            let v = vrddhi_of(text[i..].chars().next().unwrap()).unwrap();
            let text = format!("{}{v}{}", &text[..i], &text[i + 1..]);
            let before = p.snapshot();
            p.terms[ANGA].text = text;
            p.record("7.2.114", "mfjer vfdDiH", before);
            true
        },
    },
    // 6.1.73 che ca, before ṇic or āya: √vich's `i` (√pich's too, slice 10k)
    // takes tuk here, before the sanādi 7.3.86 can read it as a laghu upadhā (`viC` →
    // `vitC`, so *vicchayati*, not *vechayati*; 8.4.40 later makes the `t`
    // a `c`). vidyut-prakriya credits it at this point ("tuk-Agama can block
    // guna", `angasya.rs`). A second entry under the aṅga stage's id, with
    // that entry's very apply (`super::anga::che_ca`): the whole-word scan,
    // not an aṅga-local copy, guarded on nothing but the sūtra's own
    // condition. On √vich's laṅ cells the aṅga stage's entry then finds the
    // `C` after the `t` and declines.
    Rule {
        id: "6.1.73",
        name: "Ce ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: che_ca,
    },
    // 7.3.86 pugantalaghūpadhasya ca, before ṇic: guṇa of a laghu ik upadhā
    // before an ārdhadhātuka. `cur` → `cor`. A third entry under this id —
    // the guṇa-stage pair (nitya, and tanādi's vikalpa) reads the aṅga
    // before the vikaraṇa, by which point a curādi aṅga is the vowel-final
    // `cori` and their "final-vowel aṅgas are 7.3.84's business" early
    // return declines. This entry is the one occasion that sees the root
    // with ṇic beside it.
    Rule {
        id: "7.3.86",
        name: "pugantalaGUpaDasya ca",
        kind: RuleKind::Vidhi,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            // 1.1.57: 6.4.48's deleted `a` still stands; see 6.4.48 above.
            if !p.terms.get(NIC).is_some_and(|t| t.has(Tag::Ardhadhatuka))
                || p.terms[ANGA].has(Tag::AtLopa)
            {
                return false;
            }
            let chars: Vec<char> = p.terms[ANGA].text.chars().collect();
            let n = chars.len();
            if n < 2 || !matches!(chars[n - 2], 'i' | 'u' | 'f' | 'x') {
                return false;
            }
            let g = guna_of(chars[n - 2]).expect("an ik vowel has a guṇa");
            let before = p.snapshot();
            let mut s: String = chars[..n - 2].iter().collect();
            s.push_str(g);
            s.push(chars[n - 1]);
            p.terms[ANGA].text = s;
            p.record("7.3.86", "pugantalaGUpaDasya ca", before);
            true
        },
    },
    // 3.1.32 sanādyantā dhātavaḥ: root + ṇic, or root + āya, is a dhātu.
    // Fold the pratyaya's text into `ANGA` and remove its term, so every
    // later stage sees the three-slot layout and an i-final (or, with āya,
    // a-final) dhātu. `Tag::Dhatu` stays; `Tag::Nijanta` records that the
    // final `i` is ṇic's, so an āya aṅga does not get it.
    Rule {
        id: "3.1.32",
        name: "sanAdyantA DAtavaH",
        kind: RuleKind::Samjna,
        vikalpa: false,
        bars: &[],
        apply: |p| {
            if sanadi_pratyaya(p).is_none() {
                return false;
            }
            let before = p.snapshot();
            let pratyaya = p.terms.remove(NIC);
            p.terms[ANGA].text.push_str(&pratyaya.text);
            if pratyaya.has(Tag::Rit) {
                p.terms[ANGA].add(Tag::Nijanta);
            }
            p.record("3.1.32", "sanAdyantA DAtavaH", before);
            true
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use crate::prakriya::Prakriya;
    use crate::tinanta::terms::with_slots;
    use panini_data::{Lakara, Purusha, Vacana};

    /// The sanādi stage's own entry for `id` — looked up in `SANADI`, not
    /// `rules()`, so it can never be another stage's 1.3.9 or 7.3.86.
    fn rule(id: &str) -> &'static Rule {
        SANADI.iter().find(|r| r.id == id).unwrap()
    }

    /// `root`, as a curādi dhātu, alone or followed by ṇic as it stands
    /// after its it-lopa (`i`, ṇit), optionally already ārdhadhātuka.
    fn with_nic(root: &str, nic: Option<&[Tag]>) -> Prakriya {
        let mut anga = Term::new(root);
        anga.add(Tag::Dhatu);
        anga.add(Tag::Curadi);
        let mut terms = vec![anga];
        if let Some(tags) = nic {
            let mut t = Term::new("i");
            t.add(Tag::Pratyaya);
            for tag in tags {
                t.add(*tag);
            }
            terms.push(t);
        }
        Prakriya {
            terms: with_slots(terms),
            ..Default::default()
        }
    }

    #[test]
    fn nic_is_added_after_a_curadi_root_only() {
        let mut p = with_nic("cur", None);
        assert!((rule("3.1.25").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "Ric");
        assert!(p.terms[NIC].has(Tag::Pratyaya));
        // √bhū: no Curadi tag, no ṇic.
        let mut p = Prakriya {
            terms: with_slots(vec![Term::new("BU")]),
            ..Default::default()
        };
        assert!(!(rule("3.1.25").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC);
        assert!(p.log.is_empty());
    }

    #[test]
    fn nic_it_lopa_leaves_a_nit_i() {
        let mut p = with_nic("cur", None);
        (rule("3.1.25").apply)(&mut p);
        assert!((rule("1.3.9").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "i");
        assert!(p.terms[NIC].has(Tag::Rit));
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["3.1.25", "1.3.9"]);
        // Nothing at NIC, or something that is not ṇic: decline. The second
        // case is a tiṅ ending where ṇic would sit — the layout `samjna`
        // builds — which this rule must never touch.
        let mut p = with_nic("BU", None);
        assert!(!(rule("1.3.9").apply)(&mut p));
        let mut p = with_nic("BU", None);
        p.terms.push(Term::new("tip"));
        assert!(!(rule("1.3.9").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "tip");
    }

    /// `root` followed by 3.1.28's āya, as that rule leaves it, carrying
    /// `tags` too.
    fn with_aya(root: &str, tags: &[Tag]) -> Prakriya {
        let mut p = with_nic(root, None);
        let mut aya = Term::new("Aya");
        aya.add(Tag::Pratyaya);
        for tag in tags {
            aya.add(*tag);
        }
        p.terms.push(aya);
        p
    }

    #[test]
    fn ardhadhatuka_sesah_tags_the_sanadi_pratyaya_only() {
        let mut p = with_nic("cur", Some(&[Tag::Rit]));
        assert!((rule("3.4.114").apply)(&mut p));
        assert!(p.terms[NIC].has(Tag::Ardhadhatuka));
        // 3.1.28's āya, which has no it and so no `Tag::Rit`.
        let mut p = with_aya("DUp", &[]);
        assert!((rule("3.4.114").apply)(&mut p));
        assert!(p.terms[NIC].has(Tag::Ardhadhatuka));
        let mut p = with_nic("cur", None);
        assert!(!(rule("3.4.114").apply)(&mut p));
        let mut p = with_nic("cur", None);
        p.terms.push(Term::new("ti"));
        assert!(!(rule("3.4.114").apply)(&mut p));
        assert!(!p.terms[NIC].has(Tag::Ardhadhatuka));
    }

    #[test]
    fn ato_lopa_deletes_an_adanta_roots_a_before_ardhadhatuka_nic() {
        for (root, want) in [("kaTa", "kaT"), ("kuha", "kuh"), ("Una", "Un")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("6.4.48").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            assert!(p.terms[ANGA].has(Tag::AtLopa), "{root}");
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["6.4.48"]);
        }
        // Not `a`-final: a consonant (cur) or a long `A` (pA).
        for root in ["cur", "pA"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("6.4.48").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
            assert!(!p.terms[ANGA].has(Tag::AtLopa), "{root}");
        }
        // `a`-final, but no ārdhadhātuka follower: no ṇic at all, or a ṇic
        // that 3.4.114 has not reached yet.
        for nic in [None, Some(&[Tag::Rit][..])] {
            let mut p = with_nic("kaTa", nic);
            assert!(!(rule("6.4.48").apply)(&mut p), "{nic:?}");
            assert_eq!(p.terms[ANGA].text, "kaTa");
            assert!(!p.terms[ANGA].has(Tag::AtLopa), "{nic:?}");
        }
    }

    #[test]
    fn the_upadha_rules_decline_after_ato_lopa() {
        // 1.1.57: after 6.4.48, `kaT`'s `a` and `kuh`'s `u` sit at the upadhā,
        // but the deleted `a` still stands for 7.2.116 and 7.3.86.
        for (id, root) in [("7.2.116", "kaT"), ("7.3.86", "kuh")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            p.terms[ANGA].add(Tag::AtLopa);
            assert!(!(rule(id).apply)(&mut p), "{id} {root}");
            assert_eq!(p.terms[ANGA].text, root);
            assert!(p.log.is_empty(), "{id} {root}");
        }
        // The same shapes without the tag are ordinary roots, and both fire.
        for (id, root, want) in [("7.2.116", "kaT", "kAT"), ("7.3.86", "kuh", "koh")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule(id).apply)(&mut p), "{id} {root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
    }

    #[test]
    fn ata_upadhayah_lengthens_an_a_upadha_before_nit() {
        for (root, want) in [("laq", "lAq"), ("aw", "Aw")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.2.116").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // Not an `a` upadhā (Bakz: k; cur: u), too short to have one (a), or
        // no ṇit follower.
        for root in ["Bakz", "cur", "a"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.2.116").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        let mut p = with_nic("laq", Some(&[Tag::Ardhadhatuka]));
        assert!(!(rule("7.2.116").apply)(&mut p));
    }

    #[test]
    fn jnapadayo_mitah_credits_a_mit_root_and_changes_nothing() {
        let mut p = with_nic("jYap", None);
        p.terms[ANGA].add(Tag::Mit);
        assert!((rule("10.0493").apply)(&mut p));
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["10.0493"]);
        assert_eq!(p.terms[ANGA].text, "jYap", "a saṁjñā, not an operation");
        // √cur is curādi but not mit.
        let mut p = with_nic("cur", None);
        assert!(!(rule("10.0493").apply)(&mut p));
        assert!(p.log.is_empty());
    }

    #[test]
    fn mitam_hrasvah_shortens_a_mit_roots_long_upadha_before_nit() {
        // `jYAp` is 7.2.116's output for √jñap; the other three are
        // hand-built long upadhās, one per remaining `hrasva_of` arm that a
        // root can carry there.
        for (root, want) in [
            ("jYAp", "jYap"),
            ("cIl", "cil"),
            ("kUw", "kuw"),
            ("pFq", "pfq"),
            // √ci's two ṇic branches: 7.2.115 and 6.1.78's `cAy`, and 6.1.54
            // and 7.3.36's `cAp`.
            ("cAy", "cay"),
            ("cAp", "cap"),
        ] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            p.terms[ANGA].add(Tag::Mit);
            assert!((rule("6.4.92").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["6.4.92"]);
        }
        // A short upadhā (jYap, never lengthened), a non-vowel upadhā
        // (yakz: k), or too short to have one (A): nothing to shorten.
        for root in ["jYap", "yakz", "A"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            p.terms[ANGA].add(Tag::Mit);
            assert!(!(rule("6.4.92").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // Not mit: √śam's `SAm` keeps 7.2.116's vṛddhi.
        let mut p = with_nic("SAm", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!(!(rule("6.4.92").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "SAm");
        // Mit, but no ṇit follower: neither a bare root nor a non-ṇit
        // pratyaya at `NIC`.
        let mut p = with_nic("jYAp", None);
        p.terms[ANGA].add(Tag::Mit);
        assert!(!(rule("6.4.92").apply)(&mut p));
        let mut p = with_nic("jYAp", Some(&[Tag::Ardhadhatuka]));
        p.terms[ANGA].add(Tag::Mit);
        assert!(!(rule("6.4.92").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "jYAp");
    }

    #[test]
    fn pugantalaghupadhasya_gunates_a_laghu_ik_upadha_before_nic() {
        for (root, want) in [("cur", "cor"), ("uz", "oz"), ("kft", "kart")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.3.86").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // Guru upadhā (BUz: long U; Bakz: k), too short (u), or a ṇic not
        // (yet) ārdhadhātuka.
        for root in ["BUz", "Bakz", "u"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.86").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        let mut p = with_nic("cur", Some(&[Tag::Rit]));
        assert!(!(rule("7.3.86").apply)(&mut p));
    }

    #[test]
    fn nuk_follows_dhu_and_pri_before_nic_only() {
        for (root, want) in [("DU", "DUn"), ("prI", "prIn")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.3.37.2").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["7.3.37.2"]);
            // No ṇic, or a pratyaya at `NIC` that is not ṇit: no nuk.
            let mut p = with_nic(root, None);
            assert!(!(rule("7.3.37.2").apply)(&mut p), "{root}");
            let mut p = with_nic(root, Some(&[Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.37.2").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // Other ī- and ū-final roots before ṇic: √lī, √bhū, √mī.
        for root in ["lI", "BU", "mI"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.37.2").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        assert!(rule("7.3.37.2").vikalpa);
    }

    #[test]
    fn aco_nniti_lengthens_an_ac_final_anga_before_nic() {
        for (root, want) in [
            ("lI", "lE"),
            ("BU", "BO"),
            ("jri", "jrE"),
            ("vf", "vAr"),
            ("jF", "jAr"),
        ] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.2.115").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["7.2.115"]);
        }
        // Hal-final: a root (cur), √dhū after its nuk (DUn), and √jñā after
        // its puk (jYAp).
        for root in ["cur", "DUn", "jYAp"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.2.115").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // No ṇic, or a pratyaya at `NIC` that is not ṇit.
        let mut p = with_nic("BU", None);
        assert!(!(rule("7.2.115").apply)(&mut p));
        let mut p = with_nic("BU", Some(&[Tag::Ardhadhatuka]));
        assert!(!(rule("7.2.115").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "BU");
    }

    #[test]
    fn puk_follows_an_a_final_anga_before_nit_nic() {
        // √jñā (`10.0258 jYA`), and √ci's 6.1.54 branch (`cA`).
        for (root, want) in [("jYA", "jYAp"), ("cA", "cAp")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("7.3.36").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["7.3.36"]);
            // 7.2.115 then finds the `p`, not an ac, at the end.
            assert!(!(rule("7.2.115").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
        }
        // Not ā-final: a short `a` (kaTa, before 6.4.48), an ik (ci, BU), a
        // consonant (cur).
        for root in ["kaTa", "ci", "BU", "cur"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("7.3.36").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // No ṇic, or a pratyaya at `NIC` that is not ṇit.
        let mut p = with_nic("jYA", None);
        assert!(!(rule("7.3.36").apply)(&mut p));
        let mut p = with_nic("jYA", Some(&[Tag::Ardhadhatuka]));
        assert!(!(rule("7.3.36").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "jYA");
        assert!(!rule("7.3.36").vikalpa);
    }

    #[test]
    fn cisphuror_nau_gives_ci_its_a_on_its_row_only() {
        let mut p = with_nic("ci", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        p.ctx.dhatupatha = "10.0124";
        assert!((rule("6.1.54").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "cA");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["6.1.54"]);
        // `10.0325 ci` (bhāṣāyām) stores as the same `ci` but is not `ciY`,
        // and a row with no number is no row at all.
        for number in ["10.0325", ""] {
            let mut p = with_nic("ci", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            p.ctx.dhatupatha = number;
            assert!(!(rule("6.1.54").apply)(&mut p), "{number}");
            assert_eq!(p.terms[ANGA].text, "ci");
            assert!(p.log.is_empty(), "{number}");
        }
        // On its row, but no ṇic, or a pratyaya at `NIC` that is not ṇit.
        for nic in [None, Some(&[Tag::Ardhadhatuka][..])] {
            let mut p = with_nic("ci", nic);
            p.ctx.dhatupatha = "10.0124";
            assert!(!(rule("6.1.54").apply)(&mut p), "{nic:?}");
            assert_eq!(p.terms[ANGA].text, "ci");
        }
        let r = rule("6.1.54");
        assert!(r.vikalpa);
        assert!(r.bars.is_empty());
    }

    #[test]
    fn eco_yavayavah_resolves_the_vrddhi_before_nic() {
        for (root, want) in [("lE", "lAy"), ("BO", "BAv"), ("ne", "nay"), ("Bo", "Bav")] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!((rule("6.1.78").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, want);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["6.1.78"]);
        }
        // Not ec-final: `vAr` (7.2.115's f), `cur`.
        for root in ["vAr", "cur"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("6.1.78").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
        // No ṇic, or a pratyaya at `NIC` that is not ṇit: the guṇa stage's
        // entry, not this one, reads an aṅga against a vikaraṇa.
        let mut p = with_nic("lE", None);
        assert!(!(rule("6.1.78").apply)(&mut p));
        let mut p = with_nic("Bo", None);
        p.terms.push(Term::new("a"));
        assert!(!(rule("6.1.78").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "Bo");
    }

    #[test]
    fn mrjer_vrddhih_takes_vrddhi_before_ardhadhatuka_nic_on_mrj_only() {
        let mut p = with_nic("mfj", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        p.terms[ANGA].add(Tag::Mrj);
        assert!((rule("7.2.114").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mArj");
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["7.2.114"]);
        // Then 7.3.86 finds no laghu ik.
        assert!(!(rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "mArj");
        // The same shape without the tag is any other root: 7.3.86's guṇa.
        let mut p = with_nic("mfj", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!(!(rule("7.2.114").apply)(&mut p));
        assert!((rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "marj");
        // Tagged, but no ārdhadhātuka ṇic.
        for nic in [None, Some(&[Tag::Rit][..])] {
            let mut p = with_nic("mfj", nic);
            p.terms[ANGA].add(Tag::Mrj);
            assert!(!(rule("7.2.114").apply)(&mut p), "{nic:?}");
            assert_eq!(p.terms[ANGA].text, "mfj");
        }
    }

    #[test]
    fn aya_follows_a_tagged_root_that_took_no_nic() {
        for root in ["DUp", "viC"] {
            let mut p = with_nic(root, None);
            p.terms[ANGA].add(Tag::Aya);
            assert!((rule("3.1.28").apply)(&mut p), "{root}");
            assert_eq!(p.terms[NIC].text, "Aya");
            assert!(p.terms[NIC].has(Tag::Pratyaya));
            assert!(!p.terms[NIC].has(Tag::Rit), "āya is not ṇit");
            assert_eq!(p.terms[ANGA].text, root);
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["3.1.28"]);
        }
        // Tagged, but ṇic was taken: that branch is the ṇic one.
        let mut p = with_nic("DUp", Some(&[Tag::Rit]));
        p.terms[ANGA].add(Tag::Aya);
        assert!(!(rule("3.1.28").apply)(&mut p));
        assert_eq!(p.terms[NIC].text, "i");
        // Untagged: curādi √gup (`10.0302 gupa~`) takes no āya.
        let mut p = with_nic("gup", None);
        assert!(!(rule("3.1.28").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC);
        assert!(p.log.is_empty());
    }

    #[test]
    fn aya_folds_into_the_dhatu_without_nijanta() {
        let mut p = with_nic("DUp", None);
        p.terms[ANGA].add(Tag::Aya);
        assert!((rule("3.1.28").apply)(&mut p));
        assert!(!(rule("1.3.9").apply)(&mut p), "āya has no it");
        assert!((rule("3.4.114").apply)(&mut p));
        assert!((rule("3.1.32").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC, "the āya term is gone");
        assert_eq!(p.terms[ANGA].text, "DUpAya");
        assert!(p.terms[ANGA].has(Tag::Dhatu));
        assert!(
            !p.terms[ANGA].has(Tag::Nijanta),
            "an āya aṅga is no ṇijanta"
        );
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["3.1.28", "3.4.114", "3.1.32"]);
    }

    #[test]
    fn che_ca_gives_vich_tuk_before_guna_can_read_its_upadha() {
        // Before ṇic and before āya alike: tuk after the short `i`, and the
        // sanādi 7.3.86 then finds `t`, not a laghu ik, at the upadhā.
        for mut p in [
            with_nic("viC", Some(&[Tag::Rit, Tag::Ardhadhatuka])),
            with_aya("viC", &[Tag::Ardhadhatuka]),
        ] {
            assert!((rule("6.1.73").apply)(&mut p));
            assert_eq!(p.terms[ANGA].text, "vitC");
            assert!(!(rule("7.3.86").apply)(&mut p));
            assert_eq!(p.terms[ANGA].text, "vitC");
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, ["6.1.73"]);
        }
        // Without the tuk, 7.3.86 would read the `i`: *vechayati*.
        let mut p = with_nic("viC", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!((rule("7.3.86").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "veC");
        // No short vowel right before a `C`: a consonant (mUrC), nothing at
        // all (`C` word-initial, Cid), or no `C` (cur).
        for root in ["mUrC", "Cid", "cur"] {
            let mut p = with_nic(root, Some(&[Tag::Rit, Tag::Ardhadhatuka]));
            assert!(!(rule("6.1.73").apply)(&mut p), "{root}");
            assert_eq!(p.terms[ANGA].text, root);
        }
    }

    #[test]
    fn sanadyanta_folds_nic_into_the_dhatu() {
        let mut p = with_nic("cor", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!((rule("3.1.32").apply)(&mut p));
        assert_eq!(p.terms.len(), ANGA + 1, "the ṇic term is gone");
        assert_eq!(p.terms[ANGA].text, "cori");
        assert!(p.terms[ANGA].has(Tag::Dhatu));
        assert!(p.terms[ANGA].has(Tag::Nijanta));
        let mut p = with_nic("BU", None);
        assert!(!(rule("3.1.32").apply)(&mut p));
        let mut p = with_nic("BU", None);
        p.terms.push(Term::new("ti"));
        assert!(!(rule("3.1.32").apply)(&mut p));
        assert_eq!(p.terms.len(), NIC + 1);
    }

    /// A bare dhātu carrying `tags`, in a laṭ prathama eka context for `pada`
    /// — all 10.0496 reads.
    fn pada_dhatu(root: &str, tags: &[Tag], pada: Pada) -> Prakriya {
        let mut anga = Term::new(root);
        anga.add(Tag::Dhatu);
        for tag in tags {
            anga.add(*tag);
        }
        Prakriya {
            ctx: Context::new(Lakara::Lat, pada, Purusha::Prathama, Vacana::Eka),
            terms: with_slots(vec![anga]),
            ..Default::default()
        }
    }

    #[test]
    fn a_kusmad_sanctions_an_akusmiya_roots_atmanepada() {
        let mut p = pada_dhatu("cit", &[Tag::Curadi, Tag::Akusmiya], Pada::Atmanepada);
        assert!((rule("10.0496").apply)(&mut p));
        assert!(!p.blocked);
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["10.0496"]);
        assert_eq!(p.terms[ANGA].text, "cit", "a sanction, not an operation");
    }

    #[test]
    fn a_kusmad_blocks_an_akusmiya_roots_parasmaipada() {
        let mut p = pada_dhatu("cit", &[Tag::Curadi, Tag::Akusmiya], Pada::Parasmaipada);
        assert!(!(rule("10.0496").apply)(&mut p));
        assert!(p.blocked);
        assert!(p.log.is_empty());
    }

    #[test]
    fn a_kusmad_declines_without_the_akusmiya_licence() {
        // √cur (1.3.74's), √rudh (1.3.72's), √bhū (1.3.78's) and √ās
        // (1.3.12's) are left alone in both padas: not recorded, not blocked.
        for tags in [
            &[Tag::Curadi, Tag::Nic][..],
            &[Tag::Ubhayapadin][..],
            &[][..],
            &[Tag::Atmanepadin][..],
        ] {
            for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                let mut p = pada_dhatu("x", tags, pada);
                assert!(!(rule("10.0496").apply)(&mut p), "{tags:?} {pada:?}");
                assert!(!p.blocked, "{tags:?} {pada:?}");
                assert!(p.log.is_empty(), "{tags:?} {pada:?}");
            }
        }
    }

    #[test]
    fn a_garvad_sanctions_an_a_garviya_roots_atmanepada_and_blocks_its_parasmaipada() {
        let mut p = pada_dhatu("kuha", &[Tag::Curadi, Tag::AaGarviya], Pada::Atmanepada);
        assert!((rule("10.0497").apply)(&mut p));
        assert!(!p.blocked);
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["10.0497"]);
        assert_eq!(p.terms[ANGA].text, "kuha", "a sanction, not an operation");
        let mut p = pada_dhatu("kuha", &[Tag::Curadi, Tag::AaGarviya], Pada::Parasmaipada);
        assert!(!(rule("10.0497").apply)(&mut p));
        assert!(p.blocked);
        assert!(p.log.is_empty());
    }

    #[test]
    fn a_garvad_declines_without_the_a_garviya_licence() {
        // An ākusmīya root is 10.0496's, not 10.0497's; √cur, √rudh, √bhū
        // and √ās are left alone too. Both padas: not recorded, not blocked.
        for tags in [
            &[Tag::Curadi, Tag::Akusmiya][..],
            &[Tag::Curadi, Tag::Nic][..],
            &[Tag::Ubhayapadin][..],
            &[][..],
            &[Tag::Atmanepadin][..],
        ] {
            for pada in [Pada::Parasmaipada, Pada::Atmanepada] {
                let mut p = pada_dhatu("x", tags, pada);
                assert!(!(rule("10.0497").apply)(&mut p), "{tags:?} {pada:?}");
                assert!(!p.blocked, "{tags:?} {pada:?}");
                assert!(p.log.is_empty(), "{tags:?} {pada:?}");
            }
        }
    }

    /// A curādi root at row `number`, with its ṇic-branch pada `tag`, as the
    /// sanādi stage first sees it.
    fn optional_nic_dhatu(number: &'static str, root: &str, tag: Tag) -> Prakriya {
        let mut p = pada_dhatu(root, &[Tag::Curadi, tag], Pada::Parasmaipada);
        p.ctx.dhatupatha = number;
        p
    }

    #[test]
    fn each_optional_nic_rule_takes_the_nicless_branch_on_its_own_rows() {
        // (rule, a row it owns, that row's root and ṇic-branch pada tag)
        for (id, number, root, tag) in [
            ("10.0498", "10.0338", "yuj", Tag::Nic),
            // The idit √hiṃs is 10.0498's, not 2564's.
            ("10.0498", "10.0366", "hins", Tag::Nic),
            ("10.0499", "10.0279", "gras", Tag::Nic),
            // The idit √tuñj, the udit √vṛt and the īdit √pūr are 10.0499's.
            ("10.0499", "10.0285", "tunj", Tag::Nic),
            ("10.0499", "10.0312", "vft", Tag::Nic),
            ("10.0499", "10.0334", "pUr", Tag::Nic),
            ("2564", "10.0193", "danS", Tag::Akusmiya),
            ("2565", "10.0022", "pF", Tag::Nic),
            ("2570", "10.0230", "div", Tag::Akusmiya),
            // √ci, the one mit row whose ṇic is optional.
            ("2570", "10.0124", "ci", Tag::Nic),
            ("2571", "10.0251", "Guz", Tag::Nic),
            ("2573.1", "10.0400", "pata", Tag::Nic),
            ("2573.3", "10.0449", "garva", Tag::AaGarviya),
            ("2573.3", "10.0451", "mUtra", Tag::Nic),
        ] {
            let mut p = optional_nic_dhatu(number, root, tag);
            assert!((rule(id).apply)(&mut p), "{id} {number}");
            // The pada tag that hangs on ṇic is gone; nothing else changes.
            for t in [Tag::Nic, Tag::Akusmiya, Tag::AaGarviya] {
                assert!(!p.terms[ANGA].has(t), "{id} {number} kept {t:?}");
            }
            assert!(p.terms[ANGA].has(Tag::Curadi), "{id} {number}");
            assert_eq!(p.terms[ANGA].text, root, "a choice, not an operation");
            let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
            assert_eq!(ids, [id]);
            assert!(!p.blocked);
        }
    }

    #[test]
    fn the_nicless_fork_makes_a_nic_ubhayapada_root_ubhayapadin() {
        // `skip_nic`, through 2564 on its own row: with `Tag::NicUbhayapada`
        // the ṇic-less branch gains `Tag::Ubhayapadin` (1.3.72's); without it,
        // it gains nothing (1.3.78's śeṣa).
        for (extra, ubhaya) in [(Some(Tag::NicUbhayapada), true), (None, false)] {
            let mut p = optional_nic_dhatu("10.0193", "danS", Tag::Nic);
            if let Some(t) = extra {
                p.terms[ANGA].add(t);
            }
            assert!((rule("2564").apply)(&mut p));
            assert!(!p.terms[ANGA].has(Tag::Nic));
            assert_eq!(p.terms[ANGA].has(Tag::Ubhayapadin), ubhaya, "{extra:?}");
        }
        // The fork's rule declining leaves the ṇic branch untouched: no
        // `Tag::Ubhayapadin` there.
        let mut p = optional_nic_dhatu("10.0193", "danS", Tag::Nic);
        p.terms[ANGA].add(Tag::NicUbhayapada);
        assert!(!(rule("2570").apply)(&mut p));
        assert!(p.terms[ANGA].has(Tag::Nic));
        assert!(!p.terms[ANGA].has(Tag::Ubhayapadin));
    }

    #[test]
    fn an_optional_nic_rule_declines_off_its_rows() {
        // A sibling id's row, a curādi row outside the table (√cur, the
        // ākusmīya √cit, the adanta √kath), a row with no number at all, and
        // `10.0368 zad`: inside 10.0498's range but uncurated, so
        // the verdict is keyed on the table, not on position alone.
        for id in [
            "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3",
        ] {
            for (number, root, tag) in [
                ("10.0193", "danS", Tag::Akusmiya),
                ("10.0230", "div", Tag::Akusmiya),
                ("10.0400", "pata", Tag::Nic),
                ("10.0451", "mUtra", Tag::Nic),
                ("10.0279", "gras", Tag::Nic),
                ("10.0022", "pF", Tag::Nic),
                ("10.0251", "Guz", Tag::Nic),
                ("10.0124", "ci", Tag::Nic),
                ("10.0001", "cur", Tag::Nic),
                ("10.0192", "cit", Tag::Akusmiya),
                ("10.0389", "kaTa", Tag::Nic),
                ("", "cur", Tag::Nic),
                ("10.0368", "zad", Tag::Nic),
            ] {
                if optional_nic(number) == Some(id) {
                    continue;
                }
                let mut p = optional_nic_dhatu(number, root, tag);
                assert!(!(rule(id).apply)(&mut p), "{id} on {number}");
                assert!(p.terms[ANGA].has(tag), "{id} on {number}");
                assert!(p.log.is_empty(), "{id} on {number}");
            }
        }
    }

    #[test]
    fn the_optional_nic_rules_are_vikalpas_that_bar_nic() {
        for id in [
            "10.0498", "10.0499", "2564", "2565", "2570", "2571", "2573.1", "2573.3",
        ] {
            let r = rule(id);
            assert!(r.vikalpa, "{id}");
            assert_eq!(r.bars, ["3.1.25", "2573.2"], "{id}");
        }
        let r = rule("2573.2");
        assert!(r.vikalpa);
        assert!(r.bars.is_empty());
    }

    #[test]
    fn va_adanta_deletes_patas_a_before_nic_only_on_pata() {
        // `pata`, on the ṇic branch (nothing removed its `Nic` tag): the final
        // `a` goes, and no `Tag::AtLopa` is set, so 7.2.116 will lengthen.
        let mut p = optional_nic_dhatu("10.0400", "pata", Tag::Nic);
        assert!((rule("2573.2").apply)(&mut p));
        assert_eq!(p.terms[ANGA].text, "pat");
        assert!(!p.terms[ANGA].has(Tag::AtLopa));
        let ids: Vec<&str> = p.log.iter().map(|s| s.sutra.as_str()).collect();
        assert_eq!(ids, ["2573.2"]);
        // Then ṇic: 6.4.48 finds no `a`, and 7.2.116 lengthens.
        let mut q = with_nic("pat", Some(&[Tag::Rit, Tag::Ardhadhatuka]));
        assert!(!(rule("6.4.48").apply)(&mut q));
        assert!((rule("7.2.116").apply)(&mut q));
        assert_eq!(q.terms[ANGA].text, "pAt");
        // Other adanta rows, optional-ṇic or not, keep their `a`.
        for (number, root) in [
            ("10.0451", "mUtra"),
            ("10.0449", "garva"),
            ("10.0389", "kaTa"),
        ] {
            let mut p = optional_nic_dhatu(number, root, Tag::Nic);
            assert!(!(rule("2573.2").apply)(&mut p), "{number}");
            assert_eq!(p.terms[ANGA].text, root);
            assert!(p.log.is_empty(), "{number}");
        }
    }
}
