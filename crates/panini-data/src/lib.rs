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
        id: "BU",
        code: "BU",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "sattAyAm",
    },
    Dhatu {
        id: "nI",
        code: "nI",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        id: "ji",
        code: "ji",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "jaye",
    },
    Dhatu {
        id: "smf",
        code: "smf",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "cintAyAm",
    },
    Dhatu {
        id: "paW",
        code: "paW",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        id: "vad",
        code: "vad",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        id: "eD",
        code: "eD",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vfdDO",
    },
    Dhatu {
        id: "laB",
        code: "laB",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "prAptO",
    },
    Dhatu {
        id: "sev",
        code: "sev",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "sevane",
    },
    Dhatu {
        id: "vft",
        code: "vft",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vartane",
    },
    Dhatu {
        id: "BAz",
        code: "BAz",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        id: "Ikz",
        code: "Ikz",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "darSane",
    },
    // divādi (gaṇa 4) — vikaraṇa śyan (3.1.69)
    Dhatu {
        id: "div",
        code: "div",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "krIqAyAm",
    },
    Dhatu {
        id: "naS",
        code: "naS",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "adarSane",
    },
    Dhatu {
        id: "kup",
        code: "kup",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kroDe",
    },
    Dhatu {
        id: "man",
        code: "man",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "jYAne",
    },
    Dhatu {
        id: "yuD",
        code: "yuD",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samprahAre",
    },
    Dhatu {
        id: "vid",
        code: "vid",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "sattAyAm",
    },
    // tudādi (gaṇa 6) — vikaraṇa śa (3.1.77)
    Dhatu {
        id: "tud",
        code: "tud",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaTane",
    },
    Dhatu {
        id: "liK",
        code: "liK",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "akzaravinyAse",
    },
    Dhatu {
        id: "viS",
        code: "viS",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "praveSane",
    },
    Dhatu {
        id: "juz",
        code: "juz",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "prItisevanayoH",
    },
    Dhatu {
        id: "vij",
        code: "vij",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "BayacalanayoH",
    },
    Dhatu {
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
        id: "yA",
        code: "yA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        id: "vA",
        code: "vA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatigandhanayoH",
    },
    Dhatu {
        id: "ad",
        code: "ad",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BakzaRe",
    },
    Dhatu {
        id: "As",
        code: "As",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "upaveSane",
    },
    Dhatu {
        id: "vas",
        code: "vas",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "AcCAdane",
    },
    Dhatu {
        id: "SI",
        code: "SI",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "svapne",
    },
    Dhatu {
        id: "kliS",
        code: "kliS",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vibADane",
    },
    Dhatu {
        id: "guD",
        code: "guD",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "roze",
    },
    Dhatu {
        id: "aS",
        code: "aS",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Bojane",
    },
    Dhatu {
        id: "muz",
        code: "muz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "steye",
    },
    Dhatu {
        id: "vrI",
        code: "vrI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "varaRe",
    },
    Dhatu {
        id: "vf",
        code: "vf",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samBaktO",
    },
    // svādi (gaṇa 5) — vikaraṇa śnu (3.1.73)
    Dhatu {
        id: "Ap",
        code: "Ap",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyAptO",
    },
    Dhatu {
        id: "Sak",
        code: "Sak",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "SaktO",
    },
    Dhatu {
        id: "hi",
        code: "hi",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatO vfdDO ca",
    },
    Dhatu {
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
        id: "stiG",
        code: "stiG",
        gana: Gana::Svadi,
        pada: PadaAssignment::Atmanepada,
        artha: "Askandane",
    },
    Dhatu {
        // 07.0010 kftI~ vezwane. rudhādi's √kṛt, distinct from tudādi's
        // √kṛnt — not in the root set, so no id qualification is needed.
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
        id: "Banj",
        code: "Banj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Amardane",
    },
    Dhatu {
        // 07.0015 pi\zx~ saYcUrRane hiMsAyAM ca. Witnesses 8.4.41 (zwutva:
        // an adjacent dental assimilates to retroflex next to the root's
        // z) and 8.2.41 (the root's final z is itself replaced by k
        // before an s-initial affix).
        id: "piz",
        code: "piz",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "saYcUrRane hiMsAyAM ca",
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
        id: "inD",
        code: "inD",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Atmanepada,
        artha: "dIptO",
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
        assert_eq!(dhatus().len(), 48);
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
    fn rudhadi_holds_exactly_the_slice_7b_roots() {
        // Six roots, in table order. √hiṃs is stored `hins`, NOT `his`:
        // see its row comment. The gaṇa is still PARTIAL — nine of its 25
        // dhātupāṭha roots are ubhayapadī (`~^r`) and 1.3.72 is deferred,
        // so √rudh, the eponym, is absent. More roots would not change
        // that; only 1.3.72 will.
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
    fn slice_7b_ids_do_not_collide() {
        // rudhādi also holds `vi\da~\` and `o~vijI~`, which WOULD collide
        // with divādi's `vid` and tudādi's `vij`. Neither is in 7b — the
        // slice stops at six roots — so every rudhādi id is still its own
        // unqualified SLP1 code and the `aS.5` qualification mechanism
        // stays at exactly one user. 7a's spec predicted this would not
        // survive 7b; it does, because that prediction assumed a root set
        // including √vid.
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
}
