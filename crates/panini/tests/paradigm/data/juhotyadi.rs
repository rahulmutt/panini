//! juhotyadi's golden rows. See `super` (`data/mod.rs`) for the row
//! contracts and the concatenated `PARADIGM` / `ALTERNATES` statics.

use panini_data::Pada;

use super::{AlternateRow, ParadigmRow};

pub const PARADIGM: &[ParadigmRow] = &[
    (
        "03.0001",
        "laT",
        Pada::Parasmaipada,
        [
            "juhoti", "juhutaH", "juhvati", "juhozi", "juhuTaH", "juhuTa", "juhomi", "juhuvaH",
            "juhumaH",
        ],
    ),
    (
        "03.0001",
        "laN",
        Pada::Parasmaipada,
        [
            "ajuhod", "ajuhutAm", "ajuhavuH", "ajuhoH", "ajuhutam", "ajuhuta", "ajuhavam",
            "ajuhuva", "ajuhuma",
        ],
    ),
    (
        "03.0001",
        "loT",
        Pada::Parasmaipada,
        [
            "juhotu", "juhutAm", "juhvatu", "juhuDi", "juhutam", "juhuta", "juhavAni", "juhavAva",
            "juhavAma",
        ],
    ),
    (
        "03.0001",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "juhuyAd",
            "juhuyAtAm",
            "juhuyuH",
            "juhuyAH",
            "juhuyAtam",
            "juhuyAta",
            "juhuyAm",
            "juhuyAva",
            "juhuyAma",
        ],
    ),
    (
        "03.0020",
        "laT",
        Pada::Parasmaipada,
        [
            "ciketi", "cikitaH", "cikyati", "cikezi", "cikiTaH", "cikiTa", "cikemi", "cikivaH",
            "cikimaH",
        ],
    ),
    (
        "03.0020",
        "laN",
        Pada::Parasmaipada,
        [
            "aciked", "acikitAm", "acikayuH", "acikeH", "acikitam", "acikita", "acikayam",
            "acikiva", "acikima",
        ],
    ),
    (
        "03.0020",
        "loT",
        Pada::Parasmaipada,
        [
            "ciketu", "cikitAm", "cikyatu", "cikihi", "cikitam", "cikita", "cikayAni", "cikayAva",
            "cikayAma",
        ],
    ),
    (
        "03.0020",
        "viDiliN",
        Pada::Parasmaipada,
        [
            "cikiyAd",
            "cikiyAtAm",
            "cikiyuH",
            "cikiyAH",
            "cikiyAtam",
            "cikiyAta",
            "cikiyAm",
            "cikiyAva",
            "cikiyAma",
        ],
    ),
];

pub const ALTERNATES: &[AlternateRow] = &[
    ("03.0001", "laN", Pada::Parasmaipada, 0, "ajuhot", "8.4.56"),
    ("03.0001", "loT", Pada::Parasmaipada, 0, "juhutAd", "7.1.35"),
    (
        "03.0001",
        "loT",
        Pada::Parasmaipada,
        0,
        "juhutAt",
        "7.1.35+8.4.56",
    ),
    ("03.0001", "loT", Pada::Parasmaipada, 3, "juhutAd", "7.1.35"),
    (
        "03.0001",
        "loT",
        Pada::Parasmaipada,
        3,
        "juhutAt",
        "7.1.35+8.4.56",
    ),
    (
        "03.0001",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "juhuyAt",
        "8.4.56",
    ),
    ("03.0020", "laN", Pada::Parasmaipada, 0, "aciket", "8.4.56"),
    ("03.0020", "loT", Pada::Parasmaipada, 0, "cikitAd", "7.1.35"),
    (
        "03.0020",
        "loT",
        Pada::Parasmaipada,
        0,
        "cikitAt",
        "7.1.35+8.4.56",
    ),
    ("03.0020", "loT", Pada::Parasmaipada, 3, "cikitAd", "7.1.35"),
    (
        "03.0020",
        "loT",
        Pada::Parasmaipada,
        3,
        "cikitAt",
        "7.1.35+8.4.56",
    ),
    (
        "03.0020",
        "viDiliN",
        Pada::Parasmaipada,
        0,
        "cikiyAt",
        "8.4.56",
    ),
];
