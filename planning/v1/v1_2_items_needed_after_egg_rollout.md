# Items Needed After The `egg` Rollout

Status: the equality core and planner now run through `egg`, but `v1` spec parity is still not complete

Audience: project authors

## Purpose

This document records what remains between the current implementation state and an honest, fully implemented `v1` after the `egg` rollout.

This is intentionally a sibling to [v1_1_items_needed_for_spec_parity.md](/Users/rileyleff/Documents/dev/myco/planning/v1/v1_1_items_needed_for_spec_parity.md), not a replacement for it.

The `v1.1` note captured the project before:

- the end-to-end training proof
- dimensions and minimal units
- structured penalties
- introspection
- compile-mode validation
- `egg` integration

This `v1.2` note captures the remaining gaps after those slices landed.

## Current Assessment

### What Is Now Proven

- The world-description / compile-binding split is real and stable.
- The Python package is real enough to support tests, examples, and training.
- The same `.myco` model supports synthetic generation and learned recovery.
- The equality core is now genuinely `egg`-backed.
- Directional planning now consumes `egg`-derived registrations and extracted expressions rather than hand-written inversion logic.
- The emitted JAX artifact supports end-to-end optimization and held-out behavioral recovery.
- Same-step algebraic loops are detected and rejected.
- Dimensions, simple units, hard learned-slot bounds, and one soft penalty family exist end to end.

### What Is Still Missing For Honest `v1`

The biggest remaining gaps are no longer in the symbolic core. They are in semantic parity between the compile spec, the emitted runtime behavior, and the explanation surface.

Three of the original `v1.2` issues are now closed:

- slot binding kinds now produce distinct emitted runtime behavior
- learned initial state is now implemented in emitted artifacts
- consistency handling is now an explicit compile-time policy shared by both backends

The main remaining issues are now:

- provenance resolution is still mostly block-level in some places, even though expression text and source spans are now surfaced
- richer slot metadata is now available on compiled artifacts, but the remaining question is whether any further interface-shape detail is actually required for honest `v1`

So the project is now best described as:

- architecturally strong
- end-to-end real
- not yet fully semantically complete relative to the `v1` spec

## Recommended Next Sequence

At this point, I do **not** recommend broadening the frontend language, changing the parser, or expanding the rewrite library.

The next order should be:

1. Decide whether the current artifact-level slot metadata is already sufficient for honest `v1`, or whether further interface detail is still needed.
2. Decide whether block-level provenance is already sufficient for honest `v1`, or whether finer-grained equation identity still needs to be introduced.
3. Re-run external review once those remaining semantics are narrow and explicit.

This order matters because the remaining gaps are mostly semantic mismatches between what the API promises and what emitted code actually does.

## Work Remaining

## 1. Real Provider Semantics For Slots (Closed)

### Current State

The compile spec distinguishes among:

- `SlotBindingKind::DataSeries`
- `SlotBindingKind::Constant`
- `SlotBindingKind::Learned`

But the emitted runtime treats every slot as a callable provider surface.

That means slot kind currently affects:

- validation
- planning cost
- emitted metadata

It does **not** yet affect emitted runtime semantics in a strong way.

### Why This Matters

The spec is explicit that value sources are a load-bearing part of Myco’s architecture, not decorative metadata.

If a slot is bound as:

- a data series
- a constant
- a learned function

then emitted artifacts should reflect those choices directly rather than collapsing them all into “user passes a callable.”

### What `v1` Needs

For `v1`, the semantics can stay narrow:

- learned slot: emitted as a callable provider interface
- constant slot: emitted as a compile-time or runtime constant value path
- data-series slot: emitted as a per-step supplied series path

This does **not** require a new frontend language feature. It is a binding/emission issue.

### Acceptance Criteria

- slot binding kind changes emitted runtime behavior, not just metadata
- a constant slot can be used without supplying a callable
- a data-series slot can be used without supplying a callable
- a learned slot still uses the current callable interface

## 2. Learned Initial State Must Be Real Or Removed (Closed)

### Current State

