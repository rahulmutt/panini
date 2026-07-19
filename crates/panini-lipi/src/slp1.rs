//! SLP1 canonical mapping tables and conversions to/from IAST, HK (Harvard-Kyoto),
//! and Devanagari.
//!
//! SLP1 is designed so every phoneme (varṇa) is exactly one ASCII character, which
//! makes SLP1 input trivial to tokenize (iterate `chars()`). Parsing the *other*
//! schemes back into SLP1 requires longest-match tokenization since IAST/HK use
//! multi-character digraphs (e.g. `bh`, `kh`) and Devanagari uses a base-glyph +
//! virama/mātrā system rather than one-codepoint-per-phoneme.

use crate::Scheme;

/// (slp1_token, iast_token) pairs for the full v1 varṇa inventory: short vowels
/// `a i u f x`, long `A I U F X`, diphthongs `e E o O`, all 33 consonants,
/// anusvāra `M`, visarga `H`. 49 entries total — no gaps.
pub const IAST: &[(&str, &str)] = &[
    ("A", "\u{101}"),
    ("I", "\u{12b}"),
    ("U", "\u{16b}"),
    ("f", "\u{1e5b}"),
    ("F", "\u{1e5d}"),
    ("x", "\u{1e37}"),
    ("X", "\u{1e39}"),
    ("e", "e"),
    ("E", "ai"),
    ("o", "o"),
    ("O", "au"),
    ("a", "a"),
    ("i", "i"),
    ("u", "u"),
    ("K", "kh"),
    ("G", "gh"),
    ("N", "\u{1e45}"),
    ("C", "ch"),
    ("J", "jh"),
    ("Y", "\u{f1}"),
    ("w", "\u{1e6d}"),
    ("W", "\u{1e6d}h"),
    ("q", "\u{1e0d}"),
    ("Q", "\u{1e0d}h"),
    ("R", "\u{1e47}"),
    ("T", "th"),
    ("D", "dh"),
    ("P", "ph"),
    ("B", "bh"),
    ("S", "\u{15b}"),
    ("z", "\u{1e63}"),
    ("k", "k"),
    ("g", "g"),
    ("c", "c"),
    ("j", "j"),
    ("t", "t"),
    ("d", "d"),
    ("n", "n"),
    ("p", "p"),
    ("b", "b"),
    ("m", "m"),
    ("y", "y"),
    ("r", "r"),
    ("l", "l"),
    ("v", "v"),
    ("s", "s"),
    ("h", "h"),
    ("M", "\u{1e43}"),
    ("H", "\u{1e25}"),
];

/// (slp1_token, hk_token) pairs, Harvard-Kyoto convention: retroflex consonants
/// are capital dental letters (`T Th D Dh N`), velar/palatal nasals are `G`/`J`,
/// palatals `ś ṣ` are `z`/`S`. 49 entries — no gaps.
pub const HK: &[(&str, &str)] = &[
    ("A", "A"),
    ("I", "I"),
    ("U", "U"),
    ("f", "R"),
    ("F", "RR"),
    ("x", "lR"),
    ("X", "lRR"),
    ("e", "e"),
    ("E", "ai"),
    ("o", "o"),
    ("O", "au"),
    ("a", "a"),
    ("i", "i"),
    ("u", "u"),
    ("K", "kh"),
    ("G", "gh"),
    ("N", "G"),
    ("C", "ch"),
    ("J", "jh"),
    ("Y", "J"),
    ("w", "T"),
    ("W", "Th"),
    ("q", "D"),
    ("Q", "Dh"),
    ("R", "N"),
    ("T", "th"),
    ("D", "dh"),
    ("P", "ph"),
    ("B", "bh"),
    ("S", "z"),
    ("z", "S"),
    ("k", "k"),
    ("g", "g"),
    ("c", "c"),
    ("j", "j"),
    ("t", "t"),
    ("d", "d"),
    ("n", "n"),
    ("p", "p"),
    ("b", "b"),
    ("m", "m"),
    ("y", "y"),
    ("r", "r"),
    ("l", "l"),
    ("v", "v"),
    ("s", "s"),
    ("h", "h"),
    ("M", "M"),
    ("H", "H"),
];

/// Parse-only HK digraph aliases for long vowels (aa→A, ii→I, uu→U).
/// These are consulted during HK → SLP1 parsing only, not during SLP1 → HK emission.
/// This allows the `detect()` heuristic (which flags any ASCII containing "aa" as HK)
/// to work correctly: strings like "kanyaa" are detected as HK and then parsed with
/// these aliases to produce the correct SLP1 "kanyA" instead of two separate "a" chars.
/// Format: (slp1_token, hk_token) to match the HK table format.
const HK_PARSE_ALIASES: &[(&str, &str)] = &[("A", "aa"), ("I", "ii"), ("U", "uu")];

