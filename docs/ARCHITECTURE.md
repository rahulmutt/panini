# Architecture

Data flow for `check`:
`input → panini-lipi (→SLP1) → panini-analyze (candidates) → panini-prakriya
(derive each) → panini (keep exact matches) → render`.

## Crates
- `panini-lipi` — SLP1 ⇄ IAST/HK/Devanāgarī + scheme detection. No grammar.
- `panini-data` — curated roots, keyed by dhātupāṭha entry number and each
  tagged with its gaṇa and pada, tiṅ table, enums. No I/O beyond embedded
  data.
- `panini-prakriya` — the engine: `Term`/`Prakriya`/`Context` model, it-samjna,
  and `TINANTA_RULES`, the ordered rule list that `tinanta::derive` runs via
  `run_pipeline`. Pure SLP1, no I/O.
- `panini-analyze` — proposes candidate `(root, lakāra, pada, puruṣa, vacana)`
  inputs, one per pada the root admits (`PadaAssignment::padas()`), so an
  ubhayapadī root proposes both.
- `panini` — facade: `Panini::check` / `Panini::derive`, `Verdict`, `Analysis`.
- `panini-cli` — the `panini` binary (`check` subcommand; `--trace`, `--json`,
  `--out`, validity exit codes).

## The rule pipeline

`TINANTA_RULES` (in `crates/panini-prakriya/src/tinanta/mod.rs`) is an
ordered `&[&[Rule]]` — seven pipeline stages, each in its own file — covering
all four lakāras. Each rule self-guards on `Prakriya.ctx` (lakāra, pada,
puruṣa, vacana) and returns whether it fired. Reading the stages in order,
and the rules within each stage in order, IS reading the grammar this crate
implements; `tinanta::rules()` yields that flattened sequence.

| stage file | rules | position |
|---|---|---|
| `samjna.rs` | 1.3.12, 1.3.72, 1.3.78, 3.4.78, 1.3.9, 1.2.4 | before 3.1.68 |
| `tin.rs` | 3.4.85 … 3.4.102, 7.1.35 | before 3.1.68 |
| `vikarana.rs` | 3.1.69, 3.1.73, 3.1.77, 3.1.78, 3.1.81, 3.1.68, 2.4.72, 3.4.111, 3.1.83, 1.2.4 | contains 3.1.68 |
| `anga.rs` | 6.4.71 … 7.2.81, 6.4.23 | after 3.1.68 |
| `guna.rs` | 7.4.21, 7.3.84, 7.3.86, 7.3.84 (again — see below), 6.4.87, 6.4.77, 6.1.78, 7.3.101, 6.4.112, 6.4.113 — vowel gradation and vikaraṇa reshaping | after 3.1.68 |
| `adesha.rs` | 6.1.101 … 6.4.107, 6.4.101, 6.4.111 | after 3.1.68 |
| `tripadi.rs` | 8.2.77, 8.2.23, 8.2.25, 8.2.30, 8.2.39, 8.2.40, 8.2.41, 8.2.74, 8.2.75, 8.2.73, 8.3.15 … 8.3.59, 8.4.41, 8.4.53, 8.4.55, 8.4.1, 8.4.2, 8.4.58, 8.4.65, 8.4.56 | after 3.1.68 |

The stage boundary is file organisation, not grammar: the flattened order is
what matters, and `tinanta_rule_order_is_pinned` in `derivation_tests.rs`
pins all 87 ids verbatim (72 pre-rudhādi, the fifteen rudhādi added:
3.1.78, 6.4.23, 6.4.111, 8.2.74, 8.2.75, 8.2.73, 8.3.24, 8.4.53, 8.4.58 and
8.4.65 in slice 7a, then 8.2.30, 8.2.40, 8.2.41 and 8.4.41 in 7b, and
1.3.72 *svaritañitaḥ*, which arrived with √rudh in the ubhayapada slice).
`tinanta/terms.rs` holds the term-index constants and the reason 3.1.68
bisects the pipeline; `tinanta/sound.rs` holds the varṇa classifiers.

Rule order is load-bearing and several orderings are non-obvious; the
constraints and their justifications are documented in the design specs
under `docs/superpowers/specs/`. The exact ordered traces in
`crates/panini/tests/trace.rs` are what pin them.

