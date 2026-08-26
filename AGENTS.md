# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`) to rust 1.98.0. Do not install
  Rust globally.
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- Optional dev/audit tooling is pinned in `mise.dev.toml`. Install it on demand:
  `MISE_ENV=dev mise install`. This provides:
  - `cargo-mutants` (mutation testing) — `mise run mutants` runs
    `cargo mutants --package panini-prakriya --test-workspace=true --timeout
    4800 -j 4` (the `--test-workspace` flag is required so each **mutant** run
    exercises the `panini` crate's golden paradigm/trace/roundtrip tests, not
    just `panini-prakriya`'s own unit tests — but it does NOT apply to
    cargo-mutants' own **baseline** run, which always exercises only the
    mutated package's tests regardless of the flag). The explicit, generous
    `--timeout` is required for the same asymmetry: cargo-mutants calibrates
    its per-mutant timeout from the baseline's runtime, but the baseline here
    (`panini-prakriya`'s unit tests, ~2s) is far faster than an actual mutant
    run (the full `panini` golden suite, ~183s at 1728 cells when the mutant
    is caught in the paradigm binary and the run aborts there — but ~380s
    when it is NOT caught and the suite runs to completion; both re-measured
    in slice 7b, and ~140s / ~300s at 1620 cells before it. At the **1800**
    cells of the ubhayapada 1.3.72 slice, a standalone `mise run test` — one
    suite, no mutation campaign alongside it — measured paradigm ~207s and
    roundtrip ~240s (trace ~2s), i.e. an **uncontended** uncaught floor of
    ~450s, *more* than the ~395s a 4%-growth scaling predicts. Scale the
    floor by measurement, not by cell count). Under a cap
    that doesn't clear that uncaught-run floor — or auto-derived timing,
    which falls back to a 20s floor — a mutant that survives is recorded
    as a **timeout rather than a survivor**, so a reported zero-survivor
    run (checking only `missed.txt`) can be vacuous instead of clean:
    slice 7a's own mutation run hit this exactly, with `--timeout 300`
    producing 9 timeouts once the suite grew to 1620 cells — a cap that no
    longer had headroom over a full uncaught run, not merely over a
    caught-and-aborted one. Always pass an explicit `--timeout` with
    headroom over a full uncaught run of the workspace suite, and always
    check `timeout.txt` alongside `missed.txt`. **The cap must clear a full
    uncaught run at the parallelism you actually use, not just at `-j 1`.**
    1200s clears the ~380s uncontended floor measured at 1728 cells only
    while contention stays low: slice 7b ran the gate at `-j 16` on 24 cores
    and got **43 timeouts** where one was expected — the same vacuity,
    reached through parallelism instead of suite growth. Re-running exactly
    those at `-j 4 --timeout 2400` caught 43 of 44 in 389–449s — 2.1–2.5×
    the ~183s caught-and-aborted figure measured standalone, which is the
    only direct evidence here of what `-j 4` costs. **Nothing in this repo
    has yet measured a `-j 4` run against the 1800-cell suite.** Extrapolate
    the same 2.1–2.5× onto its ~450s uncontended floor and an uncaught
    mutant at `-j 4` lands somewhere near 950–1100s: still under the 1200s
    cap, but with a margin of tens of percent, not the 3× the 1728-cell
    figures suggest. Treat 1200s at `-j 4` as adequate-but-unverified, not
    comfortable; if a timeout appears that is not the known permanent one,
    re-run it alone before concluding anything. `cargo mutants` also
    reads `-j` from `CARGO_MUTANTS_JOBS`, so an unqualified cap can be
    defeated by the environment alone. Keep `-j` at or below 4 with the
    1200s cap, or raise the cap in step, and re-measure both the floor and
    the margin the next time the golden suite grows.
    Re-running those 9 at `--timeout 1200` resolved them into two different
    outcomes, both worth knowing about: **3 were genuine survivors** — all
    three mutants of `Context::is_tip`, whose only caller was 8.2.73's
    `is_tip() || is_sip()` guard; `dhatu_is_pada_final` plus the `s`-final
    check already selected exactly the cells that guard was trying to name,
    so it was dead weight, and both the clause and `Context::is_tip` itself
    (having no other caller) were deleted — the discipline working exactly
    as intended, on a guard the previous fix round had not thought to
    question because it looked like ordinary domain modelling, not
    redundant plumbing. **1 is a genuine, permanent timeout, not a
    survivor**: `tripadi.rs`'s ṇatva backward scan (`is_natva_target`'s
    caller) decrements a loop index with `j -= 1`; mutating that to `j /=
    1` makes `j` constant and the loop never terminates. No assertion can
    ever catch this — the mutated run never reaches one — so the 1200s cap
    itself *is* the detection mechanism, not a symptom of too short a cap.
    This is a different phenomenon from the reclassification problem above
    (a real survivor misreported as a timeout because the cap is too
    short): here the mutant genuinely does hang, at any cap, and a
    `timeout.txt` entry of this shape is the correct, permanent verdict —
    do not chase it with a bigger `--timeout` or a code change; the loop
    itself is correct, working code.
    **The ubhayapada 1.3.72 slice's own campaign did not use the 1200s
    default.** It ran by hand at `-j 4 --timeout 2400` — 522 mutants, 482
    caught, 0 missed, 39 unviable, and the one known-permanent timeout
    above — doubling the cap precisely because the ~450s uncontended floor
    times an unverified `-j 4` contention factor left no margin worth
    reading a `timeout.txt` entry against. `mise.toml`'s default was
    deliberately left at 1200 at the time, with `--timeout 2400` passed by
    hand; **that is no longer so — `mise run mutants` now runs
    `-j 4 --timeout 2400` itself**, so run the gate through the task rather
    than reconstructing the flags, and change the default only against a
    fresh measurement of the floor and the margin.
    **The pada audit measured both at 1872 cells.** Uncontended floor:
    paradigm ~205s, roundtrip ~236s, trace ~2s (uncaught total ~443s) — flat
    against the 1800-cell ~450s figure, in fact slightly below it. Campaign
    at `-j 4 --timeout 2400`: 522 mutants, 482 caught, 0 missed, 39 unviable,
    and the one known-permanent `tripadi.rs` timeout. This is the `-j 4`
    timing measurement the paragraph above calls for, taken at the suite's
    size at that point rather than the 1800 cells named there, which will
    not recur: `outcomes.json`'s per-mutant test-phase durations for the 482
    caught mutants put the median at 30.1s, p90 at 346.6s, p99 at 547.2s, and
    the max at 754.6s, with only 4 mutants over 600s and none over 1200s. The
    slowest caught mutant,
    `crates/panini-prakriya/src/tinanta/vikarana.rs:316:17`, is a near-direct
    measurement of that worst case rather than an inference from contention
    alone: it ran the golden suite to completion without being caught —
    `paradigm` passed (11 passed, 356.90s against an uncontended 205.47s
    standalone, 1.74×) and `roundtrip` passed (1 passed, 391.84s against an
    uncontended 236.36s standalone, 1.66×), with `trace` adding ~0s — and
    was caught only afterwards, by `panini-prakriya --lib`, which failed in
    0.20s. 356.90 + 391.84 ≈ 748.7s of its 754.6s total was therefore the
    golden suite running to completion without catching it, which is exactly
    the work an uncaught mutant does. The measured worst-case contention
    factor, 754.6 / 443 ≈ 1.7×, is accordingly apples-to-apples against
    the uncontended floor — both divide by an uncaught-run time, not a
    caught-and-aborted one — well under the 2.1–2.5× inferred from the
    `-j 16` re-runs above, so that extrapolation was pessimistic. 2400 kept
    real margin — roughly 3.2× the slowest observed caught mutant. 1200
    would also have sufficed in this run, since nothing exceeded it, but only
    by ~1.6× over that same 754.6s max, not the ~3× the 1728-cell figures
    showed. This run caught every mutant, so a genuinely uncaught mutant —
    one that also spends `trace`'s ~2s rather than the ~0s this one did,
    and is never caught by `--lib` — remains unobserved; but because the
    slowest caught mutant already paid the full golden-suite cost before
    `--lib` caught it, the uncaught worst case is no longer purely inferred
    from contention factors, only from the small remainder that one mutant
    leaves unmeasured. Keep `--timeout 2400` rather than dropping to 1200
    until a genuinely uncaught mutant is actually observed.
    **Slice 7c re-measured both at 2160 cells.** Uncontended floor:
    paradigm 276.99s, roundtrip 331.81s, trace 1.93s — an uncaught total of
    **610.73s**, where scaling the 1872-cell ~443s figure by cell count
    predicted ~510s. Cell count has now failed as a multiplier in both
    directions (flat from 1800 to 1872, then 38% for a 15% growth): measure
    the floor, never scale it. Campaign at `-j 4 --timeout 2400`: **522
    mutants, 482 caught, 0 missed, 39 unviable, 1 timeout** — that one being
    the known-permanent `tripadi.rs:1140:23` non-terminating-loop mutant
    described above, which is the correct verdict at any cap.
    `outcomes.json`'s per-mutant test-phase durations for the 482 caught
    mutants put the median at 46.1s, p90 at 419.0s, p99 at 679.4s and the
    max at 848.2s, with **44** mutants over 600s and none over 1200s
    (the pada audit's were 30.1 / 346.6 / 547.2 / 754.6 with only **4** over
    600s — the over-600s count jumping 4 → 44 is the number to watch here,
    not the max). **Two margins, and they answer different questions.**
    Against the worst **caught** mutant, 848.2s, *directly measured*: 2.83×
    at 2400, 1.41× at 1200. Against the worst **uncaught** run — the
    610.73s floor times the 1.70× `-j 4` contention factor the pada audit
    measured, so ~1040s, a *projection* and not a measurement: **2.31× at
    2400**, 1.16× at 1200. The uncaught figure is the one that governs the
    vacuity question, because a genuine survivor runs the suite to
    completion whereas a caught run can abort the moment it is detected —
    do not quote the caught max as if it were the margin basis, and do not
    quote the projection as if it were measured. 1200 would have passed
    this campaign too, nothing having exceeded it, but 1.16× of projected
    headroom is exactly the shape that turns a "0 missed" into a vacuous
    one on a busier machine. Keep 2400.
    **This slice (ric/vic, 8.2.30) re-measured both at 2304 cells.**
    Uncontended floor: paradigm 321.34s, roundtrip 371.81s, trace 2.00s —
    an uncaught total of **~695.15s** (wall clock 11m36.179s), where scaling
    7c's 610.73s figure by cell count predicted a much smaller move than the
    **+13.8%** actually observed against **+6.7%** cell growth. The series
    so far: flat from 1800 to 1872 cells, then +38% for +15% into 7c, and
    now +13.8% for +6.7% — cell count has again failed as a multiplier,
    this time in the superlinear direction; measure the floor, never scale
    it. Cap sanity check before the campaign: 695.15s × the pada audit's
    1.70× `-j 4` contention factor ≈ **1182s** projected worst case for an
    uncaught mutant, a **2.03×** margin under the 2400s cap, so the campaign
    ran without raising it. Campaign at `-j 4 --timeout 2400`, run via the
    `cargo-mutants` binary directly rather than the mise shim: **527
    mutants, 487 caught, 0 missed, 39 unviable, 1 timeout** (process exit
    code 3, expected whenever a timeout exists) — that one being the
    known-permanent `tripadi.rs:1156:23` non-terminating-loop mutant
    described above (its line number moved from `:1140:23` because this
    slice added code above it), which is the correct verdict at any cap.
    `outcomes.json`'s per-mutant test-phase durations for the 487 caught
    mutants put the median at 50.5s, p90 at 490.0s, p99 at 738.3s and the
    max at 976.8s, with **46** mutants over 600s and none over 1200s. The
    slowest three were all `BinaryOperator` mutants: `tripadi.rs` at
    976.8s, then `vikarana.rs` at 917.9s and 863.0s. The over-600s count
    across slices is now 4 (pada audit) → 44 (7c) → 46 (this slice), while
    the max has again barely moved — that count remains the number to
    watch. **Two margins, and they answer different questions.** Against
    the worst **caught** mutant, 976.8s, *directly measured*: 2400 / 976.8
    = **2.46×**. Against the worst **uncaught** run, the projected ~1181.8s
    above, *projected and not measured*: 2400 / 1181.8 = **2.03×**. The
    brief predicted ~530 mutants from `kutva_of` alone (two
    function-replacement, four arm-deletion); actual came in at 527, close
    but not identical, because this slice also widened 8.2.39 via
    `jashtva_of`, which the brief did not anticipate — the composition
    differs from the prediction even though the total is close. Keep 2400.
    **Slice 7d (eight rudhādi roots, no new sūtra) re-measured both at 2592
    cells.** Uncontended floor: paradigm 397.27s, roundtrip 472.38s, trace
    2.23s — an uncaught total of **871.88s** (wall clock 14m35.882s).
    Scaling the last slice's 695.15s figure by cell count (2304 → 2592,
    +12.5%) predicted ~782.04s; the measurement came in ~90s hotter —
    **+25.4% wall for +12.5% cells**, the fourth consecutive slice where
    cell count fails as a multiplier, again in the superlinear direction.
    **This is the first floor measured under rust 1.98.0** (`mise.toml`,
    `57f886f`); every earlier figure in this series, including the 695.15s
    one just cited, was measured under 1.97.1. The compiler is therefore a
    live candidate cause for the ~90s excess — named as a candidate, not
    asserted as the cause, since no isolated before/after comparison at a
    fixed cell count was taken. Cap sanity check before the campaign:
    871.88s × the pada audit's 1.70× `-j 4` contention factor ≈ **1482.2s**
    projected worst case for an uncaught mutant, a **1.62×** margin under
    the 2400s cap (the spec expected ~1450–1500s / ~1.6×), so the campaign
    ran without raising it. Campaign at `-j 4 --timeout 2400`, run via the
    `cargo-mutants` binary directly (the mise shim errored with "no version
    set for shim: cargo-mutants"): **527 mutants, 487 caught, 0 missed, 39
    unviable, 1 timeout** (487 + 39 + 1 = 527) — the mutant population
    unchanged from the last slice, as expected, since this slice adds no
    `panini-prakriya` code. The one timeout is the known-permanent
    non-terminating-loop mutant on the ṇatva backward scan,
    `tripadi.rs:1157:23: replace -= with /=` — identified **by that shape**
    (mutating `j -= 1` to `j /= 1` makes `j` constant, so the mutated run
    never reaches an assertion), not by its line number: this slice touches
    no lines in `tripadi.rs` (`git log 9ffe1ac..HEAD -- crates/panini-prakriya/src/tinanta/tripadi.rs`
    is empty), yet the line moved from `:1156:23` to `:1157:23` anyway — a
    one-line drift that predates this branch and that this slice did not
    cause. The earlier paragraphs' `:1140:23` and `:1156:23` citations are
    left as written; they were correct at the time. `outcomes.json`'s
    per-mutant test-phase durations for the 487 caught mutants put the
    median at 68.5s, p90 at 598.4s, p99 at 928.6s and the max at 1224.4s,
    with **48** mutants over 600s and, for the first time in this series,
    **1** over 1200s. The over-600s count across slices is now 4 (pada
    audit) → 44 (7c) → 46 (last slice) → 48 (this slice) — still the number
    to watch — while the max also moved further than in prior slices
    (976.8s → 1224.4s). The slowest was a `BinaryOperator` mutant on
    `tripadi.rs:1152:33` (`replace < with <=`, 1224.4s), followed by two
    `vikarana.rs` `BinaryOperator` mutants at 1028.5s and two
    `LogicalOperator` mutants at 980.3s and 961.0s. **Two margins, and they
    answer different questions.** Against the worst **caught** mutant,
    1224.4s, *directly measured*: 2400 / 1224.4 = **1.96×**. Against the
    worst **uncaught** run, the projected ~1482.2s above, *projected and
    not measured*: 2400 / 1482.2 = **1.62×**. Both margins shrank from the
    last slice's 2.46×/2.03×, consistent with the floor's outsized jump,
    but neither crossed 1×. Keep 2400.
    **This slice (rudhādi gaṇa 7e, three new sūtras) re-measured both at
    2628 cells.** Uncontended floor: paradigm 432.94s, roundtrip 508.54s,
    trace 2.22s — an uncaught total of **943.70s** (`mise run test`'s wall
    clock came in at 945s, the ~1.3s difference being `panini-prakriya`'s
    own unit-test binary at 0.17s plus the remaining small blocks). Cell
    count grew only **+1.4%** this slice (2592 → 2628, the smallest growth
    in the series), yet the floor grew from 7d's 871.88s to 943.70s, **about
    +8.2%** — cell count under-predicting the floor's move for the fifth
    consecutive slice, and by the widest margin yet relative to how little
    the cell count itself moved: the standing advice that the floor does not
    track cell count holds, and this is its sharpest confirmation so far.
    Zoomed out past any single slice, the trend is the same conclusion at
    larger scale: **~450s at 1800 cells to 943.70s at 2628 cells** — cells
    up ~46%, floor up ~110% — is emphatically superlinear, not a fixed
    multiplier of cell count in either direction.
    Cap sanity check before the campaign: 943.70s × the pada audit's
    2.1–2.5× `-j 4` contention factor range projects an uncaught mutant at
    **1982–2360s**. Against the outgoing `--timeout 2400` cap, that is a
    margin of only **1.7%–21%** — below the "tens of percent... not
    comfortable" bar this very paragraph sets, and exactly the vacuity
    shape flagged above: under too tight a cap a genuine survivor is
    recorded as a TIMEOUT rather than a MISSED, and a reported "0 missed"
    becomes meaningless rather than clean. **2400 is retired.** `mise.toml`'s
    cap is raised to **`--timeout 4800`**, with `-j 4` left unchanged — the
    2.1–2.5× contention factor was measured at `-j 4`, so changing
    parallelism would invalidate the projection built on it. At 4800, the
    margin against the projected uncaught range becomes **2.03×–2.42×**,
    back in the "roughly 2×" territory the pada audit and 7c campaigns ran
    at, rather than the sub-1.2×-to-1.02× range the outgoing cap was left
    running under.
    Campaign at `-j 4 --timeout 4800`: **547 mutants, 505 caught, 2 missed,
    39 unviable, 1 timeout** (505 + 39 + 1 = 545, plus the 2 missed = 547),
    wall clock **8h**. The timeout is the known-permanent
    `tripadi.rs`, 8.4.2's backward ṇatva scan, non-terminating-loop mutant
    (`j -= 1` -> `j /= 1`, making `j` constant so the mutated run never
    reaches an assertion) — the correct verdict at any cap, and it consumed
    the full 4800s, so raising the cap from 2400 cost exactly one extra
    ~40 minutes on this one permanent, undetectable-by-assertion case.
    **Both missed mutants are equivalent mutants, verified individually,
    not missing tests:**
    - `adesha.rs`, 6.1.87's new im arm, `replace + with *` (`s.remove(pos
      + 1)` -> `s.remove(pos)`) — removing either half
      of an adjacent `a i` pair and then overwriting index `pos` with `'e'`
      produces the same string either way, since whichever character
      survives the removal shifts into (or already sits at) `pos` and is
      immediately clobbered. True for any input reaching this arm, not
      just this suite's cells. Documented in place at that guard.
    - `tripadi.rs`, 8.3.13's own guard, `replace - with /` (`w[i - 1]` ->
      `w[i]`, eliding the second ḍh instead of the first) — both `Q`s at
      that position are the same character, so the surface result is
      identical either way; the golden suite's full 2628 cells, its
      ALTERNATES, and its traces cannot distinguish which term lost the
      character. `w[i - 1]` remains the only grammatically correct choice
      regardless — the sūtra is *ḍho ḍhe lopaḥ*, "of ḍh, before ḍh,
      elision," which names the FIRST ḍh as the one elided — but that
      correctness is not observable at the surface, hence the survivor.
      Documented in place at that guard. **Correction to this task's own
      plan:** Task 9's brief predicted this exact survivor but reasoned
      "`tfReQi` should [distinguish it], since eliding the second gives
      `tfReQ`" — that arithmetic is wrong; eliding the second `Q` gives the
      same `tfReQi` as eliding the first, which is exactly why the mutant
      survives rather than getting caught. Worth remembering so a future
      slice does not re-chase this one expecting a distinguishing cell to
      exist.
    `outcomes.json`'s per-mutant test-phase durations (**n = 508** — 547
    mutants minus the 39 unviable ones, which fail at the Build phase and
    never reach a Test phase) put the median at 73s, p90 at 682s (nearest-
    rank, the same convention used for p99 below — an earlier pass at this
    entry used a floor-indexed rank for p90 alone and reported 669s), p99 at
    1290s and the max at 4800s — that max being the known-permanent
    timeout itself, not a caught or missed mutant. Excluding the timeout,
    the max is **1345s**, and the top seven non-timeout runs were 1345s,
    1341s, 1335s, 1293s, 1290s, 1032s and 1031s, all `CaughtMutant`.
    **Contention finding that corrects this paragraph's own number — and
    an earlier draft of this same entry.** The two missed mutants ran the
    full golden suite to completion without being caught — a direct
    measurement of an uncaught `-j 4` run — at **980s** and **967s**
    against the **943.70s** uncontended floor measured above, a factor of
    **~1.02×–1.04×**. An earlier pass at this entry reported that figure as
    *the* measured contention; it is real but is not the worst case, and
    presenting it alone was misleading — those two mutants simply happened
    to run during lighter scheduling overlap, not at the ceiling. The
    honest figure comes from all 508 test phases, where the longest
    non-timeout run, 1345s, is **~1.43×** the floor — still below the
    2.1–2.5× this paragraph has projected from since the `-j 16` re-runs in
    slice 7b, so that figure remains overstated for this machine and should
    still be treated as machine-dependent, not settled — but the gap is
    narrower than the single-sample 1.02×–1.04× suggested, and this
    paragraph must not be read as claiming ~1.03× is the ceiling. Quote the
    full range (1.02×–1.43×) and its basis (two direct uncaught-run
    measurements plus the 508-sample distribution), not one flattering
    number.
    Margin arithmetic against the worst **observed non-timeout** run,
    1345s, *directly measured, not projected*: against the retired 2400s
    cap, 2400 / 1345 ≈ **1.78×**; against the new 4800s cap,
    4800 / 1345 ≈ **3.57×**. **Ruling: keep 4800.** This is now better
    supported than when the cap was first raised, for a reason the
    percentile data makes explicit: a **caught** mutant ran longer (1345s)
    than either of the two **uncaught** ones (980s, 967s), because
    scheduling overlap under `-j 4` — not whether a mutant is caught or
    times out — dominates wall-clock duration. That is exactly why a cap
    must never be provisioned from a single sampled run, uncaught or
    otherwise: the two-mutant sample that looked like ~1.03× contention was
    luck, and a cap sized to it would have left only 2400 / 1345 ≈ 1.78×
    margin against the actual worst case observed in this very campaign,
    not the false comfort the two-sample figure implied. The costs of the
    two failure directions remain asymmetric — too low a cap silently turns
    a "0 missed" into a vacuous result (the failure this repo has hit
    twice, in 7a and 7b), while too high a cap costs only the ~40 minutes
    the one permanent timeout above already shows — so over-provisioning
    stays correct even now that the real contention range is measured and
    narrower than 2.1–2.5×. The next slice should reason from the 943.70s
    floor and this **1.02×–1.43×** range — or a fresh direct measurement of
    its own — rather than re-deriving 2.1–2.5× as settled, and rather than
    quoting only the lowest sample as if it were the ceiling.
    **This slice (rudhādi gaṇa 7f, √chid and √chṛd) re-measured both at
    2772 cells.** Uncontended floor: paradigm 486.69s, roundtrip 576.91s,
    trace 2.39s — an uncaught total of **1066.834s** (`time mise run
    test`'s own wall clock). Cell count grew **+5.5%** this slice (2628 →
    2772); the floor grew from 7e's 943.70s to 1066.834s, **+13.05%** — a
    **2.37×** under-prediction, the sixth consecutive slice where scaling
    the floor by cell count would have been wrong (smaller miss than 7e's
    own ~6×, but still the wrong model in the same direction). `roundtrip`
    carried most of the growth (508.54s → 576.91s, +13.4%, above the
    overall rate); `paradigm` also outgrew cells (432.94s → 486.69s,
    +12.4%); `trace` stayed flat in absolute terms (2.22s → 2.39s, +7.7%
    on a floor too small for the multiplier to mean much).
    Cap sanity check before the campaign: 1066.834s × the 7e-measured
    **1.02×–1.43×** `-j 4` contention range (not the retired 2.1–2.5×,
    and not the 1.02× end read alone) projects an uncaught mutant at
    **1088–1526s**. Against the standing `--timeout 4800` cap, that is a
    margin of **3.15×–4.41×**, in the same "roughly 3×" territory the cap
    was raised into during 7e and comfortably clear of the brief's own
    (older, pre-4800-cap) ~2400s alarm threshold. **Ruling: keep 4800,
    unchanged.** No controller escalation was needed because the
    projection concluded "adequate," not "raise it."
    Campaign at `-j 4 --timeout 4800`: **571 mutants, 529 caught, 2
    missed, 39 unviable, 1 timeout** (529 + 39 + 1 = 569, plus the 2
    missed = 571), wall clock **10h**. Both the timeout and the two missed
    mutants were verified in place against the brief's predictions, not
    assumed from the shape of a "clean" result:
    - The timeout is the known-permanent `tripadi.rs`, 8.4.2's backward
      ṇatva scan, non-terminating-loop mutant (`j -= 1` -> `j /= 1` at
      `tripadi.rs:1396`, making `j` constant so the mutated run never
      reaches an assertion), confirmed by re-reading the construct at its
      current line rather than trusting the line number to have held
      still across two slices of drift — it hadn't (7e reported it at a
      different line; this slice adds a rule above it in the same file).
      Ran the full 4800.02s cap, the correct verdict at any cap.
    - Both missed mutants are 7e's own two verified equivalent mutants,
      confirmed unchanged at their guards: `adesha.rs:393`, 6.1.87's im
      arm (`s.remove(pos + 1)` -> `s.remove(pos)`, still documented
      in-place as equivalent because whichever half of the adjacent `a i`
      pair survives the removal is immediately clobbered by the `'e'`
      assignment that follows), and `tripadi.rs:1156`, 8.3.13's guard
      (`w[i - 1]` -> `w[i]`, still documented in-place as equivalent
      because both `Q`s at that position are the same character, so the
      surface form cannot distinguish which one the grammar actually
      elides). Neither site nor its reasoning has changed since 7e; no
      new equivalence argument was needed. **Step 5 (fix any genuine
      survivor) is a no-op this slice** — there is no genuine survivor,
      only the same two documented equivalents and the one documented
      permanent timeout, so nothing was added or deleted.
      This slice's own new code was exercised and caught, not merely
      present: 6.1.73's guard, its `- 1`, and its `idx + 1` (`anga.rs:103`,
      `:106`, `:108`) are all in `caught.txt`, together with eight caught
      mutants touching `shcutva_of` / `saturating_sub` — the three risk
      areas the brief itself flagged (`shcutva_of`'s five unwitnessed
      arms, 8.4.40's `saturating_sub(1)`, and 6.1.73's non-equivalent
      `idx + 1` `+`→`*`) all came back caught, not survived.
    `outcomes.json`'s per-mutant test-phase durations (**n = 532** — 571
    mutants minus the 39 unviable ones, which fail at the Build phase and
    never reach a Test phase) put the median at 91.20s, p90 at 1070.91s
    (nearest-rank, as in 7e), p99 at 1299.80s, and the max at 4800.02s —
    that max being the known-permanent timeout itself, not a caught or
    missed mutant, exactly as in 7e. 64 of the 532 runs exceeded 600s.
    Excluding the timeout, the max is **1521.45s** (`tripadi.rs:1391`,
    `replace < with <=`, `CaughtMutant`) — within 0.3% of this slice's own
    pre-campaign projected ceiling of 1525.57s (1.43× × 1066.834s), which
    converts that contention model from a projection into a corroborated
    one rather than a coincidence.
    **Two margins, measured, not projected, and labelled:**
    - Against the worst **caught** mutant (1521.45s, measured): 4800 /
      1521.45 ≈ **3.15×**.
    - Against the worst **uncaught** run (1118.96s, measured — this
      slice's two missed mutants ran the full golden suite to completion
      without being caught, at 1118.96s and 1115.81s, both genuine
      uncaught `-j 4` runs, not projections): 4800 / 1118.96 ≈ **4.29×**.
    Consistent with 7e's own finding, the worst-case run this campaign
    was a **caught** mutant, not an uncaught one — scheduling overlap
    under `-j 4`, not catch/miss status, still dominates wall clock.
    **Ruling: keep 4800.** Both margins comfortably clear 1×, the
    caught-mutant margin (3.15×) lands almost exactly on this slice's own
    pre-campaign projection, and the one-time cost of a wrong-direction
    cap remains the same ~40 minutes 7e already paid for the permanent
    timeout, not a repeat of the vacuous-zero failure mode from 7a/7b.
  - `cargo-deny` + `cargo-audit` (supply-chain checks) — `mise run audit` runs
    `cargo audit && cargo deny check` and is expected to pass, including
    `cargo deny check advisories`.
  - `cargo-fuzz` (fuzzing of `panini-lipi`, target at `crates/panini-lipi/fuzz`)
    — pinned here, but real fuzzing still needs a **nightly** Rust toolchain,
    which is not provisioned in this environment; install nightly yourself.

## Rules of the codebase
- SLP1 is the only internal representation; transliterate only in `panini-lipi`.
- `#![forbid(unsafe_code)]` in every non-fuzz crate (the `panini-lipi` fuzz
  target under `crates/panini-lipi/fuzz` legitimately omits it, since it uses
  `#![no_main]` plus the libfuzzer harness macro).
