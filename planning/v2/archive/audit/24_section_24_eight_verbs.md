# Audit Report — §24 Eight Workflow Verbs

Audited against corpus as of 2026-04-22.

---

## Absorbed

Corpus content that already landed in spec_new.md §24.

- **`planning/v2/v2.1_in_progress.md:858-878` (Workflow side: binding verbs):**
  > "At bind time the Python harness supplies sources for variables whose values should come from outside the `.myco` relations. The v2.1 verbs:
  > - `assume_constant(path, value)` ...
  > - `assume_series(path, values)` ...
  > - `learn_constant(path, init)` ...
  > - `learn_initial(path, init)` ...
  > - `learn_trajectory(path, init)` ...
  > - `bind_controller(path, fn, input_contract)` ...
  > - `bind_topology(...)` ...
  > - `observe(path, data)` ..."

  Absorbed verbatim into the §24 preamble list.

- **`planning/v2/v2.1_in_progress.md:880-909` (`bind_controller` specifics):**
  > "The term *controller* is workflow vocabulary only ... there is no `Controller` kind or contract. ... The callable receives exactly the fields that contract declares ... This supports 'pretrain with heuristic, fine-tune with data' workflows without language-level special casing."

  Absorbed into §24.1 (path / `fn` / input contract / output contract bullets) and the closing paragraph: "Controllers are purely workflow concept. No `.myco` keyword introduces a controller."

- **`planning/v2/v2.1_in_progress.md:902-907` (SCC participation, gradient flow via backend AD):**
  > "SCC structure is determined post-binding. ... gradient flow at train time uses the callable's native autodiff."

  Absorbed into §24.2 "Backward pass ... via the backend's AD facility (§31)."

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md:270-287` (§4.5 Opaque callable protocol):**
  > "Lean: v2.1 commits to same-backend-per-run for simplicity."
  > "gradient flow work when the callable is inside a training-time SCC? Backend AD through the callable requires the callable to live in the same AD frame as the rest of the computation."

  Absorbed into §24.2 backward-pass bullet (backend AD) and opaque-fn fallback bullet.

- **`planning/v2/v2.1_in_progress.md:1350-1374` (`bind_topology` — workflow-layer topology injection):**
  > "For geometries with data-driven topology (branching networks, irregular meshes), the Python workflow layer provides the specific graph/mesh structure"
  > "`bind_topology` uses `bind` because topology injection is structurally different from assuming a quantity's value (it provides graph structure, not a scalar or series)."

  Absorbed into §24.3 (concrete mesh, boundary identification, material coefficients, event-time capacity bullets; "the only path by which geometry becomes executable").

- **`planning/v2/v2.1_in_progress.md:934-939` (Multi-binding compilation):**
  > "Same `.myco` model, N binding sets, one callable shared across them. Essential for sparse multi-study training — every study's concrete model satisfies the same contract, so `bind_controller` with the same callable threads the same weights through all of them."

  Absorbed into §24.1 "multi-binding is supported (§23.2) through the same mechanism other verbs use" and §24.2 "Cross-run weight persistence."

- **`planning/v2/v2.1_in_progress.md:925-928` (`bind_topology(...)`):**
  > "Structurally different from scalar-value binding — supplies geometry/connectivity rather than numerical values."

  Absorbed into §24.3.

- **`planning/v2/v2.1_in_progress.md:918-923` (`assume_constant`, `assume_series`):**
  > "`assume` preserves the mathematical sense (fix a value for the purposes of this experiment); it is distinct from `bind_controller` (attach a callable) and `observe` (treat values as evidence to condition on)."

  Absorbed into §24 preamble verb catalogue.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md:64-114` (bind / observe / run primitives):**
  > "The Python library provides three kinds of verbs. ... The library type-checks supplied values against the node catalog: shape, dtype, and (where applicable) units."

  Absorbed in principle into §24; type-checking of input contracts against scope is stated in §24.1 ("Compiler checks at workflow composition that the named fields exist in scope at the binding site").

---

## Superseded

Corpus content replaced by newer decisions in §24. Note whether already in anti_spec.md.

