# Curādi 10j: the ajanta rows

Every "Later slices" list from 10b to 10i names √gṛ (`10.0231`), √yu
(`10.0235`), √smiṅ and √ci (`10.0124`) as owed. This slice curates them,
and with them the four other ajanta curādi rows no list named: `10.0152 Gf`,
`10.0258 jYA`, `10.0275 cyu` and `10.0277 BU`. Eight rows, all vowel-final
before ṇic.

Five need nothing new: 7.2.115 and the sanādi 6.1.78, from 10h, already
derive them. Three things are new:

- 7.3.36 *artihrīvlīrīknūyīkṣmāyyātāṃ puk ṇau* (`jYA`, and √ci's ā branch);
- 6.1.54 *cisphuror ṇau*, optional, on √ci;
- Kaumudī 2567, which keeps √smiṅ ātmanepadī under ṇic.

6.4.92 *mitāṃ hrasvaḥ* also moves: √ci is the long-ik mit root its comment
says must revisit it.

A throwaway prototype built all of this and audited it against vidyut
(Evidence): zero differences across the whole corpus, and every
pre-existing branch's trace byte-identical.

## Scope

**In:** 8 rows, every pada each admits, across laṭ / laṅ / loṭ / vidhiliṅ:
13 root × pada blocks × 4 lakāras × 9 = 468 cells. Codes are `stored_form`'s
output.

| row | upadeśa | code | `PadaAssignment` | padas | laṭ prathama eka |
|---|---|---|---|---|---|
| `10.0058` | `zmiN` | `smi` | `Atmanepada` | Ā | *smāyayate* |
| `10.0124` | `ciY` | `ci` | `NicUbhayapada` | P, Ā | *cayayati*, *capayati*, *cayati* |
| `10.0152` | `Gf` | `Gf` | `Nic` | P, Ā | *ghārayati* |
| `10.0231` | `gf` | `gf` | `Akusmiya` | Ā | *gārayate* |
| `10.0235` | `yu` | `yu` | `Akusmiya` | Ā | *yāvayate* |
| `10.0258` | `jYA` | `jYA` | `Nic` | P, Ā | *jñāpayati* |
| `10.0275` | `cyu` | `cyu` | `Nic` | P, Ā | *cyāvayati* |
| `10.0277` | `BU` | `BU` | `Nic` | P, Ā | *bhāvayati* |

`10.0124` joins `OPTIONAL_NIC` as `("10.0124", "2570")`: it is ñit, and
2570 is the ṇic vikalpa 10f built and 10g used for every other ñit/udit row.

**Out:**

- The ~165 plain hal-anta obligatory-ṇic rows of `10.0006`–`10.0191` and
  `10.0237`–`10.0278` (Later slices). No earlier spec listed them.
- 7.3.36's named roots (`f`, `hrI`, `vlI`, `rI`, `knUy`, `kzmAy`): none is
  a curādi row, and with no causative none meets ṇic. The rule's ā-final
  arm is the only one in scope.
- 6.1.54 on `sPura~`: tudādi, so it never meets ṇic without a causative.
- 1.4.13, which vidyut credits before 7.3.36: no sanādi rule credits it
  here, as for every earlier ṇic row.
- `10.0368 za\da~`, upasargas, the causative, and everything on 10a's to
  10i's out-of-scope lists.

### Decisions

**7.3.36 appends `p` to the aṅga's text, before 7.2.115.**
- **What it does:** before ṇit ṇic, an ā-final aṅga takes the augment puk:
  `jYA` → `jYAp` (*jñāpayati*), and on √ci's 6.1.54 branch `cA` → `cAp`.
- **Why text, not a term:** puk is kit, so by 1.1.46 *ādyantau ṭakitau*
  it is the aṅga's final part. The vārttika 7.3.37.2's nuk already sets the
  precedent: its `n` goes onto the aṅga's text (`DUn`), with no term and no
  recorded it-lopa. vidyut inserts a term, but a term between `ANGA` and
  `NIC` would move ṇic off the `NIC` index that every sanādi rule reads.
  The forms are the same either way.
- **Why before 7.2.115:** `vrddhi_of('A')` is `Some("A")`, so 7.2.115 would
  fire on a bare `jYA` and record a step that vidyut does not credit. After
  puk the aṅga ends in `p`, and 7.2.115 declines on its own guard.
- **Guard:** ṇit ṇic at `NIC`, and an aṅga ending in `A`. Not a vikalpa.

**6.1.54 is a vikalpa on √ci, keyed by row number.**
- **What it does:** before ṇic, `ci` → `cA`. Taken, 7.3.36 then gives
  `cAp`, and 6.4.92 gives `cap` (*capayati*). Declined, 7.2.115 and 6.1.78
  give `cAy`, and 6.4.92 gives `cay` (*cayayati*).
- **Why by number:** vidyut keys it on the upadeśas `ciY` and `ci\Y`. 10i
  already curated `10.0325 ci` (*bhāṣāyām*), which stores as the same `ci`
  but is not `ciY`. The prototype first keyed on the text and gave
  `10.0325` a spurious *cāpayati* in 72 cells. So the guard reads the row:
  `p.ctx.dhatupatha == "10.0124"`, the `AYA` and `MRJ` precedent. The
  plan decides whether this is a one-entry `panini-data` constant or a
  literal with a data test pinning that `10.0124`'s upadeśa is `ciY`; either
  way, a test holds the key to the upadeśa.
- **Placement:** after 6.4.48 and before 7.2.116. vidyut credits it right
  after 3.4.114, before 1.4.13.
- **The fork:** declined is index 0 (*cayayati*), taken is index 1
  (*capayati*). 2570's ṇic-less branch (*cayati*) is index 2. Each of √ci's
  cells holds three forms.

**6.4.92 moves to just after the sanādi 6.1.78.**
- **The problem:** it sits before 7.2.115. On √ci's 6.1.54-declined branch
  it would read `ci` and decline, leaving `cAy` (*cāyayati*). vidyut gives
  *cayayati*.
- **The move:** after 7.2.115 and the sanādi 6.1.78, before 7.2.114. It
  then reads `cAy` and `cAp` and shortens their upadhā.
- **Why nothing else changes:** the six earlier mit roots are a-roots.
  7.2.115, 7.3.36, 7.3.37.2 and the sanādi 6.1.78 all decline on them
  (`jYAp`, `yAm`, … end in a consonant), so 6.4.92 still reads 7.2.116's
  output. The trace dump confirms it (Evidence).
- **Its comment:** the "A long-ik mit root (√ci) must revisit" caveat
  becomes the explanation of the new placement.

**10.0493 moves ahead of the optional-ṇic vikalpas.**
- **Why:** vidyut credits 10.0493 before 2570 (trace
  `10.0493 → 2570` on *cayati*). Today the stage runs the eight vikalpas
  first, so √ci's ṇic-less trace would read `2570 → 10.0493`.
- **Why it is safe:** it fires only on `Tag::Mit` rows. Until now none of
  them had an optional ṇic, so no existing trace has a vikalpa id before
  10.0493.
- 10.0496 and 10.0497 stay after the vikalpas: vidyut decides ṇic before
  them ("First decide Ric-pratyaya, since this affects the scope of
  AkusmIya").

**Kaumudī 2567 is a samjna rule before 1.3.12, guarded on tags.**
- **What it does:** √smiṅ is ṅit, so 1.3.12 makes it ātmanepadī. Under ṇic
  1.3.74 would normally govern, and 1.3.78 would allow parasmaipada too.
  The Kaumudī reads the ṅit as meaningful only if it keeps the ṇijanta
  ātmanepadī. vidyut steps `Kaumudi("2567")` and then runs 1.3.12
  (`atmanepada.rs`); the trace is `2567 → 1.3.12`.
- **Guard:** `Tag::Curadi`, `Tag::Atmanepadin` (`PadaAssignment::Atmanepada`)
  and `Tag::Nijanta`, in the ātmanepada pada. On a parasmaipada request it
  declines, and 1.3.12 blocks as for any ātmanepadī root.
- **Where ṅit-ness lives:** the engine never sees upadeśa markers (the
  `OPTIONAL_NIC` doc). So the data layer holds it:
  `curated_pada_agrees_with_upadesha_markers` admits a curādi `Atmanepada`
  row only if its upadeśa's final marker is `N`. That keeps an anudāttet
  row like `za\da~` out: vidyut gives it 1.3.74 under ṇic, so it must be
  `Nic`, never `Atmanepada`. The test's comment already names `10.0058 zmiN`
  as the row waiting for this decision.
- **Name:** vidyut gives the id but no text. The plan takes the
  Siddhānta-kaumudī's wording from the ashtadhyayi-com data repo
  (`sutraani/kaumudi.txt`) and does not paraphrase it.

**√gṛ and √yu are ākusmīya.** 10.0496 settles their pada, as for the other
43 rows of `AKUSMIYA`. They were left out of 10c only because 7.2.115 was
not yet in.

**Stored codes need nothing new.** `stored_form("zmiN")` is `smi` by
6.1.64, which is never credited, as for every earlier ṣ-initial row.

**Homographs follow the `cah` / `rah` precedent:** a shared surface
expects one analysis per row.

- `10.0124 ci` shares 72 forms with `10.0121 cap` (*capayati*), and 40 with
  `10.0325 ci` (*cayati*, …).
- `10.0277 BU` shares 72 forms with the ādhṛṣīya `10.0382 BU` (*prāptau*,
  *bhāvayati*).
- No other new form is shared with an existing row.

**Negative witnesses become positive,** as 10i's *daṃśayati* did:
- `cayayati` is in 10i's "derives nothing" list, as the guṇa shape `10.0325
  ci` must not take. It is now √ci's (`10.0124`) declined ṇic form, so it
  leaves that list and the 10j witnesses pin it as √ci's alone.
- `jYApayati` and `jYApayate` are in 10d's list of 7.2.116-only shapes √jñap
  must not take. They are now √jñā's (`10.0258`), so they leave that list too.
- `capayati` and `capayate` gain √ci's analysis beside √cap's, so 10d's
  witnesses count two, √cap's first.

`trace_for` returns the first analysis; the plan pins which row that is
wherever a test relies on it, and greps the goldens for every witness form
before asserting an analysis count.

## Forms

Every row's laṭ prathama eka is in the Scope table. √ci's three forms per
cell are, in branch order: 6.1.54 declined (*cayayati*), taken
(*capayati*), and 2570's ṇic-less branch (*cayati*). `Nic` rows' ātmanepada
cells are 1.3.74's; √ci's ṇic-less ātmanepada (*cayate*) is 1.3.72's.

## Evidence

The prototype (throwaway branch `proto-10j`, not kept) curated the eight
rows as specified here. The one difference was 6.1.54's guard, which
started on the text `ci` and was corrected to the row number.

- **Whole corpus**, through the committed `tools/audit/` harness with only
  its totals raised: **430 roots, 26424 cells, 37070 forms**, zero
  differences (from 422 / 25956 / 36416).
- **Negative controls:**
  - `PANINI_AUDIT_PERTURB=entry`: 36 differing cells, as recorded.
  - 6.1.54 keyed on the text `ci`: 72 differing cells, all `10.0325`.
- **Census:** ALTERNATES rows 10460 → 10646. Blocked branches stay 6516:
  the ātmanepadī rows admit only ātmanepada, and √ci admits both padas on
  every branch. Root × pada × lakāra blocks 2884 → 2936.
- **Forms per cell:** ones 18268 → 18648, twos 6424 → 6432, threes 525 →
  601, sixes 363 → 365, nines 6 → 8; fours, fives and sevens unchanged.
- **Pre-existing roots:** all 42932 branches on main, blocked ones
  included, dumped with their traces and compared with the prototype's:
  byte-identical.
- **Where the new rules fire:** 6.1.54 only on `10.0124`; 7.3.36 only on
  `10.0124` and `10.0258`; 2567 only on `10.0058`.
- **Credits that move on new rows only:**
  - 7.2.115: + `10.0058`, `10.0124`, `10.0152`, `10.0231`, `10.0235`,
    `10.0275`, `10.0277`
  - sanādi 6.1.78: + `10.0058`, `10.0124`, `10.0235`, `10.0275`,
    `10.0277`
  - 6.4.92 and 10.0493: + `10.0124`
  - 2570: + `10.0124`
  - 1.3.74: + 5 rows (`10.0124`, `10.0152`, `10.0258`, `10.0275`,
    `10.0277`); 266 → 271 rows in the dump
  - 10.0496: + `10.0231`, `10.0235`
  - 1.3.12: + `10.0058`

  Rosters that enumerate credits gain them.

## Testing

**Goldens:** paradigm and alternates rows for all 8 roots, generated from
the engine by a harness that first asserts every cell's form set equals
vidyut's. Review packages leave out the appended goldens; diffs use
`--diff-algorithm=histogram`.

**Census (`derivation_set_shape_matches_the_audited_numbers`, and the
audit harness's asserts):**
- **Totals:** 430 roots / 26424 cells / 37070 forms; 2936 blocks.
- **Forms per cell:** as in Evidence.
- **ALTERNATES:** 10646. The plan copies the per-vikalpa-combination counts
  from the prototype's census and re-derives them at final HEAD.
- **Other counts:** `curated_roots_have_expected_ganas_and_padas` goes to
  430; the 1.3.74 curated-row count gains 5;
  `pada_ambiguous_surfaces_are_exactly_these` is re-derived from the
  goldens.

**Rule pins:**
- The rule-order pin (`tinanta_rule_order_is_pinned`) gains 6.1.54, 7.3.36
  and 2567, and records 6.4.92's and 10.0493's new positions.
- The vikalpa list (`VIKALPA_RULES`, `exactly_the_pinned_vikalpa_rules_are_optional`)
  gains 6.1.54.
- `the_optional_nic_ids_are_credited_only_on_their_rows`: 2570 gains
  `10.0124`.

**Data tests (`panini-data`):**
- `optional_nic_matches_upadesha_markers`: `10.0124` enters the table, and
  the comment at its exclusion (`… but 10.0124 ciY`) goes.
- `curated_pada_agrees_with_upadesha_markers`: admits a curādi `Atmanepada`
  row only where the upadeśa is ṅit (Decisions), asserted both ways like
  √bhuj's exception.
- `jnapadi_is_exactly_the_rows_10_0493_names`: its "`ciY` is not curated"
  comment goes; a derivation now holds the range's upper end too.
- The 6.1.54 key is pinned to the upadeśa `ciY` upstream.

**Unit tests, each guard in both directions:**
- 7.3.36 appends `p` to an ā-final aṅga before ṇit ṇic; declines on a
  non-ā-final aṅga, with no ṇic, and with a ṇic that is not yet ṇit.
- 6.1.54 fires on `10.0124` before ṇit ṇic; declines on `10.0325` (same
  text) and with no ṇic.
- 6.4.92 at its new position shortens `cAy` and `cAp`, and still shortens
  7.2.116's `jYAp`.
- 2567 fires on a curādi ātmanepadī ṇijanta in ātmanepada; declines in
  parasmaipada, without `Tag::Nijanta`, outside curādi, and on an
  ākusmīya row (`Tag::Akusmiya`, no `Tag::Atmanepadin`).
- 7.2.115 declines on `jYAp` (after puk).

**Corpus-wide tests (fires-only-on-rows):**
- **New:** 6.1.54, 7.3.36 and 2567 fire on exactly the rows Evidence lists.
- **Rosters that grow:** 7.2.115 and the sanādi 6.1.78
  (`the_10h_vrddhi_and_nuk_rules_fire_only_on_their_rows`), 6.4.92 and
  10.0493.

Goldens ignore traces, so the plan also diffs every pre-slice root's trace
main ↔ HEAD and requires no change.

**Witnesses (`check()`):** derived from this spec's tables, not hand-picked:
- **Per row:** each row's laṭ prathama eka in every pada it admits.
- **Per √ci branch:** *cayayati*, *capayati*, *cayati*, and their
  ātmanepada counterparts.
- **Per blocked pada:** parasmaipada fails for `smi`, `gf` and `yu`.
- **Per homograph** above, with the counts the goldens grep confirms.

**Mutation gate:**
- Scoped with `--in-diff` to the slice's production diff.
- Re-measure the floor and the full uncaught run at 26424 cells, at the
  parallelism the campaign will use, before setting `--timeout`. `10.0124`
  adds an `OPTIONAL_NIC` row with a further vikalpa (6.1.54) on its ṇic
  branch, so also re-measure the `skip_nic -> true` mutant alone.
- Run with `-o` to a dedicated directory and copy `outcomes.json` durably.
- Chunk with `--iterate` so no background shell outlives about 60 minutes.
- AGENTS.md names every missed or timeout mutant verbatim.

**Audit:** the committed `tools/audit/` harness, copied, with its totals
raised; negative control first. vidyut's dev-dependencies are repointed
at the implementation worktree for the run (they point at the prototype
now) and restored after.

## Documentation

- **AGENTS.md:**
  - the curādi progress line: "at 327 after slice 10j curated the eight
    ajanta rows"
  - the corpus counts (422 / 25956 / 36416 → 430 / 26424 / 37070)
  - the pada census
  - the fork census (threes and nines gain √ci's cells)
  - the audit record and the mutation record
- **`docs/ARCHITECTURE.md`:**
  - the rule-order pin count
  - the sanādi stage description (6.1.54, 7.3.36, the moved 6.4.92 and
    10.0493)
  - the samjna stage's pada sūtra list (2567)
  - the fork counts and census paragraphs
- **`tools/audit/README.md`:** the corpus totals and the recorded result.
- **`panini-data`:**
  - the `JNAPADI` doc ("`10.0124 ciY`, which is mit but not yet curated")
  - the `OPTIONAL_NIC` doc, if it enumerates rows
- **`tinanta/sanadi.rs`:** the module doc's rule list and order, and 6.4.92's
  comment.
- **`tinanta/samjna.rs`:** the module doc's rule list.
- **Earlier specs:** the "Later slices" lists of 10b to 10i gain a pointer
  to this spec where they name √gṛ, √yu, √smiṅ or √ci.
- **Sweep greps:**
  - root-shape literals for every new code (`smi`, `ci`, `Gf`, `gf`, `yu`,
    `jYA`, `cyu`, `BU`)
  - all of `crates/`, including tests
  - every enumeration of sibling rules: the sanādi rule list, the pada
    sūtra list, "6.4.92", "7.2.115"
  - "not yet curated", "must revisit", "later slice" phrasings that name
    these rows
  - `grep -i` for spelled-out counts ("319 curādi", "422", "fifty-nine")
  - wrapped counts
  - Re-derive every count from the test census at final HEAD.

## Later slices

- The plain obligatory-ṇic hal-anta rows: about 165 of them, across
  `10.0006`–`10.0191` and `10.0237`–`10.0278`, plus stragglers
  (`10.0397`, `10.0414`, `10.0438`, `10.0457`, `10.0462`, `10.0470`,
  `10.0491`). Prototype-audit them first: no spec has examined them, and
  each batch may hide rules.
- Upasargas, and with them `10.0368 za\da~` (7.3.78) and 6.1.76 *padāntād
  vā*.
- `gupU~`, `paRa~\` and `pana~\` join `AYA` when curated.
- A causative (hetumaṇic) slice takes 01.0934 and 10.0494, and with it
  7.3.36's named roots and 6.1.54 on `sPura~`.
