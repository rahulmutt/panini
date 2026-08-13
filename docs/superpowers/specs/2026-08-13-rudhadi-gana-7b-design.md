# rudhādi (gaṇa 7), slice 7b — the consonant families

Slice 7a landed śnam, the engine's first infix, and the representation that
makes it fit a pipeline with three fixed term slots. It paid for that
mechanism on three roots chosen to need nothing else: √kṛt, √hiṃs and √khid.

7b spends the mechanism. Its three roots — √bhañj, √piṣ, √indh — add no new
structure at all. What they add is the tripādī consonant sandhi that a
consonant-final rudhādi stem walks through: velarisation, retroflexion, and
the voiced-aspirate assimilation that a `dh`-final stem forces on a `t`-initial
ending. Four new sūtras, no new optional rule, and rudhādi reaches six roots.

The gaṇa remains **partial**, and for the same reason as in 7a: 1.3.72
*svaritañitaḥ* is still deferred, so nine of rudhādi's 25 roots — including
√rudh, the eponym — stay out. More roots would not change that; only 1.3.72
will.

## Scope

New: three roots, four sūtras, three guard widenings, two `sound.rs` helper
changes, and four of the six decisions 7a deferred. 108 new cells, 130 new
forms.

| id | dhātupāṭha | pada | laṭ prathama eka |
| --- | --- | --- | --- |
| `Banj` | 07.0016 `Ba\njo~` *āmardane* | parasmaipada | `Banakti` |
| `piz` | 07.0015 `pi\zx~` *sañcūrṇane hiṁsāyāṁ ca* | parasmaipada | `pinazwi` |
| `inD` | 07.0011 `YiinDI~\` *dīptau* | ātmanepada | `inDe` |

Counts: `PARADIGM` 1620 → **1728** cells, `ALTERNATES` 191 → **213** rows,
1811 → **1941** forms, 45 → **48** roots. The vikalpa set stays at **seven**
rules; 7b adds no optional rule.

None of the three ids collides with an existing SLP1 code, so the `aS.5`
qualification mechanism stays at one user. 7a's spec predicted otherwise —
"that does not survive 7b", on the strength of rudhādi's `vi\da~\` and
`o~vijI~` colliding with divādi's `vid` and tudādi's `vij` — but that
prediction assumed a root set including √vid. This one excludes it, so the
collision is deferred along with the root.

Out of scope, deferred:

- **Ubhayapadī roots and 1.3.72 *svaritañitaḥ***, unchanged from every gaṇa
  spec so far. The nine `~^r`-marked roots (`ru\Di~^r`, `Bi\di~^r`,
  `Ci\di~^r`, `ri\ci~^r`, `vi\ci~^r`, `kzu\di~^r`, `yu\ji~^r`, `u~Cfdi~^r`,
  `u~tfdi~^r`) stay out, as does `Bu\ja~`, which 1.3.66 *bhujo'navane* forks
  on sense rather than on a pada axis this engine models.
- **The nine other reachable rudhādi roots**: √śiṣ, √tṛh, √und, √añj, √tañc,
  √vij, √vṛj, √pṛc and √vid. Six roots is the size every completed gaṇa in
  this engine has, and these nine bring new machinery of their own — 7.1.58
  *idito num dhātoḥ* for √und, 6.4.24 for √añj and √tañc, and the two id
  collisions above for √vid and √vij.
- **6.4.24 *aniditāṁ hala upadhāyāḥ kṅiti***. 7a's spec listed this among
  "7b's consonant families"; it is not needed here. See "6.4.23 needs no
  widening" below.

### √indh is not ubhayapadī, despite its ñi

`YiinDI~\` carries a `Yi` (ñi) it-marker, and 1.3.72 reads **ñit** as well as
svarita. That would put √indh behind the same deferral as √rudh and collapse
this slice's ātmanepada arm, so it was checked rather than assumed:
vidyut-prakriya derives √indh in ātmanepada only, and derives both padas for
the `~^r` control (√rudh gives `ruRadDi` alongside `runDe`/`rundDe`). The
root's anudātta `~\` fixes its pada by 1.3.12 *anudāttaṅita ātmanepadam*, and
1.3.72's semantic condition (*kartrabhiprāye kriyāphale*) does not default on.
√indh is genuinely reachable.

## The root set, and why these three

Each root pays for exactly one phonological family, and each family has a
witness that would be wrong without it:

- **√bhañj** is the palatal-to-velar root. `Banaj` + `ti` needs 8.2.30
  *coḥ kuḥ* to reach `Banakti`, and its laṅ eka cells reach `aBanag` the same
  way, word-finally. It is also the first root other than √hiṃs whose
  `ENDING` empties under 8.2.23 — the invariant 8.2.73's comment asks to be
  re-verified before any widening.
- **√piṣ** is the retroflex root, and the heaviest of the three: it needs
  8.4.41 *ṣṭunā ṣṭuḥ* and 8.2.41 *ṣaḍhoḥ kaḥ si*, plus widenings to 8.2.39 and
  8.3.59. Its `piMzwaH` is also the **second witness** for the conditional
  anusvāra round trip 7a built — 8.3.24 fires and 8.4.58 declines, because `z`
  is śal and not yay, exactly as for √hiṃs's `hiMstaH`. A pair of rules that
  looks like a no-op now has two independent witnesses in two different
  phonological positions.
- **√indh** is the ātmanepada arm and the voiced-aspirate root. Its stem-final
  `D` meeting a `t`-initial ending needs 8.2.40 *jhaṣas tathor dho'dhaḥ*, and
  the `indDe` that results is the shape that finally makes 8.4.53's guard
  general.

## The rules

Four new sūtras, three guard widenings, two helper changes. All four new rules
go in `TRIPADI`, in sūtra order.

### 8.2.30 `coH kuH` — √bhañj

A *cu* sound becomes its *ku* counterpart before a jhal or word-finally.

- `Banaj` + `ti` → `Banag` + `ti`, whence `Banakti` via 8.4.55.
- `aBanaj` (8.2.23 having eaten tip's `t`) → `aBanag`, word-finally.
- Weak: `Banj` + `taH` → `Bang` + `taH`, whence `BaNktaH`.

Its **declining** cases are witnessed, which is what keeps the guard from being
written too wide: `BaYjanti` and `BaYjvaH` both leave the `j` alone, because
what follows is `a` and `v` — neither a jhal nor a word end.

### 8.2.40 `JazastaTorDo'DaH` — √indh