- **`planning/v2/spec.md:1654-1712, 1889-1989` (`slot` / `bind_slot` / `learn_slot` / `bind_slot_metadata` machinery):**
  > "A slot is a declared interface for a component that will be provided at workflow composition."
  > "`experiment.bind_slot('stomatal_control', ...)` ... `experiment.learn_slot('stomatal_control')`"

  Superseded by §24.1's `bind_controller(path, fn, input_contract, output_contract)`. Closing paragraph: "This retires the `slot` / `learn_slot` machinery and the transparent-heuristic ABI."

  Already captured in `anti_spec.md:13`:
  > "`slot` / `learn_slot` / `bind_slot` / `bind_slot_metadata` | `bind_controller(path, fn, input_contract)` | controller is workflow-only, no `.myco` kind"
  No further action needed.

- **`planning/v2/spec.md:1664-1670, 1747-1793` (`[*]` wildcard slot inputs, wildcard partition, `dyn` wildcard restrictions):**
  > "The `[*]` wildcard means 'all quantities structurally reachable from the slot's outputs via the model's relation graph...'"

  Superseded by `bind_controller`'s explicit `input_contract`. Already in `anti_spec.md:14`:
  > "`[*]` wildcard slot inputs | controller data contract | explicit I/O spec"
  No further action needed.

- **`planning/v2/spec.md:1808-1867` (transparent vs opaque slot implementations, transparent-heuristic ABI):**
  > "A slot does not declare an implementation. The implementation is supplied at workflow composition."
  > "Transparent controllers for wildcard slots ... path-rebasing."

  Superseded by §24.1 unified `bind_controller` for any callable. Already in `anti_spec.md:15`:
  > "transparent-heuristic ABI | unified `bind_controller` | one mechanism for pluggable behavior"
  No further action needed.

- **`planning/v2/v2.1_in_progress.md:849-855` ("v2.0 had a `slot` keyword ... v2.1 drops it"):**
  > "This is a deliberate change from v2.0, which had a `slot` keyword ... v2.1 drops it"

  Superseded as versioning prose. Retired per Riley's spec-prose hygiene: history belongs in dev_notes, not spec. Already handled by `anti_spec.md` slot retirement; the versioning narrative itself is retired per `anti_spec.md:68` ("'slot is gone' narrative / 'v2.0 had X' retirement prose | none — use anti_spec.md instead of in-spec versioning").

- **`planning/v2/open_questions_deprecated_use_spec_new.md:787-810` (Workflow Verb Taxonomy — four-way grouping question):**
  > "Earlier drafts grouped them into a 'four-way workflow vocabulary' (`assume` / `observe` / `learn` / `bind`) ... That grouping may or may not survive the clarified framing..."

  Superseded by §24's flat eight-verb listing. The taxonomy question is dissolved: §24 makes no claim of a four-way grouping. Specific four-way grouping is not retired in anti_spec.md but is structurally void now that §24 is locked.

  `Recommend:` Minor. Grouping question is structurally resolved by §24's flat enumeration; no anti_spec.md entry needed because nothing in spec_new.md ever committed to the four-way form.

- **`planning/v2/spec.md:1944-1989` (Trained slot serialization and rebinding, interface manifest):**
  > "After training, a learned slot's parameters can be saved and later rebound..."
  > "`experiment.bind_slot('stomatal_control', 'path/to/trained_controller')`"

  Superseded by §24.2 "Cross-run weight persistence. Trained weights persist across runs that bind the same callable (§23.3)." The slot-interface-manifest mechanism is subsumed by plain contracts and capability advertising (§6). Slot retirement already in anti_spec.md covers this.

---

## Homeless

