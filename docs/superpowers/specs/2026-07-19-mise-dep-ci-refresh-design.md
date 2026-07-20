# Design: mise dependency + CI refresh

- **Date:** 2026-07-19
- **Status:** Approved (design), pending implementation
- **Scope:** Bump the mise-pinned Rust toolchain to latest stable, bring the
  cargo dev-tools under mise as pinned tools, refresh the application lockfile,
  migrate to Rust edition 2024, and update CI to match.

## Motivation

The repo pins only `rust = "1.83.0"` in `mise.toml`; every other tool
(cargo-deny, cargo-mutants, cargo-fuzz, cargo-audit) is installed ad hoc via
`cargo install`, so those versions are neither reproducible nor recorded. The
old pin also forces two CI/docs workarounds:

1. CI manually runs `rustup component add clippy` because the mise pin does not
   declare toolchain components.
2. `AGENTS.md` documents a known caveat: `cargo deny check advisories` fails on
   rustc 1.83.0 / cargo-deny 0.17.0 because cargo-deny's rustsec parser cannot
   read CVSS 4.0 advisory vectors. The fix requires cargo-deny ≥ 0.20.2 and a
   newer rustc.

This pass bumps the toolchain, pins the dev-tools, and removes both workarounds.

## Decisions (locked in during brainstorming)

| Question | Decision |
| --- | --- |
| Rust toolchain | Bump `1.83.0` → `1.97.1` (latest stable per `mise latest rust`), pinned. |
| MSRV (`rust-version`) | Bump `1.83` → `1.97` to match the pinned toolchain (honest, CI-verified). |
| Edition | Migrate `2021` → `2024` via `cargo fix --edition`, gated by the golden paradigm test. |
| Dev-tool pinning | Pin cargo-deny/cargo-audit/cargo-mutants/cargo-fuzz via the mise `cargo:` backend. |
| Install-cost layout | Two-tier: base `mise.toml` (rust only) + `mise.dev.toml` (env `dev`) for the heavier cargo tools. |
| Audit CI job | Separate job, weekly `schedule:` cron + PR trigger, `continue-on-error: true` (non-blocking). |

## Resolved tool versions

Confirmed available via mise on 2026-07-19 (`mise latest rust`,
`mise ls-remote "cargo:<tool>"`):

| Tool | Old | New (pinned) |
| --- | --- | --- |
| rust | 1.83.0 | 1.97.1 |
| cargo-deny | 0.17.0 (ad hoc) | 0.20.2 |
| cargo-audit | ad hoc | 0.22.2 |
| cargo-mutants | ad hoc | 27.1.0 |
| cargo-fuzz | ad hoc | 0.13.2 |

## Component design

### 1. `mise.toml` — base toolchain (everyday fast path)

Replace the flat `rust = "1.83.0"` line with an inline table that also declares
the components CI needs, so the toolchain ships clippy and rustfmt itself:

```toml
[tools]
rust = { version = "1.97.1", components = "clippy,rustfmt" }
```

The `[tasks.*]` block is unchanged except for one addition (see below). Verified:
mise 2026.7.5 parses the inline-table `components` form without error.

**Add a `fmt-check` task** (the existing `fmt` task writes; CI needs a
non-mutating check):

```toml
[tasks.fmt-check]
run = "cargo fmt --all --check"
```

**Rationale for the two-tier split:** every `mise install` installs all tools in
the merged config. If the four cargo dev-tools lived in the base `[tools]`, the
common build/test/lint CI job (which needs none of them) would compile them from
source — potentially 5–15 min on a cold runner. Keeping the base config to
`rust` only keeps that path lean.

### 2. `mise.dev.toml` — pinned dev-tools (config environment `dev`)

New file, loaded only when `MISE_ENV=dev` is set (mise config-environment
convention `mise.<env>.toml`):

```toml
[tools]
"cargo:cargo-deny"    = "0.20.2"
"cargo:cargo-audit"   = "0.22.2"
"cargo:cargo-mutants" = "27.1.0"
"cargo:cargo-fuzz"    = "0.13.2"
```

- Pulled on demand with `MISE_ENV=dev mise install` — locally by contributors
  who run audit/mutants/fuzz, and by the CI audit job.
- `cargo-audit` is required by the `audit` task (`cargo audit && cargo deny
  check`) even though it was not in the original ask; it is included here.
- `cargo-fuzz` is pinned for reproducibility, but real fuzzing still needs a
  nightly toolchain (not pinned, not in CI) — it stays a manual opt-in.

**Alternative considered (not chosen):** a single flat `mise.toml` holding all
five tools. Simpler, but forces the source compile of all dev-tools on every
`mise install` including the fast CI job. Rejected in favor of the two-tier
split; revisit only if the config-environment indirection proves annoying.

