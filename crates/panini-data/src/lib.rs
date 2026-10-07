#![forbid(unsafe_code)]

use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    Bhvadi,
    Divadi,
    Tudadi,
    Adadi,
    Kryadi,
    Svadi,
    Rudhadi,
    Tanadi,
    Juhotyadi,
    Curadi,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pada {
    Parasmaipada,
    Atmanepada,
}
/// What a root *admits*, as distinct from `Context.pada`, which says what is
/// *being derived*. `Context.pada` stays the two-valued `Pada` on purpose:
/// no derivation may request an "ubhayapada" cell, because no such cell
/// exists — a root sanctioned in both padas is derived as two ordinary
/// single-pada cells, one per entry of `padas()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadaAssignment {
    Parasmaipada,
    Atmanepada,
    Ubhayapada,
    /// Both padas derive, exactly as for `Ubhayapada` — but the ātmanepada
    /// arm is sanctioned by 1.3.66 *bhujo'navane*, a sūtra that names the
    /// root, rather than by a svarita/ñit marker read through 1.3.72. The
    /// distinction is what the TRACE needs: a root carrying this assignment
    /// must never reach 1.3.72, or the derivation credits the wrong sūtra.
    ///
    /// 1.3.66 restricts ātmanepada to senses other than protecting
    /// (*anavane*). Neither this engine nor vidyut-prakriya models sense,
    /// so the assignment is UNCONDITIONAL — both readings derive, the
    /// parasmaipada one by 1.3.78's śeṣa (the *avane* reading), and the
    /// reader selects by sense. Same precedent as 1.3.72's own unmodelled
    /// *kartrabhiprāye kriyāphale*; see the 1.3.66 comment block in
    /// panini-prakriya's `tinanta/samjna.rs`.
    UbhayapadaAnavane,
    /// Both padas derive, exactly as for `Ubhayapada` — but the ātmanepada
    /// arm is sanctioned by 1.3.74 *ṇicaś ca*, a sūtra keyed on the affix
    /// ṇic, rather than by any marker of the root's. Every curādi root takes
    /// ṇic (3.1.25), so a curādi row with no pada marker of its own is
    /// curated with this — unless it is ākusmīya (`AKUSMIYA`), which is
    /// `Akusmiya`. Like `UbhayapadaAnavane`, a root carrying it must
    /// never reach 1.3.72, or the trace credits the wrong sūtra.
    ///
    /// Row-driven for now: 1.3.74 reads the pada licence the data layer
    /// supplies, not the presence of ṇic. When the causative (ṇic after a
    /// root of any gaṇa) is implemented, 1.3.74 should key on the ṇijanta
    /// stem instead, and this variant retires.
    Nic,
    /// Both padas derive on the ṇic branch, exactly as for `Nic` — and on the
    /// ṇic-LESS branch too, because the row's own upadeśa is svarita or ñit.
    /// One sūtra sanctions the ātmanepada on each branch: 1.3.74 *ṇicaś ca*
    /// where ṇic is taken, 1.3.72 *svaritañitaḥ* where it is not. vidyut-
    /// prakriya reads the svarita/ñit marker of the term it is handed, and on
    /// the ṇic branch that term is ṇic, which carries none: 1.3.72 never
    /// reaches a ṇic branch, and 1.3.74 never a ṇic-less one. Only a row whose
    /// ṇic is optional (`OPTIONAL_NIC`) can carry it.
    NicUbhayapada,
    /// Ātmanepada only, sanctioned by the dhātupāṭha gaṇasūtra 10.0496
    /// *ā kusmād ātmanepadinaḥ*: the curādi roots from `10.0192 cita~` up to
    /// `10.0236 kusma~` (`AKUSMIYA`) are ātmanepadī. Not a marker of the
    /// root's — their upadeśas carry none, so 1.3.12 never reaches them — and
    /// not the affix's, so 1.3.74 must not credit them either. A root
    /// carrying it is credited 10.0496 and no pada sūtra at all: never
    /// 1.3.12, 1.3.74 or 1.3.78.
    ///
    /// Keyed on the row's position, as the gaṇasūtra is; vidyut-prakriya
    /// reads the same range (`maybe_find_antargana`). The gaṇasūtra applies
    /// only on the ṇic branch: a row whose ṇic is optional (`OPTIONAL_NIC`,
    /// slice 10f's six) is still `Akusmiya`, the ṇic branch's verdict, and
    /// `Dhatu::padas` adds the ṇic-less branch's parasmaipada.
    Akusmiya,
    /// Ātmanepada only, sanctioned by the dhātupāṭha gaṇasūtra 10.0497
    /// *ā garvād ātmanepadinaḥ*: the curādi roots from `10.0440 pada` up to
    /// `10.0449 garva` (`AA_GARVIYA`) are ātmanepadī. Same standing as
    /// `Akusmiya`: neither a marker of the root's nor the affix's, so a root
    /// carrying it is credited 10.0497 and no pada sūtra at all: never
    /// 1.3.12, 1.3.74 or 1.3.78.
    ///
    /// Keyed on the row's position, as the gaṇasūtra is. vidyut-prakriya
    /// lists the same ten upadeśas (`AA_GARVIYA`). The gaṇasūtra applies only
    /// on the ṇic branch: `10.0449 garva`, whose ṇic is optional (`OPTIONAL_NIC`),
    /// is still `AaGarviya`, and `Dhatu::padas` adds its ṇic-less parasmaipada.
    AaGarviya,
}
/// The ākusmīya antargaṇa of curādi: dhātupāṭha rows `10.0192` (`cita~`)
/// through `10.0236` (`kusma~`), the scope of the gaṇasūtra 10.0496 *ā
/// kusmād ātmanepadinaḥ*. Compared as strings, which the zero-padded
/// numbering makes order-correct. A curated row is
/// `PadaAssignment::Akusmiya` exactly when it is curādi and in this range;
/// `curated_pada_agrees_with_upadesha_markers` holds both sides.
pub const AKUSMIYA: RangeInclusive<&str> = "10.0192"..="10.0236";

/// The jñapādi of curādi: dhātupāṭha rows `10.0118` (`jYapa~`) through
/// `10.0124` (`ciY`), the roots the gaṇasūtra 10.0493 makes mit. Compared as
/// strings, like `AKUSMIYA`. The engine's `derive` tags a curādi root in this
/// range `Tag::Mit`, which 10.0493 and 6.4.92 *mitāṃ hrasvaḥ* read. All seven
/// are curated; `10.0124 ciY`, the last, since slice 10j.
/// `jnapadi_is_exactly_the_rows_10_0493_names` pins the range to upstream.
pub const JNAPADI: RangeInclusive<&str> = "10.0118"..="10.0124";

/// The rows 3.1.28 *gupūdhūpavicchipaṇipanibhya āyaḥ* gives the pratyaya
/// āya: `10.0303 DUpa~` and `10.0304 viCa~`, which take it where their
/// optional ṇic is not taken (*dhūpāyati*, *vicchāyati*). Keyed by
/// dhātupāṭha number, as `JNAPADI` is, because a stored code cannot decide
/// it: `10.0302 gupa~` stores as `gup`, as the bhvādi `gupU~` the sūtra
/// names does, and takes no āya. vidyut-prakriya keys the sūtra on five
/// upadeśas in any gaṇa (`gupU~`, `DUpa~`, `viCa~`, `paRa~\`, `pana~\`);
/// each joins this list when its row is curated. The engine's `derive` tags
/// a listed root `Tag::Aya`, which 3.1.28 reads.
/// `aya_is_exactly_the_curated_rows_3_1_28_names` pins it to upstream.
pub const AYA: [&str; 2] = ["10.0303", "10.0304"];

/// The ā-garvīya of curādi: dhātupāṭha rows `10.0440` (`pada`) through
/// `10.0449` (`garva`), the scope of the gaṇasūtra 10.0497 *ā garvād
/// ātmanepadinaḥ*. Compared as strings, like `AKUSMIYA`. A curated row is
/// `PadaAssignment::AaGarviya` exactly when it is curādi and in this range;
/// `curated_pada_agrees_with_upadesha_markers` holds both sides. Includes
/// `10.0449 garva`, whose ṇic is optional (`OPTIONAL_NIC`): 10.0497 makes
/// only its ṇic branch ātmanepadī. `aa_garviya_is_exactly_the_rows_10_0497_names`
/// pins the range to upstream.
pub const AA_GARVIYA: RangeInclusive<&str> = "10.0440"..="10.0449";

/// The curādi rows whose ṇic is optional, each with the id of the rule that
/// makes it so: the dhātupāṭha gaṇasūtras 10.0498 *ā dhṛṣād vā* for an
/// ādhṛṣīya row (`10.0338`–`10.0388`) and 10.0499 *ā svadaḥ sakarmakāt* for
/// an āsvadīya one (`10.0279`–`10.0337`), which vidyut-prakriya decides
/// before any marker, and otherwise the Kaumudī's 2564 for an idit root,
/// 2565 for `pF`, 2570 for a ñit or udit root (udit by a final `u~`, or by
/// an initial one, as `10.0270 u~Drasa~` is), 2571 for `Guzi~r`, 2573.1
/// for `pata`, and 2573.3 for the roots that rule names (`mUtra`, `katra`,
/// `garva`). Keyed by dhātupāṭha number, as
/// `JNAPADI` is: the engine never
/// sees an upadeśa's markers, so `danS` and `div` cannot say they are idit
/// or udit. The engine's sanādi stage forks each listed row into a ṇic
/// branch and a ṇic-less one (`Dhatu::padas`).
///
/// Lists curated rows only. `optional_nic_matches_upadesha_markers`
/// re-derives every entry from the vendored upadeśa and holds the table to
/// exactly the curated rows those markers select.
pub const OPTIONAL_NIC: &[(&str, &str)] = &[
    ("10.0002", "2564"),
    ("10.0003", "2564"),
    ("10.0004", "2564"),
    ("10.0005", "2564"),
    ("10.0007", "2564"),
    ("10.0009", "2564"),
    ("10.0011", "2564"),
    ("10.0013", "2564"),
    ("10.0014", "2564"),
    ("10.0022", "2565"),
    ("10.0043", "2564"),
    ("10.0045", "2564"),
    ("10.0047", "2564"),
    ("10.0048", "2564"),
    ("10.0049", "2564"),
    ("10.0060", "2564"),
    ("10.0062", "2564"),
    ("10.0066", "2564"),
    ("10.0067", "2564"),
    ("10.0068", "2564"),
    ("10.0069", "2564"),
    ("10.0070", "2564"),
    ("10.0071", "2564"),
    ("10.0072", "2564"),
    ("10.0073", "2564"),
    ("10.0074", "2564"),
    ("10.0075", "2564"),
    ("10.0076", "2564"),
    ("10.0077", "2564"),
    ("10.0105", "2564"),
    ("10.0106", "2564"),
    ("10.0107", "2564"),
    ("10.0111", "2564"),
    ("10.0112", "2564"),
    ("10.0113", "2564"),
    ("10.0114", "2564"),
    ("10.0124", "2570"),
    ("10.0130", "2564"),
    ("10.0135", "2564"),
    ("10.0147", "2564"),
    ("10.0153", "2564"),
    ("10.0157", "2564"),
    ("10.0158", "2564"),
    ("10.0159", "2564"),
    ("10.0160", "2564"),
    ("10.0164", "2564"),
    ("10.0166", "2564"),
    ("10.0171", "2564"),
    ("10.0174", "2570"),
    ("10.0182", "2564"),
    ("10.0184", "2570"),
    ("10.0185", "2564"),
    ("10.0193", "2564"),
    ("10.0194", "2564"),
    ("10.0198", "2564"),
    ("10.0199", "2564"),
    ("10.0227", "2570"),
    ("10.0230", "2570"),
    ("10.0241", "2564"),
    ("10.0243", "2570"),
    ("10.0249", "2570"),
    ("10.0251", "2571"),
    ("10.0254", "2564"),
    ("10.0260", "2570"),
    ("10.0266", "2570"),
    ("10.0267", "2564"),
    ("10.0270", "2570"),
    ("10.0279", "10.0499"),
    ("10.0280", "10.0499"),
    ("10.0281", "10.0499"),
    ("10.0282", "10.0499"),
    ("10.0283", "10.0499"),
    ("10.0284", "10.0499"),
    ("10.0285", "10.0499"),
    ("10.0286", "10.0499"),
    ("10.0287", "10.0499"),
    ("10.0288", "10.0499"),
    ("10.0289", "10.0499"),
    ("10.0290", "10.0499"),
    ("10.0291", "10.0499"),
    ("10.0292", "10.0499"),
    ("10.0293", "10.0499"),
    ("10.0294", "10.0499"),
    ("10.0295", "10.0499"),
    ("10.0296", "10.0499"),
    ("10.0297", "10.0499"),
    ("10.0298", "10.0499"),
    ("10.0299", "10.0499"),
    ("10.0300", "10.0499"),
    ("10.0301", "10.0499"),
    ("10.0302", "10.0499"),
    ("10.0303", "10.0499"),
    ("10.0304", "10.0499"),
    ("10.0305", "10.0499"),
    ("10.0306", "10.0499"),
    ("10.0307", "10.0499"),
    ("10.0308", "10.0499"),
    ("10.0309", "10.0499"),
    ("10.0310", "10.0499"),
    ("10.0311", "10.0499"),
    ("10.0312", "10.0499"),
    ("10.0313", "10.0499"),
    ("10.0314", "10.0499"),
    ("10.0315", "10.0499"),
    ("10.0316", "10.0499"),
    ("10.0317", "10.0499"),
    ("10.0318", "10.0499"),
    ("10.0319", "10.0499"),
    ("10.0320", "10.0499"),
    ("10.0321", "10.0499"),
    ("10.0322", "10.0499"),
    ("10.0323", "10.0499"),
    ("10.0324", "10.0499"),
    ("10.0325", "10.0499"),
    ("10.0326", "10.0499"),
    ("10.0327", "10.0499"),
    ("10.0328", "10.0499"),
    ("10.0329", "10.0499"),
    ("10.0330", "10.0499"),
    ("10.0331", "10.0499"),
    ("10.0332", "10.0499"),
    ("10.0333", "10.0499"),
    ("10.0334", "10.0499"),
    ("10.0335", "10.0499"),
    ("10.0336", "10.0499"),
    ("10.0337", "10.0499"),
    ("10.0338", "10.0498"),
    ("10.0339", "10.0498"),
    ("10.0340", "10.0498"),
    ("10.0341", "10.0498"),
    ("10.0342", "10.0498"),
    ("10.0343", "10.0498"),
    ("10.0344", "10.0498"),
    ("10.0345", "10.0498"),
    ("10.0346", "10.0498"),
    ("10.0347", "10.0498"),
    ("10.0348", "10.0498"),
    ("10.0349", "10.0498"),
    ("10.0350", "10.0498"),
    ("10.0351", "10.0498"),
    ("10.0352", "10.0498"),
    ("10.0353", "10.0498"),
    ("10.0354", "10.0498"),
    ("10.0355", "10.0498"),
    ("10.0356", "10.0498"),
    ("10.0357", "10.0498"),
    ("10.0358", "10.0498"),
    ("10.0359", "10.0498"),
    ("10.0360", "10.0498"),
    ("10.0361", "10.0498"),
    ("10.0362", "10.0498"),
    ("10.0363", "10.0498"),
    ("10.0364", "10.0498"),
    ("10.0365", "10.0498"),
    ("10.0366", "10.0498"),
    ("10.0367", "10.0498"),
    ("10.0369", "10.0498"),
    ("10.0370", "10.0498"),
    ("10.0371", "10.0498"),
    ("10.0372", "10.0498"),
    ("10.0373", "10.0498"),
    ("10.0374", "10.0498"),
    ("10.0375", "10.0498"),
    ("10.0376", "10.0498"),
    ("10.0377", "10.0498"),
    ("10.0378", "10.0498"),
    ("10.0379", "10.0498"),
    ("10.0380", "10.0498"),
    ("10.0381", "10.0498"),
    ("10.0382", "10.0498"),
    ("10.0383", "10.0498"),
    ("10.0384", "10.0498"),
    ("10.0385", "10.0498"),
    ("10.0386", "10.0498"),
    ("10.0387", "10.0498"),
    ("10.0388", "10.0498"),
    ("10.0400", "2573.1"),
    ("10.0449", "2573.3"),
    ("10.0451", "2573.3"),
    ("10.0456", "2573.3"),
    ("10.0464", "2564"),
    ("10.0465", "2564"),
];

/// The id of the rule that makes row `dhatupatha`'s ṇic optional, if any
/// (`OPTIONAL_NIC`).
pub fn optional_nic(dhatupatha: &str) -> Option<&'static str> {
    OPTIONAL_NIC
        .iter()
        .find(|(n, _)| *n == dhatupatha)
        .map(|(_, id)| *id)
}
impl PadaAssignment {
    /// The padas this assignment derives. `Ubhayapada` lists parasmaipada
    /// first — pinned, not incidental; see
    /// `ubhayapada_padas_are_parasmaipada_first` for why.
    pub fn padas(&self) -> &'static [Pada] {
        match self {
            PadaAssignment::Parasmaipada => &[Pada::Parasmaipada],
            PadaAssignment::Atmanepada => &[Pada::Atmanepada],
            PadaAssignment::Ubhayapada
            | PadaAssignment::UbhayapadaAnavane
            | PadaAssignment::Nic
            | PadaAssignment::NicUbhayapada => &[Pada::Parasmaipada, Pada::Atmanepada],
            PadaAssignment::Akusmiya | PadaAssignment::AaGarviya => &[Pada::Atmanepada],
        }
    }
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
    /// Dhātupāṭha entry number — the unique key. Names a row of
    /// `data/dhatupatha.tsv`, and `dhatupatha_numbers_resolve_upstream`
    /// checks that the row it names is the right one, by it-stripping that
    /// row's upadeśa and comparing against `code`.
    pub dhatupatha: &'static str,
    /// The root's SLP1 text, as it enters the derivation. Deliberately not
    /// unique — both √aś rows spell `aS` — and never a lookup key. Where it
    /// differs from the it-stripped upadeśa the reason is a rule this engine
    /// does not derive: `07.0019` stores `hins` for `hisi~` because 7.1.58
    /// idito num dhātoḥ is kept as a stated simplification, and `05.0021`
    /// stores `stiG` for `zwiGa~\` per 6.1.64 dhātvādeḥ ṣaḥ saḥ and its
    /// vārttika, which carries the following retroflex with it.
    pub code: &'static str,
    pub gana: Gana,
    /// Which pada(s) this engine derives for this root. Curated rather than
    /// read from the upadeśa's it-markers — but no longer a *deferral*:
    /// `curated_pada_agrees_with_upadesha_markers` re-derives 102 of these 594
    /// verdicts from the vendored upadeśa via 1.3.12 / 1.3.72 / 1.3.78 and
    /// requires them to match; `07.0017`'s (√bhuj's) is 1.3.66's root-keyed
    /// exception, 428 curādi rows' are 1.3.74's, seven more 1.3.74's with ṇic
    /// and 1.3.72's without (`NicUbhayapada`), 45 ākusmīya rows'
    /// the gaṇasūtra 10.0496's, ten ā-garvīya rows' the gaṇasūtra
    /// 10.0497's and `10.0058 zmiN`'s Kaumudī 2567's and 1.3.12's, each
    /// asserted explicitly from both sides, the same way
    /// `dhatupatha_numbers_resolve_upstream` holds `code` to upstream. For
    /// the 182 curādi rows whose ṇic is optional this is the ṇic branch's
    /// pada; the test also re-derives their ṇic-less branch's from the
    /// upadeśa: 1.3.72's for the seven `NicUbhayapada` rows, 1.3.78's for the
    /// rest.
    ///
    /// The column stayed hand-written because deriving it in production means
    /// running it-stripping in production, and upadeśa preprocessing is not
    /// the tiṅanta pipeline `TINANTA_RULES` models — it needs its own pipeline
    /// concept. Until it has one, a curated column plus a non-circular test is
    /// the honest arrangement; see the deferral in
    /// `docs/superpowers/specs/2026-08-16-pada-audit-design.md`.
    ///
    /// The test covers the 594 roots curated here, not the dhātupāṭha's 2259.
    /// It catches a mis-assigned pada on a root a future slice adds; it does
    /// not make the table self-maintaining.
    pub pada: PadaAssignment,
    pub artha: &'static str,
}

impl Dhatu {
    /// The padas this root derives. `pada` is the ṇic branch's verdict; a
    /// root whose ṇic is optional (`OPTIONAL_NIC`) also derives its ṇic-less
    /// branch: parasmaipada by 1.3.78, or, for a `NicUbhayapada` row, both
    /// padas by 1.3.72, which that assignment already admits. So an
    /// ākusmīya or ā-garvīya row in the table admits both padas,
    /// parasmaipada first, as `PadaAssignment::padas` orders every
    /// two-pada assignment.
    pub fn padas(&self) -> &'static [Pada] {
        match (optional_nic(self.dhatupatha), self.pada.padas()) {
            (Some(_), [Pada::Atmanepada]) => &[Pada::Parasmaipada, Pada::Atmanepada],
            (_, padas) => padas,
        }
    }
}

