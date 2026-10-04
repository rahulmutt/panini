# Curādi gaṇa (gaṇa 10), slice 10g — bulk optional ṇic: 2564 and 2570

## Summary

10f's "Later slices" list put bulk optional-ṇic rows next: the idit rows
(Kaumudī 2564) and the ñit/udit rows (2570). 10f built the mechanism
(`OPTIONAL_NIC`, the sanādi fork, `Dhatu::padas`), so this slice is mostly
curation. It curates 59 rows: every remaining 2564 row outside the
āsvadīya and ādhṛṣīya ranges, and every remaining 2570 row except
`10.0124 ciY`.

A throwaway prototype curated all 59 and audited them against vidyut
(Evidence). 58 matched in every cell. The 59th, `10.0112 kzapi~`, differed
in one cell because 8.4.2's intervener set lacks the anusvāra. This slice
widens that set by one character. It adds no rule ids and no tags.

## Scope

**In:** 59 rows, both padas, across laṭ / laṅ / loṭ / vidhiliṅ: 4248 cells.
Every row is `PadaAssignment::Nic`: on the ṇic branch both padas come from
1.3.74, as for `mUtra`; on the ṇic-less branch the row is parasmaipada by
1.3.78, and its ātmanepada cells are blocked.

Codes are `stored_form`'s output: the it-stripped upadeśa, with 7.1.58's
num stored as `n`.

**Kaumudī 2564 (idit), 53 rows:**

| row | upadeśa | code | row | upadeśa | code |
|---|---|---|---|---|---|
| `10.0002` | `citi~` | `cint` | `10.0105` | `ulaqi~` | `ulanq` |
| `10.0003` | `yatri~` | `yantr` | `10.0106` | `paqi~` | `panq` |
| `10.0004` | `sPuqi~` | `sPunq` | `10.0107` | `pasi~` | `pans` |
| `10.0005` | `sPuwi~` | `sPunw` | `10.0111` | `capi~` | `canp` |
| `10.0007` | `kudri~` | `kundr` | `10.0112` | `kzapi~` | `kzanp` |
| `10.0009` | `spuqi~` | `spunq` | `10.0113` | `kzaji~` | `kzanj` |
| `10.0011` | `midi~` | `mind` | `10.0114` | `Caji~` | `Canj` |
| `10.0013` | `o~laqi~` | `lanq` | `10.0130` | `cubi~` | `cunb` |
| `10.0014` | `olaqi~` | `olanq` | `10.0135` | `waki~` | `wank` |
| `10.0043` | `SvaWi~` | `SvanW` | `10.0147` | `SuWi~` | `SunW` |
| `10.0045` | `tuji~` | `tunj` | `10.0153` | `paci~` | `panc` |
| `10.0047` | `piji~` | `pinj` | `10.0157` | `kubi~` | `kunb` |
| `10.0048` | `laji~` | `lanj` | `10.0158` | `kuBi~` | `kunB` |
| `10.0049` | `luji~` | `lunj` | `10.0159` | `lubi~` | `lunb` |
| `10.0060` | `paTi~` | `panT` | `10.0160` | `tubi~` | `tunb` |
| `10.0062` | `Cadi~` | `Cand` | `10.0164` | `cuwi~` | `cunw` |
| `10.0066` | `Kaqi~` | `Kanq` | `10.0166` | `dahi~` | `danh` |
| `10.0067` | `kaqi~` | `kanq` | `10.0171` | `Capi~` | `Canp` |
| `10.0068` | `kuqi~` | `kunq` | `10.0182` | `jasi~` | `jans` |
| `10.0069` | `guqi~` | `gunq` | `10.0185` | `piqi~` | `pinq` |
| `10.0070` | `kuWi~` | `kunW` | `10.0241` | `jaBi~` | `janB` |
| `10.0071` | `guWi~` | `gunW` | `10.0254` | `tasi~` | `tans` |
| `10.0072` | `Kuqi~` | `Kunq` | `10.0267` | `ligi~` | `ling` |
| `10.0073` | `vawi~` | `vanw` | `10.0464` | `vawi~` | `vanw` |
| `10.0074` | `vaqi~` | `vanq` | `10.0465` | `laji~` | `lanj` |
| `10.0075` | `caqi~` | `canq` | | | |
| `10.0076` | `maqi~` | `manq` | | | |
| `10.0077` | `Baqi~` | `Banq` | | | |

