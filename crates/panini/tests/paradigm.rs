use panini::{Panini, Verdict};

/// (root_code, [P.E, P.D, P.B, M.E, M.D, M.B, U.E, U.D, U.B]) in SLP1.
const PARADIGM: &[(&str, [&str; 9])] = &[
    (
        "BU",
        [
            "Bavati", "BavataH", "Bavanti", "Bavasi", "BavaTaH", "BavaTa", "BavAmi", "BavAvaH",
            "BavAmaH",
        ],
    ),
    (
        "nI",
        [
            "nayati", "nayataH", "nayanti", "nayasi", "nayaTaH", "nayaTa", "nayAmi", "nayAvaH",
            "nayAmaH",
        ],
    ),
    (
        "ji",
        [
            "jayati", "jayataH", "jayanti", "jayasi", "jayaTaH", "jayaTa", "jayAmi", "jayAvaH",
            "jayAmaH",
        ],
    ),
    (
        "smf",
        [
            "smarati", "smarataH", "smaranti", "smarasi", "smaraTaH", "smaraTa", "smarAmi",
            "smarAvaH", "smarAmaH",
        ],
    ),
    (
        "paW",
        [
            "paWati", "paWataH", "paWanti", "paWasi", "paWaTaH", "paWaTa", "paWAmi", "paWAvaH",
            "paWAmaH",
        ],
    ),
    (
        "vad",
        [
            "vadati", "vadataH", "vadanti", "vadasi", "vadaTaH", "vadaTa", "vadAmi", "vadAvaH",
            "vadAmaH",
        ],
    ),
];

#[test]
fn every_form_validates_and_matches() {
    let engine = Panini::new();
    for (_root, forms) in PARADIGM {
        for expected in forms {
            let r = engine.check(expected);
            assert!(
                matches!(r.verdict, Verdict::Valid),
                "expected VALID for {expected}"
            );
            assert!(
                r.analyses.iter().any(|a| a.form_slp1 == *expected),
                "no analysis produced {expected}"
            );
        }
    }
}

#[test]
fn known_nonforms_are_invalid() {
    let engine = Panini::new();
    for bad in ["Bavatu", "Bavati123", "gacCati", "tiRRati"] {
        // Bavatu (loT) and gacCati (irregular gam) are out of v1 scope -> Invalid.
        assert!(
            matches!(engine.check(bad).verdict, Verdict::Invalid),
            "expected INVALID for {bad}"
        );
    }
}
