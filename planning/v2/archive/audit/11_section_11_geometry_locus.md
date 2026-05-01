# Audit Report — §11 Geometry and Locus

Source: `planning/v2/spec_new.md §11` (fetched via `just spec-section 11`)
Corpus: `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` (chunk 01,
authoritative), `planning/v2/v2.1_in_progress.md`, `planning/v2/open_questions.md`,
`planning/v2/spec_dev_notes.md`, `planning/v2/anti_spec.md`.

---

## Absorbed

Content from the corpus that landed in spec_new.md §11.

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §3.2 — "The
  compiler generates **flux balance only** at graph junctions when it sees
  `diverg(flux_field)` ... **Continuity is not auto-generated.**" — absorbed
  into §11.8 (Default Junction Conditions).

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.4 — `locus`
  as "named structural singularity" declared inside a `geometry` block with a
  structural predicate — absorbed into §11.11 (`boundary`, `chart`, `metric`,
  `requires` vocabulary), §11.8, and the summary header.

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.7 — three
  standard boundary-condition forms Dirichlet / Neumann / Robin via `requires`
  blocks; "silence is not a free Neumann zero" — absorbed into §11.2.

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §6 (stdlib
  table) — `BranchingManifold`, `MetricGraph`, `Polar`, `Sphere`, `Euclidean`,
  `Interval` with topology / chart / loci — partially absorbed into §11.3.
  Note: §11.3 renames `Interval` to `Line1D` and `Euclidean<Dim=2>` to
  `Rectangle2D`, and adds `Ball3D` (see Conflicts).

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §3.4 and §3.5 —
  geometry coefficients via `requires` (not generics, not `hint`); `hint`
  keyword deferred/dropped — absorbed into §11.10.

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §3.3 —
  "Embedding is separate from intrinsic geometry ... lives on the entity as
  regular scalar fields" — absorbed into §11.9 (no embedding keyword) and
  §11.4 (horse/fly pattern).

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §4 — "For
  geometries with data-driven topology ... the Python workflow layer provides
  the specific graph/mesh structure" via `bind_topology` — absorbed into §11.5.

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §5 — compiler
  discretization: defaults are inferred from geometry class, override via
  `experiment.compile(spatial_config=...)` — absorbed into §11.5 and §11.6.

- `planning/v2/v2.1_in_progress.md` line 492 — "`temporal name on locus:` is
  legal by symmetry with `relation name on locus:`" — absorbed as the summary
  line for §11 ("on locus: clause applies symmetrically to `relation` and
  `temporal`").

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.3 (graph
  field semantics) — distinction between locus-scoped (one value per edge
  instance) vs edge-interior (function of interior coordinate) fields —
  absorbed into §11.7.

- `planning/v2/anti_spec.md` — "`assume_topology` | `bind_topology`" — the
  verb-lock is reflected in §11.5 which uses `bind_topology` throughout.

---

## Superseded

Corpus content replaced by a newer decision in §11. Where anti_spec.md already
records the retirement, this is noted and the item is not separately flagged.

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.5 —
  `trace(field, edge = label)` is described as "specific to graph topologies"
  with detailed `parent`/`children` edge-label API for `rooted_tree`. The
  report explicitly mentions `trace` as a language construct. §11 does not
  expose `trace` as part of §11's operator surface; §11.1 lists `trace(f,
  boundary)` only in the sense of manifold restriction ("restriction of `f` to
  the named boundary sub-locus"), not the directional-limit form for graph
  junctions. The graph-junction `trace` mechanism is intact in the corpus but
  is not presented as a named spatial operator in §11.1 — it appears implicitly
  in §11.8. This is not a conflict but a coverage gap (see Homeless).

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.3 — `field
  name: Type` declared as "`field name on locus_name: Type`" for vertex-local
  0D state; worked example uses `field junction_volume on junction: Scalar<m3>`.
  §11.7 absorbs this distinction but calls them "locus-scoped" fields. The
  specific `field ... on locus: Type` syntax is implied but not shown in §11.7.
  Not truly superseded — just less explicit.

- `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §8.4 —
  compiler-backend L'Hopital limit at declared `locus pole` to avoid
  `1/sin(theta)` NaN. Chunk 04 (`04_egraph_foundation_in_progress.md`)
  relocks this as CC5: "site-scoped rewrite" (`predicate: site(f) ∈
  locus("pole")`). Neither formulation is in §11; this is open design work (not
  homeless per the in-progress exemption).

---

## Homeless

Corpus content that is relevant to §11, not accounted for in §11, and not
already committed to anti_spec.md. Only stable, settled decisions are flagged
here; ongoing open design from in-progress chunks is excluded.

