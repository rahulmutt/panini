# Juhotyādi gaṇa (gaṇa 3), slice 3f — √kit, √tur, √dhiṣ, √dhan

The prep spec (`2026-09-05-juhotyadi-gana-design.md`) tabled this slice as:

> | 3f | √bhas, √kit, √tur, √dhiṣ, √dhan, √jan | 216 | 6.4.98
> *gamahanajanakhanaghasāṁ lopaḥ*, 6.4.42 / 6.4.43 *ye vibhāṣā* (vikalpa),
> 6.4.100 *ghasibhasor hali ca*, 8.2.26 *jhalo jhali*; 8.3.24 before `h` for
> √dhan |

We re-probed all six rows at the audited commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` and ran this engine at `b976e05`
over the same cells on hand-built rows (parasmaipada, stored text `Bas` / `kit`
/ `tur` / `Diz` / `Dan` / `jan`), diffing derivation sets cell by cell. HEAD
matches **160 of 216** cells; vidyut holds 264 forms. The table undercounted
the machinery: beside its five new rules, the probe needs five widenings —
8.2.73, 8.2.74 and 8.2.75 (all three guarded to rudhādi), 8.4.55 (which reads
only the aṅga/ending junction, so misses *bapsati*'s word-internal `B` → `p`)
and 8.4.40 (which fires only when the palatal *follows*, so misses *jajYati*'s
`jn`).

**This spec splits the table's 3f row.** The difference concentrates in two
roots:

| row | root | HEAD | cause |
|---|---|---|---|
| 03.0019 | √bhas | 7/36 | 6.4.100, 8.2.26 (new); 8.2.73, 8.2.74, 8.4.55 (widened) |
| 03.0021 | √kit | 35/36 | 8.2.75 rudhādi-only — laṅ madhyama eka lacks *acikeH* |
| 03.0022 | √tur | 36/36 | — |
| 03.0023 | √dhiṣ | 36/36 | — |
| 03.0024 | √dhan | 34/36 | 8.3.24 rudhādi-only — *daDaMsi*, *daDaMhi* |
| 03.0025 | √jan | 12/36 | 6.4.98, 6.4.42, 6.4.43 vikalpa (new); 8.4.40, 8.3.24 (widened) |

**3f takes √kit, √tur, √dhiṣ and √dhan: two widenings, no new rule.** √bhas
and √jan, with all five new rules and the other four widenings, become slice
**3f2**, which closes the gaṇa.

## Scope

**Rows (4):** `03.0021 kita~` (√kita), `03.0022 tura~` (√tura), `03.0023 Diza~`
(√dhiṣa), `03.0024 Dana~` (√dhana), all parasmaipadī by 1.3.78 (no ṅit or
svarita marker). **144 cells, 169 forms** (vidyut: 44 + 41 + 43 + 41). Suite
4428 → 4572 cells, 5486 → 5655 forms; juhotyādi 20 → 24 of its 26 rows.

**New rules: none. Widened rules (2):** 8.2.75 *daś ca* and 8.3.24 *naś
cāpadāntasya jhali*, both in `tripadi.rs`.

**No new vikalpa** — the engine stays at eleven optional rules. The 25 extra
forms are the standing 7.1.35 / 8.4.56 forks plus 8.2.75's existing fork.

**Unchanged, confirmed on HEAD's shape by the probe:** 1.3.78, 6.1.10, 7.4.60,
7.4.62 (√kit's `ki` → `ci`), 7.3.86, 7.3.87 (*cikitAni*, *tuturARi*,
*diDizARi* — 3e's inheritance, as its spec predicted), 6.4.71, 6.4.101
(*cikidDi*), 8.2.77 (*tutUrhi*), and in `tripadi.rs` 8.2.23, 8.2.39, 8.2.41,
8.3.15, 8.3.59, 8.4.41, 8.4.53, 8.4.54, 8.4.56 and 8.4.58.

**Out of scope:** √bhas and √jan (3f2); 6.4.98, 6.4.100, 6.4.42, 6.4.43,
8.2.26; widening 8.2.73, 8.2.74, 8.4.55 or 8.4.40; curādi.

## The widenings

### 8.2.75 *daś ca* — the gaṇa test is dropped

Before sip, a dhātu-final `d` optionally becomes ru, which 8.3.15 finishes to a
visarga. √kit laṅ madhyama eka: a + ci + ket + s → 6.1.68 / 8.2.23 → *aciket*
→ 8.2.39 → *aciked* → 8.2.75 → *acikeH*, beside the unforked *aciked* /
*aciket*. vidyut's trace is exactly that.

- **The guard becomes the sūtra's own conditions:** `ctx.is_sip()`,
  `dhatu_is_pada_final(p)`, and the word ends in `d`. The
  `Tag::Rudhadi` test goes.
- **Evidence the gaṇa test was an engine artefact, not a scope:** with it
  removed, the whole suite passed unchanged on `b976e05` (throwaway worktree,
  this spec's probe). No curated non-rudhādi cell reaches the rule. The
  corpus-wide test below makes that permanent.
- **Order is unchanged** (8.2.74 → 8.2.75 → 8.2.73, against sūtra order, for
  the reason the existing comment gives). √kit presents `aciked` here,
  already voiced by 8.2.39, exactly as √kṛt presents `akfRad`.
- **8.2.73 does not need to move with it.** √kit laṅ prathama eka ends in `t`,
  not `s`; 8.2.73's `s`-final check declines on its own. 8.2.73 and 8.2.74 stay
  rudhādi-only until 3f2's √bhas.
- **Comment:** the rule's comment is rewritten to drop the rudhādi framing and
  to name √kit as the second witness, and the guard's recorded hazard (8.2.73
  manufacturing a `d`) is restated for a gaṇa-free rule: it stays unreachable
  because 8.2.73 still runs later and is still rudhādi-only.

### 8.3.24 *naś cāpadāntasya jhali* — widened to juhotyādi

A non-pada-final `n` becomes anusvāra before a jhal. √dhan: *daDaMsi* (before
`s`), *daDaMhi* (before `h`, loṭ madhyama eka). This answers the prep spec's
flag ("`h` is one, so the existing arm should reach it — verify, don't
assume"): `is_jhal('h')` holds, and the rule reaches *daDaMhi* once the gaṇa
test admits juhotyādi.

- **The guard becomes `Tag::Rudhadi || Tag::Juhotyadi`.** It is not dropped.
  The gaṇa test stands in for *apadāntasya*, keeping the rule off 7.1.3
  *jho'ntaḥ*'s `n` (the existing comment). With the test removed, four trace
  pins failed on `b976e05` — *bhavanti*, *yanti*, *āpnuvanti*, *hinvanti* —
  each gaining an 8.3.24 → 8.4.58 pair.
- **Why juhotyādi is safe under it:** juhotyādi never reaches 7.1.3. Its
  aṅga is abhyasta, so 7.1.4 *ad abhyastāt* takes the `J` (*daDati*), and laṅ
  takes *jus* by 3.4.109. No `n` in the gaṇa is 7.1.3's.
- **Prior juhotyādi rows are inert:** none of the twenty curated rows carries
  an `n` before a jhal. The corpus-wide test below holds this.
- **New cells it reaches beyond the two form changes:** wherever √dhan's `n`
  meets `t` / `T` (*daDantaH*, *daDanTa*, *adaDantAm*, *daDantu*, …). 8.4.58
  turns the anusvāra straight back into `n` — it is gaṇa-free and keyed on the
  following yay — so forms are unchanged and the trace gains the 8.3.24 → 8.4.58
  pair. That is vidyut's trace too.
- **Comment:** "In this suite that `n` is always śnam's" becomes "śnam's, or
  in juhotyādi the root's own (√dhan)". The NARROW GUARD note names both gaṇas
  and records why juhotyādi cannot reach 7.1.3.

## What vidyut credits that this slice does not write

- **7.4.59** on √kit, √tur and √dhiṣ — this engine copies before guṇa (3e's
  recorded divergence).
- **6.1.68** on laṅ prathama / madhyama eka — this engine credits 8.2.23.
- **8.2.66**, **8.4.68** — the standing conventions.

## Corpus

`panini-data` gains the four rows after `03.0014`:

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0021 | `kita~` | kit | parasmai | 1.3.78 |
| 03.0022 | `tura~` | tur | parasmai | 1.3.78 |
| 03.0023 | `Diza~` | Diz | parasmai | 1.3.78 |
| 03.0024 | `Dana~` | Dan | parasmai | 1.3.78 |

Each comment names its path: √kit 7.4.62 and 8.2.75; √tur 8.2.77; √dhiṣ 8.4.41;
√dhan 8.3.24. The juhotyādi row-list assertion gains all four, and its comment
moves to twenty-four of twenty-six with "slice 3f2 closes it".
`curated_pada_agrees_with_upadesha_markers` and
`dhatupatha_numbers_resolve_upstream` must pass without edits.

## Tests

- **Goldens** (`crates/panini/tests/paradigm/data/juhotyadi.rs`): the 144
  cells, transcribed from vidyut. **ALTERNATES gains 25 rows** over 16 cells:

  | root | forked cells |
  |---|---|
  | √kit (8) | laṅ prathama eka (2), laṅ madhyama eka (3, with 8.2.75), loṭ prathama eka (3), loṭ madhyama eka (3), vidhiliṅ prathama eka (2) |
  | √tur (5) | loṭ prathama eka (3), loṭ madhyama eka (3), vidhiliṅ prathama eka (2) |
  | √dhiṣ (7) | laṅ prathama eka (2), laṅ madhyama eka (2), loṭ prathama eka (3), loṭ madhyama eka (3), vidhiliṅ prathama eka (2) |
  | √dhan (5) | loṭ prathama eka (3), loṭ madhyama eka (3), vidhiliṅ prathama eka (2) |

- **Trace pins** (`crates/panini/tests/trace/juhotyadi.rs`):
  - *acikeH* (laṅ madhyama eka): 8.2.39 → 8.2.75 → 8.3.15;
  - *aciket* (laṅ prathama eka): no 8.2.75 and no 8.2.73 step;
  - *daDaMsi* (laṭ madhyama eka): 8.3.24, no 8.4.58;
  - *daDaMhi* (loṭ madhyama eka): 8.3.24 before `h`;
  - *daDantaH* (laṭ prathama dvi): 8.3.24 → 8.4.58.
  The test at `trace/juhotyadi.rs:903` ("3f extends the allowed list") gains
  this slice's rows and its comment retargets to 3f2.
- **Guard unit tests** in `derivation_tests.rs`, slice-7 style:
  - 8.2.75 fires on a juhotyādi `d`-final sip stem and declines on tip, on a
    non-`d` final, and when the dhātu is not pada-final;
  - 8.3.24 fires on a juhotyādi `n` + jhal, and declines on a bhvādi `n` + `t`
    (the 7.1.3 shape — *bhavanti*), and on an `n` before a non-jhal (`m`, `y`).
- **Corpus-wide fires-only-on-rows test** for each widened rule: over every
  curated cell, 8.2.75 is credited only on rudhādi rows and `03.0021`, and
  8.3.24 only on rudhādi rows and `03.0024`. Goldens ignore traces, so this is
  the test that pins "no prior cell changed".
- **Prior traces:** diff every prior cell's trace between `main` and the slice
  HEAD; the 4428 prior cells stay byte-identical, traces included.
- **Spot check** in `paradigm/main.rs` beside 3e's: *acikeH*, *daDaMhi*,
  *tutorti*.

## Audit

Copy the committed `tools/audit/panini_full_audit.rs`, never rewrite it. Bump
its invariants to **101 roots, 4572 cells (508 blocks × 9), 5655 forms (4572 +
1083 ALTERNATES rows)**, with `derivation_set_shape_matches_the_audited_numbers`
raised to match. Repoint `/tmp/vidyut-full`'s dev-deps at the slice worktree
before running. Zero divergence against `8da2f90b` is required, and the
mis-resolved-root negative control must be shown failing first.

## Mutation gate

Re-measure the uncaught-suite floor at the campaign's parallelism and set the
cap at 6× it, rounded up to the next 10 s. Copy `outcomes.json` durably before
any further invocation. Expected new non-caught mutants: none — the `||` in
8.3.24's guard is killed by the bhavanti decline test on one side and the
√dhan goldens on the other; 8.2.75's guard by its unit tests and the *acikeH*
golden. AGENTS.md names the non-caught set verbatim.

## Doc sweep

README, AGENTS.md and `docs/ARCHITECTURE.md`: 97 → 101 roots, twenty → twenty-four of
twenty-six juhotyādi rows, 4428 → 4572 cells, 5486 → 5655 forms, ALTERNATES
1058 → 1083, multi-form cells 776 → 792 (two-form 587 → 594, three-form
143 → 152, four-form unchanged). The prep spec's "Later slices" notes gain a 3f
line recording the split.

The grep covers wrapped counts, spelled-out numbers, rule-scoped counts, the
root-shape literals (`kit`, `tur`, `Diz`, `Dan`, `ci`, `Da`), the two widened
rules' "rudhādi only" phrasings anywhere in comments, and every "3f" promise
in code and docs. Known at `b976e05`: `guna.rs:135`, `guna.rs:1978`,
`abhyasa.rs:744`, `anga.rs:32`, `trace/juhotyadi.rs:903`,
`panini-data/src/lib.rs:1495`, `paradigm/main.rs:594` and `:605`. Each is
rewritten as done or retargeted to 3f2 where it is √bhas's or √jan's. Recorded
file:line anchors are re-grepped at final HEAD.

## Success criteria

- 4572 cells VALID with the trace pins; the 4428 priors byte-identical, traces
  included.
- Audit zero-divergence at 101 / 4572 / 5655 against `8da2f90b`, with the
  negative control verified failing.
- `mise run test` passes.
- Gate clean at a re-measured cap; the non-caught set named verbatim in
  AGENTS.md.
- Juhotyādi at twenty-four of its twenty-six rows; no "3f" promise left that is
  not 3f2's.

## Later slices

- **3f2** (√bhas, √jan) closes the gaṇa. From this spec's probe: new 6.4.100
  and 8.2.26 for √bhas (*bapsati*, *babDaH*, *babDi*), with 8.2.73 / 8.2.74
  widened (*abaBat*, *abaBaH*) and 8.4.55 reading word-internally (`Bs` →
  `ps`); new 6.4.98, 6.4.42 and the vikalpa 6.4.43 for √jan (*jajYati*,
  *jajAtaH*, *jajAyAt* ~ *jajanyAt*), with 8.4.40 widened to a palatal on the
  left (`jn` → `jY`). vidyut runs 6.4.42 before 6.1.10; check that this engine's
  post-dvitva placement reaches the same forms. Re-probe before writing its spec.
