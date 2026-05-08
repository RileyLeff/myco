# Myco V2 Domain Architecture Amendment

**Status.** REVIEW DRAFT. This document records the current proposed
architecture for explicit domains, temporal domains, correspondences,
observation operators, event families, and realization. It is not yet
canonical. It exists so reviewers can evaluate the design as a coherent
amendment before `planning/v2/spec.md` is rewritten.

**Relationship to the canonical spec.** `planning/v2/spec.md` remains
canonical until this amendment is accepted and merged into it. This
document intentionally describes places where the current spec is likely
wrong, incomplete, or misleading under the proposed architecture.

**Review goal.** Decide whether this domain architecture is Myco-aligned,
future-proof enough for difficult modeling cases, and precise enough to
rewrite the canonical spec against. Reviewers should focus on hidden
defaults, hidden global time, inferred cross-domain alignment, workflow /
source boundary violations, and concepts that collapse distinct
semantics into one surface.

---

## 1. Why This Amendment Exists

The current v2 spec already contains partial domain machinery:

- `Domain<G>` in the geometry/locus chapter.
- `bind_topology` as a workflow-side spatial realization surface.
- event prose that assumes tick-style temporal execution.
- `config.dt`-style workflow cadence.
- `integrate(..., domain)` and mocks that already treat time-like axes
  and spatial axes similarly.
- realization providers, evidence grades, obligation sites, process
  priors, and Layer-3 semantic sites.

The emerging problem is that time, space, index axes, observation clocks,
event streams, and inferred loci are not cleanly represented by the
current spatial-first `Domain<G>` story. If Myco keeps an implicit global
time or treats topology, time, and observation alignment as separate
special cases, later support for SSA, partial-order time, calendars,
moving meshes, pseudotime, coalescent trees, event cameras, and
distributed traces will require incompatible retrofits.

This amendment proposes a more general architecture:

```text
Domains locate.
Temporal domains order and provide snapshot semantics.
Spatial domains provide geometry, measure, and operator support.
Event families define occurrence laws.
Correspondences align loci.
Observation operators measure.
Realizations execute, provide, sample, infer, or learn these structures
with evidence.
```

The goal is not to add magic. The goal is to expose the structure that
scientific models already depend on.

---

## 2. Hard Invariants

These are the design commitments that appear stable across review:

1. **Domain identity is explicit and nominal.**
   Importing a constructor does not create a domain identity. A `domain`
   declaration does.

2. **No implicit global time.**
   A model with temporal structure must name a temporal domain, either
   through a concrete domain declaration or through a generic domain
   parameter. There is no sticky default clock.

3. **Reusable code is generic over caller-owned loci.**
   A library declares a concrete domain only when that domain identity is
   part of its scientific claim. Reusable model components should usually
   accept domains through generic parameters or context contracts.

4. **Domains participate in ordinary contracts and generics.**
   There is no separate `domainreq` system. Domain capability
   requirements are ordinary contracts. Domain bundles are contracts with
   associated domains, associated compositions, and associated
   correspondences.

5. **Cross-domain correspondence is explicit.**
   A cross-domain read names a correspondence or occurs inside an
   explicit lexical correspondence scope. The compiler does not infer
   "the only coupling in scope."

6. **Composition is distinct from correspondence.**
   `Space x Time` is a domain composition, not a coupling. A
   correspondence aligns different loci. Product, bundle, quotient,
   graph-indexed, tree-indexed, and spacetime-style compositions have
   their own constructors and emitted maps.

7. **Support domains and evolution domains are distinct.**
   A field's instantaneous support is where its current value lives. A
   temporal block declares the domain over which its trajectory evolves.
   The compiler elaborates this to an explicit trajectory/lift site.

8. **Temporal domains do not own event occurrence semantics.**
   Temporal domains provide order, duration, causality, frontier, branch,
   and snapshot semantics. Event families define occurrence laws.

9. **Observation operators are source-visible measurement models.**
   Workflow supplies data to declared observation operators. Python does
   not invent measurement semantics at `observe(...)` time.

10. **Realization is staged and run-locked.**
    Source declares obligations. Workflow binds realization sources.
    Providers emit realization instances and evidence. Run records lock
    provider identity, seeds, traces, posterior policies, validation, and
    fallbacks.

11. **No hidden domain generation.**
    Per-entity surfaces, local clocks, per-site calendars, and
    event-created loci require declared domain families or equivalent
    family sites.

12. **Uncertain domain use is explicit.**
    If a domain realization is posterior, sampled, or learned, a
    deterministic downstream use must declare how that uncertainty is
    consumed: sample, MAP, marginalize, propagate posterior, or another
    named policy.

---

## 3. Core Ontology

### 3.1 Domain

A `Domain` is a declared nominal locus over which quantities, events,
observations, or structures can be located.

A domain is not:

- a unit;
- a value type;
- a plain record;
- a relation;
- a solver;
- a provider;
- a dataset;
- an arbitrary tensor shape;
- a stochastic variable by itself.

A domain answers questions like:

- where does this field live?
- when does this state evolve?
- over which entity locus is this value indexed?
- along which event occurrence locus is this effect recorded?
- in which observation coordinate system was this data measured?

Minimum conceptual payload:

