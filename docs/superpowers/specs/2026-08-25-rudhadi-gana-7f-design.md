# rudhādi (gaṇa 7), slice 7f — √chid and √chṛd, and the two sūtras they were deferred behind

Slice 7e curated √tṛh and left rudhādi at twenty-two of its twenty-five roots,
with the remaining three costed:

> **√chid and √chṛd (`07.0003`, `07.0008`)**, and with them **6.1.73 *che ca***
> (the tuk augment before a `C` after a short vowel) and **8.4.40 *stoḥ ścunā
> ścuḥ***. Without them their laṅ cells surface `aCinat` where vidyut has
> `acCinat`.

That deferral is correct, and — unlike 7d's, which undercounted √tṛh's cost by
three widenings — it is also complete. A probe of all 144 cells of `07.0003` and
`07.0008` against vidyut-prakriya at the audited commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, with full sūtra traces diffed
against this engine's implemented set, turns up **exactly those two sūtras and
nothing else**. No rule the engine already has is too narrow to carry either
root.

The reason is structural rather than lucky. Each root is shape-identical to one
already curated: √chid is √bhid with a `C` for its `B` (`Ci` + `nad`), and
√chṛd is √tṛd with a `C` for its `t` (`Cf` + `Rad`, ṇatva included). Every cell
outside laṅ derives on rules already in the pipeline; the two new sūtras fire
only where 6.4.71's aṭ-augment meets the root's initial `C`.

