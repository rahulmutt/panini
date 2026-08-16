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
    /// Unique lookup key. Usually equal to `code`. Exceptions:
    /// (1) **Collision handling**: when a later gaṇa's root collides with an
    /// SLP1 form already in use, the incumbent keeps its bare `code` as id,
    /// and only the newcomer's id is gaṇa-qualified as `{code}.{gana}`
    /// (kryādi's `aS` keeps id `aS`; svādi's colliding root gets id `aS.5`).
    /// (2) **Rule-driven storage**: id is a bare lookup key, code carries a
    /// rule-driven stored augment (e.g., rudhādi's `his` is the lookup key,
    /// but `hins` is stored because 7.1.58 idito num dhātoH is not derivable
    /// and the num is kept as a stated simplification). Never hand this to
    /// `Term::new`.
    pub id: &'static str,
    /// The root's SLP1 text, as it enters the derivation.
    pub code: &'static str,
    pub gana: Gana,
    /// Which pada(s) this engine derives for this root — a curated verdict,
    /// not the upadeśa it-markers (`anudatta_ngit`, `svarita_nit`) that the
    /// sūtras actually read. Reading real markers here would make 1.3.72
    /// fire on every root whose markers satisfy it, and √tud's do; holding
    /// this slice's non-ubhayapada scope would then require writing a false
    /// marker on √tud, which the public data API refuses to do — the same
    /// refusal `Context::default`'s doc states for claiming a "default
    /// lakāra". A documented deferral in one field, lifted root-by-root as
    /// sūtras are implemented, is honest about what it is; see "The pada
    /// model" in `docs/superpowers/specs/2026-08-15-ubhayapada-1-3-72-design.md`.
    pub pada: PadaAssignment,
    pub artha: &'static str,
}

