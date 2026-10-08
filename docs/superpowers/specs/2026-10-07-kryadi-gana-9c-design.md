# Kryādi 9c: twenty-two rows on data alone

Kryādi (9) is open at 6 of its 71 dhātus:

- `09.0040 vrI`
- `09.0045 vf`
- `09.0053 guD`
- `09.0058 kliS`
- `09.0059 aS`
- `09.0066 muz`

A throwaway prototype curated all sixty-five remaining rows against
vidyut-prakriya.

- **Twenty-two match cell for cell on data alone.**
- The other forty-three need nine rule changes. The two follow-on slices
  take them:
  - **9d:** phonology and root specials;
  - **9e:** 6.4.24's ten rows and 3.1.82.

This slice curates the twenty-two and changes no engine code. After it,
kryādi is **open at 28 of its 71** dhātus.

## Scope

**In:** the twenty-two rows below, across laṭ / laṅ / loṭ / vidhiliṅ.

**Totals, measured on the prototype:**

| | before | after | change |
|---|---|---|---|
| roots | 594 | **616** | +22 |
| cells | 38232 | **39312** | +1080 = 8 × 72 + 14 × 36 |
| forms | 49904 | **51116** | +1212 |
| `ALTERNATES` rows | 11672 | **11804** | +132 |
| blocked branches | 6552 | 6552 | none |

If svādi 5b or hetumaṇic merges first, this slice rebases and re-measures the
totals. It never adds them up by hand.

| row | upadeśa | code | pada |
|---|---|---|---|
| 09.0001 | qukrI\Y | krI | both (1.3.72) |
| 09.0002 | prI\Y | prI | both |
| 09.0003 | SrI\Y | SrI | both |
| 09.0004 | mI\Y | mI | both |
| 09.0005 | zi\Y | si | both |
| 09.0011 | yu\Y | yu | both |
| 09.0012 | knUY | knU | both |
| 09.0013 | drUY | drU | both |
| 09.0041 | BrI\ | BrI | P (1.3.78) |
| 09.0042 | kzI\z | kzI | P |
| 09.0051 | mfda~ | mfd | P |
| 09.0054 | kuza~ | kuz | P |
| 09.0056 | RaBa~ | naB | P |
| 09.0057 | tuBa~ | tuB | P |
| 09.0060 | u~Drasa~ | Dras | P |
| 09.0061 | iza~ | iz | P |
| 09.0062 | vi\za~ | viz | P |
| 09.0063 | pruza~ | pruz | P |
| 09.0064 | pluza~ | pluz | P |
| 09.0065 | puza~ | puz | P |
| 09.0067 | Kaca~ | Kac | P |
| 09.0070 | svF | svF | P |

Two of the codes are not the plain stripped upadeśa:
- `09.0005` stores `si` by 6.1.64.
- `09.0056` stores `naB` by the 6.1.65 stored-form convention (√ṇij's
  precedent).

**Out:**

- **9d:** 7.3.80 *pvādīnāṃ hrasvaḥ* (25 rows), 7.1.100 (√śṝ), 6.1.16 /
  6.1.108 / 6.4.2 (√jyā, √grah), 7.3.79 with 1.3.76 (√jñā), 8.4.41 widened
  (√mṛḍ, √heṭh), 8.4.39's śnā arm (√kṣubh), 6.4.19 (√khav), and the
  pada-aware sibling verdict that the `SF`, `kF` and `vF` pairs need.
- **9e, after svādi 5b lands 6.4.24:** 6.4.24's ten rows (√bandh, √granth,
  √manth, the two √śranth rows, √kunth, the stambh group) and 3.1.82 (√sku,
  the stambh group).

### Decisions

**No engine change, and no sibling-pair test change.**

- The prototype found three same-gaṇa sibling pairs (`SF`, `kF`, `vF`), each
  an ubhayapadī row and a parasmaipadī one. They fail
  `dhatupatha_numbers_resolve_upstream`.
- None of the twenty-two is in a pair, so that test is untouched here, and
  9d takes it.
- The two √śranth rows are 9e's.

**√khac adds one credit to an existing roster.**

- *khacñāti* credits 8.4.40 *stoḥ ścunā ścuḥ*. So
  `shcutva_off_jan_is_credited_exactly_as_before_3f3`'s row list gains
  `09.0067`.
