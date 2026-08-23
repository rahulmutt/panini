# rudhādi (gaṇa 7), slice 7e — √tṛh, and the three sūtras it was deferred behind

Slice 7d curated eight of the nine "reachable but uncurated" rudhādi roots and
left the ninth standing with its cost named:

> **√tṛh** is the ninth and the only one still out, deferred to slice 7e behind
> three sūtras the engine does not implement: 7.3.92 *tṛṇaha im* (the *im*
> augment), 8.2.31 *ho ḍhaḥ* and 8.3.13 *ḍho ḍhe lopaḥ*.

That deferral is correct as far as it goes, and it is incomplete. A probe of all
36 of `07.0018`'s cells against vidyut-prakriya at the audited commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, with full sūtra traces diffed
against this engine's implemented set, confirms the three new sūtras exactly —
and turns up **three rules the engine already has that are too narrow to carry
√tṛh**. The slice is six changes, not three.

√tṛh is parasmaipada only (vidyut derives no ātmanepada forms for `tfha~`), so
it contributes 36 cells rather than 72.

## Scope

**New rules (3):** 7.3.92 *tṛṇaha im* in `guna.rs`; 8.2.31 *ho ḍhaḥ* and 8.3.13
*ḍho ḍhe lopaḥ* in `tripadi.rs`.

**Widened rules (3):** 8.4.41 *ṣṭunā ṣṭuḥ*'s `z`-only trigger → the full ṭ-varga
(`w W q Q R`) alongside `z`;
8.2.41 *ṣaḍhoḥ kaḥ si*'s missing ḍh arm; 6.1.87 *ād guṇaḥ* gains a
SHAP-internal arm.

**New data:** one `Dhatu` row; 4 `PARADIGM` blocks (1 pada × 4 lakāras); 7
`ALTERNATES` rows; three trace pins.

**Changed:** `rudhadi_rows_are_the_twenty_one_curated_roots` renamed and
extended; the corpus totals in `panini-data`, `crates/panini/tests/paradigm.rs`
and `tools/audit/panini_full_audit.rs`; the documentation sites enumerated
under "The doc claims this slice falsifies".

No sound-table changes: `is_jhal`, `is_jhash` and `jashtva_of` already carry
`Q`, and `is_jhal` already carries `h`.

## What the probe found

The three new sūtras, with the cells that witness them:

| sūtra | operation | witness |
|---|---|---|
| 7.3.92 *tṛṇaha im* | `tfnah` → `tfnaih`, the *im* āgama after the last vowel by 1.1.47 | `tfReQi`, `tfRekzi`, `tfRehmi`, `atfReq`/`atfRew`, `tfReQu` |
| 8.2.31 *ho ḍhaḥ* | `h` → `Q` before jhal or at pada-end | `tfReQi`, `tfRQaH`, `atfReq` |
| 8.3.13 *ḍho ḍhe lopaḥ* | ḍh elided before ḍh | `tfReQi`, `tfRQaH`, `tfRQi` |

The three widenings, each of which the engine reaches only now:

| rule | as implemented | why √tṛh needs more |
|---|---|---|
| 8.4.41 *ṣṭunā ṣṭuḥ* | trigger hardcoded to `z`; substitute map already covers `t T D` | 8.2.31 produces a ṭ-varga `Q`, which must retroflex 8.2.40's `D` |
| 8.2.41 *ṣaḍhoḥ kaḥ si* | reads `z` only | the sūtra's own name is *ṣa-ḍhoḥ*, ṣ **and** ḍh: `tfneQ`+`si` → `tfnek`+`si` |
| 6.1.87 *ād guṇaḥ* | thematic SHAP + ending-initial `i`/`I` | √tṛh's coalescence is SHAP-internal: `naih` → `neh` |

8.4.41's widening was predicted by its own comment: *"NARROW GUARD, by design …
no curated root reaches a ṭ-varga-stop trigger yet. Widen both the moment a root
or a junction reaches the wider cases."* 7e is that moment.

Everything else √tṛh invokes — 8.2.40, 8.2.23, 8.3.15, 8.3.24, 8.3.59, 8.4.1,
8.4.56, 8.4.58, 6.4.101, 6.4.111, 7.1.35 — fires through arms already in the
pipeline.

