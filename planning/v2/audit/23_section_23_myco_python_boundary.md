# Audit Report — §23 The `.myco` ↔ Python Boundary

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §23.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md` "Principle":**
  > "Python does not know Myco types. The Python library is a generic data-provenance and orchestration layer."
  > "spore authors ship one artifact (`.myco` sources + `myco.toml`). There is no Python mirror package."

  Absorbed into §23 preamble "Dumb-data Python layer": "Python never sees `.myco` types as Python classes. The compiled artifact exposes a node catalog ... Spore authors ship one artifact (`.myco` sources plus `myco.toml`); there is no Python mirror package."

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md` "What Python sees":**
  > "the resulting artifact exposes a node catalog: a structured manifest of every node the workflow can touch."
  > Path, declared type shape, binding role, units.

  Absorbed into §23 preamble verbatim: "the compiled artifact exposes a node catalog (path, declared type shape, binding role, units); Python verbs (`bind`, `observe`, `run`) operate over those path names."

- **`planning/v2/v2.1_in_progress.md:918-939` "Multi-binding compilation":**
  > "Same `.myco` model, N binding sets, one callable shared across them. Essential for sparse multi-study training — every study's concrete model satisfies the same contract, so `bind_controller` with the same callable threads the same weights through all of them."

  Absorbed into §23.2 ("Callable weight reuse") and §23.3.

- **`planning/v2/v2.1_in_progress.md:880-909` "`bind_controller` specifics":**
  > "The term *controller* is workflow vocabulary only — nothing in the `.myco` identifies a variable as controller-eligible, and there is no `Controller` kind or contract."
  > "Transparent callables. `bind_controller` accepts any callable. A learned neural net, a Gaussian process, an analytic `.myco` module, or a handwritten Python function all bind the same way."

  Absorbed into §23.3 preamble and the §24.1/§24.2 cross-reference.

- **`planning/v2/open_questions_deprecated_use_spec_new.md:343-358` (RESOLVED — data contracts):**
  > "Data contracts (output-only contracts) are the interface a callable consumes. Any concrete model that satisfies the contract ... can bind to the same callable via `bind_controller(path, fn, Tree)`."

  Absorbed into §23.3. The "data contract" kind name is retired in anti_spec.md; §23.3 restates the mechanism as plain contracts.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:1083-1085` (CC1 + sampling locality):**
  > "Sampling parameters are workflow-side. Non-determinism, seed management, and sample budget are workflow concerns, not world claims — consistent with CC1 (no workflow values in `.myco`)."

  Absorbed via §23 preamble ("All numeric values ... cross this boundary") and §24.5's `run.config.seed`.

- **`planning/v2/open_questions_deprecated_use_spec_new.md:816-822` (CC1 RESOLVED):**
  > "CC1 (spec §4, anti_spec 'Dropped features') bans literal numerics in value position. Physical constants and mathematical constants ... enter at compile time via the workflow binding verbs."

  Absorbed into §23 preamble: "All numeric values (physical constants, fit parameters, data series, initial conditions, topology, observations) cross this boundary." This is the operational statement of CC1 at the boundary.

---

## Superseded

Corpus content replaced by a newer decision in spec_new.md §23. Should move to anti_spec.md if not already there.

- **`planning/v2/spec.md` §4.11 `param` declaration:**
  > "`param` declares an empirical parameter whose value comes from the workflow."
  > "Params have no default value — they must be bound via `assume_constant`, `learn_constant`, or another workflow verb."

  Superseded by CC1 and the §23 preamble: `.myco` declares structure only; there is no `param` keyword. Already retired in anti_spec.md:
  > "`param` | workflow-bound typed fields | CC1: all values enter from workflow"
  No further action needed.

- **`planning/v2/spec.md` §7 (whole section) + §15.4 / §15.5 "slot binding" and "slot metadata":**
  > "`experiment.bind_slot('stomatal_control', 'sperry/controllers/gain_risk')`"
  > "`experiment.bind_slot_metadata('stomatal_control', { 'taxon_id': 4, ... })`"

  Superseded by `bind_controller(path, fn, input_contract, output_contract)` in §24.1 and the slot-retirement captured in anti_spec.md:
  > "`slot` / `learn_slot` / `bind_slot` / `bind_slot_metadata` | `bind_controller(path, fn, input_contract)` | controller is workflow-only, no `.myco` kind"
  No further action needed.

- **`planning/v2/spec.md:1208-1212` "Python API" for `model.universals()` / `model.params()`:**
  > "`model.universals()    # returns dict of universal names, types, and default values`"
  > "`model.params()        # returns dict of param names and types that need binding`"

  Superseded by the node-catalog framing in §23 preamble (chunk 09). The Python surface is generic; there is no typed accessor per `.myco` concept. `model.params()` assumes the retired `param` kind.

  `Recommend:` Not yet in anti_spec.md. Low urgency — the `param` retirement already implies it, but a line in anti_spec.md noting that the `model.params()` / `model.universals()` typed accessors are retired in favor of the generic node-catalog surface would tidy the trail for legacy-doc readers.

