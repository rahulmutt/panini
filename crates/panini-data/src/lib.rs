#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
    Adadi,
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
    // adādi (gaṇa 2) — śap luk (2.4.72); √ad/√yā/√vā, the parasmaipada
    // roots, are covered across all four lakāras (laṭ/laṅ/loṭ/vidhiliṅ).
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
    fn has_twentysix_curated_roots_with_padas() {
        assert_eq!(dhatus().len(), 27);
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
}