## Placement and ordering

**7.3.92 → `guna.rs`**, next to 7.3.84 and 7.3.86. The pipeline is
`samjna → tin → vikarana → anga → guna → adesha → tripadi`, so it lands before
6.1.87 (`adesha.rs:349`) with no reordering. It is mutually exclusive with
6.4.111 (`adesha.rs:595`) by guard — 7.3.92 fires on pit, 6.4.111 on ṅit — so
their relative order never matters.

**8.2.31 → `tripadi.rs`** between 8.2.30 (`:308`) and 8.2.39 (`:378`). Natural
sūtra order, no tension.

**8.3.13 → `tripadi.rs` immediately after 8.4.41** (`:901`), *not* in numeric
order. The derivation forces it:

```
tfneQ | ti     8.2.31  h → Q
tfneQ | Di     8.2.40  jhaṣ Q makes ti's t → D
tfneQ | Qi     8.4.41  ṣṭutva retroflexes that D → Q     ← creates 8.3.13's condition
tfne  | Qi     8.3.13  ḍh before ḍh elided
```

The second ḍh does not exist until ṣṭutva has run. Placed in numeric order,
8.3.13 sees `tfneQ|Di`, declines, and the cell surfaces `*tfReQQi`. The file
already orders by operation where the derivation demands it — 8.2.73 sits after
8.2.75, and 8.4.56 sits last, after 8.4.65 — so this is the established idiom,
and it gets a comment saying so.

**Immediately** after 8.4.41 is load-bearing, not merely "somewhere after": see
"√tṛh does not join the six-form record" below.

Checked against every cell the placement touches:

- `tfRQaH` — 8.3.24 `n→M`, 8.4.41 `D→Q`, **8.3.13**, then 8.4.58 (`:1203`) `M→R`
- `tfRQi` — via 6.4.101 `hi→Di`, then the same tail
- `tfRQAt`/`tfRQAd` — 8.4.58 runs before 8.4.56 here where vidyut runs them the
  other way; they touch different characters, so both orders converge
- `tfRekzi` — takes the 8.2.41 branch and never reaches 8.3.13; at 8.4.41 time
  the word is `tfnek|zi`, whose `z` is followed by `i`, so the widened trigger
  finds nothing

Because 8.4.1 (`:1102`) runs **after** 8.4.41, no `R` exists in the word when
8.4.41 scans. The `R` arm of the widened trigger is therefore unreachable in the
current pipeline; it is included anyway, with a unit test, exactly as
`kutva_of` carries its witness-less `C`/`J` arms.

## The 7.3.92 guard

Four conjuncts, and **√tṛh's own 36 cells contain a negative control for every
one of them**. Drop any conjunct and a golden cell breaks.

| conjunct | fires on | negative control |
|---|---|---|
| the aṅga is `tfnah` | √tṛh under śnam | every other curated rudhādi root |
| following affix is hal-ādi | `ti si mi t tu` | `am` → `atfRaham`; loṭ uttama `Ani/Ava/Ama` → `tfRahAni` |
| pit sārvadhātuka | tip/sip/mip and their laṅ/loṭ substitutes | `tas Ta vas`, apit → ṅit by 1.2.4 → `tfRQaH`, `tfRQa`, `tfMhvaH` |
| not ṅit | — | tātaṅ (7.1.35) → `tfRQAt`/`tfRQAd`; yāsuṭ (3.4.103) → `tfMhyAt` |

The fourth is not redundant with the third: under yāsuṭ the ending `t` is still
pit, and it is the *āgama* that is ṅit. vidyut's source marks this a `HACK`
guarding against `*tfRihyAt`; here it is not a hack — `Tag::Pit` and `Tag::Nit`
are already maintained on terms, 1.2.4 sets Nit in `samjna.rs`, and 7.1.35
explicitly clears Pit before adding Nit (`tin.rs:509–514`).

**Root identification** follows the 7.1.6 *śīṅo ruṭ* idiom
(`p.terms[ANGA].text.ends_with("SI")`), but reads the **combined** stem
`ANGA + SHAP` rather than the two slots separately: the split is an
implementation artifact of the infix representation, whereas `tfnah` is what the
sūtra names. Nothing else in the corpus produces `tfnah` — √tṛd gives `tfnad`.

