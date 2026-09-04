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

**It asserts the corpus totals** (77 roots, 3492 cells, 4399 forms) rather than
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
mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit
```

Both checkout locations are env-overridable, defaulting to `/tmp/vidyut-full` and
`/workspace`:

```bash
PANINI_AUDIT_VIDYUT=/path/to/vidyut PANINI_AUDIT_REPO=/path/to/panini \
  mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit
```

## Negative controls

Run at least one before recording a clean result. Each should exit 1 and print
real form-vs-form differences:

```bash
PANINI_AUDIT_PERTURB=form  mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit
PANINI_AUDIT_PERTURB=entry mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit
```

`form` corrupts one form on this engine's side. `entry` is the one that matters:
it resolves √bhū (`01.0001`) against `01.0381` (√paṭh) — a *plausible* wrong entry,
same gaṇa, same pada, fully derivable — and should flag all 36 of √bhū's cells with
`Bavati` vs `paWati`. A control that fails only by producing an empty set proves
much less; keep this one plausible if you change it.

Optionally dump the full table:

```bash
PANINI_AUDIT_DUMP=/tmp/audit-table.tsv mise exec rust@1.98.0 -- cargo run --release --example panini_full_audit
```

## Last recorded result

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
  8.2.79 carves `kur` (and the cur-class) back out. The guard sits inside
  8.2.77's branch, rather than as a second pass, so the exclusion never
  touches a cell 8.2.77 wasn't already about to change, and the rule log
  still records 8.2.79 on every `kur` cell.

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
