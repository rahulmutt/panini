# Contributor & agent guide

## Environment
- Toolchain is pinned via `mise` (`mise install`) to rust 1.99.0. Do not install
  Rust globally.
- Tasks: `mise run build | test | lint | fmt | fmt-check | mutants | audit`.
- `mise run test` is the whole suite, including the exhaustive roundtrip:
  every derived form goes through the real `Panini::check()` and is compared
  against a brute-force oracle (`crates/panini/tests/roundtrip.rs`). It
  takes a few seconds, because `panini_analyze::candidates()` answers from a
  corpus index built once per process.
- Optional dev/audit tooling is pinned in `mise.dev.toml`. Install it on demand:
  `MISE_ENV=dev mise install`. This provides:
  - `cargo-mutants` (mutation testing) — `mise run mutants` runs
    `cargo mutants --package panini-prakriya --package panini-analyze
    --test-workspace=true --timeout 17350 -j 4`. Run the gate through the task
    rather than reconstructing the flags. The `--test-workspace` flag is
    required so each **mutant** run exercises the `panini` crate's golden
    paradigm/trace/roundtrip tests, not just the mutated packages' own unit
    tests — but it does NOT apply to
    cargo-mutants' own **baseline** run, which always exercises only the
    mutated packages' tests regardless of the flag. The explicit `--timeout`
    is still required: cargo-mutants calibrates its per-mutant timeout from
    the baseline's runtime (`panini-prakriya`'s unit tests, ~2s, with a 20s
    floor), and that auto-calibrated value does not guarantee the 5x margin
    over a full uncaught `panini` suite run (10.8-23.4s as measured in 3f3: 10.81/10.79s in the isolated probe, 13.99/23.44s under campaign load) under `-j 4` load.
    **The cap must clear a full UNCAUGHT run of the workspace suite at the
    parallelism you actually use.** Under a cap that doesn't, a mutant that
    survives is recorded as a **timeout rather than a survivor**, so a
    reported zero-survivor run that checks only `missed.txt` is vacuous
    instead of clean. Always check `timeout.txt` alongside `missed.txt`.
    This repo has hit it twice: slice 7a through suite growth (`--timeout
    300`, 9 timeouts) and slice 7b through parallelism (`-j 16`, 43
    timeouts). `cargo mutants` also reads `-j` from `CARGO_MUTANTS_JOBS`, so
    an unqualified cap can be defeated by the environment alone; keep `-j`
    at or below 4, or re-measure and raise the cap in step.
    **The floor behind the 17350s cap, measured at 40824 cells on Rust
    1.99.0, 2026-10-08.** Two `mise run test` runs took 6m34.689s and
    6m58.999s wall clock (user CPU 15m27.5s and 15m4.9s; sys 2.7s and 4.1s),
    under MODERATE EXTERNAL LOAD that was not ours (load averages
    `45.41 54.71 78.61` before the first, `68.72 68.38 77.13` after it and
    before the second, `50.68 61.41 71.07` after it, on 24 cores; no
    cargo-mutants of ours running; a stray external `python3` had held a core
    for days). The wall clocks are therefore NOT comparable with 10l's
    2m46.688s / 2m47.456s at 38160 cells on a QUIET host (load 3.73-11.20,
    user 8m38.8s / 8m43.5s, sys 0.42s / 0.41s); the user CPU is again about
    15m, as under svādi 5b's heavier load, so contention rather than the
    corpus (cells grew 2.7% from svādi 5b's 39744) sets it. They continue
    svādi 5b's 15m28.365s / 11m34.391s at 39744 cells (load 94-141, user
    15m14.2s / 15m15.7s, sys 4.8s / 4.4s), the other heavy-load floor, and 10m's
    12m41.146s / 7m42.564s at 38232 cells (load 19.53-109.07, user 14m1.8s /
    14m8.6s), and 10k's user 12m50.9s / 12m27.1s
    under load 63-122 (wall 9m31.852s / 6m9.420s at 37584 cells, load
    `122.08 95.91 75.31` to `63.16 82.28 85.55`). Nor are they comparable
    with 10j's 7m6.732s / 4m29.176s at 26424 cells (load 65-90, user 9m13.3s
    / 9m11.3s) or its 2m13.130s on a quieter host (load 11-13, user
    6m44.2s), 10i's 2m16.234s / 2m12.659s at 25956 cells (load 15-29, user
    6m33.7s / 6m25.4s), 10h's 1m27.729s / 2m20.140s at 21564 cells (load
    13-61, user 4m24.7s / 5m28.1s), 10g's 1m45.672s / 1m40.061s at 17964
    cells (load 56-64), 10f's 47.778s / 43.068s at 13716 cells (load 29-36),
    10e's 1m19.537s / 51.668s at 12996 cells (load 73-106), 10d's 16.860s /
    15.610s at 6696 cells, 10c's 16.008s / 15.686s at 6264 cells, 10b's
    23.322s / 22.548s at 5076 cells, 10a's 10.336s / 9.781s at 4932 cells,
    the 9.077s / 8.855s at 4644 cells, the 7.848s / 7.864s at 4608 cells,
    the 8.019s / 8.348s at 4572 cells on 1.98.1, or the 5.418s / 5.419s at
    4428 cells. No quiet-host floor was taken this slice; 10l's is the last
    one. An isolated `-j 4` probe (`-o` to
    `/home/dev/mutants-records/kryadi-9c-probe`, `--timeout 30000`,
    2026-10-08 09:21:21 - 11:13 UTC, load `49.47 60.79 70.76` at launch and
    `36.98 45.00 52.90` at the end) of the two documented equivalent
    mutants and the two `skip_nic` mutants (the regexes also match two
    caught `mod.rs` `derive` mutants, `mod.rs:83` 15.81s and `:84` 71.59s)
    ran the full suite uncaught in 772.14s (`adesha.rs:649:30`) and 829.59s
    (`tripadi.rs:1416:38`); both MISSED (svādi 5b's probe: 844.97s /
    790.25s; 10m's: 662.09s / 669.64s; 10l's, on a quiet host: 242.81s /
    242.16s; 10k's, under load 59-85: 450.11s / 459.65s; 10j's: 315.46s /
    309.36s; 10i's: 188.69s / 188.85s; 10h's: 160.99s / 160.28s; 10g's:
    160.20s / 163.19s; 10f's: 59.32s / 58.53s). In the same probe
    `sanadi.rs:60:5: replace skip_nic -> bool with true` was CAUGHT in
    6674.37s and `sanadi.rs:60:39` (`!=` → `==`) in 5720.63s (svādi 5b's
    probe: 8673.59s / 6324.00s; 10m's: 6759.15s / 5547.69s; 10l's:
    2242.64s / 1883.83s; 10k's: 4617.70s / 3695.74s; 10j's: 3885.06s /
    2740.27s). Under campaign load the uncaught phases were 502.35s
    (`adesha.rs:649:30`) and 445.75s (`tripadi.rs:1416:38`), both MISSED at
    17350 (svādi 5b's campaign-load phases: 982.79s / 442.61s; 10m's:
    388.14s / 457.45s; 10l's: 349.19s / 341.93s; 10k's: 411.94s / 260.17s;
    10j's: 298.02s / 250.01s; 10i's: 208.34s / 186.62s; 10h's: 142.46s /
    171.58s; 10g's: 300.16s / 107.56s). **The equivalents do not set the
    cap.** It must also clear the slowest CAUGHT mutant's test phase under
    campaign load, because a caught mutant that makes the derivation blow up
    runs far longer than a full uncaught suite before any assertion can
    fail: `skip_nic -> bool with true` makes all eight optional-ṇic vikalpa
    rules (10.0498, 10.0499, 2564, 2565, 2570, 2571, 2573.1, 2573.3) fire on
    every root, so every derivation forks 2^8 ways, and its sibling `!=` →
    `==` does the same on every root that is not the rule's own row. 10j's
    6.1.54 is a vikalpa too, but not an optional-ṇic one: it is keyed on
    √ci's rows (`CISPHUR`) and forks only those. Slice 9c adds no
    optional-ṇic rule, so the fork count is unchanged at 2^8, and the corpus
    grew 2.7% (40824 cells against 39744). In the campaign the `true` mutant
    was **CAUGHT in 6895.87s** and the `==` mutant in 4904.51s, SLIGHTLY
    SLOWER than the probe's 6674.37s / 5720.63s on `true` (+3.3%) and faster
    on `==` (-14%) (svādi 5b: campaign 43% and 39% FASTER than its probe;
    10m: 21% and 23% faster; 10l: the campaign was 2.5x and 2.0x SLOWER than
    its quiet-host probe; 10k: 38-40% faster; 10j: 3232.46s / 2287.81s; 10i:
    TIMEOUT at 1810 in the campaign, CAUGHT in 1559.33s rerun alone at `-j
    1`; 10h: 469.47s / 322.20s with 2^5 forks). The host was quieter during
    this campaign (load 13-50 against svādi 5b's 100-157), yet the `true`
    mutant ran 40% LONGER than in svādi 5b's campaign (4908.95s), which
    shows how little the launch load predicts it. The cap is max(430, 6 ×
    the longest campaign-load equivalent phase, 2 × the longest caught
    phase), rounded up to the next 10s, over both readings of the `true`
    mutant in the same `-j 4` side-by-side setup: the probe's 6674.37s and
    the campaign's 6895.87s, and the cap is never lowered on the quieter
    reading nor below the cap the campaign ran at. 2 × 6895.87s = 13791.74s
    (rounds to 13800), 2 × 6674.37s = 13348.74s, and 6 × 502.35s = 3014.10s
    (6 × 829.59s = 4977.54s for the probe's equivalents); the largest is
    13800, below the 17350 the campaign ran at and that svādi 5b set from
    its loaded probe (2 × 8673.59s = 17347.18s), so **the cap stays 17350**
    and `mise.toml` is unchanged. Margins at 17350: 2.52x over the
    campaign's `true` 6895.87s, 3.54x over its `==` 4904.51s, 2.60x over the
    probe's `true` 6674.37s, 3.03x over the probe's `==` 5720.63s, 34.5x
    over the longest campaign-load equivalent's 502.35s. svādi 5b had moved
    the cap to 17350 from 13520 (10m); 10l moved it to 11110 from 9240; 10k
    moved it to 9240 from 7780; 10j moved it to 7780 from 5140; 10i had
    moved it to 5140 from 1810 on 10i's estimate, 10g to 1810 from 1210 on
    its 300.16s phase, 1210 from 10f (430 from 10e, 320 in 10e's campaign,
    260 from 10d), 200 from 10c, 170 from 10b, 150 from 3f3 (110 before, 80
    in 3f2's campaign and 60 in 3f's, 900 against the Θ(N²) suite, before
    `candidates()` answered from a corpus index). Each new optional-ṇic
    vikalpa rule doubles the `skip_nic` mutants' work, and each corpus
    growth scales it, so re-run them alone whenever either happens. The
    `true` mutant's campaign-load phase swings by 2.4x between campaigns
    with host contention (2846.36s in 10k, 5553.65s in 10l, 5329.63s in 10m,
    4908.95s in svādi 5b, 6895.87s in 9c), and its probe phase by 4x
    (2242.64s in 10l, 6759.15s in 10m, 8673.59s in svādi 5b, 6674.37s in
    9c), and so do the equivalents' campaign-load phases (300.16s in 10g,
    142.46s in 10h, 298.02s in 10j, 411.94s in 10k, 349.19s in 10l, 388.14s
    in 10m, 982.79s in svādi 5b, 502.35s in 9c), so the cap is not lowered
    on one quiet measurement. The permanent hang below costs one full cap
    per campaign (17350s of one `-j 4` slot, and 9c's campaign spent its
    last 4h16m on it alone). Take the larger of the probe's and the
    campaign's readings (never a quiet single-mutant run), never lower the
    cap on the quieter one, and re-measure on a quiet host if one becomes
    available.
    Take the floor by measurement, never by scaling it by cell count or by a
    projected contention multiplier. Re-measure the floor and an uncaught
    `-j 4` run whenever the golden suite grows, and change `mise.toml` and
    this paragraph together.
    **One timeout is correct and permanent.** `tripadi.rs`'s 8.4.2 ṇatva
    backward scan decrements a loop index with `j -= 1`; the `j /= 1` mutant
    makes `j` constant, and the loop never terminates. No assertion can ever
    catch it — the mutated run never reaches one — so the cap itself *is*
    the detection mechanism. This is a different phenomenon from the
    reclassification problem above (a real survivor misreported because the
    cap is too short): this mutant hangs at any cap. Identify it by that
    shape rather than by its line number, which drifts, and do not chase it
    with a bigger `--timeout` or a code change; the loop is correct, working
    code. Any other timeout is a suspect survivor: re-run it alone before
    concluding anything.
    **Two tool hazards.** Every `cargo-mutants` invocation rotates
    `mutants.out` → `mutants.out.old` and discards the previous `.old`, so
    pass `-o` to an isolated directory for any follow-up run against a
    finished campaign. The mise shim fails in background shells ("no version
    is set for shim: cargo-mutants"); run the installed `cargo-mutants`
    binary directly, with the task's arguments.
    **Current record (kryādi 9c, 2026-10-08).** Campaign at `-j 4 --timeout
    17350` (the cap this paragraph and the floor paragraph set), `--package
    panini-prakriya --package panini-analyze --test-workspace=true`, `-o
    /home/dev/mutants-records/kryadi-9c`, launched detached with `env -u
    CARGO_MUTANTS_JOBS setsid nohup`, window 2026-10-08 11:13:59 -
    22:14:53 UTC (11h01m; load `36.18 44.70 52.76` at launch and
    `13.76 15.85 15.55` at the end, external and moderate), on the tree at
    `110e9ad`. **931 mutants tested: 876 caught, 52 unviable, 2 missed, 1
    timeout** (exit code 3, as with any timeout). **panini-prakriya: 919
    mutants, 868 caught, 48 unviable, 2 missed, 1 timeout.**
    **panini-analyze: 12 mutants, 8 caught, 4 unviable, 0 missed, 0
    timeout** (unchanged). The two packages sum to the 931 / 876 / 52 / 2 / 1
    total, identical to svādi 5b's. `missed.txt` holds exactly the two
    documented equivalents:
    ```
    crates/panini-prakriya/src/tinanta/adesha.rs:649:30: replace + with *
    crates/panini-prakriya/src/tinanta/tripadi.rs:1416:38: replace - with /
    ```
    `timeout.txt` holds exactly the permanent ṇatva mutant:

    ```
    crates/panini-prakriya/src/tinanta/tripadi.rs:1764:23: replace -= with /=
    ```
    The non-caught set, diffed against svādi 5b's `outcomes.durable.json` on
    the full record (package, file, column, replacement, function, genre,
    outcome), prints `no lines 28 28`, `new: []`, `gone: []`; with span
    lines `lines 55 55`, `new: []`, `gone: []` (every non-caught entry is at
    an unmoved span). Slice 9c adds no mutant: it changes no production
    code, and `cargo mutants --package panini-prakriya --list` at `110e9ad`
    is byte-identical to the one at its base `794fddf` (919 lines, `cmp`
    clean; the lists are kept at
    `/home/dev/mutants-records/kryadi-9c/list-{branch,base}.txt`).
    `--in-diff` over the slice's diff against `794fddf` printed `INFO No
    mutants to filter` for `panini-data` (`lib.rs`) and the same for
    `panini-prakriya`; `panini-analyze` has no diff. (The slice is data
    rows, goldens, tests and docs.)
    The `skip_nic` pair (`sanadi.rs:60:5` `true`, `:60:39` `==`; unmoved)
    was CAUGHT in the probe in 6674.37s and 5720.63s and in the campaign in
    6895.87s and 4904.51s (svādi 5b: 8673.59s / 6324.00s in the probe,
    4908.95s / 3876.51s in the campaign). Under campaign load the two
    uncaught equivalents' test phases were 502.35s (`adesha.rs:649:30`) and
    445.75s (`tripadi.rs:1416:38`); the permanent hang's was 17350.03s (the
    cap). Caught test phases (876) ran min 0.15s, median 43.93s, p90
    185.01s, max 6895.87s (the `true` mutant; then `==` at 4904.51s); none
    other reached 1000s: the next-slowest are `guna.rs:308` `==` at
    857.59s, `&&` 819.26s and `<=` 809.33s, `sound.rs:312` 734.06s,
    `guna.rs:311` `/` 732.55s and `adesha.rs:106` `&&` 727.69s. The cap is
    **17350** (unchanged; the floor paragraph has the arithmetic: the rule
    over this slice's readings gives 13800, and the cap is never lowered),
    set in `mise.toml` and above; this campaign ran at it and the `true`
    mutant cleared it with 2.52x. The only timeout is the permanent `j /= 1`
    hang. `outcomes.json` is kept at
    `/home/dev/mutants-records/kryadi-9c/mutants.out/outcomes.json`, with a
    durable copy at
    `/home/dev/mutants-records/kryadi-9c/outcomes.durable.json`; the probe's
    is at
    `/home/dev/mutants-records/kryadi-9c-probe/probe-outcomes.durable.json`.
    The svādi 5b record it replaces: `git show 794fddf:AGENTS.md`.
    **The per-slice history** of the floor, the cap and every campaign from
    the pada audit through slice 3d, all measured against the Θ(N²) suite,
    was removed in the commit that introduced this paragraph. Read it with
    `git show 440c8c3^:AGENTS.md`.
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
  (`crates/panini/tests/paradigm/`, 40824 cells, ten gaṇas, eight complete —
  tanādi closing at 10/10 in slice 8b (nine of its ten dhātupāṭha rows
  curated in slice 8a; √kṛ, the tenth and last, in 8b), and juhotyādi (3)
  opened in slice 3a at 2 of its 26 rows, at 4 after slice 3b curated √bhī
  and √hrī, at 8 after slice 3c curated √dā, √dhā, √mā
  and √hā (ātmanepada), at 10 after slice 3c2 curated √hā (parasmaipada) and
  √gā, at 16 after slice 3d curated √pṝ, √pṛ, √bhṛ, √ghṛ, √hṛ
  and √sṛ, at 17 after slice 3d2 curated √ṛ, at 20 after slice 3e curated √ṇij, √vij
  and √viṣ, at 24 after slice 3f curated √kit, √tur, √dhiṣ and √dhan, at 25
  after slice 3f2 curated √bhas, and closing at 26 of 26 in slice 3f3 with √jan,
and curādi (10) opened in slice 10a at 4 of its 492 dhātus (√cur, √laḍ, √bhakṣ,
√bhūṣ), at 8 after slice 10b curated the ākusmīya √cit, √vṛṣ, √mad and √kusm, at 41 after slice 10c curated thirty-three more ākusmīya roots, at 47 after slice 10d curated the jñapādi √jñap, √yam, √cah, √cap, √rah and √bal, at 139 after slice 10e curated ninety-two adanta roots, at 149 after slice 10f curated the ten optional-ṇic rows, at 208 after slice 10g curated fifty-nine more, at 258 after slice 10h curated the fifty ādhṛṣīya rows, at 319 after slice 10i curated the fifty-nine āsvadīya rows, √pṝ and √ghuṣ, at 327 after slice 10j curated the eight ajanta rows √smiṅ, √ci, √ghṛ, √gṛ, √yu, √jñā, √cyu and √bhū, at 482 after slice 10k curated the 155 plain obligatory-ṇic rows, at 490 after slice 10l curated the eight rule-bearing rows √ūrj, √cūrṇ, √aṭṭ, √kṝt, √mlecch, √gūrd, √dhras and √kṛp, at 491 after slice 10m curated √picc, narrowing 8.2.30 to a term-final cu, the one dhātu left being √ṣad (`10.0368`), which waits for upasargas, and svādi (5) closing at 38 of 38 in svādi 5b, which curated the thirty-two rows beyond the six curated in the first svādi slice (√hi, √ri, √āp, √śak, √aś, √ṣṭigh) behind 6.4.24 and 8.4.39, and kryādi (9) open at 28 of its 71 dhātus after slice 9c curated twenty-two rows on data alone —
  `PARADIGM`
    stays one-form-per-cell: a cell forked by an optional rule keeps its
    other forms — a second (7014 cells), a third (1035 cells), a fourth
    (361
    cells, rudhādi's √piṣ and — new in slice 7d — √śiṣ loṭ madhyama eka, and
    — new in slice 8a — fifteen more spread across tanādi's four ik-upadhā
    roots kziR/fR/tfR/GfR, and — new in slice 3b — √bhī's vidhiliṅ prathama
    eka, and — new in slice 3f3 — √jan's, and — new in slice 10f — `mUtra`'s
    and `katra`'s laṅ and vidhiliṅ prathama eka, and — new in slice 10g — the
    fifty-nine optional-ṇic rows', and — new in slice 10h — forty-eight
    ādhṛṣīya rows', and — new in slice 10i — the sixty-one āsvadīya, √pṝ
    and √ghuṣ rows', and — new in slice 10l — √dhras's) and
    — the loṭ parasmaipada cells of
    rudhādi's √kṛt, √rudh, √bhid, √kṣud, √tṛd, √und and — new in slice 7f —
    √chid and √chṛd,
    eight ways tied as the record until slice 8a, when the loṭ parasmaipada
    prathama AND madhyama eka of tanādi's four ik-upadhā roots kziR, fR, tfR
    and GfR doubled it to sixteen, and slice 3b's √bhī loṭ parasmaipada
    madhyama eka took it to seventeen, and slice 10f's `pata` (laṅ and
    vidhiliṅ prathama eka) and `mUtra` and `katra` (loṭ prathama and madhyama
    eka) to twenty-three, and slice 10g's fifty-nine optional-ṇic rows (loṭ
    prathama and madhyama eka) to 141, and slice 10h's forty-eight ādhṛṣīya
    rows (the same cells) and √dhū's and √prī's laṅ and vidhiliṅ prathama
    eka to 241, and slice 10i's sixty-one rows (loṭ prathama and madhyama
    eka) to 363, and slice 10j's √ci (laṅ and vidhiliṅ prathama eka) to 365,
    and slice 10l's √dhras (loṭ prathama and madhyama eka) to 367 —
    a fourth
    and fifth (prathama eka) or a fourth through sixth (madhyama eka), or
    seventh for slice 3c2's √hā (`03.0009`) loṭ madhyama eka, the one
    seven-form cell, or up to a ninth for slice 10f's `pata`, slice 10h's
    √dhū and √prī and slice 10j's √ci loṭ parasmaipada prathama and madhyama
    eka, the eight nine-form cells and the record — in
    `ALTERNATES` (12112 rows in all, so 40824 + 12112 = 52936 forms total); √bhuj
    joins neither fork record — its forks stack only 7.1.35 and 8.4.56, the
    same two-deep profile as √yuj — but the √bhuj/1.3.66 slice adds two
    trace pins of its own, `bhunkte_trace_credits_1_3_66_not_1_3_72` and
    `bhunakti_trace_credits_the_shesa_1_3_78`, and
    `derivation_set_is_exactly_pinned` asserts each cell's derivation set is
    exactly the union of the two. The suite is no longer filtered by any
    one-form-per-cell convention — the
    "retiring the conventions" slice retired the last two (7.1.35 tātaṅ,
    8.4.56 pausal cartva), and `PARADIGM`'s pinned form is now genuinely the
    first live branch (the declined derivation; but see 10f: index 0 may be
    blocked) rather than a hand-picked citation form: prathama
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
    rule analysis; kryādi (gaṇa 9) is **open** at 28 of its 71 dhātus across
    all four lakāras, the first gaṇa whose vikaraṇa (śnā) is itself reshaped
    by the ending. √kliś, √gudh, √aś (parasmaipada) landed in slice 9a; √muṣ,
    √vrī (parasmaipada) and √vṛṅ (ātmanepada) landed in slice 9b along with
    8.4.1 / 8.4.2, the engine's first ṇatva. Slice 9c curated twenty-two
    more on the rules already in the pipeline: the eight ñit rows (√krī,
    √prī, √śrī, √mī, √ṣi, √yu, √knū, √drū), the gaṇa's first ubhayapadī
    roots, by 1.3.72, and fourteen parasmaipadī by 1.3.78, among them √khac,
    the second root of 8.4.40's converse arm (*khacñāti*). √vṛṅ is still the
    gaṇa's **only** ātmanepadī root — every other ātmanepada form in kryādi
    belongs to one of those eight. The other forty-three rows need rules:
    7.3.80 *pvādīnāṃ hrasvaḥ* and the other phonology and root specials of
    slice 9d, and 6.4.24's ten rows (on svādi 5b's general rule) and 3.1.82
    for slice 9e — see
    `docs/superpowers/specs/2026-07-28-kryadi-gana-design.md` and
    `docs/superpowers/specs/2026-10-07-kryadi-gana-9c-design.md`; svādi
    (gaṇa 5) is **complete** at all 38 of its dhātupāṭha rows — first six
    roots across all four lakāras: √āp, √śak, √hi and √ri (parasmaipada),
    √aś (`05.0020`, distinct from kryādi's `09.0059`) and √ṣṭigh (`stiG`)
    (ātmanepada); then, in svādi 5b, the other thirty-two, ten ubhayapadī by
    1.3.72 and twenty-two parasmaipadī, behind 6.4.24 *aniditāṁ hala
    upadhāyāḥ kṅiti* (√dambh's *dabhnoti*; its *aniditām* is `Tag::Idit`,
    from the data layer's `IDIT`) and 8.4.39 *kṣubhnādiṣu ca* (√tṛp's
    *tṛpnoti*, keyed by row and reading śnu as `Tag::Snu`). Its vikaraṇa is śnu (3.1.73),
    and it is the first gaṇa where 7.3.84's guṇa lands on the vikaraṇa rather
    than the root: 7.3.84 now applies twice, once with respect to śnu and once
    with respect to the ending (1.4.13 makes the aṅga affix-relative), giving
    `Apnoti` against the ṅit-blocked `ApnutaH`. The other split running
    through the gaṇa is *asaṁyogapūrva* — whether śnu's `u` is preceded by a
    conjunct decides both the yaṇ alternation (6.4.87 / 6.4.77: `hinvanti`
    against `Apnuvanti`) and the hi-luk (6.4.106: `hinu` against `Apnuhi`) —
    see `docs/superpowers/specs/2026-07-29-svadi-gana-design.md`.) rudhādi
    (gaṇa 7, vikaraṇa śnam) is now **complete** — the first gaṇa curated at
    its full dhātupāṭha strength. Nine of its 25 dhātupāṭha roots are ubhayapadī
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
    re-derives all 66 verdicts from the vendored upadeśa, so the column
    cannot drift from the data that determines it. That discharges
    the **ubhayapada** deferral as such: 1.3.72 is no longer what keeps any
    root out, and — since rudhādi 7f curated √chid and √chṛd — every
    ubhayapadī-marked root in the curated set now derives, verified cell by
    cell against vidyut-prakriya.
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
    **Rudhādi 7f's own cross-implementation audit** ran the same probe
    against vidyut-prakriya at commit `8da2f90`, over the corpus grown by
    √chid and √chṛd: **zero differences across all 2772 cells / 3259
    forms / 66 roots**, with both negative controls (`entry` and `form`)
    verified failing first. The new `Rule`s behind it are 6.1.73 and
    8.4.40 — an earlier task in this slice had already proved both inert
    on the pre-7f 2628-cell corpus by a byte-for-byte dump diff of its
    own, before √chid and √chṛd were curated at all.
    **The √bhuj/1.3.66 slice's own cross-implementation audit** ran the
    same probe against vidyut-prakriya at commit
    `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, over the corpus grown by
    √bhuj: **zero differences across all 2844 cells / 3338 forms / 67
    roots**, with both negative controls (`entry` and `form`) verified
    failing first. 1.3.66 is √bhuj's only new `Rule`, and it is a
    root-keyed pada assignment structurally identical to 1.3.72's, which
    this engine already implements.
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
    √yuj's `ayunag`/`ayunak` already witnesses. Rudhādi 7d then curated
    eight of the nine
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
    step; this engine rejects that credit — its 6.4.24, landed in svādi 5b,
    never reaches a rudhādi aṅga, which 3.1.78 leaves vowel-final — and pins
    the rejection in `tests/trace/`.) That left √tṛh as the
    ninth and only reachable non-ubhayapadī root still out, deferred to
    slice 7e behind three sūtras the engine did not implement: 7.3.92
    *tṛṇaha im* (the *im* augment), 8.2.31 *ho ḍhaḥ* and 8.3.13 *ḍho ḍhe
    lopaḥ* — see the 7e paragraph above for what curating it actually
    took.
    Rudhādi 7f then curated the last two reachable ubhayapadī roots,
    taking the gaṇa to **twenty-four**: √chid (`07.0003 Ci\di~^r`) and
    √chṛd (`07.0008 u~Cfdi~^r`), behind two new sūtras — 6.1.73 *che ca*
    (the tuk augment before a `C` after a short vowel) and 8.4.40 *stoḥ
    ścunā ścuḥ* (the ścutva that follows it) — without which their laṅ
    cells would surface `aCinat` for `acCinat`. Both are pinned in both
    padas. √bhuj (`07.0017 Bu\ja~`)
    is the twenty-fifth entry, and the √bhuj/1.3.66 slice curated it: 1.3.66
    *bhujo'navane* is implemented as an unconditional ubhayapada assignment
    (`PadaAssignment::UbhayapadaAnavane` → `Tag::Anavane`, so the trace
    credits 1.3.66 and can never reach 1.3.72), with *anavane* — the
    **sense** restriction the sūtra actually imposes — recorded as an
    unimplemented sense restriction on 1.3.72's own precedent, since neither
    engine models sense. **25 curated — no rudhādi entry remains out.**
    Rudhādi is the first gaṇa curated at its full dhātupāṭha strength.
    √indh's pada was **verified, not inferred from its
    ñi**: `YiinDI~\`'s ñi it-marker is one of the two things 1.3.72 reads,
    which would have made the root ubhayapadī alongside √rudh, so it was
    checked against vidyut-prakriya — which derives √indh in ātmanepada
    only, against a `~^r` control (√rudh) that does derive both padas — and
    the root's own anudātta settles its pada by 1.3.12 *anudāttaṅita
    ātmanepadam*. śnam is the engine's first **infix**: unlike every other
    vikaraṇa it is not a suffix, and the pipeline's fixed
    `[AGAMA, ABHYASA, ANGA, SHAP, ENDING]` slots have
    nowhere to put one, so 3.1.78 splits the root across `ANGA` and `SHAP`
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
    it be deleted). Since juhotyādi 3f, 8.2.75 carries no gaṇa test (√kit's
    *acikeH*), and since 3f2 neither do 8.2.73 and 8.2.74 (√bhas's *abaBat*,
    *abaBaH*). **8.2.41 below
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
    `crates/panini/tests/trace/rudhadi.rs` is the sole pin; no surface-form golden
    catches a reversal).
    tanādi (gaṇa 8, vikaraṇa the bare `u` of 3.1.79) is now **complete** —
    slice 8a curated nine of its ten dhātupāṭha rows (√tan, √san, √kṣaṇ,
    √kṣiṇ, √ṛṇ, √tṛ and √ghṛ, all seven ubhayapadī by 1.3.72, plus √van and
    √man, both ātmanepadī by 1.3.12), taking the curated set from 67 to
    76 roots; slice 8b then curated √kṛ (`08.0010`), the tenth and last
    row, the one root 3.1.79 itself names, behind three new root-keyed
    rules — 6.4.110 *ata ut sārvadhātuke*, 6.4.108 *nityaṁ karoteḥ* and
    6.4.109 *ye ca* — taking the curated set to **77** roots and closing
    the gaṇa at 10/10. See
    `docs/ARCHITECTURE.md`'s tanādi paragraph and
    `docs/superpowers/specs/2026-08-30-tanadi-gana-design.md` for the rule
    analysis.
  and by the ordered-trace test (`crates/panini/tests/trace/`), which pins
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
  `form` negative controls verified failing first, and the first of these
  runs with new `Rule`s behind it: 7.3.92, 8.2.31 and 8.3.13.
  **Rudhādi 7f re-ran the same committed harness once more, at the same
  vidyut commit `8da2f90`, over the corpus grown by √chid and √chṛd: zero
  differences across 2772 cells / 3259 forms / 66 roots**, with both
  `entry` and `form` negative controls verified failing first, and, like
  7e's run, one with new `Rule`s behind it: 6.1.73 and 8.4.40 this time,
  where 7e's were 7.3.92, 8.2.31 and 8.3.13. **The √bhuj/1.3.66 slice
  re-ran the same committed harness once more, at the same vidyut commit
  `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, over the corpus grown by
  √bhuj: zero differences across 2844 cells / 3338 forms / 67 roots**,
  with both `entry` and `form` negative controls verified failing first —
  the record until tanādi 8a (below). Its one new `Rule`, 1.3.66, is a root-keyed pada
  assignment structurally identical to 1.3.72's already-implemented one —
  the same pattern the pipeline already carries, applied to a second
  root-list. **Tanādi 8a's own cross-implementation audit** ran the same
  committed harness once more, at the same vidyut commit
  `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, over the corpus grown by its
  nine curated roots: **zero differences across 3420 cells / 4321 forms /
  76 roots**, with the `entry` negative control verified failing first
  (exit 1, 36 √bhū cells, `Bavati` vs `paWati`) — the record until tanādi
  8b (below). It
  did not come back clean on the first pass: an initial run found four
  differing cells, all `08.0005` (√ṛṇ, `fR`) laṅ uttama-puruṣa dvi/bahu in
  both padas, diagnosed and fixed in commit `88cae65` before the clean
  re-run above. Two new `Rule`s are behind it — 3.1.79 (the bare `u`
  vikaraṇa) and 7.3.86's new vikalpa arm (the gaṇa's own guṇa/aguṇa
  fork) — plus two structural engine changes the divergence exposed: the
  asaṁyogapūrva helper (`terms.rs`, renamed `shnu_asamyogapurva` →
  `vikarana_u_asamyogapurva` to read tanādi's bare `u` as well as svādi's
  `nu`) reads only surface characters, so evaluated after 6.1.90 it could
  no longer tell a genuinely non-conjunct `u` (fR's) from a guṇa'd
  conjunct one (arR's) — 6.1.90's āṭ-vṛddhi ekādeśa merges the augment
  into both the same way, leaving the same three characters (`rR`) either
  way — and the fix moved 6.4.106/6.4.107 themselves ahead of that
  ekādeśa instead, so the helper reads the aṅga before the merge erases
  the distinction; and `run_pipeline` gained a convergent-fork collapse (dedup on
  identical live-branch text, first/declined branch kept), needed once
  7.3.86's new vikalpa arm could put a guṇa'd and an āṭ-vṛddhi'd branch on
  the same surface (`A+fR` and `A+arR` both → `ArRot`). **Tanādi 8b's own
  cross-implementation audit** ran the same committed harness once more,
  at the same vidyut commit `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`,
  over the corpus grown by √kṛ, the gaṇa's tenth and last root: **zero
  differences across 3492 cells / 4399 forms / 77 roots**, with the
  `entry` negative control verified failing first — the record until
  juhotyādi 3a's own audit (`tools/audit/README.md`'s 2026-09-07 entry,
  3564 cells / 4483 forms / 79 roots), itself superseded by juhotyādi 3b's
  own audit (`tools/audit/README.md`'s 2026-09-09 entry, 3636 cells / 4595
  forms / 81 roots), and that by juhotyādi 3c's (`tools/audit/README.md`'s
  2026-09-12 entry, 3852 cells / 4823 forms / 85 roots), and that by juhotyādi
  3c2's (`tools/audit/README.md`'s 2026-09-29 entry, 3924 cells / 4926 forms /
  87 roots), and that by juhotyādi 3d's (`tools/audit/README.md`'s 2026-09-29
  entry, 4176 cells / 5208 forms / 93 roots), and that by juhotyādi 3d2's
  (`tools/audit/README.md`'s 2026-09-30 entry, 4212 cells / 5249 forms / 94
  roots), and that by juhotyādi 3e's (`tools/audit/README.md`'s 2026-10-01
  entry, 4428 cells / 5486 forms / 97 roots), and that by juhotyādi 3f's
  (`tools/audit/README.md`'s 2026-10-01 entry, 4572 cells / 5655 forms / 101
  roots), and that by juhotyādi 3f2's (`tools/audit/README.md`'s 2026-10-01
  entry, 4608 cells / 5699 forms / 102 roots), and that by juhotyādi 3f3's
  (`tools/audit/README.md`'s 2026-10-01 entry, 4644 cells / 5750 forms / 103
  roots), and that by curādi 10a's (`tools/audit/README.md`'s 2026-10-02 entry, 4932
  cells / 6062 forms / 107 roots), and that by curādi 10b's
  (`tools/audit/README.md`'s 2026-10-02 10b entry, 5076 cells / 6206 forms / 111
  roots), and that by curādi 10c's (`tools/audit/README.md`'s 2026-10-02 10c entry,
  6264 cells / 7394 forms / 144 roots), and that by curādi 10d's
  (`tools/audit/README.md`'s 2026-10-02 10d entry, 6696 cells / 7862 forms /
  150 roots), and that by curādi 10e's (`tools/audit/README.md`'s 2026-10-02 10e
  entry, 12996 cells / 14660 forms / 242 roots), and that by curādi 10f's
  (`tools/audit/README.md`'s 2026-10-03 10f entry, 13716 cells / 15644 forms /
  252 roots), and that by curādi 10g's (`tools/audit/README.md`'s 2026-10-03 10g
  entry, 17964 cells / 22724 forms / 311 roots), and that by curādi 10h's
  (`tools/audit/README.md`'s 2026-10-04 10h entry, 21564 cells / 29096 forms /
  361 roots), and that by curādi 10i's (`tools/audit/README.md`'s 2026-10-04 10i
  entry, 25956 cells / 36416 forms / 422 roots), and that by curādi 10j's
  (`tools/audit/README.md`'s 2026-10-04 10j entry, 26424 cells / 37070 forms /
  430 roots), and that by curādi 10k's (`tools/audit/README.md`'s 2026-10-05 10k
  entry, 37584 cells / 49160 forms / 585 roots), and that by curādi 10l's
  (`tools/audit/README.md`'s 2026-10-06 10l entry, 38160 cells / 49826 forms /
  593 roots), and that by curādi 10m's (`tools/audit/README.md`'s 2026-10-06 10m
  entry, 38232 cells / 49904 forms / 594 roots), and that by svādi 5b's
  (`tools/audit/README.md`'s 2026-10-07 svādi 5b entry, 39744 cells / 51724 forms /
  626 roots), and that by kryādi 9c's
  (`tools/audit/README.md`'s 2026-10-08 9c entry, 40824 cells / 52936 forms /
  648 roots).
  Three new `Rule`s are behind it, all root-keyed to √kṛ and all in
  `guna.rs` — 6.4.110 *ata ut sārvadhātuke*, 6.4.108 *nityaṁ karoteḥ* and
  6.4.109 *ye ca* — plus one engine change with no `Rule` of its own:
  8.2.79 *na bhakurchurām* is modelled as a named exclusion guard inside
  8.2.77 *hali ca*'s own `apply` (`tripadi.rs`), rather than as a
  separate rule, so 8.2.77's lengthening declines on √kṛ's `kur` aṅga and
  vidyut-prakriya's own log still records 8.2.79 on every `kur` cell, which
  is why the forms agree. See
  `tools/audit/README.md`'s own recorded result for the full breakdown.
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
  figures — `controller.rs:152` and `tinanta/guna.rs:1233` (drifted from
  the `:130`/`:943` this ledger previously cited, as later slices added
  code above them; confirmed by re-reading each construct at its current
  line rather than trusting the old numbers to have held) cite the corpus
  size as 1872/1864-of-1872, now seven slices further stale: the corpus
  stood at 2304/2296-of-2304 as of the 8.2.30/8.2.39 slice, stood at
  2592/2584-of-2592 as of rudhādi 7d, stood at 2628/2620-of-2628 as of
  rudhādi 7e, stood at 2772/2764-of-2772 as of rudhādi 7f, stood at
  2844/2836-of-2844 as of the √bhuj/1.3.66 slice — the same 8
  cells 6.4.107 always fired on through all of those slices
  (`key_count("6.4.107") == 8` back then, pinned in
  `derivation_set_shape_matches_the_audited_numbers`,
  `crates/panini/tests/paradigm/main.rs`), unmoved by 7d, 7e, 7f or
  √bhuj/1.3.66 since 6.4.107 concerned only svādi's √hi and √ri — stood at
  3420 cells as of tanādi 8a, and now
  stands at 3492 cells as of tanādi 8b, where `controller.rs:152`'s own
  claim ("only 8 cells fire it at all") is no longer merely
  corpus-size-stale but flatly wrong: tanādi's bare `u` is asaṁyogapūrva
  for every one of its nine 8a-curated roots (√kṛ, 8b's own root, does not
  add to this count — 6.4.108 empties its `u` first), so 6.4.107 fired
  on **72**
  cells across eleven roots as of tanādi 8b, and fires on **188** across
  thirty since svādi 5b's nineteen asaṁyogapūrva roots
  (`key_count("6.4.107") == 188`, the same test), not 8 — the "8 cells"
  figure was never re-derived when the gaṇa landed. `guna.rs:1233`'s own claim ("1872 goldens move") stays stale
  only in the ordinary corpus-size sense, not wrong in kind: 40824 goldens
  would move today. Neither comment was touched by tanādi 8a or 8b, consistent
  with every slice since 7c. Rudhādi 7d touched neither comment — its one permitted
  engine-comment edit is the comment above
  `vrddhi_of_ac_vowels_all_arms` in `tinanta/sound.rs`. Rudhādi 7e, rudhādi
  7f and the √bhuj/1.3.66 slice touched neither comment either. The corpus
  stands at 3564 cells as of juhotyādi 3a (`guna.rs:1233`'s claim now
  anchored at `guna.rs:1472`, `controller.rs:152`'s at `controller.rs:153`),
  and this slice touched neither comment either. Juhotyādi 3b touched
  neither comment either: `controller.rs:153`'s anchor is unchanged, and
  `guna.rs:1472`'s has drifted further, to `guna.rs:1666` (3b's `guna.rs`
  additions — 6.4.82's widening, 6.4.77's iyaṅ arm and 6.4.115 — all land
  earlier in the file, above this test; 7.4.59 and 7.4.60 live in
  `abhyasa.rs` and cannot move a `guna.rs` line). The corpus
  stands at 3636 cells as of juhotyādi 3b. Juhotyādi 3c touched neither
  comment either; the corpus stands at 3852 cells as of 3c.
  (`controller.rs:153`'s anchor unchanged — re-confirmed empty
  `git diff main -- crates/panini-prakriya/src/controller.rs` — and
  `guna.rs:1666`'s drifted further, to `guna.rs:1939`: Task 9's own fix
  round added four lines to `guna.rs`'s 6.1.78 justification, which sits
  above it, after the line was first measured at `guna.rs:1935`). Juhotyādi 3c2 touched neither
  comment either; the corpus stands at 3924 cells as of 3c2 (`guna.rs:1939`'s claim now
  anchored at `guna.rs:2162`, `controller.rs:153`'s at `controller.rs:206`:
  3c2's `Rule.bars` field and its plumbing moved the latter, and 3c2's four
  rules in `guna.rs` moved the former; both lines measured by grep at this
  commit). Juhotyādi 3d touched neither comment either; the corpus stands at
  4176 cells as of 3d (`guna.rs:2162`'s claim now anchored at `guna.rs:2226`,
  `controller.rs:206`'s at `controller.rs:206`: 3d added 7.1.102 and 6.1.77's
  aṅga arm above it in `guna.rs`, and 3d's comment sweep reflowed the 6.4.82
  and 6.1.77 comments above it too, while nothing in 3d touched
  `controller.rs`; both lines measured by grep at this commit). Juhotyādi 3d2 touched neither
  comment either; the corpus stands at 4212 cells as of 3d2 (`guna.rs:2226`'s
  claim now anchored at `guna.rs:2228`, `controller.rs:206`'s unchanged at
  `controller.rs:206`: 3d2's 6.4.78 work grew the 6.1.78 comment in `guna.rs`
  by two lines above the test, while `controller.rs` is unchanged from 3d,
  so the 3d entry's `controller.rs:206` was correct; both lines measured by
  grep at this commit). Juhotyādi 3e touched neither comment either; the corpus
  stands at 4428 cells as of 3e (`guna.rs:2228`'s claim now anchored at
  `guna.rs:2384` (moved from 2376 by the final-review comment on 7.3.87's order), `controller.rs:206`'s at `controller.rs:206`: 3e's 7.3.87 rule
  and its tests landed in `guna.rs` above the test, while `controller.rs` is
  unchanged; both lines measured by grep at this commit). Juhotyādi 3f touched neither comment either; the corpus stands at 4572 cells as of 3f (`guna.rs:2384`'s claim anchored at `guna.rs:2386`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Juhotyādi 3f2 touched neither comment either; the corpus stands at 4608 cells as of 3f2 (`guna.rs:2386`'s claim anchored at `guna.rs:2433`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Juhotyādi 3f3 touched neither comment either; the corpus stands at 4644 cells as of 3f3 (`guna.rs:2433`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10a touched neither comment either; the corpus stands at 4932 cells as of 10a (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10b touched neither comment either; the corpus stands at 5076 cells as of 10b (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10c touched neither comment either; the corpus stands at 6264 cells as of 10c (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10d touched neither comment either; the corpus stands at 6696 cells as of 10d (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10e touched neither comment either; the corpus stands at 12996 cells as of 10e (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10f touched neither comment either; the corpus stands at 13716 cells as of 10f (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10g touched neither comment either; the corpus stands at 17964 cells as of 10g (`guna.rs:2565`'s claim anchored at `guna.rs:2565`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10h touched neither comment either, though its guṇa-stage 7.2.114 moved the first; the corpus stands at 21564 cells as of 10h (`guna.rs:2565`'s claim anchored at `guna.rs:2641`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). Curādi 10i touched neither comment either; the corpus stands at 25956 cells as of 10i (`guna.rs:2641`'s claim anchored at `guna.rs:2641`, `controller.rs:206`'s at `controller.rs:206`; both lines measured by grep at this commit). A third,
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
  branch inside `derive`. `TINANTA_RULES` is a list of nine stage arrays,
  each living in its own file under `crates/panini-prakriya/src/tinanta/`; add
  the rule to the stage its pipeline position falls in, and add its id to
  `tinanta_rule_order_is_pinned` in the same position. Which stage a rule
  belongs to is decided by its position relative to **3.1.68**, not by its
  sūtra family: rules before
  3.1.68 address the ending as `ENDING_PRE_SHAP` (index 3), rules after it as
  `ENDING` (index 4), and `terms[SHAP].text` may be empty for adādi. Two
  permanent, usually-empty slots — `AGAMA` (0) and `ABHYASA` (1) — precede
  the aṅga; see `tinanta/terms.rs`. Per-rule guard tests go beside the rule
  in its stage
  file; tests asserting a surface form or trace go in
  `tinanta/derivation_tests.rs`. A guard test looks its rule up in its own stage's static (`GUNA.iter().find(…)`, `SAMJNA.iter()…`), never with `rules().find(…)`: ids repeat across stages — 1.3.9 twice and 7.3.86 three times since curādi 10a — and `rules()` returns the first, which is the sanādi stage's. **Write a per-rule guard test where the
  rule's precondition can be built directly on a hand-built `Prakriya`.
  Where it cannot — because only an upstream rule chain produces that
  state — cite the covering derivation or trace test in the rule's own
  comment instead.** That is narrower than "every rule gets a guard test",
  and it is not the blanket exemption 7a's deferred #5 asked for ("per-rule
  guard tests for tripādī rules are not achievable"): `tripadi.rs` carries
  nineteen of them today, including
  `jhalam_jasho_ante_fires_on_any_pada_final_jhal_jashtva_of_resolves` and
  `va_avasane_fires_only_on_a_pada_final_jhal`. Whole-word scope is not what
  blocks a guard test; an unconstructible precondition is. `derive` carries
  no grammar branches: the only gana-conditioned logic there is aṅga tagging
  (`Tag::Adadi` &c.), which feeds the guarded rules rather than substituting
  for them. Fixtures shared across `crates/panini/tests/*.rs`
  integration-test binaries (e.g. `CELLS`, `LAKARA_BY_NAME`) go in
  `crates/panini/tests/common/mod.rs`, `mod`-included by each file that
  needs them — do not redefine them per test file.
- **A deleted sound's sthānivadbhāva is a tag, not rule order.** When a rule
  deletes a sound that 1.1.57 *acaḥ parasmin pūrvavidhau* must still let
  block later rules, the deletion sets a tag on the term (`Tag::AtLopa`,
  set by 6.4.48 in `tinanta::sanadi`), and each blocked rule declines on that
  tag (7.2.116 and the sanādi 7.3.86); never rely on rule order for the
  block. Any new upadhā-reader placed after 6.4.48 must check `Tag::AtLopa`
  too. The tag survives 3.1.32's fold for later stages (luṅ's abhyāsa, as
  vidyut reads its `FlagAtLopa`).
- **Optional (*vikalpa*) rules set `Rule.vikalpa = true`.** `run_pipeline`
  forks there: it clones each live branch, applies to the clone, and keeps
  the clone only if `apply` returned true, so a rule that declines its own
  guard forks nothing. The declined branch keeps its index and the applied
  clone is inserted immediately after it, which is why index 0 of a
  derivation is the engine's no-optional-rules reading. Index 0 may be
  blocked while a later branch is live (an optional-ṇic root's ṇic branch
  in a pada it does not admit, slice 10f), and a cell's form is the first
  live branch. `derive` therefore returns `Vec<Prakriya>`, and a cell may
  have more than one valid form. Add an optional rule exactly as any other —
  in its stage file, with its id in `tinanta_rule_order_is_pinned` in
  position — and also add it to
  `exactly_the_pinned_vikalpa_rules_are_optional`, which pins the whole
  optional set by id. **Twelve rules are optional today, in pipeline order:
  7.1.35, 3.4.111, 7.3.86, 6.4.117, 6.4.116, 6.4.115, 6.4.43, 6.4.107, 8.2.74, 8.2.75, 8.4.65, 8.4.56 —
  6.4.115 landed in slice 3b, the engine's ninth vikalpa rule, root-keyed to
  √bhī and forking across all four lakāras (a kṅit-sārvadhātuka fork; loṭ
  is where it stacks with 7.1.35). Slice 3c2's 6.4.117 and 6.4.116 are the
  tenth and eleventh, and slice 3f3's 6.4.43, keyed to √jan's row and forking
  its nine vidhiliṅ cells, the twelfth.** (7.3.86
  is the vikalpa entry only — its *nitya* entry, just above it in the
  pipeline, is not optional.) 7.1.35 and
  8.4.56 can both fire on one derivation, stacking into a three-branch
  cell — loṭ prathama eka forks twice, giving `Bavatu` / `BavatAd` /
  `BavatAt`. Eight rudhādi roots — √kṛt, √rudh, √bhid, √kṣud, √tṛd, √und,
  √chid and √chṛd — each
  stack three of the twelve (7.1.35,
  8.4.65, 8.4.56) on their own loṭ parasmaipada cells, and tanādi's
  kziR/fR/tfR/GfR stack a different three (7.1.35, 7.3.86, 8.4.56) on
  theirs — five branches at
  prathama eka, six at madhyama eka
  (`kfndDi` / `kfnDi` / `kfnttAd` / `kfntAd` / `kfnttAt` /
  `kfntAt`, and `rundDi` / `runDi` / `rundDAd` / `runDAd` / `rundDAt` /
  `runDAt`, and likewise for √bhid, √kṣud, √tṛd, √und, √chid and √chṛd) —
  because 8.4.56 only
  reaches the two tātaṅ (7.1.35) branches,
  not the two vowel-final ones. √yuj, ubhayapadī like √bhid, √kṣud and
  √tṛd (its own 7c cohort) but
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
    `terms[SHAP].text == "n"` for svādi and `""` for tanādi, which
    invalidates two predicates: `vikarana_u_asamyogapurva` (renamed from
    `shnu_asamyogapurva`; its first guard now matches SHAP text of `"nu"`
    (svādi) or `"u"` (tanādi)) and
    `sound_before_ending` (which reads the last char before the ending — `u`
    before the mutation, something else after) — the vikaraṇa *is* still
    śnu or tanādi's bare `u`, but
    nothing downstream can tell any more. Every rule that reads the
    vikaraṇa's u-bearing
    text must precede it — 6.4.87 and 6.4.106 via `vikarana_u_asamyogapurva`,
    6.4.77, which open-codes the same `text == "nu"` test, and 6.1.77, which
    open-codes the tanādi-specific `text == "u"` test — and all four do.
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
- **A guard that names particular roots keys on the dhātupāṭha number
  wherever the root text is ambiguous.** `ctx.dhatupatha` carries the row
  `derive` was called with, or `""` on a hand-built prakriyā, which every
  such guard declines. 6.4.87, 6.4.101 and 6.4.115 still key on `ANGA.text`
  (`hu`, `BI`), safe only because `juhotyadi_rows_are_the_twenty_six_curated_roots`
  asserts those codes stay unique; 7.4.75, 7.4.76, 6.4.116, 6.4.117, 6.4.118, 8.2.38
  and 8.2.40's *adhaḥ* key on numbers, because `03.0008` and `03.0009` share
  `hA`, and 7.4.77 (`03.0004`, `03.0005`, `03.0017`) follows that precedent. 8.4.39 (`KSUBHNADI`: `05.0028`, svādi 5b) does too: curādi's `10.0351` and `10.0355` are also `tfpa~`, stored `tfp`. 7.4.75 (`03.0012`–`03.0014`) does too: `vij` is also `06.0009` and `07.0023`. 6.4.100 (`03.0019`, slice 3f2) does too, so a curated √ghas extends its key rather than sharing a text test. 6.4.98 (`03.0025`) and 6.4.42 / 6.4.43 (`JANA_SANA`: `03.0025`, `08.0002`; slice 3f3) do too; the latter key both curated roots their sūtra names, and √san declines on its follower, the vikaraṇa `u`, not on a missing key. 7.4.78 keys
  on a number for a different reason: the sūtra names no root, the Kaumudī
  applies it to one row (03.0026), and `gA` is also `01.1101 gA\N`. A sūtra
  naming a class of roots becomes a saṁjñā tag decided from the number in `derive` —
  `Tag::Ghu` from `tinanta/samjna.rs`'s `GHU` — pinned to the vendored TSV
  rather than to a derivation, so its uncurated members are held too. A
  verdict the upadeśa carries but the stored code cannot is decided the same
  way: 6.4.24's *aniditām* reads `Tag::Idit`, from `panini_data::IDIT`,
  which `idit_matches_upadesha_markers` holds to the vendored upadeśa in
  both directions.
- **An apavāda that must stop other rules on its branch declares it in
  `Rule.bars`; never in the overridden rules' guards, and never by reading
  `p.log`.** `run_pipeline` enforces the bar per branch (a vikalpa's on its
  applied clone only), and `exactly_the_pinned_bars` requires every barred id
  to run after its barrer. Eleven rules bar others today (`exactly_the_pinned_bars`
  lists them: eight optional-ṇic rules, 7.3.87, 6.4.117 and 8.4.39). 6.4.117 *ā ca hau* is one: it changes no text,
  so without its bars 6.4.116, 6.4.113 and 6.4.112 would each still rewrite
  the `A` it keeps. 7.3.87 is another, a mandatory one: it changes
  no text and bars 7.3.86, so 7.3.86's guard carries no abhyasta exception.
  8.4.39 *kṣubhnādiṣu ca* (svādi 5b) is the eleventh pinned barrer: it changes no text and
  bars 8.4.1 and 8.4.2, so ṇatva's guards carry no √tṛp exception. Its
  guard reads the row (`KSUBHNADI`) and śnu by identity, `Tag::Snu` on the
  next non-empty term after `ANGA`, never by text (the tripādī's śnu text
  is `nu`, `no`, `nuv` or `nav`).
  7.1.6's read of `p.log` for 7.1.5 is an ENABLING condition,
  not a bar, and stays as it is.

## Where things live
See `docs/ARCHITECTURE.md`.