**Kaumudī 2570 (udit), 6 rows:** `10.0174 SraRu~` (`SraR`), `10.0184 jasu~`
(`jas`), `10.0243 jasu~` (`jas`), `10.0249 divu~` (`div`), `10.0260 SfDu~`
(`SfD`), `10.0266 ancu~` (`anc`).

**Out:**

- `10.0124 ciY` (2570): its ṇic branch needs 7.2.115, as 10f recorded.
- 10.0498 ādhṛṣīya (51 rows, 338–388) and 10.0499 āsvadīya (59 rows,
  279–337). vidyut checks both antargaṇas before idit
  (`dhatu_karya.rs`), so idit and udit rows inside those ranges are not
  2564 or 2570 rows.
- The one-row triggers 2565 (`10.0022 pF`), 2571 and 2572.
- 7.1.58 as a credited step; the stored-num simplification stands.
- The causative, and everything on 10a's to 10f's out-of-scope lists.

### Decisions

**Every row is `Nic`, and no rule changes.** None of the 59 is in
`AKUSMIYA` or `AA_GARVIYA`, and none carries an ātmanepada marker. 10f's
fork handles them unchanged: the 2564 and 2570 rules already exist and
fire on whichever rows `OPTIONAL_NIC` lists.

**Four stored-code shapes are new, and `stored_form` decides them.** The
corpus had num before `q j c k g D d S s t`. This slice adds num before
`w W` (`sPunw`, `SvanW`, `kunW`, `cunw`), before `p b B` (`canp`, `kunb`,
`janB`) and before `h` (`danh`), and strips a leading `o~` marker
(`10.0013 o~laqi~` → `lanq`). No hand-written exception is needed: each
derives through 8.3.24 → 8.4.58, or stays as anusvāra before `s` and `h`,
and matches vidyut in every cell.

**8.4.2 counts the anusvāra as an intervener.** `is_natva_intervener`
(`crates/panini-prakriya/src/tinanta/sound.rs`) gains `'M'`. 8.4.2 names
*num*, a morpheme, but ṇatva runs in the tripādī over assembled text. By
then 8.3.24 has turned a stored num before a jhal into anusvāra, and
8.4.58 turns it into a pu-class `m` only after ṇatva has run. Within a
root in the covered grammar, an anusvāra can come only from num. The doc
comment is rewritten to state this textual approximation. It drops the
current claim that "num's nasal cannot occur in the intervening position…
Revisit when either enters scope", which this slice falsifies. The āṅ
half of the note stays.

Witness: `10.0112 kzanp`, parasmaipada loṭ uttama eka, {*kṣampāṇi*,
*kṣampayāṇi*}. Before the change we derived *kṣampāni*, *kṣampayāni*.
vidyut's trace runs 8.3.24 → 8.4.2 → 8.4.58; ours ran 8.3.24 → 8.4.58.

**`optional_nic_from_upadesha` stays marker-only, guarded by a range
assertion.** The test helper reads only markers, and inside 279–388
vidyut's antargaṇa checks would preempt the marker reading. Teaching the
helper the ranges is 10.0498's and 10.0499's slice's job, so this slice
adds a guard instead: `optional_nic_matches_upadesha_markers` asserts
that no `OPTIONAL_NIC` row falls in `10.0279..=10.0388`. The assertion
fails loudly when a bulk slice reaches those ranges.