**The operation is local to SHAP in both steps**, which is the point of doing it
this way:

```
[tf, nah, ti]  →  7.3.92  →  [tf, naih, ti]   insert i before SHAP's last char
               →  6.1.87  →  [tf, neh,  ti]   SHAP-internal a + i → e
```

The rudhādi stem is split across ANGA/SHAP (`tfh` → `[tf, nah, ti]`), so the last
vowel of the combined aṅga is śnam's own `a`, which lives in SHAP. Both steps
are therefore term-internal — no cross-term surgery.

### Why 6.1.87 gets a second arm rather than a generalisation

Two alternatives were considered and rejected.

**Generalising 6.1.87 to a word-level scan** (the `word_chars` idiom the tripādī
uses), so it coalesces `a` + `i`/`I` wherever they sit, is more faithful to *ād
guṇaḥ*'s generality and is structurally the same fix the 8.2.30 episode applied.
It is rejected because it rewrites the one rule every thematic root in the corpus
passes through, for one new root. The two arms are also genuinely different
operations: the existing one is a junction coalescence that *also* consumes the
ending's initial vowel; the new one is term-internal and consumes nothing. A
two-arm rule states that; one scan would have to special-case it.

**Having 7.3.92 emit `neh` directly** and record a synthetic 6.1.87 step is
rejected outright: the trace would assert a step that never ran, and trace
fidelity is what this repo pins.

The new arm is gated on **7.3.92 having fired in this derivation**, read from
`p.log` — the idiom 6.4.72 and 7.1.6 already use — rather than on sniffing SHAP
for an `ai`. That makes it structurally unable to misfire for a root that does
not take the āgama.

## Corpus growth

Counted off the probe, not estimated:

| | before | after |
|---|---|---|
| roots | 63 | **64** |
| blocks (root × pada × lakāra) | 288 | **292** |
| cells | 2592 | **2628** |
| forms | 3014 | **3057** |
| `ALTERNATES` rows | 422 | **429** |

√tṛh's 36 cells hold 43 forms: 31 one-form cells, 3 two-form (laṅ prathama eka,
laṅ madhyama eka, vidhiliṅ prathama eka — all `8.4.56`) and 2 three-form (loṭ
prathama eka, loṭ madhyama eka — `7.1.35` and `7.1.35+8.4.56`). Key tallies go
`8.4.56` 102 → **105**, `7.1.35` 84 → **86**, `7.1.35+8.4.56` 84 → **86**; the
other seven keys are unchanged. The fork-depth census in `paradigm.rs`'s doc
comment goes 2293 → **2324** one-form, 208 → **211** two, 77 → **79** three,
with four/five/six unchanged at 2/6/6.

### √tṛh does not join the six-form record

√tṛh reaches loṭ madhyama eka in exactly the `kfnt|Di` shape that makes every
other stop-final rudhādi root a six-form cell: 8.4.53 voices, 8.4.65 *jharo
jhari savarṇe* optionally elides, and 7.1.35 × 8.4.56 multiply that by three.
√tṛh gets **three** forms instead, because 8.4.41 retroflexes the junction to
`Q|Qi` and 8.3.13 then elides *obligatorily* the very ḍh that 8.4.65 would have
forked on. The new sūtra pre-empts the optional one.

This holds only if 8.3.13 sits immediately after 8.4.41 — ahead of 8.4.53
(`:998`) and 8.4.65 (`:1271`). Placed after 8.4.65, the engine derives a
six-form cell and the audit flags it. The placement argued above on ṣṭutva
grounds is thus independently forced by the fork count: two unrelated reasons,
the same slot.

## Two claims this slice must state rather than assume

### 6.1.68 is a standing divergence, not a √tṛh finding

vidyut credits **6.1.68** *hal ṅyāb bhyo dīrghāt su-ti-sy-apṛktaṁ hal* with
deleting laṅ tip's apṛkta `t` from `atfnaih|t`. This engine has no 6.1.68 and
reaches the same surface by **8.2.23** *saṁyogāntasya lopaḥ*.