Six gaṇas are covered: bhvādi (1), divādi (4), tudādi (6), adādi (2), kryādi
(9), svādi (5) — plus rudhādi (7), **partial** (see below). gaṇa is carried
as a tag on the aṅga term (`Tag::Divadi` / `Tag::Tudadi` / `Tag::Adadi` /
`Tag::Kryadi` / `Tag::Svadi` / `Tag::Rudhadi`, mirroring how
`Tag::Atmanepadin` and `Tag::Ubhayapadin` carry pada), read by 3.1.69,
3.1.73, 3.1.77, 3.1.78, 3.1.81, and 2.4.72. The vikaraṇa itself is selected
by 3.1.68 (śap, bhvādi
and adādi), 3.1.69 (śyan, divādi), 3.1.73 (śnu, svādi), 3.1.77 (śa, tudādi),
3.1.78 (śnam, rudhādi), and 3.1.81 (śnā, kryādi).

adādi (gaṇa 2) is the only gaṇa where the vikaraṇa is *luk'd*: 3.1.68 still
inserts śap (bhvādi and adādi share the same vikaraṇa rule), and **2.4.72
*adiprabhṛtibhyaḥ śapaḥ*** then empties it for adādi roots. The śap term is
kept in place with empty text rather than removed, so the `ANGA`/`SHAP`/
`ENDING` term indices stay stable for downstream rules.

rudhādi (gaṇa 7) stretches the same three fixed slots the other way: its
vikaraṇa, śnam (3.1.78), is the engine's first **infix** rather than a
suffix, and there is no fourth slot to hold one. The root is instead split
across `ANGA` and `SHAP` — `ANGA` keeps the head through the root's last
vowel, `SHAP` holds śnam followed by whatever text of the root followed
that vowel (`kft` → `[kf, nat, ti]`; `hins` → `[hi, nans, ti]`) — so
`terms[SHAP].text` is no longer purely the vikaraṇa's own text for this
gaṇa, the way `terms[SHAP].text` may be empty for adādi. A rule reading
`SHAP` to detect "the vikaraṇa's own shape" must guard on `Tag::Rudhadi`
accordingly; see the "REPRESENTATION" note on 3.1.78 in
`tinanta/vikarana.rs` and the caveat in `tinanta/terms.rs`.

The gaṇa carries twenty-one roots — √kṛt, √hiṃs and √khid (slice 7a), √bhañj,
√piṣ and √indh (7b), √rudh (`07.0001`), the gaṇa's own eponym, which arrived
with 1.3.72 *svaritañitaḥ* in the ubhayapada slice as the engine's first
ubhayapadī root (the pada audit later added √nī and √tud, outside this
gaṇa), √bhid (`07.0002`), √kṣud (`07.0006`), √yuj (`07.0007`) and √tṛd
(`07.0009`), curated in slice 7c and all four ubhayapadī by 1.3.72, √ric
(`07.0004`) and √vic (`07.0005`), curated in the 8.2.30/8.2.39 generalization
slice and likewise ubhayapadī, and — curated in slice 7d, on the audited
numbers alone with no new sūtra — √vid (`07.0013`), √śiṣ (`07.0014`), √und
(`07.0020`), √añj (`07.0021`), √tañc (`07.0022`), √vij (`07.0023`), √vṛj
(`07.0024`) and √pṛc (`07.0025`), all eight of them non-ubhayapadī — and it
stays **partial**: nine of rudhādi's
25 dhātupāṭha roots are ubhayapadī, seven of the nine now curated. The
ubhayapada deferral itself is discharged — 1.3.72 holds none of the remaining
two back any more — but the roots still out are not all alike.
**√bhid, √kṣud, √yuj and √tṛd** were the four described as curation-only for
months without a run behind the description. Slice 7c curated them and ran
the audit: zero differences against vidyut-prakriya at commit
`8da2f90bee3ce1c07505fa432fc3729e3f7e02ea`, across the whole corpus of 2160
cells / 2496 forms / 53 roots, with the `entry` negative control verified
failing first (exit 1, 36 √bhū cells flagged). **√ric and √vic**, both
c-final, were the ones 8.2.30 *coḥ kuḥ* could not reach: its match named `j`
alone, and its substitute was a literal `'g'` rather than the 1.1.50 nearest
velar its own comment described. Widening only the match would still have
reached the right surface (`riRakti`, since 8.4.55 *khari ca* devoices the
resulting `g` to `k`), but through a wrong intermediate, so the 8.2.30/8.2.39
generalization slice replaced both the match and the substitute with one
`kutva_of` map (cu → ku) — the substitute *is* the map, not a case split, so
match and substitute read the same lookup rather than risk drifting apart
the way widening only the match would have left them.
That fix then exposed a second sūtra this slice's own design had
deliberately deferred, not 7c's: 7c touched no engine code at all, and the
guard's own history runs through the ubhayapada slice, which last widened
8.2.39 *jhalāṁ jaśo'nte*'s guard to its `t`/`z`/`D` three-literal shape
(the `D` arm, for √rudh). That three-literal guard had never had to
classify a voiceless word-final velar, because no curated root had produced
one before. With 8.2.30 now correctly producing `k`, √ric's and √vic's
declined laṅ forms fell through that guard instead of reaching `ariRag`/
`avinag`. The slice's cross-implementation audit caught the four resulting
differing cells on its first run — a real defect in real code, not only the
synthetic `entry` control catching a planted one — and 8.2.39 was
generalised the same way, to one
`jashtva_of` map read on both sides (plus a no-op guard for the table's fixed
points); the audit's second run came back clean. **√chid and √chṛd** need
two sūtras this engine does not implement, 6.1.73 *che ca* (the tuk augment
before a `C` after a short vowel) and 8.4.40 *stoḥ ścunā ścuḥ* (the ścutva
that follows), without which their laṅ cells surface `aCinat` where vidyut
has `acCinat`.
Slice 7d curated eight further reachable non-ubhayapadī roots — √śiṣ, √und,
√añj, √tañc, √vij, √vṛj, √pṛc and √vid — and its probe found that all eight
needed no sūtra this engine lacks: every one of them derives, and audits
clean, on the rules already in the pipeline. That leaves a ninth reachable
non-ubhayapadī root, √tṛh, which does need something the engine does not
have — three sūtras, 7.3.92 *tṛṇaha im* (the *im* augment), 8.2.31 *ho ḍhaḥ*
and 8.3.13 *ḍho ḍhe lopaḥ* — so it stays out on that named cost, deferred to
slice 7e; and
the twenty-fifth, √bhuj (`07.0017`), is out on different grounds again —
1.3.66 *bhujo'navane* forks its pada on sense rather than on an axis this
engine models. Six roots is the size every completed gaṇa *after bhvādi*
has here — bhvādi, the first, has twelve — so the root count is not what
makes this one partial; rudhādi is already past it at twenty-one. Nor is
1.3.72 any longer: what is left is the two-sūtra gap that keeps √chid and
√chṛd out, √tṛh's three-sūtra gap, and √bhuj's sense axis —
**4 of the 25 in all**.