**Uniqueness accounts for the optional-ṇic verdict.**
`dhatupatha_numbers_resolve_upstream` rejects `10.0174 SraRu~`: the
uncurated `10.0063 SraRa~` shares its gaṇa, its stored form `SraR` and
its artha `dAne`. The two rows are not interchangeable, though. The
engine reads `OPTIONAL_NIC` by number, and the markers put `SraRu~` there
(2570) and keep `SraRa~` out. The sibling filter therefore also requires
`optional_nic_from_upadesha(u)` to equal the curated row's own verdict.
A row is ambiguous only when a sibling agrees on gaṇa, stored form, artha
and optional-ṇic verdict. This stays true if `10.0063` is curated later.
Two rejected alternatives: an exemption list (unprincipled), and skipping
uncurated siblings (it breaks the day `10.0063` is curated).

**Homographs follow the `cah` / `rah` precedent** (`curadi` adanta
analysis test): a shared surface expects one analysis per row.

- `10.0249 divu~` reproduces every form of 10f's `10.0230 divu~`. `10.0230`
  is ākusmīya, so its ṇic branch is ātmanepada only; `10.0249` is `Nic` and
  adds parasmaipada *devayati*, which only it derives.
- `lanj` (`10.0048` / `10.0465`), `vanw` (`10.0073` / `10.0464`) and
  `jas` (`10.0184` / `10.0243`) share every form.
- `10.0014 olanq` and `10.0105 ulanq` share 28 laṅ forms: āṭ's vṛddhi
  gives `OlaRq-` for both.

## Forms

Representative cells, prototype-verified against vidyut:

| row | cell | forms |
|---|---|---|
| `10.0002 cint` | laṭ P 3sg | *cintati*, *cintayati* |
| `10.0002 cint` | laṭ Ā 3sg | *cintayate* (ṇic only) |
| `10.0112 kzanp` | loṭ P 1sg | *kṣampāṇi*, *kṣampayāṇi* |
| `10.0249 div` | laṭ P 3sg | *devati*, *devayati* (`10.0230` has *devati* only) |

Every parasmaipada cell holds both branches' forms. Every ātmanepada cell
holds the ṇic branch only.

## Evidence

The prototype (a throwaway worktree, not kept) curated all 59 rows as
specified here and ran the committed `tools/audit/` harness.

- **Negative control:** perturbing `entry` exits 1, with 36 √bhū DIFFs
  plus the one real difference.
- **Before the 8.4.2 change:** one difference, `10.0112` loṭ P 1sg.
- **After it:** zero differences across the whole corpus, prior rows
  included: **311 roots, 17964 cells, 22724 forms** (from 252 / 13716 /
  15644). The additions are 4248 = 59 × 72 cells and 7080 = 59 × 120 forms,
  the same per-root shape as `mUtra`.
- **Blocked branches:** +2124 = 59 × 36, the ṇic-less ātmanepada cells. No
  new row has an empty cell on either side.
- **Credits vidyut gives and we don't,** none of which changes a form:
  7.1.58 (the stored-num simplification), 7.3.59 on √kṣañj, and the usual
  bookkeeping ids.

## Testing

**Goldens:** paradigm and alternates rows for all 59 roots, generated by
the audit harness as in 10f.

**Census:** `derivation_set_shape_matches_the_audited_numbers` and the
harness's asserts move to 311 / 17964 / 22724. The fork and blocked-branch
census moves with them.

**Data tests (`panini-data`):**

- `optional_nic_matches_upadesha_markers`: 69 entries, plus the
  `10.0279..=10.0388` exclusion assertion (Decisions).
- `dhatupatha_numbers_resolve_upstream`: the verdict-aware sibling filter.
  A focused assertion pins that `10.0174` resolves uniquely and that
  `10.0063` stays a distinct row because its verdict is `None`.
- 1.3.74's curated-row count: 96 → 155.

**Trace rosters:** the tests that enumerate which roots credit 8.3.24,
8.4.40, 2564 and 2570 gain the new rows. 8.4.40's new credits are laṅ's
tuk-ścutva on the `C`-initial roots (`Cand`, `Canj`, `Canp`). The plan
derives each roster from this spec's row tables and the trace diff, and
does not hand-pick it.

