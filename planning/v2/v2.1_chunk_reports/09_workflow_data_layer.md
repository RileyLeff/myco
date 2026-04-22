# 09 — Workflow Data Layer

Durable summary of the Python-side / workflow-side data layer. Locks
the principle that Python is a dumb data-provenance layer and never
knows `.myco` types. Enumerates the Python-visible surface, the node
addressing scheme, and the bind / observe / run primitives. Leaves
concrete API details as open work but pins the shape.

## Principle

**Python does not know Myco types.** The Python library is a generic
data-provenance and orchestration layer. It handles data in, data out,
run orchestration, and infrastructure (RNG, wall-clock, checkpointing).
It does not handle type construction, contract implementation, or any
model semantics.

Consequence: spore authors ship one artifact (`.myco` sources +
`myco.toml`). There is no Python mirror package. The Python library's
surface grows along one axis only — data primitives.

## What Python sees

When a `.myco` model compiles, the resulting artifact exposes a **node
catalog**: a structured manifest of every node the workflow can touch.

Each catalog entry carries:

- **Path** — dotted name into the model's entity tree
  (`leaf.water_potential`, `stem.vc.b`, ...).
- **Declared type** — the `.myco` type, expressed as a shape descriptor
  (`Scalar<Pressure>`, `TimeSeries<Scalar<Dimensionless>>`,
  `Tensor<Shape=[n_wavelengths], Scalar<Radiance>>`, ...).
- **Binding role** — initial-condition, parameter, observed,
  configuration-only, etc. (Exact taxonomy open; see below.)
- **Units** — where applicable, the unit and dimension for coercion
  against user-supplied values.

Python does not see:

- Relation bodies or their equational content
- Contract implementations or method dispatch internals
- E-graph structure, rewrite choices, extraction plans
- Any spore-specific types as Python classes

## Node addressing

Nodes are addressed by path. The path resolves into the entity tree
declared in `.myco`, with a few extensions for populations and temporal
indexing.

| Path form | Meaning |
|---|---|
| `"leaf.water_potential"` | Scalar field on each Leaf instance |
| `"leaf.vc.b"` | Field-of-a-field-of-an-instance |
| `"leaf.k@t=30_days"` | Observed value at a specific time |
| `"leaf.k@all"` | All observed values across the run |
| `"species.vuln_curve"` | Field on a shared entity (non-population) |
| `"universals.R"` | Universal binding |

The exact syntax is open. The principle is: paths are stable, explicit,
and deterministic — no reflection, no string-mangling magic, no
implicit broadcasting.

## Primitives

The Python library provides three kinds of verbs. Exact spelling is
open; the shape is pinned.

### Load / compile / spawn

```python
world = myco.load("my_model")           # compile + load a .myco model
pop   = world.spawn("Leaf", n=1000)     # instantiate a population
```

### Bind

Associate a named node with data:

```python
# scalar field, one value per instance:
pop.bind("kmax", values=np.full(1000, 3.0))

# scalar field, sampled from a distribution:
pop.bind("vc.b", sampled_from=myco.lognormal(mu=1.5, sigma=0.3))

# scalar field, factory callable per-instance:
pop.bind("vc.b", factory=lambda: 3.0 + np.random.randn() * 0.2)

# time-series forcing data:
pop.bind("water_potential", series=psi_array, times=time_array)
```

The library type-checks supplied values against the node catalog:
shape, dtype, and (where applicable) units. Mismatches are errors at
bind time, not at run time.

### Observe

Request values back from a completed run:

```python
run = world.run(duration=30_days)

# scalar at final time:
k_final = run.observe("k")              # shape (n_leaves,)

# time series:
k_series = run.observe("k", all_times=True)   # shape (n_leaves, n_t)

# structured: subset of nodes as a dataframe:
df = run.observe_frame(["k", "water_potential", "net_photosynthesis"])
```

Output formats should cover the common scientific-Python shapes:
NumPy arrays, xarray DataArrays, pandas DataFrames, nested dicts.
Exact menu is open.

## Sampling / distribution primitives

The Python library ships a minimal set of distribution and utility
primitives for binding:

```python
myco.uniform(low, high)
myco.lognormal(mu, sigma)
myco.normal(mu, sigma)
myco.fixed(value)
myco.from_csv(path, column)
myco.from_dataframe(df, column)
```

These are **value providers**, not Myco distribution types. They
produce concrete numbers at bind time. They are distinct from the
`.myco` `Distribution<U>` contract, which lives inside the model and
describes equational claims about random variables.

The split is:

- **`.myco` distributions** (`Normal`, `Beta`, `Gamma`, ...) — symbolic
  objects participating in the e-graph; log-pdf and sampling are
  compile-time/extraction-time concerns.
- **Python value providers** — ordinary numerical RNG tools used to
  pick concrete bind-time values.

A user sampling a Python `myco.lognormal(...)` to initialize a field is
just writing "please draw one random number per instance." It is not
declaring a posterior or a prior; that's `.myco`-side.

## Run / control

```python
run = world.run(duration=30_days, dt=0.5_hours)
run = world.run(until=lambda s: s.leaf.water_potential.min() < -5_MPa)
run.checkpoint("day_15.ckpt")
```

RNG control, wall-clock scheduling, checkpointing, restart — all
Python-side. None of these cross into `.myco`; they are workflow
concerns.

## Mode B interaction

The dumb-data principle means **per-instance contract-type selection
cannot be driven from Python alone**. If Mode B is desired — different
leaves in the same population using different VC families — the choice
must be made `.myco`-side.

Workable `.myco`-side patterns (one is enough, not all needed):

- A discriminant field on each leaf with a tagged-union / sum-type VC.
  Python binds the discriminant; `.myco` dispatches per-tag to the
  matching relation.
- A species-level field that commits to a family; leaves inherit via
  composition (each species's population is homogeneous).
- A `.myco` sub-model per VC family, with a dispatcher relation
  selecting based on an integer tag.

The tagged-union / sum-type approach is cleanest but depends on Myco
having sum types. See §35 open for the Mode B mechanism question.

## What this locks

- Python is a data layer, not a model layer. Locked.
- Spore authors ship `.myco` + `myco.toml` only; no Python mirror.
  Locked.
- Node catalog, path-based addressing, bind / observe / run verb
  families. Shape pinned; exact API open.
- Python value providers are distinct from `.myco` distributions.
  Locked.

## Open items

- Exact syntax for node paths (especially temporal indexing).
- Exact typing of the node catalog (how units, shapes, and binding
  roles are expressed in Python).
- The complete menu of observe output formats.
- Mode B mechanism: sum types / tagged unions in `.myco`, or a
  different dispatcher story.
- Whether the node catalog is user-readable (`hypha describe
  my_model`) or only library-consumed.
- Whether bind accepts `.myco`-side expressions, or only Python-
  computed concrete values.
- Streaming / partial-observation APIs for long-running studies.
- Parameter inference / calibration API (posterior draws, MCMC
  integration). This is load-bearing for the research use case but
  depends on the distribution contract shape (chunk-08 deferred item).

## Relationship to the relation/fn lock (chunk 08)

Chunk 08's dumb-data Python principle is the partner to the "ban user
fn, everything equational" rule on the `.myco` side. Together they
establish: **all model semantics lives in `.myco`; Python handles data
and orchestration.** Neither side grows the other's responsibilities.

## Status

Principle locked. Shape of the API pinned. Exact syntax, naming, and
output-format menu open. Needs its own follow-up sweep across §24
(workflow verbs) and §31 (Python API) once the exact syntax lands.