### 3. `Cargo.toml` — manifest

```toml
[workspace.package]
edition = "2024"        # was "2021"
rust-version = "1.97"   # was "1.83"
```

- Edition migration is performed with `cargo fix --edition` (run per-package /
  across the workspace), then flipping the `edition` field. The **golden
  paradigm test** (`crates/panini/tests/paradigm.rs`) plus the full
  `cargo test --workspace` are the acceptance gate — surface forms must be
  unchanged.
- `resolver` stays `"2"` (already set); edition 2024's default resolver 3 is not
  adopted in this pass to keep the change minimal. Flagged as a possible
  follow-up, not part of this spec.
- The floating dependency ranges (`clap = "4"`, `serde = "1"`, `serde_json =
  "1"`, `proptest = "1"`) are **unchanged**; only `Cargo.lock` is refreshed.

### 4. `Cargo.lock` — application dependencies

Run `cargo update` to pull the locked dependencies to the latest versions within
the existing ranges, and commit the regenerated lockfile. Current → expected
floor (may resolve higher at implementation time):

| Crate | Current locked |
| --- | --- |
| clap | 4.5.40 |
| serde | 1.0.229 |
| serde_json | 1.0.150 |
| proptest | 1.5.0 |

Acceptance: `cargo test --workspace` green after the update.

### 5. `.github/workflows/ci.yml` — CI

Two jobs.

**Job `build-test-lint`** (evolves today's single job):

```yaml
  build-test-lint:
    name: build + test + lint (rust 1.97, mise-pinned)
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: jdx/mise-action@v2
      - run: mise run fmt-check
      - run: mise run build
      - run: mise run test
      - run: mise run lint
```

- The manual `rustup component add clippy` step is **removed** — the mise pin now
  provides clippy (and rustfmt) via `components`.
- Adds the `fmt-check` step (new task from §1).
- Job label no longer hardcodes "rust 1.83".

**Job `audit`** (new, non-blocking):

```yaml
  audit:
    name: supply-chain audit (non-blocking)
    runs-on: ubuntu-latest
    continue-on-error: true
    steps:
      - uses: actions/checkout@v4
      - uses: jdx/mise-action@v2
        env:
          MISE_ENV: dev
      - run: mise run audit
        env:
          MISE_ENV: dev
```

- Triggered on PRs, pushes to `main`, and a weekly `schedule:` cron.
- `continue-on-error: true` so a newly published upstream advisory (a
  time-sensitive event, not a code regression) does not red-X unrelated PRs.
- `MISE_ENV: dev` activates `mise.dev.toml`, installing the pinned cargo-deny +
  cargo-audit for the `audit` task.

Workflow triggers become:

```yaml
on:
  pull_request:
  push:
    branches: [main]
  schedule:
    - cron: "0 6 * * 1"   # weekly, Monday 06:00 UTC — audit job
```

Mutation testing (`mise run mutants`) and fuzzing stay **out of CI** — slow /
nightly-dependent — but their tools are now pinned for local use.

### 6. `AGENTS.md` — docs

- Replace the "Optional tools (install on demand) … `cargo install …`" guidance
  with: optional tooling is pinned in `mise.dev.toml`; install it with
  `MISE_ENV=dev mise install`.
- **Remove the cargo-deny advisories caveat** entirely — resolved by cargo-deny
  0.20.2 + rustc 1.97.1. `mise run audit` is now expected to pass.
- Update the pinned-version reference from 1.83.0 to 1.97.1 and the CI job
  description.
- Keep the cargo-fuzz nightly note (nightly is still required for real fuzzing
  and is not provisioned here).

## Out of scope

- Adopting resolver 3.
- Bumping the floating dependency major ranges in `Cargo.toml`.
- Adding mutation testing or fuzzing to CI.
- Provisioning a pinned nightly toolchain.

## Acceptance criteria

1. `mise install` provisions rust 1.97.1 with clippy + rustfmt; `cargo clippy`
   and `cargo fmt` work with no extra `rustup` step.
2. `MISE_ENV=dev mise install` provisions the four pinned cargo dev-tools.
3. `cargo test --workspace` passes on edition 2024 with the golden paradigm test
   unchanged.
4. `mise run fmt-check`, `mise run build`, `mise run test`, `mise run lint` all
   pass.
5. `MISE_ENV=dev mise run audit` passes (both `cargo audit` and
   `cargo deny check`), confirming the advisories caveat is gone.
6. CI: build-test-lint job is green with no rustup-clippy step; audit job runs
   non-blocking on PR + weekly schedule.
7. `AGENTS.md` reflects the new install flow and no longer lists the cargo-deny
   caveat.
