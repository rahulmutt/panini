#![forbid(unsafe_code)]
//! Transliteration between SLP1 (canonical internal form) and human schemes.
//!
//! `panini-lipi` is the only place transliteration happens: every other crate in
//! this workspace operates on SLP1 strings and calls into here at the boundary.

mod slp1;

/// A transliteration scheme. `Slp1` is the canonical internal representation;
/// the others are human-facing input/output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    Slp1,
    Iast,
    Hk,
    Devanagari,
}

/// Best-effort detection of the scheme a string is written in.
///
/// Heuristic (in order): any Devanagari codepoint -> `Devanagari`; any other
/// non-ASCII codepoint (IAST diacritics) -> `Iast`; ASCII containing HK-only
/// digraphs (`kh`, `bh`, `aa`) -> `Hk`; otherwise -> `Slp1`.
pub fn detect(input: &str) -> Scheme {
    if input
        .chars()
        .any(|c| ('\u{0900}'..='\u{097F}').contains(&c))
    {
        return Scheme::Devanagari;
    }
    // IAST is the only ASCII+diacritic scheme with combining/precomposed marks.
    if !input.is_ascii() {
        return Scheme::Iast;
    }
    // Heuristic: SLP1 uses capitals mid-word for aspirates/long vowels;
    // HK uses digraphs. Default ASCII to SLP1 unless HK-only digraphs appear.
    if input.contains("kh") || input.contains("bh") || input.contains("aa") {
        return Scheme::Hk;
    }
    Scheme::Slp1
}

/// Convert `input`, written in scheme `from`, to SLP1.
pub fn to_slp1(input: &str, from: Scheme) -> String {
    slp1::to_slp1(input, from)
}

/// Convert `slp1` (a string already in SLP1) to scheme `to`.
pub fn from_slp1(slp1: &str, to: Scheme) -> String {
    slp1::from_slp1(slp1, to)
}

/// Detect `input`'s scheme and convert it to SLP1, returning both.
pub fn normalize(input: &str) -> (String, Scheme) {
    let scheme = detect(input);
    (to_slp1(input, scheme), scheme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iast_to_slp1_roundtrips() {
        assert_eq!(to_slp1("bhavati", Scheme::Iast), "Bavati");
        assert_eq!(to_slp1("rāmeṇa", Scheme::Iast), "rAmeRa");
        assert_eq!(from_slp1("Bavati", Scheme::Iast), "bhavati");
    }

    #[test]
    fn detects_devanagari_and_converts() {
        let (slp1, scheme) = normalize("भवति");
        assert_eq!(scheme, Scheme::Devanagari);
        assert_eq!(slp1, "Bavati");
    }

    #[test]
    fn slp1_passthrough_detected() {
        let (slp1, scheme) = normalize("Bavati");
        assert_eq!(scheme, Scheme::Slp1);
        assert_eq!(slp1, "Bavati");
    }

    #[test]
    fn slp1_iast_roundtrip_curated() {
        for w in [
            "Bavati", "nayati", "jayati", "smarati", "paWati", "vadati", "BavAmaH",
        ] {
            let iast = from_slp1(w, Scheme::Iast);
            assert_eq!(to_slp1(&iast, Scheme::Iast), w, "roundtrip failed for {w}");
        }
    }

    /// Every SLP1 varṇa in the v1 inventory (14 vowels + 33 consonants + M + H
    /// = 49) must round-trip through each of IAST, HK, and Devanagari with no
    /// gaps in the mapping tables.
    #[test]
    fn every_varna_roundtrips_all_schemes() {
        let varnas: Vec<char> = "aAiIuUfFxXeEoOkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshMH"
            .chars()
            .collect();
        assert_eq!(varnas.len(), 49, "expected 49 varnas in inventory");
        for v in varnas {
            let slp1 = v.to_string();
            for scheme in [Scheme::Iast, Scheme::Hk, Scheme::Devanagari] {
                let other = from_slp1(&slp1, scheme);
                let back = to_slp1(&other, scheme);
                assert_eq!(
                    back, slp1,
                    "roundtrip failed for {slp1:?} via {scheme:?} (got {other:?} -> {back:?})"
                );
            }
        }
    }
}
