# Svādi 5b: the last thirty-two rows, 6.4.24 and 8.4.39

Svādi (5) is open at 6 of its 38 dhātus: `05.0012 hi`, `05.0016 Ap`,
`05.0017 Sak`, `05.0020 aS`, `05.0021 stiG`, `05.0032 ri`. This slice curates
the other thirty-two and **closes the gaṇa at 38 of 38**.

- **Thirty rows match vidyut-prakriya on data alone.**
- **Two need a rule:**
  - √dambh (`05.0026 danBu~`) needs **6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti***.
  - √tṛp (`05.0028 tfpa~`) needs **8.4.39 *kṣubhnādiṣu ca***.
- **6.4.24 lands as the general sūtra, not as a svādi special.** Kryādi's
  slice 9e has ten more rows that need it (√bandh, √granth, √manth, the
  stambh group), and it reuses the rule this slice lands.

## Scope

**In:**

- All thirty-two remaining rows:
  - ten ñit rows, ubhayapadī by 1.3.72;
  - twenty-two parasmaipadī by 1.3.78.
- Laṭ / laṅ / loṭ / vidhiliṅ.
- 6.4.24, with a data-layer idit verdict (`IDIT`) and `Tag::Idit`.
- 8.4.39's svādi arm (√tṛp + śnu).
- **Totals:**

  | | before | after |
  |---|---|---|
  | roots | 594 | **626** |
  | cells | 38232 | **39744** (+1512 = 10 × 72 + 22 × 36) |
  | forms | 49904 | **51724** (+1820) |
  | `ALTERNATES` rows | 11672 | **11980** (+308) |
  | blocked branches | 6552 | 6552 |

  - If hetumaṇic or kryādi 9c merges first, the slice rebases and re-measures
    the totals. It never adds them up by hand.

**Out:**

- 8.4.39's other arms: kṣubh + śnā is kryādi 9d's; skambh and nṛt + yaṅ have
  no curated row.
- Kryādi's 6.4.24 rows (9e).
- Upasargas.

| row | upadeśa | code | pada |
|---|---|---|---|
| 05.0001 | zu\Y | su | both |
| 05.0002 | zi\Y | si | both |
| 05.0003 | Si\Y | Si | both |
| 05.0004 | qumi\Y | mi | both |
| 05.0005 | ci\Y | ci | both |
| 05.0006 | stf\Y | stf | both |
| 05.0007 | kf\Y | kf | both |
| 05.0008 | vfY | vf | both |
| 05.0009 | Du\Y | Du | both |
| 05.0010 | DUY | DU | both |
| 05.0011 | wudu\ | du | P |
| 05.0013 | pf\ | pf | P |
| 05.0014 | spf\ | spf | P |
| 05.0015 | smf\ | smf | P |
| 05.0018 | rA\Da~ | rAD | P |
| 05.0019 | sA\Da~ | sAD | P |
| 05.0022 | tika~ | tik | P |
| 05.0023 | tiga~ | tig | P |
| 05.0024 | zaGa~ | saG | P |
| 05.0025 | YiDfzA~ | Dfz | P |
| 05.0026 | danBu~ | danB | P (6.4.24) |
| 05.0027 | fDu~ | fD | P |
| 05.0028 | tfpa~ | tfp | P (8.4.39) |
| 05.0029 | aha~ | ah | P |
| 05.0030 | daGa~ | daG | P |
| 05.0031 | camu~ | cam | P |
| 05.0033 | kzi\ | kzi | P |
| 05.0034 | ciri | ciri | P |
| 05.0035 | jiri | jiri | P |
| 05.0036 | dASa~ | dAS | P |
| 05.0037 | df\ | df | P |
| 05.0038 | f\kzi | fkzi | P |

No svādi row is idit, so 7.1.58's stored-num simplification never applies
here. `ciri`, `jiri` and `fkzi` carry no anubandha. vidyut agrees on every
cell (*ciriṇoti*, *ṛkṣiṇoti*).

### Decisions

**6.4.24 is general. It reads the following term's ṅit and the root's idit
verdict.**

- **What the rule does.** It applies when:
  - the root is not idit (`!Tag::Idit`);
  - `ANGA` ends in a hal;
  - its penultimate sound is `n`;
  - the next non-empty term after `ANGA` carries `Tag::Ngit`.

  It deletes that `n` and records once.
- **Placement.** It sits in `ANGA_RULES` directly after 6.4.23, where the
  prototype put it and where vidyut's trace puts it (after 1.2.4, before
  7.3.84).