This is **pre-existing and already audited clean**, not something √tṛh
introduces: vidyut credits 6.1.68 for every curated rudhādi root's laṅ prathama
eka — `akfRat`, `aBinat`, `apinaw`, `aBanak` — and never 8.2.23. Verified during
design. It gets a sentence in 8.2.23's comment so nobody re-opens it as a 7e
finding; it is not this slice's to litigate.

### 6.3.111 is genuinely not needed

The obvious question about 8.3.13 is whether the elision lengthens what precedes
it, per **6.3.111** *ḍhralope pūrvasya dīrgho'ṇaḥ*. In every √tṛh cell the
preceding sound is `e` (`tfRe|Qi`) or `M` (`tfM|QaH`) — never a short aṇ — so
6.3.111 has no target, and vidyut's traces never emit it either. This goes in
8.3.13's comment **with the reason**, so the next root that does present a short
vowel there finds the note rather than the bug.

### One consistency point

7.3.92's placement of the āgama is **1.1.47** *mid aco'ntyāt paraḥ* — the same
paribhāṣā 3.1.78 already cites-but-does-not-implement for śnam, and the
treatment 1.4.13 and 1.1.5 get. 7.3.92's comment cites it the same way rather
than inventing a second convention. This is what 7d's "the engine's first āgama
of that kind" meant: 1.1.47 placement is not new, but placing an *āgama* by it
is.

## The trace pins

Three, following 7d's pattern of pinning what a reviewer would otherwise
re-derive.

1. **`tfReQi`** — the whole im path in one cell: 7.3.92 → 6.1.87 → 8.2.31 →
   8.2.40 → 8.4.41 → **8.3.13** → 8.4.1. Pins 8.3.13 *after* 8.4.41; reorder
   them into numeric order and this pin fails with `*tfReQQi`.
2. **`tfRQi`** (loṭ madhyama eka) — a *negative* pin: the trace must contain
   8.3.13 and must **not** contain 8.4.65. Makes the fork-depth claim
   mechanical. Move 8.3.13 below 8.4.65 and this cell silently becomes a
   six-former.
3. **`atfReq`/`atfRew`** — pins an ordering fact invisible in the surface:
   7.3.92 lives in the `guna` stage and 8.2.23 in `tripadi`, so when 7.3.92
   tests "is the following affix hal-ādi", laṅ tip's apṛkta `t` is **still
   there**. Let 8.2.23 run first and `ENDING` is empty, the hal-ādi test fails,
   and the cell derives `*atfRah`. The two rules are in different files and
   nothing else records the dependency.

## Verification

**The 8.4.41 widening is the slice's one real regression risk**, and it is
isolated rather than argued about. A widened trigger can change existing
behaviour two ways, and only one is obvious:

- a new pair matches where none did — the intended effect;
- an **earlier** ṭ-varga pair matches first and pre-empts the `z` pair that used
  to match — 8.4.41 `return`s after its first hit, and the controller runs each
  rule once per stage pass (`for rule in *stage`).

The rule's own comment argues the widening is a strict improvement: the `z`-only
trigger is "the ONE narrowing left standing" holding 8.4.41 and 8.4.53 apart,
and widening to include `q` is precisely what would make the pair
order-independent. That is an argument. The repo's standard is evidence.

1. TDD throughout.
2. **Dump-diff the existing corpus before any new row lands.**
   `PANINI_AUDIT_DUMP` writes the full cell table: take it on `main`, apply the
   three widenings alone, take it again, require a **zero delta across all 2592
   cells**. If the widenings perturb nothing on their own, a difference after
   √tṛh arrives has exactly one possible cause. This is the reasoning 7d used to
   keep curation and new sūtras in separate slices, applied inside one slice.
3. **Cross-implementation audit** against vidyut `8da2f90`: **64 roots / 2628
   cells / 3057 forms, zero differences**, with the `entry` negative control
   verified failing *first*. Copy `tools/audit/panini_full_audit.rs` — it is
   committed precisely so it is never rewritten.
