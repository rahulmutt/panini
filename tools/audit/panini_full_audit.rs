//! Cross-implementation audit of the `panini` engine against `vidyut-prakriya`,
//! keyed on the dhātupāṭha entry number.
//!
//! This is the harness for Task 7 Step 4 of the "dhātupāṭha number as identity"
//! slice. Its one methodological commitment is this: the upstream entry for a
//! curated root is selected by `Dhatupatha::get(d.dhatupatha)` and by NOTHING
//! else. Earlier audits searched upstream for whichever entry reproduced this
//! engine's own pinned laṭ prathama eka form, which made the anchoring cell the
//! one cell the audit could not independently validate. Keying on the number
//! removes that circularity.
//!
//! What it compares: for each of the 67 curated roots, for each pada the root
//! admits (two apiece for the twelve roots that admit both padas — eleven
//! ubhayapadī by 1.3.72, plus √bhuj by 1.3.66), for each of the four
//! lakāras this engine
//! implements (laṭ, laṅ, loṭ, vidhiliṅ), for each of the nine puruṣa × vacana
//! cells — the complete DERIVATION SET, as sorted SLP1 strings. Not a single
//! form: optional (vikalpa) rules fork cells legitimately, and comparing index 0
//! would raise a false difference on √hiṃs laṅ madhyama eka, where the sets
//! match but the two engines disagree about which branch is ruleless.
//!
//! Blocked prakriyās are filtered out on this engine's side: `Panini::derive`'s
//! doc comment states that a blocked prakriyā's `text()` is a partial string
//! (often the bare root code), not a surface form.
//!
//! Corpus invariants, asserted: 67 roots, 2844 cells, 3338 forms. These are
//! facts about the repo, pinned by its own golden suite
//! (`derivation_set_shape_matches_the_audited_numbers`): 316 root×pada×lakāra
//! blocks × 9 cells, plus 494 `ALTERNATES` rows. If this harness's
//! enumeration disagrees, the harness is wrong.
//!
//! Which dhātupāṭha file: the vidyut checkout's own
//! `vidyut-prakriya/data/dhatupatha.tsv`, the pristine upstream copy at the
//! audited commit. This repo's vendored `data/dhatupatha.tsv` is that same file
//! with a 20-line `#`-prefixed provenance header prepended, and vidyut's loader
//! does not skip `#` comments (it skips only line 0 and empty lines), so it
//! cannot read the vendored copy. The harness asserts the two are byte-identical
//! modulo those `#` lines rather than assuming it.
//!
//! Setup and usage are in this repo's `tools/audit/README.md`. In brief:
//!
//!     cargo run --release --example panini_full_audit
//!
//! Both checkout locations are env-overridable, defaulting to `/tmp/vidyut-full`
//! and `/workspace`:
//!
//!     PANINI_AUDIT_VIDYUT=/path/to/vidyut  PANINI_AUDIT_REPO=/path/to/panini \
//!       cargo run --release --example panini_full_audit
//!
//! Negative controls (see `Perturb`), used to prove the harness can detect a
//! difference rather than merely printing zero:
//!
//!     PANINI_AUDIT_PERTURB=form  cargo run --release --example panini_full_audit
//!     PANINI_AUDIT_PERTURB=entry cargo run --release --example panini_full_audit
//!
//! Optionally dump the full 2844-cell table:
//!
//!     PANINI_AUDIT_DUMP=/path/to/table.tsv cargo run --release --example panini_full_audit

use std::collections::BTreeSet;
use std::fmt::Write as _;

use panini::Panini;
use panini_data::{
    Gana as PGana, Lakara as PLakara, Pada as PPada, Purusha as PPurusha, Vacana as PVacana, dhatus,
};
use vidyut_prakriya::args::{DhatuPada, Lakara, Prayoga, Purusha, Tinanta, Vacana};
use vidyut_prakriya::{Dhatupatha, Vyakarana};

/// Root of the `vidyut` checkout this example is being run from, holding the
/// pristine upstream dhātupāṭha at the audited commit. Override with
/// `PANINI_AUDIT_VIDYUT` when the checkout is not at the documented location.
fn vidyut_root() -> String {
    std::env::var("PANINI_AUDIT_VIDYUT").unwrap_or_else(|_| "/tmp/vidyut-full".to_string())
}