Both roots are ubhayapadī by 1.3.72 *svaritañitaḥ kartrabhiprāye kriyāphale*.
The sūtra reads *svarita or ñit*, and both upadeśas carry the first of those:
`Ci\di~^r` and `u~Cfdi~^r` each end in `i~^r`, where the `^` sits after the `~`
and so marks a **svarita it**, not a root accent. (Each also carries a `\`
elsewhere or not at all — `Ci\di~^r`'s is on the root vowel and says nothing
about pada; `u~Cfdi~^r` has none. `pada_from_upadesha`'s doc comment is the
place that distinction is already argued.) Each root therefore contributes 72
cells rather than 36.

## Scope

**New rules (2):** 6.1.73 *che ca* in `anga.rs`; 8.4.40 *stoḥ ścunā ścuḥ* in
`tripadi.rs`.

**Widened rules (0).** This is the claim that distinguishes 7f from 7e, and the
inertness gate below is what proves it.

**New sound-table entries (3):** `is_hrasva`, `is_shcu` and `shcutva_of` in
`sound.rs`.

**Moved (3 helpers):** `word_chars`, `set_char` and `remove_char` from
`tripadi.rs` to `terms.rs`, joined by a new `insert_char`. See "6.1.73's guard
is whole-word" below for why this slice pays for the move.

**New data:** two `Dhatu` rows; 16 `PARADIGM` blocks (2 roots × 2 padas × 4
lakāras); 58 `ALTERNATES` rows; four trace pins.

**Changed:** `rudhadi_rows_are_the_twenty_two_curated_roots` renamed and
extended; the corpus totals in `panini-data`, `crates/panini/tests/paradigm.rs`
and `tools/audit/panini_full_audit.rs`; the documentation sites enumerated under
"The doc claims this slice falsifies".

## What the probe found

The two new sūtras, with the cells that witness them:

| sūtra | operation | witness |
|---|---|---|
| 6.1.73 *che ca* | tuk (`t`) after a short vowel before `C`, by 1.1.46 *ādyantau ṭakitau* (kit → after) | every laṅ cell: `acCinat`, `acCindan`, `acCfRat`, `acCindmahi` |
| 8.4.40 *stoḥ ścunā ścuḥ* | that same `t` (stu) → `c` (ścu) before the `C` | the same cells; nothing else in the corpus |

The whole of the difference, in one trace — vidyut's own steps for `acCinat`,
with this engine's existing rules unmarked and the two new ones marked:

```
6.4.71   aw + Cinad + t          aṭ-āgama
6.1.73   a  + tCinad + t         NEW — the tuk
6.1.68   a  + tCinad             (this engine reaches the same surface by 8.2.23; see below)
8.2.39   a  + tCinad
8.4.40   a  + cCinad             NEW — ścutva
8.4.56   a  + cCinat             vā'vasāne, optional: acCinad / acCinat
```

√chṛd differs by one already-implemented step: 8.4.1 *raṣābhyāṁ no ṇaḥ* fires
between 8.2.39 and 8.4.40, giving `acCfRat`. The `f` and the `n` are adjacent
inside `Cfnad`, so the tuk — which sits before the `f`, not between it and the
`n` — cannot intervene, and 8.4.2's aṭ-intervener question never arises.

## Corpus growth

| | before | after |
|---|---|---|
| roots | 64 | **66** |
| cells | 2628 | **2772** (308 root×pada×lakāra blocks × 9) |
| forms | 3057 | **3259** |
| `ALTERNATES` rows | 429 | **487** |
| cells holding exactly one form | 2324 | **2426** |
| cells holding more than one | 304 | **346** |
| rudhādi curated | 22 of 25 | **24 of 25** |
| `TINANTA_RULES` | 90 | **92** |
| tripādī rules | 23 | **24** |
| vikalpa set | 7 | 7, unchanged |

Each root contributes 72 cells and 101 forms — 29 `ALTERNATES` rows apiece —
distributed 51 single-form cells, 18 two-form, 1 three-form, 1 five-form and 1
six-form. That is √bhid's and √tṛd's distribution exactly, and it puts both new
roots into the deepest fork the engine produces.

### √chid and √chṛd join the six-form record

`crates/panini/tests/paradigm.rs` and `docs/ARCHITECTURE.md` currently name
**six** roots whose loṭ parasmaipada eka cells stack three of the seven optional
rules (7.1.35, 8.4.65, 8.4.56) into five branches at prathama eka and six at
madhyama eka: √kṛt, √rudh, √bhid, √kṣud, √tṛd and √und. Both new roots are
dental-final rudhādi roots of exactly that shape, so the count becomes
**eight**, with `CintAt`/`CinttAt`/`CintAd`/`CinttAd`/`CinDi`/`CindDi` and the
√chṛd equivalents. This is a documented figure, not a derived one — the plan
must edit it rather than trust it.

### The pada-collision bucket grows

`paradigm.rs`'s last assertion pins the set of surfaces that occur in **both**
padas, currently twenty-two strings. √chid and √chṛd each contribute the same
pair every dental-final ubhayapadī rudhādi root does — `CinttAm`/`acCintta` and
`CfnttAm`/`acCfntta` — taking the vec to **twenty-six**. The comment above it,
which narrates the bucket slice by slice, needs a 7f sentence.

## Placement and ordering

### 6.1.73 goes in `anga.rs`, immediately after 6.4.72

`anga.rs`'s own header states its scope as "the augments and the rules that
reshape the *ending*"; tuk is an āgama, and 6.4.71 — which manufactures 6.1.73's
entire precondition by prefixing the aṭ — sits two entries above. Vidyut's trace
runs the same two rules back to back with nothing between them.

The competing home is `adesha.rs`, "Ādeśa and sandhi: 6.1.101 … 6.4.101", where
the other 6.1.x rules live. That stage's order is derivational rather than
numeric (6.1.101, 6.1.96, 6.1.90, 6.1.97, 6.1.87, 6.1.66), so sūtra number
decides nothing there either, and placing tuk four stages away from the rule it
consumes buys nothing. Either position is legal under the 3.1.68 rule that
governs stage membership; adjacency to 6.4.71 is what decides it.

Nothing between the two positions reads what the insertion changes. The tuk
lands **inside** `ANGA` (`aCi` → `atCi`), because 6.4.71 models the aṭ as a text
prefix on `ANGA` rather than as a separate term. `ANGA`'s first character stays
`a` and its penultimate stays `C`, so 6.4.72's `is_vowel(first)` guard, the guṇa
stage's upadhā reads, and `sound_before_ending` all see exactly what they saw
before.

### 8.4.40 goes in `tripadi.rs`, immediately above 8.4.41

Sūtra order, and unusually for this file it is *only* sūtra order — the two
rules cannot contend. 8.4.41's trigger is the ṣṭu class and `C` is not in it;
8.4.40's trigger is the ścu class and no ṣṭu sound is in that. Neither reads
what the other writes on any reachable input.

The rules below it are likewise inert on the new site:

- **8.4.55 *khari ca*** reads the SHAP/ENDING junction, not the tuk's position
  inside `ANGA`, and would decline anyway: its `sub == last` clause refuses
  vacuous fires, and both `t` and `c` are already their own car.
- **8.4.53 *jhalāṁ jaś jhaśi*** wants a jhaś after the jhal. `C` is voiceless,
  so it is not one.
- **8.4.1 / 8.4.2** operate on `Cfnad`'s adjacent `f`/`n`, as above.

### 8.4.65 must keep declining, and this is the interaction worth pinning

`c` and `C` are savarṇa jhars — same sthāna, same ābhyantara prayatna — so
8.4.65 *jharo jhari savarṇe* read bare would optionally elide the `c` and fork
`acCinat` to \*`aCinat`. It does not, and the reason is already in this engine:
8.4.65 carries 8.4.64's *halaḥ* by anuvṛtti, implemented as
`!is_vowel(w[i - 1])`, and the character before the `c` is the aṭ's own `a`.
Vidyut carries the same *halaḥ* (`HAL.contains(x)`) and produces the same two
forms.

So no code changes here. What 7f adds is the **witness**: no previously curated
root could put a savarṇa jhar pair immediately after a vowel, so the guard has
never been exercised in the direction that proves it necessary. A form-count
pin on `acCinat` is cheap and catches a future "simplification" of that guard
that no other cell would notice.

## 8.4.40 is implemented in one direction only

The sūtra is symmetric: a stu (`s` and the t-varga) and a ścu (`ś` and the
c-varga) in contact make the stu into its ścu correspondent, whichever comes
first. **This slice implements only stu-before-ścu** — the direction the tuk
witnesses — and the reason is 8.4.44 *śāt*.

A scan of all 2628 currently curated cells in vidyut, counting rule invocations
across every branch, gives the complete picture:

| rule | invocations on the existing corpus |
|---|---|
| 6.1.73 | 0 |
| 8.4.40 | 0 |
| 8.4.42, 8.4.43 | 0 |
| **8.4.44 *śāt*** | **118** — √aś `05.0020` (36), √kliś `09.0058` (41), √aś `09.0059` (41) |

Every one of those 118 is the converse direction, and every one is the same
shape: an `S` immediately before an `n`, in `aSnoti` and `kliSnAti`. 8.4.44
*śāt* exists precisely to stop ścutva there — without it `kliSnAti` becomes
\*`kliSYAti`.

So the converse arm's only reachable sites in this corpus are exactly the ones
śāt forbids, and implementing it has two outcomes and no third: ship it without
śāt and three curated roots break; ship it with śāt and the arm is code that
cannot fire, which `cargo mutants` reports as a survivor when deleting it
changes nothing. Neither is worth having. The arm is therefore **deliberately
absent**, documented in place at 8.4.40 with the measurement above — the same
treatment 6.3.111 already has at 8.3.13, and the same treatment 8.4.41's
correspondence side has for `d`/`n`/`s`.

Widen it, with 8.4.44 alongside, the moment a curated root puts a stu after a
ścu in a position śāt does not cover.

## The sound table

Three additions, in `sound.rs`, all following the `kutva_of` / `jashtva_of`
shape the 8.2.30/8.2.39 slice established — the substitute **is** the map, never
a case split:

| name | contents | read by |
|---|---|---|
| `is_hrasva` | `a i u f x` | 6.1.73's condition |
| `is_shcu` | `S c C j J Y` | 8.4.40's trigger |
| `shcutva_of` | `s→S t→c T→C d→j D→J n→Y` | 8.4.40's substitute |

`shcutva_of` carries all six arms even though only `t → c` has a witness here.
That is deliberate and is not the "widen without evidence" mistake the 8.2.30
slice warns against: a correspondence table is a fact about the sound system,
not a claim about reachability, and the alternative — a `t`-only map — is
exactly the hardcoded pair that made 8.2.30 wrong for √ric and √vic. The five
unwitnessed arms are covered by a direct `shcutva_of_stu_all_arms` unit test in
the style of `vrddhi_of_ac_vowels_all_arms` and
`is_savarna_stop_series_all_arms`, so they are caught by a test rather than left
to survive the mutation gate.

## 6.1.73's guard is whole-word, and that costs a helper move

6.1.73's condition is a *saṃhitā* condition — a short vowel and a `C` in
contact, anywhere — and the guard states it that way: scan the assembled word,
find the first short vowel immediately followed by `C`, insert `t` after the
vowel.

The only site any curated root can reach is inside `ANGA`, so an `ANGA`-local
guard would also work and would need no refactor. It would need a `NARROW GUARD`
comment arguing that a `C` can only ever be root-initial and that no vikaraṇa or
ending is C-initial — true today, and exactly the shape of argument that has
cost this repo twice already (8.2.39's `t`/`z`/`D` literal guard, 8.4.41's
`z`-only trigger, both of which the file's own comments had flagged as standing
hazards before they broke). Six such sites remain in `tripadi.rs` and `anga.rs`;
this slice declines to add a seventh for a rule whose real condition is one line
of scan.

Writing it that way needs `word_chars`, which is private to `tripadi.rs` today.
The move: `word_chars`, `set_char` and `remove_char` go to `terms.rs`, the
term-layout support layer, joined by a new `insert_char` beside them.
`tripadi.rs`'s twenty-odd call sites change by an import line only. `terms.rs`
is the right home under AGENTS.md's own division — `terms` and `sound` are
support layers holding no `Rule`, and these three are term-layout addressing,
not sound predicates.

## Two claims this slice must state rather than assume

### 6.1.68 is a standing divergence, and √chid inherits it

Vidyut's trace above credits 6.1.68 *hal ṅyāb bhyo dīrghāt su-ti-sy-apṛktaṁ hal*
for deleting laṅ's apṛkta `t`. This engine has no 6.1.68 and reaches the same
surface through 8.2.23 *saṁyogāntasya lopaḥ*. That divergence is already
recorded in AGENTS.md as predating √tṛh and holding across every curated rudhādi
root; √chid and √chṛd inherit it unchanged, and it is audited clean either way
because the audit compares derivation sets, not traces. **7f introduces
nothing new here and must not re-litigate it.**

### The inertness gate is the slice's central claim

7f asserts that two new rules can be added without disturbing a single one of
the 2628 existing cells. Two independent arguments say so — no curated root's
`code` contains a `C` at all, and the corpus contains no stu-before-ścu site
anywhere — but neither is the evidence. The evidence is a **byte-for-byte dump
diff against `main` over all 2628 pre-slice cells, taken after both rules land
and before either root is curated**. This is 7e's Task 2 pattern, and it is a
hard gate: a non-zero delta means the analysis above is wrong and the slice
stops there.

## The trace pins

Four, in `crates/panini/tests/trace.rs`:

1. **`acCinat` orders the new rules against the old** — 6.4.71 < 6.1.73 <
   8.4.40 < 8.4.56. This is the one pin that would catch 6.1.73 being moved to a
   stage below the tripādī, or 8.4.40 being moved above 8.2.39.
2. **`acCfRat` shows 8.4.1 and 8.4.40 co-firing**, pinning that ṇatva and
   ścutva stay disjoint on the one cell that reaches both.
3. **`Cinatti` cites neither new rule**, pinning that both are laṅ-only —
   the cheapest guard against 6.1.73's guard losing its short-vowel condition and
   firing word-initially.
4. **`acCinat` has exactly two forms**, pinning 8.4.65's decline under *halaḥ*
   as argued above. The `ALTERNATES` row count is the second alarm.

## Per-rule guard tests

Both new rules have constructible preconditions, so both get real guard tests
beside them rather than a cited derivation:

- **6.1.73**: fires on a hand-built short vowel + `C`; declines when the `C` is
  word-initial (no preceding sound); declines after a long vowel.
- **8.4.40**: fires on `t` + `C`; **declines on `S` + `n`** — the 8.4.44 *śāt*
  case, directly constructible, which is what makes the one-direction decision
  testable rather than merely documented.

Plus `shcutva_of_stu_all_arms` and an `is_shcu` / `is_hrasva` membership test in
`sound.rs`.

`tinanta_rule_order_is_pinned` gains `"6.1.73"` and `"8.4.40"` in position.
`exactly_the_pinned_vikalpa_rules_are_optional` is **unchanged** — neither new
rule is optional, and the seven-rule set is untouched.

## Verification

**Cross-implementation audit.** The committed harness,
`tools/audit/panini_full_audit.rs`, copied into the vidyut checkout per
`tools/audit/README.md` — never rewritten — at commit `8da2f90`, over 66 roots /
2772 cells / 3259 forms. Both negative controls (`entry` and `form`) verified
failing first; a zero-difference result means nothing until they do.

**Floor measurement.** The uncontended floor stood at 943.70s at 2628 cells
(paradigm 432.94s, roundtrip 508.54s, trace 2.22s). Cell growth here is +5.5%,
but cell count has failed as a multiplier for five consecutive slices, most
recently under-predicting by a factor of six (+8.2% floor for +1.4% cells).
**Measure the floor; do not scale it.**

**Mutation gate.** `mise run mutants`, which now runs `-j 4 --timeout 4800`
itself — run it through the task rather than reconstructing the flags, and note
that the mise shim has errored with "no version set for shim: cargo-mutants", in
which case invoke the `cargo-mutants` binary directly. Check `timeout.txt`
alongside `missed.txt`; a reported "0 missed" is vacuous if survivors were
reclassified as timeouts.

Expect, and verify rather than assume:

- **One timeout**, the known-permanent non-terminating-loop mutant on
  `tripadi.rs`'s 8.4.2 backward ṇatva scan (`j -= 1` → `j /= 1`). Identify it by
  that **shape**, not by line number — the number has drifted between slices
  without either slice touching the file. It is the correct verdict at any cap;
  do not chase it.
- **Two missed**, and they must be 7e's two verified equivalent mutants — the
  `adesha.rs` 6.1.87 im-arm `+`→`*`, and the `tripadi.rs` 8.3.13 guard `-`→`/`.
  A third missed mutant, or a different two, is a real finding.
- Mutant population rises from 547 by the two new rules, the four helpers and
  the three sound-table entries.

Against the 943.70s floor and the **1.02×–1.43×** `-j 4` contention range
measured across 7e's 508 test phases, 4800 keeps a margin of roughly 3× even if
the floor moves as sharply as it did last slice. **Do not re-derive the retired
2.1–2.5× figure as settled**, and do not quote the flattering 1.02× end alone.

## The doc claims this slice falsifies

Enumerated by file, because a grep for the totals will not find the ones that
are line-wrapped or rule-scoped:

| file | what changes |
|---|---|
| `AGENTS.md` | the rudhādi paragraph: √chid and √chṛd move out of the deferral list into the curated enumeration; "twenty-two" → twenty-four; "3 of the 25 remain out" → 1; the sentence naming 6.1.73 and 8.4.40 as sūtras the engine lacks must go; the golden-suite paragraph's 2628 / 429 / 3057; the audit paragraph's 2628 / 3057 / 64 |
| `docs/ARCHITECTURE.md` | the rudhādi paragraph (lines 83, 148): "twenty-two" → twenty-four; the branch-count paragraph's six roots → eight |
| `README.md` | line 18 "twenty-two"; line 37 "304 of the 2628 cells … 211" |
| `crates/panini/tests/paradigm.rs` | the 2628 / 292-blocks / 2324 / 429 assertions and their doc comments; the `ALTERNATES` key breakdown; "twenty-two rudhādi roots" (line 5368); the pada-collision vec (twenty-two → twenty-six strings) and its narrating comment |
| `tools/audit/panini_full_audit.rs` | the asserted totals (64 / 2628 / 3057) and the header arithmetic |
| `tools/audit/README.md` | the asserted-totals line, the recorded result, and the growth arithmetic |
| `crates/panini-data/src/lib.rs` | `rudhadi_rows_are_the_twenty_two_curated_roots` → `..._twenty_four_...`, with its comment block; `assert_eq!(dhatus().len(), 64)` (line 769); "the 64 roots curated here" (line 94); the stale verdict count at line 83 (see below); `pada_from_upadesha`'s accent census at line 1136, "42 of the 64 … and 29 of those" → **43 of 66 … and 30 of those** (recomputed off the vendored upadeśa; the method reproduces the existing 42/29 exactly, so the new pair is a recount and not a guess) |
| `crates/panini-prakriya/src/tinanta/tripadi.rs` | line 1031's "full 2628 cells" inside 8.3.13's comment |

Two comments carry pre-7c figures and are **left alone again**:
`controller.rs:130` and `tinanta/guna.rs:943`, both citing 1872/1864-of-1872.
They have been deliberately stale since 7c and this slice does not adopt them.

One drift **is** adopted, because it sits inside a doc comment this slice edits
anyway and contradicts its own next paragraph. `panini-data/src/lib.rs:83` says
`curated_pada_agrees_with_upadesha_markers` "re-derives every one of these **55**
verdicts" while line 94 of the same comment says the test "covers the **64**
roots curated here", and AGENTS.md says 64 as well. 55 was the figure at the
pada-audit slice and has not moved since. Read the test to establish which count
it actually enforces before editing either line — do not resolve the
contradiction by trusting AGENTS.md — then take both to 66 and note the
correction in the commit message.

## If the audit shows a difference

Stop and diagnose before touching golden data. The 8.2.30/8.2.39 slice is the
precedent: its first run found four differing cells and that was the *finding*,
not a setback — it exposed a narrow guard as a real defect. Transcribe
`PARADIGM` and `ALTERNATES` from audited output only, never from the probe
output in this document.

## Deliberately out of scope

- **√bhuj (`07.0017`)**, and with it 1.3.66 *bhujo'navane*. Now costed rather
  than merely deferred: vidyut derives all 72 cells (79 forms), and 1.3.66 is
  the **only** rule it invokes that this engine lacks — a root-keyed pada
  assignment structurally identical to the 1.3.72 rule already in `samjna.rs`,
  with no new phonology at all (√bhuj is √yuj with a `B`). What keeps it out is
  not cost but a ruling: 1.3.66 restricts ātmanepada to senses other than
  protecting, and neither engine models sense, so curating it means shipping an
  unconditional ubhayapada assignment with *anavane* recorded as unimplemented.
  That ruling deserves its own slice rather than riding along inside a
  phonological one. AGENTS.md's current framing — that √bhuj is out because its
  pada forks "on an axis this engine models" nothing of — overstates the
  obstacle and should be updated to say what it actually is.
- **8.4.44 *śāt* and the converse direction of 8.4.40**, argued above.
- **8.4.42 *na padāntāṭ ṭoranaḥ* and 8.4.43 *toḥ ṣi***, the corresponding
  exceptions on 8.4.41's side: zero invocations across the corpus, no witness.
- **8.4.41's correspondence side**, still covering only `t`/`T`/`D`. Unchanged
  by this slice and still unwitnessed for `d`/`n`/`s`.
- **6.3.111** and **6.1.68**, both deliberately absent, both already documented.
- **Splitting `crates/panini/tests/paradigm.rs`**, now 5,864 lines and headed
  for roughly 6,075. Worth doing and its own slice, for the reason 7e gave: a
  large mechanical diff sitting directly next to the data the audit exists to
  validate is the worst possible neighbour for it.
- **It-stripping as real sūtras** (1.3.2, 1.3.3, 1.3.5, 1.3.9).

## Ordering

1. The three sound-table entries and the helper move, with unit tests. No rule
   yet.
2. The two new rules — 6.1.73 in `anga.rs`, 8.4.40 in `tripadi.rs` — with their
   guard tests and their positions in `tinanta_rule_order_is_pinned`.
3. **Dump-diff against `main`: zero delta across 2628 cells. Gate.**
4. Two `Dhatu` rows; `rudhadi_rows_…` renamed and extended; `dhatus().len()`.
5. Corpus totals in `paradigm.rs` and `panini_full_audit.rs`.
6. Cross-implementation audit, negative controls first. Record the result.
7. `PARADIGM` and `ALTERNATES` transcribed from audited output; the
   pada-collision vec extended.
8. The four trace pins.
9. `mise run test` floor measurement, then the mutation campaign; both
   `missed.txt` and `timeout.txt` checked.
10. The eight-site documentation sweep.
