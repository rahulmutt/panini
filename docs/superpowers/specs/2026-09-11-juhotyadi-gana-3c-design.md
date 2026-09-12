# Juhotyādi gaṇa (gaṇa 3), slice 3c — √dā, √dhā, √mā, √hā (ātmanepada), and row identity reaches the pipeline

Slice 3b left the gaṇa at four of its twenty-six rows. 3a's spec pre-recorded
this slice's bundle:

> | 3c | √dā, √dhā, √mā, √hā ×2, √gā | 288 | 1.1.20 *dādhā ghv adāp* (saṁjñā),
> 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*, 8.2.38 *dadhas tathoś ca*, 6.4.116 /
> 6.4.117 (vikalpa) / 6.4.118 for √hā, 7.4.76 *bhṛñām it*, 7.4.78, 6.1.88
> *vṛddhir eci*; 6.4.112 / 6.4.113's abhyasta arms |

Re-probed at the audited commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` over
all 288 cells, and — new for this slice — with **this engine at `63b7b85` run
over the same cells on hand-built rows** and diffed cell by cell against vidyut,
the sūtra list holds but the table misjudges the work in four places:

- **Root text cannot identify these roots.** 7.4.76 fires on `03.0008 o~hA\N`
  (*jihIte*) and not on `03.0009 o~hA\k` (*jahAti*); both strip to `hA`. 1.1.20
  names six dhātupāṭha rows, and `02.0054 dA\p` — not ghu — strips to `dA` too.
  Every root-keyed guard in the engine reads `ANGA.text`; none could express
  either condition, and a text-keyed 7.4.76 would pass this slice and fail the
  next one silently. **The dhātupāṭha number has to reach the pipeline.**
- **8.2.40 *jhaṣas tathor dho'dhaḥ* needs its *adhaḥ* exclusion**, which the
  table does not mention. Without it √dhā's *DattaH* surfaces as \**DaDDaH*.
- **6.4.112 cannot simply gain an arm.** The sūtra carries no *aci*; the
  engine's vowel-initial test is a stand-in for "6.4.113 takes the
  consonant-initial cases". *aghoḥ* removes 6.4.113 for ghu roots, so 6.4.112
  must elide before consonants too (*dattaH*, *dadyAt*). 6.4.113 moves above it
  as its apavāda.
- **8.2.38 runs after 8.4.54, out of sūtra order**, and **6.1.101's adādi arm
  must decline on an āṭ + ec ending** so *dadE* takes the traditional
  6.1.90 → 6.1.88 path.

The table's 288 cells are also split: this slice takes the four rows that share
the ghu/abhyasta core (216 cells); √hā (parasmaipada) and √gā, which carry every
vikalpa and the chāndasa question, are a follow-up slice, **3c2**, recorded
below and decided nothing for. The 3d–3f labels are unchanged.

## Scope

**Rows (4):** `03.0010` √dā and `03.0011` √dhā (ubhayapada), `03.0007` √mā and
`03.0008` √hā (ātmanepada). **216 cells, 228 forms.** Suite 3636 → **3852**
cells, 4595 → **4823** forms, `ALTERNATES` 959 → **971** rows, roots 81 →
**85**, root × pada × lakāra blocks 404 → 428.

**New plumbing (2):** `Context.dhatupatha`; `Tag::Ghu` with its six-number
`GHU` set.

**New rules (4):** 7.4.76 *bhṛñām it* (`abhyasa.rs`); 6.4.119 *ghvasor
eddhāv abhyāsalopaś ca* (`guna.rs`); 6.1.88 *vṛddhir eci* (`adesha.rs`); 8.2.38
*dadhas tathoś ca* (`tripadi.rs`). 1.1.20 is a saṁjñā and lands as the tag, with
no recorded step — the 6.1.4 / 6.1.5 precedent.

**Restructured or widened (4):** 6.4.113 moves above 6.4.112 and gains the
*abhyasta* arm and *aghoḥ*; 6.4.112 gains the *abhyasta* arm and drops its
vowel-initial test; 6.1.101's adādi arm declines on `A` + ec; 8.2.40 gains
*adhaḥ*.

**Existing machinery, confirmed by the HEAD diff rather than assumed:** 8.4.53
(*DadDve*), 8.4.54's `D` and `J` arms, `hrasva_of`'s `A`, 3.4.109 (whose text
rewrite keeps 1.2.4's ṅit on *jus*), 7.1.4, 7.1.5, 6.1.66, 6.1.90's athematic
arm, 6.1.96's ending arm (*dadyuH*), 8.3.59 (*mimIze*), 8.4.55, the 7.1.35 and
8.4.56 forks, and the data layer: `strip_anubandhas` gives `dA`, `DA`, `mA`,
`hA`, and `pada_from_upadesha` gives ubhaya / ubhaya / ātmane / ātmane.

**No new vikalpa.** The engine stays at nine optional rules. No new fork kind
and no new branch record: eight multi-form cells, three forms at most.

**Recorded, not built:** 6.4.119's √as arm (no curated √as); 7.4.76's √bhṛñ
arm (`03.0006`, slice 3d).

**Out of scope:** slice 3c2 (below); 3d–3f; curādi; multi-ekāc reduplication;
sense conditions; 6.1.64 / 6.1.65; the 7.1.35 tātaṅ and 8.4.56 *vāvasāne*
repo-wide conventions.

## Root selection

| row | entry | root | pada | sanction |
|---|---|---|---|---|
| 03.0010 | qudA\Y | dA | ubhaya | 1.3.72 (ñit; the ḍu is 1.3.5's) |
| 03.0011 | quDA\Y | DA | ubhaya | 1.3.72 (ñit; the ḍu is 1.3.5's) |
| 03.0007 | mA\N | mA | ātmane | 1.3.12 (ṅit) |
| 03.0008 | o~hA\N | hA | ātmane | 1.3.12 (ṅit; the `o~` is 1.3.2's) |

`curated_pada_agrees_with_upadesha_markers` and
`dhatupatha_numbers_resolve_upstream` cover all four without edits.

## Row identity: `Context.dhatupatha` and `Tag::Ghu`

`derive` receives the whole `&Dhatu` but passes on only `code`, `pada` and
`gana`. Two of this slice's conditions name dhātupāṭha rows that root text
cannot separate:

- **7.4.76 *bhṛñām it*** names √bhṛñ, √māṅ and √ohāṅ: `03.0006`, `03.0007`,
  `03.0008`. `03.0009 o~hA\k` is also `hA` and must not match (*jahAti*).
- **1.1.20 *dādhā ghv adāp*** names the dā- and dhā-shaped roots except dāp and
  daip: `01.1079 dA\R`, `01.1117 de\N`, `01.1050 De\w`, `04.0043 do\`,
  `03.0010 qudA\Y`, `03.0011 quDA\Y`. `02.0054 dA\p` strips to `dA` and is
  excluded; `01.1073 dE\p` likewise.

The number is the repo's unique key already (`Dhatu::dhatupatha`'s doc comment:
"the unique key"; `code` is "never a lookup key"). So:

- **`Context` gains `dhatupatha: &'static str`**, set by `derive` from the row.
  Hand-built test prakriyās default it to `""`, which every number-keyed guard
  declines — the safe default.