- `Recommend:` **`geometry` keyword and `Domain<G>` annotation are not named in
  §11.** `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.1
  and §2.2 establish `geometry` as "a first-class language construct declared
  with the `geometry` keyword" and `Domain<G>` as "an ordinary Myco composite
  type annotated with `Domain<G = SomeGeometry>`." These are the foundational
  constructs for all of §11, but §11 never names them. A reader of §11 alone
  cannot tell how a horse acquires its geometry or what `bind_topology` is
  binding to. Add a short §11.0 (or preamble) naming `geometry` and `Domain<G>`
  with forward-references to the type system.

- `Recommend:` **`as` clause (coordinate names, units, extents) is absent from
  §11.** `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §3.1 is
  a settled decision: "the `as` clause ... is declared on the domain type, not
  inside the `geometry` block." `planning/v2/v2.1_in_progress.md` line 1142
  restates it. The `as` clause is how users attach physical coordinates to a
  domain, and the positional mapping rule (first `as` name binds to first
  `chart` binder) is stable. §11 contains no mention of `as`. This omission
  makes §11.5 (`bind_topology`) harder to understand because edge-length units
  are validated against the `as`-clause coordinate units.

- `Recommend:` **`trace()` as directional-limit primitive for graph junctions is
  absent.** `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.5
  is a settled section: "`trace(field, edge = label)` evaluates a field's
  limiting value as you approach a locus from a specific direction." The
  junction physics (pit-drop example, `replaces balance(axial_flux)`) in §11.8
  is incomplete without `trace` being defined somewhere in §11. §11.1's
  `trace(f, boundary)` is the manifold-restriction form, not the same as the
  graph-junction directional-limit form. These two uses share a name but have
  different semantics; §11 needs to distinguish them or introduce graph `trace`
  explicitly.

- `Recommend:` **Locus-scoped relations with `replaces` obligation keys are
  absent.** `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.6
  is a settled section: "When a locus-scoped relation replaces a
  compiler-generated default, it names the obligation it replaces using a stable
  semantic key — not a user-chosen relation name: `relation leaky_junction on
  junction replaces balance(axial_flux):`." This is referenced in
  `planning/v2/v2.1_in_progress.md` line 1297 and in the open-questions e-graph
  item W1. The `replaces` keyword for geometry obligations is a stable decision
  not appearing in §11.

- `Recommend:` **`identify` (periodic seam declaration) is absent from §11.**
  `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.8 is a
  settled section: "`identify phi = 0 <-> phi = 2 * pi`" on the `Sphere`
  geometry, with the scope restriction "v2.1: scalar fields only; vector/tensor
  seam transforms deferred." `planning/v2/v2.1_in_progress.md` line 1154
  summarises this. `planning/v2/open_questions.md` line 153 lists it among
  geometry-report items that "assume an e-graph without saying so." The `Sphere`
  geometry appears in §11.3 but without `identify`, making `Sphere` underdefined
  (its longitude coordinate is periodic). At minimum §11.3 should note the
  `identify` seam for `Sphere` and forward-reference a dedicated subsection.

- `Recommend:` **`bind_topology` schema and validation rules are absent.** The
  settled schema for `rooted_tree` and `metric_graph` is in
  `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §4 and §4.1:
  vertex IDs, edge list, root, edge-length units (must match `as` clause),
  vertex tags (must cover all `where`-predicate keys). §11.5 says
  "`bind_topology` supplies discretization" but does not describe what it
  validates or what schema it expects. The manifold case (no `bind_topology`
  needed; mesh resolution via `experiment.compile(spatial_config=...)`) is also
  absent.

- `Recommend:` **`continuous(field)` and `kirchhoff(potential, flux)` stdlib
  helpers are absent.** `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md`
  §3.2 and `planning/v2/v2.1_in_progress.md` lines 1286 and 1290 describe these
  as settled stdlib convenience functions for opt-in continuity at junctions.
  §11.8 states "the modeler writes an explicit `requires: left.f = right.f`" but
  does not mention the stdlib helpers. A brief note would complete the junction
  story.