After a *jhaṣ* (a voiced aspirate), `t` and `th` become `dh`.

`inD` + `te` → `inD` + `De`, which 8.4.53 then takes to `indDe`. The rule
reaches seven of √indh's cells: laṭ prathama eka and madhyama bahu, laṅ
prathama eka / madhyama eka / madhyama bahu, and loṭ prathama eka / madhyama
bahu.

### 8.2.41 `zaQoH kaH si` — √piṣ

`ṣ` or `ḍh` becomes `k` before `s`.

`pinaz` + `si` → `pinak` + `si`, whence `pinakzi` once 8.3.59 retroflexes the
ending's `s` after the new `k`.

**This rule must sit below 8.2.23**, and that is the slice's one load-bearing
new ordering constraint. See "Ordering" below.

### 8.4.41 `zwunA zwuH` — √piṣ

A dental (`s` or a *t*-varga sound) in contact with `ṣ` or a *ṭ*-varga sound
retroflexes to match.

- `pinaz` + `ti` → `pinazwi`
- `piMz` + `taH` → `piMzwaH`; `piMz` + `TaH` → `piMzWaH`
- `piMz` + `Di` → `piMz` + `Qi`, which continues into 8.4.53

**The guard must require strict adjacency.** `piMzanti` keeps its dental `n`
across the intervening `a`, and `pinazARi`'s retroflex `ṇ` is **ṇatva's**
(8.4.1 / 8.4.2), not ṣṭutva's — ṇatva is the rule that permits aṭ
intervention, and 8.4.2 is why. Two rules reach the same retroflexion by
different routes and must not be conflated; `piMzanti` is the witness that
separates them.

