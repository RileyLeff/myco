# 09 — Workflow Data Layer

Durable summary of the Python-side / workflow-side data layer. Locks
the principle that Python is a dumb data-provenance layer and never
knows `.myco` types. Locks catalog-backed addressing via `NodePath`,
`FacetPath`, and workflow-only `Selector`s. The canonical home is
`spec_new.md` §23.5 and §24.

## Principle

**Python does not know Myco types.** The Python library is a generic
data-provenance and orchestration layer. It handles data in, data out,
run orchestration, and infrastructure (RNG, wall-clock, checkpointing).
It does not handle type construction, contract implementation, or model
semantics.

Consequence: spore authors ship one artifact (`.myco` sources +
`myco.toml`). There is no Python mirror package. The Python library's
surface grows along one axis only: generic workflow primitives and data
adapters.

## What Python Sees

When a `.myco` model compiles, the resulting artifact exposes a
**catalog**: a structured manifest of every workflow-visible slot.

Each catalog entry carries:

- **Canonical path** — stable name for the declaration-level schema
  slot.
- **Declared type expression** — specialized when the compiled model is
  concrete; explicitly constrained when an interface remains generic.
- **Unit / numeric representation** — dimension, named unit where
  relevant, dtype / representation requirements, and conversion
  affordances.
- **Axes / shape facts** — static, provider-validated, runtime-bounded,
  dynamic-keyed axes, shape expression provenance, and matrix facts.
- **Binding roles / facets** — whether the slot accepts source values,
  initial values, observations, topology, process priors, controller
  outputs, or output queries.
- **Contracts / refinements** — obligations and capability requirements
  that bound values must satisfy.
- **Existence domain** — where the slot exists over instance, time,
  geometry, event phase, enum variant, or dynamic topology.
- **Diagnostics metadata** — source location, declaration origin,
  stable catalog id, and human-readable schema text.

Python does not see:

- relation bodies or their equational content;
- contract implementations or method dispatch internals;
- e-graph structure, rewrite choices, extraction plans;
- spore-specific types, entities, or enums as Python classes.

## Addressing

The canonical workflow address is a catalog-backed handle:

- **`NodePath`** — a dumb handle to a catalog node / schema slot.
- **`FacetPath`** — a dumb handle to a bindable or observable facet such
  as `path.initial`.
- **`Selector`** — a workflow-only query over catalog metadata for bulk
  binding, bulk querying, diagnostics, and policy selection.

All three are dumb workflow objects. They carry catalog identity and
schema metadata; they do not implement `.myco` semantics.

Strings remain accepted because serialized workflows, config files, CLI
commands, and quick scripts need them. A string is not the canonical
semantic object; it immediately resolves through the catalog.

```python
model = myco.load("leaf_model")
cat = model.catalog

psi = cat.path("leaf.water_potential")
workflow.bind(psi, Series(df, column="psi", unit="MPa",
                          index=["leaf_id", "time"]))

# equivalent convenience, resolved through the same catalog:
workflow.bind("leaf.water_potential",
              Series(df, column="psi", unit="MPa",
                     index=["leaf_id", "time"]))
```

Paths name schema slots, not runtime instances. Instance, time,
coordinate, event phase, and variant selection live in bind/query
arguments and catalog metadata.

## Dynamic Existence

Dynamic objects and event-driven fields are represented through the
catalog entry's existence domain. A path can be valid even when no
value exists for a particular coordinate.

```python
tip = cat.path("root.tip.position")
run.query(tip, at={"root_id": roots, "time": times}, missing="masked")
```

Required missing/existence policies:

- **`error`** — strict default for bindings and queries when requested
  coordinates include nonexistent slots.
- **`masked`** — return values plus an existence mask.
- **`ragged`** — return dynamic-keyed data for axes whose cardinality
  varies by time, event phase, or parent instance.
- **`nan`** — output convenience only when dtype / unit and downstream
  container support it; never the semantic model.

Bindings over dynamic domains must either match the existence domain or
explicitly provide an inactive / mask column. Values for pre-birth,
post-removal, inactive enum-variant, or otherwise nonexistent slots are
workflow-composition errors unless the adapter declares those rows
inactive.

## Complex Types, Enums, and Generics

Structured values bind through the whole slot. Variant-specific field
paths require explicit narrowing through the catalog:

```python
stage = cat.path("leaf.stage")
workflow.bind(stage, {"tag": "Seedling",
                      "fields": {"age": 12.0, "height": 0.08}})

seedling_height = stage.variant("Seedling").field("height")
```

An unqualified path such as `leaf.stage.height` is invalid unless the
field is common to every variant with the same type, unit, and
existence domain. This mirrors the `.myco` narrowing rule.

