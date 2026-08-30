# Tanādi gaṇa (gaṇa 8) — slices 8a and 8b

Eighth gaṇa, the u-vikaraṇa (3.1.79 *tanādikṛñbhya uḥ*). Two slices on the
5a/5b attribution precedent: **8a** lands the nine plain roots on machinery
the engine already carries for śnu, widened once; **8b** lands √kṛ and the
6.4.108–110 specials the svādi spec deliberately dodged. After 8b the gaṇa is
closed at all ten of its dhātupāṭha rows, and only juhotyādi (3) and curādi
(10) remain.

## Summary

Gaṇa 8 is the smallest gaṇa in the dhātupāṭha — ten rows, 08.0001–08.0010 —
and structurally the closest to a gaṇa the engine already has. Its vikaraṇa
is the bare `u`: like śnu it is u-final, takes guṇa to `o` before pit
sārvadhātuka endings (*tanoti* beside *śṛṇoti*), loses `hi` by 6.4.106
(*tanu* beside *hinu*), and forks optionally by 6.4.107 before m/v
(*tanvaḥ/tanuvaḥ* beside *hinvaḥ/hinuvaḥ*). The engine work is therefore one
new vikaraṇa rule plus the generalization of the śnu-keyed guards to any
u-final vikaraṇa — the approach chosen against duplicating parallel `"u"`
arms, which is the hardcoded-operative-term failure mode the adādi slices
paid for twice.