Pada is a **context coordinate**, not a branch: an ubhayapadī root
contributes *two* `PARADIGM` blocks per lakāra, one per pada, so a
(root, lakāra, puruṣa, vacana) of such a root names two distinct cells
rather than two readings of one cell. That is why 1.3.72 is deliberately
absent from the vikalpa set — an optional rule forks
one cell into several forms, whereas 1.3.72 sanctions a second cell that the
harness enumerates in its own right. `Dhatu.pada` is a `PadaAssignment`
(`Parasmaipada` / `Atmanepada` / `Ubhayapada`) and `padas()` expands it;
`Context.pada` stays the two-valued `Pada`, because no derivation may request
an "ubhayapada" cell — no such cell exists. 1.3.72's semantic condition
(*kartrabhiprāye kriyāphale*) is not modelled: both padas derive, and the
reader selects by sense.

adādi is now **complete** across all four lakāras: √yā and √vā, √ad
(parasmaipada), and √ās, √vas and √śī (ātmanepada) each derive in laṭ, laṅ,
loṭ and vidhiliṅ, including the athematic (śap-luk'd) ātmanepada path. The
consonant-final adādi roots meet the ending directly, so they are the
engine's junction witnesses: 8.4.55 cartva (√ad) and 8.2.25 dhi ca, which
elides an aṅga-final `s` before a Dh-initial affix (ADve, vaDve). √śī is the
gaṇa's only guṇa witness and lands the engine's first ṣatva; see
`docs/superpowers/specs/2026-07-25-adadi-si-5f-design.md` for the rule
analysis.

kryādi (gaṇa 9) is thematic — śnā occupies the same `SHAP` slot as śap, śyan
and śa — but it is the first gaṇa whose vikaraṇa is itself reshaped by the
ending: 6.4.112 elides its `ā` before a vowel-initial kṅit sārvadhātuka
(kliSnanti), 6.4.113 turns it into `ī` before a consonant-initial one
(kliSnItaH), and 3.1.83 replaces it wholesale with śānac before `hi` after a
consonant-final root (kliSAna). That split is driven by 1.2.4, which since
the kryādi slice tags parasmaipada apit endings ṅit as well as ātmanepada
ones — the distinction between pit `tip` (kliSnAti) and apit `tas`
(kliSnItaH) is the whole paradigm.

svādi (gaṇa 5) is thematic like divādi, tudādi and kryādi — its vikaraṇa is
śnu (3.1.73) — but it is the first gaṇa where 7.3.84's guṇa is not blocked
outright by 1.1.5: śnu is apit, so guṇa reaches it, and the ik that guṇates
belongs to the vikaraṇa rather than the root. That forces four rules to stop
reading "the aṅga's final sound" as shorthand for "the root's final sound",
because 1.4.13 *yasmāt pratyayavidhis tadādi pratyaye'ṅgam* makes the aṅga
affix-relative and svādi is the first gaṇa where the two readings diverge
visibly: **7.3.84** now has a second `Rule` entry (same id, ordered right
after the first) that guṇates `terms[SHAP]` with respect to `terms[ENDING]`
(`Ap` + `nu` + `ti` → `Apnoti`, blocked before ṅit `tas` → `ApnutaH`);
**6.1.78** *eco'yavāyāvaḥ* gained a vikaraṇa arm so the resulting `o`
converts to `av` before a vowel (`ApnavAni`); **6.1.90** *āṭaś ca*'s
athematic arm widened from `SHAP.is_empty()` to "any non-`a`/`A`-final
SHAP", since a non-empty `SHAP` no longer implies the root abuts the
ending; and **6.4.101** *hujhalbhyo her dhiḥ* was rewritten to read the
sound immediately before the ending rather than the aṅga's own final sound.
None of the four is optional and each is revealed by a different cell — see
`docs/superpowers/specs/2026-07-29-svadi-gana-design.md`'s "Three existing
rules assume the aṅga is the root" for the full analysis, and
`AGENTS.md`'s note on 7.3.84 / 1.2.4 appearing twice by design.

The gaṇa's other rules turn on a single predicate, *asaṁyogapūrva* — whether
śnu's `u` is preceded by a conjunct: 6.4.87 *huśnuvoḥ sārvadhātuke* (yaṇ,
apavāda) and 6.4.77 *aci śnudhātubhruvāṁ yvor iyaṅuvaṅau* (uvaṅ, utsarga)
split `hinvanti` from `Apnuvanti` before a vowel-initial ending, and 6.4.106
*utaś ca pratyayād asaṁyogapūrvāt* splits `hinu` from `Apnuhi` at `hi`.