- **`Tag::Ghu`** is added in `derive`'s aṅga tagging, beside the gaṇa tags, when
  the row's number is in **`GHU`**, a six-element set in `samjna.rs` with 1.1.20's
  text in its comment. Ghu is a saṁjñā — a class 1.1.20 names once and two rules
  read (6.4.113's *aghoḥ* and 6.4.119) — which is what earns it a tag; a
  single-root condition does not.
- **Single-root conditions match numbers in their own guards** — 7.4.76 on
  `03.0007 | 03.0008`, 8.2.38 and 8.2.40 on `03.0011` — each with a comment
  naming the upadeśa, because a bare number in a guard is opaque.

This is not a grammar branch in `derive`: the tag is a saṁjñā verdict feeding
guarded rules, exactly as the gaṇa tags are.

`GHU`'s four uncurated numbers have no derivation witness. They are held by a
test instead (see Testing), which pins the set exactly, so dropping one is
caught.

## The rules, in pipeline order

### `abhyasa.rs`

- **7.4.76 *bhṛñām it*** — after 7.4.62. When `ctx.dhatupatha` is `03.0007` or
  `03.0008`, the abhyāsa's vowel becomes `i`. vidyut's order is 7.4.59 → 7.4.62
  → 7.4.76 (*hA* → *ha* → *Ja* → *Ji*), and the engine's 7.4.60 → 7.4.59 →
  7.4.62 prefix already matches it. The comment names `03.0006 quBf\Y` as the
  arm 3d adds. *mimIte*, *jihIte*.

### `guna.rs`, at the end of the stage

The three rules keep the stage-end position 6.4.112 / 6.4.113 already have
(after 7.1.3 / 7.1.4 / 7.1.5 make the plural endings vowel-initial, after 7.2.79
strips yāsuṭ's and sīyuṭ's `s`, before `adesha.rs`'s coalescence rules), in
this order:

- **6.4.119 *ghvasor eddhāv abhyāsalopaś ca*** — new. When the aṅga carries
  `Ghu`, ends in `A`, and the ending is `hi`: the `A` becomes `e` and `ABHYASA`
  is emptied. *dehi*, *Dehi*. It must precede 6.4.112, whose elision would
  otherwise leave *dad* + *hi* for 6.4.101 to make \**daddhi*. The tātaṅ branch
  (7.1.35, forked in the tiṅ stage) presents `tAt`, so 6.4.119 declines there
  and 6.4.112 gives *dattAt*. 6.4.101 later sees `e` before *hi* and declines,
  unedited. The √as arm is recorded in the comment as unwitnessed.
- **6.4.113 *ī halyaghoḥ*** — moved above 6.4.112, its apavāda. The śnā arm is
  unchanged. The **abhyasta arm**: the aṅga carries `Abhyasta`, ends in `A`,
  and does **not** carry `Ghu`; the follower, read through
  `following_sarvadhatuka` (6.4.115's reasoning: under ślu the śap is empty, and
  in vidhiliṅ yāsuṭ sits on the ending's text with 3.4.103's `Ngit`), is ṅit
  and consonant-initial. The `A` becomes `I`. *mimIte*, *jihIvahe*,
  *mimItAm*. The comment that defers *aghoḥ* to this slice is replaced by the
  implementation.
- **6.4.112 *śnābhyastayor ātaḥ*** — two changes.
  - The śnā arm **drops its vowel-initial test.** With 6.4.113 above it, a
    consonant-initial kṅit ending always meets śnā as `nI`, and the `SHAP ==
    "nA"` test declines; the vowel test decides nothing and would be an
    unkillable mutant.
  - The **abhyasta arm**: the aṅga carries `Abhyasta` and ends in `A`, the
    follower is ṅit, and its initial is not tested. The `A` is elided. Ghu roots
    reach it before consonants (*dattaH*, *dadyAt*, *datse*), every abhyasta
    root before vowels (*dadati*, *mimate*, *dadIta*, *adaduH*).
  - This moves seven cells per ghu root that HEAD already gets right by
    another rule onto the rule vidyut credits: the six ātmanepada dvivacana
    cells of laṭ, laṅ and loṭ (*dadAte*, *adadAtAm*, …) from 6.1.101, and
    *adaduH* from 6.1.96's junction arm. 6.1.96's comment — 3.4.111 is its junction arm's sole
    witness — therefore stays true.

Swapping 6.4.113 above 6.4.112 moves no existing trace: no kryādi cell fires
both.

### `adesha.rs`

- **6.1.101's adādi arm** declines when the ending is `A` followed by an ec.
  vidyut, and the Kaumudī's *dadai*, merge the āṭ with its own ending first
  (6.1.90) and then the root's `A` with the result (6.1.88); HEAD's adādi arm
  takes `dA` + `AE` first instead. With the decline, 6.1.90's existing athematic
  arm sees `AE` and makes `E`. No prior is affected: √yā and √vā are
  parasmaipada only and never meet an āṭ + ec ending. **This decline is
  form-neutral** — without it, 6.1.101 then 6.1.88 still spell *dadE* — so only
  the *dadE* trace pin holds it.
- **6.1.88 *vṛddhir eci*** — new, immediately after 6.1.90. When the aṅga ends
  in `A`, `SHAP` is empty and the ending begins with an ec: the aṅga loses its
  `A` and the ending's initial becomes its vṛddhi. *dadE*, *daDE*, *mimE*,
  *jihE*. Disjoint from 6.1.90's thematic ending arm, which reads `SHAP`. The
  thematic and kryādi `A` + ec junctions keep their 6.1.90 attribution: their
  `A` is the 6.1.101 ekādeśa that contains the āṭ.

### `tripadi.rs`

- **8.2.40 gains *adhaḥ*.** It declines when `ctx.dhatupatha == "03.0011"`.
  √dhā's only jhaṣ before `t`/`T` in any of its cells is its own final `D`, so a
  narrower "the jhaṣ belongs to the aṅga" clause could never be falsified and is
  not written; the comment says so. The comment's claim that "no root in the
  suite besides √indh ever presents a jhaṣ immediately before its ending" is
  false once 6.4.112 elides √dhā's ā, and is corrected in the same commit.
- **8.2.38 *dadhas tathoś ca*** — new, **between 8.4.54 and 8.4.55**. When
  `ctx.dhatupatha == "03.0011"`, the aṅga is a single consonant (6.4.112 has
  elided the ā; 8.4.53 may already have made it `d`), and the following sound is
  `t`, `T` or `s`, or the following two are `Dv`: the abhyāsa's initial `d`
  becomes `D`. The root's own `D` is left to 8.4.53 (before `Dv`) and 8.4.55
  (before `t`/`s`), so the credited ids and their order match vidyut's —
  8.4.54 < 8.2.38 < 8.4.55 on *DattaH* and *Datse*, 8.4.53 < 8.4.54 < 8.2.38 on
  *DadDve*. It declines on *daDAti* (the ā survives before a pit ending) and on
  *daDIDvam* (a vowel follows).

  **The position is the point.** The sūtra names *dadh* — the reduplicated stem
  after 8.4.54's jaśtva has deaspirated the abhyāsa — so it must see that
  output. Placed in sūtra order it sees `Da`, finds nothing to do, and 8.4.54
  then produces \**dattaH* / \**dadDve*. The comment states this; the *DattaH*
  and *DadDve* pins hold it. ARCHITECTURE's rule-order notes gain the entry.

### Pinned rule order

`tinanta_rule_order_is_pinned` gains 7.4.76, 6.4.119, 6.1.88 and 8.2.38 and swaps
6.4.113 above 6.4.112. `exactly_the_pinned_vikalpa_rules_are_optional` is
unchanged.

## Counts

- √dā and √dhā: 72 cells, 78 forms each. Four multi-form cells each: laṅ
  prathama eka (8.4.56: *adadAd* / *adadAt*), loṭ prathama eka and madhyama eka
  (7.1.35 then 8.4.56: *dadAtu* / *dattAd* / *dattAt*, *dehi* / *dattAd* /
  *dattAt*), vidhiliṅ prathama eka (*dadyAd* / *dadyAt*): six `ALTERNATES` rows
  per root.
- √mā and √hā (ātmanepada): 36 cells, 36 forms each; no alternates.
- `PARADIGM` gains 24 blocks (8 + 8 + 4 + 4); `ALTERNATES` gains 12 rows.
- Index 0 follows the standing convention — the declined derivation: `adadAd`,
  `dadAtu`, `dehi`, `dadyAd`.

## Data

- Four `Dhatu` rows, keyed by number, in `panini-data`.
- Goldens in `tests/paradigm/data/juhotyadi.rs` and `tests/trace/juhotyadi.rs`.
- `juhotyadi_rows_are_the_four_curated_roots` becomes
  `juhotyadi_rows_are_the_eight_curated_roots`; its comment says the gaṇa stands
  at eight of twenty-six and that 3c2 and 3d–3f close it.
- The throwaway probes — `juhotyadi_3c_probe.rs`, `juhotyadi_3c_trace.rs`,
  `juhotyadi_3c_trace2.rs` and `juhotyadi_3c_head_diff.rs` under
  `/tmp/vidyut-full/vidyut-prakriya/examples` — stay uncommitted; their output
  is the appendix.

## Testing

**Per-rule guard tests** beside each rule, on hand-built `Prakriya`s:

- 7.4.76 fires for `03.0007` and `03.0008`, and **declines for a hand-built
  `03.0009` whose aṅga text is the same `hA`** — the test that numbers, not
  text, decide.
- 6.4.119 fires for a `Ghu` aṅga before `hi`; declines before `tAt`, and on an
  `A`-final abhyasta aṅga without `Ghu`.
- 6.4.113's abhyasta arm fires; declines on `Ghu`, on a vowel-initial follower,
  and on a pit follower. The existing śnā guard tests keep passing.
- 6.4.112's abhyasta arm fires before a consonant- and a vowel-initial kṅit
  follower; declines before a pit follower. The śnā arm declines on `nI`.
- 6.1.101's adādi arm declines on `AE`, and still fires on `Ani`.
- 6.1.88 fires on `A` + `E` with an empty `SHAP`; declines with a non-empty
  `SHAP`.
- 8.2.40 declines for `03.0011` and still fires on the √indh shape.
- 8.2.38 fires before `t`, `T`, `s` and `Dv`; declines while the aṅga still ends
  in `A`, before a vowel, and for any row other than `03.0011`.
- `GHU`: a test reads the vendored `data/dhatupatha.tsv` (by `include_str!`,
  test-only, the `panini-data` precedent) and asserts the six numbers name
  exactly `dA\R`, `de\N`, `De\w`, `do\`, `qudA\Y` and `quDA\Y`, and that
  `02.0054` and `01.1073` are not in the set. A `derive` test asserts a
  hand-built `02.0054` row with code `dA` gets no `Tag::Ghu`, and `03.0010`
  does.

**Trace pins (9)** in `tests/trace/juhotyadi.rs`, each holding one decision:

| cell | asserts |
|---|---|
| *mimIte* | 7.4.59 < 7.4.76 < 6.4.113; no 6.4.112 |
| *dehi* | 6.4.119; no 6.4.112, no 6.4.101 |
| *dattaH* | 6.4.112; no 6.4.113 (*aghoḥ*) |
| *DattaH* | 8.4.54 < 8.2.38 < 8.4.55; no 8.2.40 (*adhaḥ*) |
| *DadDve* | 8.4.53 < 8.4.54 < 8.2.38 |
| *daDAti* | no 8.2.38 |
| *dadE* | 6.1.90 < 6.1.88; **no 6.1.101** |
| *adaduH* | 3.4.109 < 6.4.112; no 6.1.96 |
| *dadAte* | 6.4.112; no 6.1.101 |

**Suite and audit.**

- 3852 cells VALID; the 3636 priors byte-identical, which guards the 6.4.112 /
  6.4.113 swap, the vowel-test removal, 6.1.101's decline and 8.2.40's
  exclusion.
- `derivation_set_shape_matches_the_audited_numbers` moves to 3852 cells / 85
  roots / 4823 forms; the audit's asserted totals move with it.
- The audit copies the committed `tools/audit` harness — never rewritten — with
  `/tmp/vidyut-full`'s dev-deps repointed at the slice's worktree, and must show
  zero divergence.
- `mise run test-full` (the exhaustive roundtrip) passes once before the gate.

**Mutation gate.**

- Re-measure a full UNCAUGHT run at `-j 4` first. Keep `--timeout 600` only if
  the margin over it still holds; otherwise raise it in AGENTS.md and
  `mise.toml` together.
- Campaign at `-j 4`, `--package panini-prakriya --test-workspace=true`, `-o` to
  an isolated scratch directory, launched detached as the N² slice's was.
- Copy `outcomes.json` somewhere durable before any follow-up invocation.
  AGENTS.md's dated entry names every missed and timed-out mutant verbatim, and
  diffs the non-caught set against the N² slice's record.

**Doc sweep.** README, ARCHITECTURE and AGENTS to 3852 / 4823 / 85, 971
`ALTERNATES` rows, 428 blocks, and juhotyādi at eight of twenty-six rows. The
grep includes the root-shape literals `dA`, `DA`, `mA`, `hA` alongside the
English phrasings, every file a behaviour task touched, the 8.2.40, 6.4.112 and
6.4.113 comments this slice falsifies, and 3a's "Later slices" table.

## Risks

1. **Two decisions are form-neutral.** 6.1.101's decline and 8.2.38's position
   can each be broken without changing a surface form the audit compares. The
   *dadE*, *DattaH* and *DadDve* pins are the only guard, and they run in the
   blocking tier.
2. **Numbers in guards are opaque**; a mistyped number compiles. Every number a
   guard matches has a firing guard test, and each guard's comment names its
   upadeśa.
3. **A new `Context` field.** Every hand-built prakriyā in the existing guard
   tests takes the `""` default, which declines every number-keyed guard; no
   existing test can start firing a new rule by accident.
4. **The vowel-test removal on 6.4.112's śnā arm** relies on 6.4.113 running
   first. Caught loudly if the order is wrong: kryādi's *kliSnItaH* would become
   \**kliSntaH* in the priors.
5. **Suite cost.** Four more roots grow `roundtrip_sampled`'s residual Θ(N²)
   term by roughly a tenth; hence a re-measured cap, not a projected one.
6. **Hand-derived counts.** 24 blocks, 12 rows and 228 forms are stated exactly
   so the generated goldens contradict them audibly.

## Success criteria

- 3852 cells VALID with the nine trace pins; the 3636 priors byte-identical.
- Audit zero-divergence at 85 / 3852 / 4823 against `8da2f90b`.
- `mise run test-full` passes.
- Gate clean at a re-measured cap: the non-caught set equals the documented
  equivalents plus the known permanent timeout.
- `Tag::Ghu` is decided by number and pinned to upstream; 7.4.76 declines on
  `03.0009`'s `hA`.
- README, ARCHITECTURE and AGENTS swept; juhotyādi at eight of its twenty-six
  rows.

## Later slices

After 3c the gaṇa stands at eight of twenty-six rows: 3c2 (√hā parasmaipada, √gā),
3d (the seven ṛ-final rows), 3e (√nij, √vij, √viṣ) and 3f (the six
ordinary-vowel rows) remain. 3a's table stands for 3d–3f.

### 3c2 — `03.0009 o~hA\k` and `03.0026 gA\`, recorded from this slice's probe

72 cells, 103 forms (61 + 42); suite 3852 → 3924 cells, 4823 → 4926 forms,
85 → 87 roots. 6.4.112 / 6.4.113's abhyasta arms and `Context.dhatupatha`
land in 3c. Its own work, from the probe:

- **6.4.116 *jahāteś ca*** — vikalpa, `i` for √hā's `A` before a consonant-
  initial kṅit: *jahItaH* / *jahitaH*. The engine's tenth optional rule.
- **6.4.117 *ā ca hau*** — vikalpa before *hi*: *jahAhi*. vidyut records it as
  a step that **changes no text** but blocks 6.4.113 and 6.4.116 on its branch;
  the engine's fork keeps a clone only when `apply` returns true, so 3c2 must
  decide how a text-neutral vikalpa marks its branch. Eleventh optional rule.
- **6.4.118 *lopo yi*** — √hā's `A` elided before `y`: *jahyAt*.
- **A seven-form cell**, loṭ madhyama eka: *jahAhi* / *jahIhi* / *jahihi* plus
  four tātaṅ forms. The engine's record is six.
- **7.4.78 *bahulaṁ chandasi*** for √gā's *jigAti*: a chāndasa rule vidyut
  applies on the Kaumudī's authority, and the repo has no precedent for one.
  3c2 decides whether to transcribe the attribution, record a stated
  convention, or defer √gā.
- **7.4.76 must decline on `03.0009`** — 3c's guard test already asserts it;
  3c2 supplies the derivation witness (*jahAti*, not \**jihAti*).

Re-probe before writing its spec; this record was taken a slice ahead.

## Appendix: vidyut's inventory at `8da2f90b`

### Sūtras credited on the four rows

| sūtra | 0010 | 0011 | 0007 | 0008 | status |
|---|---|---|---|---|---|
| 1.1.20 *dādhā ghv adāp* | ✓ | ✓ | — | — | **new**, as `Tag::Ghu` |
| 7.4.76 *bhṛñām it* | — | — | ✓ | ✓ | **new** |
| 6.4.119 *ghvasor eddhāv abhyāsalopaś ca* | ✓ | ✓ | — | — | **new** |
| 6.1.88 *vṛddhir eci* | ✓ | ✓ | ✓ | ✓ | **new** |
| 8.2.38 *dadhas tathoś ca* | — | ✓ | — | — | **new**, after 8.4.54 |
| 6.4.112 *śnābhyastayor ātaḥ* | ✓ | ✓ | ✓ | ✓ | **abhyasta arm**; vowel test dropped |
| 6.4.113 *ī halyaghoḥ* | — | — | ✓ | ✓ | **abhyasta arm** + *aghoḥ*; moved above 6.4.112 |
| 8.2.40 *jhaṣas tathor dho'dhaḥ* | — | — | — | — | not credited; **gains *adhaḥ*** so it stays uncredited on √dhā |
| 6.1.101 *akaḥ savarṇe dīrghaḥ* | ✓ | ✓ | ✓ | ✓ | existing; adādi arm **declines on āṭ + ec** |
| 8.4.53 *jhalāṁ jaś jhaśi* | — | ✓ | — | — | existing |
| 8.4.54 *abhyāse car ca* | — | ✓ | — | ✓ | existing |
| 8.4.55 *khari ca* | ✓ | ✓ | — | — | existing |
| 7.4.62 *kuhoś cuḥ* | — | — | — | ✓ | existing (3a) |
| 8.3.59 *ādeśapratyayayoḥ* | — | — | ✓ | ✓ | existing |
| 2.4.75, 6.1.10, 7.4.59 | ✓ | ✓ | ✓ | ✓ | existing (3a / 3b) |
| 7.1.4, 3.4.109 | ✓ | ✓ | — | — | existing (3a); parasmaipada only |
| 1.2.4, 3.1.68, 6.1.66, 6.1.90, 6.1.96, 6.4.71, 7.1.5, 7.1.35, 7.2.79, 8.2.39, 8.3.15, 8.4.56, and the 3.4.x ending machinery | ✓ | ✓ | ✓ | ✓ | existing, where each applies |
| 1.3.x, 1.4.13, 1.4.14, 3.2.123, 3.3.161, 3.3.162, 3.4.107, 3.4.113, 6.1.4, 6.1.5, 8.2.66, 8.4.37, 8.4.68 | ✓ | ✓ | ✓ | ✓ | engine convention (tags, it-lopa family, or not modelled) |

### HEAD's engine on the same cells

Hand-built rows through `Panini::derive` at `63b7b85`: **36 of 216 cells
already match**, all on √dā and √dhā (18 each). No √mā or √hā cell matches.
Seven of each ghu root's eighteen reach the right form by the wrong rule — the
six ātmanepada dvivacana cells of laṭ, laṅ and loṭ by 6.1.101, *adaduH* by
6.1.96 — the reason for the *dadAte* and *adaduH* pins.
The 180 failures resolve to the missing 6.4.112 / 6.4.113 abhyasta arms, 7.4.76,
6.4.119, 6.1.88 and 8.2.38.

### The traces that decide the slice

```
03.0010 dadE     ... 7.4.59 ["da","dA","","A","E"]  6.1.90 ["da","dA","","","E"]  6.1.88 ["da","d","","","E"]
03.0010 dehi     ... 7.4.59 ["da","dA","","hi"]     6.4.119 ["","de","","hi"]
03.0010 dattaH   ... 7.4.59 ["da","dA","","tas"]    6.4.112 ["da","d","","tas"] ... 8.4.55 ["da","t","","taH"]
03.0010 adaduH   ... 6.4.112 ["a","da","d","","us"]
03.0010 dadAte   ... 6.4.112 ["da","d","","Ate"]
03.0011 DattaH   ... 6.4.112 ["Da","D","","tas"] ... 8.4.54 ["da","D","","taH"]  8.2.38 ["Da","d","","taH"]  8.4.55 ["Da","t","","taH"]
03.0011 Datse    ... 6.4.112 ["Da","D","","se"]   8.4.54 ["da","D","","se"]   8.2.38 ["Da","d","","se"]   8.4.55 ["Da","t","","se"]
03.0011 DadDve   ... 6.4.112 ["Da","D","","Dve"]  8.4.53 ["Da","d","","Dve"]  8.4.54 ["da","d","","Dve"]  8.2.38 ["Da","d","","Dve"]
03.0007 mimIte   ... 7.4.59 ["ma","mA","","te"]   7.4.76 ["mi","mA","","te"]  6.4.113 ["mi","mI","","te"]
03.0007 mime     ... 7.4.76 ["mi","mA","","e"]    6.4.112 ["mi","m","","e"]
03.0008 jihIte   ... 7.4.59 ["ha","hA","","te"]   7.4.62 ["Ja","hA","","te"]  7.4.76 ["Ji","hA","","te"]  6.4.113 ["Ji","hI","","te"]  8.4.54 ["ji","hI","","te"]
```

vidyut's vidhiliṅ ātmanepada order (6.1.66 before 6.4.112 on *dadIta*) differs
from this engine's, which elides first; forms agree, and kryādi's *vfRIta* has
carried the same order since slice 9b.

### The paradigms

Forms within a cell are listed in sorted order, not branch order.

`03.0010 qudA\Y`, parasmaipada, 36 cells / 42 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | dadAti | dattaH | dadati | dadAsi | datTaH | datTa | dadAmi | dadvaH | dadmaH |
| laṅ | adadAd ⁄ adadAt | adattAm | adaduH | adadAH | adattam | adatta | adadAm | adadva | adadma |
| loṭ | dadAtu ⁄ dattAd ⁄ dattAt | dattAm | dadatu | dattAd ⁄ dattAt ⁄ dehi | dattam | datta | dadAni | dadAva | dadAma |
| vidhiliṅ | dadyAd ⁄ dadyAt | dadyAtAm | dadyuH | dadyAH | dadyAtam | dadyAta | dadyAm | dadyAva | dadyAma |

`03.0010 qudA\Y`, ātmanepada, 36 cells / 36 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | datte | dadAte | dadate | datse | dadATe | dadDve | dade | dadvahe | dadmahe |
| laṅ | adatta | adadAtAm | adadata | adatTAH | adadATAm | adadDvam | adadi | adadvahi | adadmahi |
| loṭ | dattAm | dadAtAm | dadatAm | datsva | dadATAm | dadDvam | dadE | dadAvahE | dadAmahE |
| vidhiliṅ | dadIta | dadIyAtAm | dadIran | dadITAH | dadIyATAm | dadIDvam | dadIya | dadIvahi | dadImahi |

`03.0011 quDA\Y`, parasmaipada, 36 cells / 42 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | daDAti | DattaH | daDati | daDAsi | DatTaH | DatTa | daDAmi | daDvaH | daDmaH |
| laṅ | adaDAd ⁄ adaDAt | aDattAm | adaDuH | adaDAH | aDattam | aDatta | adaDAm | adaDva | adaDma |
| loṭ | DattAd ⁄ DattAt ⁄ daDAtu | DattAm | daDatu | DattAd ⁄ DattAt ⁄ Dehi | Dattam | Datta | daDAni | daDAva | daDAma |
| vidhiliṅ | daDyAd ⁄ daDyAt | daDyAtAm | daDyuH | daDyAH | daDyAtam | daDyAta | daDyAm | daDyAva | daDyAma |

`03.0011 quDA\Y`, ātmanepada, 36 cells / 36 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | Datte | daDAte | daDate | Datse | daDATe | DadDve | daDe | daDvahe | daDmahe |
| laṅ | aDatta | adaDAtAm | adaData | aDatTAH | adaDATAm | aDadDvam | adaDi | adaDvahi | adaDmahi |
| loṭ | DattAm | daDAtAm | daDatAm | Datsva | daDATAm | DadDvam | daDE | daDAvahE | daDAmahE |
| vidhiliṅ | daDIta | daDIyAtAm | daDIran | daDITAH | daDIyATAm | daDIDvam | daDIya | daDIvahi | daDImahi |

`03.0007 mA\N`, ātmanepada, 36 cells / 36 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | mimIte | mimAte | mimate | mimIze | mimATe | mimIDve | mime | mimIvahe | mimImahe |
| laṅ | amimIta | amimAtAm | amimata | amimITAH | amimATAm | amimIDvam | amimi | amimIvahi | amimImahi |
| loṭ | mimItAm | mimAtAm | mimatAm | mimIzva | mimATAm | mimIDvam | mimE | mimAvahE | mimAmahE |
| vidhiliṅ | mimIta | mimIyAtAm | mimIran | mimITAH | mimIyATAm | mimIDvam | mimIya | mimIvahi | mimImahi |

`03.0008 o~hA\N`, ātmanepada, 36 cells / 36 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | jihIte | jihAte | jihate | jihIze | jihATe | jihIDve | jihe | jihIvahe | jihImahe |
| laṅ | ajihIta | ajihAtAm | ajihata | ajihITAH | ajihATAm | ajihIDvam | ajihi | ajihIvahi | ajihImahi |
| loṭ | jihItAm | jihAtAm | jihatAm | jihIzva | jihATAm | jihIDvam | jihE | jihAvahE | jihAmahE |
| vidhiliṅ | jihIta | jihIyAtAm | jihIran | jihITAH | jihIyATAm | jihIDvam | jihIya | jihIvahi | jihImahi |

### 3c2's paradigms

`03.0009 o~hA\k`, parasmaipada, 36 cells / 61 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | jahAti | jahItaH ⁄ jahitaH | jahati | jahAsi | jahITaH ⁄ jahiTaH | jahITa ⁄ jahiTa | jahAmi | jahIvaH ⁄ jahivaH | jahImaH ⁄ jahimaH |
| laṅ | ajahAd ⁄ ajahAt | ajahItAm ⁄ ajahitAm | ajahuH | ajahAH | ajahItam ⁄ ajahitam | ajahIta ⁄ ajahita | ajahAm | ajahIva ⁄ ajahiva | ajahIma ⁄ ajahima |
| loṭ | jahAtu ⁄ jahItAd ⁄ jahItAt ⁄ jahitAd ⁄ jahitAt | jahItAm ⁄ jahitAm | jahatu | jahAhi ⁄ jahIhi ⁄ jahItAd ⁄ jahItAt ⁄ jahihi ⁄ jahitAd ⁄ jahitAt | jahItam ⁄ jahitam | jahIta ⁄ jahita | jahAni | jahAva | jahAma |
| vidhiliṅ | jahyAd ⁄ jahyAt | jahyAtAm | jahyuH | jahyAH | jahyAtam | jahyAta | jahyAm | jahyAva | jahyAma |

`03.0026 gA\`, parasmaipada, 36 cells / 42 forms:

| lakāra | P.E | P.D | P.B | M.E | M.D | M.B | U.E | U.D | U.B |
|---|---|---|---|---|---|---|---|---|---|
| laṭ | jigAti | jigItaH | jigati | jigAsi | jigITaH | jigITa | jigAmi | jigIvaH | jigImaH |
| laṅ | ajigAd ⁄ ajigAt | ajigItAm | ajiguH | ajigAH | ajigItam | ajigIta | ajigAm | ajigIva | ajigIma |
| loṭ | jigAtu ⁄ jigItAd ⁄ jigItAt | jigItAm | jigatu | jigIhi ⁄ jigItAd ⁄ jigItAt | jigItam | jigIta | jigAni | jigAva | jigAma |
| vidhiliṅ | jigIyAd ⁄ jigIyAt | jigIyAtAm | jigIyuH | jigIyAH | jigIyAtam | jigIyAta | jigIyAm | jigIyAva | jigIyAma |