```text
DomainSite
  id
  source provenance
  constructor or structure
  locator type
  native facts and capabilities
  realization requirement
  domain family membership, if any
  composition parents and emitted maps, if any
```

### 3.2 Domain Constructors

Domain constructors describe reusable structure. They do not create
domain identity by themselves.

Example:

```myco
use std::time::ContinuousTime
use std::space::Surface

domain Clock: ContinuousTime<Second> as t
domain LeafSurface: Surface<2, Meter> as (u, v)
```

`ContinuousTime<Second>` and `Surface<2, Meter>` are constructors.
`Clock` and `LeafSurface` are nominal identities.

Two declarations using the same constructor are distinct domains unless
the source explicitly identifies them.

```myco
domain ExperimentA: ContinuousTime<Second> as t_a
domain ExperimentB: ContinuousTime<Second> as t_b
```

`ExperimentA` and `ExperimentB` are not the same domain.

### 3.3 Spatial Domains

Spatial domains are domains with geometry, adjacency, measure, topology,
coordinate charts, or spatial operators.

The old spatial-only syntax:

```myco
type StemAxis : Domain<G = Interval> as (z: Scalar<dimensionless>)
```

should be subsumed by first-class domain declarations:

```myco
domain StemAxis: std::space::Interval<dimensionless> as z
```

This does not mean every spatial domain is cartesian or charted by one
coordinate tuple. General manifolds, atlases, graph domains, fiber
domains, quotient domains, and provider-backed loci need constructor
specific syntax and capabilities.

### 3.4 Temporal Domains

Temporal domains are domains with order, duration, causality, event
position, branch, or snapshot semantics.

Temporal domains do not choose algorithms. They authorize operations
through contracts:

- derivative;
- step;
- delay;
- event guard;
- hazard or propensity;
- frontier snapshot;
- branch-local snapshot;
- scheduled queue;
- calendar successor.

Examples:

```myco
domain Clock: std::time::ContinuousTime<Second> as t
domain ControllerTick: std::time::DiscreteTick<Millisecond> as k
domain CivilDate: std::time::CalendarTime<UTC> as date
domain CausalTrace: std::time::PartialOrder<MessageId> as event
```

The source may define event families or temporal dynamics over these
domains. The workflow binds realizations such as fixed tick arrays,
adaptive ODE providers, replayed traces, calendar providers, SSA event
realizers, or posterior time embeddings.

### 3.5 Domain Families

Some domains are not single global loci. They are families indexed by
entities or other domains:

- one surface per leaf;
- one local clock per sensor;
- one membrane per cell;
- one site-specific civil-time alignment per field site;
- one dynamically generated branch domain per event-created lineage.

Hidden generation is not allowed. These require declared domain family
sites.

Current preferred shape:

```myco
domain Leaves: std::entity::EntityDomain<LeafId>

domain family LeafSurface over Leaves:
    std::space::Surface<2, Meter> as (u, v)
```

The total-space form is often better for fields over all family members:

```myco
domain AllLeafSurfaces:
    std::domain::FiberBundle<Leaves, fiber = Surface<2, Meter>>
    as (leaf, u, v)
```

Conceptual payload:

```text
DomainFamilySite
  family id
  index domain
  instance constructor
  instance identity rule
  realization mode
  emitted total space, if any
  emitted instance projection, if any
```

Open details are listed in Section 14.

### 3.6 Domain Composition

Domain composition builds new loci from existing loci. It is distinct
from correspondence.

Examples:

```myco
domain LeafSpaceTime:
    std::domain::Product<LeafSurface, Clock>
```

```myco
domain Vasculature:
    std::domain::FiberBundle<BranchGraph, BranchAxis>
    as (edge, z)
```

Composition constructors declare the canonical maps they emit. Products
emit projections. Bundles emit base and fiber maps. Quotients emit
quotient maps. Spacetime-style constructors may emit frame-specific
maps only if the constructor guarantees them.

There is no universal assumption that every composition has cartesian
projections.

### 3.7 Correspondence

A correspondence aligns loci. The source keyword may be `couple`, but
the internal concept is a `CorrespondenceSite`.

Correspondences are not arbitrary scientific relations. They answer
indexing and transport questions:

- how to read a quantity from one locus at another locus;
- how to pull back scalar fields;
- how to push forward measures;
- how to transport vectors or fluxes;
- how to synchronize snapshots;
- how to align observation domains with model domains.

Preferred block form:

```myco
couple Deformation {
    witness MaterialAtTime:
        std::domain::Product<MaterialSolid, Clock>

    map physical_position:
        MaterialAtTime -> PhysicalSolid

    supports pullback scalar_field
        from PhysicalSolid
        to MaterialAtTime

    supports pullback density_field
        from PhysicalSolid
        to MaterialAtTime
        requires Jacobian(physical_position)

    supports pushforward flux_field
        from MaterialAtTime
        to PhysicalSolid
        requires OrientationConsistent
}
```

The loose form:

```myco
couple A between B, C by ...
```

is useful for conversation but should probably not be canonical. It
hides witness structure and tends to make correspondences look like
single functions.

### 3.8 Coupling Families

Correspondences can also be families:

- one deformation map per leaf;
- one local-to-UTC correction per site;
- one body-to-world frame per robot;
- one sensor footprint per satellite;
- one learned alignment per subject.