- `Recommend:` **Subdimensional fields (`field name: Type over coord`) are
  absent.** `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.3
  and `planning/v2/v2.1_in_progress.md` line 1414 both settle the `over`
  keyword for fields that vary over fewer dimensions than their containing
  domain. §11.7 covers edge-interior vs locus-scoped but does not cover the
  analogous `over` form for manifold domains. The horse/fly example in
  `v2.1_in_progress.md` line 1839 uses subdimensional fields.

- `Recommend:` **`curl` is listed as settled but absent from §11.1.** Both
  `planning/v2/v2.1_in_progress.md` line 1147 and `planning/v2/open_questions.md`
  line 659 include `curl` in the settled operator list alongside `grad`,
  `diverg`, `laplacian`, `normal_grad`. §11.1 lists only `grad`, `diverg`,
  `laplace`, `normal_grad`, and `trace`. If `curl` is settled it belongs in
  §11.1; if it is deferred it should appear in the open-questions note at the
  end of §11.1's summary.

- `Recommend:` **`locus-scoped temporal` is absent from §11.**
  `planning/v2/v2.1_in_progress.md` line 492 marks this as settled: "`temporal
  name on locus:` is legal by symmetry with `relation name on locus:`."
  `planning/v2/spec_dev_notes.md` line 248 records this as part of §11 in the
  changelog. The §11 summary header mentions it ("on locus: clause applies
  symmetrically to `relation` and `temporal`") but no subsection documents the
  semantics or gives an example. Given that locus-scoped relations have their
  own §11.8 treatment, locus-scoped temporals deserve at least a short parallel
  subsection.

- `Recommend:` **Terrain-as-field deprioritization decision is absent.**
  `planning/v2/open_questions.md` line 626 states: "Terrain-as-field on a flat
  domain covers all practical use cases. Irregular boundaries are an
  elegance/efficiency concern, not a correctness concern." This is a scoping
  decision that closes a potential §11 design question. It belongs in §11 or
  anti_spec.md to prevent future re-litigation.

---

## Conflicts

Direct contradictions between spec_new.md §11 and corpus documents.

- **Stdlib geometry names.** §11.3 names six standard geometries: `Line1D`,
  `Rectangle2D`, `Ball3D`, `RootedTree`, `MetricGraph`, `BranchingManifold`.
  `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §6 names:
  `Interval`, `Euclidean<Dim>`, `Polar`, `Sphere`, `BranchingManifold`,
  `MetricGraph`. The two sets share only `BranchingManifold` and `MetricGraph`.
  `planning/v2/v2.1_in_progress.md` line 1382 repeats the chunk-01 names
  (`Interval`, `Euclidean<Dim>`, `Polar`, `Sphere`). §11.3 adds `Ball3D` (not
  present anywhere in the corpus) and renames `Interval` to `Line1D`,
  `Euclidean<Dim=2>` to `Rectangle2D`, and drops `Polar` and `Sphere`
  entirely. These are load-bearing names: code examples in chunk 01 and
  `v2.1_in_progress.md` use `Interval` and `Euclidean`. The `Sphere` worked
  example drives the `identify` and pole-L'Hopital designs.
  `Recommend:` Reconcile the stdlib names. Either adopt §11.3's names
  throughout (and update all corpus examples) or revert §11.3 to the chunk-01
  names. Any renaming is a spec-new decision that must be reflected in
  anti_spec.md with a retirement entry for the old names.

- **`laplace` vs `laplacian`.** §11.1 spells the Laplacian operator as
  `laplace`. Every other corpus document spells it `laplacian`:
  `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2 ("derive
  `grad`, `diverg`, `laplacian`, `curl`"), `planning/v2/v2.1_in_progress.md`
  line 1451 ("`laplacian()`"), line 1880 ("`laplacian(body_temp)`"),
  `planning/v2/open_questions.md` line 659 ("settled: `grad`, `diverg`,
  `laplacian`"), `planning/v2/v2.1_chunk_reports/04_egraph_foundation_in_progress.md`
  (rewrite rule `rewrite pole_laplacian: pattern: laplacian(f)`).
  `Recommend:` Standardize to `laplacian` to match every other document and
  the worked examples. Record `laplace` as a retired alias in anti_spec.md if
  it ever appeared in user-facing examples.

- **`trace` overloading.** §11.1 defines `trace(f, boundary)` as "restriction
  of `f` to the named boundary sub-locus." In
  `planning/v2/v2.1_chunk_reports/01_geometry_design_report.md` §2.5, `trace`
  is defined as a directional-limit primitive for graph junctions:
  "`trace(field, edge = label)` evaluates a field's limiting value as you
  approach a locus from a specific direction." The two are distinct operations
  sharing a name. §11.1 presents only the manifold-restriction form and gives
  the graph-junction form no home in §11.
  `Recommend:` Either split into two distinct names (e.g., `restrict` for
  manifold boundary and `trace` for graph directional-limit, matching the PDE
  literature convention) or explicitly document both overloads in §11.1 with a
  note on dispatch by topology class. The current text silently omits the
  graph form, which is required for §11.8's junction physics to be executable.