/// (slp1_token, independent_vowel_glyph, dependent_matra_sign) triples for the
/// 14 SLP1 vowels. `a`'s matra is empty (bare consonant carries inherent `a`).
pub const DEVANAGARI_VOWELS: &[(&str, &str, &str)] = &[
    ("a", "\u{905}", ""),
    ("A", "\u{906}", "\u{93e}"),
    ("i", "\u{907}", "\u{93f}"),
    ("I", "\u{908}", "\u{940}"),
    ("u", "\u{909}", "\u{941}"),
    ("U", "\u{90a}", "\u{942}"),
    ("f", "\u{90b}", "\u{943}"),
    ("F", "\u{960}", "\u{944}"),
    ("x", "\u{90c}", "\u{962}"),
    ("X", "\u{961}", "\u{963}"),
    ("e", "\u{90f}", "\u{947}"),
    ("E", "\u{910}", "\u{948}"),
    ("o", "\u{913}", "\u{94b}"),
    ("O", "\u{914}", "\u{94c}"),
];

/// (slp1_token, base_glyph) pairs for the 33 SLP1 consonants (base glyph carries
/// an inherent `a` unless followed by virāma or a mātrā).
pub const DEVANAGARI_CONSONANTS: &[(&str, &str)] = &[
    ("k", "\u{915}"),
    ("K", "\u{916}"),
    ("g", "\u{917}"),
    ("G", "\u{918}"),
    ("N", "\u{919}"),
    ("c", "\u{91a}"),
    ("C", "\u{91b}"),
    ("j", "\u{91c}"),
    ("J", "\u{91d}"),
    ("Y", "\u{91e}"),
    ("w", "\u{91f}"),
    ("W", "\u{920}"),
    ("q", "\u{921}"),
    ("Q", "\u{922}"),
    ("R", "\u{923}"),
    ("t", "\u{924}"),
    ("T", "\u{925}"),
    ("d", "\u{926}"),
    ("D", "\u{927}"),
    ("n", "\u{928}"),
    ("p", "\u{92a}"),
    ("P", "\u{92b}"),
    ("b", "\u{92c}"),
    ("B", "\u{92d}"),
    ("m", "\u{92e}"),
    ("y", "\u{92f}"),
    ("r", "\u{930}"),
    ("l", "\u{932}"),
    ("v", "\u{935}"),
    ("S", "\u{936}"),
    ("z", "\u{937}"),
    ("s", "\u{938}"),
    ("h", "\u{939}"),
];

/// (slp1_token, glyph) pairs for anusvāra `M` and visarga `H`.
pub const DEVANAGARI_OTHER: &[(&str, &str)] = &[("M", "\u{902}"), ("H", "\u{903}")];

/// Devanagari virāma (halant) sign, appended after a consonant with no following vowel.
pub const VIRAMA: &str = "\u{94d}";

pub fn to_slp1(input: &str, from: Scheme) -> String {
    match from {
        Scheme::Slp1 => input.to_string(),
        Scheme::Iast => parse_simple(input, IAST),
        Scheme::Hk => parse_hk_with_aliases(input),
        Scheme::Devanagari => devanagari_to_slp1(input),
    }
}

pub fn from_slp1(input: &str, to: Scheme) -> String {
    match to {
        Scheme::Slp1 => input.to_string(),
        Scheme::Iast => emit_simple(input, IAST),
        Scheme::Hk => emit_simple(input, HK),
        Scheme::Devanagari => slp1_to_devanagari(input),
    }
}

/// Emit each SLP1 character (already one-token-per-char) as its target-scheme
/// string via direct table lookup. Unknown characters (whitespace, punctuation,
/// digits) pass through unchanged.
fn emit_simple(input: &str, table: &[(&str, &str)]) -> String {
    let mut out = String::new();
    for c in input.chars() {
        let mut buf = [0u8; 4];
        let s: &str = c.encode_utf8(&mut buf);
        match table.iter().find(|&&(slp1_tok, _)| slp1_tok == s) {
            Some(&(_, other_tok)) => out.push_str(other_tok),
            None => out.push(c),
        }
    }
    out
}

/// Parse a string in some other scheme back into SLP1 using longest-match
/// tokenization: at each position try the longest known "other scheme" token
/// first, falling back to passthrough of a single char if nothing matches.
fn parse_simple(input: &str, table: &[(&str, &str)]) -> String {
    let mut sorted: Vec<(&str, &str)> = table.iter().map(|&(slp1, other)| (other, slp1)).collect();
    sorted.sort_by(|a, b| b.0.chars().count().cmp(&a.0.chars().count()));

    let mut out = String::new();
    let mut rest = input;
    'outer: while !rest.is_empty() {
        for &(other_tok, slp1_tok) in &sorted {
            if rest.starts_with(other_tok) {
                out.push_str(slp1_tok);
                rest = &rest[other_tok.len()..];
                continue 'outer;
            }
        }
        // No known token matches here: passthrough one char unchanged.
        let mut chars = rest.chars();
        let c = chars.next().expect("rest is non-empty");
        out.push(c);
        rest = chars.as_str();
    }
    out
}