Preferred shape:

```myco
couple family LeafDeformation over Leaves {
    witness MaterialAtTime:
        std::domain::Product<LeafSurface[leaf], Clock>

    map physical_position:
        MaterialAtTime -> PhysicalLeaf[leaf]
}
```

Exact syntax for indexing family domains is open. The important
architectural rule is stable: coupling family instances are nominal and
explicit, not generated invisibly.

### 3.9 Observation Operator

An observation operator is a source-visible measurement model. It is not
a correspondence. It can use correspondences, aggregation, noise,
missingness, exposure windows, censoring, calibration, and unit
conversion.

Example sketch:

```myco
observation_operator ThermalCameraToLeafSurface {
    data_locus:
        std::domain::Product<CameraPixels, ExposureIntervals>

    target:
        leaf.temperature over std::domain::Product<LeafSurface, ModelTime>

    uses correspondence PixelFootprint
    uses correspondence ExposureToModelTime

    measure {
        radiance[pixel, exposure] =
            integrate(
                blackbody(leaf.temperature),
                over = footprint(pixel) x exposure_window(exposure),
            )
    }

    noise:
        ThermalSensorNoise
}
```

Workflow then supplies data to the declared operator:

```python
workflow.observe(
    cat.observation_operator("ThermalCameraToLeafSurface"),
    data=image,
    over={
        "pixel": pixels @ cat.domain("CameraPixels"),
        "exposure": intervals @ cat.domain("ExposureIntervals"),
    },
)
```

Trivial path observations can have stdlib sugar, but the measurement
semantics must remain inspectable.

### 3.10 Event Family

Event families define occurrence laws over a temporal domain. They are
not the temporal domain itself.

Example:

```myco
domain T: std::time::ContinuousTime<Second> as t

event family Reaction over T {
    propensity: k * substrate

    effect {
        substrate -= one_mole
        product += one_mole
    }
}
```

Conceptual elaboration:

```text
EventFamilySite
  id
  temporal domain
  occurrence domain
  occurrence-time correspondence
  trigger / guard / hazard / propensity / schedule law
  participants and payload
  matching constraints
  conflict relation
  stale policy
  cascade semantics
  effects
  conservation obligations
  realization requirement
```

The workflow realizes the event family:

```python
workflow.bind(
    cat.event_family("Reaction"),
    myco.events.SSA(seed=42),
)
```

This replaces the earlier, weaker idea that `ReactionTime` itself is
bound to SSA. SSA realizes event occurrences, not time as such.

### 3.11 Occurrence Domains

Every event family elaborates to an occurrence domain or occurrence
stream. This needs to be first-class in the catalog because event traces
can be:

- simulated;
- replayed from data;
- inferred as latent histories;
- observed and fitted;
- queried by downstream relations.

Possible source forms:

```myco
event family Spike over NeuralClock {
    participants neuron: Neuron
    hazard: firing_rate(neuron)
    effect { ... }
}
```

or for supplied occurrence domains:

```myco
domain Spikes: std::event::OccurrenceDomain<NeuronId>

event family Spike over NeuralClock
    occurrences Spikes
{
    effect { ... }
}
```

Exact syntax remains open.

### 3.12 Realization

A realization is how a source-declared domain, correspondence,
observation operator, event family, or semantic site becomes concrete
for a run.

Realizations are workflow source objects, parallel to `Constant`,
`Series`, `Trainable`, `Prior`, `ProcessPrior`, and `Controller`.

They should satisfy ordinary contracts, such as:

```myco
contract Realization<D> { ... }
contract EventFamilyRealization<E> { ... }
contract CorrespondenceRealization<C> { ... }
```

Workflow binding should use the existing `bind` pattern:

```python
workflow.bind(
    cat.domain("Clock"),
    myco.realization.time.FixedTick(dt=0.01),
)

workflow.bind(
    cat.event_family("Reaction"),
    myco.events.SSA(seed=42),
)

workflow.bind(
    cat.correspondence("ObsToModelTime"),
    calibration_alignment,
)
```

Avoid introducing a separate `bind_domain` verb unless the existing
`bind` API cannot support typed catalog handles cleanly.

Staged realization:

```text
DomainProviderSpec
  provider identity
  target site kinds
  advertised capability contracts
  validation obligations
  run-lock schema

RealizationInstance
  actual bound, sampled, inferred, or learned object for this run
  operations supported by this instance
  materialization mode
  version and provenance references

RealizationEvidence
  obtained_by: supplied | sampled | inferred | learned | derived
  realized_object_kind: mesh | calendar | event_stream | trace |
                        posterior | coordinate_map | oracle | graph |
                        tree | finite_structure
  evidence grade
  uncertainty form
  validation records
  invalidation dependencies
```

---

## 4. Contracts, Generics, and Domain Contexts

### 4.1 Domains Satisfy Ordinary Contracts

There should be no separate "domain requirement" language. Domain
capabilities are ordinary contracts.

Sketch:

```myco
contract ODETime<U> : TemporalDomain {
    requires TotalOrder<Self>
    requires DurationMeasure<Self, U>
    requires SupportsDerivative<Self>
}

contract DiffusionSpace : SpatialDomain {
    requires HasMetric<Self>
    requires HasMeasure<Self>
    requires SupportsLaplacian<Self>
}
```

