# Data Attribution

The Dhātupāṭha entries, gaṇa assignments, and tiṅ-pratyāhāra endings in this
directory follow the standard Pāṇinian corpus. Where values were cross-checked
against openly-licensed digital sources, those sources and their licenses are
recorded below.

- Cross-reference: ashtadhyayi.com derivations (per-form).
- Cross-reference: Vidyut (github.com/ambuda-org/vidyut), data files under
  their stated open license — consult the upstream LICENSE before importing any
  file verbatim; v1 uses only the small curated subset transcribed here.
- Cross-reference: the kryādi gaṇa (gaṇa 9) entries correspond to Dhātupāṭha
  numbers 09.0058 (`kliSU~`), 09.0053 (`guDa~`) and 09.0059 (`aSa~`).
- Cross-reference: the kryādi gaṇa (gaṇa 9), slice 9b, entries correspond to
  Dhātupāṭha numbers 09.0066 (`muza~`), 09.0040 (`vrI\`) and 09.0045
  (`vfN`). √vṛṅ (`vf`) is the only pure-ātmanepadī root in the gaṇa; every
  other ātmanepada form attested in kryādi belongs to an ubhayapadī root,
  and no kryādi ubhayapadī root is curated. `panini-data` carries
  `PadaAssignment::Ubhayapada` since the 1.3.72 slice, so the pada model is
  no longer what stands in their way; what else any one of them may need has
  not been checked.
- Cross-reference: the rudhādi gaṇa (gaṇa 7), slice 7a, entries correspond to
  Dhātupāṭha numbers 07.0010 (`kftI~`), 07.0019 (`hisi~`) and 07.0012
  (`Ki\da~\`). √hiṃs is stored as `hins` — post-7.1.58 *idito num dhātoḥ* —
  because the engine models no it-markers and so cannot derive the num; a
  stated simplification of the source entry, recorded in `panini-data`.
- Cross-reference: the rudhādi gaṇa (gaṇa 7), slice 7b, entries correspond to
  Dhātupāṭha numbers 07.0016 (`Ba\njo~`), 07.0015 (`pi\zx~`) and 07.0011
  (`YiinDI~\`). √indh's pada is **ātmanepada**, taken from vidyut-prakriya's
  derivations rather than read off the entry: its `Yi` it-marker is one of
  the two things 1.3.72 *svaritañitaḥ* reads, so the entry alone would
  suggest ubhayapadī. The `~\` anudātta settles it by 1.3.12, and
  vidyut-prakriya derives the root in ātmanepada only — checked against a
  `~^r` control (07.0001 `ru\Di~^r`) that does derive both padas.
- Cross-reference: the rudhādi gaṇa (gaṇa 7), ubhayapada slice, adds
  Dhātupāṭha number 07.0001 (`ru\Di~^r`) — the gaṇa's eponym, and the first
  entry recorded as **ubhayapada**. Its `~^` svarita is what 1.3.72
  *svaritañitaḥ* reads, and — unlike `YiinDI~\` directly above — the entry
  carries no trailing `~\` anudātta it-marker for 1.3.12 to settle it with
  (the `\` it does carry is the root vowel's own accent), so it stands as
  ubhayapadī; vidyut-prakriya derives it in both padas.
- Cross-reference: the rudhādi gaṇa (gaṇa 7), slice 7d, entries correspond
  to Dhātupāṭha numbers 07.0013 (`vi\da~\`), 07.0014 (`Si\zx~`), 07.0020
  (`undI~`), 07.0021 (`anjU~`), 07.0022 (`tancU~`), 07.0023 (`o~vijI~`),
  07.0024 (`vfjI~`) and 07.0025 (`pfcI~`). None of the eight stores a
  `code` differing from a plain it-strip of its vendored upadeśa — unlike
  `07.0019` (`hisi~`, stored `hins`, post-7.1.58 *idito num dhātoḥ*, in
  the slice 7a bullet above) — so this slice records no per-entry
  deviation. `dhatupatha_numbers_resolve_upstream` already proved it in
  Task 1 Step 5; this bullet records it.
- Cross-reference: the rudhādi gaṇa (gaṇa 7), slice 7e, entries correspond
  to Dhātupāṭha number 07.0018 (`tfha~`). It stores a `code` of `tfh`, its
  it-stripped upadeśa, so this slice too records no per-entry deviation.
- Cross-reference: the rudhādi gaṇa (gaṇa 7), slice 7f, entries correspond
  to Dhātupāṭha numbers 07.0003 (`Ci\di~^r`) and 07.0008 (`u~Cfdi~^r`).
  Both store a `code` that is a plain it-strip of its vendored upadeśa —
  `Cid` and `Cfd` — so this slice too records no per-entry deviation.
- The gaṇa is **partial**: nine of rudhādi's 25 entries are ubhayapadī
  (`~^r`-marked), nine of them now curated — √rudh (`ru\Di~^r`), slice 7c's
  `Bi\di~^r`, `kzu\di~^r`, `yu\ji~^r` and `u~tfdi~^r`, the 8.2.30/8.2.39
  generalization slice's `ri\ci~^r` and `vi\ci~^r`, and slice 7f's
  `Ci\di~^r` and `u~Cfdi~^r` (with 6.1.73 *che ca* and 8.4.40 *stoḥ ścunā
  ścuḥ*). 1.3.72 is implemented, so the ubhayapada deferral is discharged;
  none of the nine remain out. The
  gaṇa's other axis, non-ubhayapadī, is narrower now: sixteen of the 25
  entries are non-ubhayapadī, slice 7d's eight took the curated count
  there to fourteen on the audited numbers alone with no new sūtra, and
  slice 7e's `tfha~` (07.0018, √tṛh, with 7.3.92 *tṛṇaha im*, 8.2.31 *ho
  ḍhaḥ* and 8.3.13 *ḍho ḍhe lopaḥ*) takes it to fifteen.
  Only one non-ubhayapadī entry remains out — `Bu\ja~` (07.0017, √bhuj,
  deferred behind 1.3.66 *bhujo'navane*: vidyut derives all 72 of its
  cells, and 1.3.66 is the only rule this engine lacks for it, a
  root-keyed pada assignment structurally identical to 1.3.72's, which
  the engine already implements; what keeps it out is the *anavane*
  **sense** restriction, which neither engine models, not the cost) —
  so what was, before slice 7e, a four-root
  remainder is now a one-root one: √bhuj alone,
  held out by the *anavane* sense restriction rather than by curation
  lag or sūtra cost.
- Cross-reference: the tanādi gaṇa (gaṇa 8), slice 8a, entries correspond to
  Dhātupāṭha numbers 08.0001 (`tanu~^`), 08.0002 (`zaRu~^`), 08.0003
  (`kzaRu~^`), 08.0004 (`kziRu~^`), 08.0005 (`fRu~^`), 08.0006 (`tfRu~^`),
  08.0007 (`GfRu~^`), 08.0008 (`vanu~\`) and 08.0009 (`manu~\`). 08.0002
  is the second `dhAtvAdeH SaH saH` row 6.1.64 stores — after √ṣṭigh
  (`stiG`, gaṇa 5) — and shares its shape: the upadeśa's `z` becomes `s`,
  and with the `z` gone its conditioned retroflex `R` reverts to `n`
  (nimitta-nāśa), so the stored `code` is `san`, not `saR`, matching what
  vidyut-prakriya derives (`sanoti`/`sanute`). `08.0008` (`vanu~\`) is
  ātmanepadī by its own anudātta marker (1.3.12); vidyut-prakriya
  additionally derives a parasmaipada `vanoti` via the gaṇasūtra Kaumudī
  2547.2, which is recorded here, not modelled, on 1.3.72's own
  sense-restriction precedent — the same record-don't-model posture, so
  the parasmaipada column this table does not carry for `van` is a
  documented deviation, not a latent diff.
- The gaṇa is **partial**: nine of tanādi's ten dhātupāṭha rows are
  curated. Seven are ubhayapadī by 1.3.72 (`tan`, `san`, `kzaR`, `kziR`,
  `fR`, `tfR` and `GfR`) and two are ātmanepadī by 1.3.12 (`van` and
  `man`). The tenth row, `08.0010` (`qukfY`, √kṛ, ñit, ubhayapadī), is
  out: 3.1.79 *tanādikṛñbhya uḥ* itself names it (*kṛñbhya*), but the root
  additionally wants 6.4.108 *nityaṁ karoteḥ* and the other 6.4.10x
  kṛ-specials this engine does not implement, deferred to slice 8b.
