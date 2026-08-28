//! adadi's golden rows. See `super` (`data/mod.rs`) for the row
//! contracts and the concatenated `PARADIGM` / `ALTERNATES` statics.

use panini_data::Pada;

use super::{AlternateRow, ParadigmRow};

pub const PARADIGM: &[ParadigmRow] = &[
    (
        "02.0044",
        "laT",
        Pada::Parasmaipada,
        [
            "yAti", "yAtaH", "yAnti", "yAsi", "yATaH", "yATa", "yAmi", "yAvaH", "yAmaH",
        ],
    ),
    (
        "02.0045",
        "laT",
        Pada::Parasmaipada,
        [
            "vAti", "vAtaH", "vAnti", "vAsi", "vATaH", "vATa", "vAmi", "vAvaH", "vAmaH",
        ],
    ),
    (
        "02.0044",
        "laN",
        Pada::Parasmaipada,
        [
            "ayAd", "ayAtAm", "ayAn", "ayAH", "ayAtam", "ayAta", "ayAm", "ayAva", "ayAma",
        ],
    ),
    (
        "02.0045",
        "laN",
        Pada::Parasmaipada,
        [
            "avAd", "avAtAm", "avAn", "avAH", "avAtam", "avAta", "avAm", "avAva", "avAma",
        ],
    ),
    (
        "02.0044",
        "loT",
        Pada::Parasmaipada,
        [
            "yAtu", "yAtAm", "yAntu", "yAhi", "yAtam", "yAta", "yAni", "yAva", "yAma",
        ],
    ),
    (
        "02.0045",
        "loT",
        Pada::Parasmaipada,
        [
            "vAtu", "vAtAm", "vAntu", "vAhi", "vAtam", "vAta", "vAni", "vAva", "vAma",
        ],
    ),
    (
        "02.0044",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "yAyAd", "yAyAtAm", "yAyuH", "yAyAH", "yAyAtam", "yAyAta", "yAyAm", "yAyAva", "yAyAma",
        ],
    ),
    (
        "02.0045",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "vAyAd", "vAyAtAm", "vAyuH", "vAyAH", "vAyAtam", "vAyAta", "vAyAm", "vAyAva", "vAyAma",
        ],
    ),
    (
        "02.0001",
        "laT",
        Pada::Parasmaipada,
        [
            "atti", "attaH", "adanti", "atsi", "atTaH", "atTa", "admi", "advaH", "admaH",
        ],
    ),
    (
        "02.0001",
        "laN",
        Pada::Parasmaipada,
        [
            "Adad", "AttAm", "Adan", "AdaH", "Attam", "Atta", "Adam", "Adva", "Adma",
        ],
    ),
    (
        "02.0001",
        "loT",
        Pada::Parasmaipada,
        [
            "attu", "attAm", "adantu", "adDi", "attam", "atta", "adAni", "adAva", "adAma",
        ],
    ),
    (
        "02.0001",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "adyAd", "adyAtAm", "adyuH", "adyAH", "adyAtam", "adyAta", "adyAm", "adyAva", "adyAma",
        ],
    ),
    (
        "02.0011",
        "laT",
        Pada::Atmanepada,
        [
            "Aste", "AsAte", "Asate", "Asse", "AsATe", "ADve", "Ase", "Asvahe", "Asmahe",
        ],
    ),
    (
        "02.0011",
        "laN",
        Pada::Atmanepada,
        [
            "Asta", "AsAtAm", "Asata", "AsTAH", "AsATAm", "ADvam", "Asi", "Asvahi", "Asmahi",
        ],
    ),
    (
        "02.0011",
        "loT",
        Pada::Atmanepada,
        [
            "AstAm", "AsAtAm", "AsatAm", "Assva", "AsATAm", "ADvam", "AsE", "AsAvahE", "AsAmahE",
        ],
    ),
    (
        "02.0011",
        "viDiliN",
        Pada::Atmanepada,
        [
            "AsIta", "AsIyAtAm", "AsIran", "AsITAH", "AsIyATAm", "AsIDvam", "AsIya", "AsIvahi",
            "AsImahi",
        ],
    ),
    (
        "02.0013",
        "laT",
        Pada::Atmanepada,
        [
            "vaste", "vasAte", "vasate", "vasse", "vasATe", "vaDve", "vase", "vasvahe", "vasmahe",
        ],
    ),
    (
        "02.0013",
        "laN",
        Pada::Atmanepada,
        [
            "avasta", "avasAtAm", "avasata", "avasTAH", "avasATAm", "avaDvam", "avasi", "avasvahi",
            "avasmahi",
        ],
    ),
    (
        "02.0013",
        "loT",
        Pada::Atmanepada,
        [
            "vastAm", "vasAtAm", "vasatAm", "vassva", "vasATAm", "vaDvam", "vasE", "vasAvahE",
            "vasAmahE",
        ],
    ),
    (
        "02.0013",
        "viDiliN",
        Pada::Atmanepada,
        [
            "vasIta",
            "vasIyAtAm",
            "vasIran",
            "vasITAH",
            "vasIyATAm",
            "vasIDvam",
            "vasIya",
            "vasIvahi",
            "vasImahi",
        ],
    ),
    (
        "02.0026",
        "laT",
        Pada::Atmanepada,
        [
            "Sete", "SayAte", "Serate", "Seze", "SayATe", "SeDve", "Saye", "Sevahe", "Semahe",
        ],
    ),
    (
        "02.0026",
        "laN",
        Pada::Atmanepada,
        [
            "aSeta", "aSayAtAm", "aSerata", "aSeTAH", "aSayATAm", "aSeDvam", "aSayi", "aSevahi",
            "aSemahi",
        ],
    ),
    (
        "02.0026",
        "loT",
        Pada::Atmanepada,
        [
            "SetAm", "SayAtAm", "SeratAm", "Sezva", "SayATAm", "SeDvam", "SayE", "SayAvahE",
            "SayAmahE",
        ],
    ),
    (
        "02.0026",
        "viDiliN",
        Pada::Atmanepada,
        [
            "SayIta",
            "SayIyAtAm",
            "SayIran",
            "SayITAH",
            "SayIyATAm",
            "SayIDvam",
            "SayIya",
            "SayIvahi",
            "SayImahi",
        ],
    ),
];

