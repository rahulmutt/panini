# Retiring the conventions — 7.1.35, 8.2.39/8.4.56, 3.4.110/111

Slice 5b taught the engine to fork at an optional rule and landed 6.4.107 as the
single witness. Its spec named the follow-up: 7.1.35 *tuhyos tātaṅ* and 8.4.56
*vāvasāne*, "the two standing one-form-per-cell conventions this machinery would
let us retire." This slice retires them.

The goldens have been filtered by convention since the first gaṇa. Every gaṇa
spec since has repeated some version of "reduced to one form per cell by the
repo's standing conventions: drop tātaṅ (7.1.35), drop the pausal `d` variant
where a `t` sibling exists (8.4.56)." After this slice there is no filter left.
That is the point of the slice, and it is also the claim it has to earn: the
engine's derivation set equals vidyut-prakriya's, in all 1512 cells, exactly.

## Scope

Unchanged: 42 roots, six gaṇas, four lakāras (laṭ, laṅ, loṭ, vidhiliṅ), nine
puruṣa × vacana cells. No new roots, no new gaṇa.

New: four sūtras, three of them optional, plus one widened arm on an existing
rule. 146 new forms across 98 cells. 48 of `PARADIGM`'s 1512 strings change.

Out of scope, deferred:

- **6.4.108 *nityaṁ karoteḥ***, still deferred as in slice 5b: √kṛ is not in
  the root set and wants 7.1.100 and the 6.4.10x kṛ-specials besides.
- **Ubhayapadī roots and 1.3.72 *svaritañitaḥ***, still deferred, as in every
  gaṇa spec so far. Note that vidyut's dhātupāṭha marks several of our roots
  ubhayapadī (`tu\da~^` among them); the audit below pins the pada explicitly
  on both sides, so this deferral does not weaken the comparison.
- **8.4.56 outside the tiṅanta**, and the rest of 8.2.39's jhal range. Both
  guards are written to the reachable slice, per the discipline that landed
  8.3.59 and 8.2.25.

## The audit that defines the slice

The slice's shape was not guessed; it was measured. A probe
(`examples/panini_full_audit.rs`, described but not reproduced under
"Verification" — see that section for what does and does not survive of it)
derives all 42 roots × 4 lakāras × 9 cells in vidyut-prakriya and prints the
**complete
set** of forms per cell rather than one form per cell. Comparing that against
`PARADIGM ∪ ALTERNATES`:

- **Forms the engine derives that vidyut does not: zero.** In 1512 cells there
  is not one over-generation. That is the direction that would indicate a bug,
  and it is clean before the slice starts.
- **Forms vidyut derives that the engine does not: 146**, falling into exactly
  four buckets and no others.

| bucket | count | cells |
| --- | --- | --- |
| 8.2.39 + 8.4.56 (`aBavat`, `Baved`) | 48 | laṅ and vidhiliṅ, prathama eka |
| 7.1.35 tātaṅ (`BavatAd`) | 48 | loṭ prathama eka and madhyama eka |
| 7.1.35 then 8.4.56 (`BavatAt`) | 48 | the same loṭ cells |
| 3.4.110/111 jus (`ayuH`, `avuH`) | 2 | √yā and √vā, laṅ prathama bahu |

Afterwards, 1406 cells hold one form, 58 hold two, and 48 hold three.

The fourth bucket was not in the slice as originally framed. It is two cells,
it is the same kind of thing as the other three — an optional rule the engine
lacks — and including it is what turns "exact except for these two cells" into
"exact". It is in scope.

## The rules

Ids and names verified against vidyut-prakriya's `data/sutrapatha.tsv`
(ashtadhyayi.com is a JS single-page app that cannot be fetched
programmatically; the TSV is what this repo checks ids and names against).

### 7.1.35 `tuhyostAtaNNASizyanyatarasyAm` (vikalpa)

*Tu* and *hi* are optionally replaced by *tātaṅ*. Both endings occur only in
loṭ parasmaipada, so the guard needs no lakāra test — reading the ending is
both necessary and sufficient, which is what makes this a self-guarding rule
rather than a branch.

