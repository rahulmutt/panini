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

**It asserts the corpus totals** (63 roots, 2592 cells, 3014 forms) rather than
reporting whatever it enumerated. Those are facts about the repo, pinned
independently by `derivation_set_shape_matches_the_audited_numbers` in
`crates/panini/tests/paradigm.rs`. If the harness disagrees, the harness is wrong.

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

2026-08-21, rudhādi 7d slice, vidyut `8da2f90`:
**zero differences across 2592 cells / 3014 forms / 63 roots**, with the
`entry` negative control verified failing first (exit 1, 36 √bhū cells,
`Bavati` vs `paWati` and so on). This slice curated eight further rudhādi
roots — Siz, und, anj, tanc, vij, vfj, pfc, vid — that a probe showed need
no new `Rule`. Because no `Rule` changed in this slice, the zero-difference
verdict is a statement about those eight rows alone: the engine was already
correct about every rule they exercise, and this audit is the proof. Corpus
totals moved from 55/2304/2654 to 63/2592/3014 (63 = 55 + 8 curated roots;
2592 = 2304 + 8 × 36 cells per root; 3014 = 2654 + 360, i.e. 422 total
`ALTERNATES` rows, up from 350).

## Scope

The harness tells both engines which pada to derive; it does not audit whether a
root's `PadaAssignment` is itself correct. Auditing the column itself is
`curated_pada_agrees_with_upadesha_markers` in `panini-data`, which re-derives
every verdict from the vendored upadeśa and runs in `cargo test` — the two
audits are complementary, and the pada audit slice ran both. This harness stays
the authority on derived **forms**.