static DHATUS: &[Dhatu] = &[
    Dhatu {
        dhatupatha: "01.0001",
        code: "BU",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "sattAyAm",
    },
    // 01.1049 `RI\Y`: the final `Y` is an it by 1.3.3 halantyam, so 1.3.72
    // svaritañitaḥ sanctions both padas (nayati / nayate). Curated
    // parasmaipada from the v1 slice until the pada audit; no deferral list
    // ever named it.
    Dhatu {
        dhatupatha: "01.1049",
        code: "nI",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "prApaRe",
    },
    Dhatu {
        dhatupatha: "01.0642",
        code: "ji",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "jaye",
    },
    Dhatu {
        dhatupatha: "01.1082",
        code: "smf",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "cintAyAm",
    },
    Dhatu {
        dhatupatha: "01.0381",
        code: "paW",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.1164",
        code: "vad",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.0002",
        code: "eD",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vfdDO",
    },
    Dhatu {
        dhatupatha: "01.1130",
        code: "laB",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "prAptO",
    },
    Dhatu {
        dhatupatha: "01.0574",
        code: "sev",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "sevane",
    },
    Dhatu {
        dhatupatha: "01.0862",
        code: "vft",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vartane",
    },
    Dhatu {
        dhatupatha: "01.0696",
        code: "BAz",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        dhatupatha: "01.0694",
        code: "Ikz",
        gana: Gana::Bhvadi,
        pada: PadaAssignment::Atmanepada,
        artha: "darSane",
    },
    // divādi (gaṇa 4) — vikaraṇa śyan (3.1.69)
    Dhatu {
        dhatupatha: "04.0001",
        code: "div",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "krIqAvijigIzAvyavahAradyutistutimodamadasvapnakAntigatizu",
    },
    Dhatu {
        dhatupatha: "04.0091",
        code: "naS",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "adarSane",
    },
    Dhatu {
        dhatupatha: "04.0146",
        code: "kup",
        gana: Gana::Divadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kroDe",
    },
    Dhatu {
        dhatupatha: "04.0073",
        code: "man",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "jYAne",
    },
    Dhatu {
        dhatupatha: "04.0069",
        code: "yuD",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samprahAre",
    },
    Dhatu {
        dhatupatha: "04.0067",
        code: "vid",
        gana: Gana::Divadi,
        pada: PadaAssignment::Atmanepada,
        artha: "sattAyAm",
    },
    // tudādi (gaṇa 6) — vikaraṇa śa (3.1.77)
    // 06.0001 `tu\da~^`: the `~^` is a svarita it, so 1.3.72 sanctions both
    // padas (tudati / tudate). Deferred behind 1.3.72 by the divādi/tudādi
    // slice, then behind curation once 1.3.72 landed; discharged by the pada
    // audit.
    Dhatu {
        dhatupatha: "06.0001",
        code: "tud",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vyaTane",
    },
    Dhatu {
        dhatupatha: "06.0092",
        code: "liK",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "akzaravinyAse",
    },
    Dhatu {
        dhatupatha: "06.0160",
        code: "viS",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "praveSane",
    },
    Dhatu {
        dhatupatha: "06.0008",
        code: "juz",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "prItisevanayoH",
    },
    Dhatu {
        dhatupatha: "06.0009",
        code: "vij",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "BayacalanayoH",
    },
    Dhatu {
        dhatupatha: "06.0131",
        code: "gur",
        gana: Gana::Tudadi,
        pada: PadaAssignment::Atmanepada,
        artha: "udyamane",
    },
    // adādi (gaṇa 2) — śap luk (2.4.72). √ad/√yā/√vā parasmaipada; √ās/√vas
    // ātmanepada — covered across all four lakāras (laṭ/laṅ/loṭ/vidhiliṅ).
    // √vas here is `vas` ācchādane (2Ā, "to wear"), NOT the far commoner
    // `vas` nivāse (1P, "to dwell", vasati); artha is the only disambiguator.
    Dhatu {
        dhatupatha: "02.0044",
        code: "yA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prApaRe",
    },
    Dhatu {
        dhatupatha: "02.0045",
        code: "vA",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatiganDanayoH",
    },
    Dhatu {
        dhatupatha: "02.0001",
        code: "ad",
        gana: Gana::Adadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BakzaRe",
    },
    Dhatu {
        dhatupatha: "02.0011",
        code: "As",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "upaveSane",
    },
    Dhatu {
        dhatupatha: "02.0013",
        code: "vas",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "AcCAdane",
    },
    Dhatu {
        dhatupatha: "02.0026",
        code: "SI",
        gana: Gana::Adadi,
        pada: PadaAssignment::Atmanepada,
        artha: "svapne",
    },
    Dhatu {
        dhatupatha: "09.0058",
        code: "kliS",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vibADane",
    },
    Dhatu {
        dhatupatha: "09.0053",
        code: "guD",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "roze",
    },
    Dhatu {
        dhatupatha: "09.0059",
        code: "aS",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Bojane",
    },
    Dhatu {
        dhatupatha: "09.0066",
        code: "muz",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "steye",
    },
    Dhatu {
        dhatupatha: "09.0040",
        code: "vrI",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "varaRe",
    },
    Dhatu {
        dhatupatha: "09.0045",
        code: "vf",
        gana: Gana::Kryadi,
        pada: PadaAssignment::Atmanepada,
        artha: "samBaktO",
    },
    // svādi (gaṇa 5) — vikaraṇa śnu (3.1.73)
    Dhatu {
        dhatupatha: "05.0016",
        code: "Ap",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyAptO",
    },
    Dhatu {
        dhatupatha: "05.0017",
        code: "Sak",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "SaktO",
    },
    Dhatu {
        dhatupatha: "05.0012",
        code: "hi",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatO vfdDO ca",
    },
    Dhatu {
        dhatupatha: "05.0032",
        code: "ri",
        gana: Gana::Svadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 05.0020 aSU~\ vyAptau. Distinct root from kryādi's 09.0059 aSa~
        // Bojane, which shares this SLP1 form — under the retired `id`
        // scheme that required a qualifier; dhātupāṭha numbers distinguish
        // the rows without one. aSnute against aSnAti is the pair.
        dhatupatha: "05.0020",
        code: "aS",
        gana: Gana::Svadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vyAptO saNGAte ca",
    },
    Dhatu {
        // 05.0021 zwiGa~\. Stored post-6.1.64 dhātvādeḥ ṣaḥ saḥ: no rule in
        // the engine performs that substitution, so it is a stated
        // simplification, not a derivation step. See the spec's Data section.
        dhatupatha: "05.0021",
        code: "stiG",
        gana: Gana::Svadi,
        pada: PadaAssignment::Atmanepada,
        artha: "Askandane",
    },
    Dhatu {
        // 07.0010 kftI~ vezwane. rudhādi's √kṛt, distinct from tudādi's
        // √kṛnt — not in the root set, so the retired `id` scheme never
        // needed to qualify it.
        dhatupatha: "07.0010",
        code: "kft",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vezwane",
    },
    Dhatu {
        // 07.0019 hisi~ hiMsAyAm. Stored post-7.1.58 idito num dhātoH: the
        // root is idit and takes num, but the engine models no it-markers
        // at all (every root here is stored post-it-elision), so 7.1.58 is
        // not derivable and the num is stored. A stated simplification, not
        // a derivation step — exactly as `stiG` is stored post-6.1.64.
        // This is the root that makes 6.4.23 SnAnnalopaH reachable: śnam
        // gives hinans, and 6.4.23 takes the root's own n back out.
        dhatupatha: "07.0019",
        code: "hins",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 07.0012 Ki\da~\ dEnye. The gaṇa's ātmanepada arm. rudhādi offers
        // only three ānudātta roots (√indh, √khid, √vid); √khid is the one
        // that needs no rule beyond the gaṇa's own.
        dhatupatha: "07.0012",
        code: "Kid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Atmanepada,
        artha: "dEnye",
    },
    Dhatu {
        // 07.0016 Ba\njo~ Amardane. Witnesses 8.2.30 coH kuH: the root's
        // cu-class final (j) becomes the matching velar (g) word-finally
        // or before a jhal-initial affix.
        dhatupatha: "07.0016",
        code: "Banj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Amardane",
    },
    Dhatu {
        // 07.0015 pi\zx~ saYcUrRane hiMsAyAm ca. Witnesses 8.4.41 (zwutva:
        // an adjacent dental assimilates to retroflex next to the root's
        // z) and 8.2.41 (the root's final z is itself replaced by k
        // before an s-initial affix).
        dhatupatha: "07.0015",
        code: "piz",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "saYcUrRane hiMsAyAm ca",
    },
    Dhatu {
        // 07.0011 YiinDI~\ dIptO. Witnesses 8.2.40 Jazas taTor Do'DaH: a
        // Jaz-class final (voiced aspirated stop, here D) aṅga turns a
        // following t/T-initial affix into D. Looks ubhayapadī too: the
        // ñi it-marker is read by 1.3.72 svaritaYitaH, alongside its own
        // svarita. It is not — the anudAtta `~\` on top of the ñi settles
        // pada by 1.3.12 anudAttaNita Atmanepadam, and vidyut-prakriya
        // derives it ātmanepada-only, checked against a `~^r` control
        // (√rudh) that does derive both padas.
        dhatupatha: "07.0011",
        code: "inD",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Atmanepada,
        artha: "dIptO",
    },
    Dhatu {
        // 07.0001 ru\Di~^r AvaraRe. The gaṇa's EPONYM, and the engine's
        // first ubhayapadī root: the `~^` svarita is what 1.3.72
        // svaritaYitaH reads, and — unlike YiinDI~\ five rows above — the
        // entry carries no trailing `~\` anudātta it-marker for 1.3.12
        // anudAttaNita Atmanepadam to read (the `\` it does carry is the
        // root vowel's own accent, not an it), so 1.3.12 does not pre-empt
        // the parasmaipada reading and both pada cells derive.
        //
        // It needs no new sūtra. Its ātmanepada arm is structurally
        // √indh's (8.2.40 JaSas taTor Do'DaH, then 8.4.65 optionally
        // eliding the `d`), and its strong parasmaipada arm is √bhañj's and
        // √piṣ's — but it does reach one arm of existing phonology no
        // curated root had reached before: laṅ prathama/madhyama eka expose
        // the dhātu's own final `D` pada-finally (8.2.23 having eaten
        // tip/sip's own consonant), which 8.2.39 JalAM jaSo'nte now reaches
        // through jashtva_of, not new phonology of its own.
        //
        // It is also the first root to reach 8.4.2 awkupvANnumvyavAye'pi —
        // the NON-ADJACENT ṇatva, trigger and target separated (here by the
        // root's own aṭ vowel `u`: r-u-n) — inside rudhādi, the one gaṇa
        // where 8.3.24 naScApadAntasya Jali is live. So ṇatva fires on the
        // strong stem (ruRadDi, ruRaDE) and declines on the weak
        // (runDanti), whose nasal 8.3.24 has already turned into an
        // anusvāra before either ṇatva rule looks. The strong/weak split
        // itself is NOT new: √kṛt has shown it since slice 7a (kfRatti vs
        // kfnttaH), at the adjacent-trigger 8.4.1.
        dhatupatha: "07.0001",
        code: "ruD",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "AvaraRe",
    },
    Dhatu {
        // 07.0002 Bi\di~^r vidAraRe. Ubhayapadī by 1.3.72 svaritaYitaH: the
        // `~^` svarita it, with no trailing `~\` for 1.3.12 to pre-empt it.
        // The plainest of slice 7c's four roots — it reaches no rule the
        // gaṇa had not already reached, and is here for coverage rather
        // than as a witness. Coverage is a sufficient reason for a root to
        // exist; the audit in that slice is what earns it its place.
        dhatupatha: "07.0002",
        code: "Bid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vidAraRe",
    },
    Dhatu {
        // 07.0006 kzu\di~^r sampezaRe. Ubhayapadī by 1.3.72. Witnesses
        // 8.4.2 awkupvANnumvyavAye'pi under a SIBILANT trigger: in
        // kzuRatti the trigger is the `z` of `kz`, the target is Snam's
        // `n`, and the root's own aw vowel `u` separates them. That is
        // √rudh's shape (ruRadDi, r-u-n) reached through z rather than r,
        // and it makes this the second root to show the strong/weak ṇatva
        // split inside rudhādi -- the one gaṇa where 8.3.24
        // naScApadAntasya Jali is live and bleeds ṇatva off the weak stem.
        // 8.4.2's other curated witnesses (vrIRAti, muzARa) are kryādi,
        // where 8.3.24 never competes.
        dhatupatha: "07.0006",
        code: "kzud",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "sampezaRe",
    },
    Dhatu {
        // 07.0007 yu\ji~^r yoge. Ubhayapadī by 1.3.72. The root that earns
        // its place structurally: it is j-final, so its strong stem reaches
        // 8.2.30 coH kuH (yunagti -> 8.4.55 Kari ca -> yunakti), and 8.2.30's
        // substitute is now the 1.1.50 nearest velar, read from kutva_of,
        // rather than the literal 'g' it used to be. √bhañj has been that
        // rule's witness since 7b; pinning √yuj's 72 cells gave the
        // generalisation slice a second independent anchor it did not have
        // to build as part of the change it was validating. √ric and √vic,
        // curated in this same slice, are the roots that generalisation
        // actually unlocked.
        dhatupatha: "07.0007",
        code: "yuj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "yoge",
    },
    Dhatu {
        // 07.0009 u~tfdi~^r hiMsAnAdarayoH. Ubhayapadī by 1.3.72. The
        // leading `u~` is an it by 1.3.2 upadeSe'j anunAsika it; it is
        // neither anudātta nor Nit, so it never reaches 1.3.12, and udit's
        // own consequence (7.2.56 udito vA, optional iw before ktvA) is not
        // a tiṅanta rule and so cannot touch these four lakāras.
        // Structurally √kṛt: ṇatva here is the ADJACENT 8.4.1 razAByAM no
        // RaH, not √kṣud's 8.4.2 -- tfRatti's trigger `f` sits directly
        // against the `n` with nothing intervening -- and it leans on
        // is_natva_trigger's `f | F` arm, the r-vowels counting as triggers
        // by 1.1.51 uraR raparaH.
        dhatupatha: "07.0009",
        code: "tfd",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAnAdarayoH",
    },
    Dhatu {
        // 07.0004 ri\ci~^r virecane. Ubhayapadī by 1.3.72 svaritaYitaH; the
        // `\` is the root vowel's own accent, not an it. THE FIRST `c` EVER
        // TO REACH 8.2.30 coH kuH: riRakti's stem-final `c` takes the
        // voiceless velar `k` directly, where √bhañj's and √yuj's `j` takes
        // `g` and needs 8.4.55 Kari ca to devoice it afterwards. That
        // one-step/two-step contrast is what pins the substitute as a real
        // 1.1.50 nearest-velar map rather than the literal 'g' it used to
        // be. Also an 8.4.2 awkupvAGnumvyavAye'pi witness: the root's `r`
        // retroflexes śnam's `n` across the intervening `i`.
        dhatupatha: "07.0004",
        code: "ric",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "virecane",
    },
    Dhatu {
        // 07.0005 vi\ci~^r pfTagBAve. Ubhayapadī by 1.3.72. The MINIMAL
        // CONTRAST to √ric: same gaṇa, same c-final shape, same vikaraṇa,
        // same 8.2.30 application -- and no ṇatva trigger at all, so
        // vinakti keeps its dental `n`. The pair isolates 8.4.2 against a
        // controlled background, the way 7c used √kṣud and √tṛd to separate
        // 8.4.2 from 8.4.1.
        dhatupatha: "07.0005",
        code: "vic",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "pfTagBAve",
    },
    Dhatu {
        // 07.0014 Si\zx~ viSezaRe. Structurally √piṣ (07.0015) with a
        // different head: both are z-final, so both drive 8.4.41 zwutva
        // (Sinazwi, the dental of `ti` retroflexed next to the root's z)
        // and 8.2.41 (the z replaced by k before an s-initial affix).
        // Curated as the witness that the z path is not piṣ-specific.
        dhatupatha: "07.0014",
        code: "Siz",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "viSezaRe",
    },
    Dhatu {
        // 07.0020 undI~ kledane. VOWEL-INITIAL and u-headed, which is what
        // makes it worth curating: its laN takes AT (6.4.72) and then
        // 6.1.90 AwaS ca, whose `u` -> `O` arm no curated root had ever
        // reached -- `vrddhi_of_ac_vowels_all_arms` in panini-prakriya's
        // sound.rs says in as many words that only e/I/E inputs occur.
        // Onad is the counterexample. The root's own `n` is 6.4.23's, and
        // 6.4.111 then takes śnam's `a`, exactly as for √bhañj.
        dhatupatha: "07.0020",
        code: "und",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kledane",
    },
    Dhatu {
        // 07.0021 anjU~ vyaktimrakzaRakAntigatizu. Vowel-initial like
        // √und, and the 8.2.30 witness among the four nasal-tailed roots
        // here: anaj -> anj (6.4.111) -> ang (8.2.30) -> aNk, the `j` arm
        // of kutva_of on a stem whose nasal 6.4.23 has already thinned.
        dhatupatha: "07.0021",
        code: "anj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "vyaktimrakzaRakAntigatizu",
    },
    Dhatu {
        // 07.0022 tancU~ saNkocane. The consonant-initial contrast to
        // √añj: same nasal tail, same 6.4.23, and a `c` rather than a `j`
        // for 8.2.30 -- so kutva_of's two cu arms are both driven by roots
        // of the same shape, differing only in voicing.
        dhatupatha: "07.0022",
        code: "tanc",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "saNkocane",
    },
    Dhatu {
        // 07.0023 o~vijI~ BayacalanayoH. The second `o~`-initial upadeśa
        // in the table, after tudādi's `06.0009`. Nothing new is needed
        // for it: 1.3.2's anunāsika-it
        // loop in strip_anubandhas takes `o~` like any other vowel + `~`
        // pair, and `curated_pada_agrees_with_upadesha_markers` checks the
        // verdict rather than trusting it.
        dhatupatha: "07.0023",
        code: "vij",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BayacalanayoH",
    },
    Dhatu {
        // 07.0024 vfjI~ varjane. f-headed, so śnam's own `n` retroflexes
        // by 8.4.1 raSAByAM no RaH -- vfRakti. The minimal contrast to
        // √pṛc below is the tail, not the trigger: both take ṇatva, and
        // only one of them also drives 8.2.30 on a `c`.
        dhatupatha: "07.0024",
        code: "vfj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "varjane",
    },
    Dhatu {
        // 07.0025 pfcI~ samparke. ṇatva by 8.4.1 like √vṛj, and 8.2.30 on
        // a `c` like √tañc -- the one curated root that stacks both, so it
        // pins that the ṇatva trigger and the kutva substitution do not
        // interfere. pfRakti / pfNktaH.
        dhatupatha: "07.0025",
        code: "pfc",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "samparke",
    },
    Dhatu {
        // 07.0013 vi\da~\ vicAraRe. Ātmanepada by 1.3.12 on its trailing
        // `~\`, and the gaṇa's third pure-ātmanepadī root after √khid and
        // √indh. Distinct from divādi's `vid` (04.0067) and every other
        // √vid by dhātupāṭha number, not by surface. 8.4.65 Jaro Jari
        // savarRe forks nearly every cell it has (vinte / vintte).
        dhatupatha: "07.0013",
        code: "vid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Atmanepada,
        artha: "vicAraRe",
    },
    Dhatu {
        // 07.0018 tfha~ hiMsAyAm. The gaṇa's ninth reachable
        // non-ubhayapadī root and the only one that needed sūtras this
        // engine lacked: 7.3.92 tfRaha im puts the *im* āgama into the
        // stem (tfnah -> tfnaih -> tfneh by 6.1.87), 8.2.31 ho QaH takes
        // the root's `h` to `Q`, and 8.3.13 Qo Qe lopaH elides it before
        // the `Q` that 8.4.41 produces -- tfReQi.
        //
        // The im is conditioned on a HAL-INITIAL PIT sārvadhātuka, which
        // is why this one root's paradigm splits three ways rather than
        // two: tfReQi/tfRekzi/tfRehmi take it, tfRQaH/tfMhanti do not
        // (apit, hence ṅit by 1.2.4), and atfRaham does not either
        // (`am` is vowel-initial).
        dhatupatha: "07.0018",
        code: "tfh",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 07.0003 Ci\di~^r dvEDIkaraRe. Ubhayapadī by 1.3.72 svaritaYitaH:
        // the `~^` is a svarita it, while the `\` is the root vowel's own
        // accent and says nothing about pada. Shape-identical to √bhid
        // (`07.0002`) -- `Ci` + `nad` where √bhid has `Bi` + `nad` -- so
        // every cell outside laṅ derives on rules already in the pipeline.
        //
        // The laṅ cells are the whole of what this root cost: 6.4.71's aṭ
        // puts a short `a` before the root's initial `C`, 6.1.73 Ce ca
        // inserts the tuk after it, and 8.4.40 stoH ScunA ScuH makes that
        // `t` a `c` -- acCinat, where the engine would otherwise reach
        // *aCinat.
        dhatupatha: "07.0003",
        code: "Cid",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dvEDIkaraRe",
    },
    Dhatu {
        // 07.0008 u~Cfdi~^r dIptidevanayoH. Ubhayapadī by 1.3.72, on the
        // same svarita it as √chid. Udit, like √tṛd (`07.0009`) -- the
        // initial `u~` matters for 7.2.56 and 1.2.26 in ārdhadhātuka
        // contexts this engine does not cover, and is inert across all four
        // sārvadhātuka lakāras here.
        //
        // Shape-identical to √tṛd: `Cf` + `Rad` where √tṛd has `tf` + `Rad`,
        // 8.4.1's ṇatva included, since the trigger is the root's own `f`.
        // The tuk 6.1.73 inserts sits in FRONT of that `f` rather than
        // between it and the `n`, so it raises no 8.4.2 intervener question.
        dhatupatha: "07.0008",
        code: "Cfd",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dIptidevanayoH",
    },
    Dhatu {
        // 07.0017 Bu\ja~ pAlanAByavahArayoH. The row that closes rudhādi
        // at 25 of 25. Its `\` sits on the ROOT VOWEL — no pada anubandha
        // at all — so the markers say parasmaipada, and the ātmanepada arm
        // comes from 1.3.66 Bujo'navane, the one curated verdict a sūtra
        // carries rather than a marker; UbhayapadaAnavane is that fact
        // stated as data, and `curated_pada_agrees_with_upadesha_markers`
        // asserts both sides of the divergence explicitly.
        //
        // No new phonology: √bhuj is √yuj (`07.0007`) with a `B` for its
        // `y` — śnam, 8.2.30's kutva on the final `j`, the same 8.4.56
        // and 7.1.35 forks, and no 8.4.65 anywhere (a velar junction is
        // never savarṇa with a dental). 7f's design probe certified 1.3.66
        // as the only rule vidyut invokes for it that this engine lacked.
        dhatupatha: "07.0017",
        code: "Buj",
        gana: Gana::Rudhadi,
        pada: PadaAssignment::UbhayapadaAnavane,
        artha: "pAlanAByavahArayoH",
    },
    Dhatu {
        // 08.0001 tanu~^ vistAre. The gaṇa's eponym and its plainest row:
        // a-upadhā (nothing for 7.3.86 to touch), n-final after a vowel
        // (asaṁyogapūrva: tanu, tanvaH/tanuvaH), svarita-it → 1.3.72 →
        // ubhayapadī. tanoti / tanute.
        dhatupatha: "08.0001",
        code: "tan",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vistAre",
    },
    Dhatu {
        // 08.0002 zaRu~^ dAne. Stored per 6.1.64 dhAtvAdeH SaH saH like
        // √ṣṭigh (stiG): the upadeśa's z becomes s, and with the z gone
        // its conditioned retroflex R reverts to n (nimitta-nāśa) —
        // sanoti / sanute, as vidyut derives. stored_form's z-arm carries
        // the same reversal so the resolve test derives san, not saR.
        dhatupatha: "08.0002",
        code: "san",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dAne",
    },
    Dhatu {
        // 08.0003 kzaRu~^ hiMsAyAm. a-upadhā like √tan; the root's own R
        // (retroflex by its z) stays. kzaRoti / kzaRute.
        dhatupatha: "08.0003",
        code: "kzaR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 08.0004 kziRu~^ hiMsAyAm. First of the four ik-upadhā roots the
        // tanādi gaṇasūtra (Kaumudī 2547.1; see guna.rs's 7.3.86 vikalpa
        // arm) forks: kziRoti / kzeRoti, and likewise through the whole
        // paradigm. Same artha as its neighbour kzaR — the number, not
        // the meaning, is the identity.
        dhatupatha: "08.0004",
        code: "kziR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 08.0005 fRu~^ gatO. Vowel-initial AND ik-upadhā (the f is both):
        // the 7.3.86 fork (fRoti/arRoti), the laṅ āṭ (6.4.72 + 6.1.90,
        // whose f → Ar vṛddhi arm this root is the first to reach:
        // ArRot), and the fork CONVERGING under that vṛddhi — A+fR and
        // A+arR are both ArR — which is what run_pipeline's convergent-
        // fork collapse exists for. In loṭ the two stems split the
        // asaṁyogapūrva test: fRu luks hi, arRu (rR conjunct) keeps it —
        // fRu beside arRuhi, the widened helper's sharpest witness.
        dhatupatha: "08.0005",
        code: "fR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "gatO",
    },
    Dhatu {
        // 08.0006 tfRu~^ adane. ik-upadhā fork: tfRoti / tarRoti.
        dhatupatha: "08.0006",
        code: "tfR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "adane",
    },
    Dhatu {
        // 08.0007 GfRu~^ dIptO. ik-upadhā fork: GfRoti / GarRoti.
        dhatupatha: "08.0007",
        code: "GfR",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dIptO",
    },
    Dhatu {
        // 08.0008 vanu~\ yAcane. Anudātta → 1.3.12 → ātmanepadī: vanute.
        // vidyut-prakriya ALSO derives vanoti, via Kaumudī 2547.2 — a
        // gaṇasūtra that optionally removes this one root's anudātta-it.
        // Curated on the dhātupāṭha marker alone, the same record-don't-
        // model posture as 1.3.72's sense condition: the audit enumerates
        // this table's padas, so the parasmaipada column it does not have
        // is a documented deviation, not a latent diff.
        dhatupatha: "08.0008",
        code: "van",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Atmanepada,
        artha: "yAcane",
    },
    Dhatu {
        // 08.0009 manu~\ avaboDane. Anudātta → 1.3.12 → ātmanepadī:
        // manute. Unlike van, no gaṇasūtra clouds it — vidyut derives no
        // parasmaipada either.
        dhatupatha: "08.0009",
        code: "man",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Atmanepada,
        artha: "avaboDane",
    },
    Dhatu {
        // 08.0010 qukf\Y karaRe. √kṛ — ñit (the final Y; the qu is a
        // ḍu-it, 1.3.5) → 1.3.72 → ubhayapadī: karoti / kurute. Rides the
        // same 3.1.79 (the sūtra's own *kṛñbhya*), with the three
        // root-keyed specials in guna.rs: 6.4.110 ata ut (kurutaH),
        // 6.4.108 nityaṁ karoteḥ (kurmaH — the lopa 6.4.107 makes
        // optional is nitya here, so NO alternates), 6.4.109 ye ca
        // (kuryAt). 8.2.77 hali ca IS implemented in this engine (since
        // the divādi slice, for dIvyati) and would lengthen `kur`'s `u`
        // to `U` — but 8.2.79 na BakurCurAm is implemented too, as a
        // named guard clause inside 8.2.77 itself (`tripadi.rs`, matching
        // an aṅga ending `kur`), so every kur cell declines the
        // lengthening and derives kurvanti, not kUrvanti.
        dhatupatha: "08.0010",
        code: "kf",
        gana: Gana::Tanadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "karaRe",
    },
    Dhatu {
        // 03.0001 hu\ dAnAdAnayoH AdAne prIRane ca. The ślu gaṇa's eponym
        // (juhoti): 2.4.75 ślu, 6.1.10 dvitva, 7.4.62 kuhoś cuḥ on the
        // abhyāsa (hu → Ju, then 8.4.54 → ju), 6.4.87's hu arm (juhvati),
        // 6.4.101's hu arm (juhuDi), 3.4.109 + 7.3.83 in laṅ (ajuhavuH).
        // The `\` sits on the root vowel — svara, not an anubandha — so
        // 1.3.78 → parasmaipadī. Slice 3a.
        dhatupatha: "03.0001",
        code: "hu",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "dAnAdAnayoH AdAne prIRane ca",
    },
    Dhatu {
        // 03.0002 `YiBI\` Baye. The initial ñi is an it by 1.3.5 and
        // decides no pada — see `pada_from_upadesha`. Parasmaipada by
        // 1.3.78. Its ī is what 6.4.115 optionally shortens (bibhītaḥ /
        // bibhitaḥ) and what 6.4.82 turns to y before a vowel (bibhyati):
        // `BiBI` is asaṁyogapūrva where √hrī's `JihrI` is not.
        dhatupatha: "03.0002",
        code: "BI",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Baye",
    },
    Dhatu {
        // 03.0003 `hrI\` lajjAyAm. The gaṇa's only cluster-initial root, so
        // its abhyāsa is 7.4.60's only witness anywhere: hrI → hI → hi → Ji
        // → ji. The conjunct is also why 6.4.82 declines and 6.4.77's iyaṅ
        // arm takes the cell instead (jihriyati).
        dhatupatha: "03.0003",
        code: "hrI",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "lajjAyAm",
    },
    Dhatu {
        // 03.0004 `pF` pAlanapUraRayoH (√pṝ). Parasmaipadī by 1.3.78 (no accent
        // mark at all). Its abhyāsa takes `i` by 7.4.77, keyed by this number
        // (piparti). 7.1.102 makes its labial-preceded ṝ `ur` wherever guṇa
        // declines (pipurati), and 8.2.77 lengthens that before a consonant
        // (pipUrtaH). Slice 3d.
        dhatupatha: "03.0004",
        code: "pF",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "pAlanapUraRayoH",
    },
    Dhatu {
        // 03.0005 `pf\` pAlanapUraRayoH (√pṛ). Parasmaipadī by 1.3.78 (the `\`
        // is the root vowel's accent). 7.4.66 then 7.4.60 then 7.4.77, keyed by
        // this number: piparti, pipftaH. Shares every guṇated form with
        // 03.0004. Slice 3d.
        dhatupatha: "03.0005",
        code: "pf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "pAlanapUraRayoH",
    },
    Dhatu {
        // 03.0006 `quBf\Y` DAraRapozaRayoH (√bhṛñ). The `qu` is an it by
        // 1.3.5; ubhayapadī by 1.3.72 (ñit). Its abhyāsa takes `i` by 7.4.76,
        // keyed by this number (bibharti), and 6.1.77's aṅga arm gives
        // bibhrati / bibhrAte. Slice 3d.
        dhatupatha: "03.0006",
        code: "Bf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "DAraRapozaRayoH",
    },
    Dhatu {
        // 03.0007 `mA\N` mAne Sabde ca (√māṅ). Ātmanepadī by 1.3.12 (final ṅ).
        // 7.4.76 bhṛñām it names it, so its abhyāsa is `mi` (mimIte) — keyed
        // by this number, not by `mA`. Not ghu: 6.4.113 gives its ī before a
        // consonant (mimIte), 6.4.112 elides its ā before a vowel (mimate).
        // Slice 3c.
        dhatupatha: "03.0007",
        code: "mA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Atmanepada,
        artha: "mAne Sabde ca",
    },
    Dhatu {
        // 03.0008 `o~hA\N` gatO (√ohāṅ). The `o~` is an it by 1.3.2;
        // ātmanepadī by 1.3.12. Enters the derivation as `hA`, exactly like
        // 03.0009 `o~hA\k` (jahāti; slice 3c2) — which is why 7.4.76, naming
        // this row and not that one, keys on the NUMBER: jihIte. Slice 3c.
        dhatupatha: "03.0008",
        code: "hA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Atmanepada,
        artha: "gatO",
    },
    Dhatu {
        // 03.0009 `o~hA\k` tyAge (√ohāk, jahāti). The `o~` is an it by 1.3.2;
        // parasmaipadī by 1.3.78. Enters the derivation as `hA`, exactly like
        // 03.0008 — so 7.4.76 declines on it by number (jahAti, not *jihAti),
        // and 6.4.116 / 6.4.117 / 6.4.118, which *jahāteḥ* names, fire on it by
        // number: jahitaH / jahItaH, jahAhi / jahihi / jahIhi, jahyAt. Slice 3c2.
        dhatupatha: "03.0009",
        code: "hA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "tyAge",
    },
    Dhatu {
        // 03.0010 `qudA\Y` dAne (√ḍudāñ). The ḍu is 1.3.5's; ubhayapadī by
        // 1.3.72 (final ñ). Ghu by 1.1.20 — decided by this number in
        // `derive`, because `02.0054 dA\p` is `dA` too and is not ghu — so
        // 6.4.113 skips it, 6.4.112 elides its ā before consonants as well
        // (dattaH, dadyAt), and 6.4.119 gives dehi. Slice 3c.
        dhatupatha: "03.0010",
        code: "dA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "dAne",
    },
    Dhatu {
        // 03.0011 `quDA\Y` DAraRapozaRayoH (√ḍudhāñ). Ghu like √dā. Its bared
        // `D` is what 8.2.40's *adhaḥ* excludes (DattaH, not *DadDaH) and what
        // 8.2.38 dadhas tathoś ca reads, re-aspirating the abhyāsa after
        // 8.4.54 (DattaH, Datse, DadDve). Both keyed by this number. Slice 3c.
        dhatupatha: "03.0011",
        code: "DA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "DAraRapozaRayoH",
    },
    Dhatu {
        // 03.0012 `Ri\ji~^r` SOcapozaRayoH (√ṇijir). Ubhayapadī by 1.3.72 (the
        // svarita `~^`). Stored `nij`: the `R` → `n` of 6.1.65 *ṇo naḥ* is the
        // stored-form convention, as for √nī. 7.4.60 trims the copy to `ni`,
        // and 7.4.75, keyed by this number, guṇates it: nenekti. 7.3.87 keeps
        // the root's `i` before the vowel-initial pit endings: nenijAni.
        // Slice 3e.
        dhatupatha: "03.0012",
        code: "nij",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "SOcapozaRayoH",
    },
    Dhatu {
        // 03.0013 `vi\ji~^r` pfTagBAve (√vijir). Ubhayapadī by 1.3.72. The
        // same path as √ṇij: vevekti, vevijAni. Its text `vij` is shared with
        // tudādi's 06.0009 and rudhādi's 07.0023, which is why 7.4.75 keys on
        // the number. Slice 3e.
        dhatupatha: "03.0013",
        code: "vij",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "pfTagBAve",
    },
    Dhatu {
        // 03.0014 `vi\zx~^` vyAptO (√viṣḷ). Ubhayapadī by 1.3.72. The same
        // abhyāsa path, with the root's `z` reaching the existing tripādī
        // rules: 8.4.41 (vevezwi), 8.2.41 then 8.3.59 (vevekzi), 8.4.41 then
        // 8.4.53 (veviqQi), 8.2.39 (aveveq). Slice 3e.
        dhatupatha: "03.0014",
        code: "viz",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Ubhayapada,
        artha: "vyAptO",
    },
    Dhatu {
        // 03.0015 `Gf\` kzaraRadIptyoH (√ghṛ). Parasmaipadī by 1.3.78. 7.4.66,
        // 7.4.60, then 7.4.62's gh → jh and 8.4.54's j: jagharti. Slice 3d.
        dhatupatha: "03.0015",
        code: "Gf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "kzaraRadIptyoH",
    },
    Dhatu {
        // 03.0016 `hf\` prasahyakaraRe (√hṛ). Parasmaipadī by 1.3.78. The same
        // path as √ghṛ, with 7.4.62's h → jh: jaharti. Slice 3d.
        dhatupatha: "03.0016",
        code: "hf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "prasahyakaraRe",
    },
    Dhatu {
        // 03.0017 `f\` gatO (√ṛ). Parasmaipadī by 1.3.78 (the `\` is the root
        // vowel's accent). The gaṇa's one vowel-initial ṛ-root: 7.4.66 gives
        // the abhyāsa `ar`, 7.4.60 trims it to `a` (no ādi hal to keep),
        // 7.4.77 makes it `i`, keyed by this number, and 6.4.78 makes that
        // `iy` before the root's vowel: iyarti. In laṅ 6.4.72's āṭ merges
        // into the abhyāsa by 6.1.90: EyaH. Slice 3d2.
        dhatupatha: "03.0017",
        code: "f",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatO",
    },
    Dhatu {
        // 03.0018 `sf\` gatO (√sṛ). Parasmaipadī by 1.3.78. 7.4.66 and 7.4.60
        // alone: sasarti. vidyut also credits 8.3.110 on sasrati, a bar on a
        // ṣatva that 8.3.59 cannot reach here (it retroflexes only an affix or
        // ādeśa `s` after the aṅga, never the root's own), so it is not
        // transcribed (see the 3d spec). Slice 3d.
        dhatupatha: "03.0018",
        code: "sf",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "gatO",
    },
    Dhatu {
        // 03.0019 `Basa~` BartsanadIptyoH (√bhasa). Parasmaipadī by 1.3.78.
        // 6.4.100 ghasibhasor hali ca elides the upadhā `a` before every
        // kṅit: 8.4.55 then devoices `Bs` to `ps` before a vowel or `y`
        // (bapsati, bapsyAt), and 8.2.26 jhalo jhali elides the `s` before a
        // `t`/`T` (babDaH). Its laṅ eka cells reach 8.2.73 (abaBat) and
        // 8.2.74 (abaBaH), the first outside rudhādi. Slice 3f2.
        dhatupatha: "03.0019",
        code: "Bas",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "BartsanadIptyoH",
    },
    Dhatu {
        // 03.0020 ki\ jYAne. ciketi / cikyati — the i-final witness for
        // 6.4.82 er anekāco'saṁyogapūrvasya (ci-ki is anekāc, k is no
        // conjunct) and the abhyāsa that 8.4.54 must leave alone (ci is
        // already car). Parasmaipadī by 1.3.78 as √hu. Slice 3a.
        dhatupatha: "03.0020",
        code: "ki",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "jYAne",
    },
    Dhatu {
        // 03.0021 `kita~` jYAne (√kita). Parasmaipadī by 1.3.78. 7.4.60 trims
        // the copy to `ki`, 7.4.62 makes it `ci`: ciketti. Laṅ madhyama eka
        // forks three ways, 8.2.75 daś ca taking the 8.2.39 `d` to ru
        // (acikeH beside aciked / aciket). 7.3.87 keeps the root's `i` before
        // the vowel-initial pit endings: cikitAni. Slice 3f.
        dhatupatha: "03.0021",
        code: "kit",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "jYAne",
    },
    Dhatu {
        // 03.0022 `tura~` tvaraRe (√tura). Parasmaipadī by 1.3.78. 7.3.86's
        // guṇa on the pit cells (tutorti); 8.2.77 hali ca lengthens the
        // upadhā before a consonant on the others (tutUrtaH, tutUrhi). Slice
        // 3f.
        dhatupatha: "03.0022",
        code: "tur",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "tvaraRe",
    },
    Dhatu {
        // 03.0023 `Diza~` Sabde (√dhiṣa). Parasmaipadī by 1.3.78. 8.4.54 makes
        // the abhyāsa `di`; the root's `z` reaches the existing tripādī rules
        // as √viṣ's does: 8.4.41 (diDezwi), 8.2.41 then 8.3.59 (diDekzi),
        // 8.4.41 then 8.4.53 (diDiqQi), 8.2.39 (adiDeq). Slice 3f.
        dhatupatha: "03.0023",
        code: "Diz",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "Sabde",
    },
    Dhatu {
        // 03.0024 `Dana~` DAnye (√dhana). Parasmaipadī by 1.3.78. 8.4.54 makes
        // the abhyāsa `da`. The root's `n` takes 8.3.24 before a jhal:
        // daDaMsi, daDaMhi; before `t`/`T` 8.4.58 restores it (daDantaH).
        // Slice 3f.
        dhatupatha: "03.0024",
        code: "Dan",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "DAnye",
    },
    Dhatu {
        // 03.0025 `jana~` janane (√jana). Parasmaipadī by 1.3.78. Before a
        // vowel-initial kṅit 6.4.98 elides its upadhā and 8.4.40 palatalizes
        // the `n` after the `j` (jajYati, ajajYuH); before a jhal-initial kṅit
        // 6.4.42 makes the `n` `ā` (jajAtaH, jajAhi), and before yāsuṭ 6.4.43
        // does so optionally (jajanyAt ~ jajAyAt). Slice 3f3, which closes the
        // gaṇa.
        dhatupatha: "03.0025",
        code: "jan",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "janane",
    },
    Dhatu {
        // 03.0026 `gA\` stutO (√gā). Parasmaipadī by 1.3.78 (the `\` is the
        // root vowel's accent). A Vedic root: its abhyāsa takes `i` by the
        // chāndasa 7.4.78, applied on the Kaumudī's authority (jigAti), keyed
        // by this number. Not ghu; 6.4.113 gives jigItaH. Slice 3c2.
        dhatupatha: "03.0026",
        code: "gA",
        gana: Gana::Juhotyadi,
        pada: PadaAssignment::Parasmaipada,
        artha: "stutO",
    },
    Dhatu {
        // 10.0001 `cura~` steye (√cur). Curādi's eponym: 3.1.25 adds ṇic,
        // 7.3.86 guṇates the laghu upadhā before it (cor-i), and 3.1.32 makes
        // `cori` the dhātu. Ubhayapadī by 1.3.74 ṇicaś ca. Slice 10a.
        dhatupatha: "10.0001",
        code: "cur",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "steye",
    },
    Dhatu {
        // 10.0010 `laqa~` upasevAyAm (√laḍ). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (lAq-i). Ubhayapadī by 1.3.74. Slice
        // 10a.
        dhatupatha: "10.0010",
        code: "laq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "upasevAyAm",
    },
    Dhatu {
        // 10.0033 `Bakza~` adane (√bhakṣ). Guru upadhā: neither 7.3.86 nor
        // 7.2.116 touches it before ṇic. Ubhayapadī by 1.3.74. Slice 10a.
        dhatupatha: "10.0033",
        code: "Bakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "adane",
    },
    Dhatu {
        // 10.0255 `BUza~` alaNkaraRe (√bhūṣ). Long upadhā, so guru: unchanged
        // before ṇic. Ubhayapadī by 1.3.74. Slice 10a.
        dhatupatha: "10.0255",
        code: "BUz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "alaNkaraRe",
    },
    Dhatu {
        // 10.0118 `jYapa~` jYAne jYApane ca (√jñap). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ
        // shortens it back (mit by the gaṇasūtra 10.0493). Ubhayapadī by
        // 1.3.74. Slice 10d.
        dhatupatha: "10.0118",
        code: "jYap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "jYAne jYApane ca",
    },
    Dhatu {
        // 10.0119 `yama~` parivezaRe (√yam). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it
        // back (mit by the gaṇasūtra 10.0493). Am-final, so 01.0934 would also
        // make it mit; this engine has no 01.0934 (it waits for a causative
        // slice), and 10.0493 alone gives *yamayati*. Ubhayapadī by 1.3.74.
        // Slice 10d.
        dhatupatha: "10.0119",
        code: "yam",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parivezaRe",
    },
    Dhatu {
        // 10.0120 `caha~` parikalkane (√cah). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it
        // back (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice
        // 10d.
        dhatupatha: "10.0120",
        code: "cah",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parikalkane",
    },
    Dhatu {
        // 10.0121 `capa~` parikalpane (√cap). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it
        // back (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice
        // 10d.
        dhatupatha: "10.0121",
        code: "cap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parikalpane",
    },
    Dhatu {
        // 10.0122 `raha~` tyAge (√rah). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it back
        // (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice 10d.
        dhatupatha: "10.0122",
        code: "rah",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "tyAge",
    },
    Dhatu {
        // 10.0123 `bala~` prARane (√bal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic and 6.4.92 mitāṃ hrasvaḥ shortens it back
        // (mit by the gaṇasūtra 10.0493). Ubhayapadī by 1.3.74. Slice 10d.
        dhatupatha: "10.0123",
        code: "bal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prARane",
    },
    Dhatu {
        // 10.0192 `cita~` saYcetane (√cit). The first ākusmīya row: 7.3.86
        // guṇates the laghu upadhā before ṇic (cet-i). Ātmanepadī by 10.0496
        // ā kusmād ātmanepadinaḥ. Slice 10b.
        dhatupatha: "10.0192",
        code: "cit",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "saYcetane",
    },
    Dhatu {
        // 10.0195 `dasa~` darSanadaMSanayoH (√das). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (dAs-i). Ātmanepadī by
        // 10.0496. Slice 10c.
        dhatupatha: "10.0195",
        code: "das",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "darSanadaMSanayoH",
    },
    Dhatu {
        // 10.0196 `qapa~` saNGAte (√ḍap). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (qAp-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0196",
        code: "qap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0197 `qipa~` saNGAte (√ḍip). 7.3.86 guṇates the laghu upadhā
        // before ṇic (qep-i). Homograph of the ubhayapadī curādi row `10.0189
        // qipa~`. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0197",
        code: "qip",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0200 `spaSa~` grahaRasaMSlezaRayoH (√spaś). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (spAS-i). Ātmanepadī by
        // 10.0496. Slice 10c.
        dhatupatha: "10.0200",
        code: "spaS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "grahaRasaMSlezaRayoH",
    },
    Dhatu {
        // 10.0201 `tarja~` tarjane (√tarj). Guru upadhā (the conjunct `rj`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0201",
        code: "tarj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "tarjane",
    },
    Dhatu {
        // 10.0202 `Bartsa~` tarjane (√bharts). Guru upadhā (the conjunct
        // `rts`), so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0202",
        code: "Barts",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "tarjane",
    },
    Dhatu {
        // 10.0203 `basta~` ardane (√bast). Guru upadhā (the conjunct `st`), so
        // unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0203",
        code: "bast",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ardane",
    },
    Dhatu {
        // 10.0204 `ganDa~` ardane (√gandh). Guru upadhā (the conjunct `nD`), so
        // unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c. Since slice 10e,
        // 8.3.24 → 8.4.58 is credited on its `n` (forms unchanged; vidyut credits
        // the pair too).
        dhatupatha: "10.0204",
        code: "ganD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ardane",
    },
    Dhatu {
        // 10.0205 `kila~` kzepe (√kil). 7.3.86 guṇates the laghu upadhā before
        // ṇic (kel-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0205",
        code: "kil",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0206 `pila~` kzepe (√pil). 7.3.86 guṇates the laghu upadhā before
        // ṇic (pel-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0206",
        code: "pil",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0207 `vizka~` hiMsAyAm (√viṣk). Guru upadhā (the conjunct `zk`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0207",
        code: "vizk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0208 `hizka~` hiMsAyAm (√hiṣk). Guru upadhā (the conjunct `zk`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0208",
        code: "hizk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0209 `nizka~` parimARe (√niṣk). Guru upadhā (the conjunct `zk`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0209",
        code: "nizk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "parimARe",
    },
    Dhatu {
        // 10.0210 `lala~` IpsAyAm (√lal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (lAl-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0210",
        code: "lal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "IpsAyAm",
    },
    Dhatu {
        // 10.0211 `kURa~` saNkoce (√kūṇ). Long upadhā vowel `U`, so unchanged
        // before ṇic. Homograph of the ubhayapadī curādi row `10.0438 kURa~`.
        // Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0211",
        code: "kUR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "saNkoce",
    },
    Dhatu {
        // 10.0212 `tURa~` pUraRe (√tūṇ). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0212",
        code: "tUR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "pUraRe",
    },
    Dhatu {
        // 10.0213 `BrURa~` ASAviSaNkayoH (√bhrūṇ). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0213",
        code: "BrUR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ASAviSaNkayoH",
    },
    Dhatu {
        // 10.0214 `SaWa~` SlAGAyAm (√śaṭh). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (SAW-i). Homograph of the ubhayapadī
        // curādi row `10.0041 SaWa~`. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0214",
        code: "SaW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "SlAGAyAm",
    },
    Dhatu {
        // 10.0215 `yakza~` pUjAyAm (√yakṣ). Guru upadhā (the conjunct `kz`),
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0215",
        code: "yakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0216 `syama~` vitarke (√syam). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (syAm-i). vidyut also credits the
        // gaṇasūtra 10.0494 nānye mito 'hetau here, in place of the am-final
        // mittva 01.0934; this engine has neither, and the forms agree. Not
        // jñapādi, so 10.0493 and 6.4.92 pass it by (slice 10d). A causative
        // slice inherits this row as a witness: once it adds 01.0934, this
        // golden fails unless 10.0494 comes with it. Ātmanepadī by 10.0496.
        // Slice 10c.
        dhatupatha: "10.0216",
        code: "syam",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "vitarke",
    },
    Dhatu {
        // 10.0217 `gUra~` udyamane (√gūr). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0217",
        code: "gUr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "udyamane",
    },
    Dhatu {
        // 10.0218 `Sama~` Alocane (√śam). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (SAm-i). vidyut also credits 10.0494 here,
        // as on `10.0216`; a witness for a causative slice. Ātmanepadī by
        // 10.0496. Slice 10c.
        dhatupatha: "10.0218",
        code: "Sam",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "Alocane",
    },
    Dhatu {
        // 10.0219 `lakza~` Alocane (√lakṣ). Guru upadhā (the conjunct `kz`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0006 lakza~`. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0219",
        code: "lakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "Alocane",
    },
    Dhatu {
        // 10.0220 `kutsa~` avakzepaRe nindane ca (√kuts). Guru upadhā (the
        // conjunct `ts`), so unchanged before ṇic. Ātmanepadī by 10.0496.
        // Slice 10c.
        dhatupatha: "10.0220",
        code: "kuts",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "avakzepaRe nindane ca",
    },
    Dhatu {
        // 10.0221 `truwa~` Cedane (√truṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (trow-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0221",
        code: "truw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0222 `kuwa~` Cedane (√kuṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (kow-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0222",
        code: "kuw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0223 `gala~` sravaRe (√gal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (gAl-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0223",
        code: "gal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "sravaRe",
    },
    Dhatu {
        // 10.0224 `Bala~` ABaRqane (√bal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (BAl-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0224",
        code: "Bal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ABaRqane",
    },
    Dhatu {
        // 10.0225 `kUwa~` ApradAne avasAdane ca (√kūṭ). Long upadhā vowel `U`,
        // so unchanged before ṇic. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0225",
        code: "kUw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "ApradAne avasAdane ca",
    },
    Dhatu {
        // 10.0226 `kuwwa~` pratApane (√kuṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0034 kuwwa~`. Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0226",
        code: "kuww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "pratApane",
    },
    Dhatu {
        // 10.0228 `vfza~` SaktibanDane (√vṛṣ). 7.3.86's ṛ arm before ṇic: the
        // laghu ṛ upadhā takes guṇa `ar` (varz-i). Ātmanepadī by 10.0496.
        // Slice 10b.
        dhatupatha: "10.0228",
        code: "vfz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "SaktibanDane",
    },
    Dhatu {
        // 10.0229 `mada~` tfptiyoge (√mad). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (mAd-i). Ātmanepadī by 10.0496. Slice
        // 10b.
        dhatupatha: "10.0229",
        code: "mad",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "tfptiyoge",
    },
    Dhatu {
        // 10.0232 `vida~` cetanAKyAnanivAsezu (√vid). 7.3.86 guṇates the laghu
        // upadhā before ṇic (ved-i). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0232",
        code: "vid",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "cetanAKyAnanivAsezu",
    },
    Dhatu {
        // 10.0233 `mAna~` stamBe (√mān). Long upadhā vowel `A`, so unchanged
        // before ṇic. Homograph of the ubhayapadī curādi row `10.0381 mAna~`;
        // and every form is also `10.0234`'s. Ātmanepadī by 10.0496. Slice
        // 10c.
        dhatupatha: "10.0233",
        code: "mAn",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "stamBe",
    },
    Dhatu {
        // 10.0234 `mana~` stamBe (√man). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (mAn-i). Every form is also `10.0233`'s
        // (mAnayate …). Ātmanepadī by 10.0496. Slice 10c.
        dhatupatha: "10.0234",
        code: "man",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "stamBe",
    },
    Dhatu {
        // 10.0236 `kusma~` kutsitasmaye (√kusm). The last ākusmīya row, the
        // one 10.0496 names: guru upadhā (the conjunct `sm`), so unchanged
        // before ṇic. Ātmanepadī by 10.0496. Slice 10b.
        dhatupatha: "10.0236",
        code: "kusm",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "kutsitasmaye",
    },
    Dhatu {
        // 10.0108 `mArga` saMskAragatyoH (√mārga). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0108",
        code: "mArga",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMskAragatyoH",
    },
    Dhatu {
        // 10.0389 `kaTa` vAkyaprabanDe (√katha). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0389",
        code: "kaTa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vAkyaprabanDe",
    },
    Dhatu {
        // 10.0390 `vara` IpsAyAm (√vara). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0390",
        code: "vara",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "IpsAyAm",
    },
    Dhatu {
        // 10.0391 `gaRa` saNKyAne (√gaṇa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0391",
        code: "gaRa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNKyAne",
    },
    Dhatu {
        // 10.0392 `SaWa` samyagavaBAzaRe (√śaṭha). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0392",
        code: "SaWa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samyagavaBAzaRe",
    },
    Dhatu {
        // 10.0393 `SvaWa` samyagavaBAzaRe (√śvaṭha). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0393",
        code: "SvaWa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samyagavaBAzaRe",
    },
    Dhatu {
        // 10.0394 `paWa` granTe vezwane ca (√paṭha). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0394",
        code: "paWa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "granTe vezwane ca",
    },
    Dhatu {
        // 10.0395 `vaWa` granTe (√vaṭha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0395",
        code: "vaWa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "granTe",
    },
    Dhatu {
        // 10.0396 `raha` tyAge (√raha). Adanta: 6.4.48 ato lopaḥ deletes the final
        // `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still stands
        // for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Shares
        // every form with the mit `10.0122 raha~` (slice 10d). Slice 10e.
        dhatupatha: "10.0396",
        code: "raha",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "tyAge",
    },
    Dhatu {
        // 10.0398 `stana` devaSabde (√stana). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0398",
        code: "stana",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "devaSabde",
    },
    Dhatu {
        // 10.0399 `gada` devaSabde (√gada). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0399",
        code: "gada",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "devaSabde",
    },
    Dhatu {
        // 10.0401 `paza` gatO (√paṣa). Adanta: 6.4.48 ato lopaḥ deletes the final
        // `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still stands
        // for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Slice 10e.
        dhatupatha: "10.0401",
        code: "paza",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO",
    },
    Dhatu {
        // 10.0402 `svara` Akzepe (√svara). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0402",
        code: "svara",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Akzepe",
    },
    Dhatu {
        // 10.0403 `raca` pratiyatne (√raca). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0403",
        code: "raca",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pratiyatne",
    },
    Dhatu {
        // 10.0404 `kala` gatO saNKyAne ca (√kala). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0404",
        code: "kala",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO saNKyAne ca",
    },
    Dhatu {
        // 10.0405 `caha` parikalkane (√caha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Shares every form with the mit `10.0120 caha~` (slice 10d). Slice 10e.
        dhatupatha: "10.0405",
        code: "caha",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parikalkane",
    },
    Dhatu {
        // 10.0406 `maha` pUjAyAm (√maha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0406",
        code: "maha",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0407 `sAra` dOrbalye (√sāra). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0407",
        code: "sAra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dOrbalye",
    },
    Dhatu {
        // 10.0408 `kfpa` dOrbalye (√kṛpa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0408",
        code: "kfpa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dOrbalye",
    },
    Dhatu {
        // 10.0409 `SraTa` dOrbalye (√śratha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0409",
        code: "SraTa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dOrbalye",
    },
    Dhatu {
        // 10.0410 `spfha` IpsAyAm (√spṛha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0410",
        code: "spfha",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "IpsAyAm",
    },
    Dhatu {
        // 10.0411 `BAma` kroDe (√bhāma). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0411",
        code: "BAma",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kroDe",
    },
    Dhatu {
        // 10.0412 `sUca` pESunye (√sūca). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0412",
        code: "sUca",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pESunye",
    },
    Dhatu {
        // 10.0413 `Kewa` BakzaRe (√kheṭa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0413",
        code: "Kewa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BakzaRe",
    },
    Dhatu {
        // 10.0415 `Kowa` BakzaRe (√khoṭa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0415",
        code: "Kowa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BakzaRe",
    },
    Dhatu {
        // 10.0416 `kzowa` kzepe (√kṣoṭa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0416",
        code: "kzowa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0417 `goma` upalepane (√goma). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0417",
        code: "goma",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "upalepane",
    },
    Dhatu {
        // 10.0418 `kumAra` krIqAyAm (√kumāra). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0418",
        code: "kumAra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "krIqAyAm",
    },
    Dhatu {
        // 10.0419 `SIla` upaDAraRe (√śīla). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0419",
        code: "SIla",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "upaDAraRe",
    },
    Dhatu {
        // 10.0420 `sAma` sAntvaprayoge (√sāma). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0420",
        code: "sAma",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sAntvaprayoge",
    },
    Dhatu {
        // 10.0421 `vela` kAlopadeSe (√vela). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0421",
        code: "vela",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kAlopadeSe",
    },
    Dhatu {
        // 10.0422 `kAla` kAlopadeSe (√kāla). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0422",
        code: "kAla",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kAlopadeSe",
    },
    Dhatu {
        // 10.0423 `palpUla` lavanavapanayoH (√palpūla). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0423",
        code: "palpUla",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "lavanavapanayoH",
    },
    Dhatu {
        // 10.0424 `vAta` suKasevanayoH (√vāta). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0424",
        code: "vAta",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "suKasevanayoH",
    },
    Dhatu {
        // 10.0425 `gaveza` mArgaRe (√gaveṣa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0425",
        code: "gaveza",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mArgaRe",
    },
    Dhatu {
        // 10.0426 `vAsa` upasevAyAm (√vāsa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0426",
        code: "vAsa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "upasevAyAm",
    },
    Dhatu {
        // 10.0427 `nivAsa` AcCAdane (√nivāsa). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0427",
        code: "nivAsa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AcCAdane",
    },
    Dhatu {
        // 10.0428 `BAja` pfTakkarmaRi (√bhāja). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0428",
        code: "BAja",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pfTakkarmaRi",
    },
    Dhatu {
        // 10.0429 `saBAja` prItidarSanayoH prItisevanayoH ca (√sabhāja). Adanta:
        // 6.4.48 ato lopaḥ deletes the final `a` before ārdhadhātuka ṇic, and by
        // 1.1.57 the deleted `a` still stands for 7.2.116 and 7.3.86, which
        // decline. Ubhayapadī by 1.3.74. Slice 10e.
        dhatupatha: "10.0429",
        code: "saBAja",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prItidarSanayoH prItisevanayoH ca",
    },
    Dhatu {
        // 10.0430 `Una` parihARe (√ūna). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0430",
        code: "Una",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parihARe",
    },
    Dhatu {
        // 10.0431 `Dvana` Sabde (√dhvana). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0431",
        code: "Dvana",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Sabde",
    },
    Dhatu {
        // 10.0432 `kUwa` paritApe paridAhe ca (√kūṭa). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Its ātmanepada shares every form with the ākusmīya `10.0225
        // kUwa~`. Slice 10e.
        dhatupatha: "10.0432",
        code: "kUwa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "paritApe paridAhe ca",
    },
    Dhatu {
        // 10.0433 `sanketa` AmantraRe (√sanketa). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Since
        // slice 10e, 8.3.24 → 8.4.58 is credited on its `n`.
        dhatupatha: "10.0433",
        code: "sanketa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AmantraRe",
    },
    Dhatu {
        // 10.0434 `grAma` AmantraRe (√grāma). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0434",
        code: "grAma",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AmantraRe",
    },
    Dhatu {
        // 10.0435 `kuRa` AmantraRe (√kuṇa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0435",
        code: "kuRa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AmantraRe",
    },
    Dhatu {
        // 10.0436 `guRa` AmantraRe (√guṇa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0436",
        code: "guRa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AmantraRe",
    },
    Dhatu {
        // 10.0437 `keta` SrAvaRe AmantraRe ca (√keta). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0437",
        code: "keta",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SrAvaRe AmantraRe ca",
    },
    Dhatu {
        // 10.0439 `stena` cOrye (√stena). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0439",
        code: "stena",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "cOrye",
    },
    Dhatu {
        // 10.0440 `pada` gatO (√pada). Adanta: 6.4.48 ato lopaḥ deletes the final
        // `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still stands
        // for 7.2.116 and 7.3.86, which decline. Ātmanepadī by the gaṇasūtra
        // 10.0497. Slice 10e.
        dhatupatha: "10.0440",
        code: "pada",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "gatO",
    },
    Dhatu {
        // 10.0441 `gfha` grahaRe (√gṛha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ātmanepadī by the
        // gaṇasūtra 10.0497. Slice 10e.
        dhatupatha: "10.0441",
        code: "gfha",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "grahaRe",
    },
    Dhatu {
        // 10.0442 `mfga` anvezaRe (√mṛga). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ātmanepadī by the
        // gaṇasūtra 10.0497. Slice 10e.
        dhatupatha: "10.0442",
        code: "mfga",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "anvezaRe",
    },
    Dhatu {
        // 10.0443 `kuha` vismApane (√kuha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ātmanepadī by the
        // gaṇasūtra 10.0497. Slice 10e.
        dhatupatha: "10.0443",
        code: "kuha",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "vismApane",
    },
    Dhatu {
        // 10.0444 `SUra` vikrAntO (√śūra). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ātmanepadī by the
        // gaṇasūtra 10.0497. Slice 10e.
        dhatupatha: "10.0444",
        code: "SUra",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "vikrAntO",
    },
    Dhatu {
        // 10.0445 `vIra` vikrAntO (√vīra). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ātmanepadī by the
        // gaṇasūtra 10.0497. Slice 10e.
        dhatupatha: "10.0445",
        code: "vIra",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "vikrAntO",
    },
    Dhatu {
        // 10.0446 `sTUla` paribfhaRe (√sthūla). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ātmanepadī by the
        // gaṇasūtra 10.0497. Slice 10e.
        dhatupatha: "10.0446",
        code: "sTUla",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "paribfhaRe",
    },
    Dhatu {
        // 10.0447 `arTa` upayAcYAyAm (√artha). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ātmanepadī by the
        // gaṇasūtra 10.0497. Slice 10e.
        dhatupatha: "10.0447",
        code: "arTa",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "upayAcYAyAm",
    },
    Dhatu {
        // 10.0448 `satra` santAnakriyAyAm (√satra). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ātmanepadī by
        // the gaṇasūtra 10.0497. Slice 10e.
        dhatupatha: "10.0448",
        code: "satra",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "santAnakriyAyAm",
    },
    Dhatu {
        // 10.0450 `sUtra` vezwane vimocane granTane ca (√sūtra). Adanta: 6.4.48
        // ato lopaḥ deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57
        // the deleted `a` still stands for 7.2.116 and 7.3.86, which decline.
        // Ubhayapadī by 1.3.74. Slice 10e.
        dhatupatha: "10.0450",
        code: "sUtra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vezwane vimocane granTane ca",
    },
    Dhatu {
        // 10.0452 `rUkza` pAruzye (√rūkṣa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0452",
        code: "rUkza",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pAruzye",
    },
    Dhatu {
        // 10.0453 `pAra` karmasamAptO (√pāra). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0453",
        code: "pAra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "karmasamAptO",
    },
    Dhatu {
        // 10.0454 `tIra` karmasamAptO (√tīra). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0454",
        code: "tIra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "karmasamAptO",
    },
    Dhatu {
        // 10.0455 `puwa` saMsarge (√puṭa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0455",
        code: "puwa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMsarge",
    },
    Dhatu {
        // 10.0458 `valka` darSane (√valka). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0458",
        code: "valka",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "darSane",
    },
    Dhatu {
        // 10.0459 `citra` citrIkaraRe (√citra). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0459",
        code: "citra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "citrIkaraRe",
    },
    Dhatu {
        // 10.0460 `ansa` samAGAte (√ansa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Since
        // slice 10e, 8.3.24 → 8.4.58 is credited on its `n`.
        dhatupatha: "10.0460",
        code: "ansa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samAGAte",
    },
    Dhatu {
        // 10.0461 `vawa` viBAjane (√vaṭa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0461",
        code: "vawa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "viBAjane",
    },
    Dhatu {
        // 10.0463 `laja` prakASane (√laja). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0463",
        code: "laja",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakASane",
    },
    Dhatu {
        // 10.0466 `miSra` samparke (√miśra). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0466",
        code: "miSra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samparke",
    },
    Dhatu {
        // 10.0467 `sangrAma` yudDe (√sangrāma). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Since
        // slice 10e, 8.3.24 → 8.4.58 is credited on its `n`.
        dhatupatha: "10.0467",
        code: "sangrAma",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "yudDe",
    },
    Dhatu {
        // 10.0468 `stoma` SlAGAyAm (√stoma). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0468",
        code: "stoma",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SlAGAyAm",
    },
    Dhatu {
        // 10.0469 `Cidra` karRaBedane (√chidra). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0469",
        code: "Cidra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "karRaBedane",
    },
    Dhatu {
        // 10.0471 `anDa` dfzwyupaGAte (√andha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Since
        // slice 10e, 8.3.24 → 8.4.58 is credited on its `n`.
        dhatupatha: "10.0471",
        code: "anDa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dfzwyupaGAte",
    },
    Dhatu {
        // 10.0472 `danqa` daRqanipAte (√danḍa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Since
        // slice 10e, 8.3.24 → 8.4.58 is credited on its `n`.
        dhatupatha: "10.0472",
        code: "danqa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "daRqanipAte",
    },
    Dhatu {
        // 10.0473 `anka` pade lakzaRe ca (√anka). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Since
        // slice 10e, 8.3.24 → 8.4.58 is credited on its `n`.
        dhatupatha: "10.0473",
        code: "anka",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pade lakzaRe ca",
    },
    Dhatu {
        // 10.0474 `anga` pade lakzaRe ca (√anga). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Since
        // slice 10e, 8.3.24 → 8.4.58 is credited on its `n`.
        dhatupatha: "10.0474",
        code: "anga",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pade lakzaRe ca",
    },
    Dhatu {
        // 10.0475 `suKa` tatkriyAyAm (√sukha). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0475",
        code: "suKa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "tatkriyAyAm",
    },
    Dhatu {
        // 10.0476 `duHKa` tatkriyAyAm (√duḥkha). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0476",
        code: "duHKa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "tatkriyAyAm",
    },
    Dhatu {
        // 10.0477 `rasa` AsvAdanasnehanayoH (√rasa). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0477",
        code: "rasa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdanasnehanayoH",
    },
    Dhatu {
        // 10.0478 `vyaya` vittasamutsarge (√vyaya). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0478",
        code: "vyaya",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vittasamutsarge",
    },
    Dhatu {
        // 10.0479 `rUpa` rUpakriyAyAm (√rūpa). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0479",
        code: "rUpa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rUpakriyAyAm",
    },
    Dhatu {
        // 10.0480 `Ceda` dvEDIkaraRe (√cheda). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0480",
        code: "Ceda",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dvEDIkaraRe",
    },
    Dhatu {
        // 10.0481 `Cada` apavAraRe (√chada). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0481",
        code: "Cada",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "apavAraRe",
    },
    Dhatu {
        // 10.0482 `lABa` preraRe (√lābha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0482",
        code: "lABa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe",
    },
    Dhatu {
        // 10.0483 `vraRa` gAtravicUrRane (√vraṇa). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0483",
        code: "vraRa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gAtravicUrRane",
    },
    Dhatu {
        // 10.0484 `varRa` varRakriyAvistAraguRavacanezu (√varṇa). Adanta: 6.4.48
        // ato lopaḥ deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57
        // the deleted `a` still stands for 7.2.116 and 7.3.86, which decline.
        // Ubhayapadī by 1.3.74. Slice 10e.
        dhatupatha: "10.0484",
        code: "varRa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "varRakriyAvistAraguRavacanezu",
    },
    Dhatu {
        // 10.0485 `parRa` haritaBAve (√parṇa). Adanta: 6.4.48 ato lopaḥ deletes
        // the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a`
        // still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0485",
        code: "parRa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "haritaBAve",
    },
    Dhatu {
        // 10.0486 `vizka` darSane (√viṣka). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74. Its
        // ātmanepada shares every form with the ākusmīya `10.0207 vizka~`. Slice
        // 10e.
        dhatupatha: "10.0486",
        code: "vizka",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "darSane",
    },
    Dhatu {
        // 10.0487 `kzipa` preraRe (√kṣipa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0487",
        code: "kzipa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe",
    },
    Dhatu {
        // 10.0488 `vasa` nivAse (√vasa). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0488",
        code: "vasa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nivAse",
    },
    Dhatu {
        // 10.0489 `tutTa` AvaraRe (√tuttha). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0489",
        code: "tutTa",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AvaraRe",
    },
    Dhatu {
        // 10.0490 `palyUla` lavanavapanayoH (√palyūla). Adanta: 6.4.48 ato lopaḥ
        // deletes the final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted
        // `a` still stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by
        // 1.3.74. Slice 10e.
        dhatupatha: "10.0490",
        code: "palyUla",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "lavanavapanayoH",
    },
    Dhatu {
        // 10.0492 `Deka` darSane (√dheka). Adanta: 6.4.48 ato lopaḥ deletes the
        // final `a` before ārdhadhātuka ṇic, and by 1.1.57 the deleted `a` still
        // stands for 7.2.116 and 7.3.86, which decline. Ubhayapadī by 1.3.74.
        // Slice 10e.
        dhatupatha: "10.0492",
        code: "Deka",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "darSane",
    },
    Dhatu {
        // 10.0193 `daSi~` daMSane (√daṃś). Idit, so its ṇic is optional by Kaumudī
        // 2564 (7.1.58's num stored, as for `hins`). Ātmanepadī by the gaṇasūtra
        // 10.0496 on its ṇic branch (*daṃśayate*); parasmaipadī by 1.3.78 on its
        // ṇic-less branch (*daṃśati*). Slice 10f.
        dhatupatha: "10.0193",
        code: "danS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "daMSane",
    },
    Dhatu {
        // 10.0194 `dasi~` darSanadaMSanayoH (√daṃs). Idit: ṇic optional by Kaumudī
        // 2564. Ātmanepadī by 10.0496 with ṇic (*daṃsayate*), parasmaipadī by
        // 1.3.78 without (*daṃsati*). Slice 10f.
        dhatupatha: "10.0194",
        code: "dans",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "darSanadaMSanayoH",
    },
    Dhatu {
        // 10.0198 `tatri~` kuwumbaDAraRe (√tantr). Idit: ṇic optional by Kaumudī
        // 2564. Ātmanepadī by 10.0496 with ṇic (*tantrayate*), parasmaipadī by
        // 1.3.78 without (*tantrati*). Slice 10f.
        dhatupatha: "10.0198",
        code: "tantr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "kuwumbaDAraRe",
    },
    Dhatu {
        // 10.0199 `matri~` guptapariBAzaRe (√mantr). Idit: ṇic optional by Kaumudī
        // 2564. Ātmanepadī by 10.0496 with ṇic (*mantrayate*), parasmaipadī by
        // 1.3.78 without (*mantrati*). Slice 10f.
        dhatupatha: "10.0199",
        code: "mantr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "guptapariBAzaRe",
    },
    Dhatu {
        // 10.0227 `vancu~` pralamBane (√vañc). Udit: ṇic optional by Kaumudī 2570.
        // Ātmanepadī by 10.0496 with ṇic (*vañcayate*), parasmaipadī by 1.3.78
        // without (*vañcati*). vidyut also runs a text-free 7.3.63 here, which
        // this engine does not model. Slice 10f.
        dhatupatha: "10.0227",
        code: "vanc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "pralamBane",
    },
    Dhatu {
        // 10.0230 `divu~` parikUjane (√div). Udit: ṇic optional by Kaumudī 2570.
        // Ātmanepadī by 10.0496 with ṇic (*devayate*), parasmaipadī by 1.3.78
        // without (*devati*, 7.3.86's guṇa before śap). Slice 10f.
        dhatupatha: "10.0230",
        code: "div",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "parikUjane",
    },
    Dhatu {
        // 10.0400 `pata` gatO (√pat). Adanta, with ṇic optional by Kaumudī 2573.1
        // (*patati*). On the ṇic branch 2573.2 optionally deletes the final `a`
        // first, so 7.2.116 lengthens (*pātayati*); otherwise 6.4.48 deletes it
        // and 7.2.116 declines (*patayati*). Ubhayapadī by 1.3.74 with ṇic,
        // parasmaipadī by 1.3.78 without. Slice 10f.
        dhatupatha: "10.0400",
        code: "pata",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO",
    },
    Dhatu {
        // 10.0449 `garva` mAne (√garv). Adanta and ā-garvīya, with ṇic optional by
        // Kaumudī 2573.3. Ātmanepadī by 10.0497 with ṇic (*garvayate*),
        // parasmaipadī by 1.3.78 without (*garvati*). Slice 10f.
        dhatupatha: "10.0449",
        code: "garva",
        gana: Gana::Curadi,
        pada: PadaAssignment::AaGarviya,
        artha: "mAne",
    },
    Dhatu {
        // 10.0451 `mUtra` prasravaRe (√mūtr). Adanta, with ṇic optional by Kaumudī
        // 2573.3. Ubhayapadī by 1.3.74 with ṇic (*mūtrayati*), parasmaipadī by
        // 1.3.78 without (*mūtrati*). Slice 10f.
        dhatupatha: "10.0451",
        code: "mUtra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prasravaRe",
    },
    Dhatu {
        // 10.0456 `katra` SETilye (√katr). Adanta, with ṇic optional by Kaumudī
        // 2573.3. Ubhayapadī by 1.3.74 with ṇic (*katrayati*), parasmaipadī by
        // 1.3.78 without (*katrati*). Slice 10f.
        dhatupatha: "10.0456",
        code: "katra",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SETilye",
    },
    Dhatu {
        // 10.0002 `citi~` smftyAm (√cint). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*cintayati*),
        // parasmaipadī by 1.3.78 without (*cintati*). Slice 10g.
        dhatupatha: "10.0002",
        code: "cint",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "smftyAm",
    },
    Dhatu {
        // 10.0003 `yatri~` saNkoce (√yantr). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*yantrayati*),
        // parasmaipadī by 1.3.78 without (*yantrati*). Slice 10g.
        dhatupatha: "10.0003",
        code: "yantr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNkoce",
    },
    Dhatu {
        // 10.0004 `sPuqi~` parihAse (√sphuṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*sphuṇḍayati*),
        // parasmaipadī by 1.3.78 without (*sphuṇḍati*). Slice 10g.
        dhatupatha: "10.0004",
        code: "sPunq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parihAse",
    },
    Dhatu {
        // 10.0005 `sPuwi~` parihAse (√sphuṇṭ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*sphuṇṭayati*),
        // parasmaipadī by 1.3.78 without (*sphuṇṭati*). Slice 10g.
        dhatupatha: "10.0005",
        code: "sPunw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parihAse",
    },
    Dhatu {
        // 10.0007 `kudri~` anftaBAzaRe (√kundr). Idit: ṇic optional by Kaumudī
        // 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*kundrayati*), parasmaipadī by 1.3.78 without (*kundrati*). Slice 10g.
        dhatupatha: "10.0007",
        code: "kundr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anftaBAzaRe",
    },
    Dhatu {
        // 10.0009 `spuqi~` parihAse (√spuṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*spuṇḍayati*),
        // parasmaipadī by 1.3.78 without (*spuṇḍati*). Slice 10g.
        dhatupatha: "10.0009",
        code: "spunq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parihAse",
    },
    Dhatu {
        // 10.0011 `midi~` snehane (√mind). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*mindayati*),
        // parasmaipadī by 1.3.78 without (*mindati*). Slice 10g.
        dhatupatha: "10.0011",
        code: "mind",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehane",
    },
    Dhatu {
        // 10.0013 `o~laqi~` utkzepaRe (√laṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*laṇḍayati*),
        // parasmaipadī by 1.3.78 without (*laṇḍati*). Slice 10g.
        dhatupatha: "10.0013",
        code: "lanq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "utkzepaRe",
    },
    Dhatu {
        // 10.0014 `olaqi~` utkzepaRe (√olaṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*olaṇḍayati*),
        // parasmaipadī by 1.3.78 without (*olaṇḍati*). Slice 10g.
        dhatupatha: "10.0014",
        code: "olanq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "utkzepaRe",
    },
    Dhatu {
        // 10.0043 `SvaWi~` asaMskAragatyoH (√śvaṇṭh). Idit: ṇic optional by
        // Kaumudī 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*śvaṇṭhayati*), parasmaipadī by 1.3.78 without (*śvaṇṭhati*). Slice
        // 10g.
        dhatupatha: "10.0043",
        code: "SvanW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "asaMskAragatyoH",
    },
    Dhatu {
        // 10.0045 `tuji~` hiMsAbalAdAnaniketanezu (√tuñj). Idit: ṇic optional by
        // Kaumudī 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*tuñjayati*), parasmaipadī by 1.3.78 without (*tuñjati*). Slice 10g.
        dhatupatha: "10.0045",
        code: "tunj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAbalAdAnaniketanezu",
    },
    Dhatu {
        // 10.0047 `piji~` hiMsAbalAdAnaniketanezu (√piñj). Idit: ṇic optional by
        // Kaumudī 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*piñjayati*), parasmaipadī by 1.3.78 without (*piñjati*). Slice 10g.
        dhatupatha: "10.0047",
        code: "pinj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAbalAdAnaniketanezu",
    },
    Dhatu {
        // 10.0048 `laji~` hiMsAbalAdAnaniketanezu (√lañj). Idit: ṇic optional by
        // Kaumudī 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*lañjayati*), parasmaipadī by 1.3.78 without (*lañjati*). Slice 10g.
        dhatupatha: "10.0048",
        code: "lanj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAbalAdAnaniketanezu",
    },
    Dhatu {
        // 10.0049 `luji~` hiMsAbalAdAnaniketanezu (√luñj). Idit: ṇic optional by
        // Kaumudī 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*luñjayati*), parasmaipadī by 1.3.78 without (*luñjati*). Slice 10g.
        dhatupatha: "10.0049",
        code: "lunj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAbalAdAnaniketanezu",
    },
    Dhatu {
        // 10.0060 `paTi~` gatO (√panth). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*panthayati*),
        // parasmaipadī by 1.3.78 without (*panthati*). Slice 10g.
        dhatupatha: "10.0060",
        code: "panT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO",
    },
    Dhatu {
        // 10.0062 `Cadi~` saMvaraRe (√chand). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*chandayati*),
        // parasmaipadī by 1.3.78 without (*chandati*). Slice 10g.
        dhatupatha: "10.0062",
        code: "Cand",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMvaraRe",
    },
    Dhatu {
        // 10.0066 `Kaqi~` Bedane (√khaṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*khaṇḍayati*),
        // parasmaipadī by 1.3.78 without (*khaṇḍati*). Slice 10g.
        dhatupatha: "10.0066",
        code: "Kanq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0067 `kaqi~` Bedane (√kaṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*kaṇḍayati*),
        // parasmaipadī by 1.3.78 without (*kaṇḍati*). Slice 10g.
        dhatupatha: "10.0067",
        code: "kanq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0068 `kuqi~` rakzaRe (√kuṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*kuṇḍayati*),
        // parasmaipadī by 1.3.78 without (*kuṇḍati*). Slice 10g.
        dhatupatha: "10.0068",
        code: "kunq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe",
    },
    Dhatu {
        // 10.0069 `guqi~` vezwane (√guṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*guṇḍayati*),
        // parasmaipadī by 1.3.78 without (*guṇḍati*). Slice 10g.
        dhatupatha: "10.0069",
        code: "gunq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vezwane",
    },
    Dhatu {
        // 10.0070 `kuWi~` rakzaRe (√kuṇṭh). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*kuṇṭhayati*),
        // parasmaipadī by 1.3.78 without (*kuṇṭhati*). Slice 10g.
        dhatupatha: "10.0070",
        code: "kunW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe",
    },
    Dhatu {
        // 10.0071 `guWi~` rakzaRe (√guṇṭh). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*guṇṭhayati*),
        // parasmaipadī by 1.3.78 without (*guṇṭhati*). Slice 10g.
        dhatupatha: "10.0071",
        code: "gunW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe",
    },
    Dhatu {
        // 10.0072 `Kuqi~` KaRqane (√khuṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*khuṇḍayati*),
        // parasmaipadī by 1.3.78 without (*khuṇḍati*). Slice 10g.
        dhatupatha: "10.0072",
        code: "Kunq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "KaRqane",
    },
    Dhatu {
        // 10.0073 `vawi~` viBAjane (√vaṇṭ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*vaṇṭayati*),
        // parasmaipadī by 1.3.78 without (*vaṇṭati*). Slice 10g.
        dhatupatha: "10.0073",
        code: "vanw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "viBAjane",
    },
    Dhatu {
        // 10.0074 `vaqi~` viBAjane (√vaṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*vaṇḍayati*),
        // parasmaipadī by 1.3.78 without (*vaṇḍati*). Slice 10g.
        dhatupatha: "10.0074",
        code: "vanq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "viBAjane",
    },
    Dhatu {
        // 10.0075 `caqi~` kope (√caṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*caṇḍayati*),
        // parasmaipadī by 1.3.78 without (*caṇḍati*). Slice 10g.
        dhatupatha: "10.0075",
        code: "canq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kope",
    },
    Dhatu {
        // 10.0076 `maqi~` BUzAyAM harze ca (√maṇḍ). Idit: ṇic optional by Kaumudī
        // 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*maṇḍayati*),
        // parasmaipadī by 1.3.78 without (*maṇḍati*). Slice 10g.
        dhatupatha: "10.0076",
        code: "manq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BUzAyAM harze ca",
    },
    Dhatu {
        // 10.0077 `Baqi~` kalyARe (√bhaṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*bhaṇḍayati*),
        // parasmaipadī by 1.3.78 without (*bhaṇḍati*). Slice 10g.
        dhatupatha: "10.0077",
        code: "Banq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kalyARe",
    },
    Dhatu {
        // 10.0105 `ulaqi~` utkzepaRe (√ulaṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*ulaṇḍayati*),
        // parasmaipadī by 1.3.78 without (*ulaṇḍati*). Slice 10g.
        dhatupatha: "10.0105",
        code: "ulanq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "utkzepaRe",
    },
    Dhatu {
        // 10.0106 `paqi~` nASane (√paṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*paṇḍayati*),
        // parasmaipadī by 1.3.78 without (*paṇḍati*). Slice 10g.
        dhatupatha: "10.0106",
        code: "panq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nASane",
    },
    Dhatu {
        // 10.0107 `pasi~` nASane (√paṃs). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*paṃsayati*),
        // parasmaipadī by 1.3.78 without (*paṃsati*). Slice 10g.
        dhatupatha: "10.0107",
        code: "pans",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nASane",
    },
    Dhatu {
        // 10.0111 `capi~` gatyAm (√camp). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*campayati*),
        // parasmaipadī by 1.3.78 without (*campati*). Slice 10g.
        dhatupatha: "10.0111",
        code: "canp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatyAm",
    },
    Dhatu {
        // 10.0112 `kzapi~` kzAntyAm (√kṣamp). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*kṣampayati*),
        // parasmaipadī by 1.3.78 without (*kṣampati*). Slice 10g.
        dhatupatha: "10.0112",
        code: "kzanp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzAntyAm",
    },
    Dhatu {
        // 10.0113 `kzaji~` kfcCrajIvane (√kṣañj). Idit: ṇic optional by Kaumudī
        // 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*kṣañjayati*), parasmaipadī by 1.3.78 without (*kṣañjati*). Slice 10g.
        dhatupatha: "10.0113",
        code: "kzanj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kfcCrajIvane",
    },
    Dhatu {
        // 10.0114 `Caji~` kfcCrajIvane (√chañj). Idit: ṇic optional by Kaumudī
        // 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*chañjayati*), parasmaipadī by 1.3.78 without (*chañjati*). Slice 10g.
        dhatupatha: "10.0114",
        code: "Canj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kfcCrajIvane",
    },
    Dhatu {
        // 10.0130 `cubi~` hiMsAyAm (√cumb). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*cumbayati*),
        // parasmaipadī by 1.3.78 without (*cumbati*). Slice 10g.
        dhatupatha: "10.0130",
        code: "cunb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0135 `waki~` banDane (√ṭaṅk). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*ṭaṅkayati*),
        // parasmaipadī by 1.3.78 without (*ṭaṅkati*). Slice 10g.
        dhatupatha: "10.0135",
        code: "wank",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "banDane",
    },
    Dhatu {
        // 10.0147 `SuWi~` SozaRe (√śuṇṭh). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*śuṇṭhayati*),
        // parasmaipadī by 1.3.78 without (*śuṇṭhati*). Slice 10g.
        dhatupatha: "10.0147",
        code: "SunW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SozaRe",
    },
    Dhatu {
        // 10.0153 `paci~` vistAravacane (√pañc). Idit: ṇic optional by Kaumudī
        // 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*pañcayati*),
        // parasmaipadī by 1.3.78 without (*pañcati*). Slice 10g.
        dhatupatha: "10.0153",
        code: "panc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vistAravacane",
    },
    Dhatu {
        // 10.0157 `kubi~` AcCAdane (√kumb). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*kumbayati*),
        // parasmaipadī by 1.3.78 without (*kumbati*). Slice 10g.
        dhatupatha: "10.0157",
        code: "kunb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AcCAdane",
    },
    Dhatu {
        // 10.0158 `kuBi~` AcCAdane (√kumbh). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*kumbhayati*),
        // parasmaipadī by 1.3.78 without (*kumbhati*). Slice 10g.
        dhatupatha: "10.0158",
        code: "kunB",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AcCAdane",
    },
    Dhatu {
        // 10.0159 `lubi~` adarSane (√lumb). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*lumbayati*),
        // parasmaipadī by 1.3.78 without (*lumbati*). Slice 10g.
        dhatupatha: "10.0159",
        code: "lunb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "adarSane",
    },
    Dhatu {
        // 10.0160 `tubi~` adarSane (√tumb). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*tumbayati*),
        // parasmaipadī by 1.3.78 without (*tumbati*). Slice 10g.
        dhatupatha: "10.0160",
        code: "tunb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "adarSane",
    },
    Dhatu {
        // 10.0164 `cuwi~` Cedane (√cuṇṭ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*cuṇṭayati*),
        // parasmaipadī by 1.3.78 without (*cuṇṭati*). Slice 10g.
        dhatupatha: "10.0164",
        code: "cunw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0166 `dahi~` rakzaRe mokzaRe ca (√daṃh). Idit: ṇic optional by
        // Kaumudī 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*daṃhayati*), parasmaipadī by 1.3.78 without (*daṃhati*). Slice 10g.
        dhatupatha: "10.0166",
        code: "danh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe mokzaRe ca",
    },
    Dhatu {
        // 10.0171 `Capi~` gatyAm (√champ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*champayati*),
        // parasmaipadī by 1.3.78 without (*champati*). Slice 10g.
        dhatupatha: "10.0171",
        code: "Canp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatyAm",
    },
    Dhatu {
        // 10.0174 `SraRu~` dAne (√śraṇ). Udit: ṇic optional by Kaumudī 2570.
        // Ubhayapadī by 1.3.74 with ṇic (*śrāṇayati*), parasmaipadī by 1.3.78
        // without (*śraṇati*). Slice 10g.
        dhatupatha: "10.0174",
        code: "SraR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dAne",
    },
    Dhatu {
        // 10.0182 `jasi~` rakzaRe mokzaRe ca (√jaṃs). Idit: ṇic optional by
        // Kaumudī 2564 (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic
        // (*jaṃsayati*), parasmaipadī by 1.3.78 without (*jaṃsati*). Slice 10g.
        dhatupatha: "10.0182",
        code: "jans",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe mokzaRe ca",
    },
    Dhatu {
        // 10.0184 `jasu~` hiMsAyAm (√jas). Udit: ṇic optional by Kaumudī 2570.
        // Ubhayapadī by 1.3.74 with ṇic (*jāsayati*), parasmaipadī by 1.3.78
        // without (*jasati*). Slice 10g.
        dhatupatha: "10.0184",
        code: "jas",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0185 `piqi~` saNGAte (√piṇḍ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*piṇḍayati*),
        // parasmaipadī by 1.3.78 without (*piṇḍati*). Slice 10g.
        dhatupatha: "10.0185",
        code: "pinq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0241 `jaBi~` nASane (√jambh). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*jambhayati*),
        // parasmaipadī by 1.3.78 without (*jambhati*). Slice 10g.
        dhatupatha: "10.0241",
        code: "janB",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nASane",
    },
    Dhatu {
        // 10.0243 `jasu~` tAqane (√jas). Udit: ṇic optional by Kaumudī 2570.
        // Ubhayapadī by 1.3.74 with ṇic (*jāsayati*), parasmaipadī by 1.3.78
        // without (*jasati*). Slice 10g.
        dhatupatha: "10.0243",
        code: "jas",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "tAqane",
    },
    Dhatu {
        // 10.0249 `divu~` mardane (√div). Udit: ṇic optional by Kaumudī 2570.
        // Ubhayapadī by 1.3.74 with ṇic (*devayati*), parasmaipadī by 1.3.78
        // without (*devati*). Slice 10g.
        dhatupatha: "10.0249",
        code: "div",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mardane",
    },
    Dhatu {
        // 10.0254 `tasi~` alaNkaraRe (√taṃs). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*taṃsayati*),
        // parasmaipadī by 1.3.78 without (*taṃsati*). Slice 10g.
        dhatupatha: "10.0254",
        code: "tans",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "alaNkaraRe",
    },
    Dhatu {
        // 10.0260 `SfDu~` prahasane prasahane ca (√śṛdh). Udit: ṇic optional by
        // Kaumudī 2570. Ubhayapadī by 1.3.74 with ṇic (*śardhayati*), parasmaipadī
        // by 1.3.78 without (*śardhati*). Slice 10g.
        dhatupatha: "10.0260",
        code: "SfD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prahasane prasahane ca",
    },
    Dhatu {
        // 10.0266 `ancu~` viSezaRe (√añc). Udit: ṇic optional by Kaumudī 2570.
        // Ubhayapadī by 1.3.74 with ṇic (*añcayati*), parasmaipadī by 1.3.78
        // without (*añcati*). Slice 10g.
        dhatupatha: "10.0266",
        code: "anc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "viSezaRe",
    },
    Dhatu {
        // 10.0267 `ligi~` citrIkaraRe (√liṅg). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*liṅgayati*),
        // parasmaipadī by 1.3.78 without (*liṅgati*). Slice 10g.
        dhatupatha: "10.0267",
        code: "ling",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "citrIkaraRe",
    },
    Dhatu {
        // 10.0464 `vawi~` prakASane (√vaṇṭ). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*vaṇṭayati*),
        // parasmaipadī by 1.3.78 without (*vaṇṭati*). Slice 10g.
        dhatupatha: "10.0464",
        code: "vanw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakASane",
    },
    Dhatu {
        // 10.0465 `laji~` prakASane (√lañj). Idit: ṇic optional by Kaumudī 2564
        // (7.1.58's num stored). Ubhayapadī by 1.3.74 with ṇic (*lañjayati*),
        // parasmaipadī by 1.3.78 without (*lañjati*). Slice 10g.
        dhatupatha: "10.0465",
        code: "lanj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakASane",
    },
    Dhatu {
        // 10.0338 `yu\ja~` saMyamane (√yuj). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*yojayati*), parasmaipadī by 1.3.78
        // without (*yojati*). Slice 10h.
        dhatupatha: "10.0338",
        code: "yuj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMyamane",
    },
    Dhatu {
        // 10.0339 `pfca~` saMyamane (√pṛc). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*parcayati*), parasmaipadī by 1.3.78
        // without (*parcati*). Slice 10h.
        dhatupatha: "10.0339",
        code: "pfc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMyamane",
    },
    Dhatu {
        // 10.0340 `arca~` pUjAyAm (√arc). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*arcayati*), parasmaipadī by 1.3.78
        // without (*arcati*). Slice 10h.
        dhatupatha: "10.0340",
        code: "arc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0341 `zaha~` marzaRe (√sah). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*sāhayati*), parasmaipadī by 1.3.78
        // without (*sahati*). Slice 10h.
        dhatupatha: "10.0341",
        code: "sah",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "marzaRe",
    },
    Dhatu {
        // 10.0342 `Ira~` kzepe (√īr). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*īrayati*), parasmaipadī by 1.3.78
        // without (*īrati*). Slice 10h.
        dhatupatha: "10.0342",
        code: "Ir",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0343 `lI` dravIkaraRe (√lī). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*lāyayati*), parasmaipadī by 1.3.78
        // without (*layati*). 7.2.115 *aco ñṇiti* takes vṛddhi of the final before ṇic.
        // Slice 10h.
        dhatupatha: "10.0343",
        code: "lI",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dravIkaraRe",
    },
    Dhatu {
        // 10.0344 `vfjI~` varjane (√vṛj). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*varjayati*), parasmaipadī by 1.3.78
        // without (*varjati*). Slice 10h.
        dhatupatha: "10.0344",
        code: "vfj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "varjane",
    },
    Dhatu {
        // 10.0345 `vfY` AvaraRe (√vṛ). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*vārayati*), and by 1.3.72 without, its
        // upadeśa being svarita or ñit (*varati* / *varate*). 7.2.115 *aco ñṇiti*
        // lengthens the final before ṇic. Slice 10h.
        dhatupatha: "10.0345",
        code: "vf",
        gana: Gana::Curadi,
        pada: PadaAssignment::NicUbhayapada,
        artha: "AvaraRe",
    },
    Dhatu {
        // 10.0346 `jF` vayohAnO (√jṝ). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*jārayati*), parasmaipadī by 1.3.78
        // without (*jarati*). 7.2.115 *aco ñṇiti* takes vṛddhi of the final before ṇic.
        // Slice 10h.
        dhatupatha: "10.0346",
        code: "jF",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vayohAnO",
    },
    Dhatu {
        // 10.0347 `jri\` vayohAnO (√jri). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*jrāyayati*), parasmaipadī by 1.3.78
        // without (*jrayati*). 7.2.115 *aco ñṇiti* takes vṛddhi of the final before ṇic.
        // Slice 10h.
        dhatupatha: "10.0347",
        code: "jri",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vayohAnO",
    },
    Dhatu {
        // 10.0348 `ri\ca~` viyojanasamparcanayoH (√ric). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*recayati*), parasmaipadī by
        // 1.3.78 without (*recati*). Slice 10h.
        dhatupatha: "10.0348",
        code: "ric",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "viyojanasamparcanayoH",
    },
    Dhatu {
        // 10.0349 `Si\za~` asarvopayoge (√śiṣ). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*śeṣayati*), parasmaipadī by 1.3.78
        // without (*śeṣati*). Slice 10h.
        dhatupatha: "10.0349",
        code: "Siz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "asarvopayoge",
    },
    Dhatu {
        // 10.0350 `ta\pa~` dAhe (√tap). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*tāpayati*), parasmaipadī by 1.3.78
        // without (*tapati*). Slice 10h.
        dhatupatha: "10.0350",
        code: "tap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dAhe",
    },
    Dhatu {
        // 10.0351 `tfpa~` tfptO sandIpane prIRane ca (√tṛp). Ādhṛṣīya: ṇic
        // optional by 10.0498. Ubhayapadī by 1.3.74 with ṇic (*tarpayati*),
        // parasmaipadī by 1.3.78 without (*tarpati*). Slice 10h.
        dhatupatha: "10.0351",
        code: "tfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "tfptO sandIpane prIRane ca",
    },
    Dhatu {
        // 10.0352 `CfdI~` sandIpane (√chṛd). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*chardayati*), parasmaipadī by 1.3.78
        // without (*chardati*). Slice 10h.
        dhatupatha: "10.0352",
        code: "Cfd",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sandIpane",
    },
    Dhatu {
        // 10.0353 `cfpa~` sandIpane (√cṛp). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*carpayati*), parasmaipadī by 1.3.78
        // without (*carpati*). Slice 10h.
        dhatupatha: "10.0353",
        code: "cfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sandIpane",
    },
    Dhatu {
        // 10.0354 `Cfpa~` sandIpane (√chṛp). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*charpayati*), parasmaipadī by 1.3.78
        // without (*charpati*). Slice 10h.
        dhatupatha: "10.0354",
        code: "Cfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sandIpane",
    },
    Dhatu {
        // 10.0355 `tfpa~` dIpane sandIpane (√tṛp). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*tarpayati*), parasmaipadī by
        // 1.3.78 without (*tarpati*). Slice 10h.
        dhatupatha: "10.0355",
        code: "tfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dIpane sandIpane",
    },
    Dhatu {
        // 10.0356 `dfpa~` sandIpane (√dṛp). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*darpayati*), parasmaipadī by 1.3.78
        // without (*darpati*). Slice 10h.
        dhatupatha: "10.0356",
        code: "dfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sandIpane",
    },
    Dhatu {
        // 10.0357 `dfBI~` Baye (√dṛbh). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*darbhayati*), parasmaipadī by 1.3.78
        // without (*darbhati*). Slice 10h.
        dhatupatha: "10.0357",
        code: "dfB",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Baye",
    },
    Dhatu {
        // 10.0358 `dfBa~` sandarBe (√dṛbh). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*darbhayati*), parasmaipadī by 1.3.78
        // without (*darbhati*). Slice 10h.
        dhatupatha: "10.0358",
        code: "dfB",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sandarBe",
    },
    Dhatu {
        // 10.0359 `lawa~` BAzAyAm (√laṭ). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*lāṭayati*), parasmaipadī by 1.3.78
        // without (*laṭati*). Slice 10h.
        dhatupatha: "10.0359",
        code: "law",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0360 `SraTa~` mokzaRe hiMsAyAM ca (√śrath). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*śrāthayati*), parasmaipadī by
        // 1.3.78 without (*śrathati*). Slice 10h.
        dhatupatha: "10.0360",
        code: "SraT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mokzaRe hiMsAyAM ca",
    },
    Dhatu {
        // 10.0361 `mI\` gatO (√mī). Ādhṛṣīya: ṇic optional by 10.0498. Ubhayapadī
        // by 1.3.74 with ṇic (*māyayati*), parasmaipadī by 1.3.78 without
        // (*mayati*). 7.2.115 *aco ñṇiti* takes vṛddhi of the final before ṇic. Slice
        // 10h.
        dhatupatha: "10.0361",
        code: "mI",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO",
    },
    Dhatu {
        // 10.0362 `granTa~` banDane (√granth). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*granthayati*), parasmaipadī by 1.3.78
        // without (*granthati*). Slice 10h.
        dhatupatha: "10.0362",
        code: "granT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "banDane",
    },
    Dhatu {
        // 10.0363 `SIka~` AmarzaRe (√śīk). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*śīkayati*), parasmaipadī by 1.3.78
        // without (*śīkati*). Slice 10h.
        dhatupatha: "10.0363",
        code: "SIk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AmarzaRe",
    },
    Dhatu {
        // 10.0364 `cIka~` AmarzaRe (√cīk). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*cīkayati*), parasmaipadī by 1.3.78
        // without (*cīkati*). Slice 10h.
        dhatupatha: "10.0364",
        code: "cIk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AmarzaRe",
    },
    Dhatu {
        // 10.0365 `arda~^` hiMsAyAm (√ard). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*ardayati*), and by 1.3.72 without, its
        // upadeśa being svarita or ñit (*ardati* / *ardate*). Slice 10h.
        dhatupatha: "10.0365",
        code: "ard",
        gana: Gana::Curadi,
        pada: PadaAssignment::NicUbhayapada,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0366 `hisi~` hiMsAyAm (√hiṃs). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*hiṃsayati*), parasmaipadī by 1.3.78
        // without (*hiṃsati*). Idit (7.1.58's num stored). Slice 10h.
        dhatupatha: "10.0366",
        code: "hins",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0367 `arha~` pUjAyAm (√arh). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*arhayati*), parasmaipadī by 1.3.78
        // without (*arhati*). Slice 10h.
        dhatupatha: "10.0367",
        code: "arh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0369 `SunDa~` SOcakarmaRi (√śundh). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*śundhayati*), parasmaipadī by
        // 1.3.78 without (*śundhati*). Slice 10h.
        dhatupatha: "10.0369",
        code: "SunD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SOcakarmaRi",
    },
    Dhatu {
        // 10.0370 `Cada~` apavAraRe (√chad). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*chādayati*), parasmaipadī by 1.3.78
        // without (*chadati*). Slice 10h.
        dhatupatha: "10.0370",
        code: "Cad",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "apavAraRe",
    },
    Dhatu {
        // 10.0371 `juza~` paritarkaRe paritarpaRe ca (√juṣ). Ādhṛṣīya: ṇic
        // optional by 10.0498. Ubhayapadī by 1.3.74 with ṇic (*joṣayati*),
        // parasmaipadī by 1.3.78 without (*joṣati*). Slice 10h.
        dhatupatha: "10.0371",
        code: "juz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "paritarkaRe paritarpaRe ca",
    },
    Dhatu {
        // 10.0372 `DUY` kampane (√dhū). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*dhāvayati*), and by 1.3.72 without, its
        // upadeśa being svarita or ñit (*dhavati* / *dhavate*). Before ṇic,
        // 7.2.115 *aco ñṇiti* or the vārttika 7.3.37.2's nuk (*dhūnayati*). Slice
        // 10h.
        dhatupatha: "10.0372",
        code: "DU",
        gana: Gana::Curadi,
        pada: PadaAssignment::NicUbhayapada,
        artha: "kampane",
    },
    Dhatu {
        // 10.0373 `prIY` tarpaRe kAntO ca (√prī). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*prāyayati*), and by 1.3.72
        // without, its upadeśa being svarita or ñit (*prayati* / *prayate*).
        // Before ṇic, 7.2.115 *aco ñṇiti* or the vārttika 7.3.37.2's nuk
        // (*prīṇayati*). Slice 10h.
        dhatupatha: "10.0373",
        code: "prI",
        gana: Gana::Curadi,
        pada: PadaAssignment::NicUbhayapada,
        artha: "tarpaRe kAntO ca",
    },
    Dhatu {
        // 10.0374 `SranTa~` sandarBe (√śranth). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*śranthayati*), parasmaipadī by 1.3.78
        // without (*śranthati*). Slice 10h.
        dhatupatha: "10.0374",
        code: "SranT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sandarBe",
    },
    Dhatu {
        // 10.0375 `granTa~` sandarBe (√granth). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*granthayati*), parasmaipadī by 1.3.78
        // without (*granthati*). Slice 10h.
        dhatupatha: "10.0375",
        code: "granT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sandarBe",
    },
    Dhatu {
        // 10.0376 `Apx~` lamBane (√āp). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*āpayati*), parasmaipadī by 1.3.78
        // without (*āpati*). Slice 10h.
        dhatupatha: "10.0376",
        code: "Ap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "lamBane",
    },
    Dhatu {
        // 10.0377 `tanu~` SradDopakaraRayoH (√tan). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*tānayati*), parasmaipadī by
        // 1.3.78 without (*tanati*). Slice 10h.
        dhatupatha: "10.0377",
        code: "tan",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SradDopakaraRayoH",
    },
    Dhatu {
        // 10.0378 `cana~` SradDopahananayoH (√can). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*cānayati*), parasmaipadī by
        // 1.3.78 without (*canati*). Slice 10h.
        dhatupatha: "10.0378",
        code: "can",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SradDopahananayoH",
    },
    Dhatu {
        // 10.0379 `vada~^` sandeSavacane (√vad). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*vādayati*), and by 1.3.72
        // without, its upadeśa being svarita or ñit (*vadati* / *vadate*). Slice
        // 10h.
        dhatupatha: "10.0379",
        code: "vad",
        gana: Gana::Curadi,
        pada: PadaAssignment::NicUbhayapada,
        artha: "sandeSavacane",
    },
    Dhatu {
        // 10.0380 `va\ca~` pariBAzaRe (√vac). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*vācayati*), parasmaipadī by 1.3.78
        // without (*vacati*). Slice 10h.
        dhatupatha: "10.0380",
        code: "vac",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pariBAzaRe",
    },
    Dhatu {
        // 10.0381 `mAna~` pUjAyAm (√mān). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*mānayati*), parasmaipadī by 1.3.78
        // without (*mānati*). Slice 10h.
        dhatupatha: "10.0381",
        code: "mAn",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0382 `BU` prAptO (√bhū). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*bhāvayati*), parasmaipadī by 1.3.78
        // without (*bhavati*). 7.2.115 *aco ñṇiti* takes vṛddhi of the final before ṇic.
        // Slice 10h.
        dhatupatha: "10.0382",
        code: "BU",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prAptO",
    },
    Dhatu {
        // 10.0383 `garha~` vinindane (√garh). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*garhayati*), parasmaipadī by 1.3.78
        // without (*garhati*). Slice 10h.
        dhatupatha: "10.0383",
        code: "garh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vinindane",
    },
    Dhatu {
        // 10.0384 `mArga~` anvezaRe (√mārg). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*mārgayati*), parasmaipadī by 1.3.78
        // without (*mārgati*). Slice 10h.
        dhatupatha: "10.0384",
        code: "mArg",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anvezaRe",
    },
    Dhatu {
        // 10.0385 `kaWi~` Soke (√kaṇṭh). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*kaṇṭhayati*), parasmaipadī by 1.3.78
        // without (*kaṇṭhati*). Idit (7.1.58's num stored). Slice 10h.
        dhatupatha: "10.0385",
        code: "kanW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Soke",
    },
    Dhatu {
        // 10.0386 `mfjU~` SOcAlaNkArayoH (√mṛj). Ādhṛṣīya: ṇic optional by
        // 10.0498. Ubhayapadī by 1.3.74 with ṇic (*mārjayati*), parasmaipadī by
        // 1.3.78 without (*mārjati*). 7.2.114 *mṛjer vṛddhiḥ* on both branches.
        // Slice 10h.
        dhatupatha: "10.0386",
        code: "mfj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SOcAlaNkArayoH",
    },
    Dhatu {
        // 10.0387 `mfza~^` titikzAyAm (√mṛṣ). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*marṣayati*), and by 1.3.72 without, its
        // upadeśa being svarita or ñit (*marṣati* / *marṣate*). Slice 10h.
        dhatupatha: "10.0387",
        code: "mfz",
        gana: Gana::Curadi,
        pada: PadaAssignment::NicUbhayapada,
        artha: "titikzAyAm",
    },
    Dhatu {
        // 10.0388 `Dfza~` prasahane (√dhṛṣ). Ādhṛṣīya: ṇic optional by 10.0498.
        // Ubhayapadī by 1.3.74 with ṇic (*dharṣayati*), parasmaipadī by 1.3.78
        // without (*dharṣati*). Slice 10h.
        dhatupatha: "10.0388",
        code: "Dfz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prasahane",
    },
    Dhatu {
        // 10.0022 `pF` pUraRe (√pṝ). Ṇic optional by Kaumudī 2565. Ubhayapadī by
        // 1.3.74 with ṇic (*pārayati*), parasmaipadī by 1.3.78 without (*parati*).
        // 7.2.115 *aco ñṇiti* lengthens the final before ṇic. Slice 10i.
        dhatupatha: "10.0022",
        code: "pF",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUraRe",
    },
    Dhatu {
        // 10.0251 `Guzi~r` viSabdane (√ghuṣ). Ṇic optional by Kaumudī 2571.
        // Ubhayapadī by 1.3.74 with ṇic (*ghoṣayati*), parasmaipadī by 1.3.78
        // without (*ghoṣati*). Irit (`i~r`), not idit, so not 2564's. Slice 10i.
        dhatupatha: "10.0251",
        code: "Guz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "viSabdane",
    },
    Dhatu {
        // 10.0279 `grasa~` grahaRe (√gras). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*grāsayati*), parasmaipadī by 1.3.78
        // without (*grasati*). Slice 10i.
        dhatupatha: "10.0279",
        code: "gras",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "grahaRe",
    },
    Dhatu {
        // 10.0280 `puza~` DAraRe (√puṣ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*poṣayati*), parasmaipadī by 1.3.78
        // without (*poṣati*). Slice 10i.
        dhatupatha: "10.0280",
        code: "puz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "DAraRe",
    },
    Dhatu {
        // 10.0281 `dala~` vidAraRe (√dal). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*dālayati*), parasmaipadī by 1.3.78
        // without (*dalati*). Slice 10i.
        dhatupatha: "10.0281",
        code: "dal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vidAraRe",
    },
    Dhatu {
        // 10.0282 `pawa~` BAzAyAm (√paṭ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*pāṭayati*), parasmaipadī by 1.3.78
        // without (*paṭati*). Slice 10i.
        dhatupatha: "10.0282",
        code: "paw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0283 `puwa~` BAzAyAm (√puṭ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*poṭayati*), parasmaipadī by 1.3.78
        // without (*poṭati*). Slice 10i.
        dhatupatha: "10.0283",
        code: "puw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0284 `luwa~` BAzAyAm (√luṭ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*loṭayati*), parasmaipadī by 1.3.78
        // without (*loṭati*). Slice 10i.
        dhatupatha: "10.0284",
        code: "luw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0285 `tuji~` BAzAyAm (√tuñj). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*tuñjayati*), parasmaipadī by 1.3.78
        // without (*tuñjati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0285",
        code: "tunj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0286 `miji~` BAzAyAm (√miñj). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*miñjayati*), parasmaipadī by 1.3.78
        // without (*miñjati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0286",
        code: "minj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0287 `piji~` BAzAyAm (√piñj). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*piñjayati*), parasmaipadī by 1.3.78
        // without (*piñjati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0287",
        code: "pinj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0288 `laka~` AsvAdane (√lak). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*lākayati*), parasmaipadī by 1.3.78
        // without (*lakati*). Slice 10i.
        dhatupatha: "10.0288",
        code: "lak",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0289 `luji~` BAzAyAm (√luñj). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*luñjayati*), parasmaipadī by 1.3.78
        // without (*luñjati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0289",
        code: "lunj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0290 `Baji~` BAzAyAm (√bhañj). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*bhañjayati*), parasmaipadī by 1.3.78
        // without (*bhañjati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0290",
        code: "Banj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0291 `laGi~` BAzAyAm (√laṅgh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*laṅghayati*), parasmaipadī by 1.3.78
        // without (*laṅghati*). Idit (7.1.58's num stored). Listed twice in the
        // dhātupāṭha, identically: `10.0291` and `10.0327`. Slice 10i.
        dhatupatha: "10.0291",
        code: "lanG",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0292 `trasi~` BAzAyAm (√traṃs). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*traṃsayati*), parasmaipadī by 1.3.78
        // without (*traṃsati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0292",
        code: "trans",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0293 `pisi~` BAzAyAm (√piṃs). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*piṃsayati*), parasmaipadī by 1.3.78
        // without (*piṃsati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0293",
        code: "pins",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0294 `kusi~` BAzAyAm (√kuṃs). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*kuṃsayati*), parasmaipadī by 1.3.78
        // without (*kuṃsati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0294",
        code: "kuns",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0295 `daSi~` BAzAyAm (√daṃś). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*daṃśayati*), parasmaipadī by 1.3.78
        // without (*daṃśati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0295",
        code: "danS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0296 `kuSi~` BAzAyAm (√kuṃś). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*kuṃśayati*), parasmaipadī by 1.3.78
        // without (*kuṃśati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0296",
        code: "kunS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0297 `Gawa~` BAzAyAm (√ghaṭ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*ghāṭayati*), parasmaipadī by 1.3.78
        // without (*ghaṭati*). Slice 10i.
        dhatupatha: "10.0297",
        code: "Gaw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0298 `Gawi~` BAzAyAm (√ghaṇṭ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*ghaṇṭayati*), parasmaipadī by 1.3.78
        // without (*ghaṇṭati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0298",
        code: "Ganw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0299 `bfhi~` BAzAyAm (√bṛṃh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*bṛṃhayati*), parasmaipadī by 1.3.78
        // without (*bṛṃhati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0299",
        code: "bfnh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0300 `barha~` BAzAyAm (√barh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*barhayati*), parasmaipadī by 1.3.78
        // without (*barhati*). Slice 10i.
        dhatupatha: "10.0300",
        code: "barh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0301 `balha~` BAzAyAm (√balh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*balhayati*), parasmaipadī by 1.3.78
        // without (*balhati*). Slice 10i.
        dhatupatha: "10.0301",
        code: "balh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0302 `gupa~` BAzAyAm (√gup). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*gopayati*), parasmaipadī by 1.3.78
        // without (*gopati*). Slice 10i.
        dhatupatha: "10.0302",
        code: "gup",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0303 `DUpa~` BAzAyAm (√dhūp). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*dhūpayati*), parasmaipadī by 1.3.78
        // without (*dhūpāyati*). Without ṇic, 3.1.28 gives āya. Slice 10i.
        dhatupatha: "10.0303",
        code: "DUp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0304 `viCa~` BAzAyAm (√vich). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*vicchayati*), parasmaipadī by 1.3.78
        // without (*vicchāyati*). Without ṇic, 3.1.28 gives āya; on both branches
        // 6.1.73's tuk comes before guṇa. Slice 10i.
        dhatupatha: "10.0304",
        code: "viC",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0305 `cIva~` BAzAyAm (√cīv). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*cīvayati*), parasmaipadī by 1.3.78
        // without (*cīvati*). Slice 10i.
        dhatupatha: "10.0305",
        code: "cIv",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0306 `puTa~` BAzAyAm (√puth). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*pothayati*), parasmaipadī by 1.3.78
        // without (*pothati*). Slice 10i.
        dhatupatha: "10.0306",
        code: "puT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0307 `lokf~` BAzAyAm (√lok). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*lokayati*), parasmaipadī by 1.3.78
        // without (*lokati*). Slice 10i.
        dhatupatha: "10.0307",
        code: "lok",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0308 `locf~` BAzAyAm (√loc). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*locayati*), parasmaipadī by 1.3.78
        // without (*locati*). Slice 10i.
        dhatupatha: "10.0308",
        code: "loc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0309 `Rada~` BAzAyAm (√nad). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*nādayati*), parasmaipadī by 1.3.78
        // without (*nadati*). Slice 10i.
        dhatupatha: "10.0309",
        code: "nad",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0310 `kupa~` BAzAyAm (√kup). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*kopayati*), parasmaipadī by 1.3.78
        // without (*kopati*). Slice 10i.
        dhatupatha: "10.0310",
        code: "kup",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0311 `tarka~` BAzAyAm (√tark). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*tarkayati*), parasmaipadī by 1.3.78
        // without (*tarkati*). Slice 10i.
        dhatupatha: "10.0311",
        code: "tark",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0312 `vftu~` BAzAyAm (√vṛt). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*vartayati*), parasmaipadī by 1.3.78
        // without (*vartati*). Udit, but āsvadīya: 10.0499's, not 2570's. Slice
        // 10i.
        dhatupatha: "10.0312",
        code: "vft",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0313 `vfDu~` BAzAyAm (√vṛdh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*vardhayati*), parasmaipadī by 1.3.78
        // without (*vardhati*). Udit, but āsvadīya: 10.0499's, not 2570's. Slice
        // 10i.
        dhatupatha: "10.0313",
        code: "vfD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0314 `ruwa~` BAzAyAm (√ruṭ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*roṭayati*), parasmaipadī by 1.3.78
        // without (*roṭati*). Slice 10i.
        dhatupatha: "10.0314",
        code: "ruw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0315 `laji~` BAzAyAm (√lañj). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*lañjayati*), parasmaipadī by 1.3.78
        // without (*lañjati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0315",
        code: "lanj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0316 `aji~` BAzAyAm (√añj). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*añjayati*), parasmaipadī by 1.3.78
        // without (*añjati*). Idit (7.1.58's num stored), but āsvadīya: 10.0499's,
        // not 2564's. Slice 10i.
        dhatupatha: "10.0316",
        code: "anj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0317 `dasi~` BAzAyAm (√daṃs). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*daṃsayati*), parasmaipadī by 1.3.78
        // without (*daṃsati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0317",
        code: "dans",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0318 `BfSi~` BAzAyAm (√bhṛṃś). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*bhṛṃśayati*), parasmaipadī by 1.3.78
        // without (*bhṛṃśati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0318",
        code: "BfnS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0319 `ruSi~` BAzAyAm (√ruṃś). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*ruṃśayati*), parasmaipadī by 1.3.78
        // without (*ruṃśati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0319",
        code: "runS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0320 `SIka~` BAzAyAm (√śīk). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*śīkayati*), parasmaipadī by 1.3.78
        // without (*śīkati*). Slice 10i.
        dhatupatha: "10.0320",
        code: "SIk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0321 `rusi~` BAzAyAm (√ruṃs). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*ruṃsayati*), parasmaipadī by 1.3.78
        // without (*ruṃsati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0321",
        code: "runs",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0322 `nawi~` BAzAyAm (√naṇṭ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*naṇṭayati*), parasmaipadī by 1.3.78
        // without (*naṇṭati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0322",
        code: "nanw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0323 `puwi~` BAzAyAm (√puṇṭ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*puṇṭayati*), parasmaipadī by 1.3.78
        // without (*puṇṭati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0323",
        code: "punw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0324 `ji` BAzAyAm (√ji). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*jāyayati*), parasmaipadī by 1.3.78
        // without (*jayati*). 7.2.115 *aco ñṇiti* lengthens the final before ṇic.
        // Slice 10i.
        dhatupatha: "10.0324",
        code: "ji",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0325 `ci` BAzAyAm (√ci). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*cāyayati*), parasmaipadī by 1.3.78
        // without (*cayati*). 7.2.115 *aco ñṇiti* lengthens the final before ṇic.
        // Slice 10i.
        dhatupatha: "10.0325",
        code: "ci",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0326 `raDi~` BAzAyAm (√randh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*randhayati*), parasmaipadī by 1.3.78
        // without (*randhati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0326",
        code: "ranD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0327 `laGi~` BAzAyAm (√laṅgh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*laṅghayati*), parasmaipadī by 1.3.78
        // without (*laṅghati*). Idit (7.1.58's num stored). Listed twice in the
        // dhātupāṭha, identically: `10.0291` and `10.0327`. Slice 10i.
        dhatupatha: "10.0327",
        code: "lanG",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0328 `ahi~` BAzAyAm (√aṃh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*aṃhayati*), parasmaipadī by 1.3.78
        // without (*aṃhati*). Idit (7.1.58's num stored), but āsvadīya: 10.0499's,
        // not 2564's. Slice 10i.
        dhatupatha: "10.0328",
        code: "anh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0329 `rahi~` BAzAyAm (√raṃh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*raṃhayati*), parasmaipadī by 1.3.78
        // without (*raṃhati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0329",
        code: "ranh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0330 `mahi~` BAzAyAm (√maṃh). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*maṃhayati*), parasmaipadī by 1.3.78
        // without (*maṃhati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0330",
        code: "manh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0331 `laqi~` BAzAyAm (√laṇḍ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*laṇḍayati*), parasmaipadī by 1.3.78
        // without (*laṇḍati*). Idit (7.1.58's num stored), but āsvadīya:
        // 10.0499's, not 2564's. Slice 10i.
        dhatupatha: "10.0331",
        code: "lanq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0332 `taqa~` BAzAyAm (√taḍ). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*tāḍayati*), parasmaipadī by 1.3.78
        // without (*taḍati*). Slice 10i.
        dhatupatha: "10.0332",
        code: "taq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0333 `nala~` BAzAyAm (√nal). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*nālayati*), parasmaipadī by 1.3.78
        // without (*nalati*). Slice 10i.
        dhatupatha: "10.0333",
        code: "nal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0334 `pUrI~` ApyAyane (√pūr). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*pūrayati*), parasmaipadī by 1.3.78
        // without (*pūrati*). Īdit, but āsvadīya: 10.0499's, not 2572's. Slice
        // 10i.
        dhatupatha: "10.0334",
        code: "pUr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "ApyAyane",
    },
    Dhatu {
        // 10.0335 `ruja~` hiMsAyAm (√ruj). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*rojayati*), parasmaipadī by 1.3.78
        // without (*rojati*). Slice 10i.
        dhatupatha: "10.0335",
        code: "ruj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0336 `zvada~` AsvAdane (√svad). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*svādayati*), parasmaipadī by 1.3.78
        // without (*svadati*). Slice 10i.
        dhatupatha: "10.0336",
        code: "svad",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0337 `svAda~` AsvAdane (√svād). Āsvadīya: ṇic optional by 10.0499.
        // Ubhayapadī by 1.3.74 with ṇic (*svādayati*), parasmaipadī by 1.3.78
        // without (*svādati*). Slice 10i.
        dhatupatha: "10.0337",
        code: "svAd",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0058 `zmiN` anAdare (√smiṅ). Ṅit, so ātmanepadī, and under ṇic
        // Kaumudī 2567 keeps it so: 1.3.12 credits, not 1.3.74 (*smāyayate*).
        // 7.2.115 *aco ñṇiti* and the sanādi 6.1.78 make `smAy`. Stored per
        // 6.1.64. Slice 10j.
        dhatupatha: "10.0058",
        code: "smi",
        gana: Gana::Curadi,
        pada: PadaAssignment::Atmanepada,
        artha: "anAdare",
    },
    Dhatu {
        // 10.0124 `ciY` cayane (√ci). Jñapādi, so mit (10.0493); ñit, so ṇic
        // optional by Kaumudī 2570. Ubhayapadī by 1.3.74 with ṇic, where 6.1.54
        // *cisphuror ṇau* optionally gives `cA` and 7.3.36 its puk
        // (*capayati*), or 7.2.115 and 6.1.78 give `cAy` (*cayayati*), and
        // 6.4.92 shortens both; ubhayapadī by 1.3.72 without (*cayati* /
        // *cayate*). Slice 10j.
        dhatupatha: "10.0124",
        code: "ci",
        gana: Gana::Curadi,
        pada: PadaAssignment::NicUbhayapada,
        artha: "cayane",
    },
    Dhatu {
        // 10.0152 `Gf` prasravaRe (√ghṛ). Ubhayapadī by 1.3.74 (*ghārayati*).
        // 7.2.115 *aco ñṇiti* makes `GAr` (1.1.51). Slice 10j.
        dhatupatha: "10.0152",
        code: "Gf",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prasravaRe",
    },
    Dhatu {
        // 10.0231 `gf` vijYAne (√gṛ). Ākusmīya: ātmanepadī by 10.0496
        // (*gārayate*). 7.2.115 makes `gAr`. Slice 10j.
        dhatupatha: "10.0231",
        code: "gf",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "vijYAne",
    },
    Dhatu {
        // 10.0235 `yu` jugupsAyAm (√yu). Ākusmīya: ātmanepadī by 10.0496
        // (*yāvayate*). 7.2.115 and the sanādi 6.1.78 make `yAv`. Slice 10j.
        dhatupatha: "10.0235",
        code: "yu",
        gana: Gana::Curadi,
        pada: PadaAssignment::Akusmiya,
        artha: "jugupsAyAm",
    },
    Dhatu {
        // 10.0258 `jYA` niyoge (√jñā). Ubhayapadī by 1.3.74 (*jñāpayati*).
        // 7.3.36 adds puk; outside the jñapādi, so not mit, and the `A` stays
        // long. Slice 10j.
        dhatupatha: "10.0258",
        code: "jYA",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "niyoge",
    },
    Dhatu {
        // 10.0275 `cyu` sahane hasane ca (√cyu). Ubhayapadī by 1.3.74
        // (*cyāvayati*). 7.2.115 and the sanādi 6.1.78 make `cyAv`. Slice 10j.
        dhatupatha: "10.0275",
        code: "cyu",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sahane hasane ca",
    },
    Dhatu {
        // 10.0277 `BU` avakalkane (√bhū). Ubhayapadī by 1.3.74 (*bhāvayati*),
        // as the ādhṛṣīya `10.0382 BU` is on its ṇic branch. 7.2.115 and the
        // sanādi 6.1.78 make `BAv`. Slice 10j.
        dhatupatha: "10.0277",
        code: "BU",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avakalkane",
    },
    Dhatu {
        // 10.0006 `lakza~` darSanANkanayoH (√lakṣ). Guru upadhā (the conjunct
        // `kz`), so unchanged before ṇic. Shares its ātmanepada forms with the
        // ākusmīya `10.0219 lakza~`. Ubhayapadī by 1.3.74 (*lakṣayati*). Slice
        // 10k.
        dhatupatha: "10.0006",
        code: "lakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "darSanANkanayoH",
    },
    Dhatu {
        // 10.0008 `kudf~` anftaBAzaRe (√kud). 7.3.86 guṇates the laghu upadhā
        // before ṇic (kod-i). Ubhayapadī by 1.3.74 (*kodayati*). Slice 10k.
        dhatupatha: "10.0008",
        code: "kud",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anftaBAzaRe",
    },
    Dhatu {
        // 10.0012 `mida~` snehane (√mid). 7.3.86 guṇates the laghu upadhā
        // before ṇic (med-i). Ubhayapadī by 1.3.74 (*medayati*). Slice 10k.
        dhatupatha: "10.0012",
        code: "mid",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehane",
    },
    Dhatu {
        // 10.0015 `jala~` apavAraRe (√jal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (jAl-i). Ubhayapadī by 1.3.74 (*jālayati*).
        // Slice 10k.
        dhatupatha: "10.0015",
        code: "jal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "apavAraRe",
    },
    Dhatu {
        // 10.0016 `laja~` apavAraRe (√laj). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (lAj-i). Ubhayapadī by 1.3.74 (*lājayati*).
        // Slice 10k.
        dhatupatha: "10.0016",
        code: "laj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "apavAraRe",
    },
    Dhatu {
        // 10.0017 `pIqa~` avagAhane (√pīḍ). Long upadhā vowel `I`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*pīḍayati*). Slice 10k.
        dhatupatha: "10.0017",
        code: "pIq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avagAhane",
    },
    Dhatu {
        // 10.0018 `nawa~` avaspandane (√naṭ). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (nAw-i). Ubhayapadī by 1.3.74
        // (*nāṭayati*). Slice 10k.
        dhatupatha: "10.0018",
        code: "naw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avaspandane",
    },
    Dhatu {
        // 10.0019 `SraTa~` prayatne (√śrath). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (SrAT-i). Homograph of the ubhayapadī
        // curādi row `10.0360 SraTa~`: every form of this row is also the
        // partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*śrāthayati*).
        // Slice 10k.
        dhatupatha: "10.0019",
        code: "SraT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prayatne",
    },
    Dhatu {
        // 10.0020 `baDa~` saMyamane (√badh). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (bAD-i). Ubhayapadī by 1.3.74
        // (*bādhayati*). Slice 10k.
        dhatupatha: "10.0020",
        code: "baD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMyamane",
    },
    Dhatu {
        // 10.0021 `banDa~` saMyamane (√bandh). Guru upadhā (the conjunct `nD`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra before `D`,
        // and 8.4.58 restores `n`. Ubhayapadī by 1.3.74 (*bandhayati*). Slice
        // 10k.
        dhatupatha: "10.0021",
        code: "banD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMyamane",
    },
    Dhatu {
        // 10.0024 `pakza~` parigrahe (√pakṣ). Guru upadhā (the conjunct `kz`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*pakṣayati*). Slice
        // 10k.
        dhatupatha: "10.0024",
        code: "pakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "parigrahe",
    },
    Dhatu {
        // 10.0025 `varRa~` preraRe (√varṇ). Guru upadhā (the conjunct `rR`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0484 varRa`: every form of this row is also the partner's (its
        // ṇic branch's). Ubhayapadī by 1.3.74 (*varṇayati*). Slice 10k.
        dhatupatha: "10.0025",
        code: "varR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe",
    },
    Dhatu {
        // 10.0027 `praTa~` praKyAne (√prath). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (prAT-i). Ubhayapadī by 1.3.74
        // (*prāthayati*). Slice 10k.
        dhatupatha: "10.0027",
        code: "praT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "praKyAne",
    },
    Dhatu {
        // 10.0028 `pfTa~` prakzepe (√pṛth). 7.3.86 guṇates the laghu upadhā `f`
        // to `ar` (1.1.51) before ṇic (parT-i). Shares every form with `10.0186
        // parTa~`. Ubhayapadī by 1.3.74 (*parthayati*). Slice 10k.
        dhatupatha: "10.0028",
        code: "pfT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakzepe",
    },
    Dhatu {
        // 10.0029 `paTa~` prakzepe (√path). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (pAT-i). Ubhayapadī by 1.3.74
        // (*pāthayati*). Slice 10k.
        dhatupatha: "10.0029",
        code: "paT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakzepe",
    },
    Dhatu {
        // 10.0030 `zanba~` sambanDane (√sanb). Guru upadhā (the conjunct `nb`),
        // so unchanged before ṇic. Stored per 6.1.64. 8.3.24 makes its `n` an
        // anusvāra, and 8.4.58 makes that `m` before `b`. Ubhayapadī by 1.3.74
        // (*sambayati*). Slice 10k.
        dhatupatha: "10.0030",
        code: "sanb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sambanDane",
    },
    Dhatu {
        // 10.0031 `Sanba~` sambanDane (√śanb). Guru upadhā (the conjunct `nb`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra, and 8.4.58
        // makes that `m` before `b`. Ubhayapadī by 1.3.74 (*śambayati*). Slice
        // 10k.
        dhatupatha: "10.0031",
        code: "Sanb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sambanDane",
    },
    Dhatu {
        // 10.0032 `sAnba~` sambanDane (√sānb). Guru upadhā (the conjunct `nb`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra, and 8.4.58
        // makes that `m` before `b`. Ubhayapadī by 1.3.74 (*sāmbayati*). Slice
        // 10k.
        dhatupatha: "10.0032",
        code: "sAnb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sambanDane",
    },
    Dhatu {
        // 10.0034 `kuwwa~` CedanaBartsanayoH (√kuṭṭ). Guru upadhā (the conjunct
        // `ww`), so unchanged before ṇic. Shares its ātmanepada forms with the
        // ākusmīya `10.0226 kuwwa~`. Ubhayapadī by 1.3.74 (*kuṭṭayati*). Slice
        // 10k.
        dhatupatha: "10.0034",
        code: "kuww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "CedanaBartsanayoH",
    },
    Dhatu {
        // 10.0035 `puwwa~` alpIBAve (√puṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*puṭṭayati*). Slice
        // 10k.
        dhatupatha: "10.0035",
        code: "puww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "alpIBAve",
    },
    Dhatu {
        // 10.0036 `cuwwa~` alpIBAve (√cuṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*cuṭṭayati*). Slice
        // 10k.
        dhatupatha: "10.0036",
        code: "cuww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "alpIBAve",
    },
    Dhatu {
        // 10.0038 `zuwwa~` anAdare (√suṭṭ). Guru upadhā (the conjunct `ww`), so
        // unchanged before ṇic. Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*suṭṭayati*). Slice 10k.
        dhatupatha: "10.0038",
        code: "suww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anAdare",
    },
    Dhatu {
        // 10.0039 `lunwa~` steye (√lunṭ). Guru upadhā (the conjunct `nw`), so
        // unchanged before ṇic. 8.3.24 makes its `n` an anusvāra, and 8.4.58
        // makes that `R` before `w`. Ubhayapadī by 1.3.74 (*luṇṭayati*). Slice
        // 10k.
        dhatupatha: "10.0039",
        code: "lunw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "steye",
    },
    Dhatu {
        // 10.0040 `lunWa~` steye (√lunṭh). Guru upadhā (the conjunct `nW`), so
        // unchanged before ṇic. 8.3.24 makes its `n` an anusvāra, and 8.4.58
        // makes that `R` before `W`. Ubhayapadī by 1.3.74 (*luṇṭhayati*). Slice
        // 10k.
        dhatupatha: "10.0040",
        code: "lunW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "steye",
    },
    Dhatu {
        // 10.0041 `SaWa~` asaMskAragatyoH (√śaṭh). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (SAW-i). Shares its
        // ātmanepada forms with the ākusmīya `10.0214 SaWa~`. Ubhayapadī by
        // 1.3.74 (*śāṭhayati*). Slice 10k.
        dhatupatha: "10.0041",
        code: "SaW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "asaMskAragatyoH",
    },
    Dhatu {
        // 10.0042 `SvaWa~` asaMskAragatyoH (√śvaṭh). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (SvAW-i). Ubhayapadī by
        // 1.3.74 (*śvāṭhayati*). Slice 10k.
        dhatupatha: "10.0042",
        code: "SvaW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "asaMskAragatyoH",
    },
    Dhatu {
        // 10.0044 `tuja~` hiMsAbalAdAnaniketanezu (√tuj). 7.3.86 guṇates the
        // laghu upadhā before ṇic (toj-i). Ubhayapadī by 1.3.74 (*tojayati*).
        // Slice 10k.
        dhatupatha: "10.0044",
        code: "tuj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAbalAdAnaniketanezu",
    },
    Dhatu {
        // 10.0046 `pija~` hiMsAbalAdAnaniketanezu (√pij). 7.3.86 guṇates the
        // laghu upadhā before ṇic (pej-i). Ubhayapadī by 1.3.74 (*pejayati*).
        // Slice 10k.
        dhatupatha: "10.0046",
        code: "pij",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAbalAdAnaniketanezu",
    },
    Dhatu {
        // 10.0050 `pisa~` gatO (√pis). 7.3.86 guṇates the laghu upadhā before
        // ṇic (pes-i). Ubhayapadī by 1.3.74 (*pesayati*). Slice 10k.
        dhatupatha: "10.0050",
        code: "pis",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO",
    },
    Dhatu {
        // 10.0051 `zAntva~` sAmaprayoge (√sāntv). Guru upadhā (the conjunct
        // `ntv`), so unchanged before ṇic. Stored per 6.1.64. 8.3.24 makes its
        // `n` an anusvāra before `t`, and 8.4.58 restores `n`. Shares every
        // form with `10.0052 sAntva~`. `CONVERGENT_UPADESHA_PAIR` lets the two
        // resolve. Ubhayapadī by 1.3.74 (*sāntvayati*). Slice 10k.
        dhatupatha: "10.0051",
        code: "sAntv",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sAmaprayoge",
    },
    Dhatu {
        // 10.0052 `sAntva~` sAmaprayoge (√sāntv). Guru upadhā (the conjunct
        // `ntv`), so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra
        // before `t`, and 8.4.58 restores `n`. Shares every form with `10.0051
        // zAntva~`. `CONVERGENT_UPADESHA_PAIR` lets the two resolve. Ubhayapadī
        // by 1.3.74 (*sāntvayati*). Slice 10k.
        dhatupatha: "10.0052",
        code: "sAntv",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sAmaprayoge",
    },
    Dhatu {
        // 10.0053 `Svalka~` pariBAzaRe (√śvalk). Guru upadhā (the conjunct
        // `lk`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śvalkayati*).
        // Slice 10k.
        dhatupatha: "10.0053",
        code: "Svalk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pariBAzaRe",
    },
    Dhatu {
        // 10.0054 `valka~` pariBAzaRe (√valk). Guru upadhā (the conjunct
        // `lk`), so unchanged before ṇic. Homograph of the ubhayapadī curādi
        // row `10.0458 valka`: every form of this row is also the partner's
        // (its ṇic branch's). Ubhayapadī by 1.3.74 (*valkayati*). Slice 10k.
        dhatupatha: "10.0054",
        code: "valk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pariBAzaRe",
    },
    Dhatu {
        // 10.0055 `zRiha~` snehane (√snih). 7.3.86 guṇates the laghu upadhā
        // before ṇic (sneh-i). Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*snehayati*). Slice 10k.
        dhatupatha: "10.0055",
        code: "snih",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehane",
    },
    Dhatu {
        // 10.0056 `sPiwa~` hiMsAyAm (√sphiṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (sPew-i). Ubhayapadī by 1.3.74 (*spheṭayati*). Slice 10k.
        dhatupatha: "10.0056",
        code: "sPiw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0057 `smiwa~` anAdare (√smiṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (smew-i). Ubhayapadī by 1.3.74 (*smeṭayati*). Slice 10k.
        dhatupatha: "10.0057",
        code: "smiw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anAdare",
    },
    Dhatu {
        // 10.0059 `Sliza~` SlezaRe (√śliṣ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (Slez-i). Ubhayapadī by 1.3.74 (*śleṣayati*). Slice 10k.
        dhatupatha: "10.0059",
        code: "Sliz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SlezaRe",
    },
    Dhatu {
        // 10.0061 `piCa~` kuwwane (√pich). The sanādi 6.1.73 che ca gives tuk
        // before its `C` (picC), so the upadhā is guru and unchanged before
        // ṇic. Ubhayapadī by 1.3.74 (*picchayati*). Slice 10k.
        dhatupatha: "10.0061",
        code: "piC",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kuwwane",
    },
    Dhatu {
        // 10.0063 `SraRa~` dAne (√śraṇ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (SrAR-i). Homograph of the ubhayapadī
        // curādi row `10.0174 SraRu~`: every form of this row is also the
        // partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*śrāṇayati*).
        // Slice 10k.
        dhatupatha: "10.0063",
        code: "SraR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "dAne",
    },
    Dhatu {
        // 10.0064 `taqa~` AGAte (√taḍ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (tAq-i). Homograph of the ubhayapadī
        // curādi row `10.0332 taqa~`: every form of this row is also the
        // partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*tāḍayati*).
        // Slice 10k.
        dhatupatha: "10.0064",
        code: "taq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AGAte",
    },
    Dhatu {
        // 10.0065 `Kaqa~` Bedane (√khaḍ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (KAq-i). Ubhayapadī by 1.3.74
        // (*khāḍayati*). Slice 10k.
        dhatupatha: "10.0065",
        code: "Kaq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0078 `Carda~` vamane (√chard). Guru upadhā (the conjunct `rd`),
        // so unchanged before ṇic. In laṅ the aṭ takes 6.1.73's tuk (acC-).
        // Homograph of the ubhayapadī curādi row `10.0352 CfdI~`: every form
        // of this row is also the partner's (its ṇic branch's). Ubhayapadī by
        // 1.3.74 (*chardayati*). Slice 10k.
        dhatupatha: "10.0078",
        code: "Card",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vamane",
    },
    Dhatu {
        // 10.0079 `pusta~` AdarAnAdarayoH (√pust). Guru upadhā (the conjunct
        // `st`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*pustayati*).
        // Slice 10k.
        dhatupatha: "10.0079",
        code: "pust",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AdarAnAdarayoH",
    },
    Dhatu {
        // 10.0080 `busta~` AdarAnAdarayoH (√bust). Guru upadhā (the conjunct
        // `st`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*bustayati*).
        // Slice 10k.
        dhatupatha: "10.0080",
        code: "bust",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AdarAnAdarayoH",
    },
    Dhatu {
        // 10.0081 `cuda~` saYcodane (√cud). 7.3.86 guṇates the laghu upadhā
        // before ṇic (cod-i). Ubhayapadī by 1.3.74 (*codayati*). Slice 10k.
        dhatupatha: "10.0081",
        code: "cud",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saYcodane",
    },
    Dhatu {
        // 10.0082 `nakka~` nASane (√nakk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*nakkayati*). Slice 10k.
        dhatupatha: "10.0082",
        code: "nakk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nASane",
    },
    Dhatu {
        // 10.0083 `Dakka~` nASane (√dhakk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*dhakkayati*). Slice 10k.
        dhatupatha: "10.0083",
        code: "Dakk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nASane",
    },
    Dhatu {
        // 10.0084 `cakka~` vyaTane (√cakk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*cakkayati*). Slice 10k.
        dhatupatha: "10.0084",
        code: "cakk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaTane",
    },
    Dhatu {
        // 10.0085 `cukka~` vyaTane (√cukk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*cukkayati*). Slice 10k.
        dhatupatha: "10.0085",
        code: "cukk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaTane",
    },
    Dhatu {
        // 10.0086 `kzala~` SOcakarmaRi (√kṣal). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (kzAl-i). Ubhayapadī by 1.3.74
        // (*kṣālayati*). Slice 10k.
        dhatupatha: "10.0086",
        code: "kzal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SOcakarmaRi",
    },
    Dhatu {
        // 10.0087 `tala~` pratizWAyAm (√tal). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (tAl-i). Ubhayapadī by 1.3.74
        // (*tālayati*). Slice 10k.
        dhatupatha: "10.0087",
        code: "tal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pratizWAyAm",
    },
    Dhatu {
        // 10.0088 `tula~` unmAne (√tul). 7.3.86 guṇates the laghu upadhā before
        // ṇic (tol-i). Ubhayapadī by 1.3.74 (*tolayati*). Slice 10k.
        dhatupatha: "10.0088",
        code: "tul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "unmAne",
    },
    Dhatu {
        // 10.0089 `dula~` utkzepe (√dul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (dol-i). Ubhayapadī by 1.3.74 (*dolayati*). Slice 10k.
        dhatupatha: "10.0089",
        code: "dul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "utkzepe",
    },
    Dhatu {
        // 10.0090 `pula~` mahattve (√pul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (pol-i). Shares every form with `10.0131 pula~`.
        // Ubhayapadī by 1.3.74 (*polayati*). Slice 10k.
        dhatupatha: "10.0090",
        code: "pul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mahattve",
    },
    Dhatu {
        // 10.0091 `cula~` samucCrAye (√cul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (col-i). Ubhayapadī by 1.3.74 (*colayati*). Slice 10k.
        dhatupatha: "10.0091",
        code: "cul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samucCrAye",
    },
    Dhatu {
        // 10.0092 `mUla~` rohaRe (√mūl). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*mūlayati*). Slice 10k.
        dhatupatha: "10.0092",
        code: "mUl",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rohaRe",
    },
    Dhatu {
        // 10.0093 `kala~` kzepe (√kal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (kAl-i). Homograph of the ubhayapadī
        // curādi row `10.0422 kAla`: every form of this row is also the
        // partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*kālayati*).
        // Slice 10k.
        dhatupatha: "10.0093",
        code: "kal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0094 `vila~` kzepe (√vil). 7.3.86 guṇates the laghu upadhā before ṇic (vel-i).
        // Homograph of the ubhayapadī curādi row `10.0421 vela`: every form of this row is also the
        // partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*velayati*). Slice 10k.
        dhatupatha: "10.0094",
        code: "vil",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0095 `bila~` Bedane (√bil). 7.3.86 guṇates the laghu upadhā before
        // ṇic (bel-i). Ubhayapadī by 1.3.74 (*belayati*). Slice 10k.
        dhatupatha: "10.0095",
        code: "bil",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0096 `tila~` snehane (√til). 7.3.86 guṇates the laghu upadhā
        // before ṇic (tel-i). Ubhayapadī by 1.3.74 (*telayati*). Slice 10k.
        dhatupatha: "10.0096",
        code: "til",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehane",
    },
    Dhatu {
        // 10.0097 `cala~` BftO (√cal). 7.2.116 ata upadhāyāḥ lengthens the `a`
        // upadhā before ṇit ṇic (cAl-i). Ubhayapadī by 1.3.74 (*cālayati*).
        // Slice 10k.
        dhatupatha: "10.0097",
        code: "cal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BftO",
    },
    Dhatu {
        // 10.0098 `pAla~` rakzaRe (√pāl). Long upadhā vowel `A`, so unchanged
        // before ṇic. Shares every form with `10.0099 pala~`. Ubhayapadī by
        // 1.3.74 (*pālayati*). Slice 10k.
        dhatupatha: "10.0098",
        code: "pAl",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe",
    },
    Dhatu {
        // 10.0099 `pala~` rakzaRe (√pal). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (pAl-i). Shares every form with `10.0098
        // pAla~`. Ubhayapadī by 1.3.74 (*pālayati*). Slice 10k.
        dhatupatha: "10.0099",
        code: "pal",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "rakzaRe",
    },
    Dhatu {
        // 10.0100 `lUza~` hiMsAyAm (√lūṣ). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*lūṣayati*). Slice 10k.
        dhatupatha: "10.0100",
        code: "lUz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0101 `Sulba~` mAne (√śulb). Guru upadhā (the conjunct `lb`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*śulbayati*). Slice 10k.
        dhatupatha: "10.0101",
        code: "Sulb",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mAne",
    },
    Dhatu {
        // 10.0102 `SUrpa~` mAne (√śūrp). Guru upadhā (the conjunct `rp`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*śūrpayati*). Slice 10k.
        dhatupatha: "10.0102",
        code: "SUrp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mAne",
    },
    Dhatu {
        // 10.0103 `cuwa~` Cedane (√cuṭ). 7.3.86 guṇates the laghu upadhā before
        // ṇic (cow-i). Ubhayapadī by 1.3.74 (*coṭayati*). Slice 10k.
        dhatupatha: "10.0103",
        code: "cuw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0104 `muwa~` saYcUrRane (√muṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (mow-i). Ubhayapadī by 1.3.74 (*moṭayati*). Slice 10k.
        dhatupatha: "10.0104",
        code: "muw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saYcUrRane",
    },
    Dhatu {
        // 10.0109 `vraja~` saMskAragatyoH (√vraj). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (vrAj-i). Ubhayapadī by
        // 1.3.74 (*vrājayati*). Slice 10k.
        dhatupatha: "10.0109",
        code: "vraj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMskAragatyoH",
    },
    Dhatu {
        // 10.0110 `Sulka~` atisparSane (√śulk). Guru upadhā (the conjunct
        // `lk`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śulkayati*).
        // Slice 10k.
        dhatupatha: "10.0110",
        code: "Sulk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "atisparSane",
    },
    Dhatu {
        // 10.0115 `Svarta~` gatyAm (√śvart). Guru upadhā (the conjunct `rt`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śvartayati*). Slice
        // 10k.
        dhatupatha: "10.0115",
        code: "Svart",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatyAm",
    },
    Dhatu {
        // 10.0116 `svarta~` kfcCrajIvane, gatyAm (√svart). Guru upadhā (the
        // conjunct `rt`), so unchanged before ṇic. Ubhayapadī by 1.3.74
        // (*svartayati*). Slice 10k.
        dhatupatha: "10.0116",
        code: "svart",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kfcCrajIvane, gatyAm",
    },
    Dhatu {
        // 10.0117 `SvaBra~` gatyAm (√śvabhr). Guru upadhā (the conjunct `Br`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śvabhrayati*). Slice
        // 10k.
        dhatupatha: "10.0117",
        code: "SvaBr",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatyAm",
    },
    Dhatu {
        // 10.0125 `Gawwa~` calane (√ghaṭṭ). Guru upadhā (the conjunct `ww`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*ghaṭṭayati*). Slice 10k.
        dhatupatha: "10.0125",
        code: "Gaww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "calane",
    },
    Dhatu {
        // 10.0126 `musta~` saNGAte (√must). Guru upadhā (the conjunct `st`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*mustayati*). Slice 10k.
        dhatupatha: "10.0126",
        code: "must",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0127 `Kawwa~` saMvaraRe (√khaṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*khaṭṭayati*). Slice
        // 10k.
        dhatupatha: "10.0127",
        code: "Kaww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMvaraRe",
    },
    Dhatu {
        // 10.0128 `zawwa~` hiMsAyAm (√saṭṭ). Guru upadhā (the conjunct `ww`),
        // so unchanged before ṇic. Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*saṭṭayati*). Slice 10k.
        dhatupatha: "10.0128",
        code: "saww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0129 `sPiwwa~` hiMsAyAm (√sphiṭṭ). Guru upadhā (the conjunct
        // `ww`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*sphiṭṭayati*).
        // Slice 10k.
        dhatupatha: "10.0129",
        code: "sPiww",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0131 `pula~` saNGAte (√pul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (pol-i). Shares every form with `10.0090 pula~`.
        // Ubhayapadī by 1.3.74 (*polayati*). Slice 10k.
        dhatupatha: "10.0131",
        code: "pul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0132 `pUrRa~` saNGAte (√pūrṇ). Guru upadhā (the conjunct `rR`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*pūrṇayati*). Slice 10k.
        dhatupatha: "10.0132",
        code: "pUrR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0133 `puRa~` saNGAte (√puṇ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (poR-i). Ubhayapadī by 1.3.74 (*poṇayati*). Slice 10k.
        dhatupatha: "10.0133",
        code: "puR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0134 `punsa~` aBivarDane (√puns). Guru upadhā (the conjunct `ns`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra before `s`,
        // which stays. Ubhayapadī by 1.3.74 (*puṃsayati*). Slice 10k.
        dhatupatha: "10.0134",
        code: "puns",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "aBivarDane",
    },
    Dhatu {
        // 10.0136 `vyapa~` kzaye (√vyap). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (vyAp-i). Ubhayapadī by 1.3.74
        // (*vyāpayati*). Slice 10k.
        dhatupatha: "10.0136",
        code: "vyap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzaye",
    },
    Dhatu {
        // 10.0137 `vyaya~` kzaye (√vyay). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (vyAy-i). Ubhayapadī by 1.3.74
        // (*vyāyayati*). Slice 10k.
        dhatupatha: "10.0137",
        code: "vyay",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzaye",
    },
    Dhatu {
        // 10.0138 `pUla~` saNGAte (√pūl). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*pūlayati*). Slice 10k.
        dhatupatha: "10.0138",
        code: "pUl",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0139 `DUsa~` kAntikaraRe (√dhūs). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*dhūsayati*). Slice 10k.
        dhatupatha: "10.0139",
        code: "DUs",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kAntikaraRe",
    },
    Dhatu {
        // 10.0140 `DUza~` kAntikaraRe (√dhūṣ). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*dhūṣayati*). Slice 10k.
        dhatupatha: "10.0140",
        code: "DUz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kAntikaraRe",
    },
    Dhatu {
        // 10.0141 `DUSa~` kAntikaraRe (√dhūś). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*dhūśayati*). Slice 10k.
        dhatupatha: "10.0141",
        code: "DUS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kAntikaraRe",
    },
    Dhatu {
        // 10.0142 `kIwa~` varRe (√kīṭ). Long upadhā vowel `I`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*kīṭayati*). Slice 10k.
        dhatupatha: "10.0142",
        code: "kIw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "varRe",
    },
    Dhatu {
        // 10.0143 `cUrRa~` saNkocane (√cūrṇ). Guru upadhā (the conjunct `rR`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*cūrṇayati*).
        // Homograph of `10.0026 curRa~` (slice 10l), which 8.2.78 lengthens to
        // the same `cUrR`: every form of that row is also this one's. Slice
        // 10k.
        dhatupatha: "10.0143",
        code: "cUrR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNkocane",
    },
    Dhatu {
        // 10.0144 `pUja~` pUjAyAm (√pūj). Long upadhā vowel `U`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*pūjayati*). Slice 10k.
        dhatupatha: "10.0144",
        code: "pUj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0145 `arka~` stavane (√ark). Guru upadhā (the conjunct `rk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*arkayati*). Slice 10k.
        dhatupatha: "10.0145",
        code: "ark",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "stavane",
    },
    Dhatu {
        // 10.0146 `SuWa~` Alasye (√śuṭh). 7.3.86 guṇates the laghu upadhā
        // before ṇic (SoW-i). Ubhayapadī by 1.3.74 (*śoṭhayati*). Slice 10k.
        dhatupatha: "10.0146",
        code: "SuW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Alasye",
    },
    Dhatu {
        // 10.0148 `juqa~` preraRe (√juḍ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (joq-i). Ubhayapadī by 1.3.74 (*joḍayati*). Slice 10k.
        dhatupatha: "10.0148",
        code: "juq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe",
    },
    Dhatu {
        // 10.0149 `gaja~` SabdArTe (√gaj). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (gAj-i). Ubhayapadī by 1.3.74 (*gājayati*).
        // Slice 10k.
        dhatupatha: "10.0149",
        code: "gaj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SabdArTe",
    },
    Dhatu {
        // 10.0150 `mArja~` SabdArTe (√mārj). Guru upadhā (the conjunct `rj`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0386 mfjU~`: every form of this row is also the partner's (its
        // ṇic branch's). Ubhayapadī by 1.3.74 (*mārjayati*). Slice 10k.
        dhatupatha: "10.0150",
        code: "mArj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SabdArTe",
    },
    Dhatu {
        // 10.0151 `marca~` SabdArTe (√marc). Guru upadhā (the conjunct `rc`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*marcayati*). Slice
        // 10k.
        dhatupatha: "10.0151",
        code: "marc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SabdArTe",
    },
    Dhatu {
        // 10.0154 `tija~` niSAne (√tij). 7.3.86 guṇates the laghu upadhā before
        // ṇic (tej-i). Ubhayapadī by 1.3.74 (*tejayati*). Slice 10k.
        dhatupatha: "10.0154",
        code: "tij",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "niSAne",
    },
    Dhatu {
        // 10.0156 `varDa~` CedanapUraRayoH (√vardh). Guru upadhā (the conjunct
        // `rD`), so unchanged before ṇic. Homograph of the ubhayapadī curādi
        // row `10.0313 vfDu~`: every form of this row is also the partner's
        // (its ṇic branch's). Ubhayapadī by 1.3.74 (*vardhayati*). Slice 10k.
        dhatupatha: "10.0156",
        code: "varD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "CedanapUraRayoH",
    },
    Dhatu {
        // 10.0161 `hlapa~` vyaktAyAM vAci (√hlap). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (hlAp-i). Ubhayapadī by
        // 1.3.74 (*hlāpayati*). Slice 10k.
        dhatupatha: "10.0161",
        code: "hlap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        // 10.0162 `klapa~` vyaktAyAM vAci (√klap). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (klAp-i). Ubhayapadī by
        // 1.3.74 (*klāpayati*). Slice 10k.
        dhatupatha: "10.0162",
        code: "klap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        // 10.0163 `hrapa~` vyaktAyAM vAci (√hrap). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (hrAp-i). Ubhayapadī by
        // 1.3.74 (*hrāpayati*). Slice 10k.
        dhatupatha: "10.0163",
        code: "hrap",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "vyaktAyAM vAci",
    },
    Dhatu {
        // 10.0165 `brIsa~` hiMsAyAm (√brīs). Long upadhā vowel `I`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*brīsayati*). Slice 10k.
        dhatupatha: "10.0165",
        code: "brIs",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0167 `ila~` preraRe (√il). 7.3.86 guṇates the laghu upadhā before
        // ṇic (el-i). Ubhayapadī by 1.3.74 (*elayati*). Slice 10k.
        dhatupatha: "10.0167",
        code: "il",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe",
    },
    Dhatu {
        // 10.0168 `mrakza~` mlecCane (√mrakṣ). Guru upadhā (the conjunct `kz`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*mrakṣayati*). Slice
        // 10k.
        dhatupatha: "10.0168",
        code: "mrakz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mlecCane",
    },
    Dhatu {
        // 10.0169 `asta~` saNGAte (√ast). Guru upadhā (the conjunct `st`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*astayati*). Slice 10k.
        dhatupatha: "10.0169",
        code: "ast",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0172 `brUsa~` hiMsAyAm (√brūs). Long upadhā vowel `U`, so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*brūsayati*). Slice 10k.
        dhatupatha: "10.0172",
        code: "brUs",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0173 `barha~` hiMsAyAm (√barh). Guru upadhā (the conjunct `rh`),
        // so unchanged before ṇic. Homograph of the ubhayapadī curādi row
        // `10.0300 barha~`: every form of this row is also the partner's (its
        // ṇic branch's). Ubhayapadī by 1.3.74 (*barhayati*). Slice 10k.
        dhatupatha: "10.0173",
        code: "barh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "hiMsAyAm",
    },
    Dhatu {
        // 10.0176 `bula~` nimajjane (√bul). 7.3.86 guṇates the laghu upadhā
        // before ṇic (bol-i). Ubhayapadī by 1.3.74 (*bolayati*). Slice 10k.
        dhatupatha: "10.0176",
        code: "bul",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nimajjane",
    },
    Dhatu {
        // 10.0177 `garja~` Sabde (√garj). Guru upadhā (the conjunct `rj`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*garjayati*). Slice 10k.
        dhatupatha: "10.0177",
        code: "garj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Sabde",
    },
    Dhatu {
        // 10.0178 `garda~` Sabde (√gard). Guru upadhā (the conjunct `rd`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*gardayati*). Slice 10k.
        dhatupatha: "10.0178",
        code: "gard",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Sabde",
    },
    Dhatu {
        // 10.0179 `garDa~` aBikANkzAyAm (√gardh). Guru upadhā (the conjunct
        // `rD`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*gardhayati*).
        // Slice 10k.
        dhatupatha: "10.0179",
        code: "garD",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "aBikANkzAyAm",
    },
    Dhatu {
        // 10.0181 `pUrva~` niketane (√pūrv). Guru upadhā (the conjunct `rv`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*pūrvayati*). Slice
        // 10k.
        dhatupatha: "10.0181",
        code: "pUrv",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "niketane",
    },
    Dhatu {
        // 10.0183 `Iqa~` stutO (√īḍ). Long upadhā vowel `I`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*īḍayati*). Slice 10k.
        dhatupatha: "10.0183",
        code: "Iq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "stutO",
    },
    Dhatu {
        // 10.0186 `parTa~` prakzepe (√parth). Guru upadhā (the conjunct `rT`),
        // so unchanged before ṇic. Shares every form with `10.0028 pfTa~`.
        // Ubhayapadī by 1.3.74 (*parthayati*). Slice 10k.
        dhatupatha: "10.0186",
        code: "parT",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "prakzepe",
    },
    Dhatu {
        // 10.0187 `ruza~` roze (√ruṣ). 7.3.86 guṇates the laghu upadhā before
        // ṇic (roz-i). Ubhayapadī by 1.3.74 (*roṣayati*). Slice 10k.
        dhatupatha: "10.0187",
        code: "ruz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "roze",
    },
    Dhatu {
        // 10.0188 `ruwa~` roze (√ruṭ). 7.3.86 guṇates the laghu upadhā before ṇic (row-i).
        // Homograph of the ubhayapadī curādi row `10.0314 ruwa~`: every form of this row is also
        // the partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*roṭayati*). Slice 10k.
        dhatupatha: "10.0188",
        code: "ruw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "roze",
    },
    Dhatu {
        // 10.0189 `qipa~` kzepe (√ḍip). 7.3.86 guṇates the laghu upadhā before
        // ṇic (qep-i). Shares its ātmanepada forms with the ākusmīya `10.0197
        // qipa~`. Ubhayapadī by 1.3.74 (*ḍepayati*). Slice 10k.
        dhatupatha: "10.0189",
        code: "qip",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzepe",
    },
    Dhatu {
        // 10.0190 `zwupa~` samucCrAye (√stup). 7.3.86 guṇates the laghu upadhā
        // before ṇic (stop-i). Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*stopayati*). Slice 10k.
        dhatupatha: "10.0190",
        code: "stup",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samucCrAye",
    },
    Dhatu {
        // 10.0191 `zwUpa~` samucCrAye (√stūp). Long upadhā vowel `U`, so
        // unchanged before ṇic. Stored per 6.1.64. Ubhayapadī by 1.3.74
        // (*stūpayati*). Slice 10k.
        dhatupatha: "10.0191",
        code: "stUp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "samucCrAye",
    },
    Dhatu {
        // 10.0237 `carca~` aDyayane (√carc). Guru upadhā (the conjunct `rc`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*carcayati*). Slice
        // 10k.
        dhatupatha: "10.0237",
        code: "carc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "aDyayane",
    },
    Dhatu {
        // 10.0238 `bukka~` BazaRe (√bukk). Guru upadhā (the conjunct `kk`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*bukkayati*). Slice 10k.
        dhatupatha: "10.0238",
        code: "bukk",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BazaRe",
    },
    Dhatu {
        // 10.0239 `Sabda~` AvizkAre, BazaRe (√śabd). Guru upadhā (the conjunct
        // `bd`), so unchanged before ṇic. Ubhayapadī by 1.3.74 (*śabdayati*).
        // Slice 10k.
        dhatupatha: "10.0239",
        code: "Sabd",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AvizkAre, BazaRe",
    },
    Dhatu {
        // 10.0240 `kaRa~` nimIlane (√kaṇ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (kAR-i). Ubhayapadī by 1.3.74 (*kāṇayati*).
        // Slice 10k.
        dhatupatha: "10.0240",
        code: "kaR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nimIlane",
    },
    Dhatu {
        // 10.0242 `zUda~` kzaraRe AsravaRe ApravaRe GAte ca (√sūd). Long upadhā
        // vowel `U`, so unchanged before ṇic. Stored per 6.1.64. Ubhayapadī by
        // 1.3.74 (*sūdayati*). Slice 10k.
        dhatupatha: "10.0242",
        code: "sUd",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kzaraRe AsravaRe ApravaRe GAte ca",
    },
    Dhatu {
        // 10.0244 `paSa~` banDane (√paś). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (pAS-i). Ubhayapadī by 1.3.74 (*pāśayati*).
        // Slice 10k.
        dhatupatha: "10.0244",
        code: "paS",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "banDane",
    },
    Dhatu {
        // 10.0245 `ama~` roge (√am). 7.2.116 ata upadhāyāḥ lengthens the `a`
        // upadhā before ṇit ṇic (Am-i). Ubhayapadī by 1.3.74 (*āmayati*). Slice
        // 10k.
        dhatupatha: "10.0245",
        code: "am",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "roge",
    },
    Dhatu {
        // 10.0246 `cawa~` Bedane (√caṭ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (cAw-i). Ubhayapadī by 1.3.74 (*cāṭayati*).
        // Slice 10k.
        dhatupatha: "10.0246",
        code: "caw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0247 `sPuwa~` Bedane (√sphuṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (sPow-i). Ubhayapadī by 1.3.74 (*sphoṭayati*). Slice 10k.
        dhatupatha: "10.0247",
        code: "sPuw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0248 `Gawa~` saNGAte (√ghaṭ). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (GAw-i). Homograph of the ubhayapadī
        // curādi row `10.0297 Gawa~`: every form of this row is also the
        // partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*ghāṭayati*).
        // Slice 10k.
        dhatupatha: "10.0248",
        code: "Gaw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNGAte",
    },
    Dhatu {
        // 10.0250 `arja~` pratiyatne (√arj). Guru upadhā (the conjunct `rj`),
        // so unchanged before ṇic. Ubhayapadī by 1.3.74 (*arjayati*). Slice
        // 10k.
        dhatupatha: "10.0250",
        code: "arj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pratiyatne",
    },
    Dhatu {
        // 10.0252 `kranda~` sAtatye (√krand). Guru upadhā (the conjunct `nd`),
        // so unchanged before ṇic. 8.3.24 makes its `n` an anusvāra before `d`,
        // and 8.4.58 restores `n`. Ubhayapadī by 1.3.74 (*krandayati*). Slice
        // 10k.
        dhatupatha: "10.0252",
        code: "krand",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sAtatye",
    },
    Dhatu {
        // 10.0253 `lasa~` Silpayoge (√las). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (lAs-i). Ubhayapadī by 1.3.74 (*lāsayati*).
        // Slice 10k.
        dhatupatha: "10.0253",
        code: "las",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Silpayoge",
    },
    Dhatu {
        // 10.0256 `mokza~` mocane (√mokṣ). Guru upadhā (the conjunct `kz`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*mokṣayati*). Slice 10k.
        dhatupatha: "10.0256",
        code: "mokz",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "mocane",
    },
    Dhatu {
        // 10.0257 `arha~` pUjAyAm (√arh). Guru upadhā (the conjunct `rh`), so unchanged before ṇic.
        // Homograph of the ubhayapadī curādi row `10.0367 arha~`: every form of this row is also
        // the partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*arhayati*). Slice 10k.
        dhatupatha: "10.0257",
        code: "arh",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUjAyAm",
    },
    Dhatu {
        // 10.0259 `Baja~` viSrARane (√bhaj). 7.2.116 ata upadhāyāḥ lengthens
        // the `a` upadhā before ṇit ṇic (BAj-i). Homograph of the ubhayapadī
        // curādi row `10.0428 BAja`: every form of this row is also the
        // partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*bhājayati*).
        // Slice 10k.
        dhatupatha: "10.0259",
        code: "Baj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "viSrARane",
    },
    Dhatu {
        // 10.0261 `yata~` nikAropaskArayoH (√yat). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (yAt-i). Ubhayapadī by 1.3.74
        // (*yātayati*). Slice 10k.
        dhatupatha: "10.0261",
        code: "yat",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "nikAropaskArayoH",
    },
    Dhatu {
        // 10.0262 `raka~` AsvAdane (√rak). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (rAk-i). Ubhayapadī by 1.3.74 (*rākayati*).
        // Slice 10k.
        dhatupatha: "10.0262",
        code: "rak",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0263 `laga~` AsvAdane (√lag). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (lAg-i). Ubhayapadī by 1.3.74 (*lāgayati*).
        // Slice 10k.
        dhatupatha: "10.0263",
        code: "lag",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0264 `raGa~` AsvAdane (√ragh). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (rAG-i). Ubhayapadī by 1.3.74
        // (*rāghayati*). Slice 10k.
        dhatupatha: "10.0264",
        code: "raG",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0265 `raga~` AsvAdane (√rag). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (rAg-i). Ubhayapadī by 1.3.74 (*rāgayati*).
        // Slice 10k.
        dhatupatha: "10.0265",
        code: "rag",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "AsvAdane",
    },
    Dhatu {
        // 10.0268 `muda~` saMsarge (√mud). 7.3.86 guṇates the laghu upadhā
        // before ṇic (mod-i). Ubhayapadī by 1.3.74 (*modayati*). Slice 10k.
        dhatupatha: "10.0268",
        code: "mud",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMsarge",
    },
    Dhatu {
        // 10.0269 `trasa~` DAraRe grahaRe vAraRe ca (√tras). 7.2.116 ata
        // upadhāyāḥ lengthens the `a` upadhā before ṇit ṇic (trAs-i).
        // Ubhayapadī by 1.3.74 (*trāsayati*). Slice 10k.
        dhatupatha: "10.0269",
        code: "tras",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "DAraRe grahaRe vAraRe ca",
    },
    Dhatu {
        // 10.0271 `uDrasa~` uYCe (√udhras). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (uDrAs-i). Ubhayapadī by 1.3.74
        // (*udhrāsayati*). Slice 10k.
        dhatupatha: "10.0271",
        code: "uDras",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "uYCe",
    },
    Dhatu {
        // 10.0272 `muca~` pramocane modane ca (√muc). 7.3.86 guṇates the laghu
        // upadhā before ṇic (moc-i). Ubhayapadī by 1.3.74 (*mocayati*). Slice
        // 10k.
        dhatupatha: "10.0272",
        code: "muc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pramocane modane ca",
    },
    Dhatu {
        // 10.0273 `vasa~` snehacCedApaharaRezu (√vas). 7.2.116 ata upadhāyāḥ
        // lengthens the `a` upadhā before ṇit ṇic (vAs-i). Homograph of the
        // ubhayapadī curādi row `10.0426 vAsa`: every form of this row is also
        // the partner's (its ṇic branch's). Ubhayapadī by 1.3.74 (*vāsayati*).
        // Slice 10k.
        dhatupatha: "10.0273",
        code: "vas",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "snehacCedApaharaRezu",
    },
    Dhatu {
        // 10.0274 `cara~` saMSaye (√car). 7.2.116 ata upadhāyāḥ lengthens the
        // `a` upadhā before ṇit ṇic (cAr-i). Ubhayapadī by 1.3.74 (*cārayati*).
        // Slice 10k.
        dhatupatha: "10.0274",
        code: "car",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMSaye",
    },
    Dhatu {
        // 10.0276 `cyusa~` sahane hasane ca (√cyus). 7.3.86 guṇates the laghu
        // upadhā before ṇic (cyos-i). Ubhayapadī by 1.3.74 (*cyosayati*). Slice
        // 10k.
        dhatupatha: "10.0276",
        code: "cyus",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "sahane hasane ca",
    },
    Dhatu {
        // 10.0397 `raNga~` gatO (√raṅg). Guru upadhā (the conjunct `Ng`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*raṅgayati*). Slice 10k.
        dhatupatha: "10.0397",
        code: "raNg",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "gatO",
    },
    Dhatu {
        // 10.0414 `Keqa~` BakzaRe (√kheḍ). Long upadhā vowel `e`, so unchanged
        // before ṇic. Ubhayapadī by 1.3.74 (*kheḍayati*). Slice 10k.
        dhatupatha: "10.0414",
        code: "Keq",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BakzaRe",
    },
    Dhatu {
        // 10.0438 `kURa~` saNkocane (√kūṇ). Long upadhā vowel `U`, so unchanged
        // before ṇic. Shares its ātmanepada forms with the ākusmīya `10.0211
        // kURa~`. Ubhayapadī by 1.3.74 (*kūṇayati*). Slice 10k.
        dhatupatha: "10.0438",
        code: "kUR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saNkocane",
    },
    Dhatu {
        // 10.0457 `karta~` SETilye (√kart). Guru upadhā (the conjunct `rt`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*kartayati*). Slice 10k.
        dhatupatha: "10.0457",
        code: "kart",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "SETilye",
    },
    Dhatu {
        // 10.0462 `Cuwa~` Cedane (√chuṭ). 7.3.86 guṇates the laghu upadhā
        // before ṇic (Cow-i). In laṅ the aṭ takes 6.1.73's tuk (acC-).
        // Ubhayapadī by 1.3.74 (*choṭayati*). Slice 10k.
        dhatupatha: "10.0462",
        code: "Cuw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Cedane",
    },
    Dhatu {
        // 10.0470 `karRa~` Bedane (√karṇ). Guru upadhā (the conjunct `rR`), so
        // unchanged before ṇic. Ubhayapadī by 1.3.74 (*karṇayati*). Slice 10k.
        dhatupatha: "10.0470",
        code: "karR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "Bedane",
    },
    Dhatu {
        // 10.0491 `ruWa~` BAzAyAm (√ruṭh). 7.3.86 guṇates the laghu upadhā
        // before ṇic (roW-i). Ubhayapadī by 1.3.74 (*roṭhayati*). Slice 10k.
        dhatupatha: "10.0491",
        code: "ruW",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "BAzAyAm",
    },
    Dhatu {
        // 10.0023 `urja~` balaprARanayoH (√ūrj). Guru upadhā (the conjunct
        // `rj`), so unchanged before ṇic; the tripādī 8.2.78 upadhāyāṃ ca
        // lengthens the `u` before the `r` upadhā (*ūrjayati*), but not in
        // laṅ, where 6.1.90 has merged it into the āṭ (*aurjayat*).
        // Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0023",
        code: "urj",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "balaprARanayoH",
    },
    Dhatu {
        // 10.0026 `curRa~` preraRe (√cūrṇ). Guru upadhā (the conjunct `rR`),
        // so unchanged before ṇic; 8.2.78 lengthens the `u` (*cūrṇayati*).
        // Homograph of the ubhayapadī `10.0143 cUrRa~`: every form of this
        // row is also that row's. Ubhayapadī by 1.3.74. The artha keeps
        // upstream's trailing space. Slice 10l.
        dhatupatha: "10.0026",
        code: "curR",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "preraRe ",
    },
    Dhatu {
        // 10.0037 `adwa~` anAdare (√aṭṭ). Guru upadhā (the conjunct `dw`),
        // so unchanged before ṇic; 8.4.41 retroflexes the `d` before the
        // `w`, and 8.4.55 makes it `w` (*aṭṭayati*). Ubhayapadī by 1.3.74.
        // Slice 10l.
        dhatupatha: "10.0037",
        code: "adw",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "anAdare",
    },
    Dhatu {
        // 10.0155 `kFta~` saMSabdane (√kṝt). 7.1.101 upadhāyāś ca makes the
        // `F` upadhā `ir` before ṇic (kirt-i), guru before `rt`, so 7.3.86
        // declines; 8.2.78 lengthens the `i` (*kīrtayati*). Ubhayapadī by
        // 1.3.74. Slice 10l.
        dhatupatha: "10.0155",
        code: "kFt",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "saMSabdane",
    },
    Dhatu {
        // 10.0170 `mleCa~` avyaktAyAM vAci (√mlecch). 6.1.75 dīrghāt adds tuk
        // after the long `e` before ṇic (mletC-i), and 8.4.40 makes it `c`
        // (*mlecchayati*). Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0170",
        code: "mleC",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avyaktAyAM vAci",
    },
    Dhatu {
        // 10.0180 `gurda~` pUrvaniketane (√gūrd). Guru upadhā (the conjunct
        // `rd`), so unchanged before ṇic; 8.2.78 lengthens the `u`
        // (*gūrdayati*). Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0180",
        code: "gurd",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "pUrvaniketane",
    },
    Dhatu {
        // 10.0270 `u~Drasa~` uYCe (√dhras). Udit by its initial `u~`, so its
        // ṇic is optional by Kaumudī 2570 (`OPTIONAL_NIC`). With ṇic, 7.2.116
        // lengthens the `a` upadhā (*dhrāsayati*), ubhayapadī by 1.3.74;
        // without, parasmaipadī by 1.3.78 (*dhrasati*). The next row,
        // `10.0271 uDrasa~`, has no `u~` and takes ṇic. Slice 10l.
        dhatupatha: "10.0270",
        code: "Dras",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "uYCe",
    },
    Dhatu {
        // 10.0278 `kfpa~` avakalkane (√kṛp). 7.3.86 guṇates the `f` before
        // ṇic (karp-i), and the tripādī 8.2.18 kṛpo ro laḥ makes it `kalp`
        // (*kalpayati*), keyed by row (`KRP`): `10.0408 kfpa` is another
        // root. Ubhayapadī by 1.3.74. Slice 10l.
        dhatupatha: "10.0278",
        code: "kfp",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "avakalkane",
    },
    Dhatu {
        // 10.0175 `picca~` kuwwane (√picc). Guru upadhā (the conjunct `cc`),
        // so unchanged before ṇic. 8.2.30 coH kuH declines on the first `c`:
        // it stands before a jhal, the second `c`, but inside the root, not
        // at a term's end (*piccayati*, not *pikcayati*). Ubhayapadī by
        // 1.3.74. Slice 10m.
        dhatupatha: "10.0175",
        code: "picc",
        gana: Gana::Curadi,
        pada: PadaAssignment::Nic,
        artha: "kuwwane",
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
    fn curated_roots_have_expected_ganas_and_padas() {
        assert_eq!(dhatus().len(), 594);
        let bu = dhatus().iter().find(|d| d.dhatupatha == "01.0001").unwrap();
        assert!(matches!(bu.pada, PadaAssignment::Parasmaipada));
        let labh = dhatus().iter().find(|d| d.dhatupatha == "01.1130").unwrap();
        assert!(matches!(labh.pada, PadaAssignment::Atmanepada));
        // Both vowel-initial atmanepadi roots must be present (they exercise
        // the AT-augment path 6.4.72/6.1.90).
        assert!(dhatus().iter().any(|d| d.dhatupatha == "01.0002"));
        assert!(dhatus().iter().any(|d| d.dhatupatha == "01.0694"));
        // Divadi/tudadi still present.
        let div = dhatus().iter().find(|d| d.dhatupatha == "04.0001").unwrap();
        assert!(matches!(div.gana, Gana::Divadi));
        let tud = dhatus().iter().find(|d| d.dhatupatha == "06.0001").unwrap();
        assert!(matches!(tud.gana, Gana::Tudadi));
        // New: adadi (gaṇa 2), both ā-final parasmaipada.
        let ya = dhatus().iter().find(|d| d.dhatupatha == "02.0044").unwrap();
        assert!(matches!(ya.gana, Gana::Adadi) && matches!(ya.pada, PadaAssignment::Parasmaipada));
        let va = dhatus().iter().find(|d| d.dhatupatha == "02.0045").unwrap();
        assert!(matches!(va.gana, Gana::Adadi) && matches!(va.pada, PadaAssignment::Parasmaipada));
        // adādi ātmanepada: √ās (slice 5d), √vas (slice 5e), and √śī (this slice) closes gaṇa.
        let as_ = dhatus().iter().find(|d| d.dhatupatha == "02.0011").unwrap();
        assert!(matches!(as_.gana, Gana::Adadi) && matches!(as_.pada, PadaAssignment::Atmanepada));
        let vas = dhatus().iter().find(|d| d.dhatupatha == "02.0013").unwrap();
        assert!(matches!(vas.gana, Gana::Adadi) && matches!(vas.pada, PadaAssignment::Atmanepada));
        // √vas ācchādane (2Ā), not √vas nivāse (1P) — artha disambiguates.
        assert_eq!(vas.artha, "AcCAdane");
        // adādi ātmanepada: √śī (this slice) closes the gaṇa.
        let shi = dhatus().iter().find(|d| d.dhatupatha == "02.0026").unwrap();
        assert!(matches!(shi.gana, Gana::Adadi) && matches!(shi.pada, PadaAssignment::Atmanepada));
        assert_eq!(shi.artha, "svapne");
        // kryādi (gaṇa 9), slice 9a: kliS/guD/aS, all parasmaipada.
        let klis = dhatus().iter().find(|d| d.dhatupatha == "09.0058").unwrap();
        assert!(
            matches!(klis.gana, Gana::Kryadi) && matches!(klis.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(klis.artha, "vibADane");
        let gud = dhatus().iter().find(|d| d.dhatupatha == "09.0053").unwrap();
        assert!(
            matches!(gud.gana, Gana::Kryadi) && matches!(gud.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(gud.artha, "roze");
        let ash = dhatus().iter().find(|d| d.dhatupatha == "09.0059").unwrap();
        assert!(
            matches!(ash.gana, Gana::Kryadi) && matches!(ash.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(ash.artha, "Bojane");
        // kryādi, slice 9b: muz/vrI parasmaipada, vf (√vṛṅ) atmanepada --
        // the gaṇa's only pure-atmanepadi root.
        let muz = dhatus().iter().find(|d| d.dhatupatha == "09.0066").unwrap();
        assert!(
            matches!(muz.gana, Gana::Kryadi) && matches!(muz.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(muz.artha, "steye");
        let vri = dhatus().iter().find(|d| d.dhatupatha == "09.0040").unwrap();
        assert!(
            matches!(vri.gana, Gana::Kryadi) && matches!(vri.pada, PadaAssignment::Parasmaipada)
        );
        assert_eq!(vri.artha, "varaRe");
        let vf = dhatus().iter().find(|d| d.dhatupatha == "09.0045").unwrap();
        assert!(matches!(vf.gana, Gana::Kryadi) && matches!(vf.pada, PadaAssignment::Atmanepada));
        assert_eq!(vf.artha, "samBaktO");
        // New: svādi (gaṇa 5), all four parasmaipadī.
        for number in ["05.0016", "05.0017", "05.0012", "05.0032"] {
            let d = dhatus().iter().find(|d| d.dhatupatha == number).unwrap();
            assert!(matches!(d.gana, Gana::Svadi));
            assert!(matches!(d.pada, PadaAssignment::Parasmaipada));
        }
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
            .find(|d| d.dhatupatha == "02.0001")
            .expect("√ad present");
        assert!(matches!(ad.gana, Gana::Adadi));
        assert!(matches!(ad.pada, PadaAssignment::Parasmaipada));
        assert_eq!(ad.artha, "BakzaRe");
    }

    #[test]
    fn as_is_registered_as_adadi_atmanepada() {
        let as_ = dhatus()
            .iter()
            .find(|d| d.dhatupatha == "02.0011")
            .expect("√ās present");
        assert!(matches!(as_.gana, Gana::Adadi));
        assert!(matches!(as_.pada, PadaAssignment::Atmanepada));
        assert_eq!(as_.artha, "upaveSane");
    }

    #[test]
    fn dhatupatha_is_the_key_and_is_unique() {
        let keys: Vec<&str> = dhatus().iter().map(|d| d.dhatupatha).collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            keys.len(),
            "dhātupāṭha numbers must be unique"
        );
        // Uniqueness here is a property of the source, not of a convention
        // this repo maintains: upstream numbers are unique across all 2259
        // entries, which `dhatupatha_numbers_resolve_upstream` also asserts.
        // That is the whole reason the number can serve as the key where the
        // SLP1 `code` could not — `code` is NOT unique (both √aś rows share
        // it), and the retired `Dhatu::id` existed only to paper over that.
        for d in dhatus() {
            assert_eq!(
                d.dhatupatha.len(),
                7,
                "{} is not a well-formed dhātupāṭha number",
                d.dhatupatha
            );
        }
    }

    #[test]
    fn the_two_ash_roots_are_distinct_rows() {
        let svadi = dhatus().iter().find(|d| d.dhatupatha == "05.0020").unwrap();
        let kryadi = dhatus().iter().find(|d| d.dhatupatha == "09.0059").unwrap();
        assert!(matches!(svadi.gana, Gana::Svadi));
        assert!(matches!(kryadi.gana, Gana::Kryadi));
        assert!(matches!(svadi.pada, PadaAssignment::Atmanepada));
        assert!(matches!(kryadi.pada, PadaAssignment::Parasmaipada));
        // Same surface text, different rows — and now distinct by
        // construction rather than by a hand-applied qualifier, since their
        // numbers come from different gaṇas of the source.
        assert_eq!(svadi.code, kryadi.code);
    }

    #[test]
    fn rudhadi_rows_are_the_twenty_five_curated_roots() {
        // √rudh, the gaṇa's eponym, arrived with 1.3.72 svaritaYitaH and
        // PadaAssignment::Ubhayapada. Slice 7c added √bhid, √kṣud, √yuj and
        // √tṛd; the 8.2.30/8.2.39 generalization slice added √ric and √vic
        // once 8.2.30 stopped hardcoding its `j` -> `g` pair.
        //
        // Slice 7d adds the eight roots that a probe against
        // vidyut-prakriya showed need NO sūtra this engine lacks: √śiṣ,
        // √und, √añj, √tañc, √vij, √vṛj, √pṛc and √vid. The probe compared
        // the sūtras each root's derivations invoke against this engine's
        // implemented set; `tools/audit/README.md`'s recorded result is
        // what turned that into a byte-for-byte verdict.
        //
        // `vi\da~\` and `o~vijI~` are the two entries whose SLP1 surfaces
        // WOULD have collided with divādi's `vid` and tudādi's `vij` under
        // the retired `id` scheme. Both are curated here, and under number
        // keying the question does not arise: `07.0013` and `07.0023` are
        // distinct from `04.0067` and `06.0009` whether or not their
        // surfaces agree. This is the slice that would have paid for the
        // retired scheme, and does not.
        //
        // After slice 7d the gaṇa was still PARTIAL: 21 of its 25
        // dhātupāṭha roots, so FOUR remained out, and they did not all
        // cost the same. √tṛh wanted 7.3.92 tfRaha im with 8.2.31 ho QaH
        // and 8.3.13 Qo Qe lopaH -- slice 7e curated it, below. √chid and
        // √chṛd still want 6.1.73 Ce ca plus 8.4.40 stoH ScunA ScuH.
        // And √bhuj is out on different grounds again: 1.3.66 Bujo'navane
        // forks its pada on sense.
        //
        // Slice 7e adds √tṛh, the ninth and last of the "reachable
        // non-ubhayapadī" roots 7d's probe separated out. It was the one
        // that did NOT come free: 7.3.92 tfRaha im, 8.2.31 ho QaH and
        // 8.3.13 Qo Qe lopaH are all new in this slice, and 8.4.41, 8.2.41
        // and 6.1.87 all had to widen to carry it. Parasmaipada: `tfha~`
        // carries no anudātta and no ñi, so 1.3.78 SezAt kartari
        // parasmaipadam settles it, and vidyut-prakriya derives no
        // ātmanepada forms for the entry.
        //
        // Slice 7f adds √chid and √chṛd, the last two ubhayapadī roots and
        // the last two that needed a sūtra: 6.1.73 Ce ca puts the tuk after
        // laṅ's aṭ-augment before their initial `C`, and 8.4.40 stoH ScunA
        // ScuH makes that `t` a `c` -- acCinat, acCfRat. Neither root needed
        // anything else: √chid is √bhid with a `C` for its `B`, and √chṛd is
        // √tṛd with a `C` for its `t`, ṇatva included, so every cell outside
        // laṅ derives on rules that were already in the pipeline.
        //
        // The √bhuj slice then closed the gaṇa at TWENTY-FIVE OF
        // TWENTY-FIVE: √bhuj (`07.0017`) needed no phonology at all — it
        // is √yuj with a `B` — and exactly one rule, 1.3.66 Bujo'navane, a
        // root-keyed pada assignment structurally identical to 1.3.72's.
        // 1.3.66 restricts ātmanepada to senses other than protecting, and
        // neither engine models sense, so the row ships an UNCONDITIONAL
        // ubhayapada assignment (PadaAssignment::UbhayapadaAnavane) with
        // anavane recorded as unimplemented — the same precedent 1.3.72
        // set for kartrabhiprAye kriyAPale. No rudhādi entry remains out.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Rudhadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("07.0010", "kft", PadaAssignment::Parasmaipada),
                ("07.0019", "hins", PadaAssignment::Parasmaipada),
                ("07.0012", "Kid", PadaAssignment::Atmanepada),
                ("07.0016", "Banj", PadaAssignment::Parasmaipada),
                ("07.0015", "piz", PadaAssignment::Parasmaipada),
                ("07.0011", "inD", PadaAssignment::Atmanepada),
                ("07.0001", "ruD", PadaAssignment::Ubhayapada),
                ("07.0002", "Bid", PadaAssignment::Ubhayapada),
                ("07.0006", "kzud", PadaAssignment::Ubhayapada),
                ("07.0007", "yuj", PadaAssignment::Ubhayapada),
                ("07.0009", "tfd", PadaAssignment::Ubhayapada),
                ("07.0004", "ric", PadaAssignment::Ubhayapada),
                ("07.0005", "vic", PadaAssignment::Ubhayapada),
                ("07.0014", "Siz", PadaAssignment::Parasmaipada),
                ("07.0020", "und", PadaAssignment::Parasmaipada),
                ("07.0021", "anj", PadaAssignment::Parasmaipada),
                ("07.0022", "tanc", PadaAssignment::Parasmaipada),
                ("07.0023", "vij", PadaAssignment::Parasmaipada),
                ("07.0024", "vfj", PadaAssignment::Parasmaipada),
                ("07.0025", "pfc", PadaAssignment::Parasmaipada),
                ("07.0013", "vid", PadaAssignment::Atmanepada),
                ("07.0018", "tfh", PadaAssignment::Parasmaipada),
                ("07.0003", "Cid", PadaAssignment::Ubhayapada),
                ("07.0008", "Cfd", PadaAssignment::Ubhayapada),
                ("07.0017", "Buj", PadaAssignment::UbhayapadaAnavane),
            ]
        );
    }

    #[test]
    fn tanadi_rows_are_the_ten_curated_roots() {
        // The gaṇa closes at 10/10 in this slice. Seven svarita-it (1.3.72,
        // ubhayapadī) and two anudātta (1.3.12, ātmanepadī) ride 3.1.79's
        // bare u; √kṛ is ñit (`qukf\Y` — the `qu` is a ḍu-it, 1.3.5; the
        // `\` accent grave sits on the root vowel; the final `Y` is what
        // 1.3.72 reads) → ubhayapadī: karoti / kurute.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Tanadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("08.0001", "tan", PadaAssignment::Ubhayapada),
                ("08.0002", "san", PadaAssignment::Ubhayapada),
                ("08.0003", "kzaR", PadaAssignment::Ubhayapada),
                ("08.0004", "kziR", PadaAssignment::Ubhayapada),
                ("08.0005", "fR", PadaAssignment::Ubhayapada),
                ("08.0006", "tfR", PadaAssignment::Ubhayapada),
                ("08.0007", "GfR", PadaAssignment::Ubhayapada),
                ("08.0008", "van", PadaAssignment::Atmanepada),
                ("08.0009", "man", PadaAssignment::Atmanepada),
                ("08.0010", "kf", PadaAssignment::Ubhayapada),
            ]
        );
    }

    #[test]
    fn curadi_rows_are_the_four_hundred_ninety_one_curated_roots() {
        // Slice 10a opens gaṇa 10 with four roots that need only ṇic
        // (3.1.25), 3.1.32 and the guṇa/vṛddhi before ṇic: √cur (7.3.86),
        // √laḍ (7.2.116), √bhakṣ and √bhūṣ (neither). None carries a pada
        // marker; all four are ubhayapadī by 1.3.74 ṇicaś ca. Slice 10b adds
        // four ākusmīya roots, ātmanepadī by 10.0496 ā kusmād ātmanepadinaḥ:
        // √cit and √vṛṣ (7.3.86), √mad (7.2.116), √kusm (neither). Slice 10c
        // adds thirty-three more ākusmīya rows that need nothing
        // new: six by 7.3.86, ten by 7.2.116, seventeen unchanged before ṇic.
        // Slice 10d adds six of the seven jñapādi (`JNAPADI`), mit by the
        // gaṇasūtra 10.0493, whose upadhā 6.4.92 shortens back after
        // 7.2.116: √jñap, √yam, √cah, √cap, √rah, √bal, ubhayapadī by 1.3.74.
        // Slice 10e adds ninety-two adanta roots (`10.0108 mArga` and
        // `10.0389 kaTa` … `10.0492 Deka`): 6.4.48 deletes the final `a`
        // before ṇic, and 7.2.116 / 7.3.86 decline on its `Tag::AtLopa`.
        // Eighty-three are ubhayapadī by 1.3.74; the nine ā-garvīya
        // (`AA_GARVIYA`) are ātmanepadī by 10.0497. Slice 10f adds the ten
        // rows whose ṇic is optional (`OPTIONAL_NIC`): six ākusmīya, `garva`,
        // `mUtra`, `katra` and `pata`, each curated with its ṇic branch's
        // pada. Slice 10g adds fifty-nine more, every idit (Kaumudī 2564) and
        // ñit/udit (2570) row outside the āsvadīya and ādhṛṣīya but `10.0124 ciY`,
        // all ubhayapadī by 1.3.74 with ṇic. Slice 10h adds fifty of the
        // fifty-one ādhṛṣīya (10.0498, all but `10.0368 za\da~`), ubhayapadī by
        // 1.3.74 with ṇic; the six svarita or ñit among them are
        // `NicUbhayapada`, ubhayapadī by 1.3.72 without it too. Slice 10i adds
        // the fifty-nine āsvadīya (10.0499), `10.0022 pF` (Kaumudī 2565) and
        // `10.0251 Guzi~r` (2571), ubhayapadī by 1.3.74 with ṇic. Slice 10j
        // adds the eight ajanta rows 7.2.115, 6.1.54 and 7.3.36 reach: √ghṛ,
        // √jñā, √cyu and √bhū, ubhayapadī by 1.3.74; √gṛ and √yu, the last two
        // ākusmīya, by 10.0496; √ci, the last jñapādi, `NicUbhayapada` with
        // ṇic optional by 2570; and √smiṅ, ātmanepadī under ṇic by Kaumudī
        // 2567. Slice 10k adds the 155 plain obligatory-ṇic rows that need
        // nothing new: thirty-seven by 7.3.86, forty by 7.2.116 and
        // seventy-eight unchanged before ṇic, all ubhayapadī by 1.3.74. Slice
        // 10l adds eight rows that bring rules of their own: √ūrj, √cūrṇ and
        // √gūrd (8.2.78), √aṭṭ (8.4.41's stu-before-ṭu arm), √kṝt (7.1.101,
        // then 8.2.78), √mlecch (6.1.75), √dhras (udit by its initial `u~`,
        // ṇic optional by 2570) and √kṛp (8.2.18), all ubhayapadī by 1.3.74.
        // Slice 10m adds √picc, whose root-internal `cc` 8.2.30 no longer
        // velarises, ubhayapadī by 1.3.74. The gaṇa is OPEN at 491 of its 492
        // dhātus; the one out is `10.0368 za\da~`, which waits for upasargas
        // (`curadi_has_492_dhatus_and_only_zad_is_uncurated`).
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Curadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("10.0001", "cur", PadaAssignment::Nic),
                ("10.0010", "laq", PadaAssignment::Nic),
                ("10.0033", "Bakz", PadaAssignment::Nic),
                ("10.0255", "BUz", PadaAssignment::Nic),
                ("10.0118", "jYap", PadaAssignment::Nic),
                ("10.0119", "yam", PadaAssignment::Nic),
                ("10.0120", "cah", PadaAssignment::Nic),
                ("10.0121", "cap", PadaAssignment::Nic),
                ("10.0122", "rah", PadaAssignment::Nic),
                ("10.0123", "bal", PadaAssignment::Nic),
                ("10.0192", "cit", PadaAssignment::Akusmiya),
                ("10.0195", "das", PadaAssignment::Akusmiya),
                ("10.0196", "qap", PadaAssignment::Akusmiya),
                ("10.0197", "qip", PadaAssignment::Akusmiya),
                ("10.0200", "spaS", PadaAssignment::Akusmiya),
                ("10.0201", "tarj", PadaAssignment::Akusmiya),
                ("10.0202", "Barts", PadaAssignment::Akusmiya),
                ("10.0203", "bast", PadaAssignment::Akusmiya),
                ("10.0204", "ganD", PadaAssignment::Akusmiya),
                ("10.0205", "kil", PadaAssignment::Akusmiya),
                ("10.0206", "pil", PadaAssignment::Akusmiya),
                ("10.0207", "vizk", PadaAssignment::Akusmiya),
                ("10.0208", "hizk", PadaAssignment::Akusmiya),
                ("10.0209", "nizk", PadaAssignment::Akusmiya),
                ("10.0210", "lal", PadaAssignment::Akusmiya),
                ("10.0211", "kUR", PadaAssignment::Akusmiya),
                ("10.0212", "tUR", PadaAssignment::Akusmiya),
                ("10.0213", "BrUR", PadaAssignment::Akusmiya),
                ("10.0214", "SaW", PadaAssignment::Akusmiya),
                ("10.0215", "yakz", PadaAssignment::Akusmiya),
                ("10.0216", "syam", PadaAssignment::Akusmiya),
                ("10.0217", "gUr", PadaAssignment::Akusmiya),
                ("10.0218", "Sam", PadaAssignment::Akusmiya),
                ("10.0219", "lakz", PadaAssignment::Akusmiya),
                ("10.0220", "kuts", PadaAssignment::Akusmiya),
                ("10.0221", "truw", PadaAssignment::Akusmiya),
                ("10.0222", "kuw", PadaAssignment::Akusmiya),
                ("10.0223", "gal", PadaAssignment::Akusmiya),
                ("10.0224", "Bal", PadaAssignment::Akusmiya),
                ("10.0225", "kUw", PadaAssignment::Akusmiya),
                ("10.0226", "kuww", PadaAssignment::Akusmiya),
                ("10.0228", "vfz", PadaAssignment::Akusmiya),
                ("10.0229", "mad", PadaAssignment::Akusmiya),
                ("10.0232", "vid", PadaAssignment::Akusmiya),
                ("10.0233", "mAn", PadaAssignment::Akusmiya),
                ("10.0234", "man", PadaAssignment::Akusmiya),
                ("10.0236", "kusm", PadaAssignment::Akusmiya),
                ("10.0108", "mArga", PadaAssignment::Nic),
                ("10.0389", "kaTa", PadaAssignment::Nic),
                ("10.0390", "vara", PadaAssignment::Nic),
                ("10.0391", "gaRa", PadaAssignment::Nic),
                ("10.0392", "SaWa", PadaAssignment::Nic),
                ("10.0393", "SvaWa", PadaAssignment::Nic),
                ("10.0394", "paWa", PadaAssignment::Nic),
                ("10.0395", "vaWa", PadaAssignment::Nic),
                ("10.0396", "raha", PadaAssignment::Nic),
                ("10.0398", "stana", PadaAssignment::Nic),
                ("10.0399", "gada", PadaAssignment::Nic),
                ("10.0401", "paza", PadaAssignment::Nic),
                ("10.0402", "svara", PadaAssignment::Nic),
                ("10.0403", "raca", PadaAssignment::Nic),
                ("10.0404", "kala", PadaAssignment::Nic),
                ("10.0405", "caha", PadaAssignment::Nic),
                ("10.0406", "maha", PadaAssignment::Nic),
                ("10.0407", "sAra", PadaAssignment::Nic),
                ("10.0408", "kfpa", PadaAssignment::Nic),
                ("10.0409", "SraTa", PadaAssignment::Nic),
                ("10.0410", "spfha", PadaAssignment::Nic),
                ("10.0411", "BAma", PadaAssignment::Nic),
                ("10.0412", "sUca", PadaAssignment::Nic),
                ("10.0413", "Kewa", PadaAssignment::Nic),
                ("10.0415", "Kowa", PadaAssignment::Nic),
                ("10.0416", "kzowa", PadaAssignment::Nic),
                ("10.0417", "goma", PadaAssignment::Nic),
                ("10.0418", "kumAra", PadaAssignment::Nic),
                ("10.0419", "SIla", PadaAssignment::Nic),
                ("10.0420", "sAma", PadaAssignment::Nic),
                ("10.0421", "vela", PadaAssignment::Nic),
                ("10.0422", "kAla", PadaAssignment::Nic),
                ("10.0423", "palpUla", PadaAssignment::Nic),
                ("10.0424", "vAta", PadaAssignment::Nic),
                ("10.0425", "gaveza", PadaAssignment::Nic),
                ("10.0426", "vAsa", PadaAssignment::Nic),
                ("10.0427", "nivAsa", PadaAssignment::Nic),
                ("10.0428", "BAja", PadaAssignment::Nic),
                ("10.0429", "saBAja", PadaAssignment::Nic),
                ("10.0430", "Una", PadaAssignment::Nic),
                ("10.0431", "Dvana", PadaAssignment::Nic),
                ("10.0432", "kUwa", PadaAssignment::Nic),
                ("10.0433", "sanketa", PadaAssignment::Nic),
                ("10.0434", "grAma", PadaAssignment::Nic),
                ("10.0435", "kuRa", PadaAssignment::Nic),
                ("10.0436", "guRa", PadaAssignment::Nic),
                ("10.0437", "keta", PadaAssignment::Nic),
                ("10.0439", "stena", PadaAssignment::Nic),
                ("10.0440", "pada", PadaAssignment::AaGarviya),
                ("10.0441", "gfha", PadaAssignment::AaGarviya),
                ("10.0442", "mfga", PadaAssignment::AaGarviya),
                ("10.0443", "kuha", PadaAssignment::AaGarviya),
                ("10.0444", "SUra", PadaAssignment::AaGarviya),
                ("10.0445", "vIra", PadaAssignment::AaGarviya),
                ("10.0446", "sTUla", PadaAssignment::AaGarviya),
                ("10.0447", "arTa", PadaAssignment::AaGarviya),
                ("10.0448", "satra", PadaAssignment::AaGarviya),
                ("10.0450", "sUtra", PadaAssignment::Nic),
                ("10.0452", "rUkza", PadaAssignment::Nic),
                ("10.0453", "pAra", PadaAssignment::Nic),
                ("10.0454", "tIra", PadaAssignment::Nic),
                ("10.0455", "puwa", PadaAssignment::Nic),
                ("10.0458", "valka", PadaAssignment::Nic),
                ("10.0459", "citra", PadaAssignment::Nic),
                ("10.0460", "ansa", PadaAssignment::Nic),
                ("10.0461", "vawa", PadaAssignment::Nic),
                ("10.0463", "laja", PadaAssignment::Nic),
                ("10.0466", "miSra", PadaAssignment::Nic),
                ("10.0467", "sangrAma", PadaAssignment::Nic),
                ("10.0468", "stoma", PadaAssignment::Nic),
                ("10.0469", "Cidra", PadaAssignment::Nic),
                ("10.0471", "anDa", PadaAssignment::Nic),
                ("10.0472", "danqa", PadaAssignment::Nic),
                ("10.0473", "anka", PadaAssignment::Nic),
                ("10.0474", "anga", PadaAssignment::Nic),
                ("10.0475", "suKa", PadaAssignment::Nic),
                ("10.0476", "duHKa", PadaAssignment::Nic),
                ("10.0477", "rasa", PadaAssignment::Nic),
                ("10.0478", "vyaya", PadaAssignment::Nic),
                ("10.0479", "rUpa", PadaAssignment::Nic),
                ("10.0480", "Ceda", PadaAssignment::Nic),
                ("10.0481", "Cada", PadaAssignment::Nic),
                ("10.0482", "lABa", PadaAssignment::Nic),
                ("10.0483", "vraRa", PadaAssignment::Nic),
                ("10.0484", "varRa", PadaAssignment::Nic),
                ("10.0485", "parRa", PadaAssignment::Nic),
                ("10.0486", "vizka", PadaAssignment::Nic),
                ("10.0487", "kzipa", PadaAssignment::Nic),
                ("10.0488", "vasa", PadaAssignment::Nic),
                ("10.0489", "tutTa", PadaAssignment::Nic),
                ("10.0490", "palyUla", PadaAssignment::Nic),
                ("10.0492", "Deka", PadaAssignment::Nic),
                ("10.0193", "danS", PadaAssignment::Akusmiya),
                ("10.0194", "dans", PadaAssignment::Akusmiya),
                ("10.0198", "tantr", PadaAssignment::Akusmiya),
                ("10.0199", "mantr", PadaAssignment::Akusmiya),
                ("10.0227", "vanc", PadaAssignment::Akusmiya),
                ("10.0230", "div", PadaAssignment::Akusmiya),
                ("10.0400", "pata", PadaAssignment::Nic),
                ("10.0449", "garva", PadaAssignment::AaGarviya),
                ("10.0451", "mUtra", PadaAssignment::Nic),
                ("10.0456", "katra", PadaAssignment::Nic),
                ("10.0002", "cint", PadaAssignment::Nic),
                ("10.0003", "yantr", PadaAssignment::Nic),
                ("10.0004", "sPunq", PadaAssignment::Nic),
                ("10.0005", "sPunw", PadaAssignment::Nic),
                ("10.0007", "kundr", PadaAssignment::Nic),
                ("10.0009", "spunq", PadaAssignment::Nic),
                ("10.0011", "mind", PadaAssignment::Nic),
                ("10.0013", "lanq", PadaAssignment::Nic),
                ("10.0014", "olanq", PadaAssignment::Nic),
                ("10.0043", "SvanW", PadaAssignment::Nic),
                ("10.0045", "tunj", PadaAssignment::Nic),
                ("10.0047", "pinj", PadaAssignment::Nic),
                ("10.0048", "lanj", PadaAssignment::Nic),
                ("10.0049", "lunj", PadaAssignment::Nic),
                ("10.0060", "panT", PadaAssignment::Nic),
                ("10.0062", "Cand", PadaAssignment::Nic),
                ("10.0066", "Kanq", PadaAssignment::Nic),
                ("10.0067", "kanq", PadaAssignment::Nic),
                ("10.0068", "kunq", PadaAssignment::Nic),
                ("10.0069", "gunq", PadaAssignment::Nic),
                ("10.0070", "kunW", PadaAssignment::Nic),
                ("10.0071", "gunW", PadaAssignment::Nic),
                ("10.0072", "Kunq", PadaAssignment::Nic),
                ("10.0073", "vanw", PadaAssignment::Nic),
                ("10.0074", "vanq", PadaAssignment::Nic),
                ("10.0075", "canq", PadaAssignment::Nic),
                ("10.0076", "manq", PadaAssignment::Nic),
                ("10.0077", "Banq", PadaAssignment::Nic),
                ("10.0105", "ulanq", PadaAssignment::Nic),
                ("10.0106", "panq", PadaAssignment::Nic),
                ("10.0107", "pans", PadaAssignment::Nic),
                ("10.0111", "canp", PadaAssignment::Nic),
                ("10.0112", "kzanp", PadaAssignment::Nic),
                ("10.0113", "kzanj", PadaAssignment::Nic),
                ("10.0114", "Canj", PadaAssignment::Nic),
                ("10.0130", "cunb", PadaAssignment::Nic),
                ("10.0135", "wank", PadaAssignment::Nic),
                ("10.0147", "SunW", PadaAssignment::Nic),
                ("10.0153", "panc", PadaAssignment::Nic),
                ("10.0157", "kunb", PadaAssignment::Nic),
                ("10.0158", "kunB", PadaAssignment::Nic),
                ("10.0159", "lunb", PadaAssignment::Nic),
                ("10.0160", "tunb", PadaAssignment::Nic),
                ("10.0164", "cunw", PadaAssignment::Nic),
                ("10.0166", "danh", PadaAssignment::Nic),
                ("10.0171", "Canp", PadaAssignment::Nic),
                ("10.0174", "SraR", PadaAssignment::Nic),
                ("10.0182", "jans", PadaAssignment::Nic),
                ("10.0184", "jas", PadaAssignment::Nic),
                ("10.0185", "pinq", PadaAssignment::Nic),
                ("10.0241", "janB", PadaAssignment::Nic),
                ("10.0243", "jas", PadaAssignment::Nic),
                ("10.0249", "div", PadaAssignment::Nic),
                ("10.0254", "tans", PadaAssignment::Nic),
                ("10.0260", "SfD", PadaAssignment::Nic),
                ("10.0266", "anc", PadaAssignment::Nic),
                ("10.0267", "ling", PadaAssignment::Nic),
                ("10.0464", "vanw", PadaAssignment::Nic),
                ("10.0465", "lanj", PadaAssignment::Nic),
                ("10.0338", "yuj", PadaAssignment::Nic),
                ("10.0339", "pfc", PadaAssignment::Nic),
                ("10.0340", "arc", PadaAssignment::Nic),
                ("10.0341", "sah", PadaAssignment::Nic),
                ("10.0342", "Ir", PadaAssignment::Nic),
                ("10.0343", "lI", PadaAssignment::Nic),
                ("10.0344", "vfj", PadaAssignment::Nic),
                ("10.0345", "vf", PadaAssignment::NicUbhayapada),
                ("10.0346", "jF", PadaAssignment::Nic),
                ("10.0347", "jri", PadaAssignment::Nic),
                ("10.0348", "ric", PadaAssignment::Nic),
                ("10.0349", "Siz", PadaAssignment::Nic),
                ("10.0350", "tap", PadaAssignment::Nic),
                ("10.0351", "tfp", PadaAssignment::Nic),
                ("10.0352", "Cfd", PadaAssignment::Nic),
                ("10.0353", "cfp", PadaAssignment::Nic),
                ("10.0354", "Cfp", PadaAssignment::Nic),
                ("10.0355", "tfp", PadaAssignment::Nic),
                ("10.0356", "dfp", PadaAssignment::Nic),
                ("10.0357", "dfB", PadaAssignment::Nic),
                ("10.0358", "dfB", PadaAssignment::Nic),
                ("10.0359", "law", PadaAssignment::Nic),
                ("10.0360", "SraT", PadaAssignment::Nic),
                ("10.0361", "mI", PadaAssignment::Nic),
                ("10.0362", "granT", PadaAssignment::Nic),
                ("10.0363", "SIk", PadaAssignment::Nic),
                ("10.0364", "cIk", PadaAssignment::Nic),
                ("10.0365", "ard", PadaAssignment::NicUbhayapada),
                ("10.0366", "hins", PadaAssignment::Nic),
                ("10.0367", "arh", PadaAssignment::Nic),
                ("10.0369", "SunD", PadaAssignment::Nic),
                ("10.0370", "Cad", PadaAssignment::Nic),
                ("10.0371", "juz", PadaAssignment::Nic),
                ("10.0372", "DU", PadaAssignment::NicUbhayapada),
                ("10.0373", "prI", PadaAssignment::NicUbhayapada),
                ("10.0374", "SranT", PadaAssignment::Nic),
                ("10.0375", "granT", PadaAssignment::Nic),
                ("10.0376", "Ap", PadaAssignment::Nic),
                ("10.0377", "tan", PadaAssignment::Nic),
                ("10.0378", "can", PadaAssignment::Nic),
                ("10.0379", "vad", PadaAssignment::NicUbhayapada),
                ("10.0380", "vac", PadaAssignment::Nic),
                ("10.0381", "mAn", PadaAssignment::Nic),
                ("10.0382", "BU", PadaAssignment::Nic),
                ("10.0383", "garh", PadaAssignment::Nic),
                ("10.0384", "mArg", PadaAssignment::Nic),
                ("10.0385", "kanW", PadaAssignment::Nic),
                ("10.0386", "mfj", PadaAssignment::Nic),
                ("10.0387", "mfz", PadaAssignment::NicUbhayapada),
                ("10.0388", "Dfz", PadaAssignment::Nic),
                ("10.0022", "pF", PadaAssignment::Nic),
                ("10.0251", "Guz", PadaAssignment::Nic),
                ("10.0279", "gras", PadaAssignment::Nic),
                ("10.0280", "puz", PadaAssignment::Nic),
                ("10.0281", "dal", PadaAssignment::Nic),
                ("10.0282", "paw", PadaAssignment::Nic),
                ("10.0283", "puw", PadaAssignment::Nic),
                ("10.0284", "luw", PadaAssignment::Nic),
                ("10.0285", "tunj", PadaAssignment::Nic),
                ("10.0286", "minj", PadaAssignment::Nic),
                ("10.0287", "pinj", PadaAssignment::Nic),
                ("10.0288", "lak", PadaAssignment::Nic),
                ("10.0289", "lunj", PadaAssignment::Nic),
                ("10.0290", "Banj", PadaAssignment::Nic),
                ("10.0291", "lanG", PadaAssignment::Nic),
                ("10.0292", "trans", PadaAssignment::Nic),
                ("10.0293", "pins", PadaAssignment::Nic),
                ("10.0294", "kuns", PadaAssignment::Nic),
                ("10.0295", "danS", PadaAssignment::Nic),
                ("10.0296", "kunS", PadaAssignment::Nic),
                ("10.0297", "Gaw", PadaAssignment::Nic),
                ("10.0298", "Ganw", PadaAssignment::Nic),
                ("10.0299", "bfnh", PadaAssignment::Nic),
                ("10.0300", "barh", PadaAssignment::Nic),
                ("10.0301", "balh", PadaAssignment::Nic),
                ("10.0302", "gup", PadaAssignment::Nic),
                ("10.0303", "DUp", PadaAssignment::Nic),
                ("10.0304", "viC", PadaAssignment::Nic),
                ("10.0305", "cIv", PadaAssignment::Nic),
                ("10.0306", "puT", PadaAssignment::Nic),
                ("10.0307", "lok", PadaAssignment::Nic),
                ("10.0308", "loc", PadaAssignment::Nic),
                ("10.0309", "nad", PadaAssignment::Nic),
                ("10.0310", "kup", PadaAssignment::Nic),
                ("10.0311", "tark", PadaAssignment::Nic),
                ("10.0312", "vft", PadaAssignment::Nic),
                ("10.0313", "vfD", PadaAssignment::Nic),
                ("10.0314", "ruw", PadaAssignment::Nic),
                ("10.0315", "lanj", PadaAssignment::Nic),
                ("10.0316", "anj", PadaAssignment::Nic),
                ("10.0317", "dans", PadaAssignment::Nic),
                ("10.0318", "BfnS", PadaAssignment::Nic),
                ("10.0319", "runS", PadaAssignment::Nic),
                ("10.0320", "SIk", PadaAssignment::Nic),
                ("10.0321", "runs", PadaAssignment::Nic),
                ("10.0322", "nanw", PadaAssignment::Nic),
                ("10.0323", "punw", PadaAssignment::Nic),
                ("10.0324", "ji", PadaAssignment::Nic),
                ("10.0325", "ci", PadaAssignment::Nic),
                ("10.0326", "ranD", PadaAssignment::Nic),
                ("10.0327", "lanG", PadaAssignment::Nic),
                ("10.0328", "anh", PadaAssignment::Nic),
                ("10.0329", "ranh", PadaAssignment::Nic),
                ("10.0330", "manh", PadaAssignment::Nic),
                ("10.0331", "lanq", PadaAssignment::Nic),
                ("10.0332", "taq", PadaAssignment::Nic),
                ("10.0333", "nal", PadaAssignment::Nic),
                ("10.0334", "pUr", PadaAssignment::Nic),
                ("10.0335", "ruj", PadaAssignment::Nic),
                ("10.0336", "svad", PadaAssignment::Nic),
                ("10.0337", "svAd", PadaAssignment::Nic),
                ("10.0058", "smi", PadaAssignment::Atmanepada),
                ("10.0124", "ci", PadaAssignment::NicUbhayapada),
                ("10.0152", "Gf", PadaAssignment::Nic),
                ("10.0231", "gf", PadaAssignment::Akusmiya),
                ("10.0235", "yu", PadaAssignment::Akusmiya),
                ("10.0258", "jYA", PadaAssignment::Nic),
                ("10.0275", "cyu", PadaAssignment::Nic),
                ("10.0277", "BU", PadaAssignment::Nic),
                ("10.0006", "lakz", PadaAssignment::Nic),
                ("10.0008", "kud", PadaAssignment::Nic),
                ("10.0012", "mid", PadaAssignment::Nic),
                ("10.0015", "jal", PadaAssignment::Nic),
                ("10.0016", "laj", PadaAssignment::Nic),
                ("10.0017", "pIq", PadaAssignment::Nic),
                ("10.0018", "naw", PadaAssignment::Nic),
                ("10.0019", "SraT", PadaAssignment::Nic),
                ("10.0020", "baD", PadaAssignment::Nic),
                ("10.0021", "banD", PadaAssignment::Nic),
                ("10.0024", "pakz", PadaAssignment::Nic),
                ("10.0025", "varR", PadaAssignment::Nic),
                ("10.0027", "praT", PadaAssignment::Nic),
                ("10.0028", "pfT", PadaAssignment::Nic),
                ("10.0029", "paT", PadaAssignment::Nic),
                ("10.0030", "sanb", PadaAssignment::Nic),
                ("10.0031", "Sanb", PadaAssignment::Nic),
                ("10.0032", "sAnb", PadaAssignment::Nic),
                ("10.0034", "kuww", PadaAssignment::Nic),
                ("10.0035", "puww", PadaAssignment::Nic),
                ("10.0036", "cuww", PadaAssignment::Nic),
                ("10.0038", "suww", PadaAssignment::Nic),
                ("10.0039", "lunw", PadaAssignment::Nic),
                ("10.0040", "lunW", PadaAssignment::Nic),
                ("10.0041", "SaW", PadaAssignment::Nic),
                ("10.0042", "SvaW", PadaAssignment::Nic),
                ("10.0044", "tuj", PadaAssignment::Nic),
                ("10.0046", "pij", PadaAssignment::Nic),
                ("10.0050", "pis", PadaAssignment::Nic),
                ("10.0051", "sAntv", PadaAssignment::Nic),
                ("10.0052", "sAntv", PadaAssignment::Nic),
                ("10.0053", "Svalk", PadaAssignment::Nic),
                ("10.0054", "valk", PadaAssignment::Nic),
                ("10.0055", "snih", PadaAssignment::Nic),
                ("10.0056", "sPiw", PadaAssignment::Nic),
                ("10.0057", "smiw", PadaAssignment::Nic),
                ("10.0059", "Sliz", PadaAssignment::Nic),
                ("10.0061", "piC", PadaAssignment::Nic),
                ("10.0063", "SraR", PadaAssignment::Nic),
                ("10.0064", "taq", PadaAssignment::Nic),
                ("10.0065", "Kaq", PadaAssignment::Nic),
                ("10.0078", "Card", PadaAssignment::Nic),
                ("10.0079", "pust", PadaAssignment::Nic),
                ("10.0080", "bust", PadaAssignment::Nic),
                ("10.0081", "cud", PadaAssignment::Nic),
                ("10.0082", "nakk", PadaAssignment::Nic),
                ("10.0083", "Dakk", PadaAssignment::Nic),
                ("10.0084", "cakk", PadaAssignment::Nic),
                ("10.0085", "cukk", PadaAssignment::Nic),
                ("10.0086", "kzal", PadaAssignment::Nic),
                ("10.0087", "tal", PadaAssignment::Nic),
                ("10.0088", "tul", PadaAssignment::Nic),
                ("10.0089", "dul", PadaAssignment::Nic),
                ("10.0090", "pul", PadaAssignment::Nic),
                ("10.0091", "cul", PadaAssignment::Nic),
                ("10.0092", "mUl", PadaAssignment::Nic),
                ("10.0093", "kal", PadaAssignment::Nic),
                ("10.0094", "vil", PadaAssignment::Nic),
                ("10.0095", "bil", PadaAssignment::Nic),
                ("10.0096", "til", PadaAssignment::Nic),
                ("10.0097", "cal", PadaAssignment::Nic),
                ("10.0098", "pAl", PadaAssignment::Nic),
                ("10.0099", "pal", PadaAssignment::Nic),
                ("10.0100", "lUz", PadaAssignment::Nic),
                ("10.0101", "Sulb", PadaAssignment::Nic),
                ("10.0102", "SUrp", PadaAssignment::Nic),
                ("10.0103", "cuw", PadaAssignment::Nic),
                ("10.0104", "muw", PadaAssignment::Nic),
                ("10.0109", "vraj", PadaAssignment::Nic),
                ("10.0110", "Sulk", PadaAssignment::Nic),
                ("10.0115", "Svart", PadaAssignment::Nic),
                ("10.0116", "svart", PadaAssignment::Nic),
                ("10.0117", "SvaBr", PadaAssignment::Nic),
                ("10.0125", "Gaww", PadaAssignment::Nic),
                ("10.0126", "must", PadaAssignment::Nic),
                ("10.0127", "Kaww", PadaAssignment::Nic),
                ("10.0128", "saww", PadaAssignment::Nic),
                ("10.0129", "sPiww", PadaAssignment::Nic),
                ("10.0131", "pul", PadaAssignment::Nic),
                ("10.0132", "pUrR", PadaAssignment::Nic),
                ("10.0133", "puR", PadaAssignment::Nic),
                ("10.0134", "puns", PadaAssignment::Nic),
                ("10.0136", "vyap", PadaAssignment::Nic),
                ("10.0137", "vyay", PadaAssignment::Nic),
                ("10.0138", "pUl", PadaAssignment::Nic),
                ("10.0139", "DUs", PadaAssignment::Nic),
                ("10.0140", "DUz", PadaAssignment::Nic),
                ("10.0141", "DUS", PadaAssignment::Nic),
                ("10.0142", "kIw", PadaAssignment::Nic),
                ("10.0143", "cUrR", PadaAssignment::Nic),
                ("10.0144", "pUj", PadaAssignment::Nic),
                ("10.0145", "ark", PadaAssignment::Nic),
                ("10.0146", "SuW", PadaAssignment::Nic),
                ("10.0148", "juq", PadaAssignment::Nic),
                ("10.0149", "gaj", PadaAssignment::Nic),
                ("10.0150", "mArj", PadaAssignment::Nic),
                ("10.0151", "marc", PadaAssignment::Nic),
                ("10.0154", "tij", PadaAssignment::Nic),
                ("10.0156", "varD", PadaAssignment::Nic),
                ("10.0161", "hlap", PadaAssignment::Nic),
                ("10.0162", "klap", PadaAssignment::Nic),
                ("10.0163", "hrap", PadaAssignment::Nic),
                ("10.0165", "brIs", PadaAssignment::Nic),
                ("10.0167", "il", PadaAssignment::Nic),
                ("10.0168", "mrakz", PadaAssignment::Nic),
                ("10.0169", "ast", PadaAssignment::Nic),
                ("10.0172", "brUs", PadaAssignment::Nic),
                ("10.0173", "barh", PadaAssignment::Nic),
                ("10.0176", "bul", PadaAssignment::Nic),
                ("10.0177", "garj", PadaAssignment::Nic),
                ("10.0178", "gard", PadaAssignment::Nic),
                ("10.0179", "garD", PadaAssignment::Nic),
                ("10.0181", "pUrv", PadaAssignment::Nic),
                ("10.0183", "Iq", PadaAssignment::Nic),
                ("10.0186", "parT", PadaAssignment::Nic),
                ("10.0187", "ruz", PadaAssignment::Nic),
                ("10.0188", "ruw", PadaAssignment::Nic),
                ("10.0189", "qip", PadaAssignment::Nic),
                ("10.0190", "stup", PadaAssignment::Nic),
                ("10.0191", "stUp", PadaAssignment::Nic),
                ("10.0237", "carc", PadaAssignment::Nic),
                ("10.0238", "bukk", PadaAssignment::Nic),
                ("10.0239", "Sabd", PadaAssignment::Nic),
                ("10.0240", "kaR", PadaAssignment::Nic),
                ("10.0242", "sUd", PadaAssignment::Nic),
                ("10.0244", "paS", PadaAssignment::Nic),
                ("10.0245", "am", PadaAssignment::Nic),
                ("10.0246", "caw", PadaAssignment::Nic),
                ("10.0247", "sPuw", PadaAssignment::Nic),
                ("10.0248", "Gaw", PadaAssignment::Nic),
                ("10.0250", "arj", PadaAssignment::Nic),
                ("10.0252", "krand", PadaAssignment::Nic),
                ("10.0253", "las", PadaAssignment::Nic),
                ("10.0256", "mokz", PadaAssignment::Nic),
                ("10.0257", "arh", PadaAssignment::Nic),
                ("10.0259", "Baj", PadaAssignment::Nic),
                ("10.0261", "yat", PadaAssignment::Nic),
                ("10.0262", "rak", PadaAssignment::Nic),
                ("10.0263", "lag", PadaAssignment::Nic),
                ("10.0264", "raG", PadaAssignment::Nic),
                ("10.0265", "rag", PadaAssignment::Nic),
                ("10.0268", "mud", PadaAssignment::Nic),
                ("10.0269", "tras", PadaAssignment::Nic),
                ("10.0271", "uDras", PadaAssignment::Nic),
                ("10.0272", "muc", PadaAssignment::Nic),
                ("10.0273", "vas", PadaAssignment::Nic),
                ("10.0274", "car", PadaAssignment::Nic),
                ("10.0276", "cyus", PadaAssignment::Nic),
                ("10.0397", "raNg", PadaAssignment::Nic),
                ("10.0414", "Keq", PadaAssignment::Nic),
                ("10.0438", "kUR", PadaAssignment::Nic),
                ("10.0457", "kart", PadaAssignment::Nic),
                ("10.0462", "Cuw", PadaAssignment::Nic),
                ("10.0470", "karR", PadaAssignment::Nic),
                ("10.0491", "ruW", PadaAssignment::Nic),
                ("10.0023", "urj", PadaAssignment::Nic),
                ("10.0026", "curR", PadaAssignment::Nic),
                ("10.0037", "adw", PadaAssignment::Nic),
                ("10.0155", "kFt", PadaAssignment::Nic),
                ("10.0170", "mleC", PadaAssignment::Nic),
                ("10.0180", "gurd", PadaAssignment::Nic),
                ("10.0270", "Dras", PadaAssignment::Nic),
                ("10.0278", "kfp", PadaAssignment::Nic),
                ("10.0175", "picc", PadaAssignment::Nic),
            ]
        );
    }

    #[test]
    fn juhotyadi_rows_are_the_twenty_six_curated_roots() {
        // Slice 3a opened the ślu gaṇa with its eponym √hu and √ki, the two
        // roots that exercise dvitva, 7.4.62, 7.1.4, 3.4.109/7.3.83, 6.4.82,
        // 6.4.87's and 6.4.101's hu arms and 8.4.54 with nothing else. Slice
        // 3b adds √bhī and √hrī: both parasmaipadī, √bhī's by 1.3.78 (the
        // `\` sits on the root vowel, not on an it) and √hrī's the same
        // way, plus the vikaraṇa's 6.4.115, 7.4.60 and 6.4.77's iyaṅ arm.
        // Slice 3c adds √dā and √dhā (ubhayapadī by 1.3.72, ghu by 1.1.20)
        // and √mā and √hā (ātmanepadī by 1.3.12).
        // Slice 3c2 adds √hā parasmaipada (03.0009, jahāti) and √gā
        // (03.0026), both parasmaipadī by 1.3.78. Slice 3d adds the six
        // consonant-initial ṛ-roots, √pṝ, √pṛ, √bhṛ (ubhayapadī by 1.3.72,
        // ñit), √ghṛ, √hṛ and √sṛ. Slice 3d2 adds √ṛ (03.0017), parasmaipadī
        // by 1.3.78, the gaṇa's one vowel-initial ṛ-root. Slice 3e adds √ṇij,
        // √vij and √viṣ (03.0012–03.0014), ubhayapadī by 1.3.72, the three
        // roots 7.4.75 names. Slice 3f adds √kit, √tur, √dhiṣ and √dhan
        // (03.0021–03.0024), parasmaipadī by 1.3.78. Slice 3f2 adds √bhas
        // (03.0019), parasmaipadī by 1.3.78. Slice 3f3 adds √jan (03.0025),
        // parasmaipadī by 1.3.78, and closes the gaṇa at all 26 of its
        // dhātupāṭha rows.
        let rows: Vec<_> = dhatus()
            .iter()
            .filter(|d| d.gana == Gana::Juhotyadi)
            .map(|d| (d.dhatupatha, d.code, d.pada))
            .collect();
        assert_eq!(
            rows,
            vec![
                ("03.0001", "hu", PadaAssignment::Parasmaipada),
                ("03.0002", "BI", PadaAssignment::Parasmaipada),
                ("03.0003", "hrI", PadaAssignment::Parasmaipada),
                ("03.0004", "pF", PadaAssignment::Parasmaipada),
                ("03.0005", "pf", PadaAssignment::Parasmaipada),
                ("03.0006", "Bf", PadaAssignment::Ubhayapada),
                ("03.0007", "mA", PadaAssignment::Atmanepada),
                ("03.0008", "hA", PadaAssignment::Atmanepada),
                ("03.0009", "hA", PadaAssignment::Parasmaipada),
                ("03.0010", "dA", PadaAssignment::Ubhayapada),
                ("03.0011", "DA", PadaAssignment::Ubhayapada),
                ("03.0012", "nij", PadaAssignment::Ubhayapada),
                ("03.0013", "vij", PadaAssignment::Ubhayapada),
                ("03.0014", "viz", PadaAssignment::Ubhayapada),
                ("03.0015", "Gf", PadaAssignment::Parasmaipada),
                ("03.0016", "hf", PadaAssignment::Parasmaipada),
                ("03.0017", "f", PadaAssignment::Parasmaipada),
                ("03.0018", "sf", PadaAssignment::Parasmaipada),
                ("03.0019", "Bas", PadaAssignment::Parasmaipada),
                ("03.0020", "ki", PadaAssignment::Parasmaipada),
                ("03.0021", "kit", PadaAssignment::Parasmaipada),
                ("03.0022", "tur", PadaAssignment::Parasmaipada),
                ("03.0023", "Diz", PadaAssignment::Parasmaipada),
                ("03.0024", "Dan", PadaAssignment::Parasmaipada),
                ("03.0025", "jan", PadaAssignment::Parasmaipada),
                ("03.0026", "gA", PadaAssignment::Parasmaipada),
            ]
        );
        // guna.rs's 6.4.87 and adesha.rs's 6.4.101 both identify √hu by
        // `ANGA.text == "hu"` with no gaṇa clause, resting on "no other
        // curated root reads `hu`" — a premise stated only in those rules'
        // comments and checked nowhere else. `code` is NOT unique in
        // general (vij, vid, man, aS already repeat), so this is a real
        // tripwire, not a tautology. 6.4.115 now identifies √bhī the same
        // way, by `ANGA.text == "BI"` with no gaṇa clause, so `BI` must
        // stay unique too.
        assert_eq!(
            dhatus().iter().filter(|d| d.code == "hu").count(),
            1,
            "6.4.87 and 6.4.101 key on ANGA.text == \"hu\" with no gaṇa \
             clause; if a second curated root ever reads \"hu\" both rules \
             need a gaṇa guard"
        );
        assert_eq!(
            dhatus().iter().filter(|d| d.code == "BI").count(),
            1,
            "6.4.115 keys on ANGA.text == \"BI\" with no gaṇa clause; if a \
             second curated root ever reads \"BI\" it needs a gaṇa guard"
        );
        // Every root-specific rule of slices 3c, 3c2, 3d, 3d2, 3e and 3f2 (7.4.75, 7.4.76,
        // 7.4.77, 7.4.78, 8.2.38, 8.2.40's adhaḥ, 6.4.116–6.4.118, 6.4.100) and
        // 1.1.20's Tag::Ghu key on the dhātupāṭha NUMBER, so `hA`, `dA`, `DA`,
        // `gA`, `pf`, `pF`, `Bf`, `f` and `Bas` need no uniqueness tripwire. `hA` is held by two
        // rows, 03.0008 and 03.0009, which is exactly why. Slice 3d's 7.4.66
        // and 7.1.102 name sounds (a ṛ-vowel; a labial before ṝ), not roots. Slice
        // 3d2's 6.4.78 names sounds too (an abhyāsa-final i/u before a dissimilar
        // vowel).
    }

    #[test]
    fn padas_maps_each_assignment_to_its_derivable_padas() {
        assert_eq!(PadaAssignment::Parasmaipada.padas(), &[Pada::Parasmaipada]);
        assert_eq!(PadaAssignment::Atmanepada.padas(), &[Pada::Atmanepada]);
        assert_eq!(
            PadaAssignment::Ubhayapada.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
        assert_eq!(
            PadaAssignment::UbhayapadaAnavane.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
        assert_eq!(
            PadaAssignment::Nic.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
        assert_eq!(
            PadaAssignment::NicUbhayapada.padas(),
            &[Pada::Parasmaipada, Pada::Atmanepada]
        );
        assert_eq!(PadaAssignment::Akusmiya.padas(), &[Pada::Atmanepada]);
        assert_eq!(PadaAssignment::AaGarviya.padas(), &[Pada::Atmanepada]);
    }

    #[test]
    fn dhatu_padas_add_the_nicless_parasmaipada() {
        // A root whose ṇic is optional derives its ṇic branch's padas plus the
        // ṇic-less branch's parasmaipada, parasmaipada first. One row per
        // ṇic-branch assignment, and a row outside the table for contrast.
        let padas = |n: &str| dhatus().iter().find(|d| d.dhatupatha == n).unwrap().padas();
        let both = &[Pada::Parasmaipada, Pada::Atmanepada];
        assert_eq!(padas("10.0193"), both, "ākusmīya daSi~");
        assert_eq!(padas("10.0449"), both, "ā-garvīya garva");
        assert_eq!(padas("10.0451"), both, "1.3.74 mUtra");
        assert_eq!(
            padas("10.0192"),
            &[Pada::Atmanepada],
            "ākusmīya cita~, ṇic only"
        );
        assert_eq!(
            padas("10.0440"),
            &[Pada::Atmanepada],
            "ā-garvīya pada, ṇic only"
        );
        assert_eq!(padas("01.0001"), &[Pada::Parasmaipada]);
        for d in dhatus() {
            if optional_nic(d.dhatupatha).is_none() {
                assert_eq!(d.padas(), d.pada.padas(), "{}", d.dhatupatha);
            }
        }
    }

    #[test]
    fn ubhayapada_padas_are_parasmaipada_first() {
        // Pinned, not incidental: the paradigm and roundtrip harnesses loop
        // over the whole `padas()` slice, so its order is not load-bearing
        // for them, and
        // every `d.pada.padas()[0]` call site (the in-crate unit-test
        // helpers across the workspace) only ever sees single-pada roots
        // today. A mutant that reversed this slice would survive with no
        // test able to catch it — the same shape as the three `Context::is_tip`
        // survivors slice 7b found — so the order is asserted directly here.
        assert_eq!(PadaAssignment::Ubhayapada.padas()[0], Pada::Parasmaipada);
        assert_eq!(
            PadaAssignment::UbhayapadaAnavane.padas()[0],
            Pada::Parasmaipada
        );
        assert_eq!(PadaAssignment::Nic.padas()[0], Pada::Parasmaipada);
    }

    #[test]
    fn every_curated_root_admits_at_least_one_pada() {
        for d in dhatus() {
            assert!(
                !d.pada.padas().is_empty(),
                "{} admits no pada at all",
                d.dhatupatha
            );
        }
    }

    /// Upstream's dhātupāṭha, vendored at the commit named in its header.
    /// `include_str!` sits inside `#[cfg(test)]`, so the 54K reaches the test
    /// binary only and never the library.
    const UPSTREAM: &str = include_str!("../../../data/dhatupatha.tsv");

    /// `(number, upadeśa, artha)` for every upstream row.
    fn upstream_rows() -> Vec<(&'static str, &'static str, &'static str)> {
        UPSTREAM
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| {
                let mut f = l.split('\t');
                match (f.next(), f.next(), f.next()) {
                    // Skip upstream's own `code	dhatu	artha` header row.
                    (Some(n), Some(u), Some(a)) if n != "code" => Some((n, u, a)),
                    _ => None,
                }
            })
            .collect()
    }

    /// True for an SLP1 consonant (*hal*). SLP1's vowels are the fourteen
    /// listed here; `~`, being notation rather than a sound, is not a hal.
    fn is_hal(c: char) -> bool {
        c.is_alphabetic() && !"aAiIuUfFxXeEoO".contains(c)
    }

    /// Strips the anubandhas from an upstream upadeśa.
    ///
    /// **Not grammar the pipeline owes a `Rule`** — it never runs in a
    /// derivation. It exists so `dhatupatha_numbers_resolve_upstream` can
    /// relate an upstream row to our stored `code` without consulting
    /// anything this repo wrote, which is the assertion that makes the
    /// cross-implementation audit non-circular.
    fn strip_anubandhas(upadesha: &str) -> String {
        // Accent notation: anudātta `\`, svarita `^`. Marks, not sounds.
        let s: String = upadesha
            .chars()
            .filter(|c| *c != '\\' && *c != '^')
            .collect();

        // 1.3.3 halantyam is decided on the ORIGINAL upadeśa, before 1.3.2
        // deletes anything. Getting this order wrong corrupts silently rather
        // than failing loudly: `paWa~` ends in the vowel `a` (marked
        // anunāsika by the `~` after it), so its `W` is root-final and must
        // survive — deciding after the deletion would strip it to `pa`, and
        // would strip `tfha~` to `tf`, destroying a real root-final `h` while
        // still producing a plausible string. `ru\Di~^r` genuinely ends in
        // the consonant `r`, so that `r` IS an it.
        let ends_in_hal = s.chars().last().is_some_and(is_hal);

        // 1.3.2 upadeśe'j-anunāsika it, with 1.3.9 tasya lopaḥ. Upstream
        // marks an anunāsika it with a following `~`, so each `X~` pair goes.
        let mut t = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if chars.peek() == Some(&'~') {
                chars.next();
                continue;
            }
            t.push(c);
        }

        // 1.3.5 ādir ñiṭuḍavaḥ: an initial ñi / ṭu / ḍu is it.
        for prefix in ["Yi", "wu", "qu"] {
            if let Some(rest) = t.strip_prefix(prefix) {
                t = rest.to_string();
                break;
            }
        }

        // 1.3.3 halantyam, on the verdict reached above.
        if ends_in_hal && t.chars().count() > 1 {
            t.pop();
        }
        t
    }

    /// The pada a root's own upadeśa assigns it, by 1.3.12 / 1.3.72 / 1.3.78.
    ///
    /// **Not grammar the pipeline owes a `Rule`** — the same standing as
    /// `strip_anubandhas`, for the same reason: it never runs in a
    /// derivation. It exists so `curated_pada_agrees_with_upadesha_markers`
    /// can re-derive the `pada` column from upstream without consulting
    /// anything this repo wrote about the root.
    ///
    /// The accent notation is the whole difficulty. Upstream writes an accent
    /// AFTER the `~` that marks an anunāsika it, so `~\` is an anudātta it and
    /// `~^` a svarita it — whereas a `\` sitting directly on a vowel elsewhere
    /// is the ROOT's own accent and says nothing about pada. Counted off the
    /// vendored upadeśa: 73 of the 594 curated roots carry a `\` at all, and 52
    /// of those carry one on a root vowel — `01.0642 ji\`, `01.1082 smf\` and
    /// `02.0001 a\da~` among them — so conflating the two does not fail
    /// loudly; it silently calls most of the table ātmanepada.
    fn pada_from_upadesha(upadesha: &str) -> PadaAssignment {
        // Accents attached to an it vowel, and only those.
        let anudatta_it = upadesha.contains("~\\");
        let svarita_it = upadesha.contains("~^");

        // 1.3.3 halantyam, decided on the accent-stripped upadeśa: a final hal
        // is an it. `SIN` and `vfN` reach 1.3.12 this way, `RI\Y` reaches
        // 1.3.72, and none of the three carries a `~` at all.
        let bare: String = upadesha
            .chars()
            .filter(|c| *c != '\\' && *c != '^')
            .collect();
        let final_it = bare.chars().last().filter(|c| is_hal(*c));
        let ngit = final_it == Some('N');
        // ONLY a final `Y`. 1.3.5 ādir ñiṭuḍavaḥ does make an initial `Yi`
        // an it — `strip_anubandhas` above strips it, which is why √bhī's
        // curated code is `BI` — but the it it supplies is not an anubandha
        // 1.3.72 svaritañitaḥ reads. vidyut-prakriya fires 1.3.72 on none of
        // the dhātupāṭha's fourteen `Yi`-initial rows; every ātmanepada
        // verdict among them comes from a `~\` and every parasmaipada one
        // from 1.3.78. Pinned by
        // `bhi_is_parasmaipada_despite_its_initial_nyi`.
        //
        // This read `final_it == Some('Y') || bare.starts_with("Yi")` until
        // slice 3b. The disjunct decided nothing for the first 79 curated
        // roots — √indh, the only `Yi`-initial one, is caught by the
        // ātmanepada branch below — so it sat unfalsified until √bhī became
        // the first root whose verdict it decided, and decided wrongly.
        // The same argument forbids a `wu`/`qu` arm here, for the stronger
        // reason that ṭu and ḍu are not ñ-its at all.
        let nyit = final_it == Some('Y');

        // The two conditions are DISJOINT over every upstream row — no
        // upadeśa carries both an anudātta/ṅ it and a svarita/ñ it — so this
        // order decides nothing today, and `no_upadesha_satisfies_both_pada_branches`
        // is what asserts that rather than leaving it assumed. The order is
        // kept because 1.3.12 is the apavāda by tradition: if upstream ever
        // grows a row satisfying both, that test fails first and this
        // sequence is the answer already in place.
        if anudatta_it || ngit {
            // 1.3.12 anudāttaṅita ātmanepadam.
            return PadaAssignment::Atmanepada;
        }
        if svarita_it || nyit {
            // 1.3.72 svaritañitaḥ kartrabhiprāye kriyāphale — ubhayapada,
            // since 1.3.78 supplies the parasmaipada arm.
            return PadaAssignment::Ubhayapada;
        }
        // 1.3.78 śeṣāt kartari parasmaipadam.
        PadaAssignment::Parasmaipada
    }

    /// 6.1.64 dhātvādeḥ ṣaḥ saḥ (ṣ → s) and 6.1.65 ṇo naḥ (ṇ → n). A
    /// root-initial ṣ or ṇ in the upadeśa is stored substituted, because no
    /// rule in this engine performs either substitution. For `zwiGa~\` the
    /// retroflex immediately after goes with it, under the vārttika on
    /// 6.1.64 (ṣṭ → st), which is exactly what `stiG` records.
    ///
    /// The `zw` → `st` arm of that vārttika (6.1.64.2) is handled here, and
    /// so is the plain `z`/`zR`/`zaR` arm — a lone `z` becomes `s`, and any
    /// `R` left in what follows (conditioned by the now-gone retroflex)
    /// reverts to `n`: `zaRu~` stores as `san`. Only `zW` → `sT` and the
    /// companion vārttika 6.1.64.1 exempting `zWiv`/`zvazk` from any change
    /// remain unimplemented: no curated root needs them today. `01.1077
    /// zWA\` (√sthā), `01.0641`/`04.0004 zWivu~` and `01.0105 zvazka~\` are
    /// the upstream roots that would exercise them; a future slice curating
    /// any of those must extend this function first, or
    /// `dhatupatha_numbers_resolve_upstream` fails loudly (√sthā would
    /// demand `sWA`, which this function does not produce).
    fn dhatvadeh_sha_sa(code: String) -> String {
        if let Some(rest) = code.strip_prefix('z') {
            let rest = rest
                .strip_prefix('w')
                .map_or_else(|| rest.to_string(), |r| format!("t{r}"));
            // With the z gone, the retroflexion it conditioned goes too
            // (nimitte naṣṭe naimittikasya apy anivṛttiḥ is the paribhāṣā
            // AGAINST this — but 6.1.64 is nipātana territory and the
            // attested stems are san (zaRu~), stiG (zwiGa~): the R
            // reverts, the w hardens to t). vidyut derives sanoti.
            return format!("s{}", rest.replace('R', "n"));
        }
        if let Some(rest) = code.strip_prefix('R') {
            return format!("n{rest}");
        }
        code
    }

    /// The form this repo stores as `Dhatu::code`, derived from an upstream
    /// upadeśa.
    fn stored_form(upadesha: &str) -> String {
        let s = dhatvadeh_sha_sa(strip_anubandhas(upadesha));
        // 7.1.58 idito num dhātoḥ is not derivable here, so an idit root is
        // stored with the num already inserted, after its last vowel (1.1.47
        // mid aco 'ntyāt paraḥ): `hisi~` stores as `hins`. An upadeśa is idit
        // when its LAST marker is `i~`; a non-final `i~` belongs to another
        // marker — irit `i~r` (`ru\Di~^r`) or `cakzi~N`. This is the single
        // deviation between an it-stripped upadeśa and a stored `code`. It is
        // applied to every upadeśa, not only curated ones, because the
        // sibling check below compares a curated row against its uncurated
        // neighbours: `10.0194 dasi~` must store as `dans`, not collide with
        // `10.0195 dasa~`'s `das`.
        let idit = upadesha.trim_end_matches(['\\', '^']).ends_with("i~");
        match s.rfind(|c: char| !is_hal(c)) {
            Some(i) if idit => format!("{}n{}", &s[..=i], &s[i + 1..]),
            _ => s,
        }
    }

    #[test]
    fn stored_form_inserts_num_for_exactly_the_idit_upadeshas() {
        // idit: the num lands after the last vowel.
        assert_eq!(stored_form("hisi~"), "hins");
        assert_eq!(stored_form("dasi~"), "dans");
        assert_eq!(stored_form("tatri~"), "tantr");
        assert_eq!(stored_form("aci~^"), "anc");
        // A non-final `i~` is another marker's: no num.
        assert_eq!(stored_form("ru\\Di~^r"), "ruD");
        assert_eq!(stored_form("ca\\kzi~\\N"), "cakz");
        // No `i~` at all: the plain it-stripped form.
        assert_eq!(stored_form("dasa~"), "das");
        assert_eq!(stored_form("kusma~"), "kusm");
    }

    /// The id of the rule that makes curādi row `number`'s ṇic optional, read
    /// as vidyut-prakriya reads it: first the two antargaṇas, by position
    /// (10.0498, rows `10.0338`–`10.0388`, and 10.0499, rows
    /// `10.0279`–`10.0337`, before any marker), then the upadeśa: 2564 for an
    /// idit root (last marker `i~`), 2565 for an `F`-final one, 2570 for a ñit
    /// or udit one (last marker `Y` or `u~`, or an initial `u~`: vidyut's
    /// it-reading tags a root udit from a `u~` marker anywhere in the
    /// upadeśa, so `10.0270 u~Drasa~` counts), 2571 for `Guzi~r`, 2573.1 for
    /// `pata`, and 2573.3 for the three roots the Kaumudī names there. 2573.3
    /// is a list, not a shape: `Cidra`, `sUtra` and the rest have a conjunct
    /// before their final `a` and take ṇic. vidyut's 2572 (īdit) has no arm:
    /// every īdit curādi row is in one of the two antargaṇas, which come
    /// first, as `optional_nic_matches_upadesha_markers` pins.
    fn optional_nic_from_upadesha(number: &str, upadesha: &str) -> Option<&'static str> {
        if ("10.0338"..="10.0388").contains(&number) {
            return Some("10.0498");
        }
        if ("10.0279"..="10.0337").contains(&number) {
            return Some("10.0499");
        }
        let u = upadesha.trim_end_matches(['\\', '^']);
        if u.ends_with("i~") {
            Some("2564")
        } else if u.ends_with('F') {
            Some("2565")
        } else if u.ends_with("u~") || u.starts_with("u~") || u.ends_with('Y') {
            Some("2570")
        } else if u == "Guzi~r" {
            Some("2571")
        } else if u == "pata" {
            Some("2573.1")
        } else if ["mUtra", "katra", "garva"].contains(&u) {
            Some("2573.3")
        } else {
            None
        }
    }

    #[test]
    fn optional_nic_matches_upadesha_markers() {
        // Every table entry is what its upadeśa says, and every curated
        // curādi row whose upadeśa says so is in the table: the table is
        // exactly the curated rows those markers select. Non-circular, as
        // `dhatupatha_numbers_resolve_upstream` is: the upadeśa comes from
        // the vendored dhātupāṭha, not from the table.
        let rows = upstream_rows();
        let upadesha = |n: &str| rows.iter().find(|(m, _, _)| *m == n).unwrap().1;
        for (n, id) in OPTIONAL_NIC {
            assert_eq!(optional_nic_from_upadesha(n, upadesha(n)), Some(*id), "{n}");
            assert!(
                dhatus().iter().any(|d| d.dhatupatha == *n),
                "{n} is not curated"
            );
        }
        for d in dhatus().iter().filter(|d| d.gana == Gana::Curadi) {
            assert_eq!(
                optional_nic(d.dhatupatha),
                optional_nic_from_upadesha(d.dhatupatha, upadesha(d.dhatupatha)),
                "{} {}",
                d.dhatupatha,
                upadesha(d.dhatupatha)
            );
        }
        assert_eq!(OPTIONAL_NIC.len(), 182);
        // `10.0368 za\da~` stays out: the reading above makes it 10.0498's,
        // but vidyut derives it with the upasarga ā, which this engine does
        // not model.
        assert!(OPTIONAL_NIC.iter().all(|(n, _)| *n != "10.0368"));
        assert_eq!(
            optional_nic_from_upadesha("10.0368", upadesha("10.0368")),
            Some("10.0498")
        );
        // The one-row triggers reach one row each, and 2572 (īdit) none:
        // outside the two antargaṇas `pF` is the only `F`-final curādi row
        // (`10.0346 jF` is ādhṛṣīya), `Guzi~r` is one row, and every īdit
        // curādi row is inside.
        let antargana = |n: &str| ("10.0279"..="10.0388").contains(&n);
        let shape = |u: &str| u.trim_end_matches(['\\', '^']).to_string();
        let curadi: Vec<(&str, String)> = rows
            .iter()
            .filter(|(n, _, _)| n.starts_with("10."))
            .map(|(n, u, _)| (*n, shape(u)))
            .collect();
        let f_final: Vec<&str> = curadi
            .iter()
            .filter(|(n, u)| !antargana(n) && u.ends_with('F'))
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(f_final, ["10.0022"]);
        let ghuz: Vec<&str> = curadi
            .iter()
            .filter(|(_, u)| u == "Guzi~r")
            .map(|(n, _)| *n)
            .collect();
        assert_eq!(ghuz, ["10.0251"]);
        let idit: Vec<&str> = curadi
            .iter()
            .filter(|(_, u)| u.ends_with("I~"))
            .map(|(n, _)| *n)
            .collect();
        assert!(!idit.is_empty());
        assert!(idit.iter().all(|n| antargana(n)), "{idit:?}");
        // 2573.3 is the Kaumudī's list: a conjunct-before-`a` curādi root
        // outside it (`10.0469 Cidra`, curated in 10e) is no optional-ṇic row.
        assert_eq!(optional_nic_from_upadesha("10.0469", "Cidra"), None);
        assert_eq!(optional_nic("10.0469"), None);
    }

    #[test]
    fn aa_garviya_is_exactly_the_rows_10_0497_names() {
        // The gaṇasūtra 10.0497 follows `10.0449 garva` and makes the ten
        // rows from `10.0440 pada` ātmanepadī. vidyut-prakriya lists the
        // same ten upadeśas (`AA_GARVIYA`). `garva`'s ṇic is optional, so this
        // test — not only its ṇic branch's derivations — holds the range's
        // upper end.
        let rows = upstream_rows();
        let in_range: Vec<&str> = rows
            .iter()
            .filter(|(n, _, _)| AA_GARVIYA.contains(n))
            .map(|(_, u, _)| *u)
            .collect();
        assert_eq!(
            in_range,
            [
                "pada", "gfha", "mfga", "kuha", "SUra", "vIra", "sTUla", "arTa", "satra", "garva"
            ]
        );
        // The neighbours on either side exist upstream and fall outside.
        for n in ["10.0439", "10.0450"] {
            assert!(rows.iter().any(|(m, _, _)| *m == n), "{n}");
            assert!(!AA_GARVIYA.contains(&n), "{n}");
        }
    }

    #[test]
    fn jnapadi_is_exactly_the_rows_10_0493_names() {
        // The gaṇasūtra 10.0493 follows `10.0124 ciY` and makes the seven
        // rows from `10.0118 jYapa~` mit. vidyut-prakriya lists the same
        // seven upadeśas (`JNAP_ADI`). All seven are curated since slice 10j,
        // so derivations credit 10.0493 at both ends of the range; this test
        // holds the range itself to upstream.
        let rows = upstream_rows();
        let in_range: Vec<&str> = rows
            .iter()
            .filter(|(n, _, _)| JNAPADI.contains(n))
            .map(|(_, u, _)| *u)
            .collect();
        assert_eq!(
            in_range,
            ["jYapa~", "yama~", "caha~", "capa~", "raha~", "bala~", "ciY"]
        );
        // The neighbours on either side exist upstream and fall outside.
        for n in ["10.0117", "10.0125"] {
            assert!(rows.iter().any(|(m, _, _)| *m == n), "{n}");
            assert!(!JNAPADI.contains(&n), "{n}");
        }
    }

    /// The one pair of curated rows the dhātupāṭha lists twice, identically
    /// (upadeśa, artha and optional-ṇic verdict): `laGi~ BAzAyAm` at
    /// `10.0291` and `10.0327`, both āsvadīya. vidyut-prakriya derives both,
    /// so both are curated, and each is the other's one allowed sibling in
    /// `dhatupatha_numbers_resolve_upstream`.
    const IDENTICAL_UPSTREAM_PAIR: [&str; 2] = ["10.0291", "10.0327"];

    /// The one pair of curated rows whose upadeśas differ but which store the
    /// same code with the same artha: `zAntva~` (`10.0051`) and `sAntva~`
    /// (`10.0052`), both *sAmaprayoge*. 6.1.64 dhātvādeḥ ṣaḥ saḥ makes the
    /// first `sAntv` too. vidyut-prakriya derives both, so both are curated
    /// (slice 10k), and each is the other's one allowed sibling in
    /// `dhatupatha_numbers_resolve_upstream`.
    const CONVERGENT_UPADESHA_PAIR: [&str; 2] = ["10.0051", "10.0052"];

    #[test]
    fn dhatupatha_numbers_resolve_upstream() {
        let rows = upstream_rows();
        let count = rows.len();
        assert!(
            count > 2000,
            "vendored dhātupāṭha looks truncated: {count} rows"
        );
        let mut numbers: Vec<&str> = rows.iter().map(|(n, _, _)| *n).collect();
        numbers.sort_unstable();
        numbers.dedup();
        assert_eq!(
            numbers.len(),
            count,
            "upstream numbers must be unique for one to serve as our key"
        );

        for d in dhatus() {
            let (_, upadesha, artha) = rows
                .iter()
                .find(|(n, _, _)| *n == d.dhatupatha)
                .unwrap_or_else(|| panic!("{} names no upstream row", d.dhatupatha));
            assert_eq!(
                *artha, d.artha,
                "{} artha diverges from upstream",
                d.dhatupatha
            );
            // THIS is the assertion that breaks the circularity. Matching on
            // number and artha alone would still pass if a number pointed at
            // a sibling entry sharing an artha, and upstream has 8- and
            // 15-way artha collisions (`vyaktAyAM vAci`, `vfdDO`). Relating
            // the upadeśa to the code is the only check that cannot be
            // satisfied by copying back the choice we made.
            let stripped = stored_form(upadesha);
            assert_eq!(
                stripped, d.code,
                "{} {upadesha} it-strips to {stripped}, but DHATUS stores {}",
                d.dhatupatha, d.code
            );
            // The spec's claim is that the number resolves the root
            // UNIQUELY, not merely that the row it names matches. A number
            // pointing at one of several siblings sharing both the
            // it-stripped upadeśa and the artha within the same gaṇa would
            // still pass every assertion above. Scope to the gaṇa (the
            // number's two-digit prefix, per `gana_matches_dhatupatha_prefix`)
            // because upstream reuses (code, artha) pairs across gaṇas too.
            // Siblings that differ in their optional-ṇic verdict are distinct
            // roots to this engine, which reads that verdict by number
            // (`OPTIONAL_NIC`): `10.0174 SraRu~` (2570) beside `10.0063 SraRa~`
            // (none) is the case, pinned below, and so is `10.0367 arha~`
            // (10.0498) beside `10.0257 arha~` (none). The verdict only tells
            // curādi rows apart (the engine keys `OPTIONAL_NIC` by number
            // there alone), so every other gaṇa compares no verdict. The two
            // rows of `IDENTICAL_UPSTREAM_PAIR` differ in nothing, and the two
            // of `CONVERGENT_UPADESHA_PAIR` only in what 6.1.64 levels, so each
            // has the other as its one allowed sibling.
            let gana_prefix = &d.dhatupatha[..2];
            let verdict = |n: &str, u: &str| {
                if gana_prefix == "10" {
                    optional_nic_from_upadesha(n, u)
                } else {
                    None
                }
            };
            let own_verdict = verdict(d.dhatupatha, upadesha);
            let siblings = rows
                .iter()
                .filter(|(n, u, a)| {
                    n.starts_with(gana_prefix)
                        && stored_form(u) == stripped
                        && *a == *artha
                        && verdict(n, u) == own_verdict
                })
                .count();
            let allowed = if IDENTICAL_UPSTREAM_PAIR.contains(&d.dhatupatha)
                || CONVERGENT_UPADESHA_PAIR.contains(&d.dhatupatha)
            {
                2
            } else {
                1
            };
            assert_eq!(
                siblings, allowed,
                "{} is ambiguous: {siblings} rows in gaṇa {gana_prefix} share \
                 ({stripped}, {artha})",
                d.dhatupatha
            );
        }
        // √śraṇ: `10.0174 SraRu~` and `10.0063 SraRa~` (curated since slice
        // 10k) share gaṇa, stored form and artha. Only the optional-ṇic
        // verdict tells them apart, so the two stay distinct rows.
        let row = |n: &str| *rows.iter().find(|(m, _, _)| *m == n).unwrap();
        let (_, sran_a, artha_a) = row("10.0063");
        let (_, sran_u, artha_u) = row("10.0174");
        assert_eq!(stored_form(sran_a), "SraR");
        assert_eq!(stored_form(sran_u), "SraR");
        assert_eq!(artha_a, artha_u);
        assert_eq!(optional_nic_from_upadesha("10.0174", sran_u), Some("2570"));
        assert_eq!(optional_nic_from_upadesha("10.0063", sran_a), None);
        assert_eq!(optional_nic("10.0174"), Some("2570"));
        assert_eq!(optional_nic("10.0063"), None);
        // `laGi~` twice: the pair is still identical upstream, and both rows
        // are curated.
        let (_, u1, a1) = row(IDENTICAL_UPSTREAM_PAIR[0]);
        let (_, u2, a2) = row(IDENTICAL_UPSTREAM_PAIR[1]);
        assert_eq!((u1, a1), ("laGi~", "BAzAyAm"));
        assert_eq!((u2, a2), (u1, a1));
        for n in IDENTICAL_UPSTREAM_PAIR {
            assert!(dhatus().iter().any(|d| d.dhatupatha == n), "{n}");
        }
        // `zAntva~` and `sAntva~`: the upadeśas differ only in their first
        // sound, the `z` 6.1.64 makes `s`, and the artha is shared, so both
        // store `sAntv`. Both rows are curated.
        let (_, z, artha_z) = row(CONVERGENT_UPADESHA_PAIR[0]);
        let (_, s, artha_s) = row(CONVERGENT_UPADESHA_PAIR[1]);
        assert_eq!((z, s), ("zAntva~", "sAntva~"));
        assert_eq!(z.strip_prefix('z'), s.strip_prefix('s'));
        assert_eq!((artha_z, artha_s), ("sAmaprayoge", "sAmaprayoge"));
        assert_eq!(stored_form(z), "sAntv");
        assert_eq!(stored_form(s), "sAntv");
        for n in CONVERGENT_UPADESHA_PAIR {
            assert!(dhatus().iter().any(|d| d.dhatupatha == n), "{n}");
        }
    }

    #[test]
    fn curadi_has_492_dhatus_and_only_zad_is_uncurated() {
        // Upstream numbers curādi's rows 10.0001-10.0509, but the last
        // seventeen, 10.0493-10.0509, are gaṇasūtras (`-` in the upadeśa
        // column), not dhātus. The gaṇa's denominator is therefore 492, and
        // the one dhātu still out is `10.0368 za\da~`, which waits for
        // upasargas. Curating it, or anything else, must update the counts.
        let rows: Vec<_> = upstream_rows()
            .into_iter()
            .filter(|(n, _, _)| n.starts_with("10."))
            .collect();
        let (dhatu_rows, sutra_rows): (Vec<_>, Vec<_>) =
            rows.iter().partition(|(_, u, _)| *u != "-");
        assert_eq!(dhatu_rows.len(), 492);
        let sutras: Vec<&str> = sutra_rows.iter().map(|(n, _, _)| *n).collect();
        let expected: Vec<String> = (493..=509).map(|i| format!("10.0{i}")).collect();
        assert_eq!(sutras, expected);
        let uncurated: Vec<&str> = dhatu_rows
            .iter()
            .map(|(n, _, _)| *n)
            .filter(|n| !dhatus().iter().any(|d| d.dhatupatha == *n))
            .collect();
        assert_eq!(uncurated, ["10.0368"]);
    }

    #[test]
    fn aya_is_exactly_the_curated_rows_3_1_28_names() {
        // 3.1.28 names five roots, and vidyut-prakriya keys it on their
        // upadeśas in any gaṇa. `AYA` is exactly the curated rows that carry
        // one. Non-circular: the upadeśa comes from the vendored dhātupāṭha.
        let rows = upstream_rows();
        let named = ["gupU~", "DUpa~", "viCa~", "paRa~\\", "pana~\\"];
        for u in named {
            assert!(rows.iter().any(|(_, v, _)| *v == u), "{u} is not upstream");
        }
        let mut curated: Vec<&str> = dhatus()
            .iter()
            .filter(|d| {
                rows.iter()
                    .any(|(n, u, _)| *n == d.dhatupatha && named.contains(u))
            })
            .map(|d| d.dhatupatha)
            .collect();
        curated.sort_unstable();
        assert_eq!(curated, AYA);
        // Curādi `10.0302 gupa~` stores as `gup`, as the bhvādi `gupU~` does,
        // and is not named: why `AYA` is keyed by number.
        let gupa = rows.iter().find(|(n, _, _)| *n == "10.0302").unwrap().1;
        assert_eq!(stored_form(gupa), stored_form("gupU~"));
        assert!(!AYA.contains(&"10.0302"));
    }

    #[test]
    fn curated_pada_agrees_with_upadesha_markers() {
        let rows = upstream_rows();
        let mut wrong: Vec<String> = Vec::new();
        for d in dhatus() {
            let (_, upadesha, _) = rows
                .iter()
                .find(|(n, _, _)| *n == d.dhatupatha)
                .unwrap_or_else(|| panic!("{} names no upstream row", d.dhatupatha));
            let derived = pada_from_upadesha(upadesha);
            // 1.3.66 Bujo'navane: the one curated verdict no marker can
            // carry. `Bu\ja~` has no pada anubandha at all — its `\` sits
            // on the root vowel, the exact conflation this function's doc
            // comment warns against — so the markers CORRECTLY derive
            // parasmaipada, and the ātmanepada arm comes from a sūtra that
            // names the root, which is why Pāṇini needed 1.3.66 in the
            // first place. Both sides are asserted so this exception fails
            // loudly if either the upadeśa reading or the curated column
            // ever drifts.
            if d.dhatupatha == "07.0017" {
                assert_eq!(
                    derived,
                    PadaAssignment::Parasmaipada,
                    "Bu\\ja~ grew a pada marker; 1.3.66's exception is stale"
                );
                assert_eq!(
                    d.pada,
                    PadaAssignment::UbhayapadaAnavane,
                    "07.0017 is the root 1.3.66 names; its pada is curated, not marker-derived"
                );
                continue;
            }
            // Curādi: no curated row's pada comes from a marker of its own,
            // so every upadeśa CORRECTLY derives parasmaipada, and the curated
            // column names the sanction instead. Inside `AKUSMIYA` it is the
            // gaṇasūtra 10.0496 ā kusmād ātmanepadinaḥ (`Akusmiya`,
            // ātmanepada only); inside `AA_GARVIYA`, its twin 10.0497 ā
            // garvād ātmanepadinaḥ (`AaGarviya`); outside both, the affix's
            // 1.3.74 ṇicaś ca (`Nic`, both padas). Asserted both ways, like √bhuj above, so a
            // row on the wrong side of the range boundary fails here. A marker
            // decides only a ṇic-less branch, so a marked curādi row is allowed
            // only where its ṇic is optional and the marker is svarita or ñit:
            // its ṇic-less branch is 1.3.72's (`NicUbhayapada`, slice 10h's six
            // ādhṛṣīya, and √ci). One marked row decides its ṇic branch too: a
            // ṅit row stays ātmanepadī under ṇic by Kaumudī 2567, and 1.3.12
            // credits (`10.0058 zmiN`, the one ṅit curādi row upstream). The
            // check reads the ṅ, not the verdict: an anudāttet row (`za\da~`)
            // marks ātmanepada too, but takes 1.3.74 under ṇic. Any other
            // marked curādi row fails the first assertion until a slice
            // decides its pada.
            if d.gana == Gana::Curadi {
                let ngit = upadesha.trim_end_matches(['\\', '^']).ends_with('N');
                let nicless_ubhaya =
                    optional_nic(d.dhatupatha).is_some() && derived == PadaAssignment::Ubhayapada;
                if ngit {
                    assert_eq!(
                        derived,
                        PadaAssignment::Atmanepada,
                        "{} {upadesha}: ṅit, so 1.3.12's",
                        d.dhatupatha
                    );
                } else if !nicless_ubhaya {
                    assert_eq!(
                        derived,
                        PadaAssignment::Parasmaipada,
                        "{} {upadesha}: a marked curādi row needs its own pada decision",
                        d.dhatupatha
                    );
                }
                let (want, why) = if AKUSMIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::Akusmiya, "ākusmīya, so 10.0496's")
                } else if AA_GARVIYA.contains(&d.dhatupatha) {
                    (PadaAssignment::AaGarviya, "ā-garvīya, so 10.0497's")
                } else if ngit {
                    (PadaAssignment::Atmanepada, "ṅit, so Kaumudī 2567's")
                } else if nicless_ubhaya {
                    (
                        PadaAssignment::NicUbhayapada,
                        "svarita or ñit with optional ṇic, so 1.3.74's and 1.3.72's",
                    )
                } else {
                    (
                        PadaAssignment::Nic,
                        "outside the ākusmīya and the ā-garvīya, so 1.3.74's",
                    )
                };
                assert_eq!(d.pada, want, "{} is curādi and {why}", d.dhatupatha);
                // That is the ṇic branch's pada. A row whose ṇic is optional
                // also derives without it, where its own markers decide by
                // 1.3.12 / 1.3.72 / 1.3.78: ubhayapadī for a `NicUbhayapada`
                // row, parasmaipada for every other row listed.
                if optional_nic(d.dhatupatha).is_some() {
                    let (nicless, by) = if d.pada == PadaAssignment::NicUbhayapada {
                        (PadaAssignment::Ubhayapada, "1.3.72")
                    } else {
                        (PadaAssignment::Parasmaipada, "1.3.78")
                    };
                    assert_eq!(
                        derived, nicless,
                        "{} {upadesha}: its ṇic-less branch is {by}'s",
                        d.dhatupatha
                    );
                    assert!(d.padas().contains(&Pada::Parasmaipada), "{}", d.dhatupatha);
                }
                continue;
            }
            if derived != d.pada {
                wrong.push(format!(
                    "{} {} ({upadesha}): curated {:?}, markers say {derived:?}",
                    d.dhatupatha, d.code, d.pada
                ));
            }
        }
        assert!(
            wrong.is_empty(),
            "pada column disagrees with the vendored upadeśa:\n  {}",
            wrong.join("\n  ")
        );
        // The ṅit arm above reaches one row: `10.0058 zmiN` is the one ṅit
        // curādi row upstream.
        let ngit_curadi: Vec<&str> = rows
            .iter()
            .filter(|(n, u, _)| {
                n.starts_with("10.") && u.trim_end_matches(['\\', '^']).ends_with('N')
            })
            .map(|(n, _, _)| *n)
            .collect();
        assert_eq!(ngit_curadi, ["10.0058"]);
    }

    #[test]
    fn bhi_is_parasmaipada_despite_its_initial_nyi() {
        // `YiBI\` carries a ñi that 1.3.5 ādir ñiṭuḍavaḥ makes an it — and
        // that is ALL it makes it. The it so supplied is not an anubandha
        // 1.3.72 svaritañitaḥ reads: vidyut-prakriya fires 1.3.72 on NONE
        // of the dhātupāṭha's fourteen `Yi`-initial rows (01.0594, 01.0844,
        // 01.0845, 01.0846, 01.0884, 01.1133, 02.0063, 03.0002, 04.0127,
        // 04.0141, 04.0158, 04.0159, 05.0025, 07.0011). Every ātmanepada
        // verdict among them comes from a `~\`, every parasmaipada one from
        // 1.3.78.
        //
        // √bhī is the first curated root to reach the clause this pins the
        // removal of; before slice 3b it decided nothing, which is how it
        // stayed wrong. `\` here sits on the root vowel, not on an it.
        assert_eq!(pada_from_upadesha("YiBI\\"), PadaAssignment::Parasmaipada);
    }

    #[test]
    fn indh_is_atmanepada_by_its_anudatta_it() {
        // `YiinDI~\`'s ātmanepada comes from the `~\` and nothing else. Its
        // initial ñi is an it by 1.3.5 but decides no pada — see
        // `bhi_is_parasmaipada_despite_its_initial_nyi`.
        //
        // This test used to be named `..._despite_satisfying_1_3_72` and
        // pinned the PRECEDENCE of 1.3.12 over 1.3.72, on the premise that
        // √indh satisfied both branches. It satisfied the second only via
        // the deleted `Yi` clause. The precedence is now carried by
        // `no_upadesha_satisfies_both_pada_branches` instead.
        assert_eq!(pada_from_upadesha("YiinDI~\\"), PadaAssignment::Atmanepada);
    }

    #[test]
    fn no_upadesha_satisfies_both_pada_branches() {
        // With the `Yi` clause gone, NO row in the dhātupāṭha satisfies both
        // of `pada_from_upadesha`'s branch conditions, so their order decides
        // nothing and swapping the two `if` blocks is an unkillable mutant.
        //
        // This test is what replaces that lost falsifiability: it asserts the
        // disjointness as an invariant of the DATA. If upstream ever grows a
        // row carrying both an anudātta/ṅ it and a svarita/ñ it, the
        // precedence question returns loudly instead of silently, and
        // 1.3.12-before-1.3.72 has to be re-argued rather than assumed.
        //
        // Same tripwire idiom as the `code`-uniqueness assertion in
        // `juhotyadi_rows_are_the_twenty_six_curated_roots`.
        //
        // This re-implements `pada_from_upadesha`'s two branch conditions
        // rather than calling the function, deliberately: an independent
        // encoding is what makes this a check ON the function instead of a
        // tautology restating it. The caveat that duplication buys: a future
        // `wu`/`qu` arm added to the function would be invisible here until
        // this test is updated to match.
        let mut both: Vec<&str> = Vec::new();
        for (_, upadesha, _) in upstream_rows() {
            let anudatta_it = upadesha.contains("~\\");
            let svarita_it = upadesha.contains("~^");
            let bare: String = upadesha
                .chars()
                .filter(|c| *c != '\\' && *c != '^')
                .collect();
            let final_it = bare.chars().last().filter(|c| is_hal(*c));
            let atmanepada_branch = anudatta_it || final_it == Some('N');
            let ubhayapada_branch = svarita_it || final_it == Some('Y');
            if atmanepada_branch && ubhayapada_branch {
                both.push(upadesha);
            }
        }
        assert!(
            both.is_empty(),
            "these upadeśas satisfy both branches, so their order is load-bearing again: {both:?}"
        );
    }

    #[test]
    fn a_final_hal_it_assigns_pada_without_any_tilde() {
        // 1.3.3 halantyam is the only marker these three have — no `~`
        // anywhere — so a check that looked only for `~\` / `~^` would call
        // all three parasmaipada and still agree with the column on two.
        assert_eq!(pada_from_upadesha("SIN"), PadaAssignment::Atmanepada); // 02.0026 √śī
        assert_eq!(pada_from_upadesha("vfN"), PadaAssignment::Atmanepada); // 09.0045 √vṛṅ
        assert_eq!(pada_from_upadesha("RI\\Y"), PadaAssignment::Ubhayapada); // 01.1049 √nī
    }

    #[test]
    fn a_root_vowel_accent_does_not_assign_pada() {
        // The failure mode that would make the whole audit vacuous: 43 of the
        // 66 curated roots carry a `\` somewhere in their upadeśa, 30 of them
        // on a root vowel rather than on an it. Reading every `\` as 1.3.12's
        // anudātta would call 24 of the 43 ātmanepada wrongly (15 curated
        // parasmaipada, 9 ubhayapada); the other 19 carry a genuine `~\` and
        // are curated ātmanepada anyway, which is why agreement with the
        // column would still hold on every genuinely ātmanepada root, and only
        // a parasmaipada witness catches it.
        assert_eq!(pada_from_upadesha("ji\\"), PadaAssignment::Parasmaipada); // 01.0642
        assert_eq!(pada_from_upadesha("a\\da~"), PadaAssignment::Parasmaipada); // 02.0001
        assert_eq!(pada_from_upadesha("Ba\\njo~"), PadaAssignment::Parasmaipada); // 07.0016
        // And the converse: the accent that DOES assign, on an it vowel.
        assert_eq!(pada_from_upadesha("Ki\\da~\\"), PadaAssignment::Atmanepada); // 07.0012
        assert_eq!(pada_from_upadesha("tu\\da~^"), PadaAssignment::Ubhayapada); // 06.0001
    }

    #[test]
    fn gana_matches_dhatupatha_prefix() {
        // The number's prefix encodes the gaṇa, so `Dhatu::gana` is redundant
        // with it. The field stays (the rule pipeline reads the enum
        // pervasively, and deriving it would mean parsing a string on every
        // lookup), and the redundancy becomes this check instead — a number
        // typed into the wrong gaṇa's block still names a real upstream row,
        // so nothing else would catch it.
        //
        // Mapped variant → prefix, not the inverse, so a new variant must
        // name its prefix here before it compiles.
        for d in dhatus() {
            let expected = match d.gana {
                Gana::Bhvadi => "01",
                Gana::Adadi => "02",
                Gana::Juhotyadi => "03",
                Gana::Divadi => "04",
                Gana::Svadi => "05",
                Gana::Tudadi => "06",
                Gana::Rudhadi => "07",
                Gana::Tanadi => "08",
                Gana::Kryadi => "09",
                Gana::Curadi => "10",
            };
            assert!(
                d.dhatupatha.starts_with(expected),
                "{:?} root {} has number {}, which is not in gaṇa {expected}",
                d.gana,
                d.code,
                d.dhatupatha
            );
        }
    }
}