/// Root of the `panini` repo whose engine is under audit, holding the vendored
/// `data/dhatupatha.tsv` this checks for byte-identity against upstream.
/// Override with `PANINI_AUDIT_REPO`.
///
/// Deliberately not defaulted to a path inside a worktree: worktrees are
/// transient, and a stale absolute path is how this harness would rot between
/// slices. Point it at whichever checkout you added to `Cargo.toml`.
fn panini_root() -> String {
    std::env::var("PANINI_AUDIT_REPO").unwrap_or_else(|_| "/workspace".to_string())
}

/// The pristine upstream dhātupāṭha at the audited commit.
fn upstream_tsv() -> String {
    format!("{}/vidyut-prakriya/data/dhatupatha.tsv", vidyut_root())
}

/// The repo's vendored copy, checked here for byte-identity modulo its header.
fn vendored_tsv() -> String {
    format!("{}/data/dhatupatha.tsv", panini_root())
}

/// Read a path, naming the env var that redirects it when it is missing — a
/// stale path should say how to fix itself, not just fail to open.
fn read_or_explain(path: &str, env_var: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {path}: {e}\nset {env_var} to override"))
}

/// Deliberate corruptions, for proving the comparison has teeth.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Perturb {
    /// Audit honestly.
    None,
    /// Corrupt ONE form on this engine's side (√bhū laṭ prathama eka:
    /// `Bavati` -> `BavatiXX`). Set sizes are preserved, so the corpus totals
    /// still hold and the only thing that can fire is the set comparison.
    Form,
    /// Resolve ONE root against the WRONG upstream entry: √bhū (`01.0001`)
    /// looked up as `01.0381` (√paṭh). This is the failure the slice exists to
    /// rule out — a number that names the wrong upstream row — so it is the
    /// negative control that matters most. `01.0381` is chosen deliberately as
    /// a PLAUSIBLE wrong answer: same gaṇa, same pada, fully derivable in all
    /// 36 cells, so the mismatch shows up as real form-vs-form disagreement
    /// (`Bavati` vs `paWati`) rather than as an empty upstream set.
    Entry,
}

impl Perturb {
    fn from_env() -> Self {
        match std::env::var("PANINI_AUDIT_PERTURB").as_deref() {
            Ok("form") => Perturb::Form,
            Ok("entry") => Perturb::Entry,
            Ok("") | Ok("none") | Err(_) => Perturb::None,
            Ok(other) => panic!("unknown PANINI_AUDIT_PERTURB={other:?}"),
        }
    }
}

/// The four lakāras this engine implements, with their SLP1 display names.
const LAKARAS: [(PLakara, Lakara, &str); 4] = [
    (PLakara::Lat, Lakara::Lat, "laT"),
    (PLakara::Lan, Lakara::Lan, "laN"),
    (PLakara::Lot, Lakara::Lot, "loT"),
    (PLakara::VidhiLin, Lakara::VidhiLin, "viDiliN"),
];

/// The nine puruṣa × vacana cells, in the order the repo's goldens use.
const CELLS: [(PPurusha, Purusha, PVacana, Vacana, &str); 9] = [
    (
        PPurusha::Prathama,
        Purusha::Prathama,
        PVacana::Eka,
        Vacana::Eka,
        "prathama.eka",
    ),
    (
        PPurusha::Prathama,
        Purusha::Prathama,
        PVacana::Dvi,
        Vacana::Dvi,
        "prathama.dvi",
    ),
    (
        PPurusha::Prathama,
        Purusha::Prathama,
        PVacana::Bahu,
        Vacana::Bahu,
        "prathama.bahu",
    ),
    (
        PPurusha::Madhyama,
        Purusha::Madhyama,
        PVacana::Eka,
        Vacana::Eka,
        "madhyama.eka",
    ),
    (
        PPurusha::Madhyama,
        Purusha::Madhyama,
        PVacana::Dvi,
        Vacana::Dvi,
        "madhyama.dvi",
    ),
    (
        PPurusha::Madhyama,
        Purusha::Madhyama,
        PVacana::Bahu,
        Vacana::Bahu,
        "madhyama.bahu",
    ),
    (
        PPurusha::Uttama,
        Purusha::Uttama,
        PVacana::Eka,
        Vacana::Eka,
        "uttama.eka",
    ),
    (
        PPurusha::Uttama,
        Purusha::Uttama,
        PVacana::Dvi,
        Vacana::Dvi,
        "uttama.dvi",
    ),
    (
        PPurusha::Uttama,
        Purusha::Uttama,
        PVacana::Bahu,
        Vacana::Bahu,
        "uttama.bahu",
    ),
];