Corpus content relevant to §24, not in §24, not in anti_spec.md. Highest-value bucket.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:457-465, 1087-1094, 1163-1164, 1675-1676` (workflow extraction and loss-estimation configuration):**
  > "`run.config.extraction_policy` and `run.config.loss_estimation` are v2.1 workflow verbs."
  > "`run.config.extraction_policy = { compute_weight: 1.0, loss_weight: 10.0, loss_cap: 0.01 }`"
  > "`run.config.loss_estimation = { sampling: { n_samples: 1000, seed: 42, strategy: 'stratified' }, ... }`"

  §24.5 lists representative run-config fields (`seed`, `backend`, `verbosity`, `dt`, `profile`) but does not name `run.config.extraction_policy` or `run.config.loss_estimation`, both of which are locked workflow-surface fields per chunk 04 O2.4 and §19.1 extraction.

  `Recommend:` Add `run.config.extraction_policy` and `run.config.loss_estimation` to §24.5's representative fields list, or add a cross-reference to §19 / §14 where their shape is specified. This is settled design, not open work.

- **`planning/v2/v2.1_chunk_reports/06_backend_abstraction_in_progress.md:170, 334-335` (backend fallback policy as workflow knob):**
  > "Workflow knob: `run.config.backend.fallback = 'error' | 'host' | 'emulate'`"
  > "`run.config.backend`, capability probing, fallback policy, version pinning — all new workflow verbs."

  §24.5 names `run.config.backend` and mentions "capability-fallback mode (error / host / emulate, §31)" inline, but does not explicitly name capability probing or version pinning as run-config surfaces. Chunk 06 §4.7 commits to version pinning ("`run.config.backend` includes version pin option").

  `Recommend:` Add a bullet in §24.5 for backend version pinning (or a forward reference to §31). Light touch; one sentence.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md:64-74, 150-160` (load / compile / spawn and run / control verbs):**
  > "`world = myco.load('my_model')` ... `pop = world.spawn('Leaf', n=1000)`"
  > "`run = world.run(duration=30_days, dt=0.5_hours)` ... `run.checkpoint('day_15.ckpt')`"

  §24 covers binding verbs only. Chunk 09 separates `load` / `spawn` / `run` / `checkpoint` as an orthogonal verb family distinct from the eight binding verbs. §24.5 mentions run-config but not the run lifecycle verbs.

  `Recommend:` Add a brief forward reference in §24 (preamble or §24.4 "Future Verbs Beyond the Eight") noting that `load` / `spawn` / `run` / `checkpoint` are separate orchestration verbs tracked in §31 or the data-layer chunk, not among the eight binding verbs. Without this, §24's "eight verbs are the complete workflow-composition surface" is ambiguous: it is the complete binding surface, not the complete Python-API surface.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md:119-148` (Python value providers vs `.myco` distributions):**
  > "`myco.uniform(low, high)` ... `myco.lognormal(mu, sigma)` ... These are value providers, not Myco distribution types. They produce concrete numbers at bind time. They are distinct from the `.myco` `Distribution<U>` contract."

  §24's `assume_constant` / `assume_series` bullets do not address the case where a workflow-side value provider (RNG) supplies the numbers rather than a fixed array. This is a locked distinction per chunk 09 ("Python value providers are distinct from `.myco` distributions. Locked.") but absent from §24.

  `Recommend:` Add one sentence to the `assume_series` / `assume_constant` description in §24 noting that the supplied value may come from a Python value provider (numpy array, `myco.lognormal(...)` draw) and that this is distinct from the `.myco` `Distribution<U>` contract (§13). Settled design, not open work.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md:94-96` (bind-time type-checking failure mode):**
  > "The library type-checks supplied values against the node catalog: shape, dtype, and (where applicable) units. Mismatches are errors at bind time, not at run time."

  §24.1 states the compiler checks input-contract field presence at workflow composition, but does not state the broader type-checking invariant (shape / dtype / units at bind time for `assume_*` and `observe`). This is a locked guarantee per chunk 09 absent from §24.

  `Recommend:` Add a sentence to §24 preamble or a dedicated §24.x bullet: "All workflow verbs type-check their supplied values against the node catalog (shape, dtype, units) at workflow composition; mismatches are bind-time errors, not run-time."

- **`planning/v2/open_questions_deprecated_use_spec_new.md:881-887` (Workflow-side API for epistemic priors, `assume_prior`):**
  > "Parameter priors (Bayesian beliefs about unknown values) live workflow-side. The verb name and signature (`assume_prior(path, Distribution)` or similar) ... are workflow design questions. Not blocking v2.1 `.myco` language spec."

  §24.4 lists `assume_prior` as a candidate future verb and defers it to Tier 2 PPL. The cross-reference between epistemic `~` (§13.1) and `assume_prior` is already acknowledged in `audit/adjudication.md:229` and `audit/13_section_13_probabilistic_programming.md:116`. §24.4 correctly defers.

  `Recommend:` No action. Cross-reference exists.

- **`planning/v2/v2.1_chunk_reports/09_workflow_data_layer.md:162-180` (Mode B interaction — per-instance contract-type selection):**
  > "The dumb-data principle means per-instance contract-type selection cannot be driven from Python alone. If Mode B is desired ... the choice must be made `.myco`-side."

  This is a constraint on binding-verb semantics: the verbs cannot drive per-instance type dispatch. §24 does not state this constraint. It is relevant to `bind_controller` in particular (a single callable cannot be per-instance-dispatched from the workflow verb alone).

  `Recommend:` Optional. Add a Mode-B caveat to §24.1 noting that per-instance type dispatch must be expressed in `.myco` (tagged-union or species-level commitment), not driven from Python via the binding verbs. This is locked per chunk 09 but nowhere stated in spec_new.md §24. Low priority; arguably belongs in the type system sections, not §24.

- **`planning/v2/open_questions_deprecated_use_spec_new.md:776-783` (cross-backend callable interop):**
  > "if workflow A trains a callable on backend X (e.g., PyTorch), can workflow B bind the same callable when running on backend Y (e.g., JAX)? Weight-format translation, gradient-plumbing compatibility, and advertised-capability reconciliation all need to be specified."

  §24.2 "Cross-run weight persistence" states persistence across runs that bind the same callable, but §24 does not acknowledge the cross-backend case, which is open design. The single-backend-per-run lock (chunk 06 §4.6) caps intra-run scope but leaves cross-run interop open.

  `Recommend:` Add a caveat to §24.2 "Cross-run weight persistence" bullet: persistence within a consistent backend is settled; cross-backend persistence is tracked as an open item (v2.2+) in chunk 06 §4.5 / §35.

- **`planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md:1375-1382` (workflow-constant injection as observation-style merge):**
  > "Workflow-bound values enter the e-graph as observation-style equalities at workflow composition — semantically identical to `observe(path, data)`, just delivered by a different workflow verb."

  This is a load-bearing semantic claim about the eight verbs: several of them (`assume_constant`, `assume_series`, `bind_known_constants`, `observe`) ultimately deliver the same mechanism (e-class merge with a literal value). §17.1 merge source 2 ("workflow constant injection") covers the merge-source framing. §24 does not cross-reference §17.1 or say that the verb catalog maps onto a narrower set of substrate mechanisms.

  `Recommend:` Add a brief note to §24 preamble cross-referencing §17.1 source 2 ("workflow constant injection"): the eight verbs are the user-facing binding surface; internally they reduce to a small number of merge / envelope / fact-registration mechanisms. Helps readers coming from §17.

---

## Conflicts

Direct contradictions between spec_new.md §24 and any corpus doc.

- **§24 preamble "`bind_controller` ... `input_contract`" vs §24.1 "`bind_controller(path, fn, input_contract, output_contract)`":**

  §24 preamble lists `bind_controller` without signature detail. §24.1 gives the signature as four-argument with both `input_contract` and `output_contract`. `planning/v2/v2.1_in_progress.md:867` specifies `bind_controller(path, fn, input_contract)` (three-argument); `open_questions_deprecated_use_spec_new.md:347` uses `bind_controller(path, fn, Tree)` (three-argument with `Tree` as the contract). anti_spec.md:13 retires the slot machinery with replacement "`bind_controller(path, fn, input_contract)`" (three-argument).

  §24.1 introduces a fourth argument `output_contract` that is absent from all corpus sources. The output contract is semantically useful (capability obligations on output drive gradient flow and admissibility per §24.1), but its elevation to a formal signature argument is novel in spec_new.md §24.1 relative to the corpus.

  `Recommend:` Decide whether `output_contract` is a separate argument or is inferred from the callable's declared return type (in which case it is not a signature slot). If a separate argument, update anti_spec.md:13's replacement cell to the four-argument form for consistency; update `v2.1_in_progress.md:867` to match (or mark as stale). If inferred, reword §24.1 to describe the output contract as a property of `fn`'s declared return type, not a signature argument.

- **§24.5 `run.config.dt` framing vs `v2.1_in_progress.md:325-330` "`dt` as workflow-layer concern":**

  §24.5 states `run.config.dt` is "referenced via `assume_constant` in a discrete-time model (§9.1)." `v2.1_in_progress.md:325-326` specifies: "For `step(·)`, the tick cadence is set at the workflow layer via `assume_constant('config.dt', …)` or `assume_series(…)` for variable time-stepping." `v2.1_in_progress.md:328-330` also notes `dt` "remains a normal quantity in the world model when it appears as a physical quantity."

  The conflict is minor: §24.5's phrasing "referenced via `assume_constant`" implicitly excludes the `assume_series` path. Variable time-stepping via `assume_series("run.config.dt", ...)` is a locked feature.

  `Recommend:` Change §24.5's `run.config.dt` bullet to note that both `assume_constant` (fixed tick) and `assume_series` (variable tick) paths are supported, matching `v2.1_in_progress.md:325-326`.
