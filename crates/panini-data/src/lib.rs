#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
    Adadi,
    Kryadi,
    Svadi,
    Rudhadi,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pada {
    Parasmaipada,
    Atmanepada,
}
/// What a root *admits*, as distinct from `Context.pada`, which says what is
/// *being derived*. `Context.pada` stays the two-valued `Pada` on purpose:
/// no derivation may request an "ubhayapada" cell, because no such cell
/// exists — a root sanctioned in both padas is derived as two ordinary
/// single-pada cells, one per entry of `padas()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadaAssignment {
    Parasmaipada,
    Atmanepada,
    Ubhayapada,
}
impl PadaAssignment {
    /// The padas this assignment derives. `Ubhayapada` lists parasmaipada
    /// first — pinned, not incidental; see
    /// `ubhayapada_padas_are_parasmaipada_first` for why.
    pub fn padas(&self) -> &'static [Pada] {
        match self {
            PadaAssignment::Parasmaipada => &[Pada::Parasmaipada],
            PadaAssignment::Atmanepada => &[Pada::Atmanepada],
            PadaAssignment::Ubhayapada => &[Pada::Parasmaipada, Pada::Atmanepada],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lakara {
    Lat,
    Lan,
    Lot,
    /// The optative use of liṅ (sārvadhātuka: bhavet). The benedictive use
    /// (āśīrliṅ, ārdhadhātuka: bhūyāt) derives differently and will be a
    /// separate variant when implemented.
    VidhiLin,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purusha {
    Prathama,
    Madhyama,
    Uttama,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vacana {
    Eka,
    Dvi,
    Bahu,
}

#[derive(Debug, Clone, Copy)]
pub struct Dhatu {
    /// Dhātupāṭha entry number — the unique key. Names a row of
    /// `data/dhatupatha.tsv`, and `dhatupatha_numbers_resolve_upstream`
    /// checks that the row it names is the right one, by it-stripping that
    /// row's upadeśa and comparing against `code`.
    pub dhatupatha: &'static str,
    /// The root's SLP1 text, as it enters the derivation. Deliberately not
    /// unique — both √aś rows spell `aS` — and never a lookup key. Where it
    /// differs from the it-stripped upadeśa the reason is a rule this engine
    /// does not derive: `07.0019` stores `hins` for `hisi~` because 7.1.58
    /// idito num dhātoḥ is kept as a stated simplification, and `05.0021`
    /// stores `stiG` for `zwiGa~\` per 6.1.64 dhātvādeḥ ṣaḥ saḥ and its
    /// vārttika, which carries the following retroflex with it.
    pub code: &'static str,
    pub gana: Gana,
    /// Which pada(s) this engine derives for this root. Curated rather than
    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives every one of
    /// these 66 verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 /
    /// 1.3.78 and requires it to match, the same way
    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream.
    ///
    /// The column stayed hand-written because deriving it in production means
    /// running it-stripping in production, and upadeśa preprocessing is not
    /// the tiṅanta pipeline `TINANTA_RULES` models — it needs its own pipeline
    /// concept. Until it has one, a curated column plus a non-circular test is
    /// the honest arrangement; see the deferral in
    /// `docs/superpowers/specs/2026-08-16-pada-audit-design.md`.
    ///
    /// The test covers the 66 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
    /// not make the table self-maintaining.
    pub pada: PadaAssignment,
    pub artha: &'static str,
}

static DHATUS: &[Dhatu] = &[
    Dhatu {
        dhatupatha: "01.0001",
        code: "BU",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "sattAyAm",
    },
    // 01.1049 `RI\Y`: the final `Y` is an it by 1.3.3 halantyam, so 1.3.72
    // svaritañitaḥ sanctions both padas (nayati / nayate). Curated
    // parasmaipada from the v1 slice until the pada audit; no deferral list
    // ever named it.
    Dhatu {
        dhatupatha: "01.1049",
        code: "nI",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "prApaRe",
    },
    Dhatu {
        dhatupatha: "01.0642",
        code: "ji",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "jaye",
    },
    Dhatu {
        dhatupatha: "01.1082",
        code: "smf",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "cintAyAm",
    },
    Dhatu {
        dhatupatha: "01.0381",
        code: "paW",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.1164",
        code: "vad",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.0002",
        code: "eD",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vfdDO",
    },
    Dhatu {
        dhatupatha: "01.1130",
        code: "laB",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "prAptO",
    },
    Dhatu {
        dhatupatha: "01.0574",
        code: "sev",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "sevane",
    },
    Dhatu {
        dhatupatha: "01.0862",
        code: "vft",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vartane",
    },
    Dhatu {
        dhatupatha: "01.0696",
        code: "BAz",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.0694",
        code: "Ikz",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "darSane",
    },
    // divādi (gaṇa 4) — vikaraṇa śyan (3.1.69)
    Dhatu {
        dhatupatha: "04.0001",
        code: "div",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "krIqAvijigIzAvyavahAradyutistutimodamadasvapnakAntigatizu",
    },
    Dhatu {
        dhatupatha: "04.0091",
        code: "naS",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "adarSane",
    },
    Dhatu {
        dhatupatha: "04.0146",
        code: "kup",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kroDe",
    },
    Dhatu {
        dhatupatha: "04.0073",
        code: "man",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "jYAne",
    },
    Dhatu {
        dhatupatha: "04.0069",
        code: "yuD",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samprahAre",
    },
    Dhatu {
        dhatupatha: "04.0067",
        code: "vid",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "sattAyAm",
    },
    // tudādi (gaṇa 6) — vikaraṇa śa (3.1.77)
    // 06.0001 `tu\da~^`: the `~^` is a svarita it, so 1.3.72 sanctions both
    // padas (tudati / tudate). Deferred behind 1.3.72 by the divādi/tudādi
    // slice, then behind curation once 1.3.72 landed; discharged by the pada
    // audit.
    Dhatu {
        dhatupatha: "06.0001",
        code: "tud",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vyaTane",
    },
    Dhatu {
        dhatupatha: "06.0092",
        code: "liK",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "akzaravinyAse",
    },
    Dhatu {
        dhatupatha: "06.0160",
        code: "viS",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "praveSane",
    },
    Dhatu {
        dhatupatha: "06.0008",
        code: "juz",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "prItisevanayoH",
    },
    Dhatu {
        dhatupatha: "06.0009",
        code: "vij",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "BayacalanayoH",
    },
    Dhatu {
        dhatupatha: "06.0131",
        code: "gur",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "udyamane",
    },
    // adādi (gaṇa 2) — śap luk (2.4.72). √ad/√yā/√vā parasmaipada; √ās/√vas
    // ātmanepada — covered across all four lakāras (laṭ/laṅ/loṭ/vidhiliṅ).
    // √vas here is `vas` ācchādane (2Ā, "to wear"), NOT the far commoner
    // `vas` nivāse (1P, "to dwell", vasati); artha is the only disambiguator.
    Dhatu {
        dhatupatha: "02.0044",
        code: "yA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        dhatupatha: "02.0045",
        code: "vA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatiganDanayoH",
    },
    Dhatu {
        dhatupatha: "02.0001",
        code: "ad",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BakzaRe",
    },
    Dhatu {
        dhatupatha: "02.0011",
        code: "As",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "upaveSane",
    },
    Dhatu {
        dhatupatha: "02.0013",
        code: "vas",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "AcCAdane",
    },
    Dhatu {
        dhatupatha: "02.0026",
        code: "SI",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "svapne",
    },
    Dhatu {
        dhatupatha: "09.0058",
        code: "kliS",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vibADane",
    },
    Dhatu {
        dhatupatha: "09.0053",
        code: "guD",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "roze",
    },
    Dhatu {
        dhatupatha: "09.0059",
        code: "aS",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Bojane",
    },
    Dhatu {
        dhatupatha: "09.0066",
        code: "muz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "steye",
    },
    Dhatu {
        dhatupatha: "09.0040",
        code: "vrI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "varaRe",
    },
    Dhatu {
        dhatupatha: "09.0045",
        code: "vf",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samBaktO",
    },
    // svādi (gaṇa 5) — vikaraṇa śnu (3.1.73)
    Dhatu {
        dhatupatha: "05.0016",
        code: "Ap",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyAptO",
    },
    Dhatu {
        dhatupatha: "05.0017",
        code: "Sak",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "SaktO",
    },
    Dhatu {
        dhatupatha: "05.0012",
        code: "hi",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatO vfdDO ca",
    },
    Dhatu {
        dhatupatha: "05.0032",
        code: "ri",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 05.0020 aSU~\ vyAptau. Distinct root from kryādi's 09.0059 aSa~
        // Bojane, which shares this SLP1 form — under the retired `id`
        // scheme that required a qualifier; dhātupāṭha numbers distinguish
        // the rows without one. aSnute against aSnAti is the pair.
        dhatupatha: "05.0020",
        code: "aS",
        gana: Gana::Svadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vyAptO saNGAte ca",
    },
    Dhatu {
        // 05.0021 zwiGa~\. Stored post-6.1.64 dhātvādeḥ ṣaḥ saḥ: no rule in
        // the engine performs that substitution, so it is a stated
        // simplification, not a derivation step. See the spec's Data section.
        dhatupatha: "05.0021",
        code: "stiG",
        gana: Gana::Svadi,
        pada: PadaAssignment::Atmanepada,
        artha: "Askandane",
    },
    Dhatu {
        // 07.0010 kftI~ vezwane. rudhādi's √kṛt, distinct from tudādi's
        // √kṛnt — not in the root set, so the retired `id` scheme never
        // needed to qualify it.
        dhatupatha: "07.0010",
        code: "kft",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vezwane",
    },
    Dhatu {
        // 07.0019 hisi~ hiMsAyAm. Stored post-7.1.58 idito num dhātoH: the
        // root is idit and takes num, but the engine models no it-markers
        // at all (every root here is stored post-it-elision), so 7.1.58 is
        // not derivable and the num is stored. A stated simplification, not
        // a derivation step — exactly as `stiG` is stored post-6.1.64.
        // This is the root that makes 6.4.23 SnAnnalopaH reachable: śnam
        // gives hinans, and 6.4.23 takes the root's own n back out.
        dhatupatha: "07.0019",
        code: "hins",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 07.0012 Ki\da~\ dEnye. The gaṇa's ātmanepada arm. rudhādi offers
        // only three ānudātta roots (√indh, √khid, √vid); √khid is the one
        // that needs no rule beyond the gaṇa's own.
        dhatupatha: "07.0012",
        code: "Kid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Atmanepada,
        artha: "dEnye",
    },
    Dhatu {
        // 07.0016 Ba\njo~ Amardane. Witnesses 8.2.30 coH kuH: the root's
        // cu-class final (j) becomes the matching velar (g) word-finally
        // or before a jhal-initial affix.
        dhatupatha: "07.0016",
        code: "Banj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Amardane",
    },
    Dhatu {
        // 07.0015 pi\zx~ saYcUrRane hiMsAyAm ca. Witnesses 8.4.41 (zwutva:
        // an adjacent dental assimilates to retroflex next to the root's
        // z) and 8.2.41 (the root's final z is itself replaced by k
        // before an s-initial affix).
        dhatupatha: "07.0015",
        code: "piz",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "saYcUrRane hiMsAyAm ca",
    },
    Dhatu {
        // 07.0011 YiinDI~\ dIptO. Witnesses 8.2.40 Jazas taTor Do'DaH: a
        // Jaz-class final (voiced aspirated stop, here D) aṅga turns a
        // following t/T-initial affix into D. Looks ubhayapadī too: the
        // ñi it-marker is read by 1.3.72 svaritaYitaH, alongside its own
        // svarita. It is not — the anudAtta `~\` on top of the ñi settles
        // pada by 1.3.12 anudAttaNita Atmanepadam, and vidyut-prakriya
        // derives it ātmanepada-only, checked against a `~^r` control
        // (√rudh) that does derive both padas.
        dhatupatha: "07.0011",
        code: "inD",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Atmanepada,
        artha: "dIptO",
    },
    Dhatu {
        // 07.0001 ru\Di~^r AvaraRe. The gaṇa's EPONYM, and the engine's
        // first ubhayapadī root: the `~^` svarita is what 1.3.72
        // svaritaYitaH reads, and — unlike YiinDI~\ five rows above — the
        // entry carries no trailing `~\` anudātta it-marker for 1.3.12
        // anudAttaNita Atmanepadam to read (the `\` it does carry is the
        // root vowel's own accent, not an it), so 1.3.12 does not pre-empt
        // the parasmaipada reading and both pada cells derive.
        //
        // It needs no new sūtra. Its ātmanepada arm is structurally
        // √indh's (8.2.40 JaSas taTor Do'DaH, then 8.4.65 optionally
        // eliding the `d`), and its strong parasmaipada arm is √bhañj's and
        // √piṣ's — but it does reach one arm of existing phonology no
        // curated root had reached before: laṅ prathama/madhyama eka expose
        // the dhātu's own final `D` pada-finally (8.2.23 having eaten
        // tip/sip's own consonant), which 8.2.39 JalAM jaSo'nte now reaches
        // through jashtva_of, not new phonology of its own.
        //
        // It is also the first root to reach 8.4.2 awkupvANnumvyavAye'pi —
        // the NON-ADJACENT ṇatva, trigger and target separated (here by the
        // root's own aṭ vowel `u`: r-u-n) — inside rudhādi, the one gaṇa
        // where 8.3.24 naScApadAntasya Jali is live. So ṇatva fires on the
        // strong stem (ruRadDi, ruRaDE) and declines on the weak
        // (runDanti), whose nasal 8.3.24 has already turned into an
        // anusvāra before either ṇatva rule looks. The strong/weak split
        // itself is NOT new: √kṛt has shown it since slice 7a (kfRatti vs
        // kfnttaH), at the adjacent-trigger 8.4.1.
        dhatupatha: "07.0001",
        code: "ruD",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "AvaraRe",
    },
    Dhatu {
        // 07.0002 Bi\di~^r vidAraRe. Ubhayapadī by 1.3.72 svaritaYitaH: the
        // `~^` svarita it, with no trailing `~\` for 1.3.12 to pre-empt it.
        // The plainest of slice 7c's four roots — it reaches no rule the
        // gaṇa had not already reached, and is here for coverage rather
        // than as a witness. Coverage is a sufficient reason for a root to
        // exist; the audit in that slice is what earns it its place.
        dhatupatha: "07.0002",
        code: "Bid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vidAraRe",
    },
    Dhatu {
        // 07.0006 kzu\di~^r sampezaRe. Ubhayapadī by 1.3.72. Witnesses
        // 8.4.2 awkupvANnumvyavAye'pi under a SIBILANT trigger: in
        // kzuRatti the trigger is the `z` of `kz`, the target is Snam's
        // `n`, and the root's own aw vowel `u` separates them. That is
        // √rudh's shape (ruRadDi, r-u-n) reached through z rather than r,
        // and it makes this the second root to show the strong/weak ṇatva
        // split inside rudhādi -- the one gaṇa where 8.3.24
        // naScApadAntasya Jali is live and bleeds ṇatva off the weak stem.
        // 8.4.2's other curated witnesses (vrIRAti, muzARa) are kryādi,
        // where 8.3.24 never competes.
        dhatupatha: "07.0006",
        code: "kzud",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "sampezaRe",
    },
    Dhatu {
        // 07.0007 yu\ji~^r yoge. Ubhayapadī by 1.3.72. The root that earns
        // its place structurally: it is j-final, so its strong stem reaches
        // 8.2.30 coH kuH (yunagti -> 8.4.55 Kari ca -> yunakti), and 8.2.30's
        // substitute is now the 1.1.50 nearest velar, read from kutva_of,
        // rather than the literal 'g' it used to be. √bhañj has been that
        // rule's witness since 7b; pinning √yuj's 72 cells gave the
        // generalisation slice a second independent anchor it did not have
        // to build as part of the change it was validating. √ric and √vic,
        // curated in this same slice, are the roots that generalisation
        // actually unlocked.
        dhatupatha: "07.0007",
        code: "yuj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "yoge",
    },
    Dhatu {
        // 07.0009 u~tfdi~^r hiMsAnAdarayoH. Ubhayapadī by 1.3.72. The
        // leading `u~` is an it by 1.3.2 upadeSe'j anunAsika it; it is
        // neither anudātta nor Nit, so it never reaches 1.3.12, and udit's
        // own consequence (7.2.56 udito vA, optional iw before ktvA) is not
        // a tiṅanta rule and so cannot touch these four lakāras.
        // Structurally √kṛt: ṇatva here is the ADJACENT 8.4.1 razAByAM no
        // RaH, not √kṣud's 8.4.2 -- tfRatti's trigger `f` sits directly
        // against the `n` with nothing intervening -- and it leans on
        // is_natva_trigger's `f | F` arm, the r-vowels counting as triggers
        // by 1.1.51 uraR raparaH.
        dhatupatha: "07.0009",
        code: "tfd",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAnAdarayoH",
    },
    Dhatu {
        // 07.0004 ri\ci~^r virecane. Ubhayapadī by 1.3.72 svaritaYitaH; the
        // `\` is the root vowel's own accent, not an it. THE FIRST `c` EVER
        // TO REACH 8.2.30 coH kuH: riRakti's stem-final `c` takes the
        // voiceless velar `k` directly, where √bhañj's and √yuj's `j` takes
        // `g` and needs 8.4.55 Kari ca to devoice it afterwards. That
        // one-step/two-step contrast is what pins the substitute as a real
        // 1.1.50 nearest-velar map rather than the literal 'g' it used to
        // be. Also an 8.4.2 awkupvAGnumvyavAye'pi witness: the root's `r`
        // retroflexes śnam's `n` across the intervening `i`.
        dhatupatha: "07.0004",
        code: "ric",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "virecane",
    },
    Dhatu {
        // 07.0005 vi\ci~^r pfTagBAve. Ubhayapadī by 1.3.72. The MINIMAL
        // CONTRAST to √ric: same gaṇa, same c-final shape, same vikaraṇa,
        // same 8.2.30 application -- and no ṇatva trigger at all, so
        // vinakti keeps its dental `n`. The pair isolates 8.4.2 against a
        // controlled background, the way 7c used √kṣud and √tṛd to separate
        // 8.4.2 from 8.4.1.
        dhatupatha: "07.0005",
        code: "vic",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "pfTagBAve",
    },
    Dhatu {
        // 07.0014 Si\zx~ viSezaRe. Structurally √piṣ (07.0015) with a
        // different head: both are z-final, so both drive 8.4.41 zwutva
        // (Sinazwi, the dental of `ti` retroflexed next to the root's z)
        // and 8.2.41 (the z replaced by k before an s-initial affix).
        // Curated as the witness that the z path is not piṣ-specific.
        dhatupatha: "07.0014",
        code: "Siz",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "viSezaRe",
    },
    Dhatu {
        // 07.0020 undI~ kledane. VOWEL-INITIAL and u-headed, which is what
        // makes it worth curating: its laN takes AT (6.4.72) and then
        // 6.1.90 AwaS ca, whose `u` -> `O` arm no curated root had ever
        // reached -- `vrddhi_of_ac_vowels_all_arms` in panini-prakriya's
        // sound.rs says in as many words that only e/I/E inputs occur.
        // Onad is the counterexample. The root's own `n` is 6.4.23's, and
        // 6.4.111 then takes śnam's `a`, exactly as for √bhañj.
        dhatupatha: "07.0020",
        code: "und",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kledane",
    },
    Dhatu {
        // 07.0021 anjU~ vyaktimrakzaRakAntigatizu. Vowel-initial like
        // √und, and the 8.2.30 witness among the four nasal-tailed roots
        // here: anaj -> anj (6.4.111) -> ang (8.2.30) -> aNk, the `j` arm
        // of kutva_of on a stem whose nasal 6.4.23 has already thinned.
        dhatupatha: "07.0021",
        code: "anj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktimrakzaRakAntigatizu",
    },
    Dhatu {
        // 07.0022 tancU~ saNkocane. The consonant-initial contrast to
        // √añj: same nasal tail, same 6.4.23, and a `c` rather than a `j`
        // for 8.2.30 -- so kutva_of's two cu arms are both driven by roots
        // of the same shape, differing only in voicing.
        dhatupatha: "07.0022",
        code: "tanc",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "saNkocane",
    },
    Dhatu {
        // 07.0023 o~vijI~ BayacalanayoH. The second `o~`-initial upadeśa
        // in the table, after tudādi's `06.0009`. Nothing new is needed
        // for it: 1.3.2's anunāsika-it
        // loop in strip_anubandhas takes `o~` like any other vowel + `~`
        // pair, and `curated_pada_agrees_with_upadesha_markers` checks the
        // verdict rather than trusting it.
        dhatupatha: "07.0023",
        code: "vij",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BayacalanayoH",
    },
    Dhatu {
        // 07.0024 vfjI~ varjane. f-headed, so śnam's own `n` retroflexes
        // by 8.4.1 raSAByAM no RaH -- vfRakti. The minimal contrast to
        // √pṛc below is the tail, not the trigger: both take ṇatva, and
        // only one of them also drives 8.2.30 on a `c`.
        dhatupatha: "07.0024",
        code: "vfj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "varjane",
    },
    Dhatu {
        // 07.0025 pfcI~ samparke. ṇatva by 8.4.1 like √vṛj, and 8.2.30 on
        // a `c` like √tañc -- the one curated root that stacks both, so it
        // pins that the ṇatva trigger and the kutva substitution do not
        // interfere. pfRakti / pfNktaH.
        dhatupatha: "07.0025",
        code: "pfc",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "samparke",
    },
    Dhatu {
        // 07.0013 vi\da~\ vicAraRe. Ātmanepada by 1.3.12 on its trailing
        // `~\`, and the gaṇa's third pure-ātmanepadī root after √khid and
        // √indh. Distinct from divādi's `vid` (04.0067) and every other
        // √vid by dhātupāṭha number, not by surface. 8.4.65 Jaro Jari
        // savarRe forks nearly every cell it has (vinte / vintte).
        dhatupatha: "07.0013",
        code: "vid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vicAraRe",
    },
    Dhatu {
        // 07.0018 tfha~ hiMsAyAm. The gaṇa's ninth reachable
        // non-ubhayapadī root and the only one that needed sūtras this
        // engine lacked: 7.3.92 tfRaha im puts the *im* āgama into the
        // stem (tfnah -> tfnaih -> tfneh by 6.1.87), 8.2.31 ho QaH takes
        // the root's `h` to `Q`, and 8.3.13 Qo Qe lopaH elides it before
        // the `Q` that 8.4.41 produces -- tfReQi.
        //
        // The im is conditioned on a HAL-INITIAL PIT sārvadhātuka, which
        // is why this one root's paradigm splits three ways rather than
        // two: tfReQi/tfRekzi/tfRehmi take it, tfRQaH/tfMhanti do not
        // (apit, hence ṅit by 1.2.4), and atfRaham does not either
        // (`am` is vowel-initial).
        dhatupatha: "07.0018",
        code: "tfh",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 07.0003 Ci\di~^r dvEDIkaraRe. Ubhayapadī by 1.3.72 svaritaYitaH:
        // the `~^` is a svarita it, while the `\` is the root vowel's own
        // accent and says nothing about pada. Shape-identical to √bhid
        // (`07.0002`) -- `Ci` + `nad` where √bhid has `Bi` + `nad` -- so
        // every cell outside laṅ derives on rules already in the pipeline.
        //
        // The laṅ cells are the whole of what this root cost: 6.4.71's aṭ
        // puts a short `a` before the root's initial `C`, 6.1.73 Ce ca
        // inserts the tuk after it, and 8.4.40 stoH ScunA ScuH makes that
        // `t` a `c` -- acCinat, where the engine would otherwise reach
        // *aCinat.
        dhatupatha: "07.0003",
        code: "Cid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dvEDIkaraRe",
    },
    Dhatu {
        // 07.0008 u~Cfdi~^r dIptidevanayoH. Ubhayapadī by 1.3.72, on the
        // same svarita it as √chid. Udit, like √tṛd (`07.0009`) -- the
        // initial `u~` matters for 7.2.56 and 1.2.26 in ārdhadhātuka
        // contexts this engine does not cover, and is inert across all four
        // sārvadhātuka lakāras here.
        //
        // Shape-identical to √tṛd: `Cf` + `Rad` where √tṛd has `tf` + `Rad`,
        // 8.4.1's ṇatva included, since the trigger is the root's own `f`.
        // The tuk 6.1.73 inserts sits in FRONT of that `f` rather than
        // between it and the `n`, so it raises no 8.4.2 intervener question.
        dhatupatha: "07.0008",
        code: "Cfd",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dIptidevanayoH",
    },
];

pub fn dhatus() -> &'static [Dhatu] {
    DHATUS
}

pub fn tin_ending(pada: Pada, purusha: Purusha, vacana: Vacana) -> &'static str {
    use Purusha::*;
    use Vacana::*;
    match pada {
        Pada::Parasmaipada => match (purusha, vacana) {
            (Prathama, Eka) => "tip",
            (Prathama, Dvi) => "tas",
            (Prathama, Bahu) => "Ji",
            (Madhyama, Eka) => "sip",
            (Madhyama, Dvi) => "Tas",
            (Madhyama, Bahu) => "Ta",
            (Uttama, Eka) => "mip",
            (Uttama, Dvi) => "vas",
            (Uttama, Bahu) => "mas",
        },
        Pada::Atmanepada => match (purusha, vacana) {
            (Prathama, Eka) => "ta",
            (Prathama, Dvi) => "AtAm",
            (Prathama, Bahu) => "Ja",
            (Madhyama, Eka) => "TAs",
            (Madhyama, Dvi) => "ATAm",
            (Madhyama, Bahu) => "Dvam",
            (Uttama, Eka) => "iw",
            (Uttama, Dvi) => "vahi",
            (Uttama, Bahu) => "mahiN",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curated_roots_have_expected_ganas_and_padas() {
        assert_eq!(dhatus().len(), 66);
        let bu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
        assert!(matches!(bu.pada, PadaAssignment::Parasmaipada));
        let labh = dhatus().iter().find(|d| d.dhatupatha == "01.1130").unwrap();
        assert!(matches!(labh.pada, PadaAssignment::Atmanepada));
        // Both vowel-initial atmanepadi roots must be present (they exercise
        // the AT-augment path 6.4.72/6.1.90).
        assert!(dhatus().iter().any(|d| d.dhatupatha == "01.0002"));
        assert!(dhatus().iter().any(|d| d.dhatupatha == "01.0694"));
        // Divadi/tudadi still present.
        let div = dhatus().iter().find(|d| d.dhatupatha == "04.0001").unwrap();
        assert!(matches!(div.gana, Gana::Divadi));
        let tud = dhatus().iter().find(|d| d.dhatupatha == "06.0001").unwrap();
        assert!(matches!(tud.gana, Gana::Tudadi));
        // New: adadi (gaṇa 2), both ā-final parasmaipada.
        let ya = dhatus().iter().find(|d| d.dhatupatha == "02.0044").unwrap();
        assert!(matches!(ya.gana, Gana::Adadi) && matches!(ya.pada, PadaAssignment::Parasmaipada));
        let va = dhatus().iter().find(|d| d.dhatupatha == "02.0045").unwrap();
        assert!(matches!(va.gana, Gana::Adadi) && matches!(va.pada, PadaAssignment::Parasmaipada));
        // adādi ātmanepada: √ās (slice 5d), √vas (slice 5e), and √śī (this slice) closes gaṇa.
        let as_ = dhatus().iter().find(|d| d.dhatupatha == "02.0011").unwrap();
        assert!(matches!(as_.gana, Gana::Adadi) && matches!(as_.pada, PadaAssignment::Atmanepada));
        let vas = dhatus().iter().find(|d| d.dhatupatha == "02.0013").unwrap();
        assert!(matches!(vas.gana, Gana::Adadi) && matches!(vas.pada, PadaAssignment::Atmanepada));
        // √vas ācchādane (2Ā), not √vas nivāse (1P) — artha disambiguates.
        assert_eq!(vas.artha, "AcCAdane");
        // adādi ātmanepada: √śī (this slice) closes the gaṇa.
        let shi = dhatus().iter().find(|d| d.dhatupatha == "02.0026").unwrap();
        assert!(matches!(shi.gana, Gana::Adadi) && matches!(shi.pada, PadaAssignment::Atmanepada));
        assert_eq!(shi.artha, "svapne");
        // kryādi (gaṇa 9), slice 9a: kliS/guD/aS, all parasmaipada.
        let klis = dhatus().iter().find(|d| d.dhatupatha == "09.0058").unwrap();
        assert!(
            matches!(klis.gana, Gana::Kryadi) && matches!(klis.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(klis.artha, "vibADane");
        let gud = dhatus().iter().find(|d| d.dhatupatha == "09.0053").unwrap();
        assert!(
            matches!(gud.gana, Gana::Kryadi) && matches!(gud.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(gud.artha, "roze");
        let ash = dhatus().iter().find(|d| d.dhatupatha == "09.0059").unwrap();
        assert!(
            matches!(ash.gana, Gana::Kryadi) && matches!(ash.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(ash.artha, "Bojane");
        // kryādi, slice 9b: muz/vrI parasmaipada, vf (√vṛṅ) atmanepada --
        // the gaṇa's only pure-atmanepadi root.
        let muz = dhatus().iter().find(|d| d.dhatupatha == "09.0066").unwrap();
        assert!(
            matches!(muz.gana, Gana::Kryadi) && matches!(muz.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(muz.artha, "steye");
        let vri = dhatus().iter().find(|d| d.dhatupatha == "09.0040").unwrap();
        assert!(
            matches!(vri.gana, Gana::Kryadi) && matches!(vri.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(vri.artha, "varaRe");
        let vf = dhatus().iter().find(|d| d.dhatupatha == "09.0045").unwrap();
        assert!(matches!(vf.gana, Gana::Kryadi) && matches!(vf.pada, PadaAssignment::Atmanepada));
        assert_eq!(vf.artha, "samBaktO");
        // New: svādi (gaṇa 5), all four parasmaipadī.
        for number in ["05.0016", "05.0017", "05.0012", "05.0032"] {
            let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
            assert!(matches!(d.gana, Gana::Svadi));
            assert!(matches!(d.pada, PadaAssignment::Parasmaipada));
        }
    }

    #[test]
    fn atmanepada_tin_endings_are_raw_upadesha_forms() {
        use Purusha::*;
        use Vacana::*;
        let cases = [
            ((Prathama, Eka), "ta"),
            ((Prathama, Dvi), "AtAm"),
            ((Prathama, Bahu), "Ja"),
            ((Madhyama, Eka), "TAs"),
            ((Madhyama, Dvi), "ATAm"),
            ((Madhyama, Bahu), "Dvam"),
            ((Uttama, Eka), "iw"),
            ((Uttama, Dvi), "vahi"),
            ((Uttama, Bahu), "mahiN"),
        ];
        for ((pu, va), expected) in cases {
            assert_eq!(tin_ending(Pada::Atmanepada, pu, va), expected);
        }
    }

    #[test]
    fn tin_endings_are_marked_forms() {
        assert_eq!(
            tin_ending(Pada::Parasmaipada, Purusha::Prathama, Vacana::Eka),
            "tip"
        );
        assert_eq!(
            tin_ending(Pada::Parasmaipada, Purusha::Uttama, Vacana::Bahu),
            "mas"
        );
        assert_eq!(
            tin_ending(Pada::Parasmaipada, Purusha::Prathama, Vacana::Bahu),
            "Ji"
        );
    }

    #[test]
    fn ad_is_registered_as_adadi_parasmaipada() {
        let ad = dhatus()
            .iter()
            .find(|d| d.dhatupatha == "02.0001")
            .expect("√ad present");
        assert!(matches!(ad.gana, Gana::Adadi));
        assert!(matches!(ad.pada, PadaAssignment::Parasmaipada));
        assert_eq!(ad.artha, "BakzaRe");
    }

    #[test]
    fn as_is_registered_as_adadi_atmanepada() {
        let as_ = dhatus()
            .iter()
            .find(|d| d.dhatupatha == "02.0011")
            .expect("√ās present");
        assert!(matches!(as_.gana, Gana::Adadi));
        assert!(matches!(as_.pada, PadaAssignment::Atmanepada));
        assert_eq!(as_.artha, "upaveSane");
    }

    #[test]
    fn dhatupatha_is_the_key_and_is_unique() {
        let keys: Vec<&str> = dhatus().iter().map(|d| d.dhatupatha).collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            keys.len(),
            "dhātupāṭha numbers must be unique"
        );
        // Uniqueness here is a property of the source, not of a convention
        // this repo maintains: upstream numbers are unique across all 2259
        // entries, which `dhatupatha_numbers_resolve_upstream` also asserts.
        // That is the whole reason the number can serve as the key where the
        // SLP1 `code` could not — `code` is NOT unique (both √aś rows share
        // it), and the retired `Dhatu::id` existed only to paper over that.
        for d in dhatus() {
            assert_eq!(
                d.dhatupatha.len(),
                7,
                "{} is not a well-formed dhātupāṭha number",
                d.dhatupatha
            );
        }
    }

    #[test]
    fn the_two_ash_roots_are_distinct_rows() {
        let svadi = dhatus().iter().find(|d| d.dhatupatha == "05.0020").unwrap();
        let kryadi = dhatus().iter().find(|d| d.dhatupatha == "09.0059").unwrap();
        assert!(matches!(svadi.gana, Gana::Svadi));
        assert!(matches!(kryadi.gana, Gana::Kryadi));
        assert!(matches!(svadi.pada, PadaAssignment::Atmanepada));
        assert!(matches!(kryadi.pada, PadaAssignment::Parasmaipada));
        // Same surface text, different rows — and now distinct by
        // construction rather than by a hand-applied qualifier, since their
        // numbers come from different gaṇas of the source.
        assert_eq!(svadi.code, kryadi.code);
    }

    #[test]
    fn rudhadi_rows_are_the_twenty_four_curated_roots() {
        // √rudh, the gaṇa's eponym, arrived with 1.3.72 svaritaYitaH and
        // PadaAssignment::Ubhayapada. Slice 7c added √bhid, √kṣud, √yuj and
        // √tṛd; the 8.2.30/8.2.39 generalization slice added √ric and √vic
        // once 8.2.30 stopped hardcoding its `j` -> `g` pair.
        //
        // Slice 7d adds the eight roots that a probe against
        // vidyut-prakriya showed need NO sūtra this engine lacks: √śiṣ,
        // √und, √añj, √tañc, √vij, √vṛj, √pṛc and √vid. The probe compared
        // the sūtras each root's derivations invoke against this engine's
        // implemented set; `tools/audit/README.md`'s recorded result is
        // what turned that into a byte-for-byte verdict.
        //
        // `vi\da~\` and `o~vijI~` are the two entries whose SLP1 surfaces
        // WOULD have collided with divādi's `vid` and tudādi's `vij` under
        // the retired `id` scheme. Both are curated here, and under number
        // keying the question does not arise: `07.0013` and `07.0023` are
        // distinct from `04.0067` and `06.0009` whether or not their
        // surfaces agree. This is the slice that would have paid for the
        // retired scheme, and does not.
        //
        // After slice 7d the gaṇa was still PARTIAL: 21 of its 25
        // dhātupāṭha roots, so FOUR remained out, and they did not all
        // cost the same. √tṛh wanted 7.3.92 tfRaha im with 8.2.31 ho QaH
        // and 8.3.13 Qo Qe lopaH -- slice 7e curated it, below. √chid and
        // √chṛd still want 6.1.73 Ce ca plus 8.4.40 stoH ScunA ScuH.
        // And √bhuj is out on different grounds again: 1.3.66 Bujo'navane
        // forks its pada on sense.
        //
        // Slice 7e adds √tṛh, the ninth and last of the "reachable
        // non-ubhayapadī" roots 7d's probe separated out. It was the one
        // that did NOT come free: 7.3.92 tfRaha im, 8.2.31 ho QaH and
        // 8.3.13 Qo Qe lopaH are all new in this slice, and 8.4.41, 8.2.41
        // and 6.1.87 all had to widen to carry it. Parasmaipada: `tfha~`
        // carries no anudātta and no ñi, so 1.3.78 SezAt kartari
        // parasmaipadam settles it, and vidyut-prakriya derives no
        // ātmanepada forms for the entry.
        //
        // Slice 7f adds √chid and √chṛd, the last two ubhayapadī roots and
        // the last two that needed a sūtra: 6.1.73 Ce ca puts the tuk after
        // laṅ's aṭ-augment before their initial `C`, and 8.4.40 stoH ScunA
        // ScuH makes that `t` a `c` -- acCinat, acCfRat. Neither root needed
        // anything else: √chid is √bhid with a `C` for its `B`, and √chṛd is
        // √tṛd with a `C` for its `t`, ṇatva included, so every cell outside
        // laṅ derives on rules that were already in the pipeline.
        //
        // ONE of rudhādi's 25 is still out after this: √bhuj (`07.0017`),
        // and not for want of phonology -- vidyut derives all 72 of its
        // cells and 1.3.66 Bujo'navane is the only rule this engine lacks,
        // a root-keyed pada assignment structurally identical to 1.3.72's.
        // What keeps it out is that 1.3.66 restricts ātmanepada to senses
        // other than protecting, and neither engine models sense.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Rudhadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("07.0010", "kft", PadaAssignment::Parasmaipada),
                ("07.0019", "hins", PadaAssignment::Parasmaipada),
                ("07.0012", "Kid", PadaAssignment::Atmanepada),
                ("07.0016", "Banj", PadaAssignment::Parasmaipada),
                ("07.0015", "piz", PadaAssignment::Parasmaipada),
                ("07.0011", "inD", PadaAssignment::Atmanepada),
                ("07.0001", "ruD", PadaAssignment::Ubhayapada),
                ("07.0002", "Bid", PadaAssignment::Ubhayapada),
                ("07.0006", "kzud", PadaAssignment::Ubhayapada),
                ("07.0007", "yuj", PadaAssignment::Ubhayapada),
                ("07.0009", "tfd", PadaAssignment::Ubhayapada),
                ("07.0004", "ric", PadaAssignment::Ubhayapada),
                ("07.0005", "vic", PadaAssignment::Ubhayapada),
                ("07.0014", "Siz", PadaAssignment::Parasmaipada),
                ("07.0020", "und", PadaAssignment::Parasmaipada),
                ("07.0021", "anj", PadaAssignment::Parasmaipada),
                ("07.0022", "tanc", PadaAssignment::Parasmaipada),
                ("07.0023", "vij", PadaAssignment::Parasmaipada),
                ("07.0024", "vfj", PadaAssignment::Parasmaipada),
                ("07.0025", "pfc", PadaAssignment::Parasmaipada),
                ("07.0013", "vid", PadaAssignment::Atmanepada),
                ("07.0018", "tfh", PadaAssignment::Parasmaipada),
                ("07.0003", "Cid", PadaAssignment::Ubhayapada),
                ("07.0008", "Cfd", PadaAssignment::Ubhayapada),
            ]
        );
    }

    #[test]
    fn padas_maps_each_assignment_to_its_derivable_padas() {
        assert_eq!(PadaAssignment::Parasmaipada.padas(), &[Pada::Parasmaipada]);
        assert_eq!(PadaAssignment::Atmanepada.padas(), &[Pada::Atmanepada]);
        assert_eq!(
            PadaAssignment::Ubhayapada.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
    }

    #[test]
    fn ubhayapada_padas_are_parasmaipada_first() {
        // Pinned, not incidental: the paradigm and roundtrip harnesses loop
        // over the whole `padas()` slice, so they can't see its order, and
        // every `d.pada.padas()[0]` call site (the in-crate unit-test
        // helpers across the workspace) only ever sees single-pada roots
        // today. A mutant that reversed this slice would survive with no
        // test able to catch it — the same shape as the three `Context::is_tip`
        // survivors slice 7b found — so the order is asserted directly here.
        assert_eq!(PadaAssignment::Ubhayapada.padas()[0], Pada::Parasmaipada);
    }

    #[test]
    fn every_curated_root_admits_at_least_one_pada() {
        for d in dhatus() {
            assert!(
                !d.pada.padas().is_empty(),
                "{} admits no pada at all",
                d.dhatupatha
            );
        }
    }

    /// Upstream's dhātupāṭha, vendored at the commit named in its header.
    /// `include_str!` sits inside `#[cfg(test)]`, so the 54K reaches the test
    /// binary only and never the library.
    const UPSTREAM: &str = include_str!("../../../data/dhatupatha.tsv");

    /// `(number, upadeśa, artha)` for every upstream row.
    fn upstream_rows() -> Vec<(&'static str, &'static str, &'static str)> {
        UPSTREAM
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| {
                let mut f = l.split('\t');
                match (f.next(), f.next(), f.next()) {
                    // Skip upstream's own `code	dhatu	artha` header row.
                    (Some(n), Some(u), Some(a)) if n != "code" => Some((n, u, a)),
                    _ => None,
                }
            })
            .collect()
    }

    /// True for an SLP1 consonant (*hal*). SLP1's vowels are the fourteen
    /// listed here; `~`, being notation rather than a sound, is not a hal.
    fn is_hal(c: char) -> bool {
        c.is_alphabetic() && !"aAiIuUfFxXeEoO".contains(c)
    }

    /// Strips the anubandhas from an upstream upadeśa.
    ///
    /// **Not grammar the pipeline owes a `Rule`** — it never runs in a
    /// derivation. It exists so `dhatupatha_numbers_resolve_upstream` can
    /// relate an upstream row to our stored `code` without consulting
    /// anything this repo wrote, which is the assertion that makes the
    /// cross-implementation audit non-circular.
    fn strip_anubandhas(upadesha: &str) -> String {
        // Accent notation: anudātta `\`, svarita `^`. Marks, not sounds.
        let s: String = upadesha
            .chars()
            .filter(|c| *c != '\\' && *c != '^')
            .collect();

        // 1.3.3 halantyam is decided on the ORIGINAL upadeśa, before 1.3.2
        // deletes anything. Getting this order wrong corrupts silently rather
        // than failing loudly: `paWa~` ends in the vowel `a` (marked
        // anunāsika by the `~` after it), so its `W` is root-final and must
        // survive — deciding after the deletion would strip it to `pa`, and
        // would strip `tfha~` to `tf`, destroying a real root-final `h` while
        // still producing a plausible string. `ru\Di~^r` genuinely ends in
        // the consonant `r`, so that `r` IS an it.
        let ends_in_hal = s.chars().last().is_some_and(is_hal);

        // 1.3.2 upadeśe'j-anunāsika it, with 1.3.9 tasya lopaḥ. Upstream
        // marks an anunāsika it with a following `~`, so each `X~` pair goes.
        let mut t = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if chars.peek() == Some(&'~') {
                chars.next();
                continue;
            }
            t.push(c);
        }

        // 1.3.5 ādir ñiṭuḍavaḥ: an initial ñi / ṭu / ḍu is it.
        for prefix in ["Yi", "wu", "qu"] {
            if let Some(rest) = t.strip_prefix(prefix) {
                t = rest.to_string();
                break;
            }
        }

        // 1.3.3 halantyam, on the verdict reached above.
        if ends_in_hal && t.chars().count() > 1 {
            t.pop();
        }
        t
    }

    /// The pada a root's own upadeśa assigns it, by 1.3.12 / 1.3.72 / 1.3.78.
    ///
    /// **Not grammar the pipeline owes a `Rule`** — the same standing as
    /// `strip_anubandhas`, for the same reason: it never runs in a
    /// derivation. It exists so `curated_pada_agrees_with_upadesha_markers`
    /// can re-derive the `pada` column from upstream without consulting
    /// anything this repo wrote about the root.
    ///
    /// The accent notation is the whole difficulty. Upstream writes an accent
    /// AFTER the `~` that marks an anunāsika it, so `~\` is an anudātta it and
    /// `~^` a svarita it — whereas a `\` sitting directly on a vowel elsewhere
    /// is the ROOT's own accent and says nothing about pada. Counted off the
    /// vendored upadeśa: 43 of the 66 curated roots carry a `\` at all, and 30
    /// of those carry one on a root vowel — `01.0642 ji\`, `01.1082 smf\` and
    /// `02.0001 a\da~` among them — so conflating the two does not fail
    /// loudly; it silently calls most of the table ātmanepada.
    fn pada_from_upadesha(upadesha: &str) -> PadaAssignment {
        // Accents attached to an it vowel, and only those.
        let anudatta_it = upadesha.contains("~\\");
        let svarita_it = upadesha.contains("~^");

        // 1.3.3 halantyam, decided on the accent-stripped upadeśa: a final hal
        // is an it. `SIN` and `vfN` reach 1.3.12 this way, `RI\Y` reaches
        // 1.3.72, and none of the three carries a `~` at all.
        let bare: String = upadesha
            .chars()
            .filter(|c| *c != '\\' && *c != '^')
            .collect();
        let final_it = bare.chars().last().filter(|c| is_hal(*c));
        let ngit = final_it == Some('N');
        // 1.3.5 ādir ñiṭuḍavaḥ supplies a ñ it as an initial `Yi` too. Do NOT
        // extend this to `wu`/`qu`: the sūtra makes ñi, ṭu AND ḍu its, but
        // only ñi is a ñ-it, and 1.3.72 reads *svarita or ñit* specifically
        // — adding a wu/qu arm here would wrongly make every ṭu/ḍu-initial
        // root ubhayapadī. `01.1130 qula\Ba~\z` (√labh) is a curated
        // ḍu-initial root; it already comes out Ātmanepada correctly via its
        // own `~\`, not via this function.
        let nyit = final_it == Some('Y') || bare.starts_with("Yi");

        // ORDER IS LOAD-BEARING. 1.3.12 is tested first because `YiinDI~\`
        // (√indh) satisfies both it and 1.3.72, and must come out ātmanepada.
        // Pinned by `indh_is_atmanepada_despite_satisfying_1_3_72`.
        if anudatta_it || ngit {
            // 1.3.12 anudāttaṅita ātmanepadam.
            return PadaAssignment::Atmanepada;
        }
        if svarita_it || nyit {
            // 1.3.72 svaritañitaḥ kartrabhiprāye kriyāphale — ubhayapada,
            // since 1.3.78 supplies the parasmaipada arm.
            return PadaAssignment::Ubhayapada;
        }
        // 1.3.78 śeṣāt kartari parasmaipadam.
        PadaAssignment::Parasmaipada
    }

    /// 6.1.64 dhātvādeḥ ṣaḥ saḥ (ṣ → s) and 6.1.65 ṇo naḥ (ṇ → n). A
    /// root-initial ṣ or ṇ in the upadeśa is stored substituted, because no
    /// rule in this engine performs either substitution. For `zwiGa~\` the
    /// retroflex immediately after goes with it, under the vārttika on
    /// 6.1.64 (ṣṭ → st), which is exactly what `stiG` records.
    ///
    /// Only the `zw` → `st` arm of that vārttika (6.1.64.2) is handled here.
    /// Its other arms — `zW` → `sT`, `zR` → `sn`, `zaR` → `san` — and the
    /// companion vārttika 6.1.64.1 exempting `zWiv`/`zvazk` from any change
    /// are unimplemented: no curated root needs them today. `01.1077 zWA\`
    /// (√sthā), `01.0641`/`04.0004 zWivu~`, `01.0105 zvazka~\` and
    /// `01.0535 zaRa~` are the upstream roots that would exercise them; a
    /// future slice curating any of those must extend this function first,
    /// or `dhatupatha_numbers_resolve_upstream` fails loudly (√sthā would
    /// demand `sWA`, which this function does not produce).
    fn dhatvadeh_sha_sa(code: String) -> String {
        if let Some(rest) = code.strip_prefix('z') {
            let rest = rest
                .strip_prefix('w')
                .map_or_else(|| rest.to_string(), |r| format!("t{r}"));
            return format!("s{rest}");
        }
        if let Some(rest) = code.strip_prefix('R') {
            return format!("n{rest}");
        }
        code
    }

    /// The form this repo stores as `Dhatu::code`, derived from an upstream
    /// upadeśa.
    fn stored_form(upadesha: &str) -> String {
        let s = dhatvadeh_sha_sa(strip_anubandhas(upadesha));
        // 7.1.58 idito num dhātoḥ is not derivable here, so √hiṃs is stored
        // with the num already inserted. This is the single deviation between
        // an it-stripped upadeśa and a stored `code`, and it is the same one
        // the retired `Dhatu::id` doc comment recorded.
        if s == "his" { "hins".to_string() } else { s }
    }

    #[test]
    fn dhatupatha_numbers_resolve_upstream() {
        let rows = upstream_rows();
        let count = rows.len();
        assert!(
            count > 2000,
            "vendored dhātupāṭha looks truncated: {count} rows"
        );
        let mut numbers: Vec<&str> = rows.iter().map(|(n, _, _)| *n).collect();
        numbers.sort_unstable();
        numbers.dedup();
        assert_eq!(
            numbers.len(),
            count,
            "upstream numbers must be unique for one to serve as our key"
        );

        for d in dhatus() {
            let (_, upadesha, artha) = rows
                .iter()
                .find(|(n, _, _)| *n == d.dhatupatha)
                .unwrap_or_else(|| panic!("{} names no upstream row", d.dhatupatha));
            assert_eq!(
                *artha, d.artha,
                "{} artha diverges from upstream",
                d.dhatupatha
            );
            // THIS is the assertion that breaks the circularity. Matching on
            // number and artha alone would still pass if a number pointed at
            // a sibling entry sharing an artha, and upstream has 8- and
            // 15-way artha collisions (`vyaktAyAM vAci`, `vfdDO`). Relating
            // the upadeśa to the code is the only check that cannot be
            // satisfied by copying back the choice we made.
            let stripped = stored_form(upadesha);
            assert_eq!(
                stripped, d.code,
                "{} {upadesha} it-strips to {stripped}, but DHATUS stores {}",
                d.dhatupatha, d.code
            );
            // The spec's claim is that the number resolves the root
            // UNIQUELY, not merely that the row it names matches. A number
            // pointing at one of several siblings sharing both the
            // it-stripped upadeśa and the artha within the same gaṇa would
            // still pass every assertion above. Scope to the gaṇa (the
            // number's two-digit prefix, per `gana_matches_dhatupatha_prefix`)
            // because upstream reuses (code, artha) pairs across gaṇas too.
            let gana_prefix = &d.dhatupatha[..2];
            let siblings = rows
                .iter()
                .filter(|(n, u, a)| {
                    n.starts_with(gana_prefix) && stored_form(u) == stripped && *a == *artha
                })
                .count();
            assert_eq!(
                siblings, 1,
                "{} is ambiguous: {siblings} rows in gaṇa {gana_prefix} share \
                 ({stripped}, {artha})",
                d.dhatupatha
            );
        }
    }

    #[test]
    fn curated_pada_agrees_with_upadesha_markers() {
        let rows = upstream_rows();
        let mut wrong: Vec<String> = Vec::new();
        for d in dhatus() {
            let (_, upadesha, _) = rows
                .iter()
                .find(|(n, _, _)| *n == d.dhatupatha)
                .unwrap_or_else(|| panic!("{} names no upstream row", d.dhatupatha));
            let derived = pada_from_upadesha(upadesha);
            if derived != d.pada {
                wrong.push(format!(
                    "{} {} ({upadesha}): curated {:?}, markers say {derived:?}",
                    d.dhatupatha, d.code, d.pada
                ));
            }
        }
        assert!(
            wrong.is_empty(),
            "pada column disagrees with the vendored upadeśa:\n  {}",
            wrong.join("\n  ")
        );
    }

    #[test]
    fn indh_is_atmanepada_despite_satisfying_1_3_72() {
        // `YiinDI~\` carries a ñi that 1.3.72 reads AND an anudātta it that
        // 1.3.12 reads. 1.3.12 wins: vidyut-prakriya derives √indh in
        // ātmanepada only, checked in the ubhayapada slice against √rudh as a
        // `~^r` control. Reversing the two clauses in `pada_from_upadesha`
        // grows √indh a parasmaipada column it must not have.
        //
        // This is the second, independent encoding of the precedence that
        // `Tag::Ubhayapadin`'s doc comment in `panini-prakriya` states. It is
        // asserted here so a reversal fails rather than quietly re-deriving
        // that tag's own opinion.
        assert_eq!(pada_from_upadesha("YiinDI~\\"), PadaAssignment::Atmanepada);
    }

    #[test]
    fn a_final_hal_it_assigns_pada_without_any_tilde() {
        // 1.3.3 halantyam is the only marker these three have — no `~`
        // anywhere — so a check that looked only for `~\` / `~^` would call
        // all three parasmaipada and still agree with the column on two.
        assert_eq!(pada_from_upadesha("SIN"), PadaAssignment::Atmanepada); // 02.0026 √śī
        assert_eq!(pada_from_upadesha("vfN"), PadaAssignment::Atmanepada); // 09.0045 √vṛṅ
        assert_eq!(pada_from_upadesha("RI\\Y"), PadaAssignment::Ubhayapada); // 01.1049 √nī
    }

    #[test]
    fn a_root_vowel_accent_does_not_assign_pada() {
        // The failure mode that would make the whole audit vacuous: 42 of the
        // 64 curated roots carry a `\` somewhere in their upadeśa, 29 of them
        // on a root vowel rather than on an it. Reading every `\` as 1.3.12's
        // anudātta would call 23 of the 42 ātmanepada wrongly (15 curated
        // parasmaipada, 8 ubhayapada); the other 19 carry a genuine `~\` and
        // are curated ātmanepada anyway, which is why agreement with the
        // column would still hold on every genuinely ātmanepada root, and only
        // a parasmaipada witness catches it.
        assert_eq!(pada_from_upadesha("ji\\"), PadaAssignment::Parasmaipada); // 01.0642
        assert_eq!(pada_from_upadesha("a\\da~"), PadaAssignment::Parasmaipada); // 02.0001
        assert_eq!(pada_from_upadesha("Ba\\njo~"), PadaAssignment::Parasmaipada); // 07.0016
        // And the converse: the accent that DOES assign, on an it vowel.
        assert_eq!(pada_from_upadesha("Ki\\da~\\"), PadaAssignment::Atmanepada); // 07.0012
        assert_eq!(pada_from_upadesha("tu\\da~^"), PadaAssignment::Ubhayapada); // 06.0001
    }

    #[test]
    fn gana_matches_dhatupatha_prefix() {
        // The number's prefix encodes the gaṇa, so `Dhatu::gana` is redundant
        // with it. The field stays (the rule pipeline reads the enum
        // pervasively, and deriving it would mean parsing a string on every
        // lookup), and the redundancy becomes this check instead — a number
        // typed into the wrong gaṇa's block still names a real upstream row,
        // so nothing else would catch it.
        //
        // Mapped variant → prefix, not the inverse: this engine covers seven
        // of the ten gaṇas, so 03, 08 and 10 have no `Gana` variant.
        for d in dhatus() {
            let expected = match d.gana {
                Gana::Bhvadi => "01",
                Gana::Adadi => "02",
                Gana::Divadi => "04",
                Gana::Svadi => "05",
                Gana::Tudadi => "06",
                Gana::Rudhadi => "07",
                Gana::Kryadi => "09",
            };
            assert!(
                d.dhatupatha.starts_with(expected),
                "{:?} root {} has number {}, which is not in gaṇa {expected}",
                d.gana,
                d.code,
                d.dhatupatha
            );
        }
    }
}
