# Curādi 10l: eight rule-bearing rows

10k's "Later slices" left nine plain obligatory-ṇic rows that need rules or
data beyond what the engine had. This slice takes eight of them. The ninth,
`10.0175 picca~`, needs 8.2.30 narrowed, and the rudhādi rows (*ric*, *vic*,
*bhañj*) depend on that rule. It goes to slice **10m** on its own, so the
change gets its own trace diff and review.

A throwaway prototype off 10k's HEAD (Evidence) re-derived what each row
needs. It corrects 10k's table in three places:

- `kFta~` takes **7.1.101** *upadhāyāś ca*, not 7.1.100 *ṝta id dhātoḥ*.
  The ṝ is the upadhā, not the final.
- `adwa~` also passes through the existing **8.4.55** *khari ca*.
- `u~Drasa~`'s optional ṇic needs `optional_nic_from_upadesha` to treat an
  initial `u~` as udit, not only a final one.

The slice adds four rules (7.1.101, 6.1.75, 8.2.18, 8.2.78), widens one
(8.4.41), and adds one `OPTIONAL_NIC` entry. After it, curādi is **open at
490 of its 509** rows, and 10m's `picca~` would bring it to 491.

## Scope

**In:** 8 rows, `PadaAssignment::Nic`, ubhayapadī by 1.3.74 *ṇicaś ca*,
across laṭ / laṅ / loṭ / vidhiliṅ. `10.0270`'s ṇic is optional (Kaumudī
2570), so its parasmaipada cells also derive the ṇic-less branch, and its
ṇic-less ātmanepada branch is blocked.

- **576 cells** (8 × 72), **666 forms**. Seven rows have 78 forms each:
  each parasmaipada block's four usual multi-form cells add 6. `10.0270`
  has 120. So **90 new `ALTERNATES` rows** (11576 → 11666).
- **Blocked branches:** 6516 → **6552**, `10.0270`'s 36 ṇic-less
  ātmanepada cells.
- Codes are `stored_form`'s output.

Each row, with what it needs and its laṭ parasmaipada prathama eka (vidyut's
form, matched by the prototype):

| row | code | needs | laṭ P 3sg |
|---|---|---|---|
| `10.0023 urja~` | `urj` | 8.2.78 | Urjayati |
| `10.0026 curRa~` | `curR` | 8.2.78 | cUrRayati |
| `10.0180 gurda~` | `gurd` | 8.2.78 | gUrdayati |
| `10.0155 kFta~` | `kFt` | 7.1.101, then 8.2.78 | kIrtayati |
| `10.0278 kfpa~` | `kfp` | 7.3.86, then 8.2.18 | kalpayati |
| `10.0170 mleCa~` | `mleC` | 6.1.75, then 8.4.40 | mlecCayati |
| `10.0037 adwa~` | `adw` | 8.4.41 widened, then 8.4.55 | awwayati |
| `10.0270 u~Drasa~` | `Dras` | `OPTIONAL_NIC` (2570); 7.2.116 on the ṇic branch | Drasati, DrAsayati |

`urj`'s laṅ cells take no 8.2.78. 6.1.90 merges the āṭ into the root's `u`,
leaving *aurj-* with no ik. vidyut agrees, so `urj` credits 8.2.78 on 59 of
its 78 branches.

**Out:**

- `10.0175 picca~` and the 8.2.30 narrowing: slice 10m.
- `10.0368 za\da~`, upasargas and the causative, as before.
- The gaṇasūtra rows `10.0493`–`10.0509`, which are not dhātus.
- An aṅga-stage 6.1.75 entry. No curated row reaches it: a laṅ aṭ is short.
  An unreachable entry would only add mutants that nothing catches.
- Widening the existing 8.4.41 arm (a tu *after* a ṣṭu) past t/T/D. Its own
  comment's evidence rule still holds, and no row reaches it.

### Decisions

**7.1.101 is a sanādi entry, before 7.3.86.**

- vidyut credits it right after ṇic's it-lopa and before 3.1.32 (`kFt+i` →
  `kirt+i`).
- At that point the `i` it makes is guru, since `rt` follows. So 7.3.86
  finds no laghu upadhā, and the root takes no guṇa (*kīrtayati*, not
  *kartayati*).
- Guarded on ṇic's ṇit at `NIC` and an `F` upadhā. It writes `ir` in one
  step, with 1.1.51 *uraṇ raparaḥ* uncredited. That is 7.1.102's convention
  here, and vidyut's.