Then reusable models use the normal generic surface:

```myco
type Diffusion<S: DiffusionSpace, T: ODETime<Second>> {
    concentration: Field<Concentration> over S
    diffusivity: Diffusivity

    temporal over T {
        d concentration =
            diffusivity * laplacian(concentration, over = S)
    }
}
```

### 4.2 Domain Contexts Are Contracts With Associated Members

Complex reusable models should not take dozens of generic parameters.
Domain contexts bundle related domains, compositions, and
correspondences using contracts with associated members.

Sketch:

```myco
contract LeafWorld {
    associated domain Space: DiffusionSpace
    associated domain Time: ODETime<Second>

    associated domain Trajectory:
        std::domain::Product<Space, Time>
}

type LeafModel<W: LeafWorld> {
    chlorophyll: Field<Concentration> over W::Space

    temporal over W::Time {
        d chlorophyll =
            diffusion(chlorophyll, over = W::Space)
    }
}
```

Associated domains and correspondences are a real extension of the
current contract system. They are not a new domain-specific requirement
language, but the spec must explicitly define:

- associated domain members;
- associated composition members;
- associated correspondence members;
- satisfaction checking;
- coherence;
- diagnostics;
- how `hypha explain` reports associated members.

### 4.3 No Implicit Equality Between Associated Domains

Associated domains are nominal. Same constructor and same bounds do not
imply same identity.

This is invalid without an explicit equality or correspondence:

```myco
type Model<P: PlantWorld, S: SensorWorld> {
    // P::Time and S::Time are not automatically the same domain.
}
```

The model must require identity or correspondence explicitly:

```myco
type Model<P: PlantWorld, S: SensorWorld>
where SameDomain<P::Time, S::Time>
{
    ...
}
```

or:

```myco
contract SensedPlantWorld {
    associated domain PlantTime: ODETime<Second>
    associated domain SensorTime: TemporalDomain

    associated correspondence SensorToPlantTime:
        SupportsRead<SensorTime, PlantTime>
}
```

`SameDomain` and `identify domain` need careful design.

---

## 5. Source and Workflow Boundary

The proposed split:

`.myco` source declares:

- nominal domain identities;
- domain family declarations;
- domain compositions;
- domain capability requirements;
- state support domains;
- temporal/evolution domains;
- event-family occurrence laws;
- correspondences and supported operations;
- observation operators;
- coupling and realization obligations;
- constraints and scientific semantics.

Workflow supplies:

- concrete values;
- initial conditions;
- data;
- observations and fit targets;
- domain realizations;
- correspondence realizations;
- event-family realizations;
- observation data bindings;
- inference recipes;
- backend/provider choices;
- seeds, tolerances, fallback, and run-lock metadata.

Workflow must not create source semantics. Source must not bind run
values or choose execution algorithms.

---

## 6. Temporal Syntax

### 6.1 No Naked Global `d`

`d(x)` is invalid outside an explicit temporal context or explicit
domain argument.

Preferred concise form:

```myco
temporal over Clock {
    d biomass = assimilation - respiration
}
```

Explicit form:

```myco
d(biomass, over = Clock) = assimilation - respiration
```

`step` follows the same rule for discrete-step temporal domains.

### 6.2 Support Domain vs Evolution Domain

A state declaration names instantaneous support:

```myco
concentration: Field<Concentration> over LeafSurface
```

A temporal block names evolution:

```myco
temporal over Clock {
    d concentration =
        diffusivity * laplacian(concentration, over = LeafSurface)
}
```

This does not silently rewrite the source declaration to
`LeafSurface x Clock`. Instead, the compiler emits a visible
`TemporalLiftSite`.

Conceptual payload:

```text
TemporalLiftSite
  state path
  instantaneous support domain
  evolution domain
  trajectory locus
  emitted projections
  initial-boundary obligations
  observation/fitting facets
  conservation locus implications
```

The catalog should expose both instantaneous and trajectory facets so
workflow observations can target the intended object.

### 6.3 Temporal Context Scope

Current recommendation:

- `temporal over T` provides the default temporal domain for `d` and
  `step` within the block.
- event-family declarations inside the block may inherit `T`, but this
  needs to be locked explicitly.
- `integrate`, `observe`, `fit_to`, cross-domain reads, and
  correspondence operations should still name their domains or
  correspondences explicitly unless a separate lexical scope does so.

This is an open syntax rule, not yet canonical.

### 6.4 Partial and Branching Time

`TemporalDomain` alone authorizes nothing. Operations require capability
contracts.

Examples:

```myco
contract ODETime<U> : TemporalDomain {
    requires TotalOrder<Self>
    requires DurationMeasure<Self, U>
    requires SupportsDerivative<Self>
}

contract CausalTime : TemporalDomain {
    requires PartialOrder<Self>
    requires SupportsFrontierSnapshot<Self>
}

contract BranchingTime : TemporalDomain {
    requires BranchOrder<Self>
    requires SupportsBranchLocalSnapshot<Self>
}
```

Reads in partial-order time require causal-frontier semantics:

```myco
let visible_state =
    state @ DistTime frontier msg.causal_frontier
```