fn pada_name(p: PPada) -> &'static str {
    match p {
        PPada::Parasmaipada => "parasmEpadam",
        PPada::Atmanepada => "Atmanepadam",
    }
}

fn to_vidyut_pada(p: PPada) -> DhatuPada {
    match p {
        PPada::Parasmaipada => DhatuPada::Parasmaipada,
        PPada::Atmanepada => DhatuPada::Atmanepada,
    }
}

/// The gaṇa the two-digit prefix of a dhātupāṭha number names. Used only to
/// print the resolution table; the engine's own suite asserts the invariant.
fn gana_of_number(number: &str) -> &'static str {
    match &number[..2] {
        "01" => "Bhvadi",
        "02" => "Adadi",
        "03" => "Juhotyadi",
        "04" => "Divadi",
        "05" => "Svadi",
        "06" => "Tudadi",
        "07" => "Rudhadi",
        "08" => "Tanadi",
        "09" => "Kryadi",
        "10" => "Curadi",
        _ => "?",
    }
}

fn gana_name(g: PGana) -> &'static str {
    match g {
        PGana::Bhvadi => "Bhvadi",
        PGana::Adadi => "Adadi",
        PGana::Divadi => "Divadi",
        PGana::Svadi => "Svadi",
        PGana::Tudadi => "Tudadi",
        PGana::Rudhadi => "Rudhadi",
        PGana::Kryadi => "Kryadi",
    }
}

/// Confirms the vendored TSV is upstream's file plus a `#`-comment header, so
/// that feeding vidyut the upstream copy is provably feeding it the same data
/// the engine's numbers were resolved against.
fn check_vendored_matches_upstream() {
    let upstream = read_or_explain(&upstream_tsv(), "PANINI_AUDIT_VIDYUT");
    let vendored = read_or_explain(&vendored_tsv(), "PANINI_AUDIT_REPO");
    let header_lines = vendored.lines().take_while(|l| l.starts_with('#')).count();
    let stripped: String = vendored
        .lines()
        .skip(header_lines)
        .map(|l| format!("{l}\n"))
        .collect();
    assert_eq!(
        header_lines, 20,
        "vendored dhatupatha.tsv should carry a 20-line '#' provenance header"
    );
    assert_eq!(
        stripped, upstream,
        "vendored dhatupatha.tsv, minus its '#' header, must be byte-identical to upstream's"
    );
    println!(
        "dhatupatha.tsv: vendored copy == upstream copy after dropping {header_lines} '#' header \
         lines ({} data bytes). Loading the upstream copy, because vidyut's loader does not skip \
         '#' comments.",
        upstream.len()
    );
}

/// Demonstrates that the `blocked` filter is a real filter and not decoration.
/// No cell IN the corpus is blocked — every (root, pada) pair the corpus
/// enumerates comes from `d.pada.padas()`, i.e. a pada the root is sanctioned
/// in — so this probe steps deliberately OUTSIDE the corpus and asks √bhū
/// (parasmaipada-only) for an ātmanepada form, which 1.3.12/1.3.78 must block.
/// It prints the partial, not-a-surface-form text that would have polluted the
/// comparison had the filter been omitted.
fn probe_blocked_filter(panini: &Panini) {
    let bhu = dhatus()
        .iter()
        .find(|d| d.dhatupatha == "01.0001")
        .expect("01.0001");
    let branches = panini.derive(
        bhu,
        PLakara::Lat,
        PPada::Atmanepada,
        PPurusha::Prathama,
        PVacana::Eka,
    );
    println!("\n=== blocked-prakriyā probe (outside the corpus) ===");
    println!(
        "01.0001 (BU) Atmanepadam laT prathama.eka -> {} branch(es)",
        branches.len()
    );
    for p in &branches {
        println!("  blocked={} text={:?}", p.blocked, p.text());
    }
    let live = branches.iter().filter(|p| !p.blocked).count();
    println!("  live (non-blocked) branches: {live}");
    assert!(
        branches.iter().any(|p| p.blocked),
        "the blocked filter must actually have something to filter"
    );
}