- Grammar changes are gated by the golden paradigm test
  (`crates/panini/tests/paradigm.rs`, 2628 cells, six complete gaṇas plus
  rudhādi partial — `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (211 cells), a third (79 cells), a fourth (2
    cells, rudhādi's √piṣ and — new in slice 7d — √śiṣ loṭ madhyama eka) and
    — the loṭ parasmaipada cells of
    rudhādi's √kṛt, √rudh, √bhid, √kṣud, √tṛd and — new in slice 7d — √und,
    six ways tied as the
    sharpest forks in the suite — a fourth
    and fifth (prathama eka) or a fourth through sixth (madhyama eka) — in
    `ALTERNATES` (429 rows in all, so 2628 + 429 = 3057 forms total), and
    `derivation_set_is_exactly_pinned` asserts each cell's derivation set is
    exactly the union of the two. The suite is no longer filtered by any
    one-form-per-cell convention — the
    "retiring the conventions" slice retired the last two (7.1.35 tātaṅ,
    8.4.56 pausal cartva), and `PARADIGM`'s index 0 is now genuinely the
    declined derivation rather than a hand-picked citation form: prathama
    eka of laṅ and vidhiliṅ is the jaś form for parasmaipada roots
    (`aBavad`, `Baved`), since 8.2.39 *jhalāṁ jaśo'nte* is obligatory;
    bhvādi/divādi/
    tudādi are complete across laṭ/laṅ/loṭ/vidhiliṅ × parasmaipada/
    ātmanepada, and adādi (gaṇa 2) is now **complete** — √yā/√vā/√ad
    (parasmaipada) and √ās/√vas/√śī (ātmanepada) are complete across all four
    lakāras (laṭ/laṅ/loṭ/vidhiliṅ). √ad (parasmaipada) lands the internal
    junction sandhi cartva (8.4.55); √ās (ātmanepada) lands 7.1.5
    ātmanepadeṣv anataḥ and extends 6.1.90 āṭaś ca / 6.1.66 lopo vyor vali to
    the athematic (śap-luk'd) ātmanepada path (loṭ 1sg + optative); √vas
    (ātmanepada) is the second witness for 8.2.25 dhi ca, which elides an
    aṅga-final `s` before a Dh-initial affix (ADve, vaDve) — it replaced the
    8.4.53 jaśtva analysis slice 5d shipped, and 8.4.53 was removed as
    unreachable; √śī (ātmanepada) closes the gaṇa and lands 7.4.21 śīṅaḥ
    sārvadhātuke guṇaḥ (guṇa despite the ṅit ending — the gaṇa's only visible
    guṇa), 7.1.6 śīṅo ruṭ (Serate), and 8.3.59 ādeśapratyayayoḥ, the engine's
    first ṣatva (Seze, Sezva) — see
    `docs/superpowers/specs/2026-07-25-adadi-si-5f-design.md` for the full
    rule analysis; kryādi (gaṇa 9) is now **complete** — six roots across all
    four lakāras, the first gaṇa whose vikaraṇa (śnā) is itself reshaped by
    the ending. √kliś, √gudh, √aś (parasmaipada) landed in slice 9a; √muṣ,
    √vrī (parasmaipada) and √vṛṅ (ātmanepada) landed in slice 9b along with
    8.4.1 / 8.4.2, the engine's first ṇatva. √vṛṅ is the gaṇa's **only**
    ātmanepadī root — every other ātmanepada form in kryādi belongs to an
    ubhayapadī root, and no kryādi ubhayapadī root is curated. The
    ubhayapada slice landed 1.3.72 *svaritañitaḥ* (with rudhādi's √rudh),
    so the pada model no longer stands in their way; whether any given one
    needs phonology of its own is a per-root question nobody has asked yet —
    see `docs/superpowers/specs/2026-07-28-kryadi-gana-design.md`; svādi
    (gaṇa 5) is now **complete** — six roots across all four lakāras: √āp,
    √śak, √hi and √ri (parasmaipada), √aś (`05.0020`, distinct from kryādi's
    `09.0059`) and √ṣṭigh (`stiG`) (ātmanepada). Its vikaraṇa is śnu (3.1.73),
    and it is the first gaṇa where 7.3.84's guṇa lands on the vikaraṇa rather
    than the root: 7.3.84 now applies twice, once with respect to śnu and once
    with respect to the ending (1.4.13 makes the aṅga affix-relative), giving
    `Apnoti` against the ṅit-blocked `ApnutaH`. The other split running
    through the gaṇa is *asaṁyogapūrva* — whether śnu's `u` is preceded by a
    conjunct decides both the yaṇ alternation (6.4.87 / 6.4.77: `hinvanti`
    against `Apnuvanti`) and the hi-luk (6.4.106: `hinu` against `Apnuhi`) —
    see `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`.) rudhādi
    (gaṇa 7, vikaraṇa śnam) is **partial**, not complete — the first gaṇa
    described that way. Nine of its 25 dhātupāṭha roots are ubhayapadī
    (`~^`-marked); slice 7a lands three roots that need nothing beyond the
    gaṇa's own spine (√kṛt, √hiṃs — stored `hins` — and √khid), 7b adds
    three more, one per consonant family: √bhañj (cu-class final), √piṣ
    (ṣ-final) and √indh (jhaṣ-final, the gaṇa's second ātmanepada root),
    and the ubhayapada slice adds the gaṇa's own **eponym**, √rudh
    (`07.0001 ru\Di~^r`), with 1.3.72 *svaritañitaḥ* — the engine's first
    ubhayapadī root, deriving a full paradigm in each pada. Slice 7c then
    curated four more, taking the gaṇa from seven roots to **eleven**:
    √bhid (`07.0002 Bi\di~^r`), √kṣud (`07.0006 kzu\di~^r`), √yuj
    (`07.0007 yu\ji~^r`) and √tṛd (`07.0009 u~tfdi~^r`), all four
    ubhayapadī by 1.3.72 and all four pinned in both padas. The
    8.2.30/8.2.39 generalization slice then curated two more, taking the
    gaṇa to **thirteen**: √ric (`07.0004 ri\ci~^r`) and √vic
    (`07.0005 vi\ci~^r`), also ubhayapadī by 1.3.72 and pinned in both
    padas. Rudhādi 7d then curated eight more, on the audited numbers
    alone with no new sūtra, taking the gaṇa to **twenty-one**: √vid
    (`07.0013 vi\da~\`, ātmanepadī), √śiṣ (`07.0014 Si\zx~`), √und
    (`07.0020 undI~`), √añj (`07.0021 anjU~`), √tañc (`07.0022 tancU~`),
    √vij (`07.0023 o~vijI~`), √vṛj (`07.0024 vfjI~`) and √pṛc
    (`07.0025 pfcI~`), the other seven parasmaipadī and none of the eight
    ubhayapadī.
    Rudhādi 7e then curated the ninth and last reachable non-ubhayapadī
    root, taking the gaṇa to **twenty-two**: √tṛh (`07.0018 tfha~`), behind
    three new sūtras — 7.3.92 *tṛṇaha im* (the *im* augment), 8.2.31 *ho
    ḍhaḥ* and 8.3.13 *ḍho ḍhe lopaḥ* — and three widenings of rules the
    engine already had: 8.4.41 *ṣṭunā ṣṭuḥ* (its trigger widened from a
    bare `z` literal to the full ṣṭu class `z`/`w`/`W`/`q`/`Q`/`R`), 8.2.41
    *ṣaḍhoḥ kaḥ si* (widened from `z` alone to `z`/`Q`, the dvandva's other
    named sound) and 6.1.87 *ād guṇaḥ* (a second arm, for the `a i` the
    *im* augment leaves wholly inside `SHAP`, distinct from the junction
    arm's ending-initial `i`/`I`). **7d's own deferral undercounted the
    gap**: it named only the three sūtras √tṛh lacked outright, but three
    rules the engine already had were too narrow to carry the root as
    well — and 8.4.41's own guard comment, titled NARROW GUARD before this
    slice renamed it, had predicted exactly this widening. √tṛh's deepest
    cells — the ones every other stop-final rudhādi root turns into
    six-form forks (8.4.53 voices, 8.4.65 optionally elides, 7.1.35 and
    8.4.56 each multiply that by three) — hold only **three** forms,
    because 8.3.13 obligatorily elides the very ḍh that 8.4.65 would
    otherwise fork on; `tripadi.rs`'s comment on 8.3.13 and
    `trnaddhi_trace_has_8_3_13_and_no_8_4_65` in `panini`'s trace suite are
    the record. A standing divergence from vidyut-prakriya's own traces,
    not introduced by this slice: vidyut credits 6.1.68 *hal ṅyāb bhyo
    dīrghāt su-ti-sy-apṛktaṁ hal* for the apṛkta-`t` deletion every curated
    rudhādi root's laṅ takes (`akfRat`, `aBinat`, `apinaw`, `aBanak`, and
    now `atfReq`); this engine has no 6.1.68 and reaches the same surface
    through 8.2.23 *saṁyogāntasya lopaḥ* instead — audited clean either
    way, and predating √tṛh. And 6.3.111 *ḍhralope pūrvasya dīrgho'ṇaḥ* is
    deliberately unimplemented: it lengthens a preceding *aṇ* before the
    very ḍh-elision 8.3.13 performs, but no √tṛh cell presents one there
    (the elided ḍh always follows `e` or `M`), and vidyut-prakriya's own
    traces do not emit it either — the reason is recorded in place at
    8.3.13, not merely deferred.
    `curated_pada_agrees_with_upadesha_markers` in `panini-data` now
    re-derives all 64 verdicts from the vendored upadeśa, so the column
    cannot drift from the data that determines it. That discharges
    the **ubhayapada** deferral as such: 1.3.72 is no longer what keeps any
    root out, and the **two** ubhayapadī roots still outside the curated
    set are out for narrower, root-specific reasons, verified cell by cell
    against vidyut-prakriya.
    The pada audit added two more: `01.1049 RI\Y` (√nī, bhvādi) and
    `06.0001 tu\da~^` (√tud, tudādi), both ubhayapadī by 1.3.72 and both
    curated parasmaipada until then. √tud was a known deferral; √nī was
    named by no deferral list and was read past by every slice from v1 on.
    **√bhid, √kṣud, √yuj and √tṛd were called "curation-only" for months
    with no run behind the claim; 7c ran it.** None of the four needed a
    new sūtra, and the whole-corpus cross-implementation audit of
    2026-08-17, against vidyut-prakriya at commit
    `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, found **zero differences
    across all 2160 cells / 2496 forms / 53 roots** — with the `entry`
    negative control **verified failing first** (exit 1, 36 √bhū cells
    flagged), so the zero is not vacuous. Byte-identity to vidyut is now a
    sourced result rather than an assertion.
    **The 8.2.30/8.2.39 generalization slice ran the harness twice.** The
    first run, against vidyut-prakriya at the same `8da2f90` commit, found
    **four differing cells** — √ric's and √vic's declined laṅ **parasmaipada**
    prathama and madhyama eka — which is what exposed 8.2.39's narrow
    guard as a real defect rather than a documented deferral: real code,
    not only the synthetic `entry` control, tripped the harness. After
    8.2.39 was generalised, the second run came back clean: **zero
    differences across all 2304 cells / 2654 forms / 55 roots**, with the
    `entry` negative control verified failing first (36 √bhū cells) both
    times.
    **Rudhādi 7d's own cross-implementation audit** ran the same probe
    against vidyut-prakriya at commit `8da2f90`, over the grown corpus:
    **zero differences across all 2592 cells / 3014 forms / 63 roots**,
    with the `entry` negative control verified failing first. No `Rule`
    changed — the eight roots derive on the rules already in the pipeline.
    **Rudhādi 7e's own cross-implementation audit** ran the same probe
    against vidyut-prakriya at commit `8da2f90`, over the corpus grown by
    √tṛh: **zero differences across all 2628 cells / 3057 forms / 64
    roots**, with both negative controls (`entry` and `form`) verified
    failing first. This is the first of these audits with new `Rule`s
    behind it: 7.3.92, 8.2.31 and 8.3.13, plus the two tripādī widenings
    above (8.4.41 and 8.2.41) — an earlier task in this slice had already
    proved those two widenings inert on the pre-7e 2592-cell corpus by a
    byte-for-byte dump diff of its own, before √tṛh was curated at all.
    6.1.87's second arm was not part of that dump diff: it landed later,
    and is inert by construction rather than by measurement — it is
    gated on `7.3.92` appearing in `p.log`, so no pre-7e derivation can
    reach it, and the residual risk is what the 2628-cell audit covers.
    **√ric and √vic** needed no new sūtra, but 8.2.30 *coḥ kuḥ* needed more
    than the one-line guard widening it looked like: they are c-final, and
    the rule was hardcoded to a single `j` → `g` pair — its match read `j`
    alone AND its substitute was a literal `'g'`, while its comment claimed
    a 1.1.50 *sthāne'ntaratamaḥ* nearest-velar substitution (voicing and
    aspiration preserved) that the code did not implement. Widening the
    match alone would have reached the right surface (`riRakti`, since
    8.4.55 *khari ca* devoices the resulting `g` to `k` before `ti`) but
    through a wrong intermediate (`riRagti`), so both match and substitute
    now read one `kutva_of` map (cu → ku) instead — the substitute *is* the
    map, not a case split. That fix exposed 8.2.39 *jhalāṁ jaśo'nte* as
    narrower than it looked: its three-literal guard (`t`/`z`/`D`) had
    never had to classify a voiceless word-final velar, because no curated
    root had produced one before √ric and √vic did. 8.2.39 now reads a
    `jashtva_of` map on both sides too, plus a no-op guard for the table's
    fixed points, so √ric's and √vic's declined laṅ prathama and madhyama
    eka reach `ariRag`/`avinag` (jaśtva-voiced), with 8.4.56 *vā'vasāne*
    supplying the optional `ariRak`/`avinak` — the same √bhañj-pattern fork
    √yuj's `ayunag`/`ayunak` already witnesses. **√chid and √chṛd** need two
    sūtras the engine does not have — 6.1.73 *che ca*, the tuk augment
    before a `C` after a short vowel, and 8.4.40 *stoḥ ścunā ścuḥ*, the
    ścutva that follows it — without which their laṅ cells surface
    `aCinat` for `acCinat`. Rudhādi 7d then curated eight of the nine
    remaining reachable non-ubhayapadī roots — √śiṣ, √und, √añj, √tañc,
    √vij, √vṛj, √pṛc and √vid — on the audited numbers alone, with no new
    sūtra: each exercises machinery already in the pipeline — 6.4.23 *śnān
    nalopaḥ* then 6.4.111 *śnasor allopaḥ* for √und, √añj and √tañc (√und
    additionally taking 6.4.72 *āṭ* with 6.1.90 *āṭaś ca*), 8.2.30 *coḥ
    kuḥ* (kutva) for √añj, √tañc and √pṛc, 8.4.1 *raṣābhyāṁ no ṇaḥ* (ṇatva)
    for √vṛj and √pṛc, 8.4.41 and 8.2.41 for √śiṣ, and 8.4.65 *jharo jhari
    savarṇe* for √vid — plus two SLP1 surface collisions, which number
    keying makes moot, rather than needing anything new. (vidyut-prakriya
    credits 6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti* for √und's `unad → und`
    step; this engine rejects that credit, does not implement 6.4.24 at
    all, and pins the rejection in `trace.rs`.) That left √tṛh as the
    ninth and only reachable non-ubhayapadī root still out, deferred to
    slice 7e behind three sūtras the engine did not implement: 7.3.92
    *tṛṇaha im* (the *im* augment), 8.2.31 *ho ḍhaḥ* and 8.3.13 *ḍho ḍhe
    lopaḥ* — see the 7e paragraph above for what curating it actually
    took. √bhuj (`07.0017 Bu\ja~`)
    is the twenty-fifth entry and out on
    different grounds again: 1.3.66
    *bhujo'navane* forks its pada on **sense**, not on an axis this engine
    models. 22 curated + 2 uncurated ubhayapadī + √bhuj = 25,
    so **3 of the 25 remain out**.
    The root count is not what keeps the gaṇa partial — twenty-two is well
    past the six every completed gaṇa *after bhvādi* has here (bhvādi,
    the first, has twelve) — and neither, any longer, is 1.3.72: what
    remains is the two-sūtra gap that keeps √chid and √chṛd out, and
    √bhuj's sense axis. √indh's pada was **verified, not inferred from its
    ñi**: `YiinDI~\`'s ñi it-marker is one of the two things 1.3.72 reads,
    which would have made the root ubhayapadī alongside √rudh, so it was
    checked against vidyut-prakriya — which derives √indh in ātmanepada
    only, against a `~^r` control (√rudh) that does derive both padas — and
    the root's own anudātta settles its pada by 1.3.12 *anudāttaṅita
    ātmanepadam*. śnam is the engine's first **infix**: unlike every other
    vikaraṇa it is not a suffix, and the pipeline's fixed
    `[ANGA, SHAP, ENDING]` slots have
    nowhere to put one, so 3.1.78 splits the root across the first two
    instead — `terms[SHAP].text` for rudhādi is śnam followed by the root's
    own tail, not the vikaraṇa alone (`kft` → `[kf, nat, ti]`); see the
    "REPRESENTATION" note on 3.1.78 in `tinanta/vikarana.rs` and the caveat
    in `tinanta/terms.rs`. 8.4.53 *jhalāṁ jaś jhaśi* was restored in 7a,
    with `kfndDi` as its witness, after `9fa8e5f` removed it as
    unreachable — 8.2.25 *dhi ca* bled every path that used to reach it, but
    √kṛt's stem-final `t` (not an `s`) is genuinely jaśtva's; 7b generalised
    its guard from the one shape 7a's witnesses reached it through (a
    word-final `Di`) to the sūtra's own condition — any jhal before any
    jhaś, anywhere in the word — which is what carries √piṣ's loṭ madhyama
    eka to `piMqQi` (`piRqQi` after 8.4.58) and lets √indh's mid-word `D`s
    reach the rule at all. 7b's own four sūtras are one per family: 8.2.30
    *coḥ kuḥ* (√bhañj's `j` → `g`), 8.4.41 *ṣṭunā ṣṭuḥ* and 8.2.41 *ṣaḍhoḥ
    kaḥ si* (√piṣ), and 8.2.40 *jhaṣas tathor dho'dhaḥ* (√indh). √rudh
    needed no new phonology of its own: 1.3.72 aside, the ubhayapada slice
    only widened 8.2.39 *jhalāṁ jaśo'nte*'s guard by exactly one arm (`D`),
    which is what makes `aruRad` derivable — and, through 8.2.75 *daś ca*'s
    own `ends_with('d')` guard, the `aruRaH` branch too. (That `t`/`z`/`D`
    three-literal guard was not the end of the story: the 8.2.30/8.2.39
    generalization slice later replaced it with a `jashtva_of` map, once
    √ric and √vic exercised a shape it could not classify — see above.)
    The vikalpa
    set is unchanged at **seven** rules, in pipeline order: 7.1.35, 3.4.111,
    6.4.107, 8.2.74, 8.2.75, 8.4.65, 8.4.56 — 7b is the first gaṇa slice
    since the `vikalpa` flag landed (`53e03e7`) to add none, and the
    ubhayapada slice adds none either: 1.3.72 is deliberately **not**
    optional, because a root's two padas are two cells, not two branches of
    one cell; kryādi and svādi predate the flag rather than having declined
    to use it. Four
    orderings in the tripādī are deliberate, and they differ in what they
    pin. **8.2.74 and 8.2.75 above 8.2.73**, against sūtra order, are
    *derivation* constraints — both replace the dhātu's own final, and
    8.2.73 manufactures a `d` from that final, so 8.2.74 run below it would
    find `d` where it needs `s` and never derive `ahinaH`; reversing 8.2.74
    and 8.2.73 was tried and empirically fails four tests, and the order is
    pinned by `shnams_ru_fires_on_the_dhatus_own_final` plus
    `tinanta_rule_order_is_pinned` (7b moved 8.2.75 above 8.2.73 for the
    same structural reason, which made its `p.log` read unreachable and let
    it be deleted). **8.2.41 below
    8.2.23** is a *derivation* constraint too, and the sharpest one this
    slice adds: at laṅ madhyama eka the ending is a bare `s`, and 8.2.23
    *saṁyogāntasya lopaḥ* elides it before 8.2.41 can see it, so the cell
    reduces exactly as laṅ prathama eka does. Reversed, √piṣ surfaces
    `apinak` instead of `apinaq`/`apinaw` — a real-looking form that splits
    madhyama eka from prathama eka and that no guard test would flag; only
    `shadhoh_kah_si_declines_when_8_2_23_ate_the_s_first` and
    `apinaq_trace_pins_8_2_23_above_8_2_41` catch it.
    **8.4.41 above 8.4.53** is sūtra order, and it is
    load-bearing *as this engine implements them*: 8.4.41's trigger set is
    narrowed to `z` alone, so reversed, 8.4.53's `z → q` jaśtva consumes
    the trigger first and √piṣ stalls at `piMqDi`. Widening that trigger to
    the ṭ-varga stops the sūtra also names (`w W q Q R`) would restore
    convergence and make the placement sūtra order only. **8.4.65 above
    8.4.56**, by contrast, is only a *trace-order* constraint — both
    orderings derive the same six forms for the cell that exercises them,
    and the wrong order only changes which intermediate form each optional
    branch's trace passes through
    (`krntat_trace_shows_savarna_elision_above_pausal` in
    `crates/panini/tests/trace.rs` is the sole pin; no surface-form golden
    catches a reversal).
  and by the ordered-trace test (`crates/panini/tests/trace.rs`), which pins
  rule order. Surface forms and trace order there are the source of truth;
  sūtra ids/names in traces must match the cited reference. In practice that
  reference is vidyut-prakriya's machine-readable `data/sutrapatha.tsv`
  (ashtadhyayi.com is a JS single-page app that cannot be fetched
  programmatically), and that is what specs, plans, and verification in this
  repo actually check ids/names against.
  Each gaṇa slice also runs a **whole-corpus cross-implementation audit**
  against that same vendored checkout, comparing derivation sets cell by cell.
  The harness is `tools/audit/panini_full_audit.rs`; it is a `vidyut-prakriya`
  example rather than a workspace member (it depends on both engines), so
  `tools/audit/README.md` covers copying it into a vidyut checkout and running
  it, including the negative controls that must pass before a zero-difference
  result means anything. It lived out-of-repo until 2026-08-16, which cost
  three slices a from-scratch rebuild apiece (7b: 1728 cells, 1941
  forms, zero differences; the ubhayapada slice audited its own addition —
  √rudh's 72 cells, split per pada via `Tinanta::builder().pada(...)`, at
  vidyut commit `8da2f90`, plus the negative that vidyut derives √indh in
  ātmanepada only against √rudh as the `~^r` control — and the corpus it sat
  in then stood at 1872 cells and 2114 forms). The pada audit ran the harness
  after it was already committed in-repo, needing no rebuild, and its own
  full-corpus run found the same **zero differences across 1872 cells / 2114
  forms / 49 roots** at vidyut commit `8da2f90`, with both negative controls
  verified failing first. **Slice 7c re-ran that same committed harness over
  the grown corpus: zero differences across 2160 cells / 2496 forms / 53
  roots**, at vidyut commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, with
  the `entry` negative control **verified failing first** — exit 1, 36 √bhū
  cells flagged. **The 8.2.30/8.2.39 generalization slice ran it twice.**
  The first run, at the
  vidyut commit, found four differing cells — √ric's and √vic's declined
  laṅ **parasmaipada** prathama and madhyama eka — the first time this
  harness caught a real defect in real code rather than only the synthetic
  `entry` control; that is what exposed 8.2.39's guard as too narrow. After
  8.2.39 was generalised alongside 8.2.30, the second run came back clean:
  **zero differences across 2304 cells / 2654 forms / 55 roots**, with
  `entry` verified failing first both times. **Rudhādi 7d re-ran the same
  committed harness once more, at the same vidyut commit `8da2f90`, over
  the corpus grown by its eight roots: zero differences across 2592 cells
  / 3014 forms / 63 roots**, with `entry` verified failing first — the
  first of these runs with no `Rule` diff behind
  it at all: the eight roots derive on the rules already in the pipeline.
  **Rudhādi 7e re-ran the same committed harness once more, at the same
  vidyut commit `8da2f90`, over the corpus grown by √tṛh: zero differences
  across 2628 cells / 3057 forms / 64 roots**, with both `entry` and
  `form` negative controls verified failing first — the current record,
  and the first of these runs with new `Rule`s behind it: 7.3.92, 8.2.31
  and 8.3.13.
  Those
  totals are asserted by the harness itself rather than reported from
  whatever it happened to enumerate, so a corpus that grows without the
  harness being updated fails loudly instead of quietly auditing a subset.
  The harness resolves each root
  to a `data/dhatupatha.tsv` entry by its **dhātupāṭha number** (`07.0016` for
  √bhañj), which is `Dhatu::dhatupatha` and the root's identity in this repo.
  That closed the one circularity this audit used to carry: selection
  previously required vidyut to reproduce **this engine's own pinned laṭ
  prathama eka form**, so for a root whose new sūtra shaped exactly that cell
  — √bhañj's `Banakti`, √piṣ's `pinazwi`, √indh's `indDe` — the anchoring cell
  was the one cell the audit could not independently validate. The numbers
  themselves are held honest in-repo by `dhatupatha_numbers_resolve_upstream`,
  which it-strips each vendored upadeśa (1.3.2, 1.3.5, 1.3.3, then 6.1.64 and
  6.1.65) and compares it against the stored `code` — an assertion that cannot
  be satisfied by copying back our own choice, unlike matching on number or
  artha alone (upstream has 8- and 15-way artha collisions).
  Two comments inside `crates/panini-prakriya/src` still carry pre-7c
  figures — `controller.rs:130` and `tinanta/guna.rs:943` cite the corpus
  size as 1872/1864-of-1872, now three slices further stale: the corpus
  stood at 2304/2296-of-2304 as of the 8.2.30/8.2.39 slice, stood at
  2592/2584-of-2592 as of rudhādi 7d, and stands at 2628/2620-of-2628 as
  of rudhādi 7e — the same 8
  cells 6.4.107 always fired on (`key_count("6.4.107") == 8`, pinned at
  `paradigm.rs:5444`), unmoved by 7d or 7e since 6.4.107 concerns only
  svādi's
  √hi and √ri. Rudhādi 7d touched neither comment — its one permitted
  engine-comment edit is the comment above
  `vrddhi_of_ac_vowels_all_arms` in `tinanta/sound.rs`. Rudhādi 7e touched
  neither comment either. A third,
  `tinanta/tripadi.rs`'s comment on 8.2.30 (formerly the one calling √bhañj
  rudhādi's one cu-final curated root), was **not** left stale the same
  way: the 8.2.30/8.2.39 generalization slice rewrote it in place, since
  generalising that rule past a single hardcoded root was the whole point
  of that slice, and its diff to `panini-prakriya` was never meant to stay
  empty the way 7c's was. The remaining two were left as-is deliberately by
  7c: 7c's central claim was a byte-identical, engine-untouched slice, and
  its mutation gate's validity depended on that package's diff staying
  empty, so no comment inside it was touched even to fix drift, at the
  time. This note is the record of that deferral, now down to two
  comments.
- New grammar goes in `TINANTA_RULES` as a self-guarding `Rule`, not as a
  branch inside `derive`. `TINANTA_RULES` is a list of seven stage arrays,
  each living in its own file under `crates/panini-prakriya/src/tinanta/`; add
  the rule to the stage its pipeline position falls in, and add its id to
  `tinanta_rule_order_is_pinned` in the same position. Which stage a rule
  belongs to is decided by its position relative to **3.1.68**, not by its
  sūtra family: rules before
  3.1.68 address the ending as `ENDING_PRE_SHAP` (index 1), rules after it as
  `ENDING` (index 2), and `terms[SHAP].text` may be empty for adādi. See
  `tinanta/terms.rs`. Per-rule guard tests go beside the rule in its stage
  file; tests asserting a surface form or trace go in
  `tinanta/derivation_tests.rs`. **Write a per-rule guard test where the
  rule's precondition can be built directly on a hand-built `Prakriya`.
  Where it cannot — because only an upstream rule chain produces that
  state — cite the covering derivation or trace test in the rule's own
  comment instead.** That is narrower than "every rule gets a guard test",
  and it is not the blanket exemption 7a's deferred #5 asked for ("per-rule
  guard tests for tripādī rules are not achievable"): `tripadi.rs` carries
  eighteen of them today, including
  `jhalam_jasho_ante_fires_on_any_pada_final_jhal_jashtva_of_resolves` and
  `va_avasane_fires_only_on_a_pada_final_jhal`. Whole-word scope is not what
  blocks a guard test; an unconstructible precondition is. `derive` carries
  no grammar branches: the only gana-conditioned logic there is aṅga tagging
  (`Tag::Adadi` &c.), which feeds the guarded rules rather than substituting
  for them. Fixtures shared across `crates/panini/tests/*.rs`
  integration-test binaries (e.g. `CELLS`, `LAKARA_BY_NAME`) go in
  `crates/panini/tests/common/mod.rs`, `mod`-included by each file that
  needs them — do not redefine them per test file.
- **Optional (*vikalpa*) rules set `Rule.vikalpa = true`.** `run_pipeline`
  forks there: it clones each live branch, applies to the clone, and keeps
  the clone only if `apply` returned true, so a rule that declines its own
  guard forks nothing. The declined branch keeps its index and the applied
  clone is inserted immediately after it, which is why index 0 of a
  derivation is always what the engine would have produced with no optional
  rules at all. `derive` therefore returns `Vec<Prakriya>`, and a cell may
  have more than one valid form. Add an optional rule exactly as any other —
  in its stage file, with its id in `tinanta_rule_order_is_pinned` in
  position — and also add it to
  `exactly_the_pinned_vikalpa_rules_are_optional`, which pins the whole
  optional set by id. **Seven rules are optional today, in pipeline order:
  7.1.35, 3.4.111, 6.4.107, 8.2.74, 8.2.75, 8.4.65, 8.4.56.** 7.1.35 and
  8.4.56 can both fire on one derivation, stacking into a three-branch
  cell — loṭ prathama eka forks twice, giving `Bavatu` / `BavatAd` /
  `BavatAt`. Five rudhādi roots — √kṛt, √rudh, √bhid, √kṣud and √tṛd — each
  stack three of the seven (7.1.35,
  8.4.65, 8.4.56) on their own loṭ parasmaipada cells — five branches at
  prathama eka, six at madhyama eka
  (`kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` /
  `kfntAt`, and `rundDi` / `runDi` / `rundDAd` / `runDAd` / `rundDAt` /
  `runDAt`, and likewise for √bhid, √kṣud and √tṛd) — because 8.4.56 only
  reaches the two tātaṅ (7.1.35) branches,
  not the two vowel-final ones. √yuj, ubhayapadī like those last three but
  not dental-final, stops at three forms in the same two cells
  (`yunaktu`/`yuNktAd`/`yuNktAt`, `yuNgDi`/`yuNktAd`/`yuNktAt`): 8.2.30 *coḥ
  kuḥ* replaces its stem-final palatal `j` with the velar `g` (8.4.55 *khari
  ca* devoices that to `k` before the `t` of tātaṅ, which is where `yuNktAd`'s
  `k` comes from), so the junction 8.4.65 would need is velar against dental
  — `g` + `D`, then `k` + `t` — and never savarṇa the way the dental-final
  roots' `d` + `D` and geminate `t` + `t` are. 8.4.65's site never arises. See
  `docs/ARCHITECTURE.md`'s branch-count
  paragraph for the full accounting.
- **An optional rule's position relative to its consumers depends on what its
  mutation does to the predicates they read — the operative question is not
  "does a consumer read what I wrote?" but "does my mutation make the
  predicate lie?".** Nothing enforces either direction.
  - If the mutation **destroys the evidence** for a predicate without
    changing the underlying grammatical fact, the rule must sit **after**
    every such consumer, or a consumer placed below it would be right on one
    branch and wrong on the other — surfacing as half a paradigm being wrong
    with both halves individually plausible. 6.4.107 leaves
    `terms[SHAP].text == "n"`, which invalidates two predicates:
    `shnu_asamyogapurva` (whose first guard is `== "nu"`) and
    `sound_before_ending` (which reads the last char before the ending — `u`
    before the mutation, `n` after) — the vikaraṇa *is* still śnu, but
    nothing downstream can tell any more. Every rule that reads śnu's `nu`
    text must precede it — 6.4.87 and 6.4.106 via `shnu_asamyogapurva`, and
    6.4.77, which open-codes the same `text == "nu"` test — and all three do.
    `sound_before_ending`'s one consumer below 6.4.107, 6.4.101 (`her DiH`,
    `crates/panini-prakriya/src/tinanta/adesha.rs`), is the exception a
    provably disjoint guard covers: it requires `ENDING.text == "hi"`, which
    6.4.107 already excludes by requiring an m- or v-initial ending, so the
    two rules never contend and 6.4.101 is safe where it sits.
  - If instead the mutation **changes the fact itself**, the rule must sit
    **before** every such consumer, so they read the new value rather than a
    stale one. 7.1.35 replaces the ending `tu`/`hi` with tātaṅ, and the
    ending genuinely is no longer `hi` — a consumer below gets the *right*
    answer, one above gets a stale one. 3.1.83 *halaḥ śnaḥ śānac ca*, 6.4.105
    *ato heḥ*, and 6.4.106 *utaś ca* all read `ENDING`/`ENDING_PRE_SHAP`'s
    text directly, and 7.3.84's second (ending-relative) application reads
    its ṅitva, so all four must sit below 7.1.35 to see the tātaṅ shape
    rather than the pre-mutation `hi` — kryādi's tātaṅ branch would surface
    `kliSAnatAt` instead of `kliSnItAt` if 3.1.83 ran first. 7.1.35 is
    ordered above all of them, at the end of the tiṅ stage, and nothing
    enforces that but the `kliSnItAt` trace pin.
- **7.3.84 and 1.2.4 each appear twice in `TINANTA_RULES`, by design — do not
  "deduplicate" them.** 1.4.13 *yasmāt pratyayavidhis tadādi pratyaye'ṅgam*
  makes the aṅga affix-relative, and a derivation with a live vikaraṇa has
  two affixes for these rules to apply with respect to: once for the
  vikaraṇa, once for the tiṅ ending. Svādi is where this became visible for
  7.3.84 (`Apnoti` needs the vikaraṇa-relative application; `ApnutaH` blocks
  it because `tas` is ṅit) but 1.2.4's second entry predates it. See
  `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`'s "7.3.84
  *sārvadhātukārdhadhātukayoḥ*, second application" section.
- The `panini-cli` binary has a single subcommand, `check` (flags `--trace`,
  `--json`, `--out`, `--in`). There is no `derive` subcommand in v1. `--in auto`
  (the default) auto-detects the input transliteration scheme; passing an
  explicit `--in` scheme (`slp1`/`iast`/`hk`/`deva`) makes that scheme
  authoritative, overriding auto-detection.
- **When a rule's substitute set widens, re-check every downstream guard
  that enumerates literals over the sounds it can now emit, in the same
  slice.** The 8.2.30/8.2.39 generalization slice paid for skipping this:
  once 8.2.30 read `kutva_of` instead of a hardcoded `g`, its output
  alphabet gained a voiceless `k`, and 8.2.39's `t`/`z`/`D`-only literal
  guard had never seen a word-final voiceless velar because nothing had
  ever produced one. Six `NARROW GUARD` sites remain in `tripadi.rs` and
  `anga.rs` today — each is a deliberate, commented narrowing, and each is
  a standing instance of this same hazard for the next slice that widens
  what feeds it.

## Where things live
See `docs/ARCHITECTURE.md`.