### 6.4.23 `SnAnnalopaH` needs no widening

7a's comment on this rule says "7b widens it for √bhañj, √und and √indh, whose
tails are `fj`, `nd` and `nD`". Both halves of that are wrong and should be
corrected rather than acted on:

- The guard is already `rest.starts_with('n')`, which covers √bhañj's `nj`
  and √indh's `nD` unchanged. The rule gains **witnesses**, not a widening.
- √bhañj's tail is `nj`, not `fj` — a typo.

The elision itself is unchanged in character. `Banj` splits as `Ba | na | nj`
under 3.1.78's representation; 6.4.23 removes the root's own `n`, leaving
`Ba | na | j`; 6.4.111 removes śnam's `a` in the weak cells, leaving
`Ba | n | j`. The `n` surviving in `BaNktaH` is śnam's, not the root's.

This is also why **6.4.24 is not needed**. The nasal that drops in all three
roots is the one sitting immediately behind śnam's `na`, which is 6.4.23's by
its own terms. 6.4.24 governs the penultimate nasal of roots like √añj and
√tañc, and both are out of scope.

### Widening 8.2.39 `JalAM jaSo'nte`

Guard is `p.text().ends_with('t')` with a hardcoded `s.push('d')`. √piṣ's
`apinaz` needs `z` → `q`.

Widen the guard to exactly `{t, z}` and take the substitute from
`jashtva_of`. **Deliberately not widened to all jhal**: a word-final `s` is
8.2.66 / 8.3.15's business, not jaśtva's, and a blanket widening would rewrite
√hiṃs's `ahinas` to `ahinad` before 8.2.74 and 8.2.73 could act on it,
destroying the ru alternation. Narrowing to the reachable set is also the
repo's standing discipline — every arm keeps a witness and the mutation gate
stays clean.

`jhalam_jasho_ante_fires_only_on_a_pada_final_t` needs renaming and a `z` arm.

### Widening 8.3.59 `AdeSapratyayayoH`

Guard requires the sound before the s-initial affix to be a vowel other than
`a`/`A`. `pinak` + `si` presents a `k`.

This is precisely the widening the rule's own comment predicts — "widen it the
moment a root lands whose aṅga ends in h/y/v/r/l or `k` before an s-initial
affix". Add `k` only; the remaining iṇ sounds stay unreachable and unwritten.

### Widening 8.4.53 `JalAM jaS JaSi`

Currently guarded to "the word ends in `i` and the penult is `D`" — that is,
only the `Di` ending 7a reached it through. √indh needs it at `De`, `Da`,
`DAm`, `Dve`, `DAH` and `Dvam`, and √piṣ needs it with `Q` as the conditioning
sound rather than `D`.

Generalise the condition to "the following sound is a *jhaś*". Two existing
clauses keep the widened rule off the cells it must not touch, and they are
different clauses: √khid's `KindDve` presents a `d` that is already its own
jaś, which the **no-op guard** declines; √hiṃs's `hinDi` presents an `n`,
for which **`jashtva_of` returns `None`**. Both remain index 0 of their cells.

The widening is also self-limiting upstream. 8.2.40 is the only new source of
a `D`-initial ending, and it requires a jhaṣ abutting the ending directly —
which no thematic root reaches, since the vikaraṇa intervenes (`laBate`,
`yuDyate`, `guDnAti`), and no other athematic stem in the suite ends in a
jhaṣ (`vaste`, `Aste`, `Sete` all present an `s`, `A` or `e`). √indh is alone
in reaching it.

### Helper changes in `sound.rs`

- **`jashtva_of` gains `'z' => 'q'`.** Sibilants have no jaś by place alone;
  1.1.50 *sthāne'ntaratamaḥ* selects the nearest, which for retroflex `ṣ` is
  retroflex `ḍ`. One arm serves both 8.2.39 (`apinaq`) and 8.4.53 (`piRqQi`).
  `'S'` and `'s'` stay out: unreachable here, and `s` in particular belongs to
  8.2.66.
- **A new `is_jhash`**, for 8.4.53's generalised condition.