8.4.1 / 8.4.2 are the engine's first ṇatva. They are guarded to skip an `n`
that is word-final or immediately followed by a jhal — the effect of 8.4.37
*padāntasya* and of 8.3.24 *naś cāpadāntasya jhali* bleeding the rule, neither
of which is modelled here because the engine has no anusvāra machinery. The
guard is exactly equivalent within tripādī order; it costs trace fidelity, and
it is the first thing liṭ and luṅ will want retired. `asmaran` and `BAzante`
are the goldens that pin it.

`derive` itself carries no scope gate — it only tags the dhātu and
runs `TINANTA_RULES` (see `panini_prakriya::tinanta::derive`). A wrong-pada
derivation is instead blocked by the pada-sanction rules (1.3.12 *anudāttaṅita
ātmanepadam* / 1.3.78 *śeṣāt kartari parasmaipadam*), which set
`Prakriya.blocked` when the requested pada doesn't match the root's tag; a
blocked prakriya's partial text never counts as a match in `Panini::check`.
1.3.72 *svaritañitaḥ* is the third of them and the only one that never
blocks: it sanctions the ātmanepada of a root tagged `Tag::Ubhayapadin`, and
1.3.78's ātmanepada arm declines rather than blocks for such a root, so both
padas derive. `INVALID` means "not derivable within the covered grammar,"
not "ungrammatical in Sanskrit." Coverage of the enumerable (root × lakāra)
space is pinned by
`crates/panini/tests/paradigm.rs::paradigm_covers_every_enumerable_cell`.

## Optional rules and the derivation set

`derive` returns `Vec<Prakriya>`, not one. Most cells yield exactly one
branch; a cell forks when an optional (*vikalpa*) rule — a sūtra saying
*anyatarasyām* / *vā* / *vibhāṣā* — actually fires, and both readings are
valid Sanskrit reported side by side.