The compile API and mode validation explicitly allow:

- `InitialStateSource::Learned`

But emitted artifacts still require the caller to provide `initial_state` explicitly, and generated `init_params()` is still empty.

So the compiler currently accepts a workflow it does not actually implement.

### Why This Matters

This is the clearest remaining API/runtime mismatch in the current codebase.

It is acceptable to support learned initial state in `v1`.
It is also acceptable to defer learned initial state beyond `v1`.

It is **not** acceptable to keep advertising it while the emitted artifact ignores it.

### Decision

Pick one of these and do it decisively:

1. Implement learned initial state.
2. Remove learned initial state from `v1` compile validation and public API.

### Recommendation

For `v1`, I would implement the narrowest real version:

- state quantities marked as learned initial state become named trainable parameters in emitted JAX artifacts
- `init_params()` returns those parameters
- rollout initialization can be built from those learned parameters plus caller-supplied fixed initial states

### Acceptance Criteria

- `InitialStateSource::Learned` is either fully supported in the emitter or fully removed from `v1`
- there is no longer any accepted-but-nonfunctional compile path

## 3. Consistency Must Become An Explicit Compile-Time Policy (Closed)

### Current State

The current implementation already surfaces overdetermination and emits consistency loss.

But there are still three mismatches with the spec:

- consistency is effectively always on
- the Python and JAX backends do not treat alternative paths identically
- mixed slot/mechanistic alternatives are not yet handled cleanly across both backends

### Why This Matters

The spec does **not** say that every non-canonical path always becomes a penalty.
It says that alternative paths may become:

- diagnostics
- consistency checks
- both

depending on compile-time intent.

### What `v1` Needs

Keep the policy narrow and explicit:

- `consistency = off`
- `consistency = equation_only`
- `consistency = all_alternatives`

or an equivalent simple switch in the compile layer.

### Acceptance Criteria

- emitted consistency behavior is a compile-time choice rather than a hard-coded default
- Python and JAX backends agree on what counts as a consistency contributor
- mixed slot/mechanistic overdetermination is handled intentionally rather than by omission

## 4. Compile Modes Need To Affect Emitted Artifacts (Closed)

### Current State

`Simulate`, `Fit`, and `Train` now have mode-aware validation, but emitted artifacts are still mostly the same across all three modes apart from metadata.

### Why This Matters

The spec allows compile modes to remain lightweight, but it still expects them to mean something operationally.

Right now they mostly mean:

- validation gatekeeping

That is useful, but it is weaker than the intended contract.

### What `v1` Needs

Keep the distinction small but visible:

## 5. Provenance And Introspection Need To Explain Real Extracted Paths (Mostly Closed)

### Current State

The explanation surface now includes:

- rendered chosen expressions
- rendered alternative expressions
- provenance labels
- source spans carried into Python and CLI explanations

That is enough to explain the TinyTree recovery path and point back to the originating declarations.

### What Still Needs Cleanup

One smaller gap remains:

- provenance lookup is still block-level in places where a finer-grained equation identity would be cleaner

### Acceptance Criteria

- chosen and alternative paths show rendered expressions and stable source locations
- reported costs match extracted path costs consistently across the explanation surface
- provenance resolution is stable enough that a user can reliably map explanations back to source

## 6. Slot Metadata On Compiled Artifacts (Closed Enough For Current `v1`)

### Current State

Compiled artifacts now expose typed metadata directly through the Python package, including:

- compile mode
- consistency policy
- whether loss helpers are emitted
- learned initial-state quantities
- learned slots
- slot interfaces with input/output names and arities

This closes the earlier gap where slot metadata only existed as generated-module globals and was not surfaced cleanly through the package boundary.

### Remaining Question

The remaining issue is no longer whether the information exists.
It is whether further interface detail beyond names, kinds, and arities is actually required for honest `v1`.

### Acceptance Criteria

- callers can inspect slot interfaces without executing generated module source
- artifact metadata is consistent between Rust core and Python package surfaces
- the current metadata surface is either accepted as sufficient for `v1`, or explicitly narrowed in the spec

