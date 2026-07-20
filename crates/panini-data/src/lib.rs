#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pada {
    Parasmaipada,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lakara {
    Lat,
    Lan,
    Lot,
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
    pub artha: &'static str,
}

static DHATUS: &[Dhatu] = &[
    Dhatu {
        code: "BU",
        gana: Gana::Bhvadi,
        artha: "sattAyAm",
    },
    Dhatu {
        code: "nI",
        gana: Gana::Bhvadi,
        artha: "prApaRe",
    },
    Dhatu {
        code: "ji",
        gana: Gana::Bhvadi,
        artha: "jaye",
    },
    Dhatu {
        code: "smf",
        gana: Gana::Bhvadi,
        artha: "cintAyAm",
    },
    Dhatu {
        code: "paW",
        gana: Gana::Bhvadi,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        code: "vad",
        gana: Gana::Bhvadi,
        artha: "vyaktAyAM vAci",
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_six_curated_roots() {
        assert_eq!(dhatus().len(), 6);
        assert!(dhatus().iter().any(|d| d.code == "BU"));
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
}