/// Parse HK input with long-vowel digraph aliases (aa→A, ii→I, uu→U).
/// This is needed because the `detect()` heuristic flags any ASCII containing "aa"
/// as HK, but the standard HK table only has single-char tokens ("A", "I", "U").
/// By including the digraph aliases in the parse phase, we ensure that strings
/// like "kanyaa" parse correctly to "kanyA" instead of mis-tokenizing as two "a"s.
fn parse_hk_with_aliases(input: &str) -> String {
    let mut sorted: Vec<(&str, &str)> = HK
        .iter()
        .chain(HK_PARSE_ALIASES.iter())
        .map(|&(slp1, other)| (other, slp1))
        .collect();
    sorted.sort_by(|a, b| b.0.chars().count().cmp(&a.0.chars().count()));
    sorted.dedup_by(|a, b| a.0 == b.0); // Aliases take precedence (they're later in the chain)

    let mut out = String::new();
    let mut rest = input;
    'outer: while !rest.is_empty() {
        for &(other_tok, slp1_tok) in &sorted {
            if rest.starts_with(other_tok) {
                out.push_str(slp1_tok);
                rest = &rest[other_tok.len()..];
                continue 'outer;
            }
        }
        // No known token matches here: passthrough one char unchanged.
        let mut chars = rest.chars();
        let c = chars.next().expect("rest is non-empty");
        out.push(c);
        rest = chars.as_str();
    }
    out
}

/// Flush a pending bare consonant (base glyph + virāma) when it is not followed
/// by a vowel — i.e. it is followed by another consonant, a non-`a` marker, or
/// the end of the word.
fn flush_bare(out: &mut String, pending: &mut Option<&'static str>) {
    if let Some(base) = pending.take() {
        out.push_str(base);
        out.push_str(VIRAMA);
    }
}

/// SLP1 -> Devanagari, handling the inherent-`a` / virāma / mātrā logic:
/// - bare consonant (no following vowel): base glyph + virāma (`B` -> `भ्`)
/// - consonant + `a`: base glyph alone (`Ba` -> `भ`)
/// - consonant + other vowel: base glyph + dependent mātrā (`Bi` -> `भि`)
/// - vowel with no preceding consonant (word-initial or post-vowel): independent
///   vowel glyph (`a` -> `अ`)
fn slp1_to_devanagari(input: &str) -> String {
    let mut out = String::new();
    let mut pending: Option<&'static str> = None;

    for c in input.chars() {
        let mut buf = [0u8; 4];
        let s: &str = c.encode_utf8(&mut buf);

        if let Some(&(_, base)) = DEVANAGARI_CONSONANTS.iter().find(|&&(slp1, _)| slp1 == s) {
            flush_bare(&mut out, &mut pending);
            pending = Some(base);
            continue;
        }

        if let Some(&(_, indep, matra)) = DEVANAGARI_VOWELS.iter().find(|&&(slp1, _, _)| slp1 == s)
        {
            match pending.take() {
                Some(base) => {
                    out.push_str(base);
                    out.push_str(matra); // empty for inherent `a`
                }
                None => out.push_str(indep),
            }
            continue;
        }

        if let Some(&(_, glyph)) = DEVANAGARI_OTHER.iter().find(|&&(slp1, _)| slp1 == s) {
            flush_bare(&mut out, &mut pending);
            out.push_str(glyph);
            continue;
        }

        // Unknown character (whitespace, punctuation, digits): flush then passthrough.
        flush_bare(&mut out, &mut pending);
        out.push(c);
    }
    flush_bare(&mut out, &mut pending);
    out
}

/// Devanagari -> SLP1, inverse of [`slp1_to_devanagari`]: a consonant glyph is
/// followed by either a virāma (bare consonant, no vowel emitted), a dependent
/// mātrā (emit that vowel), or nothing/another glyph (inherent `a`).
fn devanagari_to_slp1(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let mut buf = [0u8; 4];
        let s: &str = c.encode_utf8(&mut buf);

        if let Some(&(slp1_tok, _)) = DEVANAGARI_CONSONANTS.iter().find(|&&(_, base)| base == s) {
            out.push_str(slp1_tok);
            i += 1;

            if i < chars.len() {
                let next = chars[i];
                let mut nbuf = [0u8; 4];
                let ns: &str = next.encode_utf8(&mut nbuf);

                if ns == VIRAMA {
                    i += 1; // bare consonant, no vowel
                    continue;
                }
                if let Some(&(vowel_slp1, _, _)) = DEVANAGARI_VOWELS
                    .iter()
                    .find(|&&(_, _, matra)| !matra.is_empty() && matra == ns)
                {
                    out.push_str(vowel_slp1);
                    i += 1;
                    continue;
                }
            }
            out.push('a'); // inherent vowel, nothing consumed
            continue;
        }

        if let Some(&(vowel_slp1, _, _)) =
            DEVANAGARI_VOWELS.iter().find(|&&(_, indep, _)| indep == s)
        {
            out.push_str(vowel_slp1);
            i += 1;
            continue;
        }

        if let Some(&(slp1_tok, _)) = DEVANAGARI_OTHER.iter().find(|&&(_, glyph)| glyph == s) {
            out.push_str(slp1_tok);
            i += 1;
            continue;
        }

        // Unknown character (whitespace, punctuation, digits): passthrough.
        out.push(c);
        i += 1;
    }
    out
}
