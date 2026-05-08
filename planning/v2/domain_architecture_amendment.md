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

1. **Nominal Domain Identity.**
   Domains are nominal identities, not ordinary types, values, records,
   or shapes. Importing a constructor does not create a domain identity.
   A `domain` declaration does.

2. **No Implicit Domain Merge.**
   Same constructor, same chart, same unit, same name, and same
   capability contract never make two domains the same. Identity is
   preserved by importing or passing the same domain identity, not by
   structural coincidence.

3. **No Implicit Global Time.**
   A model with temporal structure must name a temporal domain, either
   through a concrete domain declaration or through a generic domain
   parameter. There is no sticky default clock.

4. **Invariant Domain Parameters.**
   Domain parameters are invariant. Two domains satisfying the same
   capability contract are not interchangeable unless the source states
   `SameDomain<...>` or provides an explicit correspondence.

5. **Reusable Code Is Generic Over Caller-Owned Loci.**
   A library declares a concrete domain only when that domain identity is
   part of its scientific claim. Reusable model components should usually
   accept domains through generic parameters or context contracts.

6. **Contracts, Not A Parallel Domain Requirement System.**
   There is no separate `domainreq` system. Domain capability
   requirements are ordinary contracts. Domain contexts are contracts
   with associated domains, associated domain maps, and associated
   correspondences.

7. **Cross-Domain Correspondence Is Explicit.**
   A cross-domain read names a correspondence or occurs inside an
   explicit lexical correspondence scope. The compiler does not infer
   "the only coupling in scope."

8. **Composition Is Distinct From Correspondence.**
   `Space x Time` is a domain composition, not a coupling. A
   correspondence aligns different loci. Product, bundle, quotient,
   graph-indexed, tree-indexed, and spacetime-style compositions have
   their own constructors and emitted domain maps.

9. **Support Domains And Evolution Domains Are Distinct.**
   A field's instantaneous support is where its current value lives. A
   temporal block declares the domain over which its trajectory evolves.
   The compiler elaborates this to an explicit trajectory/lift site.

10. **Temporal Domains Do Not Own Event Occurrence Semantics.**
    Temporal domains provide order, duration, causality, frontier,
    branch, and snapshot semantics. Event families define occurrence
    laws.

11. **Observation Operators Are Source-Visible Measurement Models.**
    Workflow supplies data to declared observation operators. Python does
    not invent measurement semantics at `observe(...)` time.

12. **Realization Is Staged, Typed, And Run-Locked.**
    Source declares obligations. Workflow binds realization sources.
    Providers emit realization instances and evidence. Run records lock
    the realization dependency DAG: provider identity, source data,
    seeds, traces, posterior policies, validation, fallbacks, and
    realization-to-realization dependencies.

13. **No Hidden Domain Generation.**
    Per-entity surfaces, local clocks, per-site calendars, and
    event-created loci require declared domain families or equivalent
    family sites.

14. **Uncertain Domain Use Is Explicit.**
    If a domain realization is posterior, sampled, or learned, a
    deterministic downstream use must declare how that uncertainty is
    consumed: sample, MAP, marginalize, propagate posterior, or another
    named policy.

15. **Domain Equality Is Compile-Time Identity.**
    `SameDomain<A, B>` is an identity constraint at the domain metalevel,
    not a modeled relation, correspondence, provider claim, or
    evidence-backed fact. It is satisfied only when `A` and `B` name the
    same `DomainSite`.

16. **Run-Lock DAGs Are Not Solve Graphs.**
    Realization dependency DAGs record artifact, provider, source-data,
    and provenance dependencies. Semantic solve graphs contain
    equations, relation fulfillments, state variables, event effects, and
    correspondence-map fulfillments; they may contain ordinary SCCs.

17. **Temporal Execution Is Coordinated Explicitly.**
    Temporal domains define order and snapshot structure. Temporal lifts,
    event families, delayed occurrence maps, adaptive root detection, and
    output snapshots define source semantics over that structure. A
    compiler-emitted temporal execution site coordinates compatible
    realizations so workflow cannot accidentally assemble an incoherent
    hidden scheduler.

The ergonomic rule behind these invariants is "explicit at the boundary,
concise inside the boundary." A lexical block may name a temporal domain
or correspondence once and allow shorter forms inside that block, but it
must not infer that context from imports, shape, unit, name, or the
current number of candidates in scope.

---

## 3. Core Ontology

### 3.0 Common Site Shape

The new surface introduces several indexed structures: domains, domain
families, correspondences, correspondence families, event families,
occurrence domains, observation operators, and realizations. They should
share a common implementation shape instead of becoming unrelated
special cases.

Each site should answer:

- what identity it introduces;
- what parameters, if any, index its instances;
- what constructor, contract, or declaration gives it structure;
- what capabilities it advertises or requires;
- what obligations it emits;
- what realization hooks it exposes to workflow;
- how it appears in the catalog and in `hypha explain`.

The sites differ semantically, but their identity, parameterization,
capability, obligation, realization, and explanation machinery should
reuse the same substrate wherever possible.

The common site substrate must not erase site kind. A `DomainSite`,
`DomainMapSite`, `CorrespondenceSite`, `ObservationOperatorSite`,
`EventFamilySite`, and `RealizationInstance` may share catalog,
provenance, capability, obligation, and explain machinery, but they are
not interchangeable semantic objects.

Initial site kinds:

```text
SiteKind =
  Domain
  DomainFamily
  DomainComposition
  DomainMap
  SubLocus
  Correspondence
  CorrespondenceFamily
  TemporalLift
  TemporalExecution
  EventFamily
  OccurrenceDomain
  ObservationOperator
  Realization
  WorkflowAxis
```

### 3.1 Domain

A `Domain` is a declared nominal locus over which quantities, events,
observations, or structures can be located.

Domains live at a distinct metalevel from values and ordinary value
types. A generic parameter such as `<T: TemporalDomain>` ranges over
domain identities satisfying a contract; it is not a scalar value and
not a runtime object. A domain family may have runtime-indexed
instances, but those instances are created only through declared family
sites.

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
the same nominal identity is imported, re-exported, passed through a
generic/context parameter, or explicitly aliased under rules still to be
settled. A domain alias, if added, must be identity-preserving:

```myco
domain alias LocalTime = shared_clock::ExperimentClock
```

This would create no new `DomainSite`, emit no correspondence, and
change no constructor, capability, realization obligation, or coordinate
semantics. It is just a local name for an existing identity.

```myco
domain ExperimentA: ContinuousTime<Second> as t_a
domain ExperimentB: ContinuousTime<Second> as t_b
```

`ExperimentA` and `ExperimentB` are not the same domain.

Importing an existing domain identity from another spore preserves that
identity. Importing a constructor does not. This distinction is
load-bearing for reusable code:

```myco
use std::time::ContinuousTime       // imports a constructor
use shared_clock::ExperimentClock   // imports a domain identity

domain LocalClock: ContinuousTime<Second> as t
```

`LocalClock` is new. `shared_clock::ExperimentClock` remains the same
domain identity it had in its defining spore.

Library spores that declare concrete module-scope domains should be
linted unless they opt in to that commitment. A reusable diffusion
library that hardcodes its own clock likely made a composability
mistake; a library modeling a named instrument or field apparatus may
own its clock legitimately.

The `as` clause binds local coordinate names for the domain constructor's
locator or chart parameters. It does not participate in nominal identity.
Two charts that use the same names over different domain identities are
still different domains, and two chart names for the same imported
domain should be treated as local names for the same identity.

For simple constructors:

```myco
domain Clock: ContinuousTime<Second> as t
domain LeafSurface: Surface<2, Meter> as (u, v)
```

`t`, `u`, and `v` are coordinate binders. For structured constructors
such as fiber bundles, the constructor defines the chart structure that
the `as` clause destructures:

```myco
domain Vasculature:
    std::domain::FiberBundle<BranchGraph, BranchAxis>
    as (edge, z)
```

An imported domain may be locally chart-renamed without becoming a new
identity:

```myco
use shared_clock::ExperimentClock as Clock

domain alias LocalClock = Clock
// A future chart-alias syntax may allow `Clock as tau`, but it still
// names the imported `ExperimentClock` identity.
```

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

Module-scope domains are module-owned identities. Generic domain
parameters are caller-owned identities. Per-entity or per-instance loci
should not be created by putting a `domain` declaration inside a
reusable type body unless the semantics of that declaration are
explicitly specified. In most cases, the right surface is a domain
family or a total-space composition.

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

The total-space/bundle form should be the preferred representation for
fields spanning all family members. `LeafSurface[leaf]` is best treated
as a restriction or view over the declared family/total-space site, not
as arbitrary runtime identity generation.

An entity-local type needs an explicit binding to its index in the
family. The preferred shape is a standard contract that exposes the
entity's coordinate in the index domain:

```myco
contract IndexedEntity<I: Domain> {
    index: Locator<I>
}

type Leaf : IndexedEntity<Leaves> {
    chlorophyll:
        Field<Concentration> over LeafSurface[self.index]
}
```