- No guṇa-stage entry: no curated ṇic-less aṅga has an `F` upadhā.

**6.1.75 is a sanādi entry beside the sanādi 6.1.73, sharing its scan.**

- vidyut runs both sūtras in one scan and picks the id by the vowel's
  length. Here the tuk logic in `anga::che_ca` becomes a shared helper that
  takes the vowel test. `che_ca` (short vowel, 6.1.73) and a new `dirghat`
  (long vowel, 6.1.75) each call it, so the two cannot drift.
- *Dīrgha* is every vowel `is_hrasva` rejects (`A I U F X e E o O`), as
  vidyut's `is_dirgha`.
- It sits after the sanādi 6.1.73 and before 7.3.86, where vidyut credits it
  (`mleC+i` → `mletC+i`). 8.4.40 then makes *mlecch-*.

**8.2.18 is keyed by row, through `samjna::KRP`.**

- vidyut keys the sūtra on three upadeśas: `kfpU~\` (`01.0866`), `kfpa~\`
  (`01.0875`) and `kfpa~` (`10.0278`). `10.0408 kfpa` is a different
  upadeśa, and vidyut does not key it.
- So `KRP = ["10.0278"]` in `samjna.rs`, beside `CISPHUR`, and read the same
  way, `KRP.contains(&p.ctx.dhatupatha)`: no new tag. The two bhvādi rows
  join it when curated.
- A data test pins it: each listed row's upstream upadeśa is one of the
  three, and the two uncurated ones are absent from `dhatus()`. This is
  `cisphur_is_the_ciy_row_6_1_54_names`' pattern.
- The rule is tripādī, first in `TRIPADI`, as vidyut runs it before the rest
  of 8.2. It replaces `r` → `l` and `f` → `x` in `ANGA` (*karpay-* →
  *kalpay-*).

**8.2.78 is tripādī, right after 8.2.77, and reads the root inside a
ṇijanta aṅga.**

- After 3.1.32, the ṇic is part of `ANGA` (`urjay`). The rule reads the root
  as `ANGA`'s text with ṇic's `ay` stripped when the aṅga carries
  `Tag::Nijanta`, and as the whole text otherwise. In all four lakāras a
  ṇijanta aṅga reaches the tripādī as `…ay`, after 7.3.84 and 6.1.78.
- The guard is vidyut's: the root's last three sounds are a short ik, then
  `r` or `v`, then a hal. The ik lengthens. The `ay` and anything after it
  are kept.
- The trace diff confirmed it fires on no prior row.
- 8.2.79 *na bhakurchurām* needs no exclusion here. `kur` and `Cur` end in
  `r` itself, which is 8.2.77's shape, not this rule's.

**8.4.41 gains a second arm: a stu before a ṭu.**

- The sūtra pairs stu with ṣṭu in either order. vidyut implements both: its
  `(stu_x && SWU.contains(y))` arm turns `dw` into `qw`. This engine had only
  the ṣṭu-first arm.
- The new arm reads the previous sound before a ṭ-varga sound (`w W q Q R`)
  and substitutes it through a new `sound::shtutva_of`. That is the full stu
  → ṣṭu table: `s`→`z`, `t`→`w`, `T`→`W`, `d`→`q`, `D`→`Q`, `n`→`R`.
- A unit test, `shtutva_of_all_arms`, pins all six arms, after 8.4.40's
  `shcutva_of_stu_all_arms`. Only the `d` arm has a corpus witness.
- `z` is not a trigger for the new arm. 8.4.43 *toḥ ṣi* blocks a tu before
  ṣ. No curated row has `s` before `z`, so the arm makes no claim there.
- Strict adjacency is kept, as for the existing arm.
- The existing arm is unchanged, so its prior traces are byte-identical by
  construction. Its "CORRESPONDENCE side stays narrow" paragraph gains a
  sentence: the new arm, not this one, has the full table.

**`u~Drasa~` is udit by its initial `u~`.**

- vidyut tags udit from a `u~` marker anywhere in the upadeśa. 10g's
  marker reading looked only at the last marker, so it missed this row.
- `optional_nic_from_upadesha`'s 2570 arm becomes
  `u.ends_with("u~") || u.starts_with("u~") || u.ends_with('Y')`.
- `10.0270` is the only curādi upadeśa with a non-final `u~`; the vendored
  TSV confirms it.
- `OPTIONAL_NIC` gains `("10.0270", "2570")`. `10.0271 uDrasa~`, the
  next row, has no `u~` and stays plain `Nic`.

**One homograph.**

- `10.0026 curRa~` (*cūrṇayati*) shares all 72 forms with `10.0143 cUrRa~`,
  curated in 10k. Both rows' comments name the other.
- No other new form matches a curated row's.

**`10.0026`'s artha keeps upstream's trailing space.** The vendored TSV has
`"preraRe "`, and `dhatupatha_numbers_resolve_upstream` compares verbatim.
The prototype failed exactly there.

## Evidence

A throwaway worktree (`proto-10l-throwaway`, never pushed) appended the eight
rows to `DHATUS`, with the five rule changes and the data fix, on main at
`e47201c`.

- **vidyut's own prakriyās** for the eight rows, laṭ prathama eka both
  padas, came from a scratch example in `/tmp/vidyut-full`. The rule ids and
  their order above are read from that output.
- **Audit:** the committed harness (`tools/audit/`) ran with its totals
  asserts relaxed, against vidyut `8da2f90`, with the dev-deps repointed at
  the worktree. Result: **593 roots, 38160 cells, 49826 forms, zero
  differences** on the first run, and 6552 blocked branches.
- **Negative controls:** each rule, switched off by an environment variable
  in turn, made the audit fail.

  | rule switched off | differing cells |
  |---|---|
  | 6.1.75 | 72 |
  | 7.1.101 | 72 |
  | 8.2.18 | 72 |
  | 8.4.41's new arm | 72 |
  | 8.2.78 | 270 (`urj`'s 54, plus 72 each for `curR`, `gurd`, `kFt`) |

- **Prior traces:** a scratch test dumped every curated cell's branches,
  blocked ones included: index, blocked flag, text and credited ids. It ran
  on main and on the prototype. All **55676** prior branches are
  byte-identical.
- **New-rule rosters,** read from the same dump (live branches):
  - 6.1.75: `10.0170` only
  - 7.1.101: `10.0155` only
  - 8.2.18: `10.0278` only
  - 8.2.78: `10.0023` (59), `10.0026`, `10.0155`, `10.0180`
  - 8.4.41: `10.0037` added; every prior credit unchanged
  - 8.4.55: `10.0037` added
  - 8.4.40: `10.0170` added
- **The full suite, `--no-fail-fast`, failed in exactly ten tests.** Each is
  updated under Changes, and none is a defect:
  - `panini-data`: `curated_roots_have_expected_ganas_and_padas`,
    `curadi_rows_are_the_four_hundred_eighty_two_curated_roots`,
    `optional_nic_matches_upadesha_markers` (181 → 182), and
    `dhatupatha_numbers_resolve_upstream` (the trailing space)
  - `panini-prakriya`: `tinanta_rule_order_is_pinned`
  - `paradigm`: `paradigm_covers_every_enumerable_cell`
  - `trace`:
    - `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: 1.3.74 rows 419
      → 427
    - `the_optional_nic_ids_are_credited_only_on_their_rows`: 2570 on
      `10.0270`
    - `khari_ca_off_bhas_is_credited_exactly_as_before_3f2`: 8.4.55's count
      455 → 533, the 78 new credits all `10.0037`'s
    - `shcutva_off_jan_is_credited_exactly_as_before_3f3`: 8.4.40 on
      `10.0170`
  - No `check()` test failed. `cUrRayati` gains a second analysis, but no
    test counts it.