The pada column costs nothing: seven roots are svarita-it (1.3.72, already
the sanction for eleven curated roots), two are anudātta (1.3.12), and √kṛ
is ñit (1.3.72's other arm). No new pada machinery, only curated rows.

## Scope

In scope, slice 8a: nine roots × four lakāras (laṭ, laṅ, loṭ, vidhiliṅ),
both padas where sanctioned — seven ubhayapadī × 72 cells + two ātmanepadī
× 36 cells = **576 cells**, suite 2844 → 3420.

In scope, slice 8b: √kṛ, ubhayapadī, +72 cells → 3492, with 6.4.108,
6.4.109, 6.4.110.

Out of scope, deferred:

- **The other gaṇas** — juhotyādi and curādi, each its own future spec.
- **Sense conditions**, as ever: 1.3.72's *kartrabhiprāye kriyāphale* is not
  modelled; both padas derive and the reader selects by sense.
- **Sanādi and non-tiṅ uses of √kṛ** (ṇic, san, kṛt formations). 8b covers
  √kṛ the tanādi tiṅanta only.
- **6.1.64 *dhātvādeḥ ṣaḥ saḥ***, still. `zaRu~^` ships as `san` in the
  curated row, the exact convention √ṣṭigh set (`zwiGa~\` shipped as
  `stiG`).
- **7.1.35 tātaṅ and 8.4.56 vāvasāne** remain the pre-existing repo-wide
  conventions they were.

## Root selection

All ten rows of the gaṇa, from `data/dhatupatha.tsv` (already vendored):

| row | entry | root (SLP1) | pada | sanction |
|---|---|---|---|---|
| 08.0001 | tanu~^ | tan | ubhaya | 1.3.72 (svarita) |
| 08.0002 | zaRu~^ | san | ubhaya | 1.3.72; 6.1.64 convention |
| 08.0003 | kzaRu~^ | kzaR | ubhaya | 1.3.72 |
| 08.0004 | kziRu~^ | kziR | ubhaya | 1.3.72 |
| 08.0005 | fRu~^ | fR | ubhaya | 1.3.72 |
| 08.0006 | tfRu~^ | tfR | ubhaya | 1.3.72 |
| 08.0007 | GfRu~^ | GfR | ubhaya | 1.3.72 |
| 08.0008 | vanu~\ | van | ātmane | 1.3.12 |
| 08.0009 | manu~\ | man | ātmane | 1.3.12 |
| 08.0010 | qukf\Y | kf | ubhaya | 1.3.72 (ñit) — **slice 8b** |

Rows join the curated table keyed by dhātupāṭha number, the number-identity
slice's discipline. **Curation is audit-gated on the 7d precedent**: a root
vidyut-prakriya cannot arbitrate is dropped from the slice and the drop is
recorded here — that is a finding, not a failure. The table above is the
intent, not a promise; `PARADIGM` and `ALTERNATES` are transcribed from
audited output only, never from this document.

Root-shape notes, each a reason the audit run matters:

- **fR is the gaṇa's first vowel-initial athematic root in laṅ**: the āṭ
  augment path (6.4.72 *āḍ ajādīnām*, 6.1.90 *āṭaś ca*) exists in the
  engine; whether it produces *ārṇot*-shaped forms unchanged is for the
  audit to confirm.
- **tfR, GfR, kziR carry a laghu upadhā** (ṛ, ṛ, i). Whether 7.3.86
  *pugantalaghūpadhasya ca* gunas it — equivalently, how the bare `u`'s
  sārvadhātuka/ārdhadhātuka status resolves, a question the tradition
  itself litigates — is **settled by the audit, not by this spec**. See
  Risks.
- **kzaR, kziR, fR, tfR, GfR are ṇ-carrying already**; no new ṇatva work is
  expected, but the tripadi scan gets nine new paradigms of exercise.

## Grammar

### 3.1.79 *tanādikṛñbhya uḥ* — `vikarana.rs`

New rule. Apavāda to 3.1.68, ordered in the 3.1.69 / 3.1.73 / 3.1.77 /
3.1.78 block, identical in shape to 3.1.73: gaṇa test, vikaraṇa text `u`,
`Tag::Vikarana`. The sūtra's own text names √kṛ (*kṛñbhya*), so 8b adds no
vikaraṇa rule — the same 3.1.79 fires for it.

### The asaṁyogapūrva predicate widens — `terms.rs`

`shnu_asamyogapurva` currently reads `SHAP.text == "nu"`. It becomes "the
vikaraṇa's final sound is `u` and that `u` is asaṁyogapūrva" — for śnu the
question reduces to the aṅga-final vowel as today; for the bare `u` it is
the aṅga's final cluster that answers it (tan-u: `n` after a vowel →
asaṁyogapūrva). The helper's two contracts survive verbatim: callers need
no gaṇa test of their own, and every reader of the vikaraṇa text stays
ordered before 6.4.107 (whose fork truncates that text on one branch). The
rename should follow the widened meaning; the enumeration comment in
`terms.rs` that counts the open-coded copies of the test must be updated in
the same commit.

What this buys for free: 6.4.106 hi-luk (*tanu*, and *ṛṇu* if fR survives
the audit), 6.4.107 optional lopa with its alternates (*tanvaḥ/tanuvaḥ*),
and the conjunct-blocked negative space (none of the nine roots is
conjunct-final, so unlike svādi the block may go unwitnessed — the goldens
say, the audit decides).

### 7.3.84, existing guard — `guna.rs`

Guṇa of the aṅga-final ik before sārvadhātuka/ārdhadhātuka. The second
application that produced *śṛṇo-* from śnu should produce *tano-* from
tan-u on the existing guard, since the guard reads the aṅga's final sound,
not the vikaraṇa's name. If it turns out to be śnu-keyed anywhere, that is
the same widening as the predicate above and lands in the same commit.

### The u → v attribution — open, audit-arbitrated

*tanvanti / tanvate*: something must turn the vikaraṇa's `u` into `v`
before vowel-initial endings. For śnu that is 6.4.87 *huśnuvoḥ
sārvadhātuke* — which names hu and śnu and therefore does **not** cover the
bare `u` — and 6.1.77 *iko yaṇ aci* is not in the engine. Which sūtra
vidyut credits (6.1.77 new, a 6.4.87 reading this engine has not needed, or
something else) is the audit's to answer, and the trace transcribes its
answer. This is the 8.2.30 precedent: the first audit divergence is the
finding that drives the rule work.

### Slice 8b: the √kṛ specials — `adesha.rs` / `guna.rs`

- **6.4.110 *ata ut sārvadhātuke*** (kṅiti, by anuvṛtti): the aṅga's `a`
  (of *kar*) becomes `u` before ṅit sārvadhātuka — *kurutaḥ, kurvanti,
  kurute*.
- **6.4.108 *nityaṁ karoteḥ***: the lopa 6.4.107 makes optional is nitya
  for √kṛ before m/v — *kurvaḥ, kurmaḥ*, **no alternates**. The engine's
  first nitya override of a vikalpa rule; 6.4.107's own comment has named
  it since svādi. Implementation shape: 6.4.108 fires before 6.4.107 and
  leaves nothing for the vikalpa fork to do on √kṛ cells.
- **6.4.109 *ye ca***: the same lopa before y — *kuryāt* and the rest of
  vidhiliṅ parasmaipada.

Root guṇa *kar* is expected from 7.3.84's existing final-ik guard (kṛ is
ṛ-final). 7.1.100 *ṝta id dhātoḥ*, the other rule svādi's deferral named,
reads **long-ṝ** roots; √kṛ is short-ṛ and it should not fire — audit
confirms. 8b lands only after 8a's goldens are certified, so the
6.4.108 ↔ 6.4.107 interplay is reviewed against a stable base.

## Data

- Curated root rows for 08.0001–08.0009 (8a) and 08.0010 (8b), keyed by
  number, pada column as tabled above.
- Goldens in the split suite's directory form: `tests/paradigm/data/tanadi.rs`
  and `tests/trace/tanadi.rs`, wired into `main.rs` the way the seven
  existing gaṇa files are.
- The audit copies the committed `tools/audit` harness — never rewritten —
  with `/tmp/vidyut-full`'s dev-deps repointed at the working tree so the
  post-slice engine is what gets audited.

## Testing

- **Byte-identical prior goldens are the primary guard** on the predicate
  widening: all 2844 existing cells and their traces unchanged. Any svādi
  drift means the widened guard fired where `"nu"` alone used to.
- The pinned rule order gains 3.1.79 (8a), then 6.4.108/109/110 (8b).
- Per-rule guard tests in the slice-7 style for each new rule, plus a
  helper test extending `shnu_asamyogapurva_is_true_exactly_for_the_...`
  to the widened domain.
- The 6.4.107 forks add tanādi alternates to `ALTERNATES`; the README fork
  census and `pada_ambiguous_surfaces_are_exactly_these` will both move —
  the enumerated test is regenerated from audited output, and the doc
  sweep greps the vikalpa rule ids and the files no task owns, per the
  standing sweep discipline.
- **Mutation floor re-measured, not scaled**: 3420 cells (then 3492) is
  past every timing figure in AGENTS.md. Before any campaign, run a
  standalone `mise run test` and re-derive the uncaught floor and the
  `-j`/`--timeout` margin; update AGENTS.md's numbers in the slice that
  changes them.

## Risks

1. **The upadhā-guṇa open point (tfR, GfR, kziR).** If the audit resolves
   it as "7.3.86 applies," those roots need guṇa arms this spec has not
   budgeted, and the slice upgrades in place — stop, record the finding
   here, and re-present the affected section. If vidyut cannot derive them
   at all, the 7d drop-rule applies instead.
2. **The widening touches svādi's live path.** Mitigated by the
   byte-identical guard above and by landing the widening plus 3.1.79
   before any tanādi golden exists, so the suite proves 2844 → 2844 first.
3. **6.4.108 against the fork machinery (8b).** A nitya rule suppressing a
   vikalpa is new; the wrong shape would silently halve or double √kṛ's
   cells. The audit's √kṛ paradigm — 72 cells, alternates enumerated — is
   the arbiter, and the 8a/8b split keeps this review isolated.
4. **Attribution drift on the u → v rule.** Whatever sūtra the audit
   credits is transcribed as-is; do not pre-implement 6.1.77 on
   this spec's guess.

## Success criteria

- 8a: all 3420 cells VALID with pinned traces; 2844 prior cells and traces
  byte-identical; audit reports zero divergence on the surviving root set;
  mutation gate clean (`missed.txt` empty, `timeout.txt` holding only the
  known permanent ṇatva-scan entry) at a re-measured cap.
- 8b: 3492 cells, same bars; √kṛ's m/v cells carry no 6.4.107 alternates.
- README, ARCHITECTURE and AGENTS.md updated by the sweep discipline;
  gaṇa count "seven" → "eight", then the root/fork censuses after 8b.