`run_pipeline` carries the branches as a worklist. At a `vikalpa` rule it
clones each live branch and applies to the clone, keeping it only if
`apply` returned true; a rule that declines its own guard therefore forks
nothing. The declined branch keeps its index and the applied clone is
inserted immediately after it, so index 0 is always the no-optional-rules
derivation. Forks are collected during a rule's sweep and inserted after
it, so no branch sees a list another branch's fork has already mutated.
The branch count is the number of distinct subsets of optional rules that
actually apply on a cell, which is bounded by 2^k (k = the number of optional
rules) and reaches it only when every optional rule fires on every branch.
It is not always 2^k in practice: loṭ prathama eka has k = 2 (7.1.35 and
8.4.56 both apply) but only **three** branches, not four, because 8.4.56
declines on the vowel-final base branch (`Bavatu`) and forks only the
tātaṅ one (`BavatAt`, alongside `Bavatu` and `BavatAd`). rudhādi's √kṛt loṭ
madhyama eka sharpens the same gap: k = 3 there (7.1.35, 8.4.65 and 8.4.56
all apply), for a 2³ bound of eight, but only **six** branches result
(`kfndDi`, `kfnDi`, `kfnttAd`, `kfntAd`, `kfnttAt`, `kfntAt`) — again
because 8.4.56 declines on the vowel-final, non-tātaṅ branches (`kfndDi`
and its 8.4.65 fork `kfnDi`), so only the two tātaṅ branches (7.1.35 alone,
and 7.1.35+8.4.65) go on to fork a third time. That six-form cell is no
longer alone at the top, and now has five companions rather than one.
√rudh's loṭ **parasmaipada** madhyama eka tied it first,
with the same k = 3 (7.1.35, 8.4.65, 8.4.56), the same 2³ bound of eight,
and the same six branches — `rundDi`, `runDi`, `rundDAd`, `runDAd`,
`rundDAt`, `runDAt` — stopping at six for exactly the reason √kṛt's does:
8.4.56 declines on `rundDi` and its 8.4.65 fork `runDi`, and only the two
tātaṅ branches fork a third time. Slice 7c's √bhid, √kṣud and √tṛd then tied
it three more times, cell for cell and mechanism for mechanism:
`BindDi`/`BinDi`/`BinttAd`/`BinttAt`/`BintAd`/`BintAt`,
`kzundDi`/`kzunDi`/`kzunttAd`/`kzunttAt`/`kzuntAd`/`kzuntAt`, and
`tfndDi`/`tfnDi`/`tfnttAd`/`tfnttAt`/`tfntAd`/`tfntAt`. Rudhādi 7d's √und
ties it a fifth time, on the same k = 3 and the same mechanism:
`undDi`/`unDi`/`unttAd`/`unttAt`/`untAd`/`untAt`. So **six cells hold
six forms and six hold five** — the loṭ parasmaipada *prathama* eka of each
of those same six roots is a five-form cell — and nothing in the suite
exceeds six. The fourth root 7c curated, √yuj, is the instructive near-miss:
ubhayapadī like the other three, but not dental-final. 8.2.30 *coḥ kuḥ*
replaces its stem-final palatal `j` with the **velar** `g` — the substitute is
a literal `'g'` in `tripadi.rs`, and it is 8.4.55 *khari ca*, later, that
devoices it to `k` before the `t` of tātaṅ. So the junction 8.4.65 would need
is velar against dental at both sites — `g` + `D` in `yuNgDi`, `k` + `t` in
`yuNktAd` — and never savarṇa the way the dental-final roots' `d` + `D` and
geminate `t` + `t` are. 8.4.65's site never arises, and √yuj's two loṭ
parasmaipada eka cells stop at three forms
(`yunaktu`/`yuNktAd`/`yuNktAt` and `yuNgDi`/`yuNktAd`/`yuNktAt`).
The deepest cell slice 7b adds
is shallower: √piṣ's
loṭ madhyama eka holds **four** forms — `piRqQi`, `piRQi`, `piMzwAd`,
`piMzwAt` — with the same k = 3 (7.1.35, 8.4.65 and 8.4.56 all apply) and
the same 2³ bound of eight. It stops at four because 8.4.65 and 8.4.56 never
reach the same branch here: 8.4.65 forks the non-tātaṅ branch only
(`piRqQi` → `piRQi`) and 8.4.56 the tātaṅ one only
(`piMzwAd` → `piMzwAt`), where for √kṛt 8.4.65 forks the tātaṅ branch as
well and the two compound into `kfntAt`. √indh is the flat case at the
other end — each of its seven forked cells forks on 8.4.65 alone, two
branches apiece. Branches that converge on the same text are not
deduplicated — one form with two derivations is information, not noise.

