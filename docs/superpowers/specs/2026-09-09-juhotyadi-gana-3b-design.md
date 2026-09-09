# Juhotyādi gaṇa (gaṇa 3), slice 3b — √bhī and √hrī, and the abhyāsa's own shape

Slice 3a landed dvitva and the abhyāsa core on √hu and √ki, and left the gaṇa
at two of its twenty-six rows. Its spec pre-recorded this slice's bundle:

> | 3b | √bhī, √hrī | 72 | 7.4.59 *hrasvaḥ*, 7.4.60 *halādiḥ śeṣaḥ*, 6.4.115
> *bhiyo'nyatarasyām* (vikalpa), 6.4.77's dhātu arm; 8.4.54 widened to `B` |

That table was written a slice ahead, from a probe of the whole gaṇa rather
than of these two rows. Re-probed at the audited commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` over all 72 cells, with full sūtra
traces diffed against this engine's implemented set, it is **wrong in both
directions**: it names one edit that is not needed and misses two that are.

- **8.4.54 needs no widening.** Its substitute is `deaspirate_of`, which has
  carried `B -> b` since it was written, and whose doc comment already names
  √bhī as the arm's customer. The slice supplies the witness, not the code.
- **6.4.82 must widen to a long `I`**, which the table does not mention.
  *bibhyati* is 6.4.82's, and the aṅga's vowel is still long when it fires:
  7.4.59 shortened the abhyāsa, not the root.
- **`pada_from_upadesha` mis-assigns √bhī**, which the table does not
  mention either. Its `Yi` clause has never been reached by a curated root,
  and √bhī is the first one to reach it.

The rest of the bundle holds. This spec is the corrected inventory.

## Scope

**New rules (3):** 7.4.60 *halādiḥ śeṣaḥ* and 7.4.59 *hrasvaḥ* in
`abhyasa.rs`; 6.4.115 *bhiyo'nyatarasyām* in `guna.rs`, the engine's ninth
vikalpa.

**New arm on an existing rule (1):** 6.4.77's iyaṅ arm, in `guna.rs`.

**Widened rules (1):** 6.4.82 *er anekāco'saṁyogapūrvasya*, from a final `i`
to a final `i` or `I`.

**Corrected data-layer verdicts (2):** the `bare.starts_with("Yi")` disjunct
in `panini-data`'s `pada_from_upadesha` is deleted, and the branch-order
claim that deletion makes vacuous is replaced by a disjointness invariant.

**New sound-table entries (0).** `deaspirate_of` already carries `B -> b`;
`is_natva_intervener` already carries `y` and every vowel.

**New data:** two `Dhatu` rows; 8 `PARADIGM` blocks (2 roots × 1 pada × 4
lakāras); 40 `ALTERNATES` rows; five trace pins.

**Changed:** `juhotyadi_rows_are_the_two_curated_roots` renamed and extended;
the corpus totals and the cell-fork distribution in `panini-data`,
`crates/panini/tests/paradigm/main.rs`, `tools/audit/`, `README.md`,
`AGENTS.md` and `docs/ARCHITECTURE.md`.

**Out of scope**, unchanged from 3a's list: curādi; multi-ekāc reduplication
(both roots are a single ekāc when 6.1.10 fires); sense conditions; 6.1.64 /
6.1.65; the 7.1.35 tātaṅ and 8.4.56 *vāvasāne* repo-wide conventions. The
uvaṅ half of 6.4.77 is deferred with them — see "6.4.77's iyaṅ arm" below.

## Root selection

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0002 | `YiBI\` | `BI` | parasmai | 1.3.78 *śeṣāt kartari parasmaipadam* |
| 03.0003 | `hrI\` | `hrI` | parasmai | 1.3.78 |

Both are parasmaipada-only, so each contributes 36 cells rather than 72.

Neither upadeśa carries an accent on an it: √hrī's `\` sits on the root
vowel and √bhī's does too, which is the distinction
`pada_from_upadesha`'s doc comment already argues at length. √hrī therefore
reaches 1.3.78 with nothing to say about it. √bhī does not, and that is this
slice's one data-layer finding.

### The `Yi` clause is wrong, and √bhī is the first root that can say so

`pada_from_upadesha` computes

```
let nyit = final_it == Some('Y') || bare.starts_with("Yi");
```

on the reasoning, written into the comment above it, that 1.3.5 *ādir
ñiṭuḍavaḥ* makes an initial `Yi` an it and that the it so supplied is a
**ñ**-it, which 1.3.72 *svaritañitaḥ* then reads. The first half is right and
the second is not.

The clause has never decided anything. Of the 79 curated roots exactly one is
`Yi`-initial — `07.0011 YiinDI~\`, √indh — and it is caught by the earlier
`anudatta_it` branch, which the comment itself flags as load-bearing and
which `indh_is_atmanepada_despite_satisfying_1_3_72` pins. So the disjunct
has sat unfalsified since it was written, and √bhī is the first root whose
verdict it decides. It decides it wrongly: `YiBI\` has no `~\` and no final
hal, so `nyit` alone carries it to `Ubhayapada`, and
`curated_pada_agrees_with_upadesha_markers` then fails against the curated
`Parasmaipada` this slice needs.

The evidence that parasmaipada is right is not one root's say-so. The
dhātupāṭha has **fourteen** `Yi`-initial rows, and vidyut-prakriya fires
1.3.72 on **none** of them:

| row | entry | pada derived | 1.3.72 |
|---|---|---|---|
| 01.0594 | `YiPalA~` | Parasmaipada *phalati* | no |
| 01.0844 | `YimidA~\` | Ātmanepada *medate* | no |
| 01.0845 | `YizvidA~\` | Ātmanepada *svedate* | no |
| 01.0846 | `YikzvidA~\` | Ātmanepada *kṣvedate* | no |
| 01.0884 | `YitvarA~\` | Ātmanepada *tvarate* | no |
| 01.1133 | `YizvidA~` | Parasmaipada *svedati* | no |
| 02.0063 | `Yizva\pa~` | Parasmaipada *svapiti* | no |
| 03.0002 | `YiBI\` | Parasmaipada *bibheti* | no |
| 04.0127 | `YizvidA~` | Parasmaipada *svidyati* | no |
| 04.0141 | `YitfzA~` | Parasmaipada *tṛṣyati* | no |
| 04.0158 | `YimidA~` | Parasmaipada *medyati* | no |
| 04.0159 | `YikzvidA~` | Parasmaipada *kṣvidyati* | no |
| 05.0025 | `YiDfzA~` | Parasmaipada *dhṛṣṇoti* | no |
| 07.0011 | `YiinDI~\` | Ātmanepada *indhe* | no |

Every ātmanepada verdict in that column comes from a `~\`, every
parasmaipada one from 1.3.78. The initial ñi decides no pada anywhere in the
dhātupāṭha. It is an it — 1.3.5 is real, and `strip_anubandhas` correctly
strips it, which is why the curated `code` is `BI` and not `YiBI` — but it is
not an anubandha 1.3.72 reads.

So the disjunct goes, the comment is rewritten to record this scan as the
reason rather than the reverse claim, and a test pins √bhī's parasmaipada
verdict from the upadeśa. `strip_anubandhas`' own `Yi` handling is untouched:
that one is right, and `BI` is what it produces.

Nothing else in the corpus moves, because nothing else ever reached the
clause. That is what makes this a one-commit correction at the head of the
slice rather than a slice of its own — but it is also exactly why it needs
the test: an unfalsifiable clause is how it survived this long.

### Deleting it makes a second claim vacuous, and that has to be paid for

`pada_from_upadesha` orders its two branches under a comment that says so in
capitals:

```
// ORDER IS LOAD-BEARING. 1.3.12 is tested first because `YiinDI~\`
// (√indh) satisfies both it and 1.3.72, and must come out ātmanepada.
// Pinned by `indh_is_atmanepada_despite_satisfying_1_3_72`.
```

√indh satisfies 1.3.72's branch **only through the `Yi` clause**. Remove the
clause and that sentence stops being true — and not merely for √indh. A scan
of all 2280 upstream rows for an upadeśa satisfying both branch conditions
(`~\` or a final `N` it, and `~^` or a final `Y` it) returns **zero rows**.
The two conditions are disjoint over the whole dhātupāṭha, so after this
slice the branch order is not load-bearing at all: swapping the two `if`
blocks changes no verdict anywhere, and cargo-mutants will report exactly
that as a survivor.

Leaving the capitals in place over a claim that has become false is the worse
of the two failures this slice is trying to fix, so the correction is paid
for in full:

- The comment is rewritten to record what is now true — the conditions are
  disjoint across every upstream row; the order is retained because 1.3.12
  is the apavāda by tradition, which is a statement about grammar rather
  than about a live test.
- `indh_is_atmanepada_despite_satisfying_1_3_72` is renamed
  `indh_is_atmanepada_by_its_anudatta_it` and its comment records that the
  ñi it carries decides nothing, so the assertion is about 1.3.12's own
  reading rather than about precedence.
- **A disjointness test is added** over all upstream rows, asserting that no
  upadeśa satisfies both branch conditions. This is what keeps the ordering
  falsifiable: it converts a claim no cell can test into an invariant the
  data must uphold, and if upstream ever grows such a row the precedence
  question returns loudly instead of silently. It is the same tripwire idiom
  as the `code`-uniqueness assertion in
  `juhotyadi_rows_are_the_two_curated_roots`.

Without the third bullet this slice would trade one unfalsifiable clause for
another, which is not a fix.

## The rules, in pipeline order

### `abhyasa.rs` — the abhyāsa gets its own shape

3a's `abhyasa.rs` holds 6.1.10 and 7.4.62 and nothing between them, because
√hu's and √ki's abhyāsas need no reshaping: both roots are a single short
vowel behind a single consonant. Both of this slice's roots break one of
those assumptions, and the two rules that repair them are the 7.4.5x pair the
Kaumudī orders before 7.4.62.

- **7.4.60 *halādiḥ śeṣaḥ*** — of the abhyāsa's initial consonant cluster
  only the first consonant remains. `hrI` -> `hI`. **√hrī is the only
  consonant-cluster-initial row in the entire gaṇa** — `hu`, `BI`, `dA`,
  `DA`, `mA`, `hA`, `gA`, `Bf`, `pf`, `pF`, `Gf`, `hf`, `sf`, `f`, `nij`,
  `vij`, `viz`, `Bas`, `kit`, `tur`, `Diz`, `Dan`, `jan`, `ki` all begin with
  a single consonant or a vowel — so this rule will have exactly one witness
  when the gaṇa is closed, and no later slice will add another. That is
  worth stating rather than discovering in 3f: the rule is not
  under-witnessed pending a future slice; one witness is all it will ever
  have, and the guard test carries the load a second root would otherwise
  share.
- **7.4.59 *hrasvaḥ*** — the abhyāsa's vowel becomes hrasva. `BI` -> `Bi`,
  and `hI` -> `hi` on 7.4.60's output.

Both take a **no-op guard**, the 8.4.53 idiom the slice already uses at
8.4.54: computed result equal to input means return `false` and record
nothing. Without it every √hu and √ki trace gains a step and the 3564
byte-identical priors break loudly — which is the point of stating the bar in
those terms.

Order is **7.4.60 < 7.4.59 < 7.4.62**, matching vidyut's. For √hrī the forms
agree under either order of the first two (`hrI` -> `hI` -> `hi` and `hrI` ->
`hri` -> `hi`), so the order is a trace decision, not a form decision, and
following the source the audit compares against costs nothing.

This is where 3a's divergence stays visible. vidyut copies the **guṇated**
stem and shortens it back (`hre` -> `he` -> `hi`); this engine copies the bare
root and shortens that (`hrI` -> `hI` -> `hi`). Both reach `hi`, and 7.4.59
fires in both — but in vidyut it is repairing a guṇa it just made, and here it
is doing the work the sūtra names. The module's header comment already
predicts this slice as the one where 7.4.59 "fires only on the long-vowel
roots it names"; that prediction is now witnessed and the comment can say so.

### `guna.rs` — 6.4.115, then the two ī-final dispositions

- **6.4.115 *bhiyo'nyatarasyām*** (vikalpa) — √bhī's `I` optionally becomes
  hrasva. `BI` -> `Bi`, on the **aṅga**, not the abhyāsa, which 7.4.59 has
  already shortened. The sūtra names the root, so the guard is root-keyed
  (`ANGA.text == "BI"`) on the 6.4.87 / 6.4.101 precedent 3a set — with no
  gaṇa clause beside it, for the reason 3a gives: such a clause could never
  be falsified.

  Its other two conditions come by anuvṛtti from 6.4.113 — *hali* and
  *kṅiti* — and **both are independently falsifiable on this slice's own
  cells**, which is why both are written rather than one standing in for the
  other:

  | drop this clause | wrong form it produces | cell that catches it |
  |---|---|---|
  | the kṅit test | `*biBizi`, `*biBimi` | laṭ madhyama/uttama eka: `si`, `mi` are hal-initial but **pit** |
  | the *hali* test | `*biBiyati` | laṭ prathama bahu: `ati` is kṅit but **ajādi** |

  The follower is `following_sarvadhatuka`, which returns `ENDING` here since
  ślu has emptied `SHAP`. That is correct for vidhiliṅ too: 3.4.103 prefixes
  yāsuṭ onto the ending's own text and tags that term `Ngit`, so the term the
  helper returns is `yAt` — hal-initial and ṅit — and all nine vidhiliṅ cells
  fork, as vidyut has them.

  **No cell forces its position** in `GUNA_RULES`: wherever guṇa fires the
  kṅit test declines, and where 6.4.82 fires first the `ANGA.text == "BI"`
  test declines against the `By` it left. It goes beside 6.4.113, the rule
  it inherits *hali* and *kṅiti* from and the place that anuvṛtti is already
  argued; the *bibhitaḥ* trace pin is what holds it there.

- **6.4.82 *er anekāco'saṁyogapūrvasya*** — widened from

  ```
  if anga.last() != Some(&'i') { return false; }
  ```

  to accept `I` as well. The sūtra's *eḥ* denotes both lengths by 1.1.69 /
  1.1.70 *aṇudit savarṇasya*, so this is the rule's stated scope, not an
  extension of it, and **both lengths are witnessed**: √ki's `ciki` + `ati`
  -> `cikyati` (3a, short) and √bhī's `BiBI` + `ati` -> `biByati` (this
  slice, long). Neither branch is a free mutation survivor.

  Its `asaṁyogapūrva` walk — open-coded over `ABHYASA` plus `ANGA`, which
  `vikarana_u_asamyogapurva`'s doc comment explicitly tells later slices not
  to unify — needs no change, and it is what routes the two roots apart:

  | aṅga pair | two sounds before the final vowel | verdict |
  |---|---|---|
  | `Bi` + `BI` | `B`, then the vowel `i` | asaṁyogapūrva — 6.4.82 fires |
  | `Ji` + `hrI` | `r`, then the consonant `h` | saṁyogapūrva — 6.4.82 declines |

- **6.4.77's iyaṅ arm** — what 6.4.82 declines, its utsarga takes:
  `hrI` + `ati` -> `hriy` + `ati`, *jihriyati*. The guard is aṅga-final `I`
  plus a vowel-initial follower, general over roots rather than keyed to
  √hrī, since 6.4.77 names no root.

  **The uvaṅ half is not written.** `U` -> `uv` has no cell in the suite —
  no juhotyādi row is ū-final — and an arm with no witness is a guaranteed
  mutation survivor. This is the narrowness the rule's own comment already
  practises for its śnu arm and 6.1.78 practises for its three: widen by arm,
  with a witness. The comment records the deferral and names the condition
  for lifting it.

  A general arm is safe here because of where 6.4.77 already sits — after
  7.3.84 and after its apavāda 6.4.82. Four curated roots end in ī/ū, and all
  four are past it by then:

  | root | why the arm cannot reach it |
  |---|---|
  | √bhū (`BU`) | 7.3.84 has guṇated it to `Bo` — śap is **pit**, so 1.1.5 does not block |
  | √nī (`nI`) | likewise `ne` |
  | √śī (`SI`) | 7.4.21 guṇates it before every sārvadhātuka — `Se` |
  | √vrī (`vrI`) | its follower is śnā, hal-initial |

  The 3564 byte-identical priors are what turn that table from an argument
  into a proof.

### Unchanged, but newly witnessed

- **8.4.54 *abhyāse car ca*** — `deaspirate_of('B') == Some('b')` already.
  `biBeti` -> `bibheti` is the arm's first live cell; the sound table's
  comment predicting it becomes a statement of fact.
- **8.4.2 *aṭkupvāṅnumvyavāye'pi*** — *jihrayāṇi*: the `r` of `hray`, then
  `a`, `y`, `A`, then `ni`. All three interveners are already in
  `is_natva_intervener` (every vowel, plus `y`), and vidyut credits 8.4.2 on
  exactly this cell. The engine's ṇatva reaches its first abhyāsa-bearing
  word with no edit.
- **7.4.62 *kuhoś cuḥ*** — √hrī's `h` -> `J` is the same arm √hu exercises.
  √bhī's `B` is neither ku nor `h`, so 7.4.62 declines there and 8.4.54 alone
  does the work — the minimal contrast between the two roots' abhyāsas.
- **3.4.109, 2.4.75, 6.1.10, 7.1.4, 7.3.83, 6.1.78, 6.1.96, 8.3.59, 8.2.66,
  8.3.15, 8.2.39, 8.4.68** and the tātaṅ / 8.4.56 forks — 3a's machinery on
  3a's guards.

## Counts

72 new cells, **112 new forms**, 40 new `ALTERNATES` rows.

| | before | after |
|---|---|---|
| roots | 79 | **81** |
| cells | 3564 | **3636** |
| forms | 4483 | **4595** |
| `ALTERNATES` rows | 919 | **959** |

Per root, from the probe:

| row | cells | forms | of which 6.4.115's |
|---|---|---|---|
| 03.0002 `YiBI\` | 36 | 70 | 28 |
| 03.0003 `hrI\` | 36 | 42 | — |

√bhī's 70 forms fall out of 42 — √hrī's count exactly — plus the 28 that
6.4.115 forks off. The two roots are otherwise the same paradigm, which is
the arithmetic check that the vikalpa is the whole of the difference between
them.

### The fork distribution

The cell-fork buckets `paradigm/main.rs` and `README.md` both enumerate:

| forms per cell | before | 3b adds | after |
|---|---|---|---|
| one | 2882 | 43 | **2925** |
| two | 526 | 24 | **550** |
| three | 115 | 2 | **117** |
| four | 17 | 1 | **18** |
| five | 8 | 1 | **9** |
| six | 16 | 1 | **17** |

**√bhī's loṭ parasmaipada madhyama eka joins the six-form record** —
`biBIhi` / `biBihi` / `biBItAd` / `biBitAd` / `biBItAt` / `biBitAt` — and it
does so by a **third distinct k = 3 stack** against the same 2³ bound of
eight. The record's sixteen existing cells reach six two ways: rudhādi's
7.1.35 / 8.4.65 / 8.4.56, and tanādi's 7.1.35 / 7.3.86 / 8.4.56. This one is
7.1.35 / **6.4.115** / 8.4.56. The record moves to seventeen cells and three
mechanisms, and `docs/ARCHITECTURE.md`'s branch-count discussion is where
that belongs.

√bhī's loṭ prathama eka joins the five-form bucket (`biBetu` beside the four
tātaṅ readings) and its vidhiliṅ prathama eka the four-form bucket
(6.4.115 × 8.4.56); √hrī's two loṭ eka cells join the three-form bucket on
the standing tātaṅ fork alone.

### The `ALTERNATES` keys

Three existing keys gain four rows each — `8.4.56`, `7.1.35` and
`7.1.35+8.4.56`, from the two roots' laṅ/vidhiliṅ prathama eka and loṭ eka
cells — and 6.4.115 opens **four new keys**:

| key | rows | where |
|---|---|---|
| `6.4.115` | 23 | laṭ 5, laṅ 5, loṭ 4, vidhiliṅ 9 |
| `7.1.35+6.4.115` | 2 | loṭ prathama eka, loṭ madhyama eka |
| `7.1.35+6.4.115+8.4.56` | 2 | the same two cells |
| `6.4.115+8.4.56` | 1 | vidhiliṅ prathama eka |

23 + 2 + 2 + 1 = 28 new-key rows, plus 12 on existing keys, is the 40.

**Every number in this section is projected from the vidyut probe and must be
re-derived from the generated goldens during implementation, not trusted from
here.** They are stated precisely so that a mismatch is a loud failure rather
than a silent drift; a slice that finds different numbers has found a real
disagreement and should stop.

## Data

- Curated rows for `03.0002` and `03.0003`, keyed by number, `pada`
  `Parasmaipada` for both.
- Goldens appended to `crates/panini/tests/paradigm/data/juhotyadi.rs` and
  `crates/panini/tests/trace/juhotyadi.rs` — the files 3a created, already
  wired into both `main.rs` files.
- The audit copies the committed `tools/audit` harness — never rewritten —
  with `/tmp/vidyut-full`'s dev-deps repointed at **this** worktree. They
  currently point at the deleted `juhotyadi-3a` worktree; repointing them is
  the first audit step, or the harness silently audits whatever tree the path
  resolves to.
- The throwaway probes behind this spec (`juhotyadi_3b_probe.rs`,
  `juhotyadi_3b_trace.rs`, `juhotyadi_3b_count.rs`, `yi_pada_scan.rs` under
  `/tmp/vidyut-full/vidyut-prakriya/examples`) stay uncommitted; their output
  is this document's tables.

## Testing

- **The 3564 priors byte-identical.** This is the slice's primary gate, and
  it is doing real work here rather than being ceremony: it is the proof
  behind 6.4.82's widening, 6.4.77's general arm and both no-op guards.
- Per-rule guard tests for 7.4.60, 7.4.59, 6.4.115 and 6.4.77's iyaṅ arm,
  in the slice-7 style — including the two negative cases 6.4.115's table
  above names, since a guard test that only shows the rule firing pins half
  of it.
- Three `panini-data` tests: √bhī parasmaipada re-derived from `YiBI\`; the
  renamed `indh_is_atmanepada_by_its_anudatta_it`; and the branch-disjointness
  invariant over all upstream rows. The last is what keeps the branch order
  falsifiable once the `Yi` clause is gone — see "Deleting it makes a second
  claim vacuous".
- `juhotyadi_rows_are_the_two_curated_roots` -> `..._four_curated_roots`,
  extended.
- The pinned rule order gains 7.4.60, 7.4.59 and 6.4.115.
- **Trace pins (5):**
  - *jihreti* — orders 7.4.60 < 7.4.59 < 7.4.62 < 8.4.54.
  - *jihriyati* — credits 6.4.77 and **not** 6.4.82.
  - *bibhyati* — credits 6.4.82 on a long `I`, and carries **no** 6.4.115
    (the *hali* clause's witness).
  - *bibhitaḥ* beside *bibhītaḥ* — 6.4.115's fork, with 7.4.59 on the
    abhyāsa in both arms.
  - *jihrayāṇi* — credits 8.4.2 across `a`, `y`, `A`.
- Audit at zero divergence on both roots;
  `derivation_set_shape_matches_the_audited_numbers` and the audit's asserted
  totals move to 3636 / 4595 / 81 together.
- Mutation gate through `mise run mutants` (`-j 4 --timeout 4800`), with the
  uncontended floor **re-measured at 3636 cells, not scaled from 3564** —
  AGENTS.md's own instruction, and the one the 1800-cell measurement showed
  matters. `timeout.txt` checked alongside `missed.txt` for anything beyond
  the known permanent `tripadi.rs` ṇatva-scan entry. The campaign is chunked
  with `--iterate` against the 60-minute background-shell ceiling, and every
  invocation passes `-o` so a probe run cannot rotate a finished campaign's
  `outcomes.json` away.

## Risks

1. **6.4.82's widening reaches a root it should not.** Four curated roots end
   in ī/ū and the table above argues all four are past it. The argument is
   only as good as the priors that test it — which is why the byte-identical
   bar is stated as the gate rather than as a formality.
2. **The deleted `Yi` clause.** It moves no golden, so its removal is
   invisible to every existing test — the same property that let it be wrong
   for this long. The new √bhī pin is the only thing standing between it and
   a future re-introduction; the comment must therefore carry the fourteen-row
   scan, not just the verdict.
3. **A missing no-op guard on 7.4.59 or 7.4.60.** Silently rewrites √hu's and
   √ki's traces. Caught loudly by the priors; listed here because "caught
   loudly" is a claim about a gate that has to actually run.
4. **The projected counts.** Forty `ALTERNATES` rows across four new keys and
   a six-form cell are a lot of hand-derived arithmetic. They are stated to
   the row so that the generated goldens contradict them audibly.
5. **Suite cost.** The standing per-slice gate, now against 3636 cells. If a
   pre-campaign projection exceeds 4800s the cap is raised in AGENTS.md and
   `mise.toml` together, never silently — 3a's rule, unchanged.

## Success criteria

- 3636 cells VALID with the five pinned traces; the 3564 priors byte-identical.
- Audit zero-divergence at 3636 / 4595 / 81 against `8da2f90b`.
- Gate clean: `missed.txt` empty, `timeout.txt` holding only the known
  permanent entry, at a cap re-measured against a 3636-cell uncaught run.
- √bhī's pada re-derives from `YiBI\` as parasmaipada, and the fourteen-row
  scan is in the comment that says why.
- README, ARCHITECTURE and AGENTS carry 3636 / 4595 / 81, the six-form
  record at seventeen cells and three mechanisms, and juhotyādi at four of
  its twenty-six rows.

## Later slices

3a's table stands for 3c–3f, less the two corrections this slice makes to its
own row. Two of those corrections generalise and should be read before the
next spec is written:

- **A rule the table calls "widened" may already be general.** 8.4.54 was.
  Grep the substitute before costing the edit.
- **A rule the table does not mention may still need widening.** 6.4.82 did.
  The table was built from a gaṇa-wide rule-set diff, which shows *which*
  sūtras fire, not *which arms* of this engine's rules they land in.

After 3b the gaṇa stands at four of twenty-six rows: 3c (√dā, √dhā, √mā,
√hā ×2, √gā), 3d (the seven ṛ-final rows), 3e (√nij, √vij, √viṣ) and 3f (the
six ordinary-vowel rows) remain.

## Appendix: vidyut's inventory at `8da2f90b`

### Sūtras credited, by root

Fifty-seven sūtras over the 72 cells. Those **not** in this engine's
implemented set, or not on a guard that reaches these roots, are the slice's
work; every other one is existing machinery.

| sūtra | 03.0002 | 03.0003 | status |
|---|---|---|---|
| 7.4.60 *halādiḥ śeṣaḥ* | — | ✓ | **new** |
| 7.4.59 *hrasvaḥ* | ✓ | ✓ | **new** |
| 6.4.115 *bhiyo'nyatarasyām* | ✓ | — | **new**, vikalpa |
| 6.4.77 *aci śnudhātubhruvāṁ …* | — | ✓ | **new arm** (iyaṅ) |
| 6.4.82 *er anekāco'saṁyogapūrvasya* | ✓ | — | **widened** (`I`) |
| 1.3.5 *ādir ñiṭuḍavaḥ* | ✓ | — | data layer; see above |
| 7.4.62 *kuhoś cuḥ* | — | ✓ | existing (3a) |
| 8.4.54 *abhyāse car ca* | ✓ | ✓ | existing; `B` arm already written |
| 8.4.2 *aṭkupvāṅnumvyavāye'pi* | — | ✓ | existing |
| 2.4.75, 6.1.10, 6.1.4, 6.1.5, 7.1.4, 3.4.109, 7.3.83 | ✓ | ✓ | existing (3a) |
| 1.1.5, 1.2.4, 1.4.13, 1.4.14, 3.1.68, 6.1.78, 6.1.96, 6.1.101, 6.4.71, 7.1.35, 7.2.79, 7.3.84, 8.2.39, 8.2.66, 8.3.15, 8.3.59, 8.4.56, 8.4.68, and the 1.3.x / 3.2.x / 3.3.x / 3.4.x machinery | ✓ | ✓ | existing |

### The two paradigms

`03.0002 YiBI\`, parasmaipada, 36 cells / 70 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | biBeti | biBItaH ⁄ biBitaH | biByati | biBezi | biBITaH ⁄ biBiTaH | biBITa ⁄ biBiTa | biBemi | biBIvaH ⁄ biBivaH | biBImaH ⁄ biBimaH |
| laṅ | abiBet ⁄ abiBed | abiBItAm ⁄ abiBitAm | abiBayuH | abiBeH | abiBItam ⁄ abiBitam | abiBIta ⁄ abiBita | abiBayam | abiBIva ⁄ abiBiva | abiBIma ⁄ abiBima |
| loṭ | biBetu ⁄ biBItAt ⁄ biBItAd ⁄ biBitAt ⁄ biBitAd | biBItAm ⁄ biBitAm | biByatu | biBIhi ⁄ biBihi ⁄ biBItAt ⁄ biBItAd ⁄ biBitAt ⁄ biBitAd | biBItam ⁄ biBitam | biBIta ⁄ biBita | biBayAni | biBayAva | biBayAma |
| vidhiliṅ | biBIyAt ⁄ biBIyAd ⁄ biBiyAt ⁄ biBiyAd | biBIyAtAm ⁄ biBiyAtAm | biBIyuH ⁄ biBiyuH | biBIyAH ⁄ biBiyAH | biBIyAtam ⁄ biBiyAtam | biBIyAta ⁄ biBiyAta | biBIyAm ⁄ biBiyAm | biBIyAva ⁄ biBiyAva | biBIyAma ⁄ biBiyAma |

`03.0003 hrI\`, parasmaipada, 36 cells / 42 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | jihreti | jihrItaH | jihriyati | jihrezi | jihrITaH | jihrITa | jihremi | jihrIvaH | jihrImaH |
| laṅ | ajihret ⁄ ajihred | ajihrItAm | ajihrayuH | ajihreH | ajihrItam | ajihrIta | ajihrayam | ajihrIva | ajihrIma |
| loṭ | jihretu ⁄ jihrItAt ⁄ jihrItAd | jihrItAm | jihriyatu | jihrIhi ⁄ jihrItAt ⁄ jihrItAd | jihrItam | jihrIta | jihrayARi | jihrayAva | jihrayAma |
| vidhiliṅ | jihrIyAt ⁄ jihrIyAd | jihrIyAtAm | jihrIyuH | jihrIyAH | jihrIyAtam | jihrIyAta | jihrIyAm | jihrIyAva | jihrIyAma |

### The three traces that decide the slice

vidyut's own steps, with this engine's existing rules unmarked and the
slice's work marked. Terms are vidyut's four (abhyāsa, aṅga, śap, ending);
this engine's five-slot layout adds the empty `AGAMA` in front except where
6.4.71 fills it.

```
### 03.0002 laṭ prathama bahu => biByati        the 6.4.82 widening
   3.1.68    ["BI", "Sap", "Ji"]
   2.4.75    ["BI", "", "Ji"]          ślu
   6.1.10    ["BI", "BI", "", "Ji"]    dvitva
   7.1.4     ["BI", "BI", "", "ati"]   jhi -> ati
   7.4.59  * ["Bi", "BI", "", "ati"]   NEW — the abhyāsa only
   6.4.82  * ["Bi", "By", "", "ati"]   WIDENED — the aṅga's I is still long
   8.4.54    ["bi", "By", "", "ati"]   existing; B -> b

### 03.0002 laṭ prathama dvi => biBitaH         6.4.115's fork
   6.1.10    ["BI", "BI", "", "tas"]
   7.4.59  * ["Bi", "BI", "", "tas"]
   6.4.115 * ["Bi", "Bi", "", "tas"]   NEW, vikalpa — hal-initial kṅit `tas`
   8.4.54    ["bi", "Bi", "", "taH"]

### 03.0003 laṭ prathama bahu => jihriyati      7.4.60 and the iyaṅ arm
   6.1.10    ["hrI", "hrI", "", "Ji"]
   7.1.4     ["hrI", "hrI", "", "ati"]
   7.4.60  * ["hI",  "hrI", "", "ati"]  NEW — the cluster loses its r
   7.4.59  * ["hi",  "hrI", "", "ati"]  NEW
   7.4.62    ["Ji",  "hrI", "", "ati"]  existing (3a) — h -> J
   6.4.77  * ["Ji",  "hriy", "", "ati"] NEW ARM — 6.4.82 declined: r after h
   8.4.54    ["ji",  "hriy", "", "ati"] existing; J -> j
```
