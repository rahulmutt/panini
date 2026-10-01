# Cross-implementation audit

`panini_full_audit.rs` compares this engine's derivations against
[`vidyut-prakriya`](https://github.com/ambuda-org/vidyut), cell by cell, over the
whole curated corpus. Every gaṇa slice runs it.

It is a `vidyut-prakriya` **example**, not a member of this workspace — it depends
on both engines, and only one of them is ours. It lives here anyway because the
three slices before this one each rebuilt it from scratch, having no copy to start
from. Nothing under `tools/` is compiled by `cargo build`; the Cargo workspace is
`crates/*`.

## What it commits to

**Entry selection is by dhātupāṭha number and nothing else** —
`Dhatupatha::get(d.dhatupatha)`. Earlier audits searched upstream for whichever
entry reproduced *this engine's own pinned laṭ prathama eka form*, which made the
anchoring cell the one cell the audit could not independently validate. That
circularity is what keying on the number removes; do not reintroduce a fallback.

**It compares derivation sets, never a single form.** Optional (vikalpa) rules
fork cells legitimately. Comparing index 0 raises a false difference on √hiṃs laṅ
madhyama eka, where the sets agree but the two engines disagree about which branch
is ruleless.

**It filters blocked prakriyās** on this engine's side. `Panini::derive`'s doc
comment states that a blocked prakriyā's `text()` is a partial string — often the
bare root code — not a surface form.

**It asserts the corpus totals** (102 roots, 4608 cells, 5699 forms) rather than
reporting whatever it enumerated. Those totals are corroborated by
`derivation_set_shape_matches_the_audited_numbers` in
`crates/panini/tests/paradigm/main.rs`, which each slice raises to the same totals
alongside the golden rows that justify them — the two can be out of step
mid-slice, while that landing is in progress. Once both are current, if the
harness disagrees, the harness is wrong.

**A zero-difference result means nothing on its own.** Prove the harness can
detect a difference before believing one — see Negative controls below.

## Setup

Clone `vidyut` at the commit this repo's `data/dhatupatha.tsv` was vendored from.
That commit is recorded in the vendored file's own header; check it rather than
trusting this README:

```bash
head -20 data/dhatupatha.tsv | grep commit
```

```bash
cd /tmp && git clone --filter=blob:none https://github.com/ambuda-org/vidyut vidyut-full
cd vidyut-full && git checkout <the commit from that header>
```

Add this repo's crates as dev-dependencies of `vidyut-prakriya`, pointing at your
checkout — these are for the example only and are not upstream:

```toml
# /tmp/vidyut-full/vidyut-prakriya/Cargo.toml, under [dev-dependencies]
panini = { path = "/workspace/crates/panini" }
panini-data = { path = "/workspace/crates/panini-data" }
```

Then put the harness where Cargo will find it:

```bash
cp /workspace/tools/audit/panini_full_audit.rs \
   /tmp/vidyut-full/vidyut-prakriya/examples/
```

There is no `mise.toml` in the vidyut checkout, so `mise exec -- cargo` will not
resolve a toolchain there. Name it explicitly:

```bash
cd /tmp/vidyut-full/vidyut-prakriya
mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit
```

Both checkout locations are env-overridable, defaulting to `/tmp/vidyut-full` and
`/workspace`:

```bash
PANINI_AUDIT_VIDYUT=/path/to/vidyut PANINI_AUDIT_REPO=/path/to/panini \
  mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit
```

## Negative controls

Run at least one before recording a clean result. Each should exit 1 and print
real form-vs-form differences:

```bash
PANINI_AUDIT_PERTURB=form  mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit
PANINI_AUDIT_PERTURB=entry mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit
```

`form` corrupts one form on this engine's side. `entry` is the one that matters:
it resolves √bhū (`01.0001`) against `01.0381` (√paṭh) — a *plausible* wrong entry,
same gaṇa, same pada, fully derivable — and should flag all 36 of √bhū's cells with
`Bavati` vs `paWati`. A control that fails only by producing an empty set proves
much less; keep this one plausible if you change it.

Optionally dump the full table:

```bash
PANINI_AUDIT_DUMP=/tmp/audit-table.tsv mise exec rust@1.99.0 -- cargo run --release --example panini_full_audit
```

## Last recorded result

2026-10-01, juhotyādi 3f2 slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4608
cells / 5699 forms / 102 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3f2 slice: √bhas (`03.0019`), with two
new rules — 6.4.100 *ghasibhasor hali ca* (*bapsati*) and 8.2.26 *jhalo jhali*
(*babDaH*) — 8.2.73 and 8.2.74 without their rudhādi gaṇa test (*abaBat*,
*abaBaH*), and 8.4.55 *khari ca* reading the whole word (`Bs` → `ps`). A
main-vs-branch dump of every prior cell's traces was byte-identical.

Totals: 102 = 101 + 1; 4608 = 4572 + 36 (4 root×pada×lakāra blocks × 9); 5699
= 5655 + 36 + 8 new `ALTERNATES` rows (1083 → 1091), measured via the
harness's corpus block, not assumed.

2026-10-01, juhotyādi 3f slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4572
cells / 5655 forms / 101 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3f slice: √kit (`03.0021`), √tur
(`03.0022`), √dhiṣ (`03.0023`) and √dhan (`03.0024`), with no new rule —
8.2.75 *daś ca* without its rudhādi gaṇa test (√kit's *acikeH*) and 8.3.24
*naś cāpadāntasya jhali* admitting juhotyādi (√dhan's *daDaMsi*, *daDaMhi*).

Totals: 101 = 97 + 4; 4572 = 4428 + 144 (16 root×pada×lakāra blocks × 9); 5655
= 5486 + 144 + 25 new `ALTERNATES` rows (1058 → 1083), measured via the
harness's corpus block, not assumed.

2026-10-01, juhotyādi 3e slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4428
cells / 5486 forms / 97 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3e slice: 7.4.75 *nijāṁ trayāṇāṁ
guṇaḥ ślau* and 7.3.87 *nābhyastasyāci piti sārvadhātuke* (barring 7.3.86),
added for √ṇij (`03.0012`), √vij (`03.0013`) and √viṣ (`03.0014`), with √viṣ's
`z` sandhi reaching the existing tripādī rules unchanged.

Totals: 97 = 94 + 3; 4428 = 4212 + 216 (24 root×pada×lakāra blocks × 9); 5486
= 5249 + 216 + 21 new `ALTERNATES` rows (1037 → 1058), measured via the
harness's corpus block, not assumed.

2026-09-30, juhotyādi 3d2 slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4212
cells / 5249 forms / 94 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3d2 slice: 6.4.78
*abhyāsasyāsavarṇe*, 7.4.60 on a vowel-initial abhyāsa, and 7.4.77's √ṛ row,
added for √ṛ (`03.0017`), with 6.4.72 reading `ANGA` and 6.1.90 writing into the abhyāsa, both
unchanged.

Totals: 94 = 93 + 1; 4212 = 4176 + 36 (4 root×pada×lakāra blocks × 9); 5249
= 5208 + 36 + 5 new `ALTERNATES` rows (1032 → 1037), measured via the
harness's corpus block, not assumed.

2026-09-29, juhotyādi 3d slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 4176
cells / 5208 forms / 93 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3d slice: 7.4.66 *ur at*, 7.4.60
*halādiḥ śeṣaḥ* widened to every non-initial consonant, 7.4.76's √bhṛñ row,
7.4.77 *arti-pipartyoś ca*, 7.1.102 *ud oṣṭhyapūrvasya*, 6.1.77's aṅga arm,
8.2.77 reading the ending on the ślu path, and 8.3.59 *ādeśapratyayayoḥ*
given an `r` arm (*bibharṣi*) — added for √pṝ (`03.0004`), √pṛ (`03.0005`),
√bhṛ (`03.0006`), √ghṛ (`03.0015`), √hṛ (`03.0016`) and √sṛ (`03.0018`).

Totals: 93 = 87 + 6; 4176 = 3924 + 252 (28 root×pada×lakāra blocks × 9); 5208
= 4926 + 252 + 30 new `ALTERNATES` rows (1002 → 1032), measured via the
harness's corpus block, not assumed.

2026-09-29, juhotyādi 3c2 slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 3924
cells / 4926 forms / 87 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3c2 slice: `Rule.bars` (the apavāda
relation declared on a rule and enforced by `run_pipeline`), 7.4.78
*bahulaṁ chandasi* for √gā on the Kaumudī's authority, 6.4.118 *lopo yi*,
and the vikalpas 6.4.117 *ā ca hau* (text-neutral, barring 6.4.116 / 6.4.113
/ 6.4.112) and 6.4.116 *jahāteś ca* — added for √hā parasmaipada (`03.0009`)
and √gā (`03.0026`).

Totals: 87 = 85 + 2; 3924 = 3852 + 72 (8 root×pada×lakāra blocks × 9); 4926 =
4823 + 72 + 31 new `ALTERNATES` rows (971 → 1002), measured via the harness's
corpus block, not assumed.

2026-09-12, juhotyādi 3c slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 3852
cells / 4823 forms / 85 roots**, with the `entry` negative control verified
failing (36 √bhū cells).

The verdict covers the whole juhotyādi 3c slice: the dhātupāṭha number
reaching the pipeline (`Context.dhatupatha`, with 1.1.20 as `Tag::Ghu`), four
new rules (7.4.76 *bhṛñām it*, 6.4.119 *ghvasor eddhāv abhyāsalopaś ca*,
6.1.88 *vṛddhir eci*, 8.2.38 *dadhas tathoś ca*), 6.4.113 moved above 6.4.112
with both given abhyasta arms, 6.1.101's āṭ + ec decline and 8.2.40's *adhaḥ* —
added for √dā (`03.0010`), √dhā (`03.0011`), √mā (`03.0007`) and √hā
(`03.0008`).

Totals: 85 = 81 + 4; 3852 = 3636 + 216 (24 root×pada×lakāra blocks × 9 —
√dā and √dhā two padas × four lakāras each, √mā and √hā one × four); 4823 =
4595 + 216 + 12 new `ALTERNATES` rows (959 → 971), measured via the
harness's corpus block, not assumed.

2026-09-09, juhotyādi 3b slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 3636
cells / 4595 forms / 81 roots**, with the `entry` negative control verified
failing (36 √bhū cells, `Bavati` vs `paWati` and so on — identical DIFF
signature to every prior slice, since the control targets `01.0001`/
`01.0381`, both outside this slice's scope).

The verdict now covers the whole juhotyādi 3b slice: the two new rules
(7.4.60 *halādiḥ śeṣaḥ*, 7.4.59 *hrasvaḥ*), the widened 6.4.82 (now
covering a long `I`) and 6.4.77 (its new iyaṅ arm), and 6.4.115 *bhiyo
'nyatarasyām*, the engine's ninth vikalpa rule — added for the gaṇa's next
two curated roots, √bhī (`03.0002`) and √hrī (`03.0003`). A zero-difference
result across every laṭ/laṅ/loṭ/vidhiliṅ cell of both roots' derivations is
the audit's confirmation that both new rules, both widened rules, and
6.4.115's fork reproduce vidyut-prakriya's forms exactly, not merely this
engine's own goldens.

Totals: 81 = 79 + 2 (√bhī, √hrī); 3636 = 3564 + 72 (2 roots × 1 pada ×
4 lakāras × 9 cells — both roots are parasmaipada-only); 4595 = 4483 + 72
+ 40 new `ALTERNATES` rows (919 → 959), the 72 being the new cells'
baseline forms and the 40 measured via the harness's own corpus block, not
assumed.

2026-09-07, juhotyādi 3a slice, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 3564
cells / 4483 forms / 79 roots**, with the `entry` negative control verified
failing first: exit 1, 36 √bhū cells, `Bavati` vs `paWati` and so on —
identical DIFF signature to every prior slice, since the control targets
`01.0001`/`01.0381`, both outside this slice's scope. (An earlier run of
the same control, taken before this entry's total-assertion updates below
were in place, hit the harness's then-stale `assert_eq!(roots_seen.len(),
77)` and panicked with exit 101 immediately after printing the same 36
DIFFs; re-run after the totals were updated, it exits 1 via the intended
DIFF path.)

The verdict now covers the whole juhotyādi 3a slice: the eight new rules
(2.4.75, 6.1.10, 7.4.62, 3.4.109, 7.1.4, 7.3.83, 6.4.82, 8.4.54), the two
widened arms (6.4.87, 6.4.101), the new `abhyasa` stage, and the
dvitva-before-guṇa order (forms identical, trace order not) — added for
the gaṇa's first two curated roots, √hu (`03.0001`) and √ki (`03.0020`).
A zero-difference result across every laṭ/laṅ/loṭ/vidhiliṅ cell of both
roots' derivations is the audit's confirmation that all eight new rules,
both widened arms, and the reduplication (abhyāsa) machinery they depend
on reproduce vidyut-prakriya's forms exactly, not merely this engine's own
goldens.

Totals: 79 = 77 + 2 (√hu, √ki); 3564 = 3492 + 72 (2 roots × 1 pada ×
4 lakāras × 9 cells — both roots are parasmaipada-only); 4483 = 4399 + 72
+ 12 new `ALTERNATES` rows (907 → 919), the 72 being the new cells'
baseline forms and the 12 measured via the harness's own corpus block, not
assumed.

2026-09-06, juhotyādi prep — five-slot layout, vidyut
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`: **zero differences across 3492
cells / 4399 forms / 77 roots**, with the `entry` negative control verified
failing first (exit 1, 36 √bhū cells, `Bavati` vs `paWati` and so on —
unchanged from 8b, as expected since the control targets `01.0001`/
`01.0381`, both outside this slice's scope). Totals are unchanged from 8b:
this slice touched no root, rule, or golden; it only reshaped every
prakriya's term layout into five fixed slots (`AGAMA` 0, `ABHYASA` 1,
`ANGA` 2, `SHAP` 3, `ENDING` 4) and moved laṅ's augment into the `AGAMA`
slot. The verdict now covers two structural changes: **6.4.71/6.4.72**
writing the augment into its own `AGAMA` term instead of prefixing it onto
the aṅga's text, and **6.1.90**'s aṅga arm merging that `AGAMA` term's text
into the first non-empty term after it (rather than reading a single
already-prefixed aṅga term) — a zero-difference, unchanged-totals result
confirms the reslotting is transparent to every derived form.

2026-09-04, tanādi 8b slice, vidyut `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`:
**zero differences across 3492 cells / 4399 forms / 77 roots**, with the
`entry` negative control verified failing first (exit 1, 36 √bhū cells,
`Bavati` vs `paWati` and so on — unchanged from 8a, as expected since the
control targets `01.0001`/`01.0381`, both outside this slice's root). This
run is the audit gate for √kṛ (`08.0010`), the tenth and final tanādi root,
deferred from 8a.

The verdict now covers three engine changes added for √kṛ:

- **6.4.110 (`ata ut sArvaDAtuke`), 6.4.108 (`nityaM karoteH`), 6.4.109
  (`ye ca`)** — the three √kṛ-specific aṅga rules, in `tinanta/guna.rs`.
  6.4.110 turns `kar`'s `a` to `u` before a kṅit sārvadhātuka; 6.4.108 makes
  6.4.107's optional u-lopa NITYA (obligatory) for √kṛ once the aṅga has
  already become `kur`; 6.4.109 extends that same obligatory lopa to a
  following `y`, producing `kuryāt`-shaped vidhiliṅ parasmaipada forms.
- **8.2.79 (`na BakurCurAm`) modelled as a named exclusion guard inside
  8.2.77's own `apply`**, in `tinanta/tripadi.rs`, rather than as a
  separate rule — 8.2.77 (`hali ca`) would otherwise lengthen `kur`'s
  upadhā the same way it does for other short-ik-upadhā roots ending in
  r/v before a hal-initial sārvadhātuka, deriving a wrong `*kUrvanti`;
  8.2.79 carves `kur` back out (the sūtra's *bha*/*chur* members have no
  curated root). The guard sits inside
  8.2.77's branch, rather than as a second pass, so the exclusion never
  touches a cell 8.2.77 wasn't already about to change. This engine's own
  rule log never records 8.2.79 — the guard returns `false` before
  `p.record` runs — while vidyut-prakriya's log does record it on every
  `kur` cell; the forms agree regardless, which is what the audit checks.

Corpus totals moved from 76/3420/4321 to 77/3492/4399: 77 = 76 + 1 (√kṛ,
`08.0010`); 3492 = 3420 + 72 cells (one ubhayapadī root × 2 padas × 4
lakāras × 9 puruṣa/vacana cells); 4399 = 4321 + 78, of which 72 are the new
cells' baseline forms and 6 are new `ALTERNATES` rows (901 → 907) — exactly
the plan's projected six: the loṭ tātaṅ pairs, laṅ's 8.4.56 row, and
vidhiliṅ's `kuryAd`/`kuryAt`. Measured via the harness's own corpus block,
not assumed.

2026-09-01, tanādi 8a slice, vidyut `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`:
**zero differences across 3420 cells / 4321 forms / 76 roots**, with the
`entry` negative control verified failing first (exit 1, 36 √bhū cells,
`Bavati` vs `paWati` and so on). This run is the audit gate for the tanādi
(gaṇa 8, 8a) slice's nine curated roots (`08.0001`–`08.0009`; the tenth,
√kṛ `08.0010`, is deferred to 8b). It first ran non-clean: an initial pass
found 4 differing cells, all `08.0005` (fR) laṅ uttama-puruṣa dvi/bahu, both
padas. Diagnosis and fix (commit `88cae65`) before the clean re-run below.

Two structural engine changes this verdict now covers:

- **The u-vikaraṇa generalization.** 3.1.79 *tanādikṛñbhya uḥ* gives gaṇa 8
  the bare `u` vikaraṇa; `terms.rs`'s `shnu_asamyogapurva` widened to
  `vikarana_u_asamyogapurva` so every rule that used to read only śnu's
  `nu` now also reads the bare `u`. This needed `run_pipeline`'s
  convergent-fork collapse (dedup live branches on identical final surface
  text, first/declined branch kept) once 7.3.86's new tanādi vikalpa arm
  could put a guṇa'd and an āṭ-vṛddhi'd branch on the same surface (`A+fR`
  and `A+arR` both → `ArRot`).
- **The 6.4.106/6.4.107-before-6.1.90 reorder.** The widened
  asaṁyogapūrva helper must read the aṅga *before* laṅ's āṭ-vṛddhi ekādeśa
  (6.1.90) merges the augment into it — read after, a genuinely
  non-conjunct `u` (fR's) and a guṇa'd conjunct one (arR's) render as the
  same three characters (`rR`) and become indistinguishable, which is what
  produced the four-cell divergence above. Fixed by moving 6.4.106/6.4.107
  ahead of 6.1.90's aṅga arm, order confirmed by tracing vidyut's own
  credited rule sequence for `08.0005` laṅ uttama dvi/bahu; `arRuhi`'s
  decline (the genuinely conjunct, guṇa'd branch) is unaffected and pinned
  by a regression test.

Corpus totals moved from 67/2844/3338 to 76/3420/4321 (76 = 67 + 9 curated
tanādi roots; 3420 = 2844 + 576 cells, where 576 = 64 root×pada×lakāra
blocks × 9 — 64 blocks = 16 pada-blocks (7 ubhayapadī roots × 2 padas + 2
ātmanepada-only roots × 1 pada) × 4 lakāras; 4321 = 3338 + 983, the
measured form total for the new cells). The `ALTERNATES` growth this
implies, 494 → 901 (+407, all from the 576 new tanādi cells), is a
noticeably steeper rate than the rest of the corpus — ~0.71 alternates per
tanādi cell vs. ~0.17 elsewhere. Expected, not a defect: every one of the
nine tanādi roots takes the bare `u` directly after a single non-conjunct
final consonant, so 6.4.107's optional m/v-lopa is asaṁyogapūrva (and
therefore live) far more broadly than it ever was for svādi's `śnu`, and
7.3.86's new vikalpa arm compounds it for four of the nine roots. Measured
via `PANINI_AUDIT_DUMP`, not assumed.

## Scope

The harness tells both engines which pada to derive; it does not audit whether a
root's `PadaAssignment` is itself correct. Auditing the column itself is
`curated_pada_agrees_with_upadesha_markers` in `panini-data`, which re-derives
every verdict from the vendored upadeśa and runs in `cargo test` — the two
audits are complementary, and the pada audit slice ran both. This harness stays
the authority on derived **forms**.