- This is the expected widening of a roster test, not a new rule. The
  rewording names √khac, as 3f3 named √jan.

**Codes may repeat across gaṇas. Surface forms do not collide.**

- These codes are shared with other gaṇas:

  | code | also stored by |
  |---|---|
  | mI, prI, yu, Dras, puz | curādi rows |
  | viz | juhotyādi `03.0014` |
  | si | svādi `05.0002` (if 5b lands first) |

- The prototype's whole-corpus dump found no surface form of the twenty-two
  equal to any other root's.
- The plan derives its `check()` homograph assertions from the goldens and
  greps every homograph row, per the 3f lesson, instead of hand-picking them.

## Evidence

The throwaway prototype was built off `b2707e7`, in worktree `proto-kryadi`,
with scratch output in `…/scratchpad/kryadi/`.

- **Data-only audit.** The data-only run matched these twenty-two rows on
  every cell.
- **The negative control was not run on the prototype.**
  - The 1863 differences on its data-only run show that the copy can see
    differences. That is weaker than the `entry` control, so the slice runs
    the control before recording anything.
- **Prior traces.** A dump of every prior branch (56456 lines, blocked ones
  included) was byte-identical to main's.
- **Suite.** Apart from totals, goldens, the sibling pairs and the √khac
  roster, the full suite with `--no-fail-fast` passed, roundtrip included.

## Changes

- **Data, `panini-data`:**
  - the twenty-two rows with their artha;
  - the kryādi row-list test, renamed for its count;
  - `Dhatu::pada`'s census and the row count;
  - `curated_pada_agrees_with_upadesha_markers` covers all twenty-two.
- **Goldens, `crates/panini/tests/paradigm/data/kryadi.rs`:**
  - the paradigm rows and their 132 `ALTERNATES` rows;
  - `derivation_set_shape_matches_the_audited_numbers` gains its 9c
    paragraph and totals.
- **Tests:**
  - trace pins for *krīṇāti* (8.4.2 across the `I`) and *krīṇīte*
    (ātmanepada, 6.4.113);
  - *khacñāti*;
  - the √khac roster update;
  - goldens-derived homograph checks;
  - a main↔HEAD trace dump of every prior branch, byte-identical.
- **Engine:** none.

## Audit

- Copy the committed harness into a private vidyut copy.
- Repoint that copy's dev-dependencies at this worktree.
- Raise the totals to 616 / 39312 / 51116.
- Run the `entry` negative control.
- Record the result in `tools/audit/README.md`.

## Mutation gate

- **No production diff.** `--in-diff` over the data crate's diff should list
  only test or data code, as 10k's did.
- **Full campaign.** The suite grows by 2.8%, so a full campaign still runs.
- **Queue.** It runs after svādi 5b's in the serial queue, on the rebased
  tree.
- **Floor and cap.** The floor and the `skip_nic` probe are re-measured, and
  the cap follows AGENTS.md's rule. `mise.toml` and the AGENTS.md paragraph
  change together.
- **Record.** Every missed and timeout mutant is named verbatim.

## Doc sweep

- **README Scope:** kryādi's coverage is 28 of 71, naming the 9c rows by
  group.
- **Totals everywhere:** 594, 38232, 49904 and 11672, and the multi-form cell
  counts.
- **ARCHITECTURE and AGENTS.md:** the gaṇa coverage sentences, plus the
  floor/cap paragraph and the mutation record.
- **Sweep greps:**
  - old numerals and their spelled-out forms with `grep -i`;
  - wrapped counts and "6 of its 71";
  - all of `crates/`, including tests;
  - file:line anchors, re-grepped at final HEAD.

## Success criteria

- All 1080 new cells match vidyut, and the `entry` control fails.
- Every prior branch's trace is byte-identical to main's.
- `mise run test`, `lint` and `fmt-check` are green.
- The mutation gate is met at a re-measured cap.
- Kryādi is open at 28 of 71.

## Later slices

- **9d:** 7.3.80, 7.1.100, 6.1.16/6.1.108/6.4.2, 7.3.79 and 1.3.76, 8.4.41
  widened, 8.4.39's śnā arm, 6.4.19, and the sibling-pair verdict. That is
  34 rows.
- **9e:** 6.4.24's ten rows (on svādi 5b's general rule) and 3.1.82, which is
  a new vikalpa and forks only its own rows.