Reads in branching time require a branch or an explicit cross-branch
correspondence:

```myco
let baseline =
    outcome @ BranchingTime branch control_branch
```

or:

```myco
let paired =
    outcome @ BranchingTime via CounterfactualPairing
```

Exact syntax remains open.

---

## 7. Correspondence Semantics

### 7.1 Cross-Domain Reads

Same-domain reads need no correspondence. Cross-domain reads require:

- a named correspondence;
- a named supported operation when the value kind is not safely
  inferred; or
- an explicit lexical correspondence scope.

Simple scalar read:

```myco
temperature @ ModelTime via SensorToModelTime
```

Explicit operation for non-scalar transport:

```myco
rho_material =
    rho_physical via Deformation.pullback_density

velocity_body =
    velocity_world via BodyFrame.pullback_vector

mass_material =
    mass_physical via Deformation.pullback_measure
```

The compiler should not infer cross-domain correspondence merely because
one coupling is currently in scope.

### 7.2 Supported Operations Are Value-Kind Aware

Transporting a scalar, density, measure, vector, tensor, flux,
distribution, event stream, or conserved quantity is not the same
operation.

Correspondence supported operations should record:

```text
SupportedOperation
  name
  source domain
  target domain
  value kind
  variance/frame information, if relevant
  measure semantics, if relevant
  snapshot semantics
  totality: total | partial | conditional | stochastic | posterior
  exactness: exact | approximate | sampled | posterior
  required facts
  emitted facts
  evidence requirement
```

### 7.3 Correspondence Obligations

A correspondence declaration emits obligation sites for its declared
maps and supported operations.

Current recommendation:

- one `ObligationSite` per declared map slot;
- default cardinality `exactly_one`;
- fulfillment relations use `fulfills Correspondence.map_name`;
- coupled SCCs form normally when fulfillment relations share variables
  with model physics.

Example:

```myco
couple LeafDeformation {
    witness MaterialAtTime:
        std::domain::Product<MaterialLeaf, Clock>

    map physical_position:
        MaterialAtTime -> PhysicalLeaf
}

type Leaf {
    displacement: Vector<m> over MaterialLeaf

    temporal over Clock {
        relation deformation_law
            fulfills LeafDeformation.physical_position
        {
            physical_position(x) = x + displacement(x)
        }
    }
}
```

This example is illustrative. It still needs final syntax for
instance-level/family correspondences.

### 7.4 State-Dependent Correspondences

State-dependent correspondences are valid. They are not trusted static
metadata and they are not rejected categorically.

Examples:

- deformation map depending on displacement;
- sensor footprint depending on satellite position;
- body/world frame depending on robot pose;
- m/z calibration depending on instrument drift;
- clock alignment depending on learned latency.

If correspondence maps depend on state that is solved together with the
physics using the map, the compiler forms a coupled SCC and requires a
realization/solver/evidence path.

---

## 8. Event Families

### 8.1 Event Family Source Semantics

An event family can declare:

- home temporal domain;
- trigger or guard;
- hazard or propensity;
- participants;
- occurrence payload;
- matching/resource constraints;
- conflict relation;
- stale policy;
- delay/scheduling law;
- cascade semantics;
- effects;
- conservation fulfillments.

Example:

```myco
event family Bind over T {
    propensity: k_bind * substrate * enzyme

    effect {
        substrate -= one
        complex += one
    }

    schedules Release after delay ~ ReleaseDelay {
        stale: CancelIfComplexGone
    }
}

event family Release over T {
    effect {
        complex -= one
        product += one
    }
}
```

### 8.2 Event Realization

Workflow realizes event occurrences:

```python
workflow.bind(
    cat.event_family("Bind"),
    myco.events.SSA(seed=42),
)

workflow.bind(
    cat.event_family("Release"),
    myco.events.DelayedQueue(),
)
```

Different realizations can produce different evidence grades:

- exact SSA trace;
- replayed trace;
- posterior event history;
- tau-leap approximation;
- priority queue simulation;
- adaptive root localization.

Approximate event realizations must enter the approximation ledger.

### 8.3 Matching Is Source Semantics

Matching/resource rules are often world claims, not scheduler tricks.

Examples:

- price-time priority in an order book;
- bee/flower matching in pollination;
- molecule collision pairing;
- bed assignment in hospital models;
- right-of-way in traffic.

Source-level event families should be able to declare matching and
resource constraints. Workflow may choose a realization algorithm that
satisfies them.

### 8.4 Event Occurrence Catalog Objects

Event-family realizations produce occurrence streams. These need catalog
handles so they can be:

- observed;
- replayed;
- queried;
- used downstream;
- compared to data;
- recorded in run lock.

This is not yet fully specified.

---

## 9. Observation Operators

### 9.1 Observation Is Not Direct Equality

Observation does not equate data with model paths. It attaches data to a
declared measurement model.

Trivial observations can elaborate to stdlib identity-style operators,
but the general architecture should be:

```text
source: declares observation operator
workflow: supplies data and coordinates to that operator
compiler: builds likelihood/evidence/residual terms
```

### 9.2 Observation Operators vs Correspondences

Correspondences align loci. Observation operators measure.

An observation operator can use several correspondences and add:

- aggregation;
- exposure intervals;
- noise model;
- censoring;
- missingness;
- calibration;
- unit conversion;
- likelihood structure.

This is essential for:

- cameras;
- satellite products;
- field censuses;
- lab assays;
- single-cell sequencing;
- event cameras;
- PMU sensors;
- market feeds;
- epidemiological reports.

### 9.3 Workflow Shape

Preferred general workflow shape:

```python
workflow.observe(
    cat.observation_operator("CaseReports"),
    data=case_data,
    over={
        "date": report_dates @ cat.domain("ReportCalendar"),
        "jurisdiction": areas @ cat.domain("Jurisdiction"),
    },
)
```

Trivial path sugar can exist only if it elaborates to a source-visible
observation site or stdlib observation operator.

---

## 10. Workflow Axes vs Source Domains

A source domain is a locus over which modeled quantities, events, or
observations live.

A workflow axis indexes repeated uses of the model.

Source domains can include:

- model time;
- physical space;
- observation clocks;
- camera pixels;
- field plots;
- cohorts;
- sampling frames;
- sensor frames;
- event occurrence streams.

Workflow axes include:

- Monte Carlo replicate;
- parameter sweep;
- scenario batch;
- cross-validation fold;
- train/test split;
- backend comparison run.

Workflow axes should be represented in the catalog/workflow layer, not
smuggled into `.myco` source laws. Source laws may be lifted over
workflow axes by workflow composition, but they should not depend on
workflow axes unless those axes are promoted to declared source domains
with explicit semantics.

This area needs a catalog mechanism parallel to domains:

```text
WorkflowAxis
  id
  role
  coordinate set
  binding/provenance
  lifting semantics
```

---

## 11. Identity, Refactoring, and `identify domain`

Nominal identity is valuable but creates a refactoring footgun:

- a model originally declares local `domain Time`;
- later `Time` is moved to a shared spore;
- one file keeps the old local declaration;
- now two structurally identical clocks exist.

The compiler should not silently merge them.

Possible tools:

1. `hypha check --domain-audit` reports structurally similar nominal
   domains across a workspace.
2. `identify domain A = B` explicitly declares that two nominal domains
   are the same locus.

`identify domain` would need to be designed carefully because it is an
identity merge at the domain level. It should preserve provenance and
avoid automatic structural merging.

This is currently open.

---

## 12. Examples

### 12.1 Minimal Explicit ODE

```myco
use std::time::ContinuousTime

domain Clock: ContinuousTime<Second> as t

type Decay {
    x: Scalar<dimensionless>
    k: Scalar<1 / s>

    temporal over Clock {
        d x = -k * x
    }
}
```

No implicit time exists. `Clock` is explicit, but the equation remains
short.

### 12.2 Reusable Diffusion

```myco
contract ODETime<U> : TemporalDomain {
    requires TotalOrder<Self>
    requires DurationMeasure<Self, U>
    requires SupportsDerivative<Self>
}

contract DiffusionSpace : SpatialDomain {
    requires HasMetric<Self>
    requires HasMeasure<Self>
    requires SupportsLaplacian<Self>
}

type Diffusion<S: DiffusionSpace, T: ODETime<Second>> {
    concentration: Field<Concentration> over S
    diffusivity: Diffusivity

    temporal over T {
        d concentration =
            diffusivity * laplacian(concentration, over = S)
    }
}
```

### 12.3 Leaf World Context

```myco
contract LeafWorld {
    associated domain Space: DiffusionSpace
    associated domain Time: ODETime<Second>
    associated domain Trajectory:
        std::domain::Product<Space, Time>
}

type LeafModel<W: LeafWorld> {
    chlorophyll: Field<Concentration> over W::Space

    temporal over W::Time {
        d chlorophyll =
            diffusion(chlorophyll, over = W::Space)
    }
}
```

### 12.4 Per-Leaf Surfaces

```myco
domain Leaves: std::entity::EntityDomain<LeafId>

domain AllLeafSurfaces:
    std::domain::FiberBundle<
        base = Leaves,
        fiber = std::space::Surface<2, Meter>,
    > as (leaf, u, v)

type Canopy {
    chlorophyll: Field<Concentration> over AllLeafSurfaces
}
```

This avoids hidden per-leaf domain generation.

### 12.5 Observation Clock

```myco
domain ModelTime: std::time::ContinuousTime<Second> as t
domain ObsClock: std::time::CalendarTime<UTC> as date

couple ObsToModelTime {
    witness ObsInstant
    map model_time: ObsInstant -> ModelTime
    map obs_date: ObsInstant -> ObsClock

    supports read scalar_field
        from ModelTime
        to ObsClock
}

observation_operator BiomassCensus {
    data_locus: ObsClock
    target: plant.biomass over ModelTime
    uses correspondence ObsToModelTime
    noise: Gaussian<BiomassError>
}
```

### 12.6 Chemical Event Family

```myco
domain T: std::time::ContinuousTime<Second> as t

event family Reaction over T {
    propensity: k * substrate

    effect {
        substrate -= one_mole
        product += one_mole
    }
}
```

Workflow:

```python
workflow.bind(
    cat.event_family("Reaction"),
    myco.events.SSA(seed=42),
)
```