- `simulate`: no observation payload required, loss helpers optional
- `fit`: loss helpers emitted, learned-slot interfaces optional
- `train`: loss helpers emitted, learned interfaces / trainable metadata surfaced

This can be implemented without branching the whole compiler.

### Acceptance Criteria

- mode selection changes emitted artifact surface in observable ways
- users can tell from the artifact what workflow it is meant to support
- mode behavior remains simple and documented

## 5. Provenance And Introspection Need One More Step

### Current State

The current explanation surface is useful. It can already answer:

- what was chosen
- what alternatives existed
- what remains unresolved

But it still falls short of the spec’s intended debugging surface because it does not deeply expose:

- the extracted expression that actually won
- stable provenance links or spans for the winning path
- why one extracted path beat another beyond a scalar cost number

### Why This Matters

Now that the planner is truly `egg`-driven, explainability is more important, not less.

If extraction becomes more sophisticated while explanations stay shallow, the compiler becomes harder to trust.

### What `v1` Needs

For `v1`, this can stay text-first:

- surface the chosen extracted expression text
- surface alternative extracted expression text when available
- preserve original relation / block labels in explanations
- expose enough provenance to map back to model declarations

### Acceptance Criteria

- a user can inspect the expression actually extracted for a chosen path
- a user can see why an alternative was not selected
- source-aware context survives through lowering and extraction in a stable way

## 6. Slot Interface Metadata Needs To Be Stronger

### Current State

The compiler tracks slot arity and names, but the emitted metadata is still relatively thin, especially on the JAX side.

### Why This Matters

The spec explicitly wants the compiler to surface slot interface contracts so that the Python caller and emitted backend do not rely on manual counting or implicit assumptions.

This matters more once:

- learned initial state exists
- multiple output slots matter
- non-scalar quantities or bundles show up later

### What `v1` Needs

For now, keep it narrow:

- input names
- output names
- input arity
- output arity
- compile-time slot kind

If practical, also include the aggregate scalar dimensionality implied by the current `v1` representation.

### Acceptance Criteria

- both Python and JAX emitted artifacts expose explicit slot interface metadata
- callers do not have to infer interface shape from generated code text

## 7. Optional But Valuable: Tighten Planner Cost Reporting

### Current State

Path selection now uses extracted expressions from `egg`, but some stored plan and blocked-candidate fields still report legacy or incomplete costs.

This is not a correctness bug for execution.
It is a debugging and introspection issue.

### Why It Matters

Once path choice is driven by extracted buildable forms rather than procedural inversion rules, cost reporting needs to match what actually happened or the explanation surface becomes misleading.

### Acceptance Criteria

- chosen path cost reflects the actual extracted candidate cost
- blocked candidate cost reflects the same cost model users see in explanations
- cost reporting is no longer a legacy vestige of the pre-egg planner

## What Does **Not** Need Immediate Attention

These are still reasonable to leave alone for now:

- parser rewrite to `nom`/`pest`
- broader rewrite library
- more advanced branch reasoning
- richer observation operators
- visualization tooling
- multiple backend targets

Those may matter later, but they are not the main blockers to a truthful `v1`.

## Recommended Immediate Milestone

The next milestone should be:

1. implement real slot binding runtime semantics
2. resolve learned initial state honestly
3. add explicit consistency policy in the compile layer
4. make compile modes affect emitted artifacts visibly
5. deepen explanations enough to surface extracted expressions and stable provenance

If those land cleanly, the resulting project is close to an honest `v1` rather than just a strong prototype.

## Definition Of Done For Honest `v1`

The project can claim a true `v1` when all of the following are true:

- the symbolic core and planner are genuinely `egg`-driven for the arithmetic MVP
- the same `.myco` model supports synthetic generation and learned recovery
- provider kinds mean something real in emitted artifacts
- there is no accepted compile path that the backend cannot actually execute
- compile modes affect emitted artifact behavior, not just validation
- consistency behavior is explicit and intentional
- explanations are strong enough that the first real plant model remains debuggable

That is the current target.