Seven rules are optional, in pipeline order: **7.1.35** *tuhyos tātaṅ āśiṣy
anyatarasyām*, **3.4.111** *laṅaḥ śākaṭāyanasyaiva*, **6.4.107** *lopaś
cāsyānyatarasyāṁ mvoḥ*, **8.2.74** *sipi dhāto rur vā*, **8.2.75** *daś ca*,
**8.4.65** *jharo jhari savarṇe*, and **8.4.56** *vā'vasāne*. Three of the
seven arrived with rudhādi 7a: 8.2.74 and 8.2.75 the *ru* alternation on
√hiṃs's own final before sip, and 8.4.65 the savarṇa elision that also
produces the six-branch witness above. Slice 7b added none — the
first gaṇa slice not to since the `vikalpa` flag landed. See
`exactly_the_pinned_vikalpa_rules_are_optional` in `derivation_tests.rs`,
which pins the whole set by id.

6.4.107 elides śnu's `u` before `m` and `v` when that `u` is
*asaṁyogapūrva*, forking 8 cells: √hi and √ri (the gaṇa's only
asaṁyogapūrva roots) in laṭ and laṅ uttama dvi/bahu, whose
`vas`/`mas`/`va`/`ma` are the only m/v-initial endings in scope — `hinvaH ~
hinuvaH`, `ahinma ~ ahinuma`. 6.4.108 *nityaṁ karoteḥ*, which makes the
same lopa obligatory for √kṛ and is what makes this rule optional, is out
of scope with √kṛ itself.

7.1.35 optionally replaces the loṭ endings `tu`/`hi` with tātaṅ (then
8.2.39 obligatorily voices its final `t` to `d`), forking 70 cells (loṭ
prathama and madhyama eka across the 35 roots with a parasmaipada column —
`tu`/`hi` are parasmaipada endings, so the curated set's 20 ātmanepada-only
roots never reach this guard, and the nine ubhayapadī roots √rudh, √nī,
√tud, √bhid, √kṣud, √yuj, √tṛd, √ric and √vic reach it
in their parasmaipada cells only; 35 + 20 = the 55 curated
roots) — `Bavatu ~ BavatAd`, `Bava ~ BavatAd`. 8.4.56
optionally devoices a pada-final jaś (produced by the now-obligatory 8.2.39)
back to its car at the end of an utterance, forking 81 cells outright: laṅ
and vidhiliṅ prathama eka across those same 35 parasmaipada columns (70 of
them — 8.2.39's `d` is a parasmaipada-ending artifact, ātmanepada's
laṅ/vidhiliṅ prathama eka endings are vowel-final and never reach a jhal),
plus eleven rudhādi laṅ *madhyama* eka cells (√kṛt, √hiṃs, √bhañj, √piṣ,
√rudh, √bhid, √kṣud, √yuj, √tṛd, √ric and √vic — every rudhādi root with a
parasmaipada column), where 8.2.23
*saṁyogāntasya lopaḥ* has eaten the ending's own `s` and left the stem's
jaś pada-final after all — `aBavad ~ aBavat`, `Baved ~ Bavet`,
`apinaq ~ apinaw`, `aruRad ~ aruRat` — and forking a further 70 (the same
loṭ cells 7.1.35 just forked) by devoicing the tātaṅ branch's `BavatAd` to
`BavatAt`, which is
what stacks the two rules into the three-branch loṭ cells above rather
than a fourth branch: 8.4.56 declines on the vowel-final base branch
(`Bavatu`), so only the tātaṅ branch forks again. 3.4.111 optionally
applies Śākaṭāyana's jus in laṅ prathama bahu after an ā-final aṅga **with
no live vikaraṇa** — adādi's śap is luk'd by 2.4.72, so its aṅga stands
directly before the ending, but kryādi's ā-final śnā vikaraṇa (reduced only
later by 6.4.112/6.4.113) would otherwise read as the same condition and
fork a spurious form — forking 2 cells — √yā and √vā, the only roots in
scope that meet it — `ayAn ~ ayuH`, `avAn ~ avuH`.

`CheckResult.analyses` needed no change: it was already a `Vec<Analysis>`,
since one surface form can already have several analyses. A fork adds
members to a list that already existed, and no CLI or `--json` shape
changed.

## Rule trace
Every applied sūtra is logged as a `RuleStep { sutra, name, before, after }`.
The `check` result carries the full trace per analysis.