```rust
Rule {
    id: "7.1.35",
    name: "tuhyostAtaNNASizyanyatarasyAm",
    kind: RuleKind::Vidhi,
    vikalpa: true,
    apply: |p| {
        let e = p.terms[ENDING_PRE_SHAP].text.as_str();
        if e != "tu" && e != "hi" {
            return false;
        }
        let before = p.snapshot();
        p.terms[ENDING_PRE_SHAP].text = "tAtaN".into();
        p.record("7.1.35", "tuhyostAtaNNASizyanyatarasyAm", before);
        // tātaṅ's ṅ is a real it-marker, not 1.2.4's atideśa. tu arrived pit
        // from 3.4.78; clear that before adding the ṅit, or the term claims
        // both — the same two-line shape 3.4.87 uses for hi.
        let before = p.snapshot();
        p.terms[ENDING_PRE_SHAP].text = "tAt".into();
        p.terms[ENDING_PRE_SHAP].remove(Tag::Pit);
        p.terms[ENDING_PRE_SHAP].add(Tag::Ngit);
        p.record("1.3.9", "tasya lopaH", before);
        true
    },
},
```

The two-step record — substitute the upadeśa, then strip the it by 1.3.9 —
follows 3.4.108 *jher jus*, which does the same for `jus` → `us`, and matches
vidyut's own trace (`7.1.35 | kliS + nA + tAta~N` … `1.3.9 | kliS + nA + tAt`).

The ṅit is what earns the forms. `Apnotu` guṇates śnu through 7.3.84's second,
vikaraṇa-relative application because *tu* is pit; `ApnutAt` does not, because
tātaṅ is ṅit and 1.1.5 blocks it. `Bavatu` → `BavatAt` **keeps** its guṇa,
because that one is śap-relative and śap is unaffected by the ending's
ṅitva — the two applications of 7.3.84 that svādi forced apart are visible
again here, from the other side.

tātaṅ also makes prathama eka and madhyama eka syncretic: `Bavatu` and `Bava`
both alternate to `BavatAt`. 48 `ALTERNATES` rows, 24 distinct strings.

**On *āśiṣi*.** The sūtra restricts tātaṅ to the benedictive sense. The engine
has no semantic input and never has — `check` answers "is this derivable within
the covered grammar", not "is this the right word here". We apply the rule
unconditionally and record that as a known, deliberate over-application of the
sūtra's own condition. Every form it admits is a real Sanskrit form; what the
engine cannot say is when to use it. This is the first rule in the repo with a
semantic condition, so the precedent is worth stating rather than assuming.

### 8.2.39 `JalAM jaSo'nte` (obligatory)

A pada-final jhal becomes jaś. This is new obligatory grammar, not a
convention being lifted: without it there is no voiced final for 8.4.56 to
devoice, which is why `aBavat` has been the engine's only form all along.

Narrow guard, by design, the same discipline that landed 8.3.59 and 8.2.25:
the only jhal reachable at pada-end in this suite is `t`, so the rule
implements `t` → `d` and nothing else. The other candidate is `s`, and 8.2.66
*sasajuṣo ruḥ* is its apavāda — already implemented inside the rule this repo
labels 8.3.15. Widen the moment a root lands whose pada-final sound is some
other jhal.

```rust
Rule {
    id: "8.2.39",
    name: "JalAM jaSo'nte",
    kind: RuleKind::Vidhi,
    vikalpa: false,
    apply: |p| {
        if !p.text().ends_with('t') {
            return false;
        }
        // The `t` is always the ending's; it is read positionally rather than
        // as terms[ENDING] because 6.4.105/6.4.106 can leave that term empty
        // (Bava, hinu), and a future luk could do so for a consonant-final
        // aṅga.
        let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
            return false;
        };
        let before = p.snapshot();
        let mut s: Vec<char> = p.terms[idx].text.chars().collect();
        s.pop();
        s.push('d');
        p.terms[idx].text = s.into_iter().collect();
        p.record("8.2.39", "JalAM jaSo'nte", before);
        true
    },
},
```

**It does not contend with 8.4.55 cartva.** The shape that would collide — an
aṅga-final jhal directly before a pada-final `t` — cannot arise, because 8.2.23
*saṁyogāntasya lopaḥ* sits earlier in the tripādī and drops the second
consonant first. √ad, the one root whose aṅga ends in a jhal, has `Adat` in
laṅ prathama eka: a vowel before the ending, so cartva never sees the case.
Plain sūtra order is therefore safe, and 8.2.39 goes where it belongs, after
8.2.25 and before 8.3.15.