- **`planning/v2/spec.md:3419` four-verb vocabulary `assume` / `observe` / `learn` / `bind`:**
  > "`assume`: supply a value. `observe`: provide evidence. `learn`: declare something as trainable. `bind`: provide a specific implementation for a slot."

  Superseded by the eight-verb taxonomy (§24). The `open_questions_deprecated_use_spec_new.md:787-810` "Workflow Verb Taxonomy" entry already flags this as a live question, correctly noting the four-verb grouping may or may not survive. §24 locks the eight-verb form; the four-verb form is informational only.

---

## Homeless

Corpus content relevant to the `.myco` ↔ Python boundary that is not accounted for in spec_new.md §23 and not already committed to anti_spec.md. Highest-value bucket.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md` "Sampling / distribution primitives" split:**
  > "Python value providers ... are distinct from `.myco` distributions. Locked."
  > "A user sampling a Python `myco.lognormal(...)` to initialize a field is just writing 'please draw one random number per instance.' It is not declaring a posterior or a prior; that's `.myco`-side."

  This is a locked principle from chunk 09 ("Python value providers are distinct from `.myco` distributions. Locked.") with direct boundary consequences: the Python-side `myco.uniform` / `myco.lognormal` / `myco.normal` / `myco.from_csv` helpers are value providers, not model claims. §23 does not mention this split; §13 (probabilistic programming) owns the `.myco` side but does not cross-reference the Python-side helpers.

  `Recommend:` Add a subsection or preamble bullet under §23 distinguishing Python value providers (RNG utilities used at bind time) from `.myco` `Distribution<U>` contracts. The distinction is load-bearing: a user binding `assume_constant("kmax", values=myco.lognormal(...).sample())` is not declaring a prior.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md` "Mode B interaction":**
  > "The dumb-data principle means per-instance contract-type selection cannot be driven from Python alone. If Mode B is desired ... the choice must be made `.myco`-side."

  This is a locked boundary constraint from chunk 09: Python cannot select contract implementations per-instance. §23 does not state this constraint; it is implicit in the "Python never sees `.myco` types" framing but not spelled out.

  `Recommend:` Add a bullet under §23's dumb-data paragraph noting that per-instance contract-implementation selection lives `.myco`-side (via tagged unions / discriminants), not Python-side. This is a stable boundary implication, not open work.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md` "Run / control":**
  > "RNG control, wall-clock scheduling, checkpointing, restart — all Python-side. None of these cross into `.myco`; they are workflow concerns."

  Explicitly locked in chunk 09. §24.5 covers `run.config.seed` and some overlap, but §23 does not state the "checkpoint / restart / wall-clock all live Python-side" boundary rule. §24.5 lists run-config fields rather than the principle.

  `Recommend:` Add a bullet or line under §23's preamble listing the workflow-only capabilities (RNG, checkpoint / restart, wall-clock scheduling) that never cross into `.myco`. Companion to the "all numeric values cross this boundary" statement.

- **`planning/v2/v2.1_in_progress.md:49-55` glossary "Controller" and "input_contract":**
  > "Controller — workflow vocabulary for the callable attached to a variable by `bind_controller`. *Not* a `.myco` kind."
  > "`input_contract` — third argument to `bind_controller` — it defines the callable's visibility."

  §23.3 restates the first half (controller is workflow vocabulary); the second half (visibility = input_contract; expanding visibility requires expanding the contract) is not covered in §23 or §24.1. §24.1 describes the input contract as "types the controller reads from its scope" but does not state the invariant that visibility cannot be expanded without expanding the contract.

  `Recommend:` Add to §24.1 (or cross-reference from §23.3) a sentence stating that the input contract defines the callable's complete visibility surface, and that expanding what a callable can see means expanding the contract. This is a load-bearing reuse invariant from v2.1_in_progress.md, not open design.

- **`planning/v2/v2.1_in_progress.md:902-907` "SCC participation" of controllers:**
  > "When the callable's declared inputs include variables that are part of an algebraic loop, the callable's output variable joins that SCC as an opaque factor. Symbolic reasoning (`deriv(·)`, algebraic elimination) does not pass through it; gradient flow at train time uses the callable's native autodiff. SCC structure is determined post-binding — the planner runs after the workflow is applied."

  §24.2 covers gradient flow through the backend's AD but does not state the SCC participation rule explicitly: controllers inside loops join the SCC as opaque factors and block symbolic reasoning. §23.2 ("plan parameterized by its binding surface") touches the "planner runs post-binding" idea but does not pin down opaque-factor SCC participation.

  `Recommend:` Add to §24.2 (or §23.2) a sentence stating that `bind_controller` introduces an opaque factor into any SCC the controller's inputs participate in, and that symbolic reasoning does not cross the controller boundary. Deferred from chunk 04 / v2.1_in_progress.md as a stable decision.

