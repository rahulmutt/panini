use panini::{Panini, Verdict};

/// (root_code, lakara_label, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]) in SLP1.
const PARADIGM: &[(&str, &str, [&str; 9])] = &[
    (
        "BU",
        "laT",
        [
            "Bavati", "BavataH", "Bavanti", "Bavasi", "BavaTaH", "BavaTa", "BavAmi", "BavAvaH",
            "BavAmaH",
        ],
    ),
    (
        "nI",
        "laT",
        [
            "nayati", "nayataH", "nayanti", "nayasi", "nayaTaH", "nayaTa", "nayAmi", "nayAvaH",
            "nayAmaH",
        ],
    ),
    (
        "ji",
        "laT",
        [
            "jayati", "jayataH", "jayanti", "jayasi", "jayaTaH", "jayaTa", "jayAmi", "jayAvaH",
            "jayAmaH",
        ],
    ),
    (
        "smf",
        "laT",
        [
            "smarati", "smarataH", "smaranti", "smarasi", "smaraTaH", "smaraTa", "smarAmi",
            "smarAvaH", "smarAmaH",
        ],
    ),
    (
        "paW",
        "laT",
        [
            "paWati", "paWataH", "paWanti", "paWasi", "paWaTaH", "paWaTa", "paWAmi", "paWAvaH",
            "paWAmaH",
        ],
    ),
    (
        "vad",
        "laT",
        [
            "vadati", "vadataH", "vadanti", "vadasi", "vadaTaH", "vadaTa", "vadAmi", "vadAvaH",
            "vadAmaH",
        ],
    ),
    (
        "BU",
        "laN",
        [
            "aBavat", "aBavatAm", "aBavan", "aBavaH", "aBavatam", "aBavata", "aBavam", "aBavAva",
            "aBavAma",
        ],
    ),
    (
        "nI",
        "laN",
        [
            "anayat", "anayatAm", "anayan", "anayaH", "anayatam", "anayata", "anayam", "anayAva",
            "anayAma",
        ],
    ),
    (
        "ji",
        "laN",
        [
            "ajayat", "ajayatAm", "ajayan", "ajayaH", "ajayatam", "ajayata", "ajayam", "ajayAva",
            "ajayAma",
        ],
    ),
    (
        "smf",
        "laN",
        [
            "asmarat",
            "asmaratAm",
            "asmaran",
            "asmaraH",
            "asmaratam",
            "asmarata",
            "asmaram",
            "asmarAva",
            "asmarAma",
        ],
    ),
    (
        "paW",
        "laN",
        [
            "apaWat", "apaWatAm", "apaWan", "apaWaH", "apaWatam", "apaWata", "apaWam", "apaWAva",
            "apaWAma",
        ],
    ),
    (
        "vad",
        "laN",
        [
            "avadat", "avadatAm", "avadan", "avadaH", "avadatam", "avadata", "avadam", "avadAva",
            "avadAma",
        ],
    ),
    (
        "BU",
        "loT",
        [
            "Bavatu", "BavatAm", "Bavantu", "Bava", "Bavatam", "Bavata", "BavAni", "BavAva",
            "BavAma",
        ],
    ),
    (
        "nI",
        "loT",
        [
            "nayatu", "nayatAm", "nayantu", "naya", "nayatam", "nayata", "nayAni", "nayAva",
            "nayAma",
        ],
    ),
    (
        "ji",
        "loT",
        [
            "jayatu", "jayatAm", "jayantu", "jaya", "jayatam", "jayata", "jayAni", "jayAva",
            "jayAma",
        ],
    ),
    (
        "smf",
        "loT",
        [
            "smaratu", "smaratAm", "smarantu", "smara", "smaratam", "smarata", "smarAni",
            "smarAva", "smarAma",
        ],
    ),
    (
        "paW",
        "loT",
        [
            "paWatu", "paWatAm", "paWantu", "paWa", "paWatam", "paWata", "paWAni", "paWAva",
            "paWAma",
        ],
    ),
    (
        "vad",
        "loT",
        [
            "vadatu", "vadatAm", "vadantu", "vada", "vadatam", "vadata", "vadAni", "vadAva",
            "vadAma",
        ],
    ),
    (
        "BU",
        "viDiliN",
        [
            "Bavet", "BavetAm", "BaveyuH", "BaveH", "Bavetam", "Baveta", "Baveyam", "Baveva",
            "Bavema",
        ],
    ),
    (
        "nI",
        "viDiliN",
        [
            "nayet", "nayetAm", "nayeyuH", "nayeH", "nayetam", "nayeta", "nayeyam", "nayeva",
            "nayema",
        ],
    ),
    (
        "ji",
        "viDiliN",
        [
            "jayet", "jayetAm", "jayeyuH", "jayeH", "jayetam", "jayeta", "jayeyam", "jayeva",
            "jayema",
        ],
    ),
    (
        "smf",
        "viDiliN",
        [
            "smaret", "smaretAm", "smareyuH", "smareH", "smaretam", "smareta", "smareyam",
            "smareva", "smarema",
        ],
    ),
    (
        "paW",
        "viDiliN",
        [
            "paWet", "paWetAm", "paWeyuH", "paWeH", "paWetam", "paWeta", "paWeyam", "paWeva",
            "paWema",
        ],
    ),
    (
        "vad",
        "viDiliN",
        [
            "vadet", "vadetAm", "vadeyuH", "vadeH", "vadetam", "vadeta", "vadeyam", "vadeva",
            "vadema",
        ],
    ),
];

#[test]
fn every_form_validates_and_matches() {
    let engine = Panini::new();
    for (root, lakara, forms) in PARADIGM {
        for expected in forms {
            let r = engine.check(expected);
            assert!(
                matches!(r.verdict, Verdict::Valid),
                "expected VALID for {expected} ({root} {lakara})"
            );
            assert!(
                r.analyses.iter().any(|a| a.form_slp1 == *expected
                    && a.dhatu == *root
                    && panini::lakara_name(a.lakara) == *lakara),
                "no {lakara} analysis of {root} produced {expected}"
            );
        }
    }
}

#[test]
fn known_nonforms_are_invalid() {
    let engine = Panini::new();
    for bad in [
        // Real cross-lakāra confusions, not junk: laṅ endings require the
        // aṭ-āgama (6.4.71), and laṭ endings forbid it.
        "Bavat",    // laṅ 3sg ending without the augment
        "aBavanti", // augment on a laṭ form
        "aBavatu",  // augment on a loṭ form
        "aBavet",   // laṅ's aṭ-āgama on a vidhiliṅ form
        "Bavetu",   // loṭ's er uḥ ending on a vidhiliṅ stem
        // Still out of scope entirely.
        "gacCati",
        "Bavati123",
        "tiRRati",
        // Wrong pada: the root's pada tag gates the whole derivation
        // (1.3.12 / 1.3.78) and the analyzer proposes only the tagged pada.
        "laBati", // atmanepadin root with a parasmaipada ending
        "Bavate", // parasmaipada root with an atmanepada ending
        "eDati",  // vowel-initial atmanepadin root, parasmaipada ending
        "alaBat", // laN parasmaipada shape on an atmanepadin root
        "laB",    // a bare root code is not a surface form
    ] {
        assert!(
            matches!(engine.check(bad).verdict, Verdict::Invalid),
            "expected INVALID for {bad}"
        );
    }
}