### 8.4.56 `vA'vasAne` (vikalpa)

Continuing *khari ca*'s cartva: at the end of an utterance, a jhal optionally
becomes its car. After 8.2.39 the only reachable jhal-final is `d`, so the
rule is `d` → `t`, reusing `is_jhal` and `cartva_of` from `sound.rs`.

```rust
Rule {
    id: "8.4.56",
    name: "vA'vasAne",
    kind: RuleKind::Vidhi,
    vikalpa: true,
    apply: |p| {
        let Some(last) = p.text().chars().last() else {
            return false;
        };
        if !is_jhal(last) {
            return false;
        }
        let Some(sub) = cartva_of(last) else {
            return false;
        };
        if sub == last {
            return false;
        }
        let Some(idx) = p.terms.iter().rposition(|t| !t.text.is_empty()) else {
            return false;
        };
        let before = p.snapshot();
        let mut s: Vec<char> = p.terms[idx].text.chars().collect();
        s.pop();
        s.push(sub);
        p.terms[idx].text = s.into_iter().collect();
        p.record("8.4.56", "vA'vasAne", before);
        true
    },
},
```

It is the **last rule in the pipeline**. *Avasāna* is the end of the
utterance, so the rule should see the finished word; and last position
satisfies the AGENTS ordering constraint for optional rules trivially, since
there is no consumer below it at all.

### 3.4.110 `AtaH` / 3.4.111 `laNaH SAkawAyanasyEva` (vikalpa)

After an ā-final aṅga, jhi is replaced by jus (3.4.110), and in laṅ that
replacement is Śākaṭāyana's — i.e. optional (3.4.111). One rule implements the
pair, cited under 3.4.111, because 3.4.110 supplies only the condition and is
never separately observable here. vidyut records the single step as `3.4.111`
for exactly this reason.

```rust
Rule {
    id: "3.4.111",
    name: "laNaH SAkawAyanasyEva",
    kind: RuleKind::Vidhi,
    vikalpa: true,
    apply: |p| {
        // `J`, not `Ji`: 3.4.100 itaś ca has already dropped jhi's final `i`
        // by this point (laṅ is ṅit-like and parasmaipada). The term is still
        // jhi — 3.4.110/111 replace the whole of it — but its text is not.
        if !matches!(p.ctx.lakara, Lakara::Lan) || p.terms[ENDING].text != "J" {
            return false;
        }
        if sound_before_ending(p) != Some('A') {
            return false;
        }
        let before = p.snapshot();
        p.terms[ENDING].text = "jus".into();
        p.record("3.4.111", "laNaH SAkawAyanasyEva", before);
        let before = p.snapshot();
        p.terms[ENDING].text = "us".into();
        p.record("1.3.9", "tasya lopaH", before);
        true
    },
},
```

Placing it late is what forces the `"J"` guard, and that is the trade worth
naming. The alternative — putting it beside 3.4.108 in the tiṅ stage, where
the ending is still `Ji` and the rule mirrors vidyut's own step order — costs
the affix-relative reading below, because no vikaraṇa exists yet and
`sound_before_ending` would then be reading the dhātu under another name. The
guard is uglier this way and correct for a reason; the comment carries the
reason.

**The ā is read affix-relatively, not from the dhātu.** The rule is placed
after 2.4.72, so the vikaraṇa question is already settled, and it asks
`sound_before_ending` rather than "does the root end in ā". For adādi the two
give the same answer, but they diverge the moment a thematic ā-final root
lands: its śap would stand between the ā and the ending, and 3.4.110's
condition would no longer hold. Reading the dhātu directly would over-fire
there, silently, on a branch nobody is looking at. This is the same failure
mode as the aṅga/śap confusions 1.4.13 has already caused in this repo, and
the fix is the same — ask what actually precedes the ending.

## The 6.1.96 junction arm

`ayuH` needs one thing the engine does not have. vidyut derives it as
`a + yA + us` → **6.1.96 *usyapadāntāt*** → `a + y + us`, eliding the aṅga's ā
across the junction. The repo's 6.1.96 (`adesha.rs`, landed in slice 5b for the
adādi vidhiliṅ yāsuṭ junction) elides `a`/`A` only **inside** the ending:

```rust
let e = &p.terms[ENDING].text;
if !e.ends_with("us") { return false; }
let pre = e.chars().nth(n.wrapping_sub(3));
if !matches!(pre, Some('a') | Some('A')) { return false; }
```

For `yAus` the preceding char is inside the ending and the rule fires. For a
bare `us` there is no preceding char in the ending at all, and the rule
declines. The sūtra covers both readings; the implementation covers one.

So 6.1.96 gains a second arm: when the ending is exactly `us`, the `a`/`A` to
elide is the aṅga's final sound. This is the same shape as the widening
6.1.90's athematic arm took in the svādi slice, and it is written as an arm on
the existing rule rather than a new rule, because it is the same sūtra.

**The new arm cannot disturb the existing 1512.** A bare `us` ending arises in
this suite only from 3.4.111: 3.4.108 *jher jus* is vidhiliṅ-only, and by the
time 6.1.96 runs in vidhiliṅ the yāsuṭ of 3.4.103 has already made the ending
`yAus` (or `yus`, if the first arm has fired). Every other cell reaches 6.1.96
with an ending that is not `us`. That makes 3.4.111 the arm's only witness
today, which the rule comment should say plainly.

## Placement and ordering

`TINANTA_RULES` goes from 68 rules to 72. `tinanta_rule_order_is_pinned` gains
the four ids in position:

| rule | goes | stage file |
| --- | --- | --- |
| 7.1.35 | end of the tiṅ stage, after 3.4.102 | `tin.rs` |
| 3.4.111 | after 2.4.72, before 3.1.83 | `vikarana.rs` |
| 8.2.39 | after 8.2.25, before 8.3.15 | `tripadi.rs` |
| 8.4.56 | last rule in the pipeline | `tripadi.rs` |

7.1.35 lands before 3.1.68 and so addresses the ending as `ENDING_PRE_SHAP`;
3.4.111 lands after it and so addresses the ending as `ENDING`. Both follow
the AGENTS rule that stage membership is decided by position relative to
3.1.68, not by sūtra family — which is why a 3.4 rule sitting in `vikarana.rs`
is correct rather than misfiled.

### 7.1.35 must sit above every rule that reads `hi`

Three rules read the ending `hi` and would fire wrongly on the tātaṅ branch if
7.1.35 ran below them:

- **3.1.83 *halaḥ śnaḥ śānac ca*** reshapes śnā → śāna before `hi`. Below it,
  kryādi's tātaṅ branch would surface `kliSAnatAt`. Above it, 3.1.83 declines
  on that branch, 6.4.113 then gives `nI`, and the form is `kliSnItAt` —
  which is what vidyut derives.
- **6.4.105 *ato heḥ*** (the hi-luk after `a`, giving `Bava`).
- **6.4.106 *utaś ca*** (the hi-luk after non-conjunct `u`, giving `hinu`).

With 7.1.35 above all three, each correctly declines on the forked branch and
correctly fires on the base branch. It must also sit above 7.3.84, whose
second application reads the ending's ṅitva.

### The constraint's direction, sharpened

AGENTS.md currently states, from slice 5b:

> An optional rule must be ordered **after** every consumer of a predicate its
> own mutation invalidates.

Taken literally that would put 7.1.35 *below* 3.1.83, which is wrong. The
constraint needs its reason restored, because the two cases are opposites:

- **6.4.107 destroys the evidence for a predicate without changing the
  grammatical fact.** After it elides śnu's `u`, `shnu_asamyogapurva` can no
  longer recognise śnu and `sound_before_ending` reports `n` for `u` — but the
  vikaraṇa *is* still śnu. A consumer below gets a wrong answer, so consumers
  must sit above.
- **7.1.35 changes the fact itself.** The ending genuinely is no longer `hi`.
  A consumer below gets the *right* answer; a consumer above gets a stale one.

The operative question is not "does a consumer read what I wrote?" but "does
my mutation make the predicate lie?". Both directions are load-bearing and
nothing enforces either, so AGENTS.md gets this rewritten with both cases
named. 3.4.111 is a third instance of the 7.1.35 kind: it really does replace
jhi, so 7.1.3 *jho'ntaḥ* below it correctly declines, and 6.1.96 below it
correctly fires on the `us` it produced.

### Two vikalpa rules stack for the first time

loṭ prathama eka forks twice. `run_pipeline` clones each live branch, applies
to the clone, and inserts the clone immediately after the branch it forked:

```
                    [Bavatu]
7.1.35   ->         [Bavatu, BavatAt]
8.2.39   ->         [Bavatu, BavatAd]      (obligatory, both branches offered)
8.4.56   ->         [Bavatu, BavatAd, BavatAt]
```

Index 0 is still the derivation with no optional rule applied. 6.4.107 alone
never produced a three-branch derivation, so this is real new coverage of the
clone-and-insert path, and it gets pinned as such in `trace.rs`.

## What PARADIGM and ALTERNATES become

### PARADIGM: 48 strings change

`derivation_set_is_exactly_pinned` asserts `branches[0].text() == expected` —
"index 0 must be the declined derivation". 8.2.39 is obligatory, so for every
cell whose form ended in `t`, index 0 is now the jaś form. Those cells are
prathama eka of laṅ and vidhiliṅ for the 24 parasmaipada roots, and they are
the only consonant-final forms in the whole suite (every other form ends in a
vowel, `H`, `m` or `n`, none of which 8.2.39 touches).

The alternative — keeping the current strings and teaching the test that
`PARADIGM` means "the citation form" — was considered and rejected. You cannot
retire a convention and go on pinning its output as the canonical form; and
the index-0 assertion is a stronger invariant than byte-stability of a table
that was frozen for a different slice's reasons.

The 48 flips, in `PARADIGM` block order:

| root | lakāra | was | becomes |
| --- | --- | --- | --- |
| BU | laN | aBavat | aBavad |
| nI | laN | anayat | anayad |
| ji | laN | ajayat | ajayad |
| smf | laN | asmarat | asmarad |
| paW | laN | apaWat | apaWad |
| vad | laN | avadat | avadad |
| BU | viDiliN | Bavet | Baved |
| nI | viDiliN | nayet | nayed |
| ji | viDiliN | jayet | jayed |
| smf | viDiliN | smaret | smared |
| paW | viDiliN | paWet | paWed |
| vad | viDiliN | vadet | vaded |
| div | laN | adIvyat | adIvyad |
| naS | laN | anaSyat | anaSyad |
| kup | laN | akupyat | akupyad |
| tud | laN | atudat | atudad |
| liK | laN | aliKat | aliKad |
| viS | laN | aviSat | aviSad |
| div | viDiliN | dIvyet | dIvyed |
| naS | viDiliN | naSyet | naSyed |
| kup | viDiliN | kupyet | kupyed |
| tud | viDiliN | tudet | tuded |
| liK | viDiliN | liKet | liKed |
| viS | viDiliN | viSet | viSed |
| yA | laN | ayAt | ayAd |
| vA | laN | avAt | avAd |
| yA | viDiliN | yAyAt | yAyAd |
| vA | viDiliN | vAyAt | vAyAd |
| ad | laN | Adat | Adad |
| ad | viDiliN | adyAt | adyAd |
| kliS | laN | akliSnAt | akliSnAd |
| kliS | viDiliN | kliSnIyAt | kliSnIyAd |
| guD | laN | aguDnAt | aguDnAd |
| guD | viDiliN | guDnIyAt | guDnIyAd |
| aS | laN | ASnAt | ASnAd |
| aS | viDiliN | aSnIyAt | aSnIyAd |
| muz | laN | amuzRAt | amuzRAd |
| muz | viDiliN | muzRIyAt | muzRIyAd |
| vrI | laN | avrIRAt | avrIRAd |
| vrI | viDiliN | vrIRIyAt | vrIRIyAd |
| Ap | laN | Apnot | Apnod |
| Ap | viDiliN | ApnuyAt | ApnuyAd |
| Sak | laN | aSaknot | aSaknod |
| Sak | viDiliN | SaknuyAt | SaknuyAd |
| hi | laN | ahinot | ahinod |
| hi | viDiliN | hinuyAt | hinuyAd |
| ri | laN | ariRot | ariRod |
| ri | viDiliN | riRuyAt | riRuyAd |

The remaining 1464 strings are byte-identical.

### ALTERNATES: 8 rows become 154, and gains a column

The row shape becomes `(root_id, lakara_label, cell, form, vikalpa_key)`,
where the key names the set of optional rules applied on that branch:

| key | rows |
| --- | --- |
| `8.4.56` | 48 |
| `7.1.35` | 48 |
| `7.1.35+8.4.56` | 48 |
| `3.4.111` | 2 |
| `6.4.107` | 8 |

The eight existing 6.4.107 rows keep their forms and cells exactly; they only
gain the key.

The column pays for itself with a new test,
`every_alternate_names_the_vikalpa_rules_that_produced_it`: find the branch
whose text matches the row, intersect its recorded trace with the pinned
vikalpa set, and assert equality with the declared key. Without it,
`ALTERNATES` is 154 strings that a wrong rule could satisfy by accident —
`BavatAt` is right whether or not 8.4.56 is what produced it. With it, each
golden is tied to the grammar that derives it. It also subsumes the arithmetic
check that the per-key counts are 48/48/48/2/8.

The full table is generated by the audit probe and reproduced in the
implementation plan, in `PARADIGM` block order.

## Verification

### The audit probe

The probe is a design-time reference, not a CI dependency: goldens stay
hand-pinned, and vidyut is never linked into this workspace. It lives in a
vidyut checkout as `vidyut-prakriya/examples/panini_full_audit.rs`, keyed by
`(Gana, dhātupāṭha number, DhatuPada)` for all 42 roots — the pada is pinned
explicitly on both sides so that vidyut's ubhayapadī markings cannot smear the
comparison. It prints `id \t lakara \t cell \t form1/form2/…`, and the
comparison against `PARADIGM ∪ ALTERNATES` is a set equality per cell.

The slice is done when that comparison is empty in both directions for all
1512 cells. It is run once before implementation (its result is the table
above) and once after.

**What actually happened:** only the before run did. No vidyut-prakriya
checkout exists in this workspace, and the probe's own source was not
preserved anywhere this repo can reach — not in this file (the table above is
its output, not its code), and not in this repo's git history — so the after
run could not be reproduced here. What replaced it is a local shape test,
`derivation_set_shape_matches_the_audited_numbers` in
`crates/panini/tests/paradigm.rs`: it pins the cell-count and `ALTERNATES`-key
shape this design-time audit recorded — 1406/58/48 one/two/three-form cells,
154 `ALTERNATES` rows keyed 48/48/48/2/8 — derived from `PARADIGM ∪
ALTERNATES` rather than hand-copied. That test composes with
`derivation_set_is_exactly_pinned`, which independently drives the engine on
every cell and asserts its actual derivation set equals `PARADIGM`'s pinned
form plus exactly its `ALTERNATES` rows. Together the two guarantee the
engine derives exactly the set this design-time audit recorded, with every
alternate produced by the optional rule its key names. What they cannot show,
because vidyut is unreachable in this workspace, is whether the audit table
itself is right — that is, whether it truly matches what vidyut-prakriya
derives. So the claim this slice can actually stand behind is "the engine's
derivation set equals the audited set," not "equals vidyut-prakriya's set."

### `tin.rs`, `vikarana.rs`, `tripadi.rs` — per-rule guards, beside the rule

Each new rule gets its guard tests in the stage file it lives in, per AGENTS:

- 7.1.35 fires on `tu` and on `hi`, declines on every other ending, sets Ngit
  and clears Pit.
- 3.4.111 fires only in laṅ, only on the post-3.4.100 `J`, only after an ā;
  and it is the `sound_before_ending` reading that is asserted, not a dhātu
  test.
- 8.2.39 fires only pada-finally; 8.4.56 fires only on a jhal.
- 6.1.96's new arm fires on a bare `us` after an `a`/`A`-final aṅga and
  declines when the ending has its own preceding vowel (the yāsuṭ case, which
  the first arm owns).

### `derivation_tests.rs`

`tinanta_rule_order_is_pinned` grows from 68 to 72 ids, with each new id in
position. `exactly_the_pinned_vikalpa_rules_are_optional` grows from
`{6.4.107}` to `{3.4.111, 6.4.107, 7.1.35, 8.4.56}`.

### `paradigm.rs` — the golden table

`PARADIGM` takes the 48 flips; `ALTERNATES` takes its fifth column and grows
to 154 rows. `derivation_set_is_exactly_pinned` and
`every_alternate_names_a_real_cell` need no change beyond the wider row shape.
`every_alternate_validates_and_matches` likewise. The new test is
`every_alternate_names_the_vikalpa_rules_that_produced_it`.