/// Shows, concretely, why this harness compares SETS. √hiṃs (`07.0019`) laṅ
/// madhyama eka is the documented case (recorded by the rudhādi 7a slice) where
/// both engines derive the same three forms but disagree about which branch is
/// the ruleless one. An index-0 comparison would report a false difference here;
/// a set comparison passes.
fn probe_index_zero_divergence(panini: &Panini, up: &vidyut_prakriya::args::Dhatu, v: &Vyakarana) {
    let his = dhatus()
        .iter()
        .find(|d| d.dhatupatha == "07.0019")
        .expect("07.0019");
    let ours: Vec<String> = panini
        .derive(
            his,
            PLakara::Lan,
            PPada::Parasmaipada,
            PPurusha::Madhyama,
            PVacana::Eka,
        )
        .iter()
        .filter(|p| !p.blocked)
        .map(|p| p.text())
        .collect();
    let t = Tinanta::builder()
        .dhatu(up.clone())
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lan)
        .pada(DhatuPada::Parasmaipada)
        .build()
        .expect("build tinanta args");
    let theirs: Vec<String> = v.derive_tinantas(&t).iter().map(|p| p.text()).collect();
    println!("\n=== index-0 divergence probe (07.0019 hins laN madhyama.eka) ===");
    println!("panini branch order: {ours:?}  (index 0 = {:?})", ours[0]);
    println!("vidyut branch order: {theirs:?}  (index 0 = {:?})", theirs[0]);
    let mut a = ours.clone();
    let mut b = theirs.clone();
    a.sort();
    a.dedup();
    b.sort();
    b.dedup();
    println!(
        "as sets: {}  |  by index 0: {}",
        if a == b { "EQUAL" } else { "DIFFERENT" },
        if ours[0] == theirs[0] {
            "equal"
        } else {
            "DIFFERENT (this is why the audit compares sets)"
        }
    );
}

struct Diff {
    number: &'static str,
    code: &'static str,
    pada: &'static str,
    lakara: &'static str,
    cell: &'static str,
    ours: Vec<String>,
    theirs: Vec<String>,
}