- **Why the ṅit reading.** 1.2.4's second application tags an apit
  sārvadhātuka vikaraṇa `Tag::Ngit`: śnu, śnā, śyan, śa. Where the vikaraṇa
  is empty (luk, ślu), the next non-empty term is the ending, which 1.2.4's
  first application tags. So reading the next non-empty term's `Ngit` is the
  sūtra's *kṅiti* for every gaṇa. Śap is pit and never ṅit, so bhvādi is
  untouched by construction.
- **Why an idit verdict and not text.**
  - The engine strips anubandhas, and 7.1.58 *idito num dhātoḥ* is a stated
    simplification: idit roots are stored with their num already in
    (`07.0019 hins` for `hisi~`).
  - So `hins` and a hypothetical anidit `hins` look alike, and only the
    upadeśa can tell them apart.
  - `IDIT` is a curated `&[&str]` of dhātupāṭha numbers in `panini-data`, as
    `OPTIONAL_NIC` and `AYA` are.
  - `derive()` sets `Tag::Idit` from it, as it sets `Tag::Mrj`.
  - A data test holds it to the vendored upadeśa **in both directions**: every
    curated row whose upadeśa carries the `i~` it-marker is listed, and every
    listed row carries it. It uses the same marker reading as
    `optional_nic_from_upadesha`'s 2564 arm.
- **The idit guard is unreachable today.** No curated idit root with a
  nasal upadhā meets a ṅit affix:
  - curādi's idit rows meet ṇic or śap;
  - √hiṃs's nasal is 6.4.23's, inside śnam's split.

  So, as with 10m's `j != ANGA`, a **hand-built unit test** pins it: an idit
  `danB`-shaped term before a ṅit śnu must not lose its `n`.
- **Existing traces must not move.** Rudhādi's śnam roots are untouched:
  3.1.78 splits the root's tail into `SHAP` (`Ba | na | nj`), so `ANGA` has
  no nasal upadhā. The full trace dump (Tests) is what proves this, not this
  argument.
- **Ownership.** This slice owns the rule. 9e only adds rows to its witness
  list.

**8.4.39 is keyed by row.**

- It is a no-op rule placed just before 8.4.1, whose `bars` are `["8.4.1",
  "8.4.2"]`.
- It fires when the row is `05.0028` (a `KSUBHNADI` constant in `samjna.rs`'s
  style, holding one row) and the next non-empty term after `ANGA` is śnu.
- **Why by row:** curādi's `10.0351` and `10.0355` are also `tfpa~` and store
  `tfp`. Kryādi 9d appends `09.0055 kzuB` (+ śnā) to the same constant.
- **8.4.39 changes no text.** It records and bars, as 6.4.117 *ā ca hau*
  does.

**`cisphur_is_the_ciy_row_6_1_54_names` is reworded, not weakened.** It
asserted `05.0005 ci\Y` is uncurated. It now asserts the row is curated and
outside `CISPHUR`, and that its forms carry no 6.1.54 credit.

**Shared codes are allowed. There is no surface collision.**

- These new codes repeat existing ones, which is allowed because `code` is not
  a key:

  | new code | existing rows |
  |---|---|
  | kf | 08.0010 |
  | vf | 09.0045, 10.0345 |
  | ci | 10.0124, 10.0325 |
  | DU | 10.0372 |
  | pf | 03.0005 |
  | smf | 01.1082 |
  | Dfz | 10.0388 |
  | tfp | 10.0351, 10.0355 |

- The prototype's whole-corpus dump found no surface form of a new row equal
  to any other root's.
- The plan derives its `check()` homograph assertions from the goldens, not
  from this list.

## Evidence

- **Throwaway prototype off `b2707e7`**, worktree `proto-svadi`, scratch
  `…/scratchpad/svadi/`.
  - Data alone: 72 differing cells, √dambh's 36 and √tṛp's 36.
  - With both rules: **zero differences** over 39744 cells, 51724 forms and
    626 roots.
  - The `entry` negative control flagged its 36 √bhū cells.
  - vidyut's step traces:
    - √dambh: 1.2.4 → 6.4.24 → 7.3.84, giving *dabhnoti*;
    - √tṛp: 8.4.39 blocking ṇatva, giving *tṛpnoti*.
  - A trace dump of all 594 prior roots' branches (56456 lines, blocked ones
    included) was **byte-identical** with and without the engine edits.
- **The prototype tested a narrower 6.4.24** (gated on `Tag::Svadi`). The
  general guard here must reproduce that byte-identical result.
  - Plan task 1 re-runs the dump with the general guard before anything
    else.
  - If any prior trace moves, the slice stops and this spec is amended.

## Changes

### Data: `panini-data`

- **The thirty-two rows**, with their artha, as tabled.
- **`IDIT`** and its doc comment: the 7.1.58 storage convention and why
  6.4.24 needs it.
