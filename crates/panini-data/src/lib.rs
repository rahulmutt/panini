#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
    Adadi,
    Kryadi,
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
    pub code: &'static str,
    pub gana: Gana,
    /// Which pada this root takes. Ubhayapadi roots are out of scope; each
    /// curated root has exactly one pada.
    pub pada: Pada,
    pub artha: &'static str,
}

static DHATUS: &[Dhatu] = &[
    Dhatu {
        code: "BU",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "sattAyAm",
    },
    Dhatu {
        code: "nI",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        code: "ji",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "jaye",
    },
    Dhatu {
        code: "smf",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "cintAyAm",
    },
    Dhatu {
        code: "paW",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        code: "vad",
        gana: Gana::Bhvadi,
        pada: Pada::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        code: "eD",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vfdDO",
    },
    Dhatu {
        code: "laB",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "prAptO",
    },
    Dhatu {
        code: "sev",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "sevane",
    },
    Dhatu {
        code: "vft",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vartane",
    },
    Dhatu {
        code: "BAz",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        code: "Ikz",
        gana: Gana::Bhvadi,
        pada: Pada::Atmanepada,
        artha: "darSane",
    },
    // divādi (gaṇa 4) — vikaraṇa śyan (3.1.69)
    Dhatu {
        code: "div",
        gana: Gana::Divadi,
        pada: Pada::Parasmaipada,
        artha: "krIqAyAm",
    },
    Dhatu {
        code: "naS",
        gana: Gana::Divadi,
        pada: Pada::Parasmaipada,
        artha: "adarSane",
    },
    Dhatu {
        code: "kup",
        gana: Gana::Divadi,
        pada: Pada::Parasmaipada,
        artha: "kroDe",
    },
    Dhatu {
        code: "man",
        gana: Gana::Divadi,
        pada: Pada::Atmanepada,
        artha: "jYAne",
    },
    Dhatu {
        code: "yuD",
        gana: Gana::Divadi,
        pada: Pada::Atmanepada,
        artha: "samprahAre",
    },
    Dhatu {
        code: "vid",
        gana: Gana::Divadi,
        pada: Pada::Atmanepada,
        artha: "sattAyAm",
    },
    // tudādi (gaṇa 6) — vikaraṇa śa (3.1.77)
    Dhatu {
        code: "tud",
        gana: Gana::Tudadi,
        pada: Pada::Parasmaipada,
        artha: "vyaTane",
    },
    Dhatu {
        code: "liK",
        gana: Gana::Tudadi,
        pada: Pada::Parasmaipada,
        artha: "akzaravinyAse",
    },
    Dhatu {
        code: "viS",
        gana: Gana::Tudadi,
        pada: Pada::Parasmaipada,
        artha: "praveSane",
    },
    Dhatu {
        code: "juz",
        gana: Gana::Tudadi,
        pada: Pada::Atmanepada,
        artha: "prItisevanayoH",
    },
    Dhatu {
        code: "vij",
        gana: Gana::Tudadi,
        pada: Pada::Atmanepada,
        artha: "BayacalanayoH",
    },
    Dhatu {
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
        code: "yA",
        gana: Gana::Adadi,
        pada: Pada::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        code: "vA",
        gana: Gana::Adadi,
        pada: Pada::Parasmaipada,
        artha: "gatigandhanayoH",
    },
    Dhatu {
        code: "ad",
        gana: Gana::Adadi,
        pada: Pada::Parasmaipada,
        artha: "BakzaRe",
    },
    Dhatu {
        code: "As",
        gana: Gana::Adadi,
        pada: Pada::Atmanepada,
        artha: "upaveSane",
    },
    Dhatu {
        code: "vas",
        gana: Gana::Adadi,
        pada: Pada::Atmanepada,
        artha: "AcCAdane",
    },
    Dhatu {
        code: "SI",
        gana: Gana::Adadi,
        pada: Pada::Atmanepada,
        artha: "svapne",
    },
    Dhatu {
        code: "kliS",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "vibADane",
    },
    Dhatu {
        code: "guD",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "roze",
    },
    Dhatu {
        code: "aS",
        gana: Gana::Kryadi,
        pada: Pada::Parasmaipada,
        artha: "Bojane",
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
        assert_eq!(dhatus().len(), 33);
        let bu = dhatus().iter().find(|d| d.code == "BU").unwrap();
        assert!(matches!(bu.pada, Pada::Parasmaipada));
        let labh = dhatus().iter().find(|d| d.code == "laB").unwrap();
        assert!(matches!(labh.pada, Pada::Atmanepada));
        // Both vowel-initial atmanepadi roots must be present (they exercise
        // the AT-augment path 6.4.72/6.1.90).
        assert!(dhatus().iter().any(|d| d.code == "eD"));
        assert!(dhatus().iter().any(|d| d.code == "Ikz"));
        // Divadi/tudadi still present.
        let div = dhatus().iter().find(|d| d.code == "div").unwrap();
        assert!(matches!(div.gana, Gana::Divadi));
        let tud = dhatus().iter().find(|d| d.code == "tud").unwrap();
        assert!(matches!(tud.gana, Gana::Tudadi));
        // New: adadi (gaṇa 2), both ā-final parasmaipada.
        let ya = dhatus().iter().find(|d| d.code == "yA").unwrap();
        assert!(matches!(ya.gana, Gana::Adadi) && matches!(ya.pada, Pada::Parasmaipada));
        let va = dhatus().iter().find(|d| d.code == "vA").unwrap();
        assert!(matches!(va.gana, Gana::Adadi) && matches!(va.pada, Pada::Parasmaipada));
        // adādi ātmanepada: √ās (slice 5d), √vas (slice 5e), and √śī (this slice) closes gaṇa.
        let as_ = dhatus().iter().find(|d| d.code == "As").unwrap();
        assert!(matches!(as_.gana, Gana::Adadi) && matches!(as_.pada, Pada::Atmanepada));
        let vas = dhatus().iter().find(|d| d.code == "vas").unwrap();
        assert!(matches!(vas.gana, Gana::Adadi) && matches!(vas.pada, Pada::Atmanepada));
        // √vas ācchādane (2Ā), not √vas nivāse (1P) — artha disambiguates.
        assert_eq!(vas.artha, "AcCAdane");
        // adādi ātmanepada: √śī (this slice) closes the gaṇa.
        let shi = dhatus().iter().find(|d| d.code == "SI").unwrap();
        assert!(matches!(shi.gana, Gana::Adadi) && matches!(shi.pada, Pada::Atmanepada));
        assert_eq!(shi.artha, "svapne");
        // kryādi (gaṇa 9), slice 9a: kliS/guD/aS, all parasmaipada.
        let klis = dhatus().iter().find(|d| d.code == "kliS").unwrap();
        assert!(matches!(klis.gana, Gana::Kryadi) && matches!(klis.pada, Pada::Parasmaipada));
        assert_eq!(klis.artha, "vibADane");
        let gud = dhatus().iter().find(|d| d.code == "guD").unwrap();
        assert!(matches!(gud.gana, Gana::Kryadi) && matches!(gud.pada, Pada::Parasmaipada));
        assert_eq!(gud.artha, "roze");
        let ash = dhatus().iter().find(|d| d.code == "aS").unwrap();
        assert!(matches!(ash.gana, Gana::Kryadi) && matches!(ash.pada, Pada::Parasmaipada));
        assert_eq!(ash.artha, "Bojane");
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
            .find(|d| d.code == "ad")
            .expect("√ad present");
        assert!(matches!(ad.gana, Gana::Adadi));
        assert!(matches!(ad.pada, Pada::Parasmaipada));
        assert_eq!(ad.artha, "BakzaRe");
    }

    #[test]
    fn as_is_registered_as_adadi_atmanepada() {
        let as_ = dhatus()
            .iter()
            .find(|d| d.code == "As")
            .expect("√ās present");
        assert!(matches!(as_.gana, Gana::Adadi));
        assert!(matches!(as_.pada, Pada::Atmanepada));
        assert_eq!(as_.artha, "upaveSane");
    }
}