Exact syntax for `self.index` and `LeafSurface[...]` is open, but the
rule is not: a type may refer to "its own" fiber only through an
explicit family-instance context. Outside such a context, fields over all
members should use the total-space domain.

The preferred ergonomic surface can be sugar over the same contract:

```myco
type Leaf indexed by Leaves {
    chlorophyll:
        Field<Concentration> over LeafSurface[self]
}
```

This form introduces `self` as a locator in the `Leaves` index domain
for the type body. It does not create a hidden domain instance; it binds
the entity-local view to the declared family site.

Conceptual payload:

```text
DomainFamilySite
  family id
  index domain
  instance constructor
  instance identity rule
  realization mode
  topology/dynamic phase inherited from the index domain
  emitted total space, if any
  emitted instance projection, if any
  entity/fiber binding contract, if any
```

Family instance identity should be keyed explicitly:

```text
FamilyInstanceIdentityKey
  family site id
  index-domain identity
  index entity identity

EventCreatedFamilyInstanceIdentityKey
  family site id
  birth event occurrence identity
  generated entity identity
```

Across runs, family-instance identity is comparable only when the index
identity is comparable. For posterior or sampled event histories,
generated family instances are draw-local unless a correspondence or
posterior identity policy relates them.

If the index domain is event-created or dynamically changing, the domain
family inherits the same dynamic-topology phase. For example, a family
indexed by a `CapacityMask` entity domain can allocate family instances
up to the declared capacity, while a family indexed by an `EventReplan`
domain forces re-planning when new instances appear. This should hook
directly into the existing dynamic topology handlers rather than invent
a separate "runtime domain creation" path.

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

Emitted maps are named catalog objects. A product constructor can emit
`Product.proj_left` and `Product.proj_right`; a bundle constructor can
emit base and fiber maps; a quotient constructor can emit a quotient
map. These emitted maps may be compiler-proven for the constructor, but
they are still visible. The compiler should not invent additional
projections because a composition "looks product-like."

These emitted maps should be represented as `DomainMapSite`s, not as
general correspondences:

```text
DomainMapSite
  id
  source domain
  target domain
  constructor provenance
  exactness and totality guaranteed by the constructor
  emitted operation capabilities
```

`ProjectionSite`, `BundleProjectionSite`, and `QuotientMapSite` are
domain maps. A `CorrespondenceSite` is broader: it may be partial,
many-to-many, learned, posterior, state-dependent, or evidence-bearing.
This distinction lets `hypha explain` say whether a read used a
compiler-proven projection or a modeled alignment.

Domain maps and correspondences should share the same operation
interface where that is mathematically valid. A compiler-proven product
projection should be usable by the same read/pullback syntax as a
correspondence operation, but its capabilities are generated from the
constructor rather than supplied by a provider or model relation.
DomainMapSite is therefore not a semantic subtype of
CorrespondenceSite, but both can satisfy shared operation contracts such
as `SupportsRead`, `SupportsPullback`, or `SupportsPushforward`.

DomainMapSites automatically emit capability facts for mathematically
valid transports. For example, a product projection can emit exact scalar
pullback/read capabilities and a zero-cost broadcast interpretation where
the constructor justifies it. Users should not need to wrap built-in
product projections in manual correspondence blocks.

The compiler may not silently compose domain maps and correspondences
across arbitrary paths. Same-domain reads need no map; direct emitted
maps from declared compositions are legal; explicit named compositions
are legal. A path like `A -> B -> C` does not silently become an `A -> C`
transport merely because the intermediate maps exist.

Domain projections emitted by composition constructors are structural
domain maps. They are unrelated to runtime constraint projection or any
optimization/projection rule used to discharge residuals.

Sub-loci should be explicit but lighter than domains:

```text
SubLocusSite
  parent domain
  selector, boundary, region, chart slice, or footprint
  no independent nominal identity unless promoted to a DomainSite
  no independent realization obligation unless the constructor requires one
```

Examples include a boundary of `LeafSurface`, an interval of `Clock`, an
edge subset of a graph, a one-sided trace locus at an interface, or a
pixel footprint region. A sub-locus is how the language can name
boundaries and regions without turning every selector into a new domain.

### 3.7 Correspondence