### 12.7 Moving Mesh Correspondence

```myco
domain Material: std::space::ReferenceBody<Meter> as X
domain Physical: std::space::Euclidean<3, Meter> as x
domain Time: std::time::ContinuousTime<Second> as t

domain MaterialAtTime:
    std::domain::Product<Material, Time>

couple Deformation {
    witness MaterialAtTime

    map physical_position:
        MaterialAtTime -> Physical

    supports pullback scalar_field
        from Physical
        to MaterialAtTime

    supports pullback density_field
        from Physical
        to MaterialAtTime
        requires Jacobian(physical_position)
}
```

Fulfillment can depend on state and enter a CouplingSCC.

---

## 13. Spec Impact Map

If accepted, this amendment likely requires changes to:

- **Glossary.** Add DomainSite, DomainFamilySite, DomainCompositionSite,
  TemporalLiftSite, CorrespondenceSite, ObservationOperatorSite,
  EventFamilySite, OccurrenceDomainSite, RealizationInstance,
  RealizationEvidence, WorkflowAxis.

- **Types and contracts.** Domains as generic parameters; associated
  domains/compositions/correspondences in contracts; coherence rules.

- **Geometry and locus.** Reframe old `Domain<G>` as spatial-domain
  constructor/declaration syntax.

- **State and time.** Remove implicit global time; rewrite `d`, `step`,
  `config.dt`, temporal blocks, initial conditions, and temporal
  realization.

- **Events.** Replace tick-centered firing order with event families,
  occurrence domains, event realization, matching, delayed events, and
  cascade semantics.

- **Collections and axes.** Clarify source domains vs workflow axes and
  domain families over entity domains.

- **Stochastic machinery.** Connect latent/posterior domain
  realizations to inference and posterior-use policies.

- **Integration/operators.** Domain measure, composition, projection,
  pushforward/pullback, and operator support contracts.

- **Layered e-graph.** Add Layer-3 sites for domains, domain
  realizations, correspondences, observation operators, event families,
  occurrence domains, and temporal lifts.

- **SCCs and residual extraction.** CouplingSCCs, TemporalLiftSites,
  domain realization dependencies, and observation-operator residuals.

- **Workflow.** Bind domain/event/correspondence/observation realization
  via typed catalog handles; preserve `bind` if possible.

- **Backend capabilities.** Temporal/domain/correspondence/event/
  observation capabilities.

- **Realization providers.** Generalize provider machinery beyond
  spatial/discrete operators.

- **Anti-spec.** Add rows listed below.

- **Mocks.** Rewrite mocks to use explicit domains, temporal blocks, and
  observation/event/correspondence surfaces where relevant.

---

## 14. Open Design Questions

These are not settled and should be the focus of the next review round.

### 14.1 Associated Members in Contracts

Questions:

- exact syntax for `associated domain`;
- exact syntax for `associated correspondence`;
- whether associated compositions are domains, members, or aliases;
- satisfaction checking;
- coherence rules;
- diagnostics;
- how generic bounds over associated domains compose.

### 14.2 Domain Families

Questions:

- instantiation timing;
- identity across runs;
- event-created domain family instances;
- relation to entity identity;
- relation to dynamic collections;
- whether total-space bundle form should be preferred over
  `Family[id]` syntax;
- how `hypha explain` displays family identities.

### 14.3 Coupling Families

Questions:

- whether syntax parallels domain families exactly;
- how instance-level state fulfills family maps;
- identity and run-lock behavior;
- how family correspondences compose with domain family total spaces.

### 14.4 Correspondence Syntax

Questions:

- final canonical `couple` block shape;
- whether `couple` or `correspondence` is the source keyword;
- how to spell witness domains;
- how to spell maps and supported operations;
- how to express partial, stochastic, posterior, many-to-many, and
  interval correspondences;
- how to reference operation-specific map names at use sites.

### 14.5 Coupling Obligations and SCCs

Questions:

- exact obligation keys;
- cardinality of map/support fulfillments;
- partial fulfillment;
- coupled-SCC classification;
- interaction with objective terms;
- validation and evidence grades for correspondence properties.

### 14.6 TemporalLiftSite

Questions:

- exact catalog facets for instantaneous vs trajectory views;
- how observations target lifted trajectories;
- how initial and boundary conditions attach;
- whether two temporal blocks can evolve the same state over different
  domains;
- conservation locus rules for lifted states.

### 14.7 Temporal Scope

Questions:

- does `temporal over T` apply only to `d` and `step`;
- can event-family declarations inherit the temporal domain inside the
  block;
- should `integrate` ever inherit the block domain;
- is there a general `using` lexical mechanism for domains,
  correspondences, and temporal contexts.

### 14.8 Partial and Branching Time

Questions:

- exact syntax for causal frontier reads;
- exact syntax for branch-local reads;
- which temporal operations are valid over partial-order domains;
- whether branching time is a domain, event occurrence structure,
  process-prior structure, or all three in different contexts;
- posterior-use policy for uncertain branching structures.

### 14.9 Event Occurrence Domains

Questions:

- source syntax for occurrence domains;
- how event payloads are typed;
- how replayed traces bind;
- how observed event streams attach;
- how delayed queues and stale policies are represented;
- how event occurrence domains interact with event-created entities.