static DHATUS: &[Dhatu] = &[
    Dhatu {
        dhatupatha: "01.0001",
        id: "BU",
        code: "BU",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "sattAyAm",
    },
    Dhatu {
        dhatupatha: "01.1049",
        id: "nI",
        code: "nI",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        dhatupatha: "01.0642",
        id: "ji",
        code: "ji",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "jaye",
    },
    Dhatu {
        dhatupatha: "01.1082",
        id: "smf",
        code: "smf",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "cintAyAm",
    },
    Dhatu {
        dhatupatha: "01.0381",
        id: "paW",
        code: "paW",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.1164",
        id: "vad",
        code: "vad",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.0002",
        id: "eD",
        code: "eD",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vfdDO",
    },
    Dhatu {
        dhatupatha: "01.1130",
        id: "laB",
        code: "laB",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "prAptO",
    },
    Dhatu {
        dhatupatha: "01.0574",
        id: "sev",
        code: "sev",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "sevane",
    },
    Dhatu {
        dhatupatha: "01.0862",
        id: "vft",
        code: "vft",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vartane",
    },
    Dhatu {
        dhatupatha: "01.0696",
        id: "BAz",
        code: "BAz",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.0694",
        id: "Ikz",
        code: "Ikz",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "darSane",
    },
    // divādi (gaṇa 4) — vikaraṇa śyan (3.1.69)
    Dhatu {
        dhatupatha: "04.0001",
        id: "div",
        code: "div",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "krIqAvijigIzAvyavahAradyutistutimodamadasvapnakAntigatizu",
    },
    Dhatu {
        dhatupatha: "04.0091",
        id: "naS",
        code: "naS",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "adarSane",
    },
    Dhatu {
        dhatupatha: "04.0146",
        id: "kup",
        code: "kup",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kroDe",
    },
    Dhatu {
        dhatupatha: "04.0073",
        id: "man",
        code: "man",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "jYAne",
    },
    Dhatu {
        dhatupatha: "04.0069",
        id: "yuD",
        code: "yuD",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samprahAre",
    },
    Dhatu {
        dhatupatha: "04.0067",
        id: "vid",
        code: "vid",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "sattAyAm",
    },
    // tudādi (gaṇa 6) — vikaraṇa śa (3.1.77)
    Dhatu {
        dhatupatha: "06.0001",
        id: "tud",
        code: "tud",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaTane",
    },
    Dhatu {
        dhatupatha: "06.0092",
        id: "liK",
        code: "liK",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "akzaravinyAse",
    },
    Dhatu {
        dhatupatha: "06.0160",
        id: "viS",
        code: "viS",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "praveSane",
    },
    Dhatu {
        dhatupatha: "06.0008",
        id: "juz",
        code: "juz",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "prItisevanayoH",
    },
    Dhatu {
        dhatupatha: "06.0009",
        id: "vij",
        code: "vij",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "BayacalanayoH",
    },
    Dhatu {
        dhatupatha: "06.0131",
        id: "gur",
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
        id: "yA",
        code: "yA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        dhatupatha: "02.0045",
        id: "vA",
        code: "vA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatiganDanayoH",
    },
    Dhatu {
        dhatupatha: "02.0001",
        id: "ad",
        code: "ad",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BakzaRe",
    },
    Dhatu {
        dhatupatha: "02.0011",
        id: "As",
        code: "As",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "upaveSane",
    },
    Dhatu {
        dhatupatha: "02.0013",
        id: "vas",
        code: "vas",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "AcCAdane",
    },
    Dhatu {
        dhatupatha: "02.0026",
        id: "SI",
        code: "SI",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "svapne",
    },
    Dhatu {
        dhatupatha: "09.0058",
        id: "kliS",
        code: "kliS",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vibADane",
    },
    Dhatu {
        dhatupatha: "09.0053",
        id: "guD",
        code: "guD",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "roze",
    },
    Dhatu {
        dhatupatha: "09.0059",
        id: "aS",
        code: "aS",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Bojane",
    },
    Dhatu {
        dhatupatha: "09.0066",
        id: "muz",
        code: "muz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "steye",
    },
    Dhatu {
        dhatupatha: "09.0040",
        id: "vrI",
        code: "vrI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "varaRe",
    },
    Dhatu {
        dhatupatha: "09.0045",
        id: "vf",
        code: "vf",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samBaktO",
    },
    // svādi (gaṇa 5) — vikaraṇa śnu (3.1.73)
    Dhatu {
        dhatupatha: "05.0016",
        id: "Ap",
        code: "Ap",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyAptO",
    },
    Dhatu {
        dhatupatha: "05.0017",
        id: "Sak",
        code: "Sak",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "SaktO",
    },
    Dhatu {
        dhatupatha: "05.0012",
        id: "hi",
        code: "hi",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatO vfdDO ca",
    },
    Dhatu {
        dhatupatha: "05.0032",
        id: "ri",
        code: "ri",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 05.0020 aSU~\ vyAptau. Distinct root from kryādi's 09.0059 aSa~
        // Bojane, which shares this SLP1 form — hence the qualified id.
        // aSnute against aSnAti is the pair.
        dhatupatha: "05.0020",
        id: "aS.5",
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
        id: "stiG",
        code: "stiG",
        gana: Gana::Svadi,
        pada: PadaAssignment::Atmanepada,
        artha: "Askandane",
    },
    Dhatu {
        // 07.0010 kftI~ vezwane. rudhādi's √kṛt, distinct from tudādi's
        // √kṛnt — not in the root set, so no id qualification is needed.
        dhatupatha: "07.0010",
        id: "kft",
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
        id: "his",
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
        id: "Kid",
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
        id: "Banj",
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
        id: "piz",
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
        id: "inD",
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
        // tip/sip's own consonant), which is 8.2.39 JalAM jaSo'nte's newly
        // widened `D` arm, not new phonology of its own.
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
        id: "ruD",
        code: "ruD",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "AvaraRe",
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
        assert_eq!(dhatus().len(), 49);
        let bu = dhatus().iter().find(|d| d.id == "BU").unwrap();
        assert!(matches!(bu.pada, PadaAssignment::Parasmaipada));
        let labh = dhatus().iter().find(|d| d.id == "laB").unwrap();
        assert!(matches!(labh.pada, PadaAssignment::Atmanepada));
        // Both vowel-initial atmanepadi roots must be present (they exercise
        // the AT-augment path 6.4.72/6.1.90).
        assert!(dhatus().iter().any(|d| d.id == "eD"));
        assert!(dhatus().iter().any(|d| d.id == "Ikz"));
        // Divadi/tudadi still present.
        let div = dhatus().iter().find(|d| d.id == "div").unwrap();
        assert!(matches!(div.gana, Gana::Divadi));
        let tud = dhatus().iter().find(|d| d.id == "tud").unwrap();
        assert!(matches!(tud.gana, Gana::Tudadi));
        // New: adadi (gaṇa 2), both ā-final parasmaipada.
        let ya = dhatus().iter().find(|d| d.id == "yA").unwrap();
        assert!(matches!(ya.gana, Gana::Adadi) && matches!(ya.pada, PadaAssignment::Parasmaipada));
        let va = dhatus().iter().find(|d| d.id == "vA").unwrap();
        assert!(matches!(va.gana, Gana::Adadi) && matches!(va.pada, PadaAssignment::Parasmaipada));
        // adādi ātmanepada: √ās (slice 5d), √vas (slice 5e), and √śī (this slice) closes gaṇa.
        let as_ = dhatus().iter().find(|d| d.id == "As").unwrap();
        assert!(matches!(as_.gana, Gana::Adadi) && matches!(as_.pada, PadaAssignment::Atmanepada));
        let vas = dhatus().iter().find(|d| d.id == "vas").unwrap();
        assert!(matches!(vas.gana, Gana::Adadi) && matches!(vas.pada, PadaAssignment::Atmanepada));
        // √vas ācchādane (2Ā), not √vas nivāse (1P) — artha disambiguates.
        assert_eq!(vas.artha, "AcCAdane");
        // adādi ātmanepada: √śī (this slice) closes the gaṇa.
        let shi = dhatus().iter().find(|d| d.id == "SI").unwrap();
        assert!(matches!(shi.gana, Gana::Adadi) && matches!(shi.pada, PadaAssignment::Atmanepada));
        assert_eq!(shi.artha, "svapne");
        // kryādi (gaṇa 9), slice 9a: kliS/guD/aS, all parasmaipada.
        let klis = dhatus().iter().find(|d| d.id == "kliS").unwrap();
        assert!(
            matches!(klis.gana, Gana::Kryadi) && matches!(klis.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(klis.artha, "vibADane");
        let gud = dhatus().iter().find(|d| d.id == "guD").unwrap();
        assert!(
            matches!(gud.gana, Gana::Kryadi) && matches!(gud.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(gud.artha, "roze");
        let ash = dhatus().iter().find(|d| d.id == "aS").unwrap();
        assert!(
            matches!(ash.gana, Gana::Kryadi) && matches!(ash.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(ash.artha, "Bojane");
        // kryādi, slice 9b: muz/vrI parasmaipada, vf (√vṛṅ) atmanepada --
        // the gaṇa's only pure-atmanepadi root.
        let muz = dhatus().iter().find(|d| d.id == "muz").unwrap();
        assert!(
            matches!(muz.gana, Gana::Kryadi) && matches!(muz.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(muz.artha, "steye");
        let vri = dhatus().iter().find(|d| d.id == "vrI").unwrap();
        assert!(
            matches!(vri.gana, Gana::Kryadi) && matches!(vri.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(vri.artha, "varaRe");
        let vf = dhatus().iter().find(|d| d.id == "vf").unwrap();
        assert!(matches!(vf.gana, Gana::Kryadi) && matches!(vf.pada, PadaAssignment::Atmanepada));
        assert_eq!(vf.artha, "samBaktO");
        // New: svādi (gaṇa 5), all four parasmaipadī.
        for id in ["Ap", "Sak", "hi", "ri"] {
            let d = dhatus().iter().find(|d| d.id == id).unwrap();
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
        let ad = dhatus().iter().find(|d| d.id == "ad").expect("√ad present");
        assert!(matches!(ad.gana, Gana::Adadi));
        assert!(matches!(ad.pada, PadaAssignment::Parasmaipada));
        assert_eq!(ad.artha, "BakzaRe");
    }

    #[test]
    fn as_is_registered_as_adadi_atmanepada() {
        let as_ = dhatus().iter().find(|d| d.id == "As").expect("√ās present");
        assert!(matches!(as_.gana, Gana::Adadi));
        assert!(matches!(as_.pada, PadaAssignment::Atmanepada));
        assert_eq!(as_.artha, "upaveSane");
    }

    #[test]
    fn id_is_the_lookup_key_and_is_unique() {
        let ids: Vec<&str> = dhatus().iter().map(|d| d.id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "dhatu ids must be unique");
        // Verify the contract stated in `Dhatu::id`'s doc: id equals code,
        // except for two known exceptions: (1) collision-qualified ids like
        // svādi's aS.5 for the kryādi/svādi aS collision; (2) rule-driven
        // storage exceptions like rudhādi's his/hins, where id is the lookup
        // key but code carries the rule-enforced augment (7.1.58 num here).
        for d in dhatus() {
            let is_collision_qualified = d.id.starts_with(&format!("{}.", d.code));
            let is_rule_driven_storage = d.id == "his" && d.code == "hins";
            assert!(
                d.id == d.code || is_collision_qualified || is_rule_driven_storage,
                "id {:?} must follow Dhatu::id contract: equal to code, \
                 or gaṇa-qualified as {{code}}.{{gana}} (collision), \
                 or a known rule-driven storage exception",
                d.id
            );
        }
    }

    #[test]
    fn the_two_ash_roots_are_distinct_rows() {
        let svadi = dhatus().iter().find(|d| d.id == "aS.5").unwrap();
        let kryadi = dhatus().iter().find(|d| d.id == "aS").unwrap();
        assert!(matches!(svadi.gana, Gana::Svadi));
        assert!(matches!(kryadi.gana, Gana::Kryadi));
        assert!(matches!(svadi.pada, PadaAssignment::Atmanepada));
        assert!(matches!(kryadi.pada, PadaAssignment::Parasmaipada));
        // Same surface text, different rows. If ids ever collapse, one of these
        // roots silently stops being derivable.
        assert_eq!(svadi.code, kryadi.code);
    }

    #[test]
    fn rudhadi_holds_seven_roots_including_its_eponym() {
        // Seven roots, in table order. √hiṃs is stored `hins`, NOT `his`:
        // see its row comment. √rudh, the eponym, arrived with 1.3.72
        // svaritaYitaH and PadaAssignment::Ubhayapada — the machinery that
        // 7b recorded as the one thing its absence was waiting on.
        //
        // The gaṇa is still PARTIAL (7 of its 25 dhātupāṭha roots). 1.3.72
        // no longer holds any of the remaining eight `~^r` roots back, but
        // they do not all cost the same: √bhid, √kṣud, √yuj and √tṛd are a
        // table row and an audit apiece (the engine already derives all 72
        // cells of each, byte-identical to vidyut-prakriya), √ric and √vic
        // want 8.2.30 coH kuH widened past `j`, and √chid and √chṛd want
        // 6.1.73 Ce ca plus 8.4.40 stoH ScunA ScuH, which this engine does
        // not have.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Rudhadi)
            .map(|d| (d.id, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("kft", "kft", PadaAssignment::Parasmaipada),
                ("his", "hins", PadaAssignment::Parasmaipada),
                ("Kid", "Kid", PadaAssignment::Atmanepada),
                ("Banj", "Banj", PadaAssignment::Parasmaipada),
                ("piz", "piz", PadaAssignment::Parasmaipada),
                ("inD", "inD", PadaAssignment::Atmanepada),
                ("ruD", "ruD", PadaAssignment::Ubhayapada),
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
            assert!(!d.pada.padas().is_empty(), "{} admits no pada at all", d.id);
        }
    }

    #[test]
    fn rudhadi_ids_do_not_collide() {
        // rudhādi also holds `vi\da~\` and `o~vijI~`, which WOULD collide
        // with divādi's `vid` and tudādi's `vij`. Neither is curated — the
        // gaṇa stops at seven roots — so every rudhādi id is still its own
        // unqualified SLP1 code and the `aS.5` qualification mechanism
        // stays at exactly one user. 7a's spec predicted this would not
        // survive 7b; it does, because that prediction assumed a root set
        // including √vid. √rudh does not change it either: `ruD` collides
        // with nothing in the table.
        for d in dhatus().iter().filter(|d| d.gana == Gana::Rudhadi) {
            let n = dhatus().iter().filter(|o| o.id == d.id).count();
            assert_eq!(n, 1, "rudhādi id {} is not unique in DHATUS", d.id);
        }
        let qualified: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.id.contains('.'))
            .map(|d| d.id)
            .collect();
        assert_eq!(qualified, vec!["aS.5"]);
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

    /// 6.1.64 dhātvādeḥ ṣaḥ saḥ / ṇaḥ naḥ. A root-initial ṣ or ṇ in the
    /// upadeśa is stored as s / n, because no rule in this engine performs
    /// the substitution. For `zwiGa~\` the retroflex immediately after goes
    /// with it (ṣṭ → st), which is exactly what `stiG` records.
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
        }
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