## Changes

### Engine — `panini-prakriya`

- **`tinanta/sanadi.rs`:** 6.1.75 and 7.1.101 entries, after the sanādi
  6.1.73 and before 7.3.86, as Decisions place them. The module doc's rule
  list and guard paragraph name both.
- **`tinanta/anga.rs`:** `che_ca`'s scan becomes the shared tuk helper.
  `che_ca` and `dirghat` are its two callers; the aṅga-stage 6.1.73 still
  points at `che_ca`.
- **`tinanta/tripadi.rs`:**
  - 8.2.18 first in `TRIPADI`, guarded on `KRP`.
  - 8.2.78 right after 8.2.77.
  - 8.4.41's new arm.
  - The module doc's range "8.2.77 … 8.4.56" becomes "8.2.18 … 8.4.56".
- **`tinanta/samjna.rs`:** `KRP` and its data test.
- **`tinanta/sound.rs`:** `shtutva_of` and `shtutva_of_all_arms`.
- **Unit tests,** one per new rule or arm, each on a hand-built prakriyā,
  in the style of the sanādi 6.1.73's `che_ca_gives_vich_tuk…`. Each
  asserts the firing shape and one near-miss that must decline:
  - 7.1.101: `kFt` beside ṇic → `kirt`; declines with no ṇic, and on an `F`
    final (`pF`).
  - 6.1.75: `mleC` → `mletC`; declines on a short vowel (that is 6.1.73's).
  - 8.2.18: on `10.0278` → `l`; declines on `10.0408`'s row.
  - 8.2.78: `urjay` (ṇijanta) → `Urjay`; declines on `Orjay`, on an `a`
    before the `r`, and on an aṅga whose `r` is final (8.2.77's shape).
  - 8.4.41: `dw` → `qw`; declines on `dz` (8.4.43).
- **`tinanta_rule_order_is_pinned`** gains the four ids at their positions.
  `exactly_the_pinned_vikalpa_rules_are_optional` is unchanged: none of the
  new rules is optional.

### Data — `panini-data/src/lib.rs`

- **8 `Dhatu` rows**, a 10l block after 10k's, in dhātupāṭha order. Each
  comment gives:
  - the number, upadeśa, artha and root
  - the rules it takes, in order
  - "Ubhayapadī by 1.3.74 (*…ayati*). Slice 10l."
  - for `10.0026`, its homograph partner `10.0143`, and `10.0143`'s comment
    gains the reverse pointer
- **`OPTIONAL_NIC`** gains `("10.0270", "2570")`. Its doc names the
  initial-`u~` reading.
- **`optional_nic_from_upadesha`'s** 2570 arm and doc, as in Decisions.
  `optional_nic_matches_upadesha_markers`: 181 → **182**.
- `curadi_rows_are_the_four_hundred_eighty_two_curated_roots` becomes
  `…_four_hundred_ninety_…`, its list and slice paragraph extended.
- `curated_roots_have_expected_ganas_and_padas`: 585 → **593**.
- The `pada` field doc: 1.3.74's 419 curādi rows → **427**, 585 → **593**.
  The agreement test's other buckets are re-derived, not assumed.

### Goldens — `crates/panini/tests/paradigm/data/curadi.rs`

**64 `ParadigmRow`s** (8 rows × 2 padas × 4 lakāras) and **90
`AlternateRow`s**. Generate them from the engine's output, and spot-check
them against vidyut's forms from the prototype's audit dump. Review with
`--diff-algorithm=histogram`, and keep the goldens out of review packages.

### Tests

- `derivation_set_shape_matches_the_audited_numbers`: 37584 → **38160**
  cells, 49160 → **49826** forms. Buckets are re-derived from the census.
  The doc gains a 10l paragraph and "OPEN at 490 of its 509 rows".
- `paradigm_covers_every_enumerable_cell` passes once the goldens land.
- **Rosters,** each extended with the rows Evidence names and its comment
  saying why:
  - `a_kusmad_is_credited_on_exactly_the_akusmiya_cells`: 419 → 427
  - `the_optional_nic_ids_are_credited_only_on_their_rows`: 2570 gains
    `10.0270`
  - `khari_ca_off_bhas_is_credited_exactly_as_before_3f2`: 455 → 533, the
    new credits pinned to `10.0037`
  - `shcutva_off_jan_is_credited_exactly_as_before_3f3`: + `10.0170`
- **A new corpus-wide trace test,
  `the_10l_rules_fire_only_on_their_rows`** in `trace/curadi.rs`. It pins
  each new id's exact row set, and 8.4.41's curādi credits to `10.0037`
  alone, over every curated cell (`credited(id)`). This is the guard against
  a new rule firing silently on a prior row: goldens compare forms, not
  traces.
- **`check()`:** a new `curadi_analyses_its_10l_forms`. One witness per row
  (laṭ P 3sg above), plus `10.0270`'s ṇic-less *Drasati*. `cUrRayati` gets
  exactly two analyses, `10.0026` and `10.0143`, and every other witness
  exactly one. Grep the goldens for every form before an assertion names
  its root.
- **Prior traces:** dump every prior cell's branches, blocked included, on
  main and on HEAD. They must be byte-identical, all 55676.

TDD: each rule's unit test is written first and seen failing. The census,
coverage, roster, fires-only-on-rows and `check()` assertions are added
first and seen failing with the rows absent; the rows and rules then make
them pass.

## Audit

Copy the harness from `tools/audit/`; do not rewrite it. Repoint
`/tmp/vidyut-full`'s dev-deps at the worktree first, or it audits the
pre-slice engine.

1. The `entry` negative control must fail first, on √bhū's 36 cells.
2. Then the full run must give **zero differences over 593 roots / 38160
   cells / 49826 forms**, with 6552 blocked branches.

The harness's asserted totals and their comments move with it.

## Mutation gate

This slice adds production code, so it adds mutants. Every new mutant must
be caught. The unit tests and the fires-only-on-rows test above exist for
that.

- Re-measure the full uncaught-suite floor and the `skip_nic -> true`
  reading under the campaign's load. The new optional-ṇic row adds no
  optional-ṇic id, but it adds cells.
- Set the cap by AGENTS.md's rule. Pass `-o`, and copy `outcomes.json`
  durably before any other invocation.
- Expected: the non-caught set is identical to 10k's, which AGENTS.md names
  verbatim. A new survivor in the 10l code is a missing test, to be fixed
  with one, not excused.

## Doc sweep

README, AGENTS.md, `docs/ARCHITECTURE.md`, `tools/audit/` (README and the
harness), and the test docs:

- **Counts:** 585 → 593 roots; 37584 → 38160 cells; 49160 → 49826 forms;
  11576 → 11666 `ALTERNATES`; 6516 → 6552 blocked branches.
- **Curādi:** "OPEN at 482 of its 509" → **490 of 509**; AGENTS.md's gaṇa
  history gains a 10l clause.
- **Rule inventories:** every list of implemented sūtras gains 6.1.75,
  7.1.101, 8.2.18 and 8.2.78, and says 8.4.41 now has both arms.
- **ARCHITECTURE's census paragraphs:** re-derived from the test census,
  including the "pins all N ids" line at the census paragraph's top.
- **The audit README:** a 10l "Last recorded result" entry.
- **`tinanta/sanadi.rs`:** the module doc's "6.1.73 fires only on √vich and
  √pich" gains 6.1.75 on √mlecch and 7.1.101 on √kṝt.
- **8.4.41's comment block:** the CORRESPONDENCE paragraph as Decisions
  says. Every comment that calls 8.4.41 one-directional or says "only after
  a ṣṭu" is corrected.
- **Earlier specs:**
  - 10k's "Later slices" 10l bullet gains a pointer to this spec. It also
    notes the corrections: 7.1.101 not 7.1.100, 8.4.55 on `adwa~`, and the
    initial `u~`.
- **Sweep greps:**
  - spelled-out counts with `grep -i` ("four hundred eighty-two", "five
    hundred eighty-five", "four hundred nineteen")
  - numerals: 482, 585, 419, 37584, 49160, 11576, 6516, 181
  - wrapped counts and rule-scoped counts
  - "not yet curated" / "uncurated" / "later slice" / "10l" phrasings
    naming these rows
  - all of `crates/`, including tests
  - recorded file:line anchors, re-grepped at final HEAD

## Success criteria

- The 576 new cells match vidyut, and every prior branch's trace is
  byte-identical between main and HEAD.
- The audit gives zero differences over 593 / 38160 / 49826, after its
  negative control fails.
- Each new rule is credited only on the rows this spec names, pinned by
  `the_10l_rules_fire_only_on_their_rows`.
- The mutation campaign catches every new mutant. Its non-caught set is
  unchanged from 10k's and named in AGENTS.md.
- `mise run test` and clippy are clean.

## Later slices

- **10m: `10.0175 picca~` and the 8.2.30 narrowing.** vidyut fires 8.2.30
  *coḥ kuḥ* only on a term-final cu before an affix or āgama beginning with
  a jhal, or pada-finally. This engine scans the whole word and turns
  root-internal `cc` into `kc` (*pikcayati* → *piccayati*). The rudhādi rows
  depend on 8.2.30: *ric*, *vic*, *bhañj*. Prototype off 10l's HEAD and diff
  every prior trace before planning.
- The remaining eighteen curādi rows after 10m: re-scope them by prototype
  first.
- Upasargas, and with them `10.0368 za\da~` (7.3.78) and 6.1.76 *padāntād
  vā*.
- `gupU~`, `paRa~\` and `pana~\` join `AYA` when curated. `kfpU~\` and
  `kfpa~\` join `KRP` when curated.
- A causative (hetumaṇic) slice takes 01.0934 and 10.0494, and with them
  7.3.36's named roots and 6.1.54 on `sPura~`.