`parasavarna_of` and `cartva_of` need nothing. 7a pinned all five varga arms of
the former, so `j → Y` (`BaYjanti`), `q → R` (`piRqQi`) and `g|k → N`
(`BaNktaH`) are already there; `cartva_of` already covers the retroflex series
that `apinaw` needs.

## Ordering

`TRIPADI` after this slice, with new rules in **bold** and the 7a reorder
marked:

> 8.2.77, 8.2.23, 8.2.25, **8.2.30**, 8.2.39, **8.2.40**, **8.2.41**, 8.2.74,
> 8.2.75 *(moved up)*, 8.2.73, 8.3.15, 8.3.24, 8.3.59, **8.4.41**, 8.4.53,
> 8.4.55, 8.4.1, 8.4.2, 8.4.58, 8.4.65, 8.4.56

**8.2.41 below 8.2.23 is load-bearing.** At laṅ madhyama eka, √piṣ presents
`apinaz` + `s`. 8.2.23 *saṁyogāntasya lopaḥ* eats the word-final `s` first,
leaving 8.2.41 no trigger, and the cell reduces through 8.2.39 and 8.4.56 to
`apinaq` / `apinaw` — the same pair laṅ prathama eka gives. Run 8.2.41 above
8.2.23 instead and the `ṣ` becomes `k` before the `s` is elided, yielding
`apinak`: a plausible-looking form that no guard test would flag and that
silently splits madhyama eka from prathama eka. It needs a golden and a trace
pin, not just file position.

**Everything else is sūtra order with no cell distinguishing it.** In
particular, 8.4.41 above 8.4.53 is *not* a constraint — checked both ways,
`piMz` + `Di` reaches `piRqQi` identically, because ṣṭutva and jaśtva touch
different sounds in either sequence. Recorded here so a later reader does not
rediscover it as load-bearing. The same holds for 8.2.30 against 8.2.23 and
against 8.3.24.

## The 7a deferred decisions

7a deferred six decisions to 7b. Four are taken here, one is re-verified, and
one is already closed.

### #3 — 8.2.73's invariant, re-verified

8.2.73 has no slot predicate. Its comment records why (the mutation gate proved
`is_tip() || is_sip()` had no witness) and what the rule leans on instead:
`ENDING` empties only because 8.2.23 collapsed a word-final conjunct, which in
7a happened only at tip and sip. The rule is obligatory, so the comment asks
for re-verification before any widening.

**Verdict: the invariant holds.** √bhañj and √piṣ are the first roots other
than √hiṃs to empty `ENDING` — `aBanajt` → `aBanaj`, `apinazt` → `apinaz` —
and both do so at laṅ prathama/madhyama eka, i.e. still tip and sip. 8.2.73
declines on them anyway, because neither `aBanaj` nor `apinaz` ends in `s`.

Record the verdict in place of the standing warning, and add goldens asserting
`aBanag` and `apinaq` are untouched by the ru alternation, mirroring
`rudhadi_vidhilin_madhyama_eka_is_untouched_by_the_ru_alternation`.

### #1 — 8.2.75 above 8.2.73

8.2.75 currently declines by reading `p.log` for a prior 8.2.73 step. "Decline
because 8.2.73 fired" is not a grammatical condition — it compensates for
8.2.73's own deliberate over-application to sip.

Move 8.2.75 above 8.2.73 and delete the `p.log` read, which is unreachable at
the new position. The forms are unchanged, on phonology rather than on rule
history: √hiṃs presents `ahinas` there, failing 8.2.75's own `ends_with('d')`,
and √kṛt presents `akfRad` (8.2.39 having voiced it) and fires. 7a verified
this byte-identical by two independent derivations.

Update `tinanta_rule_order_is_pinned`. `VIKALPA_RULES` is unchanged — 8.2.74
still precedes 8.2.75 either way.

### #2 — `position()` finds the first *applicable* candidate

8.3.24 and 8.4.58 locate the first `n` / `M` in the word and bail if it fails
the following-sound test, rather than continuing to the first candidate that
passes. The failure mode is silent under-application.

