# Juhotyādi gaṇa (gaṇa 3) — the layout prep and slice 3a

Ninth gaṇa, the ślu gaṇa (2.4.75 *juhotyādibhyaḥ śluḥ*). This spec covers
two slices in full — a structural **prep** that gives the term layout the
slots reduplication needs, on byte-identical goldens, and **3a**, which lands
dvitva and the abhyāsa core on √hu and √ki — and records the rule bundles of
the five slices that follow (3b–3f) so each later spec starts from an
audited inventory rather than a fresh probe. After 3f the gaṇa is closed at
all twenty-six of its dhātupāṭha rows and only curādi (10) remains.

## Summary

Gaṇa 3 is the first gaṇa whose vikaraṇa leaves a trace after it is gone:
ślu, unlike adādi's luk, triggers reduplication (6.1.10 *ślau*), and the
reduplicant — the abhyāsa — is a term of its own with its own saṁjñā
(6.1.4 *pūrvo'bhyāsaḥ*), its own shaping rules (7.4.59–7.4.78), its own
tripādī rule (8.4.54 *abhyāse car ca*) and consequences for the ending
(7.1.4 *ad abhyastāt*, 3.4.109 *sijabhyastavidibhyaś ca*) and for guṇa
(7.3.83 *jusi ca*, 7.3.87 *nābhyastasyāci piti sārvadhātuke*).

vidyut-prakriya, at the pinned commit `8da2f90b`, derives all 26 rows in
all four lakāras. Six rows are ubhayapadī (√bhṛ, √dā, √dhā, √nij, √vij,
√viṣ), two ātmanepadī (√mā, √hā ṅit) and eighteen parasmaipadī: **1152
cells**, taking the suite from 3492 to 4644 (+33%).

The engine's fixed three-slot layout (ANGA, SHAP, ENDING) has no place for
a term before the aṅga, and laṅ's aṭ is prefixed onto the aṅga's *text*.
Reduplication cannot ride inside the text the way aṭ does: 8.4.54 in the
tripādī must find the abhyāsa's own initial consonant after aṭ has been
added, and 6.1.90's ekādeśa of āṭ with √ṛ's abhyāsa (*aiyaḥ*) merges the
augment into the abhyāsa, not the root. So the layout grows two permanent
leading slots, and that is the prep.

## Scope

In scope, **prep**: the five-slot layout and the augment term. No new
gaṇa, no new rule, 3492 cells and every trace byte-identical.

In scope, **3a**: √hu (`03.0001`) and √ki (`03.0020`), parasmaipadī, four
lakāras, **72 cells** (84 forms), suite 3492 → 3564, with 2.4.75, 6.1.10,
7.4.62, 7.1.4, 3.4.109, 7.3.83, 6.4.82, 8.4.54 new and 6.4.87 widened.

Recorded for later specs, **3b–3f**: the remaining 24 rows, tabled under
"Later slices" below.

Out of scope, deferred:

- **curādi** — its own future spec, and its own layout question (ṇic sits
  between the dhātu and the vikaraṇa).
- **Multi-ekāc reduplication** (6.1.1's "first ekāc", 6.1.2 *ajāder
  dvitīyasya* beyond the monosyllabic case). Every juhotyādi root is a
  single ekāc when 6.1.10 fires; liṭ is the first customer for the general
  case.
- **Sense conditions**, as ever.
- **6.1.64 *dhātvādeḥ ṣaḥ saḥ*** and 6.1.65 *ṇo naḥ*: rows ship post-rule
  in the curated text, the convention `zwiGa~\` → `stiG` set. Whether 3e
  stores `nij` for `Ri\ji~^r` or implements 6.1.65 is 3e's to decide.
- **7.1.35 tātaṅ and 8.4.56 vāvasāne** remain the repo-wide conventions
  they were.

## Root selection

All 26 rows of `data/dhatupatha.tsv`'s gaṇa 3, in the order the later
slices take them. Pada is the audit's verdict; the marker column is what
`curated_pada_agrees_with_upadesha_markers` checks it against.

| slice | row | entry | root (SLP1) | pada | sanction |
|---|---|---|---|---|---|
| 3a | 03.0001 | hu\ | hu | parasmai | 1.3.78 (the `\` is svara, not an anubandha) |
| 3a | 03.0020 | ki\ | ki | parasmai | 1.3.78 |
| 3b | 03.0002 | YiBI\ | BI | parasmai | 1.3.78 (the Yi carries no anudātta) |
| 3b | 03.0003 | hrI\ | hrI | parasmai | 1.3.78 |
| 3c | 03.0010 | qudA\Y | dA | ubhaya | 1.3.72 (ñit) |
| 3c | 03.0011 | quDA\Y | DA | ubhaya | 1.3.72 (ñit) |
| 3c | 03.0007 | mA\N | mA | ātmane | 1.3.12 (ṅit) |
| 3c | 03.0008 | o~hA\N | hA | ātmane | 1.3.12 (ṅit) |
| 3c | 03.0009 | o~hA\k | hA | parasmai | 1.3.78 |
| 3c | 03.0026 | gA\ | gA | parasmai | 1.3.78 |
| 3d | 03.0006 | quBf\Y | Bf | ubhaya | 1.3.72 (ñit) |
| 3d | 03.0005 | pf\ | pf | parasmai | 1.3.78 |
| 3d | 03.0004 | pF | pF | parasmai | 1.3.78 |
| 3d | 03.0015 | Gf\ | Gf | parasmai | 1.3.78 |
| 3d | 03.0016 | hf\ | hf | parasmai | 1.3.78 |
| 3d | 03.0018 | sf\ | sf | parasmai | 1.3.78 |
| 3d | 03.0017 | f\ | f | parasmai | 1.3.78 |
| 3e | 03.0012 | Ri\ji~^r | nij | ubhaya | 1.3.72 (svarita) |
| 3e | 03.0013 | vi\ji~^r | vij | ubhaya | 1.3.72 (svarita) |
| 3e | 03.0014 | vi\zx~^ | viz | ubhaya | 1.3.72 (svarita) |
| 3f | 03.0019 | Basa~ | Bas | parasmai | 1.3.78 |
| 3f | 03.0021 | kita~ | kit | parasmai | 1.3.78 |
| 3f | 03.0022 | tura~ | tur | parasmai | 1.3.78 |
| 3f | 03.0023 | Diza~ | Diz | parasmai | 1.3.78 |
| 3f | 03.0024 | Dana~ | Dan | parasmai | 1.3.78 |
| 3f | 03.0025 | jana~ | jan | parasmai | 1.3.78 |

Two rows carry a flag for their slice, neither of which touches 3a:
√dhan's loṭ madhyama eka *dadhaṃhi* needs 8.3.24 before `h`, which the
engine models only before a jhal (`h` is one, so the existing arm should
reach it — verify, don't assume); √gā's *jigāti* rides 7.4.78 *bahulaṁ
chandasi*, a chāndasa rule vidyut applies on the Kaumudī's authority, and
the 3c spec must decide whether to transcribe that attribution or record
it as a stated convention.

## Prep: the term layout

### Constants — `terms.rs`

Two new slots, both permanent, both empty outside the derivations that
fill them:

| constant | value | holds |
|---|---|---|
| `AGAMA` | 0 | laṅ's aṭ / āṭ (6.4.71 / 6.4.72); empty otherwise |
| `ABHYASA` | 1 | the reduplicant (6.1.10); empty outside juhotyādi |
| `ANGA` | 2 | the dhātu, as today |
| `ENDING_PRE_SHAP` / `SHAP` | 3 | as today, still deliberately equal |
| `ENDING` | 4 | as today |

`derive` pushes two empty terms before the dhātu. The 3.1.68 bisection,
both empty-SHAP caveats and the śnam split survive verbatim: they are
stated relative to the constants. `following_sarvadhatuka`,
`sound_before_ending` and `word_chars` are already layout-agnostic — the
first two read the nearest non-empty term, the third addresses the whole
word across terms — so the tripādī and 6.1.73 *che ca* need no edit. The
**48 lines** that index `terms` by a literal number (`terms[0]`,
`terms[1]`, `terms[2]`) migrate to the constants in the same commit; a
grep for `terms\[[0-9]\]` outside `derivation_tests.rs` is a review step,
because a missed literal still compiles and silently reads the empty
augment.

This is the same empty-term-in-place idiom 2.4.72 established: a slot that
exists so the indices stay valid, and that every rule reading it must
treat as possibly empty.

### The augment leaves the aṅga's text — `anga.rs`, `adesha.rs`

Exactly three sites read the aṅga's initial character or the aṭ log, and
they are the whole of the prep's behaviour change:

- **6.4.71** keeps its consonant-initial guard on `ANGA` and writes `a`
  into `AGAMA` instead of prefixing it onto `ANGA.text`.
- **6.4.72** writes `A` into `AGAMA`. Its "did 6.4.71 fire" log scan is
  deleted with no replacement: once 6.4.71 leaves the aṅga's text alone,
  `is_vowel(first)` alone separates a vowel-initial root from one 6.4.71
  just augmented, and an "is `AGAMA` empty" clause in the scan's place
  could never be falsified (6.4.71 declines for every vowel-initial aṅga),
  which makes it a guaranteed mutation survivor.
- **6.1.90's aṅga arm** reads `AGAMA`'s `A` plus the initial vowel of the
  first non-empty term after it, writes the vṛddhi into that term and
  empties `AGAMA` — vidyut's own shape. In the prep that term is only ever
  `ANGA` (√ad, √īkṣ, √edh); in 3d it is √ṛ's abhyāsa.

Every guard that matched the augment inside the aṅga's text — 8b's
`ends_with("kar")` family, the 6.4.106/6.4.107 note about reading the aṅga
before the āṭ ekādeśa — keeps working, because the text it matches is now
the whole term. Those comments are corrected in the sweep, not rewritten;
the wart they worked around is gone.

### Why byte-identical

`Prakriya::text` and `snapshot` concatenate every term, so an empty
leading term contributes nothing and the before/after strings of every
logged step are unchanged. The prep's bar is therefore the strongest the
repo has: 3492 cells and every pinned trace identical, no golden
regenerated, no new rule, no new gaṇa tag. `Tag::Abhyasa` in `term.rs`,
unused today, stays unused until 3a. Keeping the prep purely structural is
the svādi spec's isolation logic for its `anga.rs` split: a large diff with
no behaviour change is where a rule silently changes position, and
nothing else in the prep should need grammatical review.

## Slice 3a: the reduplication core

### Data

`Gana::Juhotyadi` and `Tag::Juhotyadi` mirror the seven existing gaṇa
tags across the 25 sites that carry `Tanadi` (`panini-data/lib.rs`,
`term.rs`, `tinanta/mod.rs`, `vikarana.rs`, `guna.rs`). Rows 03.0001 and
03.0020 are parasmaipadī.

### A new stage — `abhyasa.rs`

Between `vikarana` and `anga` in `TINANTA_RULES`. It holds dvitva and
every rule that reshapes the abhyāsa, so that 6.4.71 always sees a
finished abhyāsa and 7.1.4 reads a real abhyasta tag. The svādi spec asked
for exactly this: reduplication does not belong in `guna.rs`.

**Dvitva runs before guṇa.** vidyut copies the guṇated stem and shortens
it back by 7.4.59 (*ho ho* → *hu ho*); the Kaumudī order — ślu, dvitva,
abhyāsa-kārya, then guṇa — copies the root. For all 26 roots the two
orders give the same forms (checked against the probe in the appendix),
and the traditional order lets 7.4.59 *hrasvaḥ* fire only on the
long-vowel roots it names. The audit compares form sets, not traces, so
the divergence is visible only in this engine's own pins; ARCHITECTURE's
rule-order notes record it so no later reader chases it.

**6.1.4 and 6.1.5 are tags, not steps**, on the 3.4.113 precedent: every
saṁjñā verdict lives as a tag.

### The rules, in pipeline order

- **3.4.109 *sijabhyastavidibhyaś ca*** — `tin.rs`, beside 3.4.108. laṅ
  prathama bahu jhi → jus. It runs before 3.1.68, before dvitva exists,
  so it guards on `Tag::Juhotyadi` as the data-layer stand-in for
  *abhyasta*, which every cell of this gaṇa in these four lakāras entails;
  the comment records the equivalence and names liṭ as the slice that
  must revisit it. *ajuhavuḥ*, *acikayuḥ*.
- **2.4.75 *juhotyādibhyaḥ śluḥ*** — `vikarana.rs`, beside 2.4.72. Empties
  SHAP in place, drops `Tag::Thematic`, adds a new **`Tag::Slu`** to the
  term. Slu is what separates ślu from luk: 6.1.10 reads it and nothing
  else does.
- **6.1.10 *ślau*** — `abhyasa.rs`. When SHAP carries Slu, copies
  `ANGA.text` into `ABHYASA`; sets `Tag::Abhyasa` on the copy (6.1.4) and
  `Tag::Abhyasta` on both terms (6.1.5). Monosyllabic aṅga only, by the
  scope note above.
- **7.4.62 *kuhoś cuḥ*** — `abhyasa.rs`. The abhyāsa's initial ku or `h`
  becomes the palatal, `h` → `J` as vidyut has it. *hu* → *Ju*, *ki* →
  *ci*. The inverse of the existing `kutva_of` map is the substitute.
- **7.1.4 *ad abhyastāt*** — `anga.rs`, before 7.1.3 as its apavāda. jhi
  → ati when `ANGA` is tagged abhyasta. *juhvati*, *cikyati*. 7.1.3
  self-guards because the jh is gone.
- **7.3.83 *jusi ca*** — `guna.rs`. Guṇa of the aṅga's final ik before an
  *immediately following* jus, overriding 1.1.5's ṅit block. *ajuho-*,
  then the existing 6.1.78 → *ajuhavuḥ*. Vidhiliṅ's jus is untouched
  because yāsuṭ intervenes.
- **6.4.87's hu arm** — `guna.rs`, existing rule. Guard widens from "SHAP
  is `nu`" to "SHAP is `nu`, or the aṅga is √hu under ślu" (`ANGA.text ==
  "hu"` with `Tag::Juhotyadi`). *juhvati*. The aṅga-text test is safe
  only because the prep moved aṭ out of the text — which is why the prep
  comes first.
- **6.4.82 *er anekāco'saṁyogapūrvasya*** — `guna.rs`, before 6.4.77 as
  its apavāda. A final `i` of a polysyllabic aṅga, not preceded by a
  conjunct, becomes `y` before a vowel. *Anekāc* is counted over `ABHYASA`
  plus `ANGA` together: the abhyasta whole is the aṅga. *cikyati*.
- **8.4.54 *abhyāse car ca*** — `tripadi.rs`. The abhyāsa's jhal becomes
  the corresponding car (jaś by 8.4.53's anuvṛtti). In 3a only *Ju* →
  *ju*; 3b widens it to `B` → `b`.

Everything else is existing machinery on its existing guards: 7.3.84 for
*juhoti* and *ciketi*, 6.4.101 for *juhudhi*, 8.3.59 for *juhoṣi*, the
tātaṅ and 8.4.56 forks, and the adādi athematic arms, which see an empty
SHAP exactly as they do for √ad.

### Counts

72 cells, 84 forms, no new alternates beyond the standing loṭ and
pada-final forks. Suite 3492 → 3564; roots 77 → 79.

## Data

- Curated rows for 03.0001 and 03.0020 (3a), keyed by number.
- Goldens in the split suite's directory form:
  `tests/paradigm/data/juhotyadi.rs` and `tests/trace/juhotyadi.rs`,
  wired into both `main.rs` files the way the eight existing gaṇa files
  are.
- The audit copies the committed `tools/audit` harness — never rewritten —
  with `/tmp/vidyut-full`'s dev-deps repointed at the working tree. The two
  throwaway probes this spec drew on (`examples/juhotyadi_probe.rs`,
  `examples/juhotyadi_trace.rs` under `/tmp/vidyut-full/vidyut-prakriya`)
  stay uncommitted; their output is the appendix.

## Testing

**Prep.**

- 3492 cells and every trace byte-identical, no golden regenerated.
- Per-rule guard tests for 6.4.71, 6.4.72 and 6.1.90's aṅga arm in their
  new shape; a `terms.rs` test that a fresh prakriya has exactly two
  leading empty terms and that `text()` ignores them.
- The audit runs as a negative control only: it can confirm nothing the
  goldens do not already prove.
- Mutation floor re-measured, gate at the standing `-j 4 --timeout 4800`,
  `timeout.txt` checked for anything beyond the permanent ṇatva-scan
  entry, AGENTS.md updated.

**3a.**

- Per-rule guard tests in the slice-7 style for each of the eight new
  rules and the widened 6.4.87.
- The pinned rule order gains 3.4.109, 2.4.75, 6.1.10, 7.4.62, 7.1.4,
  7.3.83, 6.4.82, 8.4.54.
- Byte-identical 3492 priors guard the 6.4.87 widening and the 7.1.4
  apavāda.
- Trace pins: *juhoti* orders 2.4.75 < 6.1.10 < 7.4.62 < 7.3.84 < 8.4.54
  and carries no 1.2.4; *juhvati* credits 7.1.4 and 6.4.87 and must not
  credit 7.1.3 or 6.1.77; *ajuhavuḥ* credits 3.4.109 and 7.3.83 and shows
  the augment in `AGAMA`; *cikyati* credits 6.4.82 and not 6.4.77.
- `derivation_set_shape_matches_the_audited_numbers` moves to 3564 cells /
  79 roots, and the audit's asserted totals move with it.
- Mutation floor re-measured, not scaled; same gate and bookkeeping as the
  prep.

## Risks

1. **The prep's literal-index migration.** A missed `terms[0]` still
   compiles and reads the empty augment. Mitigated by the byte-identical
   bar, which any such miss breaks loudly, and by the grep review step.
2. **The augment's ekādeśa moves across terms.** 6.1.90's aṅga arm is the
   one rule that rewrites two terms at once. Its existing goldens for √ad,
   √īkṣ and √edh are the pins.
3. **Dvitva before guṇa diverges from vidyut's trace order.** Forms agree
   for all 26 roots; the audit compares sets. Recorded in ARCHITECTURE.
4. **The 3.4.109 tag stand-in.** If a lakāra ever reduplicates
   conditionally, the guard is wrong. liṭ's first job; the comment says so.
5. **Suite cost.** At ~33 minutes uncontended and a ~16-hour campaign,
   each of the seven slices pays the full gate — the cadence chosen for
   this gaṇa. If a pre-campaign projection exceeds 4800s, the cap is
   raised in AGENTS.md and `mise.toml` together, never silently.

## Success criteria

- **Prep:** 3492 cells and traces identical; gate clean (`missed.txt`
  empty, `timeout.txt` holding only the known permanent entry) at a
  re-measured cap; no new rule; AGENTS.md timing appended; ARCHITECTURE's
  term-layout paragraph rewritten for five slots.
- **3a:** 3564 cells VALID with pinned traces; 3492 priors identical;
  audit zero-divergence on the two roots; gate clean; README and
  ARCHITECTURE at "nine gaṇas" with the juhotyādi paragraph; AGENTS.md
  timing appended.

## Later slices

Each gets its own spec, starting from this table and the appendix.

| slice | roots | cells | new machinery |
|---|---|---|---|
| 3b | √bhī, √hrī | 72 | 7.4.59 *hrasvaḥ*, 7.4.60 *halādiḥ śeṣaḥ*, 6.4.115 *bhiyo'nyatarasyām* (vikalpa), 6.4.77's dhātu arm; 8.4.54 widened to `B` |
| 3c | √dā, √dhā, √mā, √hā ×2, √gā | 288 | 1.1.20 *dādhā ghv adāp* (saṁjñā), 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*, 8.2.38 *dadhas taṭhoś ca*, 6.4.116 / 6.4.117 (vikalpa) / 6.4.118 for √hā, 7.4.76 *bhṛñām it*, 7.4.78, 6.1.88 *vṛddhir eci*; 6.4.112 / 6.4.113's abhyasta arms |
| 3d | √bhṛ, √pṛ, √pṝ, √ghṛ, √hṛ, √sṛ, √ṛ | 288 | 7.4.66 *ur at* (+1.1.51), 7.4.77 *arti-pipartyoś ca*, 7.1.102 *ud oṣṭhyapūrvasya*, 6.4.78 *abhyāsasyāsavarṇe*, 6.4.72 / 6.1.90 on the abhyāsa, 8.3.110 |
| 3e | √nij, √vij, √viṣ | 216 | 7.4.75 *nijāṁ trayāṇāṁ guṇaḥ ślau*, 7.3.87 *nābhyastasyāci piti sārvadhātuke*, 6.1.65 *ṇo naḥ* (or the stored-form convention) |
| 3f | √bhas, √kit, √tur, √dhiṣ, √dhan, √jan | 216 | 6.4.98 *gamahanajanakhanaghasāṁ lopaḥ*, 6.4.42 / 6.4.43 *ye vibhāṣā* (vikalpa), 6.4.100 *ghasibhasor hali ca*, 8.2.26 *jhalo jhali*; 8.3.24 before `h` for √dhan |

## Appendix: vidyut's inventory at `8da2f90b`

Per root, over four lakāras and both padas where derivable:

| row | entry | padas × cells | cells | forms |
|---|---|---|---|---|
| 03.0001 | `hu\` | Parasmaipada=36 | 36 | 42 |
| 03.0002 | `YiBI\` | Parasmaipada=36 | 36 | 70 |
| 03.0003 | `hrI\` | Parasmaipada=36 | 36 | 42 |
| 03.0004 | `pF` | Parasmaipada=36 | 36 | 41 |
| 03.0005 | `pf\` | Parasmaipada=36 | 36 | 41 |
| 03.0006 | `quBf\Y` | Parasmaipada=36, Atmanepada=36 | 72 | 77 |
| 03.0007 | `mA\N` | Atmanepada=36 | 36 | 36 |
| 03.0008 | `o~hA\N` | Atmanepada=36 | 36 | 36 |
| 03.0009 | `o~hA\k` | Parasmaipada=36 | 36 | 61 |
| 03.0010 | `qudA\Y` | Parasmaipada=36, Atmanepada=36 | 72 | 78 |
| 03.0011 | `quDA\Y` | Parasmaipada=36, Atmanepada=36 | 72 | 78 |
| 03.0012 | `Ri\ji~^r` | Parasmaipada=36, Atmanepada=36 | 72 | 79 |
| 03.0013 | `vi\ji~^r` | Parasmaipada=36, Atmanepada=36 | 72 | 79 |
| 03.0014 | `vi\zx~^` | Parasmaipada=36, Atmanepada=36 | 72 | 79 |
| 03.0015 | `Gf\` | Parasmaipada=36 | 36 | 41 |
| 03.0016 | `hf\` | Parasmaipada=36 | 36 | 41 |
| 03.0017 | `f\` | Parasmaipada=36 | 36 | 41 |
| 03.0018 | `sf\` | Parasmaipada=36 | 36 | 41 |
| 03.0019 | `Basa~` | Parasmaipada=36 | 36 | 44 |
| 03.0020 | `ki\` | Parasmaipada=36 | 36 | 42 |
| 03.0021 | `kita~` | Parasmaipada=36 | 36 | 44 |
| 03.0022 | `tura~` | Parasmaipada=36 | 36 | 41 |
| 03.0023 | `Diza~` | Parasmaipada=36 | 36 | 43 |
| 03.0024 | `Dana~` | Parasmaipada=36 | 36 | 41 |
| 03.0025 | `jana~` | Parasmaipada=36 | 36 | 51 |
| 03.0026 | `gA\` | Parasmaipada=36 | 36 | 42 |

Rules vidyut credits on these roots that this engine does not carry under
any id (rule, number of roots, rows). Ids the engine records under another
convention — the 1.3.x it-lopa family, 3.4.113, 8.2.66, 8.4.68, 3.4.107,
6.1.68 (this engine credits 8.2.23), 1.1.5, 1.1.51, 3.2.x, 3.3.x — are
listed for completeness and are not new work:

| rule | roots | rows (03.xxxx) |
|---|---|---|
| 1.1.20 | 2 | 0010,0011 |
| 1.1.5 | 17 | 0001,0002,0003,0004,0005,0006,0012,0013,0014,0015,0016,0017,0018,0020,0021,0022,0023 |
| 1.1.51 | 7 | 0004,0005,0006,0015,0016,0017,0018 |
| 1.3.1 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 1.3.2 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 1.3.3 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 1.3.3.1 | 2 | 0012,0013 |
| 1.3.4 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 1.3.5 | 4 | 0002,0006,0010,0011 |
| 1.3.7 | 24 | 0001,0002,0003,0004,0005,0006,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 1.4.13 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 1.4.14 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 2.4.75 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 3.2.111 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 3.2.123 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 3.3.161 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 3.3.162 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 3.4.107 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 3.4.109 | 24 | 0001,0002,0003,0004,0005,0006,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 3.4.113 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 6.1.10 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 6.1.4 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 6.1.5 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 6.1.65 | 1 | 0012 |
| 6.1.68 | 16 | 0004,0005,0006,0012,0013,0014,0015,0016,0017,0018,0019,0021,0022,0023,0024,0025 |
| 6.1.88 | 4 | 0007,0008,0010,0011 |
| 6.4.100 | 1 | 0019 |
| 6.4.115 | 1 | 0002 |
| 6.4.116 | 1 | 0009 |
| 6.4.117 | 1 | 0009 |
| 6.4.118 | 1 | 0009 |
| 6.4.119 | 2 | 0010,0011 |
| 6.4.42 | 1 | 0025 |
| 6.4.43 | 1 | 0025 |
| 6.4.78 | 1 | 0017 |
| 6.4.82 | 2 | 0002,0020 |
| 6.4.98 | 1 | 0025 |
| 7.1.102 | 1 | 0004 |
| 7.1.4 | 24 | 0001,0002,0003,0004,0005,0006,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 7.3.83 | 11 | 0001,0002,0003,0004,0005,0006,0015,0016,0017,0018,0020 |
| 7.3.87 | 9 | 0012,0013,0014,0019,0021,0022,0023,0024,0025 |
| 7.4.59 | 16 | 0001,0002,0003,0007,0008,0009,0010,0011,0012,0013,0014,0020,0021,0022,0023,0026 |
| 7.4.60 | 17 | 0003,0004,0005,0006,0012,0013,0014,0015,0016,0017,0018,0019,0021,0022,0023,0024,0025 |
| 7.4.62 | 9 | 0001,0003,0008,0009,0015,0016,0020,0021,0026 |
| 7.4.66 | 7 | 0004,0005,0006,0015,0016,0017,0018 |
| 7.4.75 | 3 | 0012,0013,0014 |
| 7.4.76 | 3 | 0006,0007,0008 |
| 7.4.77 | 3 | 0004,0005,0017 |
| 7.4.78 | 1 | 0026 |
| 8.2.26 | 1 | 0019 |
| 8.2.38 | 1 | 0011 |
| 8.2.66 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
| 8.3.110 | 1 | 0018 |
| 8.4.37 | 8 | 0006,0007,0008,0010,0011,0012,0013,0014 |
| 8.4.54 | 12 | 0001,0002,0003,0006,0008,0009,0011,0015,0016,0019,0023,0024 |
| 8.4.68 | 26 | 0001,0002,0003,0004,0005,0006,0007,0008,0009,0010,0011,0012,0013,0014,0015,0016,0017,0018,0019,0020,0021,0022,0023,0024,0025,0026 |
