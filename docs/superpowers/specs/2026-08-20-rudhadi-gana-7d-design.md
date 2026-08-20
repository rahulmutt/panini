# rudhādi (gaṇa 7), slice 7d — the eight roots that need no new sūtra

Three documents describe rudhādi's remaining twelve roots as splitting three
ways, and one of those ways is a bucket of nine:

> Nine further reachable non-ubhayapadī roots (√śiṣ, √tṛh, √und, √añj, √tañc,
> √vij, √vṛj, √pṛc, √vid) are simply not curated yet.

"Simply not curated" turns out to be true of eight of the nine and false of the
ninth. A probe against vidyut-prakriya at the audited commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, comparing the sūtras each root's
derivations invoke against the engine's implemented set, separates them
cleanly:

- **√śiṣ, √und, √añj, √tañc, √vij, √vṛj, √pṛc and √vid** reach every one of
  their 36 cells through sūtras this engine already has.
- **√tṛh** needs three it does not: 7.3.92 *tṛṇaha im* — a mit-āgama placed by
  1.1.47, the engine's first āgama of that kind — with 8.2.31 *ho ḍhaḥ* and
  8.3.13 *ḍho ḍhe lopaḥ* behind it, for `tfReQi` and `atfReq`/`atfRew`.

This slice curates the eight. √tṛh and its three sūtras become slice 7e.

The split is deliberate and not merely convenient. A curation-only slice's
audit is an unambiguous signal: with no engine change in the diff, a difference
means the engine is wrong about a rule it *already has*, and there is no new
rule to blame it on. Mixing the eight rows in with three new sūtras would put
both causes in play at once, in the slice whose whole job is to tell them
apart. It also halves the per-slice growth of the golden suite, which is no
longer free — see "The mutation gate" below.

## Scope

New: eight `Dhatu` rows in `panini-data`; 32 `PARADIGM` blocks (8 roots × 4
lakāras × 1 pada); 72 `ALTERNATES` rows; three trace pins.

Changed: `rudhadi_rows_are_the_thirteen_curated_roots` renamed and extended;
the corpus-total assertions in `panini-data`,
`crates/panini/tests/paradigm.rs` and `tools/audit/panini_full_audit.rs`; the
stale claim in `sound.rs`'s `vrddhi_of_ac_vowels_all_arms`; and the
documentation sites enumerated under "The doc claims this slice falsifies".

**No engine code changes.** No file under `crates/panini-prakriya/src/tinanta/`
gains, loses or alters a `Rule`. If the audit says otherwise, see "If the audit
shows a difference".

## The eight roots

Every `code` and every pada below was hand-traced through `panini-data`'s
existing `strip_anubandhas` and `pada_from_upadesha` and falls out correctly,
so the data layer needs no change to accept them. Two are worth naming: the
`o~` of `o~vijI~` is the first `o~`-initial it-marker the table carries, and it
is stripped by the existing 1.3.2 loop with no new arm; and `Si\zx~`'s `x~`
goes the same way while the root-final `z` — a real sound, not an it —
survives, because `ends_in_hal` is decided on the original upadeśa whose last
character is `~`.