7b does not force this. Every one of the 108 new cells was traced, and in all
of them the first candidate is either the correct target or correctly declines
(`Banaj` + `ti` bails on the following `a`, as it should). It is fixed anyway,
as cheap hardening in a file this slice already edits: move the following-sound
test into the search. Because the condition moves rather than multiplies, it
keeps its witnesses and the mutation gate is unaffected.

**Acceptance: the full suite is byte-identical.** A moved cell is a defect.

### #4 — the five `SHAP.ends_with('a')` proxies

6.1.101, 6.1.97, 6.1.87, 6.1.66 and 6.4.105 in `adesha.rs` test
`SHAP.text.ends_with('a')` as a stand-in for "the vikaraṇa is thematic śap". A
vowel-final rudhādi root would leave `SHAP` as exactly `"na"` and all five
would treat śnam as śap.

Replace the proxy with a positive term tag, set when an a-final vikaraṇa is
introduced (3.1.68 śap, 3.1.69 śyan, 3.1.77 śa).

**Replacing rather than adding is the point.** An added
`&& !p.terms[ANGA].has(Tag::Rudhadi)` would have no witness — gaṇa 7 holds no
vowel-final root — and the mutation gate would flag all five as dead weight,
exactly as it flagged `Context::is_tip` in 7a. A replacement leaves a single
guard clause that adādi (empty `SHAP`), kryādi (`nA`/`nI`) and svādi (`nu`,
and `n` after 6.4.107) all still witness.

**Acceptance: paradigm and trace suites byte-identical.** The risk this bounds
is a rule that mutates `SHAP` mid-derivation, where a persistent tag and a
re-read of the text could diverge; any moved cell is that divergence surfacing
and must be investigated, not absorbed.

### #5 — the per-rule guard test convention

7a's deferred note says per-rule guard tests for tripādī rules "are not
achievable", since whole-word rules only exercise through a full derivation.
That is too strong: `tripadi.rs` already carries thirteen of them, including
`jhalam_jasho_ante_fires_only_on_a_pada_final_t` and
`va_avasane_fires_only_on_a_pada_final_jhal`.

What is genuinely unachievable is a guard test for a rule whose precondition
only an upstream rule chain can produce. Amend the convention in `AGENTS.md`
to say so: **write a per-rule guard test where the precondition can be built
directly on a hand-built `Prakriya`; otherwise cite the covering derivation or
trace test in the rule's own comment.** This keeps the discipline and ends the
per-rule re-litigation.

### #6 — the permanent mutation timeout

Already closed. `tripadi.rs`'s ṇatva backward scan mutates `j -= 1` into
`j /= 1`, a genuine infinite loop no assertion can catch; the cap is the
detector, and `AGENTS.md` records it as a permanent verdict. Nothing to do
beyond expecting exactly one `timeout.txt` entry of that shape.

## Testing

### Goldens

`paradigm.rs` gains 108 cells and 22 `ALTERNATES` rows.
`derivation_set_is_exactly_pinned` then enforces that each cell's derivation
set is exactly the union of the two — no over- or under-generation.

`PARADIGM` index 0 is the derivation with **no optional rule applied**, which
is not vidyut's ordering (a sorted set). Pinned explicitly:

| root | cell | index 0 | alternates | forking rule(s) |
| --- | --- | --- | --- | --- |
| √bhañj | laṅ prathama eka | `aBanag` | `aBanak` | 8.4.56 |
| √bhañj | laṅ madhyama eka | `aBanag` | `aBanak` | 8.4.56 |
| √bhañj | loṭ prathama eka | `Banaktu` | `BaNktAd`, `BaNktAt` | 7.1.35, 8.4.56 |
| √bhañj | loṭ madhyama eka | `BaNgDi` | `BaNktAd`, `BaNktAt` | 7.1.35, 8.4.56 |
| √bhañj | vidhiliṅ prathama eka | `BaYjyAd` | `BaYjyAt` | 8.4.56 |
| √piṣ | laṅ prathama eka | `apinaq` | `apinaw` | 8.4.56 |
| √piṣ | laṅ madhyama eka | `apinaq` | `apinaw` | 8.4.56 |
| √piṣ | loṭ prathama eka | `pinazwu` | `piMzwAd`, `piMzwAt` | 7.1.35, 8.4.56 |
| √piṣ | loṭ madhyama eka | `piRqQi` | `piRQi`, `piMzwAd`, `piMzwAt` | 8.4.65, 7.1.35, 8.4.56 |
| √piṣ | vidhiliṅ prathama eka | `piMzyAd` | `piMzyAt` | 8.4.56 |
| √indh | laṭ prathama eka | `indDe` | `inDe` | 8.4.65 |
| √indh | laṭ madhyama bahu | `indDve` | `inDve` | 8.4.65 |
| √indh | laṅ prathama eka | `EndDa` | `EnDa` | 8.4.65 |
| √indh | laṅ madhyama eka | `EndDAH` | `EnDAH` | 8.4.65 |
| √indh | laṅ madhyama bahu | `EndDvam` | `EnDvam` | 8.4.65 |
| √indh | loṭ prathama eka | `indDAm` | `inDAm` | 8.4.65 |
| √indh | loṭ madhyama bahu | `indDvam` | `inDvam` | 8.4.65 |

7 + 8 + 7 = 22, which cross-checks the `ALTERNATES` delta independently of the
form tables.

√indh takes **no** tātaṅ and no pausal branch — 7.1.35 is parasmaipada loṭ's
and 8.4.56 needs a pada-final jhal — so all seven of its alternates are
8.4.65's. Its deepest cell holds two forms, against √piṣ's four; nothing in 7b
approaches 7a's six.

### Trace pins

In `trace.rs`, one per new mechanism:

- **`apinaq`** — 8.2.23 above 8.2.41, the load-bearing order.
- **`Banakti`** — 8.2.30 then 8.4.55.
- **`indDe`** — 8.2.40 then 8.4.53.
- **`pinakzi`** — 8.2.41 then 8.3.59.
- **`piMzwaH`** — 8.3.24 fires and 8.4.58 declines: the conditional round
  trip's second witness.
- **`piRQi`** — the full 8.4.41 → 8.4.53 → 8.4.58 → 8.4.65 chain.

### Guard tests

Beside each new rule in `tripadi.rs`, per the amended convention, including the
declining cases: `BaYjanti` / `BaYjvaH` for 8.2.30's jhal-or-word-end
condition, and `piMzanti` for 8.4.41's strict adjacency.

### Cross-implementation audit