fn main() {
    let perturb = Perturb::from_env();
    match perturb {
        Perturb::None => (),
        Perturb::Form => println!(
            "*** PANINI_AUDIT_PERTURB=form: NEGATIVE CONTROL. One form on the engine's side is \
             deliberately corrupted. A difference here proves the comparison has teeth. ***"
        ),
        Perturb::Entry => println!(
            "*** PANINI_AUDIT_PERTURB=entry: NEGATIVE CONTROL. Root 01.0001 is resolved against \
             upstream entry 01.0381. A difference here proves a wrong number would be caught. ***"
        ),
    }

    check_vendored_matches_upstream();

    let dp = Dhatupatha::from_path(upstream_tsv()).expect("load upstream dhatupatha");
    let v = Vyakarana::builder().log_steps(false).build();
    let panini = Panini::new();
    probe_blocked_filter(&panini);

    // --- Entry resolution, by number and by nothing else. -------------------
    let roots = dhatus();
    let mut resolution = String::new();
    writeln!(
        resolution,
        "number\tgana(number)\tengine.code\tengine.gana\tengine.pada\tengine.artha\t\
         upstream.aupadeshika\tupstream.gana\tupstream.prefixes"
    )
    .unwrap();
    let mut resolved: Vec<(&'static str, vidyut_prakriya::args::Dhatu)> = Vec::new();
    for d in roots {
        let key = if perturb == Perturb::Entry && d.dhatupatha == "01.0001" {
            "01.0381"
        } else {
            d.dhatupatha
        };
        let up = dp
            .get(key)
            .unwrap_or_else(|| panic!("no upstream entry for {key}"))
            .clone();
        writeln!(
            resolution,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{:?}",
            d.dhatupatha,
            gana_of_number(d.dhatupatha),
            d.code,
            gana_name(d.gana),
            d.pada
                .padas()
                .iter()
                .map(|p| pada_name(*p))
                .collect::<Vec<_>>()
                .join("+"),
            d.artha,
            up.aupadeshika().expect("aupadeshika"),
            up.gana().expect("gana"),
            up.prefixes(),
        )
        .unwrap();
        resolved.push((d.dhatupatha, up));
    }
    print!("\n=== entry resolution (Dhatupatha::get(d.dhatupatha)) ===\n{resolution}");

    if perturb == Perturb::None {
        probe_index_zero_divergence(
            &panini,
            &resolved
                .iter()
                .find(|(n, _)| *n == "07.0019")
                .expect("07.0019")
                .1,
            &v,
        );
    }

    // --- The corpus. --------------------------------------------------------
    let mut n_cells = 0usize;
    let mut n_forms = 0usize;
    let mut n_branches = 0usize;
    let mut n_blocked = 0usize;
    let mut roots_seen: BTreeSet<&str> = BTreeSet::new();
    let mut diffs: Vec<Diff> = Vec::new();
    let mut dump = String::new();
    let dump_path = std::env::var("PANINI_AUDIT_DUMP").ok();
    if dump_path.is_some() {
        writeln!(dump, "number\tcode\tpada\tlakara\tcell\tours\ttheirs\tsame").unwrap();
    }

    for (d, (number, up)) in roots.iter().zip(resolved.iter()) {
        assert_eq!(d.dhatupatha, *number);
        roots_seen.insert(d.dhatupatha);
        for pada in d.pada.padas() {
            for (p_lak, v_lak, lak_name) in LAKARAS {
                for (p_pu, v_pu, p_va, v_va, cell_name) in CELLS {
                    n_cells += 1;

                    // --- this engine ---
                    let branches = panini.derive(d, p_lak, *pada, p_pu, p_va);
                    // A blocked prakriyā's text() is a partial string, not a
                    // surface form. Filter before comparing anything.
                    n_blocked += branches.iter().filter(|p| p.blocked).count();
                    let live: Vec<String> = branches
                        .iter()
                        .filter(|p| !p.blocked)
                        .map(|p| p.text())
                        .collect();
                    n_branches += live.len();
                    let mut ours: Vec<String> = live.clone();
                    ours.sort();
                    ours.dedup();
                    if perturb == Perturb::Form
                        && d.dhatupatha == "01.0001"
                        && lak_name == "laT"
                        && cell_name == "prathama.eka"
                    {
                        ours = ours.iter().map(|s| format!("{s}XX")).collect();
                    }
                    n_forms += ours.len();

                    // --- vidyut-prakriya ---
                    let t = Tinanta::builder()
                        .dhatu(up.clone())
                        .prayoga(Prayoga::Kartari)
                        .purusha(v_pu)
                        .vacana(v_va)
                        .lakara(v_lak)
                        .pada(to_vidyut_pada(*pada))
                        .build()
                        .expect("build tinanta args");
                    let mut theirs: Vec<String> =
                        v.derive_tinantas(&t).iter().map(|p| p.text()).collect();
                    theirs.sort();
                    theirs.dedup();

                    let same = ours == theirs;
                    if dump_path.is_some() {
                        writeln!(
                            dump,
                            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                            d.dhatupatha,
                            d.code,
                            pada_name(*pada),
                            lak_name,
                            cell_name,
                            ours.join("|"),
                            theirs.join("|"),
                            if same { "=" } else { "DIFF" },
                        )
                        .unwrap();
                    }
                    if !same {
                        diffs.push(Diff {
                            number: d.dhatupatha,
                            code: d.code,
                            pada: pada_name(*pada),
                            lakara: lak_name,
                            cell: cell_name,
                            ours,
                            theirs,
                        });
                    }
                }
            }
        }
    }

    if let Some(path) = &dump_path {
        std::fs::write(path, &dump).expect("write dump");
        println!("\nfull cell table written to {path}");
    }

    // --- Differences first, so a totals assertion cannot hide them. ---------
    println!("\n=== differences ===");
    if diffs.is_empty() {
        println!("none");
    } else {
        for x in &diffs {
            println!(
                "DIFF {} ({}) {} {} {}\n  panini : {}\n  vidyut : {}",
                x.number,
                x.code,
                x.pada,
                x.lakara,
                x.cell,
                x.ours.join(" "),
                x.theirs.join(" "),
            );
        }
    }

    // --- Corpus totals. -----------------------------------------------------
    println!("\n=== corpus ===");
    println!("roots            : {}", roots_seen.len());
    println!("cells            : {n_cells}");
    println!("forms (set sizes): {n_forms}");
    println!("live branches    : {n_branches}");
    println!("blocked branches : {n_blocked}");
    println!("differing cells  : {}", diffs.len());

    assert_eq!(roots_seen.len(), 67, "curated roots");
    assert_eq!(n_cells, 2844, "cells: 316 root×pada×lakāra blocks × 9");
    assert_eq!(n_forms, 3338, "forms: 2844 cells + 494 ALTERNATES rows");
    assert_eq!(
        n_branches, n_forms,
        "no cell may yield two live branches with the same text"
    );

    if diffs.is_empty() {
        println!("\nAUDIT PASSED: {n_cells} cells, {n_forms} forms, zero differences.");
    } else {
        println!("\nAUDIT FAILED: {} differing cells.", diffs.len());
        std::process::exit(1);
    }
}