pub const ALTERNATES: &[AlternateRow] = &[
    ("02.0044", "laN", Pada::Parasmaipada, 0, "ayAt", "8.4.56"),
    ("02.0045", "laN", Pada::Parasmaipada, 0, "avAt", "8.4.56"),
    (
        "02.0044",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "yAyAt",
        "8.4.56",
    ),
    (
        "02.0045",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "vAyAt",
        "8.4.56",
    ),
    ("02.0001", "laN", Pada::Parasmaipada, 0, "Adat", "8.4.56"),
    (
        "02.0001",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "adyAt",
        "8.4.56",
    ),
    ("02.0044", "loT", Pada::Parasmaipada, 0, "yAtAd", "7.1.35"),
    (
        "02.0044",
        "loT",
        Pada::Parasmaipada,
        0,
        "yAtAt",
        "7.1.35+8.4.56",
    ),
    ("02.0044", "loT", Pada::Parasmaipada, 3, "yAtAd", "7.1.35"),
    (
        "02.0044",
        "loT",
        Pada::Parasmaipada,
        3,
        "yAtAt",
        "7.1.35+8.4.56",
    ),
    ("02.0045", "loT", Pada::Parasmaipada, 0, "vAtAd", "7.1.35"),
    (
        "02.0045",
        "loT",
        Pada::Parasmaipada,
        0,
        "vAtAt",
        "7.1.35+8.4.56",
    ),
    ("02.0045", "loT", Pada::Parasmaipada, 3, "vAtAd", "7.1.35"),
    (
        "02.0045",
        "loT",
        Pada::Parasmaipada,
        3,
        "vAtAt",
        "7.1.35+8.4.56",
    ),
    ("02.0001", "loT", Pada::Parasmaipada, 0, "attAd", "7.1.35"),
    (
        "02.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "attAt",
        "7.1.35+8.4.56",
    ),
    ("02.0001", "loT", Pada::Parasmaipada, 3, "attAd", "7.1.35"),
    (
        "02.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "attAt",
        "7.1.35+8.4.56",
    ),
    ("02.0044", "laN", Pada::Parasmaipada, 2, "ayuH", "3.4.111"),
    ("02.0045", "laN", Pada::Parasmaipada, 2, "avuH", "3.4.111"),
];