- **`planning/v2/open_questions_deprecated_use_spec_new.md:776-783` Tier 2 "Cross-backend callable interop":**
  > "§23.3 locks that trained callables reuse across workflows via plain contracts. What's unresolved: if workflow A trains a callable on backend X (e.g., PyTorch), can workflow B bind the same callable when running on backend Y (e.g., JAX)? Weight-format translation, gradient-plumbing compatibility, and advertised-capability reconciliation all need to be specified."

  §23.3's "trained weights plus a plain contract" framing assumes backend compatibility without addressing this. §31 locks single-backend-per-run, but the cross-run / cross-backend interop question is left open. §23.3 does not acknowledge it.

  `Recommend:` Add a caveat to §23.3 noting that cross-backend weight portability (workflow A on PyTorch, workflow B on JAX) is an open item tracked in §35 Tier 2. Without this, §23.3's "same trained callable" language reads as unconditional.

- **`planning/v2/v2.1_in_progress.md:1057-1074` "Refinement-type bounds surfaced to the workflow" and projection helpers:**
  > "The compiler does not auto-emit projection: the choice of projection flavor (hard clip, sigmoid reparameterization, soft clip) is a training-dynamics decision that varies by problem."
  > "Backend-agnostic: all three flavors are elementary ops."

  §24.5 lists `run.config.backend` / `run.config.profile` etc. §25 covers projection-flavor selection. §23 does not mention that refinement-type bounds surface as workflow-visible metadata on the node catalog. Chunk 09's catalog spec lists path, type, role, and units but omits "refinement bounds."

  `Recommend:` Add "refinement bounds (where declared)" to the node-catalog field list in §23 preamble. This is a settled piece of catalog metadata from the v2.1_in_progress training-emission lock; it belongs in the boundary description.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md` "Mode B" and `v2.1_in_progress.md:434-432` type-based `where`:**
  > "A discriminant field on each leaf with a tagged-union / sum-type VC. Python binds the discriminant; `.myco` dispatches per-tag."

  This is the resolution pattern for per-instance heterogeneity from the Python side — Python binds a discriminant, `.myco` dispatches. §23 does not document this idiom. Chunk 09 flags it as depending on Myco sum types (see §35 open).

  `Recommend:` Add a forward reference from §23 to the dispatcher idiom once Mode B sum types lock. Current omission is acceptable because sum-type support is an open §35 item; worth revisiting then.

---

## Conflicts

Direct contradictions between spec_new.md §23 and any corpus document.

- **§23 preamble "Python verbs (`bind`, `observe`, `run`)" vs. `v2.1_in_progress.md:44-46` and §24 eight-verb taxonomy:**

  §23 preamble lists Python verbs as "`bind`, `observe`, `run`." The canonical glossary in `v2.1_in_progress.md:44-46` enumerates eight verbs (`assume_constant`, `assume_series`, `learn_constant`, `learn_initial`, `learn_trajectory`, `bind_controller`, `bind_topology`, `observe`). §24 locks the same eight. There is no bare `bind` verb; `run` is a workflow operation, not a binding verb. The §23 preamble phrasing conflates verbs with verb families, and conflicts with both the glossary and §24.

  `Recommend:` Rewrite the §23 preamble line to reference the eight-verb taxonomy explicitly (or paraphrase as "`assume_*`, `learn_*`, `bind_*`, and `observe`"). The current phrasing re-introduces the retired four-verb grouping by implication.

- **§23.3 "declared input contract" vs. §24.1 "input contract + output contract":**

  §23.3 frames cross-study reuse in terms of the callable's "declared input contract" and "output type's contract," speaking of the callable satisfying "the same trained callable, provided study B's required input contract matches the callable's declared input contract." §24.1 locks `bind_controller(path, fn, input_contract, output_contract)` with both contracts as binding-verb arguments, not properties carried on the callable itself. §23.3's "callable's declared input contract" framing suggests the callable carries the input contract; §24.1 suggests the contract is supplied at bind time.

  These are reconcilable — a callable trained against a given contract is tied to it by its weight geometry — but the §23.3 phrasing ("declared input contract") obscures that the contract is a binding-verb argument, not a callable attribute.

  `Recommend:` Tighten §23.3 to say that a trained callable is reused in a new workflow by passing the same (or a compatible) `input_contract` / `output_contract` at `bind_controller`. The callable does not intrinsically "declare" a contract; it is bound under one.

- **§23.4 "contract violations on bound callables" vs. §24.1 compile-time check framing:**

  §23.4 classifies "contract violations on bound callables" as workflow composition errors (tier 2). §24.1 states: "Compiler checks at workflow composition that the named fields exist in scope at the binding site." Consistent so far. The conflict arises with the `.myco` side: contract obligations on concrete types are §3.7 / §7.4 compile-time checks (tier 1). If a `.myco` file declares a type that fails a contract obligation used as a controller's `input_contract`, that is a tier-1 error, not tier-2. §23.4 does not distinguish between "contract violation at declaration" and "contract mismatch at bind," and the boundary is not sharp.

  `Recommend:` Clarify §23.4 that contract-obligation failures on `.myco` types are tier-1 (compile) errors, while contract mismatches between a bound callable's contract and the `.myco` site's type shape are tier-2 (composition) errors. This is implicit in §7 but worth naming at the boundary where the distinction becomes operational.