`known_nonforms_are_invalid` gains a pin per new guard. Following the practice
this file already documents at length — a pin must be **what a specific
mutation actually emits**, verified by breaking the guard, not guessed — the
ones that can be named up front are:

- `ApnotAt` — 7.1.35 failing to set Ngit or to clear Pit, so 7.3.84's second
  application guṇates śnu.
- `kliSAnatAt` — 7.1.35 ordered after 3.1.83 instead of before it.
- `aBavuH` — 3.4.111 firing on a root that does not end in ā.
- `yuH` — 3.4.111 not gated to laṅ.

Pins for 8.2.39's pada-final guard and for 8.4.56 over-firing are constructed
the same way during implementation, by mutating the guard and recording what
comes out. Because 8.4.56 is optional, an over-firing guard **adds** a wrong
form rather than replacing a right one — invisible to any test that only asks
whether the right form still derives, which is exactly why
`derivation_set_is_exactly_pinned` and the per-key counts matter here.

No currently pinned non-form becomes derivable: none of them ends in a voiced
stop or in `tAt`, and the augment/lakāra confusions the list is mostly made of
are untouched.

### `trace.rs` — pinning the forks as forks

Six existing pins name `t`-final forms — `aBavat`, `Adat`, `akupyat`, `ASnAt`,
`Bavet`, `kupyet`. Each now names the **forked** branch, and each expected
trace gains `8.2.39` and then `8.4.56` at the end.

Four new pins:

- `kliSnItAt` — the 7.1.35-before-3.1.83 ordering, asserting `3.1.83` absent
  and `6.4.113` present. This is the pin that fails if 7.1.35 is ever moved
  down.
- `ApnutAt` — the ṅit block, asserting the second `7.3.84` absent.
- The `Bavatu` / `BavatAd` / `BavatAt` triple, pinned as a double fork the way
  slice 5b pinned 6.4.107's single one.
- `ayuH` — `3.4.111` then `6.1.96` then `8.3.15`.

### `roundtrip.rs`

Unchanged in shape; it walks whatever `PARADIGM` holds.

### Mutation testing

`mise run mutants` with the explicit generous timeout. The new guards are
small and each arm is reachable, so the narrow-guard discipline should keep
the survivor count at zero; any survivor means an arm has no witness and the
guard should shrink rather than the test grow.

## Documentation

- **AGENTS.md**: the vikalpa set becomes four rules, not one; the ordering
  constraint is rewritten with both directions named (the section above); the
  golden-suite paragraph records that prathama eka of laṅ and vidhiliṅ is now
  the jaś form for parasmaipada roots, and that the suite is no longer filtered
  by any convention.
- **README.md**: the scope paragraph's example of a forked cell is still
  `hinvaH`/`hinuvaH`, but the claim that "a cell may have more than one valid
  form" now covers 98 cells, and it is worth saying that a cell may have
  three.
- **docs/ARCHITECTURE.md**: three things. The stage table's rule ranges gain
  the four new ids. The claim that "6.4.107 is the only optional rule" is
  retired. And the branch-count sentence — "branch count is 2^k in the number
  of optional rules that fire; k is 1" — is **wrong** once rules stack, not
  merely stale: loṭ prathama eka has k = 2 and three branches, not four,
  because 8.4.56 declines on the base branch (`Bavatu` is vowel-final) and
  forks only the tātaṅ one. The branch count is the number of distinct
  subsets of optional rules that actually apply, which is bounded by 2^k and
  reaches it only when every optional rule fires on every branch.

## Risks

- **The 48 flipped goldens are the only irreversible-looking change.** Each is
  mechanical (`t` → `d` in the final position) and each is backed by the
  audit, which shows vidyut deriving both members of every pair.
- **7.1.35's placement is invisible and permanent**, exactly like 6.4.107's.
  It is above three rules that read `hi`, and nothing in the code enforces
  that. The `kliSnItAt` trace pin is the guard; it must assert the *absence*
  of 3.1.83, not merely the presence of the right surface form, because the
  wrong order still produces a plausible-looking word.
- **The *āśiṣi* over-application** is a deliberate divergence from the sūtra's
  stated condition, recorded here so it is not later mistaken for a bug.
- **Forking triples the branch count for 48 cells** and doubles it for 50
  more. Negligible against a suite that already runs the full pipeline 1512
  times.
