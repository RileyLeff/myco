- no riley project specific stuff
  - e.g. plant ecophys-specific stuff lives in a dedicated lib (called a spore, tentatively) external to the myco core project, i can implement that stuff separately, it doesn't belong here.

- no versioning in the spec
  - we're pre-alpha, if we say "2.1" or whatever i find that agents start to worry about what stuff belongs in 2.1 vs 2.2 vs 2.3, which is the wrong mentality. right now i want to get the spec correct and thorough up front so we don't have any surprise refactors later and minimize carnage moving forward.

- no legacy stuff or "we reorganized this" in the spec. i don't want the spec to reflect the entire history of thought and organization on the project, if something is stale or subseded, we don't want it in there. if something moved to a different section, it doesn't matter that it used to be in a previous section.

- if we encounter something that we DON'T want to be in the spec, list it in anti_spec.md so we don't have to revisit.

- mitigate duplication to the extent possible for token efficiency

- check chunk reports against spec new very thoroughly

- check each section of spec new against spec, v2.1, open questions, chunk reports. open to chunking this where a subagent is responsible for a few sections or something.

- consider token efficiency of spec

- do full within-spec review: any redundancy or conflict? anything marked as deferred to future that we really want to be in the open questions that we need to resolve before implementation?

- do full review of spec_dev_notes, spec.md, v2.1 in progress. anything in there that is currently homeless that hasn't made it into either spec or anti spec.md?

## Pre-ship mock rewrite obligations

Both `mocks/mock_sperry.myco` and `mocks/mock_potkay.myco` predate
the CC1 literal-numerics lock and other v2.1 surface decisions. Before
any release, they need a rewrite pass so the canonical examples
reflect the locked spec:

- **Strip literal values from universals.** Any `universal X =
  <number>` form in the mocks violates CC1 and §3.1. Rewrite as
  `universal X: Scalar<U>` and move the numeric value to the Python
  workflow via `assume_constant`.
- **Sperry specifically:** audit every `universal` declaration and
  move values to a paired `workflow_sperry.py` file alongside.
- **Potkay specifically:** same universal pass plus migration of any
  old `slot` / `[t+1]` timestep syntax to the locked `step(y) =
  expr` form (§10, §16).
- **Cross-check against anti_spec.md.** Any construct the mocks use
  that now lives in anti_spec.md (e.g. `slot`, `#[...]` annotations,
  `dyn` escape, `data contract` as a distinct kind) must be rewritten
  to the replacement surface.
- **Re-run through `mycoc check` once the compiler exists.** The
  mocks should compile without diagnostics against the locked spec;
  any remaining drift surfaces as compile errors at that point.

Mocks are canonical examples, so they double as a smoke test of the
locked surface. Keeping them aligned is a release gate.
