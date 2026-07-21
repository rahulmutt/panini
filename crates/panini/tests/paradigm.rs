use panini::{Panini, Verdict};
use panini_data::{Lakara, Pada, Purusha, Vacana, dhatus};
use panini_prakriya::derive;

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
    (
        "eD",
        "laT",
        [
            "eDate", "eDete", "eDante", "eDase", "eDeTe", "eDaDve", "eDe", "eDAvahe", "eDAmahe",
        ],
    ),
    (
        "laB",
        "laT",
        [
            "laBate", "laBete", "laBante", "laBase", "laBeTe", "laBaDve", "laBe", "laBAvahe",
            "laBAmahe",
        ],
    ),
    (
        "sev",
        "laT",
        [
            "sevate", "sevete", "sevante", "sevase", "seveTe", "sevaDve", "seve", "sevAvahe",
            "sevAmahe",
        ],
    ),
    (
        "vft",
        "laT",
        [
            "vartate",
            "vartete",
            "vartante",
            "vartase",
            "varteTe",
            "vartaDve",
            "varte",
            "vartAvahe",
            "vartAmahe",
        ],
    ),
    (
        "BAz",
        "laT",
        [
            "BAzate", "BAzete", "BAzante", "BAzase", "BAzeTe", "BAzaDve", "BAze", "BAzAvahe",
            "BAzAmahe",
        ],
    ),
    (
        "Ikz",
        "laT",
        [
            "Ikzate", "Ikzete", "Ikzante", "Ikzase", "IkzeTe", "IkzaDve", "Ikze", "IkzAvahe",
            "IkzAmahe",
        ],
    ),
    (
        "eD",
        "loT",
        [
            "eDatAm", "eDetAm", "eDantAm", "eDasva", "eDeTAm", "eDaDvam", "eDE", "eDAvahE",
            "eDAmahE",
        ],
    ),
    (
        "laB",
        "loT",
        [
            "laBatAm", "laBetAm", "laBantAm", "laBasva", "laBeTAm", "laBaDvam", "laBE", "laBAvahE",
            "laBAmahE",
        ],
    ),
    (
        "sev",
        "loT",
        [
            "sevatAm", "sevetAm", "sevantAm", "sevasva", "seveTAm", "sevaDvam", "sevE", "sevAvahE",
            "sevAmahE",
        ],
    ),
    (
        "vft",
        "loT",
        [
            "vartatAm",
            "vartetAm",
            "vartantAm",
            "vartasva",
            "varteTAm",
            "vartaDvam",
            "vartE",
            "vartAvahE",
            "vartAmahE",
        ],
    ),
    (
        "BAz",
        "loT",
        [
            "BAzatAm", "BAzetAm", "BAzantAm", "BAzasva", "BAzeTAm", "BAzaDvam", "BAzE", "BAzAvahE",
            "BAzAmahE",
        ],
    ),
    (
        "Ikz",
        "loT",
        [
            "IkzatAm", "IkzetAm", "IkzantAm", "Ikzasva", "IkzeTAm", "IkzaDvam", "IkzE", "IkzAvahE",
            "IkzAmahE",
        ],
    ),
    (
        "eD",
        "laN",
        [
            "EData", "EDetAm", "EDanta", "EDaTAH", "EDeTAm", "EDaDvam", "EDe", "EDAvahi", "EDAmahi",
        ],
    ),
    (
        "laB",
        "laN",
        [
            "alaBata",
            "alaBetAm",
            "alaBanta",
            "alaBaTAH",
            "alaBeTAm",
            "alaBaDvam",
            "alaBe",
            "alaBAvahi",
            "alaBAmahi",
        ],
    ),
    (
        "sev",
        "laN",
        [
            "asevata",
            "asevetAm",
            "asevanta",
            "asevaTAH",
            "aseveTAm",
            "asevaDvam",
            "aseve",
            "asevAvahi",
            "asevAmahi",
        ],
    ),
    (
        "vft",
        "laN",
        [
            "avartata",
            "avartetAm",
            "avartanta",
            "avartaTAH",
            "avarteTAm",
            "avartaDvam",
            "avarte",
            "avartAvahi",
            "avartAmahi",
        ],
    ),
    (
        "BAz",
        "laN",
        [
            "aBAzata",
            "aBAzetAm",
            "aBAzanta",
            "aBAzaTAH",
            "aBAzeTAm",
            "aBAzaDvam",
            "aBAze",
            "aBAzAvahi",
            "aBAzAmahi",
        ],
    ),
    (
        "Ikz",
        "laN",
        [
            "Ekzata", "EkzetAm", "Ekzanta", "EkzaTAH", "EkzeTAm", "EkzaDvam", "Ekze", "EkzAvahi",
            "EkzAmahi",
        ],
    ),
    (
        "eD",
        "viDiliN",
        [
            "eDeta", "eDeyAtAm", "eDeran", "eDeTAH", "eDeyATAm", "eDeDvam", "eDeya", "eDevahi",
            "eDemahi",
        ],
    ),
    (
        "laB",
        "viDiliN",
        [
            "laBeta",
            "laBeyAtAm",
            "laBeran",
            "laBeTAH",
            "laBeyATAm",
            "laBeDvam",
            "laBeya",
            "laBevahi",
            "laBemahi",
        ],
    ),
    (
        "sev",
        "viDiliN",
        [
            "seveta",
            "seveyAtAm",
            "severan",
            "seveTAH",
            "seveyATAm",
            "seveDvam",
            "seveya",
            "sevevahi",
            "sevemahi",
        ],
    ),
    (
        "vft",
        "viDiliN",
        [
            "varteta",
            "varteyAtAm",
            "varteran",
            "varteTAH",
            "varteyATAm",
            "varteDvam",
            "varteya",
            "vartevahi",
            "vartemahi",
        ],
    ),
    (
        "BAz",
        "viDiliN",
        [
            "BAzeta",
            "BAzeyAtAm",
            "BAzeran",
            "BAzeTAH",
            "BAzeyATAm",
            "BAzeDvam",
            "BAzeya",
            "BAzevahi",
            "BAzemahi",
        ],
    ),
    (
        "Ikz",
        "viDiliN",
        [
            "Ikzeta",
            "IkzeyAtAm",
            "Ikzeran",
            "IkzeTAH",
            "IkzeyATAm",
            "IkzeDvam",
            "Ikzeya",
            "Ikzevahi",
            "Ikzemahi",
        ],
    ),
];