A correspondence aligns loci. This amendment treats `correspondence` as
the canonical source keyword and `CorrespondenceSite` as the canonical
internal concept. The older word `couple` may appear in discussion or
legacy examples, but should not be the canonical v2 spelling.

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
correspondence Deformation {
    witness MaterialAtTime:
        std::domain::Product<MaterialSolid, Clock>

    leg material:
        MaterialAtTime -> MaterialAtTime

    leg physical:
        MaterialAtTime -> PhysicalSolid
        as physical_position

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

`leg` declarations define the span from the witness locus to the
participating domains. Named maps are functions exposed by those legs or
by additional correspondence structure. Supported operations are the
reads, pullbacks, pushforwards, transports, or snapshot operations that
consume those maps. This distinction matters for partial,
many-to-many, interval, and posterior correspondences where the witness
cannot be reduced to one function.

An inline witness must elaborate to a real witness/domain site. It is
not an untyped placeholder. For simple functional correspondences the
witness may be a product or graph-like constructor; for interval,
many-to-many, or posterior correspondences the witness is the locus that
makes the span explicit.

The loose form:

```myco
couple A between B, C by ...
```

is useful for conversation but should not be canonical. It
hides witness structure and tends to make correspondences look like
single functions.

Trivial correspondences should be ergonomic through stdlib constructors,
not by weakening the canonical witness/leg representation:

```myco
correspondence ObsToModel:
    std::time::correspondence::IdentityClock<ObsClock, ModelClock>
```

The stdlib form elaborates to a `CorrespondenceSite` with witness, legs,
supported operations, and evidence requirements. This keeps identity
clocks, scalar embeddings, and other common exact alignments concise
without making the loose binary form canonical.

### 3.8 Correspondence Families

Correspondences can also be families:

- one deformation map per leaf;
- one local-to-UTC correction per site;
- one body-to-world frame per robot;
- one sensor footprint per satellite;
- one learned alignment per subject.

Preferred shape:

```myco
correspondence family LeafDeformation for leaf in Leaves {
    witness MaterialAtTime:
        std::domain::Product<LeafSurface[leaf], Clock>

    leg physical:
        MaterialAtTime -> PhysicalLeaf[leaf]
        as physical_position
}
```

Exact syntax for indexing family domains is open. The important
architectural rule is stable: correspondence family instances are
nominal and explicit, not generated invisibly.

The index variable must be explicitly bound. A correspondence family
should not conjure `leaf`, `sensor`, or `site` as an implicit ambient
name.

A module-scope correspondence cannot capture instance-local state by
accident. If a correspondence map depends on fields of a particular
leaf, sensor, robot, site, or other entity, the correspondence must be a
family indexed by that entity or an associated correspondence of a
contract/type context that makes the instance identity explicit.

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
        leaf.temperature.trajectory[ModelTime]

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

Observation operators can be generic over domains and contracts, just
like types and relations. A reusable instrument model should not have to
hardcode `LeafSurface` if it can observe any compatible heat-emitting
spatial domain:

```myco
observation_operator ThermalCamera<
    S: SpatialDomain + HasMeasure,
    T: ODETime<Second>,
> {
    data_locus:
        std::domain::Product<CameraPixels, ExposureIntervals>

    target:
        temperature.trajectory[T] over S

    uses correspondence PixelFootprint<S>
    uses correspondence ExposureToModelTime<T>

    noise:
        ThermalSensorNoise
}
```

Trivial path observations can have stdlib sugar, but the measurement
semantics must remain inspectable.

Conceptual payload:

```text
ObservationOperatorSite
  id
  model locus or loci
  data locus or loci
  value type and data coordinate schema
  used domain maps and correspondences
  aggregation or integration semantics
  noise model
  missingness and censoring model
  likelihood form, optional
  residual form, optional
  evidence form, optional
  fit objective contribution, optional
  supported consumption modes:
    observe | fit_to | condition | score | posterior_predict
  calibration dependencies
  evidence and provenance requirements
```

The general workflow route from data to residual, loss, likelihood, or
evidence should pass through an `ObservationOperatorSite`. Trivial path
observations may elaborate to stdlib identity-style operators, but they
should not be a separate semantic path.

The operator declares the measurement model. Workflow chooses the
consumption mode:

```python
workflow.observe(cat.observation_operator("CaseReports"), data=...)
workflow.fit_to(cat.observation_operator("CaseReports"), data=...)
```

Both calls bind data to the same source-visible operator, but consume its
likelihood, residual, evidence, or objective forms differently.

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

Conceptual payload:

```text
OccurrenceDomainSite
  id
  event family id
  occurrence identity type
  temporal domain
  occurrence-time domain map or correspondence
  payload schema
  ordering or partial-order facts
  generated-entity effects, if any
  realization requirement
```

Event occurrence should not be an anonymous runtime log. SSA traces,
replayed market orders, spike trains, event-camera events, infection
histories, and delayed-queue firings are model-addressable loci with
typed payloads and provenance.

### 3.12 Realization

A realization is how a source-declared domain, correspondence,
observation operator, event family, or semantic site becomes concrete
for a run.

Realizations are workflow source objects, parallel to `Constant`,
`Series`, `Trainable`, `Prior`, `ProcessPrior`, and `Controller`. They
are not `.myco` source declarations that mirror value types; they are
workflow constructors that satisfy source-side realization contracts
declared by the stdlib or by imported spores.

Realization sources may consume other workflow source objects when their
contract permits it. A learned mesh resolution, calibrated clock drift,
trainable latency, posterior tree prior, or learned correspondence map
should compose with `Trainable`, `Prior`, `Series`, and `ProcessPrior`
rather than bypassing them. Gradient flow follows the existing workflow
and AD rules for the consumed source objects.

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

Realization dependencies form a DAG. For example, an event-family
realization may depend on a temporal-domain realization, which may
depend on a calendar provider, which may depend on an external dataset
hash. The run lock must record the whole DAG, not only the leaf
realization object that was passed to `workflow.bind`.

Run-lock entries pin realization/provider versions. Replaying an old run
uses the versions, seeds, source hashes, traces, and fallback decisions
recorded in that run lock; a newer stdlib provider version does not
silently change the replay. Mid-run adaptive realization, such as
adaptive mesh refinement, records monotonic realization events in the
run log rather than invalidating earlier run-lock entries.

Realizations may feed the semantic solve graph, but solve outcomes do
not mutate or invalidate run-lock entries within a run. If a solve
requires changing a realization, that change must be modeled as an
event-mutated realization with its own logged provenance.

Posterior or learned realizations need a use policy before they are
consumed as if they were deterministic. Examples include `sample`,
`MAP`, `marginalize`, `propagate_posterior`, and domain-specific
posterior query modes. The policy belongs to workflow/inference
configuration and is recorded in the run lock.

Initial stdlib posterior-use policies should include at least:

- `sample`: draw realizations according to the posterior or prior;
- `MAP`: use a named maximum-a-posteriori or point-estimate realization;
- `marginalize`: integrate downstream quantities over the posterior;
- `propagate_posterior`: carry posterior uncertainty into downstream
  SCCs instead of collapsing it.

A domain realized by a posterior or process prior acts like a stochastic
root for the SCCs that depend on it. Fields, correspondences, event
occurrences, and observation operators over that domain become
stochastic or inference-coupled through the existing probabilistic
machinery; the posterior-use policy states how that uncertainty is
queried for deterministic extraction or reporting.

Posterior use may need joint context, not independent per-site sampling:

```text
PosteriorContextSite
  jointly realized sites
  posterior draw axis, if sampled
  dependence/correlation structure
  query modes
```

`sample` should emit or use a workflow draw axis; `MAP` names a specific
realization within the posterior context; `marginalize` integrates
jointly over the context; `propagate_posterior` preserves dependence into
downstream SCCs.

### 3.13 Temporal Execution Site

A `TemporalExecutionSite` is a compiler-emitted coordination obligation
for a temporal domain or temporal SCC. It prevents separately valid
realizations from composing into an incoherent hidden scheduler.

Conceptual payload:

```text
TemporalExecutionSite
  temporal domain or temporal SCC
  participating TemporalLiftSites
  participating EventFamilySites
  participating ScheduledOccurrenceMaps
  snapshot/output requirements
  conservation obligations touched by events
  required capabilities:
    derivative integration
    event root localization
    stochastic next-event sampling
    delayed queue
    replay
    branch/frontier snapshot
  compatible realization contracts
  realization binding and run-lock fields
```

Source owns the temporal lifts, event laws, stale policies, delay laws,
matching rules, and conservation claims. Workflow binds an execution
realization that satisfies the resulting coordination obligation.

Example:

```python
workflow.bind(
    cat.temporal_execution("Clock"),
    myco.execution.HybridSSAODE(
        ode=myco.ode.AdaptiveRK(...),
        events=myco.events.NextReaction(...),
        delayed=myco.events.DelayedQueue(...),
        root_finder=myco.roots.Bracketed(...),
        seed=42,
    ),
)
```

Incompatible bindings are workflow-composition errors. For example, SSA
next-event sampling over a temporal domain realized only as a fixed tick
array is invalid unless the temporal-domain realization advertises a
capability such as `AdmitsExternalEventTimes` or the execution
coordinator supplies a coherent embedding of stochastic events into the
tick lattice.

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

    derived domain Trajectory =
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
- associated domain-map/projection members;
- associated correspondence members;
- satisfaction checking;
- coherence;
- diagnostics;
- how `hypha explain` reports associated members.

Associated compositions do not need a separate kind unless the spec
discovers a reason for one. A composition is an associated domain whose
constructor references other associated domains; its emitted projections
or maps are associated domain-map members.

Associated domains need an explicit generativity rule:

- `associated domain X: Bound` is required. The implementer must supply
  an existing nominal domain identity.
- `derived domain X = Constructor<...>` is generated. The implementing
  context owns a new nominal identity at a path such as `SimWorld::X`.
  This is permitted only because the contract explicitly declares the
  derived domain.

Sketch:

```myco
contract LeafWorld {
    associated domain Space: DiffusionSpace
    associated domain Time: ODETime<Second>

    derived domain Trajectory =
        std::domain::Product<Space, Time>
}
```

This prevents associated compositions from becoming hidden domain
generation while still avoiding boilerplate for canonical products.

Derived-domain identity is owned by the concrete context implementation,
not by each use site. All consumers of `SimWorld` see the same
`SimWorld::Trajectory` identity. A different context implementation that
derives the same structural `Product<Space, Time>` still gets a distinct
nominal derived domain unless the same identity is imported/passed or a
domain equality constraint is stated.

Identity key:

```text
DerivedDomainIdentityKey
  declaring contract member path
  implementing context identity
  context generic arguments
  derived member name
```

Candidate instantiation shape:

```myco
domain SimClock: std::time::ContinuousTime<Second> as t
domain SimSpace: std::space::Surface<2, Meter> as (u, v)

type SimWorld impl LeafWorld {
    associated domain Time = SimClock
    associated domain Space = SimSpace
}

type ConcreteLeaf = LeafModel<SimWorld>
```

This follows the Rust-associated-type intuition without creating a
parallel domain-bundle language. The implementation still must specify
coherence and elaboration order explicitly.

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

`SameDomain<A, B>` is a compile-time identity constraint. It cannot be
satisfied by a correspondence, provider validation, identical structure,
or runtime evidence. If the domains are distinct but aligned, use a
`CorrespondenceSite` instead. General `identify domain` is not part of
the current landing; if any restricted identity witness survives, it
should be exceptional and provenance-heavy.

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
- correspondence and realization obligations;
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

Two dependency graphs must remain distinct:

- **RealizationDependencyDAG.** Provenance/artifact/provider graph for
  realized objects. This graph must be acyclic and run-locked.
- **SemanticSolveGraph.** Equations, relation fulfillments, residuals,
  state variables, correspondence-map fulfillments, and event effects.
  This graph may contain ordinary SCCs.

A state-dependent correspondence fulfilled by source relations belongs
to the semantic solve graph. Its provider artifacts, if any, belong to
the realization DAG. Do not use the run-lock DAG as a solver dependency
graph.

Error tier rule:

- source-visible structural domain errors are compile-tier errors:
  missing domain declarations, invalid associated-domain bounds,
  cross-domain reads without a named domain map/correspondence, naked
  `d`, and unsupported domain operations;
- binding and realization errors are workflow-composition-tier errors:
  missing domain/event/correspondence realizations, missing
  posterior-use policies, incompatible realization capabilities,
  unfulfilled correspondence maps, and observation data-coordinate
  mismatches;
- provider/runtime failures remain runtime-tier errors only when they
  depend on actual execution or external provider behavior.

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

A state without an explicit support domain has internal support
`UnitLocus` within its containing entity/context. A scalar temporal
state therefore lifts over time as `Product<UnitLocus, T>`, normalized
to `T` in catalog display:

```myco
x: Scalar<dimensionless>

temporal over Clock {
    d x = -k * x
}
```

If a scalar state belongs to an entity collection, the entity context
contributes support. For example, `Plant.biomass` for many plants is
supported by the plant entity domain when viewed at collection scope;
`UnitLocus` applies only when the state truly has no entity or family
support.

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
  id
  state path
  instantaneous state facet
  trajectory state facet
  instantaneous support domain
  evolution domain
  trajectory locus, usually Product<support, evolution>
  emitted domain maps/projections
  derivative or update operator site
  initial-condition locus
  boundary-condition loci
  initial-boundary obligations
  observation/fitting facets
  conservation locus implications
  invalidation dependencies
```

The `TemporalLiftSite` is the canonical owner of trajectory identity.
The catalog should expose both instantaneous and trajectory facets so
workflow observations can target the intended object.

Initial conditions, boundary conditions, delayed reads, temporal
observations, and conservation obligations over evolving state attach to
the `TemporalLiftSite`, not directly to the raw field declaration.

For a derivative-supporting temporal domain, the lift emits canonical
boundary domain maps from the instantaneous support into the trajectory
at the start/frontier of the evolution domain. An `initial` block is
syntactic sugar for constraining the trajectory along that start-boundary
map.

Boundary maps depend on temporal capability:

```text
TemporalBoundaryMapSite
  temporal domain
  support domain
  boundary kind:
    start | successor | frontier | branch_root |
    branch_local | interval_endpoint
  emitted domain map or correspondence
  required temporal capability
```

Continuous total-order time can provide a start inclusion. Discrete time
can provide initial-tick and successor maps. Partial-order time can
provide initial antichain/frontier maps. Branching time can provide root
and branch-local boundary maps. `initial over T` is legal only when `T`
advertises the required boundary capability.

Initial blocks must be anchored to the relevant temporal domain. Either
spell the anchor directly:

```myco
initial over Clock {
    x = 5
}
```

or place the initial block lexically inside the temporal block:

```myco
temporal over Clock {
    initial { x = 5 }
    d x = -k * x
}
```

An unanchored type-level `initial { ... }` is invalid for lifted state
unless the spec defines a legacy migration rule that elaborates it to an
unambiguous `initial over T`.

Past or delayed reads should use the trajectory facet explicitly. For a
delay differential equation or distributed lag, do not make the
instantaneous field pretend that its support is time-indexed:

```myco
temporal over Clock {
    let old_conc =
        concentration.trajectory @ (t - delay)

    d concentration = production(old_conc) - loss(concentration)
}
```

The spelling of `.trajectory @ ...` is provisional. The semantic rule is
not: the past read targets the lifted trajectory facet, not the
instantaneous support facet.

A state path may have at most one owning `TemporalLiftSite` in a model
context. For v2, evolving the same state in two temporal blocks is a
hard compile error. A model with two clocks should declare separate
state facets and an explicit correspondence or observation/control
operator between them.

In multi-rate systems, a quantity influenced by both slow and fast
dynamics should be modeled as separate state facets over their owning
clocks, connected by explicit correspondences, relations, or
control/observation operators. It should not be modeled as one state
evolved by two clocks.

### 6.3 Temporal Context Scope

Current landing:

- `temporal over T` provides the default temporal domain for `d` and
  `step` within the block.
- event-family declarations do not inherit `T`; `event family E over T`
  must name its home temporal domain.
- `integrate`, `observe`, `fit_to`, cross-domain reads, and
  correspondence operations should still name their domains or
  correspondences explicitly unless a separate lexical scope does so.

This is the current landing. A future `using temporal T { ... }` sugar
could be considered, but `temporal over T` itself should not become a
general ambient default.

Nested temporal blocks should use ordinary lexical shadowing for the
default temporal context. However, ownership of a state's evolution
comes from its `TemporalLiftSite`, not from the most recent textual
block. A nested block over `ControllerTick` should not silently change a
state already owned by a lift over `PlantTime`.

Fresh state first evolved inside a temporal block gets its
`TemporalLiftSite` from that block's temporal domain. Subsequent
temporal updates to the same state must use the same owning lift or
error.

This augments, rather than replaces, the existing `y[t]` and `y[t+1]`
ground-term machinery. Ground terms at distinct temporal coordinates
remain distinct. The lift site records which temporal domain those terms
belong to, which trajectory facet owns them, and which realization
provides the coordinate semantics.

### 6.4 `over` as Domain Anchor

The keyword `over` should have one conceptual meaning: it anchors a
declaration, operator, or block to a domain.

Known positions:

- field support: `field: Field<T> over S`;
- temporal context: `temporal over T { ... }`;
- event-family home time: `event family E over T { ... }`;
- explicit derivative/update: `d(x, over = T)`;
- domain-sensitive operators: `integrate(expr, over = D)`;
- observation/operator declarations where a target or data locus is
  domain-indexed.

`over` should not mean "infer whatever domain is convenient." It names
the domain anchor for the syntactic form where it appears. If a syntax
position can see more than one compatible domain, the source must name
the intended one.

The existing spatial/subdimensional form that anchors a field to a
coordinate, such as `field moisture over z`, should be retired in favor
of anchoring to a domain or declared sub-locus. The `z` coordinate should
come from the domain's `as` clause:

```myco
domain SoilColumn: std::space::Interval<Meter> as z

field soil_moisture:
    Scalar<volume_fraction> over SoilColumn
```

An `over` anchor does not propagate to sibling domain-sensitive forms
unless that syntax form explicitly defines such propagation:

```myco
temporal over T {
    d x = ...
    integrate(y)      // error: integration domain is not inherited
}
```

### 6.5 Partial and Branching Time

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

Initial capability rule sketch:

| Operation | Required capability |
| --- | --- |
| `d x` | `SupportsDerivative<T>` plus duration/measure contract |
| `step x` | `SupportsStep<T>` |
| delayed scheduling | `SupportsDelay<T>` or event-family delay support |
| hazard or propensity | `SupportsHazard<T>` or event-family realization support |
| causal-frontier read | `SupportsFrontierSnapshot<T>` |
| branch-local read | `SupportsBranchSnapshot<T>` |
| calendar successor | `CalendarSuccessor<T>` |
| integration over a domain | `SupportsIntegration<D>` plus measure contract |

Hazards and propensities require a temporal reference measure or
discrete event-probability structure. A continuous-time hazard should be
typed against something like:

```myco
contract HazardTime<U> : TemporalDomain {
    requires DurationMeasure<Self, U>
    requires SupportsHazard<Self>
}
```

Discrete event probabilities should use a separate capability such as
`SupportsDiscreteEventProbability<Self>`. Propensity formulas should not
float free of the temporal semantics that give them units and snapshot
meaning.

---

## 7. Correspondence Semantics

### 7.1 Cross-Domain Reads

Same-domain reads need no correspondence. Cross-domain reads require:

- a named emitted domain map from a declared composition;
- a named correspondence;
- a named supported operation when the value kind is not safely
  inferred; or
- an explicit lexical correspondence scope.

Domain maps and correspondences do not silently compose transitively.
If a use requires a path through several maps or correspondences, the
source must name a composed map/correspondence site or spell the
operation sequence explicitly.

`convert` is not a domain-alignment mechanism. Unit, wrapper, or value
conversion can occur inside an expression or observation operator, but
locus alignment uses a `DomainMapSite` or `CorrespondenceSite`.

Simple scalar read:

```myco
temperature @ ModelTime via SensorToModelTime
```

Scoped shorthand is allowed only when the scope names the
correspondence explicitly:

```myco
using correspondence SensorToModelTime {
    temperature @ ModelTime
}
```

This is not ambient inference. Moving a line outside the block should
make the missing correspondence visible again.

The scope binds only the named correspondence and only operations that
are unambiguous for the value kind and use context. Nested scopes shadow
only for the same domain pair or operation family; they do not unset
unrelated explicit scopes. An explicit `via` at the use site overrides
the lexical shorthand.

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
one correspondence is currently in scope.

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
  declaration evidence requirement
  realization evidence supplied in this run
```

The source may require an evidence grade for using an operation; the
run's realization supplies the actual evidence record. Do not conflate
"this operation requires provider-validated Jacobians" with "this
provider has validated the Jacobian in this run."

Approximate, sampled, or posterior correspondence operations must enter
the approximation/evidence ledger just like approximate event
realizations. A correspondence-mediated approximation should not become
invisible merely because the source-level correspondence is named.

### 7.3 Correspondence Obligations

A correspondence declaration emits obligation sites for its declared
state-dependent legs/maps and supported operations.

Current landing:

- one `ObligationSite` per declared state-dependent leg/map slot;
- default cardinality `exactly_one`;
- fulfillment relations use `fulfills Correspondence.map_name`;
- coupled SCCs form normally when fulfillment relations share variables
  with model physics.

Example:

```myco
correspondence LeafDeformation {
    witness MaterialAtTime:
        std::domain::Product<MaterialLeaf, Clock>

    leg material:
        MaterialAtTime -> MaterialAtTime

    leg physical:
        MaterialAtTime -> PhysicalLeaf
        as physical_position
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
physics using the map, the ordinary SCC decomposition should group the
map fulfillment relation with the physics that depends on it. No new SCC
class is required. Diagnostics may describe an ordinary SCC as
"correspondence-coupled" when it contains correspondence-map
obligations, but this should not fork the solver taxonomy.

### 7.5 Cross-Domain Conservation

Conservation obligations should name the locus where conservation is
checked. Event effects, domain maps, and correspondences must explain how
deltas are transported into that conservation locus.

Sketch:

```myco
domain PhysicalSpaceTime:
    std::domain::Product<PhysicalSpace, Clock>

conservation Mass over PhysicalSpaceTime {
    conserved quantity total_mass
}

correspondence RxnToClock:
    std::time::correspondence::IdentityClock<RxnTime, Clock>

event family Reaction over RxnTime {
    effect {
        substrate -= one_mole
        product += one_mole
    }

    fulfills Mass
        via RxnToClock
        over PhysicalSpaceTime
}
```

The exact syntax is provisional. The semantic rule is that conservation
is not proven by the event firing in isolation. The compiler must trace
the event delta through the named temporal/spatial maps into the
conservation locus and check the corresponding obligation there.

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
        payload {
            complex_id = complex.id
        }
        stale: CancelIfComplexGone
        firing_state: captured
    }
}

event family Release over T {
    effect {
        complex -= one
        product += one
    }
}
```

For delayed events, source owns the scientific semantics:

- delay law;
- target event family;
- captured payload at scheduling time;
- stale/cancellation/merge policy;
- whether firing uses captured state, current state, payload-only state,
  or resampled state;
- conservation obligations.

Canonical firing-state policy names:

- `captured`: firing reads captured scheduling-time state;
- `current`: firing reads current state at firing time;
- `payload_only`: firing reads only declared payload;
- `resampled`: firing resamples or redraws the relevant state according
  to a declared law.

Workflow/provider owns the queue representation, sampling algorithm,
approximation choice, seed/replay, and validation evidence.

Delayed scheduling should elaborate to a model-addressable scheduled
occurrence map:

```text
ScheduledOccurrenceMap
  source occurrence domain
  target occurrence domain
  delay law
  payload capture schema
  stale/cancel/merge policy
  temporal correspondence constraints
```

This prevents `schedules Release` from becoming an opaque queue side
effect.

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

In hybrid systems, event-family bindings are checked against the
`TemporalExecutionSite` for the home temporal domain. A set of
individually valid event-family and temporal-domain realizations can
still be rejected if no coordinator can execute them coherently.

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

Event families declare occurrence domains. Event-family realizations
produce occurrence streams over those domains. These need catalog
handles so they can be:

- observed;
- replayed;
- queried;
- used downstream;
- compared to data;
- recorded in run lock.

Each occurrence domain needs an identity type and payload schema. A
possible source shape is:

```myco
event family Infection over T {
    occurrence InfectionEvent {
        source: Person
        target: Person
        strain: Strain
    }

    hazard:
        beta * contact(source, target)

    effect {
        target.infected becomes true
    }
}
```

This is not yet fully specified, but occurrence identity and payload
typing are not optional. They are the bridge between event laws, replayed
traces, observation operators, and run-lock provenance.

The source-level locus and the run-level stream must remain distinct:

```text
OccurrenceDomainSite
  source-level nominal locus of possible or actual occurrences

OccurrenceStreamRealization
  run-level realized sequence, poset, tree, sample, trace, or posterior
```

Occurrence identity should bridge to ordinary domain machinery: an
occurrence is located by its occurrence-domain identity, home temporal
coordinate or frontier, and typed payload coordinates. The exact key
shape depends on the event family and realization, but it must be
declared enough for observation, replay, querying, and provenance.

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

Observation operators should satisfy ordinary contracts, not introduce a
parallel mechanism. A stdlib contract such as
`ObservationOperator<ModelLocus, DataLocus, Value>` can require the
correspondences, aggregation relation, noise structure, missingness behavior,
and evidence shape that a workflow `observe(...)` call consumes.

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

Observation targets must distinguish instantaneous state facets from
trajectory facets. If `plant.biomass` is a scalar state evolved over
`ModelTime`, then an observation at dates targets the lifted trajectory,
not the instantaneous scalar alone. Candidate spellings include
`plant.biomass.trajectory[ModelTime]` or
`trajectory(plant.biomass over ModelTime)`; the exact syntax is open,
but the facet distinction must be visible to the catalog and explain
output.

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
workflow axes by workflow composition, but workflow must not promote an
axis into source-readable semantics after the fact. If a law needs to
depend on a scenario, cohort, sampling frame, or observation locus, that
locus must already be declared as a source domain with explicit
semantics.

This area needs a catalog mechanism parallel to domains:

```text
WorkflowAxisSite
  id
  axis kind:
    replicate | sweep | scenario_batch | posterior_draw |
    cv_fold | backend_comparison
  coordinate schema
  lifting target
  aggregation operations
  binding/provenance
  lifting semantics
  source visibility: forbidden
```

Workflow outputs may be indexed by workflow axes, such as
`MonteCarloReplicate`, `ParameterSweep`, or `PosteriorDraw`. Those axes
are catalog-addressable for workflow outputs and aggregation, but source
laws cannot read them.

### 10.1 Process Priors And Domains

`ProcessPrior<I, V>` does not create a domain by itself. It is a
workflow/probabilistic source for structured values indexed by `I`.

A process prior may define a distribution over the realization of a
declared catalog site only when that target is explicit:

```python
# Prior over a field value, not a domain.
workflow.bind(
    cat.path("temperature"),
    ProcessPrior(...),
)

# Prior over a declared latent domain realization.
workflow.bind(
    cat.domain("TransmissionTree"),
    myco.prior.CoalescentTreePrior(...),
)

# Prior over a declared event-family realization.
workflow.bind(
    cat.event_family("Infection"),
    myco.prior.HawkesEventHistoryPrior(...),
)
```

This keeps `Domain` from becoming a synonym for "any stochastic
structured object" while still allowing inferred domains, posterior
correspondences, and latent event histories.

Target kind should be typed for diagnostics, even if a single generic
workflow-source implementation handles several cases:

```text
ProcessPrior<I, V> over value path
DomainRealizationPrior<D>
CorrespondenceRealizationPrior<C>
EventFamilyRealizationPrior<E>
ObservationParameterPrior<O>
```

These are target-kind specializations of the same realization/source
machinery, not a parallel contract hierarchy. For example,
`DomainRealizationPrior<D>` is a `Realization<D>` whose source character
is stochastic and whose evidence records `obtained_by = sampled`.

For example, a `CoalescentTreePrior` bound to
`cat.domain("TransmissionTree")` must satisfy a
`Realization<TransmissionTree>`-shaped contract with stochastic evidence
and `realized_object_kind = tree`. A prior over a tree-valued field does
not silently create a tree domain.

---

## 11. Identity, Refactoring, and `identify domain`

Nominal identity is valuable but creates a refactoring footgun:

- a model originally declares local `domain Time`;
- later `Time` is moved to a shared spore;
- one file keeps the old local declaration;
- now two structurally identical clocks exist.

The compiler should not silently merge them.

Current landing: do not add general `identify domain` as routine
modeling syntax. It is too easy for it to erase the nominal-identity
discipline this amendment is trying to establish.

Preferred tools:

1. import or re-export the original domain identity;
2. pass the same domain identity through a generic/context parameter;
3. use a source-level alias if the spec adds alias syntax;
4. for structurally identical but nominally distinct loci, declare an
   exact `IdentityMap` correspondence and let extraction optimize it
   when safe.

Choice rule:

- use import/re-export, generic/context passing, or `domain alias` when
  the names should refer to the same domain identity;
- use `IdentityMap` when the loci are distinct nominal identities that
  are structurally isomorphic and should remain distinct but exactly
  aligned.

Sketch:

```myco
correspondence AToB:
    std::correspondence::IdentityMap<A, B>
where SameStructure<A, B>
```

`IdentityMap` is still a correspondence between distinct domains. It is
not domain equality, cannot satisfy `SameDomain<A, B>`, and should be
used only when the model wants to preserve the distinction while stating
an exact alignment.

`hypha check --domain-audit` should report structurally similar but
nominally distinct domains across a workspace. It should not auto-fix
them, and it should not imply they are the same locus.

If a future version still needs `identify domain`, it should be treated
as a restricted identity witness, not a convenience feature:

- source-only, never workflow-side;
- top-level only;
- impossible after conflicting facts, realizations, or correspondences
  exist;
- explainable in provenance;
- never inferred by tooling;
- rejected if either side has incompatible realization obligations.

The v2 decision is: no general domain identity merge and no restricted
post-declaration identity witness. Future versions can revisit this only
if alias/import/context passing and exact correspondences prove
insufficient.

Dynamic fusion of nominal domains is also not part of the model. If two
droplets, cells, regions, or branches may merge or split at runtime,
model them as regions, indicators, or phase fields over a declared
superset domain, or as dynamic topology within one declared family. Do
not model them as independent nominal domains that fuse into a new
identity.

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
    derived domain Trajectory =
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

Entity-local sugar can expose the same family safely:

```myco
type Leaf indexed by Leaves {
    chlorophyll:
        Field<Concentration> over LeafSurface[self]
}
```

### 12.5 Observation Clock

```myco
domain ModelTime: std::time::ContinuousTime<Second> as t
domain ObsClock: std::time::CalendarTime<UTC> as date

correspondence ObsToModelTime {
    witness ObsInstant
    leg model_time: ObsInstant -> ModelTime
    leg obs_date: ObsInstant -> ObsClock

    supports read scalar_field
        from ModelTime
        to ObsClock
}

observation_operator BiomassCensus {
    data_locus: ObsClock
    target: plant.biomass.trajectory[ModelTime]
    uses correspondence ObsToModelTime
    noise: Gaussian<BiomassError>
}
```

For trivial exact alignments, a stdlib correspondence constructor can
stand in for the full witness/leg block:

```myco
correspondence ObsToModelTime:
    std::time::correspondence::IdentityClock<ObsClock, ModelTime>
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

Hybrid execution binds the temporal coordination site, not a hidden
global scheduler:

```python
workflow.bind(
    cat.temporal_execution("T"),
    myco.execution.HybridSSAODE(
        ode=myco.ode.AdaptiveRK(...),
        events=myco.events.NextReaction(...),
        seed=42,
    ),
)
```

### 12.7 Moving Mesh Correspondence

```myco
domain Material: std::space::ReferenceBody<Meter> as X
domain Physical: std::space::Euclidean<3, Meter> as x
domain Time: std::time::ContinuousTime<Second> as t

domain MaterialAtTime:
    std::domain::Product<Material, Time>

correspondence Deformation {
    witness MaterialAtTime

    leg material:
        MaterialAtTime -> MaterialAtTime

    leg physical:
        MaterialAtTime -> Physical
        as physical_position

    supports pullback scalar_field
        from Physical
        to MaterialAtTime

    supports pullback density_field
        from Physical
        to MaterialAtTime
        requires Jacobian(physical_position)
}
```

Fulfillment can depend on state and enter the ordinary SCC
decomposition:

```myco
type Solid {
    displacement: Vector<m> over Material

    temporal over Time {
        relation deformation_law
            fulfills Deformation.physical_position
        {
            physical_position(X) = X + displacement(X)
        }

        relation stress_balance {
            ...
        }
    }
}
```

If `stress_balance` depends on `physical_position` and
`physical_position` depends on displacement solved by `stress_balance`,
the existing SCC machinery groups them. This is an ordinary SCC
containing correspondence fulfillment, not a new solver class.

### 12.8 Negative Examples

No implicit global time:

```myco
type BadDecay {
    x: Scalar<dimensionless>
    d x = -x      // error: missing temporal domain
}
```

Same constructor does not imply same domain:

```myco
domain A: std::time::ContinuousTime<Second> as t
domain B: std::time::ContinuousTime<Second> as t

relation bad {
    value_at_a = value_at_b   // error without SameDomain or correspondence
}
```

Cross-domain read missing a correspondence:

```myco
temperature @ ObsClock        // error: no named domain map/correspondence
```

Non-scalar transport needs an operation:

```myco
rho_material = rho_physical via Deformation
// error: density transport must name an operation such as
// Deformation.pullback_density
```

Workflow axes are not source temporal domains:

```myco
temporal over Replicate {
    d x = ...
}
// error unless Replicate is declared as a source domain with semantics
```

Domain family members require an explicit family-instance context:

```myco
type BadLeaf {
    chlorophyll: Field<Concentration> over LeafSurface[self]
}
// error: `self` is not bound to the Leaves index domain
```

### 12.9 Stress Cases For Review

The next review round should test at least:

- multi-rate hardware-in-the-loop control with sensor jitter and learned
  actuation latency;
- SSA reaction networks with state-dependent propensities, delayed
  queues, and matching/resource constraints;
- fluid-structure interaction with state-dependent deformation maps and
  codimension-1 interfaces;
- coalescent and transmission-tree models with posterior domain
  realizations;
- vector-clock distributed systems with causal-frontier reads;
- calendar and DST folds with partial/many-valued correspondences;
- pseudotime as a learned correspondence versus a declared biological
  domain;
- branching-time MDPs with branch identity as a discrete latent;
- mass spectrometry with calibration-dependent many-to-many m/z
  correspondences;
- adaptive mesh refinement as event-mutated realization of one source
  domain;
- event cameras and market order books as supplied event occurrence
  domains;
- rolling-shutter or satellite observations with footprint/exposure
  aggregation;
- same physical clock threaded through two reusable context contracts,
  testing `SameDomain` versus explicit correspondence;
- posterior transmission tree used with different posterior-use
  policies, testing draw-axis and MAP/marginalization behavior;
- delayed event with captured-payload versus current-state firing
  semantics;
- codimension-1 fault or interface domains with one-sided trace
  operators from adjacent bulk domains;
- context contracts that are themselves generic over temporal domains;
- event-created entities that immediately instantiate per-entity domain
  family members, such as new cells with membrane domains.

---

## 13. Spec Impact Map

If accepted, this amendment likely requires changes to:

- **Glossary.** Add DomainSite, DomainFamilySite, DomainCompositionSite,
  DomainMapSite, SubLocusSite, TemporalLiftSite,
  TemporalExecutionSite, CorrespondenceSite, ObservationOperatorSite,
  EventFamilySite, OccurrenceDomainSite, OccurrenceStreamRealization,
  ScheduledOccurrenceMap, PosteriorContextSite, RealizationInstance,
  RealizationEvidence, WorkflowAxisSite.

- **Types and contracts.** Domains as generic parameters; associated
  domains/compositions/correspondences in contracts; coherence rules.

- **Geometry and locus.** Reframe old `Domain<G>` as spatial-domain
  constructor/declaration syntax.

- **State and time.** Remove implicit global time; rewrite `d`, `step`,
  `config.dt`, temporal blocks, initial conditions, and temporal
  realization/coordination.

- **Events.** Replace tick-centered firing order with event families,
  occurrence domains, temporal execution sites, event realization,
  matching, delayed events, and cascade semantics.

- **Collections and axes.** Clarify source domains vs workflow axes and
  domain families over entity domains.

- **Stochastic machinery.** Connect latent/posterior domain
  realizations to inference and posterior-use policies.

- **Integration/operators.** Domain measure, composition, projection,
  pushforward/pullback, and operator support contracts.

- **Layered e-graph.** Add Layer-3 sites for domains, domain
  realizations, correspondences, observation operators, event families,
  occurrence domains, and temporal lifts.

- **SCCs and residual extraction.** Correspondence-map obligations that
  enter ordinary SCCs, TemporalLiftSites, domain realization
  dependencies, and observation-operator residuals.

- **Workflow.** Bind domain/event/correspondence/observation realization
  and temporal execution via typed catalog handles; preserve `bind` if
  possible.

- **Backend capabilities.** Temporal/domain/correspondence/event/
  observation capabilities.

- **Realization providers.** Generalize the existing Section 37.1 provider
  machinery beyond spatial/discrete operators. Extend provider `kind`
  values and dispatch to domain, correspondence, event-family, and
  observation-operator providers; reuse the existing evidence-grade and
  run-lock machinery rather than duplicating it.

- **Anti-spec.** Add rows listed below.

- **Mocks.** Rewrite mocks to use explicit domains, temporal blocks, and
  observation/event/correspondence surfaces where relevant.

### 13.1 Elaboration Pass Order Sketch

The exact compiler pipeline is not locked, but the spec rewrite should
preserve this dependency shape:

1. Resolve modules, imports, aliases, and constructor names.
2. Create `DomainSite`s and domain-constructor uses.
3. Elaborate contract bounds and associated members.
4. Elaborate `derived domain`s and emitted `DomainMapSite`s.
5. Elaborate `DomainFamilySite`s, total-space views, and sub-loci.
6. Elaborate fields and state support domains.
7. Elaborate temporal blocks into `TemporalLiftSite`s and
   `TemporalBoundaryMapSite`s.
8. Elaborate correspondences and correspondence families.
9. Elaborate event families into `OccurrenceDomainSite`s and
   `ScheduledOccurrenceMap`s.
10. Elaborate observation operators.
11. Emit `TemporalExecutionSite`s for coordinated temporal realization.
12. Build the semantic solve graph and SCCs.
13. Build realization obligations and the run-lock dependency DAG.

Observation operators may target trajectory facets, so temporal lifts
must exist before observation targets are resolved. Temporal execution
sites depend on both lifts and event-family sites.

---

## 14. Decision Ledger

This section ranks the remaining work. The goal is to keep architectural
blockers separate from syntax choices and stdlib package design.

### 14.1 Must Resolve Before Rewriting `spec.md`

1. **Associated members in contracts.**
   Decision so far: domain contexts are ordinary contracts extended with
   associated domains, associated domain maps/projections, and
   associated correspondences. Required associated domains are supplied
   by implementations; explicitly `derived domain` members create
   context-owned generated identities. `SameDomain<A, B>` is compile-time
   identity only. Rejected alternatives: a separate `domainreq` DSL;
   structural equality between associated domains; implicit generated
   compositions.
   Minimal syntax to settle: `associated domain Time: ODETime<Second>`,
   `derived domain Trajectory = Product<Space, Time>`,
   `associated map TimeOfTrajectory: Projection<Trajectory, Time>`,
   `associated correspondence ObsToModelTime: ...`, and
   `type SimWorld impl LeafWorld { ... }`.
   IR to settle: `AssociatedDomainMemberSite`,
   `DerivedAssociatedDomainSite`,
   `AssociatedDomainMapMemberSite`,
   `AssociatedCorrespondenceMemberSite`.
   Diagnostics to settle: missing associated member, bound mismatch,
   associated-domain identity ambiguity, and unsatisfied associated
   correspondence.

2. **Domain families and event-created instances.**
   Decision so far: runtime-indexed domains are allowed only through
   declared `DomainFamilySite`s or event-generated family obligations.
   The total-space/bundle form is preferred for fields spanning all
   members; `Family[id]` is a restriction/view. Entity-local access must
   go through an indexed-entity/family-instance binding such as
   `IndexedEntity<Leaves>`. Need to settle: exact binding syntax,
   instantiation timing, identity across runs, event-created family
   instances, inheritance from `CapacityMask`/`EventReplan` dynamic
   topology phases, and `hypha explain` display.

3. **Correspondence canonical syntax and operation typing.**
   Decision so far: `correspondence` is the canonical source and IR
   concept; loose `couple A between B, C` should not be canonical.
   Witness `leg`s define the span; named maps/functions support typed
   operations. Need to settle: exact witness declaration syntax, leg/map
   punctuation, operation names, partial/many-to-many/posterior
   correspondences, and exact evidence fields for supported operations.

4. **Correspondence obligations and SCC placement.**
   Decision so far: correspondence declarations emit obligation sites
   for state-dependent leg/map and operation slots. Fulfillment
   relations enter the ordinary SCC graph; correspondence-coupled SCCs
   are ordinary SCCs, not a separate solver class. Need to settle:
   obligation keys, default cardinality, partial fulfillment,
   objective-term interaction, and validation/evidence grades for
   correspondence facts.

5. **TemporalLiftSite ownership.**
   Decision so far: a temporal block creates or references a
   `TemporalLiftSite` that owns the trajectory facet. Past/delayed reads
   must target the trajectory facet explicitly. Scalar states use
   internal `UnitLocus` support. Initial/boundary/conservation semantics
   attach to the lift via temporal boundary maps. `initial` must be
   anchored with `initial over T` or nested inside `temporal over T`.
   One state path has at most one owning `TemporalLiftSite` in v2;
   multi-time evolution is a hard error. Need to settle: exact catalog
   facet names, observation target syntax, conservation loci, and
   reconciliation with `y[t]`/`y[t+1]` ground terms.

6. **Temporal scope and `over`.**
   Decision so far: `temporal over T` authorizes concise `d` and `step`,
   not arbitrary domain-sensitive defaults. Event families must name
   their home temporal domain. Coordinate-level `over z` should be
   retired in favor of domain or sub-locus anchors. Need to settle:
   exact migration syntax for old spatial coordinate anchors and whether
   a separate `using domain S` sugar is worth adding for repeated
   operator anchors.

7. **Event occurrence domains and payload schemas.**
   Decision so far: every event family elaborates to an occurrence
   domain with identity, home temporal domain, occurrence-time
   map/correspondence, and typed payload; realizations produce
   occurrence streams. Delayed scheduling emits `ScheduledOccurrenceMap`.
   Need to settle: source syntax for `occurrence` blocks, replayed trace
   binding, observed event streams, delayed queue payload capture,
   stale/cancellation/merge semantics, and event-created entities.

8. **Temporal execution coordination.**
   Decision so far: a `TemporalExecutionSite` coordinates temporal lifts,
   event families, delayed occurrence maps, root detection, snapshots,
   and conservation obligations over a temporal domain or temporal SCC.
   Individually valid temporal-domain and event-family realizations can
   be rejected when no coordinator can execute them coherently. Need to
   settle: exact emitted site keys, compatibility diagnostics, and the
   workflow binding surface for hybrid execution providers.

9. **ObservationOperatorSite contract shape.**
   Decision so far: observation operators are source-visible measurement
   models, trivial path observations elaborate to stdlib operators, and
   workflow chooses observe/fit/evidence consumption mode.
   Need to settle: declaration syntax, contract shape, data-coordinate
   schema, noise/missingness/censoring syntax, target facet syntax,
   multiple correspondence/domain-map references, and data-coordinate
   diagnostics.

10. **Realization contracts, providers, and posterior use.**
   Decision so far: realizations are workflow constructors satisfying
   source-side contracts; provider machinery extends Section 37.1 rather
   than duplicating it; realization dependencies form a run-locked DAG
   distinct from the semantic solve graph. Priors over declared domain,
   correspondence, and event-family sites are typed realization sources.
   Run locks pin realization/provider versions for replay.
   Need to settle: canonical `Realization<D>` contract, event-family and
   correspondence realization contracts, provider TOML `kind`
   extensions, exact run-lock schema, and posterior context/draw-axis
   semantics.

11. **Workflow axes and source leakage.**
    Decision so far: workflow axes lift, batch, replicate, and aggregate
    runs but cannot become source-readable by workflow action. Need to
    settle: catalog representation for replicate/sweep/scenario axes,
    lifting semantics, aggregate outputs, and diagnostics such as
    `workflow_axis_read_by_source_law`.

12. **Domain identity/refactoring policy.**
    Decision so far: no general `identify domain` as routine modeling
    syntax. Prefer import/re-export, domain alias, generic/context
    passing, or exact `IdentityMap` correspondence for distinct but
    isomorphic loci. Dynamic domain fusion is disallowed; use a superset
    domain, phase field, indicator, or dynamic topology within one family.
    No restricted post-declaration identity witness is included in v2.
    Need to settle: `hypha check --domain-audit` behavior.

13. **Existing surface reconciliation.**
    Need to settle how this amendment rewrites or reframes:
    old `Domain<G>`; `bind_topology`; `config.dt`; `y[t]` ground terms;
    existing `identify`; `convert`; stochastic "coupling" terminology;
    `ProcessPrior<I, V>`; spatial `over z`; Layer-3 provider sites; and
    observation/fit workflow verbs.

### 14.2 Can Resolve During Main Spec Rewrite

- exact punctuation for associated members and `impl` bodies;
- exact causal-frontier and branch-local read syntax;
- exact names of initial stdlib capability contracts;
- exact Python API names if typed `workflow.bind` handles domains,
  correspondences, event families, and observation operators;
- exact `hypha explain` rendering for domain maps, trajectory facets,
  and family instances.

Candidate domain capabilities include `TotalOrder`, `PartialOrder`,
`BranchOrder`, `DurationMeasure<U>`, `HasMetric`, `HasMeasure`,
`HasBoundary`, `HasAdjacency`, `StaticTopology`, `DynamicTopology`,
`SupportsDerivative`, `SupportsStep`, `SupportsDelay`,
`SupportsHazard`, `SupportsFrontierSnapshot`, `SupportsBranchSnapshot`,
`CalendarSuccessor`, `SupportsLaplacian`, and
`SupportsIntegration`.

Candidate correspondence capabilities include `SupportsRead`,
`SupportsPullback`, `SupportsPushforward`, `SupportsSnapshot`,
`SupportsInvert`, `Smooth(k)`, `UnitConsistent`, `MeasurePreserving`,
`OrderPreserving`, `Causal`, `Invertible`, `OrientationConsistent`, and
`SnapshotConsistent`.

### 14.3 Can Follow In Stdlib Or Provider Design

- complete calendar and timezone constructors;
- complete mesh and adaptive-refinement provider families;
- complete observation noise/missingness model catalog;
- all backend capability names;
- all domain-specific event realizers;
- provider-specific validation protocols;
- rich atlas/manifold conveniences beyond the minimal domain-map model.

### 14.4 Diagnostic Candidates

Likely new diagnostics:

- `E_DOMAIN_MISSING_TEMPORAL_LIFT`: `d` or `step` appears without
  `temporal over T` or explicit `over = T`;
- `E_DOMAIN_AMBIGUOUS_CORRESPONDENCE`: a cross-domain read lacks a
  required `via` or operation;
- `E_DOMAIN_UNFULFILLED_MAP`: a state-dependent correspondence leg/map
  lacks a fulfilling relation;
- `E_DOMAIN_INVALID_SUPPORT_READ`: a past/future read targets the
  instantaneous support facet instead of the trajectory facet;
- `E_DOMAIN_DOUBLE_EVOLUTION`: one state path gets multiple owning
  temporal lifts;
- `E_DOMAIN_INITIAL_UNANCHORED`: an `initial` block over lifted state
  lacks a temporal anchor;
- `E_DOMAIN_FAMILY_INDEX_UNBOUND`: a family member is referenced without
  a bound index-domain locator;
- `E_DOMAIN_CORRESPONDENCE_FAMILY_INDEX_UNBOUND`: a correspondence
  family uses an unbound index variable;
- `E_DOMAIN_SILENT_MAP_COMPOSITION_REJECTED`: a read would require an
  unstated chain of domain maps/correspondences;
- `E_DOMAIN_CONVERT_USED_FOR_ALIGNMENT`: `convert` is used where a
  domain map or correspondence is required;
- `E_DOMAIN_TEMPORAL_EXECUTION_INCOMPATIBLE`: temporal-domain,
  event-family, delayed-queue, root-finding, or snapshot realizations
  cannot be coordinated;
- `E_DOMAIN_HAZARD_REQUIRES_MEASURE`: hazard/propensity is declared on
  a temporal domain without the needed measure/probability capability;
- `E_DOMAIN_POSTERIOR_CONTEXT_REQUIRED`: posterior sites that must be
  sampled or marginalized jointly were configured independently;
- `E_DOMAIN_POSTERIOR_DRAW_AXIS_MISSING`: sampled posterior use lacks a
  workflow draw axis;
- `E_DOMAIN_SUBLOCUS_REQUIRES_PARENT`: a sub-locus lacks an explicit
  parent domain;
- `E_DOMAIN_BOUNDARY_MAP_UNSUPPORTED`: `initial` or boundary semantics
  require a boundary map the temporal domain does not provide;
- `E_DOMAIN_OBSERVATION_CONSUMPTION_MISSING`: workflow binds data to an
  observation operator without selecting a supported consumption mode;
- `E_DOMAIN_MODULE_CORRESPONDENCE_CAPTURES_INSTANCE_STATE`: a
  module-scope correspondence depends on instance-local state.

### 14.5 Rewrite Catalog Candidates

The rewrite catalog likely needs a domain-transport group. Candidate
rules:

- identity pullback: `pullback(x, IdentityMap)` rewrites to `x`;
- projection pullback: `pullback(x, Product.proj_left)` rewrites to a
  compiler-proven broadcast over the right axis when the value kind
  permits it;
- start-boundary evaluation: evaluating a trajectory at a
  `TemporalLiftSite` start-boundary map rewrites to the corresponding
  initial facet;
- occurrence payload resolution: querying a captured payload at the
  exact occurrence coordinate rewrites to the captured value;
- domain-map identity collapse: composed exact domain maps with an
  explicit identity proof may collapse, but only when the composition
  site is named.

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
| Hidden per-entity correspondences | Correspondence families | Instance-specific correspondences need identities |
| Module-scope correspondence capturing instance fields | Correspondence family or associated correspondence | Instance-local state needs an explicit identity parameter |
| Separate `domainreq` language | Ordinary contracts and associated members | Avoid parallel type system |
| Workflow axes as source domains by default | Catalog workflow axes; explicit source domains only when modeled | Replicates and sweeps are not world mechanisms |
| Silent deterministic use of posterior domains | Explicit posterior-use policy | Uncertain loci need explicit consumption semantics |
| Ambient lexical inference from imports or unique candidates | Explicit lexical scopes that name the domain or correspondence | Concision should be scoped, not ambient |
| Casual cartesian `S x T` as hidden product assumption | Named composition constructor with emitted maps | Product, bundle, quotient, and tree-indexed loci differ |
| `over` inheriting unrelated temporal/domain context | Domain anchor named at each relevant syntax position | Avoid recreating hidden global context under a nicer keyword |
| General `identify domain` as routine modeling | Import/re-export, alias, generic passing, or exact identity correspondence | Nominal identity should not be easy to erase |
| Event occurrence as anonymous runtime log | `OccurrenceDomainSite` with payload schema | Event histories are model-addressable loci |
| Observation target as implicit trajectory facet | Explicit instantaneous vs trajectory target facet | Avoid ambiguity introduced by `TemporalLiftSite` |
| Workflow axis promotion by workflow call | Source-declared domain plus workflow binding | Workflow must not create source-readable semantics |
| Product projection treated as correspondence | Emitted `DomainMapSite` / projection | Exact composition maps differ from evidence-bearing alignments |
| `ProcessPrior` creates hidden domain | Prior over a declared catalog site | Structured stochastic values should not silently become loci |
| Correspondence property declared without evidence mode | Required/proven/validated/posterior-supported property facts | Avoid provider trust leakage |
| Family instance from arbitrary runtime value | Domain family indexed only by declared index domain | Prevent hidden runtime identity generation |
| Same-constructor domains automatically composable as one structure | Explicit composition declaration | Same shape does not create shared locus |
| Domain parameters covariant or contravariant | Invariant by default | Same-capability domains are not interchangeable |
| Associated domain members implicitly equal across bounds | `SameDomain<A, B>` or explicit correspondence | Context bounds do not imply identity |
| Multiple temporal blocks silently evolve the same state | Compile error in v2; model separate state facets and explicit correspondence/operator | Avoid accidental double ownership |
| Realization without dependency DAG | Run-lock records full realization dependency DAG | Reproducibility needs the whole stack |
| Realization sources as Python classes mirroring `.myco` types | Workflow constructors satisfying source-side contracts | Python remains workflow, not source semantics |
| `SameDomain` satisfied by provider evidence | Compile-time identity constraint only | Identity is not empirical alignment |
| Derived associated domain with implicit identity rule | Required `associated domain` or explicit `derived domain` | Prevent hidden generativity in contracts |
| Realization DAG used as solve graph | Separate run-lock dependency DAG from semantic SCC graph | State-dependent maps can be cyclic equations |
| Event schedule without occurrence correspondence | `ScheduledOccurrenceMap` between occurrence domains | Delayed events need provenance and payload identity |
| Occurrence domain equals realized event log | `OccurrenceDomainSite` plus `OccurrenceStreamRealization` | Source locus and run realization are distinct |
| Observation operator with implicit observe/fit meaning | Operator exposes likelihood/residual/evidence forms; workflow chooses consumption | Measurement and training objective differ |
| Witness map and transport operation conflated | Witness legs plus typed supported operations | Partial and many-to-many correspondences need span semantics |
| Scalar temporal state has no support rule | Internal `UnitLocus`/singleton support normalized to time trajectory | Avoid scalar special cases in lifts |
| Dynamic fusion of nominal domains | Phase field, indicator, dynamic topology, or superset domain | Nominal identities cannot merge at runtime |
| Directly querying past states on support domains | Explicitly query the trajectory facet | Delayed reads target trajectories, not instantaneous support |
| `initial` block as literal assignment to instantaneous state | Constraint on the TemporalLift start-boundary map | Evolving variables are trajectories |
| `DomainMapSite` requiring manual transport boilerplate | Compiler-generated operation capabilities for exact domain maps | Built-in projections should be usable without hand-written correspondences |
| Domain alias as new declaration | `domain alias A = B` creates no new `DomainSite` | Aliases preserve identity |
| Identity map used as alias | Alias/import for true sameness; `IdentityMap` only for distinct but isomorphic loci | Keep equality and alignment separate |
| Event occurrence accessed without payload typing | `OccurrenceDomainSite` with declared identity and payload schema | Occurrences are typed loci |
| Event family declared without home temporal domain | `event family E over T` | Event laws need a temporal substrate |
| Realization parameters declared in `.myco` source | Workflow source object configuration | Source declares obligations; workflow chooses execution policy |
| `ContinuousTime` constructor used without unit parameter | Explicit unit parameter such as `ContinuousTime<Second>` | Constructors should not hide structural parameters |
| Per-event-family realizers as implicit scheduler | Explicit `TemporalExecutionSite` / coordinator realization | Hybrid ODE + SSA + delayed events need one coherent advance discipline |
| Hazard/propensity without temporal reference measure | `HazardTime<U>` or discrete event-probability capability | Propensities need units and snapshot semantics |
| Silent composition of domain maps and correspondences | Explicit named composition/map/correspondence operation | Prevent hidden transitive alignment |
| Independent posterior sampling per site by default | `PosteriorContextSite` and draw-axis policy | Joint inferred structures must preserve dependence |
| `convert` used for domain alignment | Domain map or correspondence operation | Value conversion is not locus correspondence |
| Sub-locus treated as implicit domain | Declared `SubLocusSite` or explicit `DomainSite` | Boundaries, regions, and selectors need semantics without hidden identity |
| Observation operator without consumption mode | Operator exposes observe/fit/evidence modes; workflow chooses | Measurement semantics and objective semantics differ |
| Domain projection confused with constraint projection | `DomainMapSite(kind = projection)` | Structural maps are not solver projections |
| Approximate correspondence operation without ledger entry | Approximation/evidence ledger entry for approximate, sampled, or posterior transport | Correspondence-mediated approximations need provenance |
| Unanchored `initial` over lifted state | `initial over T` or `initial` nested inside `temporal over T` | Initialization must choose a temporal boundary map |

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
5. Are domain families and correspondence families the right answer for
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
11. Does the decision ledger correctly separate blockers from syntax and
   stdlib design?
12. Which remaining questions above must be settled before editing
   `spec.md`?
13. What anti-spec rows are missing?
14. Which existing `spec.md` sections become silently wrong under this
   amendment?
15. What new diagnostic IDs should be added for domain identity,
   correspondence, temporal lift, event occurrence, observation
   operator, and realization errors?
16. Which rewrite-catalog rules need new groups or extensions?
17. Are there realization combinations the architecture admits at bind
   time but cannot coherently execute, especially combinations of
   temporal-domain, event-family, correspondence, and temporal-execution
   realizations?

---

## 17. Proposed Work Sequence

1. Circulate this amendment to external reviewers.
2. Resolve the open questions in Section 14.
3. Publish a final locked invariants list with cross-references to the
   amendment sections that enforce each invariant.
4. Produce an amendment-to-spec mapping that says which canonical
   sections each amendment section modifies, replaces, or adds.
5. Run local subagents over `spec.md`, `anti_spec.md`, `soul.md`, and
   mocks with the locked amendment.
6. Produce a comprehensive spec-edit plan.
7. Execute atomic spec/anti-spec/soul/mock edits.
8. Re-run review on the rewritten canonical spec.

No canonical spec rewrite should happen until the amendment's core
surface is locked.
