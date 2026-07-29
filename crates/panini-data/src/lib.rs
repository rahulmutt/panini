#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
    Adadi,
    Kryadi,
    Svadi,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pada {
    Parasmaipada,
    Atmanepada,
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
    /// Unique lookup key. Equal to `code` except where two roots in
    /// different gaṇas share an SLP1 form, in which case it is
    /// gaṇa-qualified (`aS.5` vs `aS.9`). Never hand this to `Term::new`.
    pub id: &'static str,
    /// The root's SLP1 text, as it enters the derivation.
    pub code: &'static str,
    pub gana: Gana,
    /// Which pada this root takes. Ubhayapadi roots are out of scope; each
    /// curated root has exactly one pada.
    pub pada: Pada,
    pub artha: &'static str,
}

static DHATUS: &[Dhatu] = &[
    Dhatu {
        id: "BU",
        code: "BU",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "sattAyAm",
    },
    Dhatu {
        id: "nI",
        code: "nI",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        id: "ji",
        code: "ji",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "jaye",
    },
    Dhatu {
        id: "smf",
        code: "smf",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "cintAyAm",
    },
    Dhatu {
        id: "paW",
        code: "paW",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        id: "vad",
        code: "vad",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        id: "eD",
        code: "eD",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vfdDO",
    },
    Dhatu {
        id: "laB",
        code: "laB",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "prAptO",
    },
    Dhatu {
        id: "sev",
        code: "sev",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "sevane",
    },
    Dhatu {
        id: "vft",
        code: "vft",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vartane",
    },
    Dhatu {
        id: "BAz",
        code: "BAz",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        id: "Ikz",
        code: "Ikz",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "darSane",
    },
    // divādi (gaṇa 4) — vikaraṇa śyan (3.1.69)
    Dhatu {
        id: "div",
        code: "div",
        gana: Gana::Divadi,
        pada: Pada::Parasmaipada,
        artha: "krIqAyAm",
    },
    Dhatu {
        id: "naS",
        code: "naS",
        gana: Gana::Divadi,
        pada: Pada::Parasmaipada,
        artha: "adarSane",
    },
    Dhatu {
        id: "kup",
        code: "kup",
        gana: Gana::Divadi,
        pada: Pada::Parasmaipada,
        artha: "kroDe",
    },
    Dhatu {
        id: "man",
        code: "man",
        gana: Gana::Divadi,
        pada: Pada::Atmanepada,
        artha: "jYAne",
    },
    Dhatu {
        id: "yuD",
        code: "yuD",
        gana: Gana::Divadi,
        pada: Pada::Atmanepada,
        artha: "samprahAre",
    },
    Dhatu {
        id: "vid",
        code: "vid",
        gana: Gana::Divadi,
        pada: Pada::Atmanepada,
        artha: "sattAyAm",
    },
    // tudādi (gaṇa 6) — vikaraṇa śa (3.1.77)
    Dhatu {
        id: "tud",
        code: "tud",
        gana: Gana::Tudadi,
        pada: Pada::Parasmaipada,
        artha: "vyaTane",
    },
    Dhatu {
        id: "liK",
        code: "liK",
        gana: Gana::Tudadi,
        pada: Pada::Parasmaipada,
        artha: "akzaravinyAse",
    },
    Dhatu {
        id: "viS",
        code: "viS",
        gana: Gana::Tudadi,
        pada: Pada::Parasmaipada,
        artha: "praveSane",
    },
    Dhatu {
        id: "juz",
        code: "juz",
        gana: Gana::Tudadi,
        pada: Pada::Atmanepada,
        artha: "prItisevanayoH",
    },
    Dhatu {
        id: "vij",
        code: "vij",
        gana: Gana::Tudadi,
        pada: Pada::Atmanepada,
        artha: "BayacalanayoH",
    },
    Dhatu {
        id: "gur",
        code: "gur",
        gana: Gana::Tudadi,
        pada: Pada::Atmanepada,
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
        pada: Pada::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        id: "vA",
        code: "vA",
        gana: Gana::Adadi,
        pada: Pada::Parasmaipada,
        artha: "gatigandhanayoH",
    },
    Dhatu {
        id: "ad",
        code: "ad",
        gana: Gana::Adadi,
        pada: Pada::Parasmaipada,
        artha: "BakzaRe",
    },
    Dhatu {
        id: "As",
        code: "As",
        gana: Gana::Adadi,
        pada: Pada::Atmanepada,
        artha: "upaveSane",
    },
    Dhatu {
        id: "vas",
        code: "vas",
        gana: Gana::Adadi,
        pada: Pada::Atmanepada,
        artha: "AcCAdane",
    },
    Dhatu {
        id: "SI",
        code: "SI",
        gana: Gana::Adadi,
        pada: Pada::Atmanepada,
        artha: "svapne",
    },
    Dhatu {
        id: "kliS",
        code: "kliS",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "vibADane",
    },
    Dhatu {
        id: "guD",
        code: "guD",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "roze",
    },
    Dhatu {
        id: "aS",
        code: "aS",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "Bojane",
    },
    Dhatu {
        id: "muz",
        code: "muz",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "steye",
    },
    Dhatu {
        id: "vrI",
        code: "vrI",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "varaRe",
    },
    Dhatu {
        id: "vf",
        code: "vf",
        gana: Gana::Kryadi,
        pada: Pada::Atmanepada,
        artha: "samBaktO",
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
        assert_eq!(dhatus().len(), 36);
        let bu = dhatus().iter().find(|d| d.id == "BU").unwrap();
        assert!(matches!(bu.pada, Pada::Parasmaipada));
        let labh = dhatus().iter().find(|d| d.id == "laB").unwrap();
        assert!(matches!(labh.pada, Pada::Atmanepada));
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
        assert!(matches!(ya.gana, Gana::Adadi) && matches!(ya.pada, Pada::Parasmaipada));
        let va = dhatus().iter().find(|d| d.id == "vA").unwrap();
        assert!(matches!(va.gana, Gana::Adadi) && matches!(va.pada, Pada::Parasmaipada));
        // adādi ātmanepada: √ās (slice 5d), √vas (slice 5e), and √śī (this slice) closes gaṇa.
        let as_ = dhatus().iter().find(|d| d.id == "As").unwrap();
        assert!(matches!(as_.gana, Gana::Adadi) && matches!(as_.pada, Pada::Atmanepada));
        let vas = dhatus().iter().find(|d| d.id == "vas").unwrap();
        assert!(matches!(vas.gana, Gana::Adadi) && matches!(vas.pada, Pada::Atmanepada));
        // √vas ācchādane (2Ā), not √vas nivāse (1P) — artha disambiguates.
        assert_eq!(vas.artha, "AcCAdane");
        // adādi ātmanepada: √śī (this slice) closes the gaṇa.
        let shi = dhatus().iter().find(|d| d.id == "SI").unwrap();
        assert!(matches!(shi.gana, Gana::Adadi) && matches!(shi.pada, Pada::Atmanepada));
        assert_eq!(shi.artha, "svapne");
        // kryādi (gaṇa 9), slice 9a: kliS/guD/aS, all parasmaipada.
        let klis = dhatus().iter().find(|d| d.id == "kliS").unwrap();
        assert!(matches!(klis.gana, Gana::Kryadi) && matches!(klis.pada, Pada::Parasmaipada));
        assert_eq!(klis.artha, "vibADane");
        let gud = dhatus().iter().find(|d| d.id == "guD").unwrap();
        assert!(matches!(gud.gana, Gana::Kryadi) && matches!(gud.pada, Pada::Parasmaipada));
        assert_eq!(gud.artha, "roze");
        let ash = dhatus().iter().find(|d| d.id == "aS").unwrap();
        assert!(matches!(ash.gana, Gana::Kryadi) && matches!(ash.pada, Pada::Parasmaipada));
        assert_eq!(ash.artha, "Bojane");
        // kryādi, slice 9b: muz/vrI parasmaipada, vf (√vṛṅ) atmanepada --
        // the gaṇa's only pure-atmanepadi root.
        let muz = dhatus().iter().find(|d| d.id == "muz").unwrap();
        assert!(matches!(muz.gana, Gana::Kryadi) && matches!(muz.pada, Pada::Parasmaipada));
        assert_eq!(muz.artha, "steye");
        let vri = dhatus().iter().find(|d| d.id == "vrI").unwrap();
        assert!(matches!(vri.gana, Gana::Kryadi) && matches!(vri.pada, Pada::Parasmaipada));
        assert_eq!(vri.artha, "varaRe");
        let vf = dhatus().iter().find(|d| d.id == "vf").unwrap();
        assert!(matches!(vf.gana, Gana::Kryadi) && matches!(vf.pada, Pada::Atmanepada));
        assert_eq!(vf.artha, "samBaktO");
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
        assert!(matches!(ad.pada, Pada::Parasmaipada));
        assert_eq!(ad.artha, "BakzaRe");
    }

    #[test]
    fn as_is_registered_as_adadi_atmanepada() {
        let as_ = dhatus().iter().find(|d| d.id == "As").expect("√ās present");
        assert!(matches!(as_.gana, Gana::Adadi));
        assert!(matches!(as_.pada, Pada::Atmanepada));
        assert_eq!(as_.artha, "upaveSane");
    }

    #[test]
    fn id_is_the_lookup_key_and_is_unique() {
        let ids: Vec<&str> = dhatus().iter().map(|d| d.id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "dhatu ids must be unique");
        // Until svādi lands every id equals its code; the field exists so that
        // stops being required.
        for d in dhatus() {
            assert!(!d.id.is_empty());
        }
    }
}