fn lan_a_form(code: &str, pu: Purusha, va: Vacana) -> String {
    let d = dhatus().iter().find(|d| d.code == code).unwrap();
    derive(d, Lakara::Lan, Pada::Atmanepada, pu, va).text()
}

#[test]
fn labh_lan_atmanepada_all_nine_cells() {
    let expected = [
        (Purusha::Prathama, Vacana::Eka, "alaBata"),
        (Purusha::Prathama, Vacana::Dvi, "alaBetAm"),
        (Purusha::Prathama, Vacana::Bahu, "alaBanta"),
        (Purusha::Madhyama, Vacana::Eka, "alaBaTAH"),
        (Purusha::Madhyama, Vacana::Dvi, "alaBeTAm"),
        (Purusha::Madhyama, Vacana::Bahu, "alaBaDvam"),
        (Purusha::Uttama, Vacana::Eka, "alaBe"),
        (Purusha::Uttama, Vacana::Dvi, "alaBAvahi"),
        (Purusha::Uttama, Vacana::Bahu, "alaBAmahi"),
    ];
    for (pu, va, form) in expected {
        assert_eq!(lan_a_form("laB", pu, va), form, "{pu:?} {va:?}");
    }
}

#[test]
fn vowel_initial_roots_take_at_not_a() {
    // 6.4.72 āḍ ajādīnām (apavāda to 6.4.71) + 6.1.90 vṛddhi:
    // a+eD → ED (aidhata), a+Ikz → Ekz (aikṣata).
    assert_eq!(lan_a_form("eD", Purusha::Prathama, Vacana::Eka), "EData");
    assert_eq!(lan_a_form("Ikz", Purusha::Prathama, Vacana::Eka), "Ekzata");
}

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
        // Cross-lakāra atmanepada confusions.
        "alaBeta", // laN's augment on a vidhilin form
        "laBatam", // parasmaipada dual ending on an atmanepadin root
        "laBAte",  // 7.2.81 skipped: A must become iy after the shap
        "laBesva", // lot's sva on a lat stem (3.4.91 without 3.4.90's lakara)
        "IkzAmi",  // parasmaipada uttama ending on the vowel-initial A-root
    ] {
        assert!(
            matches!(engine.check(bad).verdict, Verdict::Invalid),
            "expected INVALID for {bad}"
        );
    }
}