**Witnesses (`curadi_analyses_its_optional_nic_forms`):**

- One ṇic-less and one ṇic witness per new code shape: `cint`, `sPunw`,
  `canp`, `danh`, `lanq`, `kzanp`, `SraR`, `SfD`, `anc`. Each asserts the
  trigger id first in a ṇic-less trace with 1.3.78 and no 3.1.25, and
  3.1.25 with no trigger id in a ṇic trace.
- The existing `devati` / `devayate` witnesses expect two analyses
  (`10.0230`, `10.0249`). A new `devayati` witness expects one, `10.0249`'s.
- One witness each for `lanj`, `vanw` and `jas` expects two analyses.
- One `olanq` / `ulanq` laṅ witness expects two analyses; a laṭ form of
  each expects one.
- Before asserting a count, grep the goldens for every witness form across
  all rows, not just the new ones.

**8.4.2:**

- A focused test pins `kzanp` loṭ P 1sg to {*kṣampāṇi*, *kṣampayāṇi*} and
  checks that 8.4.2 precedes 8.4.58 in both traces. This test kills the
  mutant that deletes `'M'`.
- A corpus-wide test asserts that 8.4.2 fires across an anusvāra only on
  `kzanp` cells. A trace diff main ↔ HEAD over the pre-slice corpus must
  show no change, since goldens ignore traces.

**Mutation gate:**

- Scoped to `is_natva_intervener` and every `panini-data` function this
  slice touches.
- Before setting `--timeout`, re-measure the floor at 17964 cells: a full
  uncaught suite run at the parallelism the campaign will use.
- Run with `-o` to a dedicated directory and copy `outcomes.json` durably.
- Chunk with `--iterate` so no background shell outlives about 60 minutes.
- AGENTS.md names every missed or timeout mutant verbatim.

## Documentation

- **AGENTS.md:** the corpus counts (252 / 13716 / 15644 → 311 / 17964 /
  22724), the pada census, the fork and blocked census, the audit record
  and the mutation record.
- **ARCHITECTURE:** the 7.1.35 / 8.4.56 fork counts and any census
  paragraph. The rule-order pin count does **not** change: 8.4.2 is
  already pinned and no ids are added. A sweep must not invent a change.
- **`tools/audit/README.md`:** the corpus totals it quotes.
- **`panini-data`:** the `OPTIONAL_NIC` doc, the `Dhatu::pada` census, and
  any phrasing that implies only ten optional-ṇic rows.
- **`tinanta/sound.rs`:** the 8.4.2 doc comment (Decisions).
- **Earlier specs:** 10f's "Later slices" bullet gains a pointer to this
  spec.
- **Sweep greps:**
  - root-shape literals for every new code
  - all of `crates/`, including tests
  - `grep -i` for spelled-out counts ("ten optional", "Ninety-six")
  - wrapped counts
  - Re-derive every count from the test census at final HEAD.

## Later slices

- 10.0498 ādhṛṣīya (51 rows), then 10.0499 āsvadīya (59 rows). Each
  teaches `optional_nic_from_upadesha` its range and drops this slice's
  exclusion assertion. (Slice 10h took fifty of the ādhṛṣīya, all but
  `10.0368 za\da~`: see `2026-10-03-curadi-gana-10h-design.md`. Slice 10i
  took the āsvadīya: see `2026-10-04-curadi-gana-10i-design.md`.)
- The one-row triggers 2565 (`pF`) and 2571 (`Guzi~r`), both taken by slice
  10i. 2572 (īdit) is unreachable: every īdit curādi row is āsvadīya or
  ādhṛṣīya, and vidyut checks both antargaṇas first.
- √ci (`10.0124`), √gṛ and √yu. 7.2.115 landed in slice 10h, so √gṛ and √yu
  are curation; √ci also needs 6.1.54 and 7.3.36.
- A causative (hetumaṇic) slice takes 01.0934 and 10.0494.