- **Tests:**
  - `idit_matches_upadesha_markers`, both directions;
  - the svādi row-list test, renamed for its new count;
  - `Dhatu::pada`'s census and the row count;
  - `curated_pada_agrees_with_upadesha_markers` re-derives all 32 new
    verdicts.

### Engine: `panini-prakriya`

- **`Tag::Idit`**, set in `derive()` from `IDIT`.
- **6.4.24** in `ANGA_RULES` after 6.4.23. 6.4.23's comment drops "not needed
  in this slice" and points to 6.4.24.
- **8.4.39** in `TRIPADI` before 8.4.1, plus the `KSUBHNADI` constant.
- **Pins:** `exactly_the_pinned_bars` and `tinanta_rule_order_is_pinned` gain
  the two ids.

### Goldens: `crates/panini/tests/paradigm/data/svadi.rs`

- 32 roots' paradigm rows and their 308 `ALTERNATES` rows.
- `derivation_set_shape_matches_the_audited_numbers` gains its 5b paragraph
  and totals.

### Tests

- **Trace pins:**
  - *dabhnoti* (6.4.24);
  - *tṛpnoti* (8.4.39 credited, no 8.4.1/8.4.2);
  - *sunute* (ñit ātmanepada by 1.3.72);
  - *ṛkṣiṇoti* (ṇatva after a non-adjacent trigger; the pin records which of 8.4.1 or 8.4.2 is credited).
- **6.4.24 unit tests:**
  - fires on an anidit nasal-upadhā hal-final aṅga before a ṅit vikaraṇa;
  - declines when `Idit` (the unreachable guard, hand-built);
  - declines before a pit or non-ṅit term;
  - declines on a vowel-final or non-`n`-upadhā aṅga.
  - Witness lengths must separate index mutants on `chars[n - 2]` (n ≥ 5, per
    the 10l lesson).
- **8.4.39 unit tests:** fires on `05.0028` only; declines on `10.0351`'s
  `tfp`.
- **Fires-only-on-rows:** a corpus-wide test that 6.4.24 fires only on
  `05.0026` and 8.4.39 only on `05.0028`. 9e widens the first.
- **The CISPHUR test**, reworded as above.
- **Prior-trace stability:** a main↔HEAD dump of every prior branch, blocked
  ones included, with every step's before and after text, byte-identical.

## Audit

- Copy `tools/audit/panini_full_audit.rs` into a private vidyut copy, and
  repoint its dev-dependencies at this worktree.
- Raise the totals to 626 / 39744 / 51724.
- Run the `entry` negative control.
- Record the result in `tools/audit/README.md`.

## Mutation gate

- **`--in-diff`** over the production diff: every mutant in 6.4.24, 8.4.39 and
  the `Tag::Idit` wiring is caught or documented as equivalent.
- **This is the first of the queued campaigns.** Campaigns never run
  concurrently on this host.
- No optional-ṇic vikalpa is added, so the `skip_nic` fork count stays at 2^8.
  The corpus grows by 4.0%.
- **Floor and cap.** Re-measure the floor and an isolated `-j 4` probe of the
  documented equivalents and the `skip_nic` pair. Set the cap by AGENTS.md's
  rule (2 × max(probe, campaign) on `skip_nic`, never lowered on the quieter
  reading). `mise.toml` and the AGENTS.md paragraph change together.
- **Records.** Name every missed and timeout mutant verbatim. Copy
  `outcomes.json` durably.

## Doc sweep

- **README Scope:** svādi becomes **complete at 38 of 38**, with 6.4.24 and
  8.4.39 described.
- **Totals everywhere:** 594 → 626, 38232 → 39744, 49904 → 51724, 11672 →
  11980. Also the multi-form cell counts.
- **ARCHITECTURE:** the svādi entry, and 6.4.24's place beside 6.4.23.
- **AGENTS.md:** the gaṇa coverage sentence, the floor/cap paragraph and the
  record.
- **Sweep greps:**
  - the old numerals, and their spelled-out forms with `grep -i` ("five
    hundred ninety-four", …);
  - wrapped and rule-scoped counts;
  - "6 of its 38", "svādi … open";
  - "6.4.24 … not needed";
  - all of `crates/`, including tests;
  - file:line anchors, re-grepped at final HEAD.

## Success criteria

- All 1512 new cells match vidyut, and the `entry` control fails.
- Every prior branch's trace is byte-identical to main's.
- `mise run test`, `lint` and `fmt-check` are green.
- The mutation gate is met at a re-measured cap.
- Svādi is complete at 38 of 38.

## Later slices

- **Kryādi 9e:** 6.4.24's ten kryādi rows and 3.1.82 (optional śnu on √sku
  and the stambh group).
- **Kryādi 9d:** 8.4.39's śnā arm (√kṣubh), among its other rules.