| entry | upadeśa | `code` | pada | what it exercises |
|---|---|---|---|---|
| 07.0014 | `Si\zx~` | `Siz` | parasmaipada | 8.4.41 ṣṭutva + 8.3.24 + 8.3.59, the path curated √piṣ already witnesses (`Sinazwi`, `SiRQi`, `SiMzwaH`) |
| 07.0020 | `undI~` | `und` | parasmaipada | 6.1.90 with `u` → `O`; the fork-heavy root |
| 07.0021 | `anjU~` | `anj` | parasmaipada | vowel-initial, 6.4.23 + 6.4.111 + 8.2.30 (`anakti`, `aNktaH`, `Anag`/`Anak`) |
| 07.0022 | `tancU~` | `tanc` | parasmaipada | 8.2.30 on `c` + 8.4.53 (`tanakti`, `taNgDi`) |
| 07.0023 | `o~vijI~` | `vij` | parasmaipada | first `o~`-initial it-marker curated (`vinakti`, `viNktaH`) |
| 07.0024 | `vfjI~` | `vfj` | parasmaipada | 8.4.1 ṇatva of śnam's `n` (`vfRakti`) |
| 07.0025 | `pfcI~` | `pfc` | parasmaipada | ṇatva + 8.2.30 on `c` (`pfRakti`, `pfNktaH`) |
| 07.0013 | `vi\da~\` | `vid` | ātmanepada | ātmanepada-only; 8.4.65 forks throughout (`vinte`/`vintte`) |

Pada assignments agree with vidyut-prakriya's own derivations: it produces
parasmaipada forms only for the first seven and ātmanepada forms only for
`vi\da~\`, whose trailing `~\` reaches 1.3.12 *anudāttaṅita ātmanepadam*. None
of the eight is ubhayapadī, so each contributes 36 cells rather than 72.

## Corpus growth

Counted off the probe, not estimated:

| | before | after |
|---|---|---|
| roots | 55 | **63** |
| cells | 2304 | **2592** |
| forms | 2654 | **3014** |
| `ALTERNATES` rows | 350 | **422** |

+12.5% in cells. rudhādi goes 13 → **21 of 25**, and the "nine reachable
non-ubhayapadī roots" bucket empties to one: after 7d the gaṇa's remaining four
are √tṛh (7e's three sūtras), √chid and √chṛd (6.1.73 *che ca* with 8.4.40
*stoḥ ścunā ścuḥ*), and √bhuj (1.3.66 *bhujo'navane*, which forks its pada on
sense rather than on an axis this engine models).

## Two claims this slice must state rather than assume

### The 6.4.24 attribution

For √und and √añj, vidyut's history credits **6.4.24** *aniditāṁ hala
upadhāyāḥ kṅiti ca* with the step `unad → und` / `anaj → anj`:

```
6.4.23    ["unad", "tas"]      unand -> unad, the root's own n
6.4.24    ["und",  "tas"]      unad  -> und
6.4.111   ["und",  "tas"]      (no change)
```

That deletion is the one our **6.4.111** *śnasor allopaḥ* already performs on
curated √bhañj (`Ba|na|j → Ba|n|j`), and 6.4.111 is the correct credit: 6.4.24
deletes a nasal upadhā, and once 6.4.23 has run there is no nasal upadhā left
to delete — `unad`'s upadhā is `a`. Our path is 6.4.23 then 6.4.111, unchanged
and already ordered that way for exactly this reason.

This is the same *class* of question as the 8.2.30 episode — a surface both
accounts reach, distinguishable only by trace — with the polarity reversed:
there the engine's intermediate was wrong and the surface accidentally right;
here the engine's intermediate is right and it is vidyut's labelling that is
the odd one. It goes in a trace pin, not only a comment, because a reviewer
diffing the two histories will otherwise read a missing rule.

### √und makes a unit-test-only arm golden-reachable

`vrddhi_of_ac_vowels_all_arms` in `tinanta/sound.rs` exists because of a claim
that this slice falsifies:

> the curated roots only ever drive `vrddhi_of` through 6.1.90 with e/I/E
> inputs (never a/A/u/U/o/O), leaving those arms unreachable via golden
> derivations

√und's laṅ is `Onat` — āṭ + `u` → `O`. The `u` arm stops being unit-test-only.
The unit test stays (the remaining arms are still unreachable); its comment
must stop saying `u` is among them.

√und is also the fork-heavy root of the eight: **58 forms over 36 cells**,
against 43–44 for each of the others. Its loṭ madhyama eka holds six —
`unDi` / `undDi` / `untAd` / `untAt` / `unttAd` / `unttAt` — which **ties**
rather than breaks the current record held by √kṛt, √rudh, √bhid, √kṣud and
√tṛd. README's "nothing in the suite forks deeper than six" survives; its
enumeration of six-way cells gains √und.

## Verification

**The audit, negative control first.** Copy `tools/audit/panini_full_audit.rs`
into a vidyut checkout at `8da2f90bee3ce1c07505fa432fc3729e3f7e02ea` — copy it,
never rewrite it; three slices rebuilt it from scratch before it was committed.
Run `PANINI_AUDIT_PERTURB=entry` and confirm it exits 1 *before* believing a
clean run. The harness asserts corpus invariants rather than reporting what it
enumerated, so 55/2304/2654 → 63/2592/3014 changes in the same commit as the
rows. The audit runs **after** the table rows and **before** the goldens:
`PARADIGM` and `ALTERNATES` are transcribed from audited output, never from a
probe.

**Goldens.** 288 `PARADIGM` cells, 72 `ALTERNATES` rows, and
`derivation_set_shape_matches_the_audited_numbers` moved to the new totals.
`pada_ambiguous_surfaces_are_exactly_these` buckets across *all* roots, not per
root, so √vid's ātmanepada surfaces could in principle collide with another
root's parasmaipada cell. Do not reason about whether they do: assert against
`Vec::<&str>::new()` and read the real set off the failure, as 7c did.

**Trace pins**, three, each load-bearing rather than coverage:

1. √und `unand → unad → und`, crediting 6.4.23 then 6.4.111 — the 6.4.24
   question above, pinned.
2. √und laṅ `Onat`, through 6.1.90's `u` → `O` arm.
3. √añj `anaj → anj → ang → aNk`, showing 6.4.111, 8.2.30, 8.3.24, 8.4.55 and
   8.4.58 in order on a vowel-initial root.

**The mutation gate: re-measure and record.** No suite-speed work and no cap
change in this slice. Measure the uncontended floor with a standalone
`mise run test` (paradigm, roundtrip and trace separately), multiply by the
measured 1.70× `-j 4` contention factor, and sanity-check against the 2400s cap
*before* launching the campaign. Then run `mise run mutants`, which now carries
`-j 4 --timeout 2400` itself — run it through the task rather than
reconstructing the flags, and invoke the `cargo-mutants` binary directly rather
than the mise shim if backgrounding it. Check **both** `missed.txt` and
`timeout.txt`. Exit code 3 is expected while the known-permanent `tripadi.rs`
non-terminating-loop mutant exists; its recorded line number will shift if any
line moves above it, and a shifted line number is not a new timeout.

Record in `AGENTS.md`: the floor, the campaign counts, and the over-600s
mutant count — that last is the series to watch (4 → 44 → 46 across the last
three slices), not the max.

Projection, for comparison against the measurement rather than in place of it:
floor ~850–880s, ~1450–1500s at `-j 4`, a ~1.6× margin under 2400 where today's
is 2.03×. **Cell count has failed as a multiplier in both directions three
times now** — flat from 1800 to 1872, +38% for +15% into 7c, +13.8% for +6.7%
into the last slice — so this projection is a tripwire, not a substitute for
measuring. If the measurement lands materially worse, raise the cap in the same
slice and say so; do not ship a thin margin quietly.

## The doc claims this slice falsifies

Eight sites. A checklist, not a sweep: past slices have shipped with counts
stale in exactly one file.

| site | what changes |
|---|---|
| `README.md` | rudhādi 13 → 21 roots; "12 of the 25 remain out" → 4; 55 → 63 roots; 2304 → 2592 cells and the form total; the six-way fork enumeration gains √und |
| `docs/ARCHITECTURE.md` | the rudhādi paragraph, and its "nine further reachable non-ubhayapadī roots" sentence, which 7d reduces to √tṛh alone |
| `AGENTS.md` | suite-size figures; the recorded audit result; the new floor and campaign numbers |
| `crates/panini-data/src/lib.rs` | the rudhādi deferral comment; `rudhadi_rows_are_the_thirteen_curated_roots` → twenty-one; `dhatus().len()` |
| `crates/panini-prakriya/src/tinanta/sound.rs` | `vrddhi_of_ac_vowels_all_arms`' "never a/A/u/U/o/O", now false for `u` |
| `tools/audit/README.md` | "Last recorded result" |
| `crates/panini/tests/paradigm.rs` | the doc comments carrying the audited numbers |
| `data/ATTRIBUTION.md` | a 7d entry naming the eight entries, in the form the 7a/7b/7c entries take |

`data/ATTRIBUTION.md` records per-entry discrepancies against upstream. None of
the eight stores a `code` that differs from its it-stripped upadeśa, so no
per-entry deviation needs recording — unlike `07.0019`, stored post-7.1.58. The
7d entry states the eight and that fact.

## If the audit shows a difference

The probe compared **sūtra sets**, not surfaces: strong evidence that no engine
work is needed, and not the byte-for-byte comparison. The audit is the only
thing that can falsify it, and the posture is fixed in advance so the slice
does not become an open-ended chase.

**Ship what passes; defer the rest with a sourced note.** A root whose cells
differ joins the deferral bucket with the actual sūtra at fault named —
measured, not guessed — in the form the 6.1.73 / 8.4.40 deferral takes today.
Counts, prose and totals then reflect whatever actually shipped: a partial
slice states its own partiality rather than carrying forward numbers it did not
reach.

What the slice explicitly does **not** do is expand to implement whatever the
audit turns up. That is what 7e is for. The likeliest single candidate is √und
or √añj, if vidyut's 6.4.24 turns out to do something our 6.4.111 does not.

## Deliberately out of scope

`ALTERNATES` reaches 422 rows and `crates/panini/tests/paradigm.rs` passes
5,400 lines. Splitting the golden file is worth doing and is its own slice:
performing a large mechanical diff inside a curation slice would put it
directly next to the data the audit is there to validate.

## Ordering

1. Eight `Dhatu` rows; `rudhadi_rows_…` renamed and extended; `dhatus().len()`.
2. Corpus totals in `paradigm.rs` and `panini_full_audit.rs`.
3. Cross-implementation audit, negative control first. Record the result.
4. `PARADIGM` and `ALTERNATES` transcribed from audited output.
5. The three trace pins.
6. `mise run test` floor measurement, then `mise run mutants`; both
   `missed.txt` and `timeout.txt` checked.
7. The eight-site documentation sweep.