Compiled concrete models expose specialized generic uses wherever
specialization is known. Reusable spore/interface artifacts may expose
constrained generic entries, but the catalog spells the type parameters
and required contracts explicitly. Workflow code never constructs or
subclasses Myco type parameters in Python.

## Primitives

The Python library provides generic workflow verbs. Exact method names
can evolve; the semantic shape is locked.

### Load / Compile

```python
model = myco.load("my_model")   # compile + load a .myco model
cat = model.catalog
```

### Bind

Associate a catalog path with source data:

```python
leaf_kmax = cat.path("leaf.kmax")
workflow.bind(leaf_kmax, Constant(3.0, unit="mol_m2_s"))

vc_b = cat.path("leaf.vc.b")
workflow.bind(vc_b, Prior(myco.lognormal(mu=1.5, sigma=0.3)))

psi = cat.path("leaf.water_potential")
workflow.bind(psi, Series(df, column="psi", unit="MPa",
                          index=["leaf_id", "time"]))
```

The library type-checks supplied values against the catalog: shape,
dtype, units, contracts, existence domain, and capability requirements.
Mismatches are workflow-composition errors.

### Bulk Binding

Large models must not require one call per scalar. Bulk adapters map
external data to catalog paths explicitly or through catalog-backed
selectors.

```python
workflow.bind_frame(
    df,
    mapping={
        "leaf_id": axis("leaf"),
        "time": axis("time"),
        "psi": cat.path("leaf.water_potential"),
        "k": cat.path("leaf.conductance"),
    },
    units={"psi": "MPa", "k": "mol_m2_s"},
)
```

Required adapter families: pandas / Polars dataframes, xarray objects,
nested dict / list data, NumPy-like arrays or matrices, and file-backed
readers such as CSV or Parquet. Exact class/function names remain API
detail; catalog-driven validation is normative.

### Observe Evidence

`workflow.observe(path, data)` attaches evidence to a catalog-resolved
path as layer-2 envelope metadata. It does not assert equality unless
the `.myco` model explicitly states a hard observation model.

```python
workflow.observe(
    cat.path("leaf.water_potential"),
    data=obs_df,
    noise=Normal(sigma=obs_sd),
)
```

### Query Outputs

Output retrieval is a workflow-library query surface, not the evidence
verb. Common scientific-Python containers should be supported: NumPy
arrays, xarray DataArrays / Datasets, pandas / Polars DataFrames, and
nested dicts.

```python
run = workflow.run(duration=30_days)

k_final = run.query(cat.path("leaf.conductance"),
                    at={"time": "final"},
                    format="xarray")

df = run.query_frame([
    cat.path("leaf.conductance"),
    cat.path("leaf.water_potential"),
    cat.path("leaf.net_photosynthesis"),
])
```

## Sampling / Distribution Primitives

Workflow distribution builders and file readers are **value providers**
or source-object helpers, not Myco distribution types.

```python
myco.uniform(low, high)
myco.lognormal(mu, sigma)
myco.normal(mu, sigma)
myco.fixed(value)
myco.from_csv(path, column)
myco.from_dataframe(df, column)
```

They package values, providers, priors, or workflow artifacts for
binding. They are distinct from `.myco` `Distribution<S>` contracts,
which live inside the model and participate in graph semantics.

## Run / Control

RNG control, wall-clock scheduling, checkpointing, restart, backend
selection, and relaxation policy are Python-side workflow concerns.
None of these cross into `.myco` as model syntax.

```python
run = workflow.run(duration=30_days)
run.checkpoint("day_15.ckpt")
```

## Mode B Interaction

The dumb-data principle means per-instance contract-type selection
cannot be driven from Python alone. Heterogeneous contract selection is
represented `.myco`-side through sum types / enums (§3.10). Python binds
catalog-validated tagged records; `.myco` dispatches by explicit
`match`.

## What This Locks

- Python is a data layer, not a model layer.
- Spore authors ship `.myco` + `myco.toml` only; no Python mirror.
- Catalog-backed `NodePath` / `FacetPath` is the canonical address
  model; strings are serialization / convenience.
- `Selector`s are workflow-only catalog queries, not source wildcards.
- Paths name schema slots, not runtime instances or Python objects.
- Existence domains, masks, and ragged outputs are required for dynamic
  worlds.
- Bulk binding is first-class and adapter-driven.
- Python value providers are distinct from `.myco` distributions.

## Remaining API Details

- Exact Python method/class names for adapters and query containers.
- Full output-format menu and streaming / partial-observation APIs.
- Parameter inference / calibration API (posterior draws, MCMC
  integration), building on the locked distribution and process-prior
  semantics.

## Relationship to the Relation/Fn Lock

Chunk 08's dumb-data Python principle is the partner to the "ban user
fn, everything equational" rule on the `.myco` side. Together they
establish: **all model semantics lives in `.myco`; Python handles data
and orchestration.**