### 14.10 Observation Operators

Questions:

- exact declaration syntax;
- contract shape;
- how noise and missingness are declared;
- whether trivial path observations are stdlib operators or sugar;
- how observation operators reference multiple correspondences;
- how data coordinates are checked.

### 14.11 Realization Contracts

Questions:

- canonical `Realization<D>` contract;
- realization contracts for correspondences, event families, and
  observation operators;
- provider spec vs instance vs evidence surfaces;
- exact run-lock schema;
- posterior-use policies;
- relationship to `ProcessPrior<I, V>`.

### 14.12 Workflow Axes

Questions:

- catalog representation for replicate/sweep/scenario axes;
- lifting source laws over workflow axes;
- whether workflow axes can ever be promoted into source domains;
- diagnostics when workflow axes leak into source laws.

### 14.13 `identify domain`

Questions:

- whether explicit domain identity merge is allowed;
- how it relates to existing `identify`;
- whether it emits Layer-1 domain identity facts;
- how it appears in run-lock and provenance;
- tooling for structural-but-not-nominal duplicate domains.

### 14.14 Stdlib Capability List

Questions:

- initial stdlib domain capability contracts;
- initial correspondence capability contracts;
- which properties need evidence grades;
- how providers advertise capabilities;
- how user spores define new capabilities safely.

---

## 15. Anti-Spec Rows To Add Or Revise

Likely additions:

| Retired or forbidden | Replacement | Why |
| --- | --- | --- |
| Implicit global time | Explicit temporal domains and temporal contexts | Hidden clocks make reusable code unsafe |
| Sticky default clock | Stdlib constructors plus explicit `domain` declarations | Constructors are not identities |
| `d(x)` outside temporal context | `temporal over T` or `d(x, over = T)` | Derivative requires a named evolution domain |
| Domain identity from import | `domain Name: Constructor<...>` | Imports should not create or merge identities |
| Inferred "only coupling in scope" | Named correspondence or explicit lexical scope | Imports should not silently change alignment |
| Automatic transitive coupling | Explicit composed correspondence declaration | Alignment paths carry semantics and evidence |
| One coupling per domain pair | Multiple named correspondences | Raw, corrected, lagged, fiscal, and posterior alignments can coexist |
| Generic `read via coupling` for all values | Value-kind-specific supported operations | Scalars, densities, vectors, fluxes, and measures transport differently |
| Static trust-me coupling properties | Evidence-bearing CorrespondenceSite facts | Provider metadata is not proof |
| `ReactionTime` means SSA | Event family realization | SSA realizes occurrences, not time itself |
| Event scheduling as global run config | Event-family semantics plus temporal/event realization | Source laws and execution policy must stay separate |
| Observation as direct path equality | Source-visible observation operators | Measurement has semantics |
| Python-declared observation semantics | Workflow data bound to declared observation operators | Python should not declare model structure |
| Hidden per-entity domains | Domain families or total-space bundles | Runtime loci must be declared |
| Hidden per-entity couplings | Coupling families | Instance-specific correspondences need identities |
| Separate `domainreq` language | Ordinary contracts and associated members | Avoid parallel type system |
| Workflow axes as source domains by default | Catalog workflow axes; explicit source domains only when modeled | Replicates and sweeps are not world mechanisms |
| Silent deterministic use of posterior domains | Explicit posterior-use policy | Uncertain loci need explicit consumption semantics |

Existing rows to revise:

- old `Domain<G>`/geometry language;
- `bind_topology` as a spatial realization surface;
- temporal `[t]`/`[t+1]` retirement rows;
- observation/fit rows;
- provider-trust rows;
- hidden approximation rows;
- stochastic coupling terminology if it conflicts with domain
  correspondence.

---

## 16. Review Questions

External reviewers should answer:

1. Does this architecture preserve the Myco source/workflow split?
2. Does it avoid hidden global time without making simple models
   unbearably verbose?
3. Are domains, compositions, correspondences, observation operators,
   event families, and realizations separated correctly?
4. Does the contract/associated-member approach reuse Myco's existing
   abstractions cleanly, or does it create hidden complexity?
5. Are domain families and coupling families the right answer for
   per-entity loci?
6. Is `TemporalLiftSite` the right way to preserve natural PDE/ODE
   syntax without hidden spatiotemporal arity promotion?
7. Is the event-family/occurrence-domain split sufficient for SSA,
   replayed traces, event cameras, markets, and delayed queues?
8. Are observation operators properly source-visible, or should some
   measurement models be workflow-only?
9. How should posterior or learned domain realizations be used
   downstream?
10. What cross-domain modeling cases break this architecture?
11. Which open design questions above must be settled before editing
   `spec.md`?
12. What anti-spec rows are missing?

---

## 17. Proposed Work Sequence

1. Circulate this amendment to external reviewers.
2. Resolve the open questions in Section 14.
3. Run local subagents over `spec.md`, `anti_spec.md`, `soul.md`, and
   mocks with the locked amendment.
4. Produce a comprehensive spec-edit plan.
5. Execute atomic spec/anti-spec/soul/mock edits.
6. Re-run review on the rewritten canonical spec.

No canonical spec rewrite should happen until the amendment's core
surface is locked.