4. **Mutation gate** via `mise run mutants`, which now carries
   `-j 4 --timeout 2400` itself; run the task rather than reconstructing the
   flags. **Re-measure the uncontended floor at 2628 cells rather than scaling
   it** — the floor has not tracked cell count. Growth here is +1.4%, the
   smallest in several slices, so a near-flat measurement is expected but not
   assumable. Read `timeout.txt` alongside `missed.txt`; expect exactly the one
   known-permanent `tripadi.rs` ṇatva-scan timeout (`j -= 1` → `j /= 1`).

## The doc claims this slice falsifies

Seven sites. A checklist, not a sweep: past slices have shipped with counts
stale in exactly one file.

| site | what changes |
|---|---|
| `README.md` | rudhādi twenty-one → twenty-two roots (`:18`); "4 of the 25 remain out" → 3 (`:26`); 2592 → 2628 cells and the fork census at `:36` — 299 → 304 multi-form cells, 208 → 211 two-form, 77 → 79 three-form, the four/five/six enumerations unchanged |
| `docs/ARCHITECTURE.md` | the rudhādi paragraph; √tṛh moves from the deferral list to the curated list; "4 of the 25" → 3 |
| `AGENTS.md` | the rudhādi paragraph; suite-size figures; the recorded audit result; the new floor and campaign numbers |
| `crates/panini-data/src/lib.rs` | `rudhadi_rows_are_the_twenty_one_curated_roots` → twenty-two; `dhatus().len()` at `:750`; the 63-root prose at `:94` and the backslash census at `:1102`/`:1298` |
| `crates/panini/tests/paradigm.rs` | the totals at `:5385` and `:5433`, and the fork-depth doc comment at `:5343–5380` |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | 8.4.41's NARROW GUARD comment, which this slice falsifies, and 8.2.41's guard comment |
| `tools/audit/README.md` and `panini_full_audit.rs` | "Last recorded result"; the asserts at `:577–579` and the header at `:12`, `:24`, `:27`, `:54` |

`data/ATTRIBUTION.md` gains a 7e entry in the form the 7a–7d entries take.
`07.0018 tfha~` stores `code` `tfh`, which is its it-stripped upadeśa, so no
per-entry deviation needs recording.

## If the audit shows a difference

The posture is fixed in advance so the slice does not become an open-ended
chase. The probe compared full traces cell by cell, which is stronger evidence
than 7d's sūtra-set comparison — but the audit is still the only thing that can
falsify it.

A difference in an **existing** cell means a widening perturbed something, and
step 2's dump-diff should already have caught it; the widening is reverted and
re-approached, not patched around. A difference in a **√tṛh** cell means one of
the three new rules or the 7.3.92 guard is wrong; the guard's four conjuncts
each have a named negative control above, so the failing cell identifies the
conjunct.

What the slice does **not** do is expand to implement whatever else the audit
turns up.

## Deliberately out of scope

- **√chid and √chṛd**, and with them 6.1.73 *che ca* and 8.4.40 *stoḥ ścunā
  ścuḥ*. Unchanged by this slice; after 7e they and √bhuj are all that remain of
  rudhādi's 25.
- **√bhuj (`07.0017`)**, whose 1.3.66 *bhujo'navane* forks its pada on sense
  rather than on an axis this engine models.
- **Splitting `crates/panini/tests/paradigm.rs`**, now past 5,780 lines. Worth
  doing and its own slice: a large mechanical diff inside a slice that adds
  three sūtras would sit directly next to the data the audit exists to validate.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9).
- **6.1.68**, and **6.3.111** — both discussed above, both deliberately absent.

## Ordering

1. The three widenings alone — 8.4.41, 8.2.41, 6.1.87 — with unit tests
   including 8.4.41's unreachable `R` arm.
2. Dump-diff against `main`: zero delta across 2592 cells. **Gate.**
3. The three new rules — 7.3.92, 8.2.31, 8.3.13 — with unit tests.
4. One `Dhatu` row; `rudhadi_rows_…` renamed and extended; `dhatus().len()`.
5. Corpus totals in `paradigm.rs` and `panini_full_audit.rs`.
6. Cross-implementation audit, negative control first. Record the result.
7. `PARADIGM` and `ALTERNATES` transcribed from audited output.
8. The three trace pins.
9. `mise run test` floor measurement, then `mise run mutants`; both
   `missed.txt` and `timeout.txt` checked.
10. The seven-site documentation sweep.