Extend the vidyut-prakriya harness to all 48 roots and compare derivation
**sets**, not just index 0 — that comparison is what caught 8.2.74's ordering
in 7a. The 7b probe already written (`rudhadi_7b_probe.rs`, including the
`~^r` control that settled √indh's pada) is its seed.

### Mutation gate

The suite grows 6.7%, so the floor a full **uncaught** mutant run has to clear
rises with it. Re-derive `--timeout` with headroom above that floor, not merely
above a caught-and-aborted run: under too small a cap a real survivor is
recorded as a timeout and a "0 missed" report is vacuous. Check `timeout.txt`
alongside `missed.txt`, and expect exactly one permanent timeout (#6).

### Documentation

Counts appear in more places than is comfortable, and 7a needed a post-merge
fix wave for exactly this. Every one of these moves: `AGENTS.md` (1620 → 1728,
191 → 213, 1811 → 1941, 45 → 48 roots, and the rudhādi paragraph's root list),
`README.md`, `docs/ARCHITECTURE.md`'s branch-count paragraph,
`data/ATTRIBUTION.md`, `data/dhatupatha.tsv` and
`crates/panini-data/src/lib.rs`.

`AGENTS.md` also carries three prose corrections this slice earns: 6.4.23's
`fj` typo and its "widens" claim, 8.2.73's standing re-verification warning
(now a verdict), and the guard-test convention (#5).

### A drafting constraint on the implementation plan

Code blocks in this repo's plans get transcribed verbatim into the
implementation, scaffolding and all. The plan for this slice should state rule
conditions, witnesses and acceptance criteria, and leave the Rust to be written
against them.

## Appendix — the full 7b paradigm

Derived from vidyut-prakriya at the checkout used for 7a's audit. Forms within
a cell are that tool's sorted set; index 0 is pinned in "Goldens" above, not by
this ordering.

### √bhañj (`Banj`, parasmaipada)

| cell | laT | laN | loT | viDiliN |
| --- | --- | --- | --- | --- |
| prathama eka | `Banakti` | `aBanag` / `aBanak` | `BaNktAd` / `BaNktAt` / `Banaktu` | `BaYjyAd` / `BaYjyAt` |
| prathama dvi | `BaNktaH` | `aBaNktAm` | `BaNktAm` | `BaYjyAtAm` |
| prathama bahu | `BaYjanti` | `aBaYjan` | `BaYjantu` | `BaYjyuH` |
| madhyama eka | `Banakzi` | `aBanag` / `aBanak` | `BaNgDi` / `BaNktAd` / `BaNktAt` | `BaYjyAH` |
| madhyama dvi | `BaNkTaH` | `aBaNktam` | `BaNktam` | `BaYjyAtam` |
| madhyama bahu | `BaNkTa` | `aBaNkta` | `BaNkta` | `BaYjyAta` |
| uttama eka | `Banajmi` | `aBanajam` | `BanajAni` | `BaYjyAm` |
| uttama dvi | `BaYjvaH` | `aBaYjva` | `BanajAva` | `BaYjyAva` |
| uttama bahu | `BaYjmaH` | `aBaYjma` | `BanajAma` | `BaYjyAma` |

### √piṣ (`piz`, parasmaipada)

| cell | laT | laN | loT | viDiliN |
| --- | --- | --- | --- | --- |
| prathama eka | `pinazwi` | `apinaq` / `apinaw` | `piMzwAd` / `piMzwAt` / `pinazwu` | `piMzyAd` / `piMzyAt` |
| prathama dvi | `piMzwaH` | `apiMzwAm` | `piMzwAm` | `piMzyAtAm` |
| prathama bahu | `piMzanti` | `apiMzan` | `piMzantu` | `piMzyuH` |
| madhyama eka | `pinakzi` | `apinaq` / `apinaw` | `piMzwAd` / `piMzwAt` / `piRQi` / `piRqQi` | `piMzyAH` |
| madhyama dvi | `piMzWaH` | `apiMzwam` | `piMzwam` | `piMzyAtam` |
| madhyama bahu | `piMzWa` | `apiMzwa` | `piMzwa` | `piMzyAta` |
| uttama eka | `pinazmi` | `apinazam` | `pinazARi` | `piMzyAm` |
| uttama dvi | `piMzvaH` | `apiMzva` | `pinazAva` | `piMzyAva` |
| uttama bahu | `piMzmaH` | `apiMzma` | `pinazAma` | `piMzyAma` |

### √indh (`inD`, ātmanepada)

| cell | laT | laN | loT | viDiliN |
| --- | --- | --- | --- | --- |
| prathama eka | `inDe` / `indDe` | `EnDa` / `EndDa` | `inDAm` / `indDAm` | `inDIta` |
| prathama dvi | `inDAte` | `EnDAtAm` | `inDAtAm` | `inDIyAtAm` |
| prathama bahu | `inDate` | `EnData` | `inDatAm` | `inDIran` |
| madhyama eka | `intse` | `EnDAH` / `EndDAH` | `intsva` | `inDITAH` |
| madhyama dvi | `inDATe` | `EnDATAm` | `inDATAm` | `inDIyATAm` |
| madhyama bahu | `inDve` / `indDve` | `EnDvam` / `EndDvam` | `inDvam` / `indDvam` | `inDIDvam` |
| uttama eka | `inDe` | `EnDi` | `inaDE` | `inDIya` |
| uttama dvi | `inDvahe` | `EnDvahi` | `inaDAvahE` | `inDIvahi` |
| uttama bahu | `inDmahe` | `EnDmahi` | `inaDAmahE` | `inDImahi` |
