# Myco v2.1 — Backend Abstraction Design Report (LOCKED)

**Date:** 2026-04-20 (stub created); locked 2026-04-24
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet
**Status:** LOCKED. Backend abstraction is committed into the
canonical spec as Part V: small `CoreBackend` plus capability
profiles; hybrid AD; explicit capability mismatch policies; whole-SCC
Tier C PPL handoff; opaque-callable runtime semantics; trait
versioning; no primary backend; and a semantics-complete CPU reference
backend as the first conformance target.

---

## 1. Why this chunk exists

Multiple v2.1 design threads independently reached for the same
pattern — specify structural / mathematical claims in `.myco`, route
concrete execution through a workflow-selected backend plugin:

- **Tier C distributional backend** (chunk 04 CC4 resolution; was
  blocker B3) — Tier C distributions (copulas, opaque couplings)
  route to a PPL backend for inference. Backend returns samples,
  gradients, MCMC traces. Compiler / backend boundary undesigned.
- **Numerical linear algebra backend** (chunk 05) — matrix
  decompositions, solves, `condest`, specialized structural-subtype
  solvers route to a linear-algebra backend (cuBLAS, MKL, LAPACK,
  Eigen, candle, burn, etc.). Chunk 05 deliberately does not design
  this — it lives here.
- **GPU lowering for collections / tensor ops** — chunk 02's
  collection aggregation lowering and chunk 05's matrix ops both
  target device kernels. The backend is the same concern.
- **Opaque callables / neural controllers** — workflow
  `bind(path, Controller(...))` attaches Python/PyTorch/JAX
  functions as opaque factors in the graph. The callable runs in
  some backend's runtime; gradient flow through it depends on
  backend AD.
- **Autodiff ownership** — forward- and reverse-mode AD can be
  Myco-owned (symbolic `deriv` through the graph) or backend-owned
  (JAX `grad`, PyTorch autograd, burn autodiff). Cross-cuts all the
  above.

Riley's framing that motivated the split (2026-04-20): *"maybe
backend model needs its own chunk report and we keep matrix stuff
scoped to matrix stuff only? i'm sure backend model touches more
things than just matrix?"* — correct: it touches at least five
distinct design threads. Designing it in-place in chunk 05 would
have scope-crept chunk 05 badly and left the PPL backend problem
unsolved anyway.

**Pattern committed:** a burn-style backend trait surface with a
small mandatory core and advertised capabilities. Every
backend-dependent op routes through the trait. Workflow selects a
backend at run-time configuration. `.myco` stays backend-agnostic.

Riley's confirmation: *"we'll be able to handle this backend-
agnostic, sort of like how burn in rust is impl backend for
different gpu apis? i like your proposals overall."* (2026-04-20)

---

## 2. Current state in v2.1

The audit found the backend concern scattered across several earlier
design threads:

- **PPL backend** — chunk 04 §11 CC4 resolution commits Tier C
  distributions to "route to a PPL backend" but leaves the protocol
  undesigned. Blocker B3 in chunk 04 §11 tracked this; B3 absorbed
  here.
- **Enzyme + Rust** — `v2.1_in_progress.md:1789` mentions "long-term
  Enzyme + Rust path for LLVM-level AD" as an implementation
  direction. No interface lock.
- **`Controller` source binding** — older notes around
  `v2.1_in_progress.md:867, 880-909` attach opaque callables. No
  statement about which backend runs them or how gradients flow back.
- **GPU lowering for collections** — `02_collections_iteration_
  report.md:233-242` has a lowering table from collection ops to
  device primitives, but without a backend trait the lowering is
  target-hardcoded.
- **`condition_of` Level III** — chunk 04 §11 O2.4 Level III commits
  to runtime `condest` on assembled matrices; `condest` is a
  backend primitive.

Part V of `spec_new.md` is now the canonical surface that ties these
together.

---

## 3. Scope: what this chunk owns, what it defers

### In scope

1. **The `Backend` trait shape.** What methods every backend must
   implement (minimum API) vs. which are capability-advertised
   (optional, with fallback or hard-error policy).
2. **Capability advertising and fallback.** How a backend declares
   what it supports (SVD? sparse solves? Cholesky-specialized solve
   for PosDef?). How the compiler reacts when a requested op isn't
   advertised.
3. **AD ownership.** Resolved hybrid boundary: Myco-owned symbolic /
   algorithmic derivative structure plus backend-owned runtime AD.
   Has downstream consequences for every backend-aware op.
4. **PPL backend protocol.** Concrete handoff for Tier C
   distributional inference: what the compiler emits (envelope
   metadata, structural declarations, coupling annotations,
   log-density assembly recipe); what backend returns (samples,
   gradients, MCMC traces, diagnostic metadata); serialization of
   stochastic e-classes.
5. **Opaque callable protocol.** How workflow
   `bind(path, Controller(...))` attaches callables; how gradient
   flow works; whether the callable runs in the same backend as the
   rest of the graph or in a separate one.
6. **Mixed-backend policy.** Single-backend-per-run (simpler) vs.
   SCC-level or op-level backend dispatch (more flexible, more
   machinery).
7. **Versioning.** How backends advertise API version; how `.myco`
   files survive backend evolution.

### Out of scope (pushed to other chunks)

- **Matrix/tensor type system** (chunk 05) — what a tensor *is* as
  a type; what linear-algebra primitives exist in the language.
- **Distributional type system** (chunk 04) — the `Distribution<U>`
  contract, Z-group rewrites, envelope propagation.
- **Collection iteration semantics** (chunk 02) — aggregation rules,
  validity masks.
- **Kernel / integral semantics** (chunk 03) — the math of kernels.
- **E-graph mechanics** (chunk 04) — equality substrate, extraction.

The distinction: *what it is* lives in the other chunks; *what it
executes as* lives here.

---

## 4. Design surface — what needs to be decided

### 4.1 Backend trait minimum API — RESOLVED

Decision (2026-04-24): v2.1 uses a small mandatory `CoreBackend`
trait plus advertised capabilities and capability profiles.

Every backend must satisfy `CoreBackend`:

- Run identity, trait version reporting, and backend version
  reporting.
- Capability inspection and plan-binding diagnostics.
- Deterministic seed handling.
- Dense tensor allocation / handles.
- Elementwise arithmetic, broadcast, reductions, reshape / view /
  transpose, dense matrix multiplication, and ordinary scalar math.

This core is deliberately small. It is enough to run a basic
deterministic numerical plan and to say precisely why a richer plan
cannot bind. Cholesky, SVD, eigendecomposition, sparse kernels,
iterative solvers, PPL inference modes, runtime AD modes, complex
numbers, dynamic-keyed axes, host interop, and opaque-callable
gradients are not mandatory methods; they are advertised
capabilities.

Capability profiles are named bundles with `requires`, `provides`,
and `implies` rules. They are implementation-surface declarations,
not `.myco` source contracts or supercontracts.

```text
CapabilityProfile LinearAlgebraBasic
  requires CoreBackend
  provides solve, solve_triangular

CapabilityProfile LinearAlgebraDecompositions
  requires LinearAlgebraBasic
  provides cholesky, qr, svd, eigen

CapabilityProfile PPLHMC
  requires CoreBackend, RuntimeADReverse
  provides hamiltonian_monte_carlo, mcmc_diagnostics

CapabilityProfile OpaqueCallableAD
  requires CoreBackend, opaque_callable_runtime, RuntimeADReverse
  provides opaque_callable_ad, controller_gradients
```

Optionality is represented as advertised evidence. The compiler
constructs a required-capability set for a plan or SCC; the selected
backend either satisfies it, the workflow explicitly authorizes a
capability-mismatch policy, or binding fails.

### 4.2 Capability advertising

Backends advertise optional capabilities and capability profiles.
The compiler consults the advertised set at lowering time:

- Structural-subtype-specialized solvers (Cholesky for PosDef,
  triangular solve for triangular, etc.)
- Sparse linear algebra (sparse solves, sparse matmul)
- Iterative solvers (Krylov, multigrid, preconditioned)
- SVD, eigendecompositions
- `condest` (condition number estimation)
- Autodiff modes (forward, reverse, higher-order)

Fallback policy options:
- **Error** — backend lacks op → compile error, user switches backend.
- **Host fallback** — backend lacks op → route through a host-side
  NumPy/LAPACK fallback with serialization overhead.
- **Emulate** — synthesize the op from available primitives (e.g.,
  Cholesky from QR; slower but works).

Workflow knob: `run.config.backend.fallback = "error" | "host" |
"emulate"`.

Default policy is `error`, matching the canonical spec (§31.1). That
keeps capability mismatch from becoming a silent semantic or
performance fallback. `host` and `emulate` are explicit workflow
authorizations.

### 4.3 AD ownership — RESOLVED: hybrid boundary

Decision (2026-04-24): Myco uses a hybrid AD boundary. Myco owns
visible symbolic / algorithmic derivative structure; backends own
runtime AD over emitted kernels and opaque callables. Runtime AD may
satisfy execution needs for training and inference, but it does not
grant symbolic derivative facts unless the compiler independently
derives the same structure or an audited backend capability
explicitly certifies the relevant derivative fact.

The fork considered:

**Option A — Myco owns AD.** Symbolic `deriv` extended to every
tensor operation; Myco emits forward + backward pass against the
backend's primitive tensor ops.

Pros:
- Analysis-aware: condition bounds, envelope propagation, symbolic
  simplification all survive through gradients.
- Level II algorithmic `condition_of` (Jacobian operator norms) works
  through gradient computation.
- Backend trait stays small (only forward primitives required).

Cons:
- Enormous implementation surface — every decomposition needs a
  hand-derived adjoint.
- Matching state-of-the-art AD (XLA JIT, JAX `grad`) without
  reimplementing those systems is a multi-year undertaking.

**Option B — Delegate to backend AD.** Myco emits the forward graph;
backend handles backward pass via its native AD (JAX `grad`, PyTorch
autograd, burn autodiff, Enzyme).

Pros:
- Massive implementation savings — leverage existing mature AD.
- Performance parity with backend-native models.
- Enzyme-via-LLVM path already mentioned as long-term direction
  (`v2.1_in_progress.md:1789`).

Cons:
- Gradient quantities opaque to Myco's analysis — condition-number
  estimation can't see through them.
- Level II algorithmic bounds become fuzzy (Jacobian is computed
  numerically by backend, not symbolically by compiler).
- Envelope propagation through gradients requires backend
  cooperation or is dropped.

**Option C — Hybrid.** Myco owns AD for compile-time analysis
(symbolic `deriv` for Level I/II condition bounds, envelope
propagation, closure-policy ranking); backend owns AD for runtime
execution (actual gradient values for training / inference). Both
must agree mathematically but operate in different regimes.

Pros:
- Analysis stays rigorous; execution stays fast.
- Same pattern as everything else in Myco (compiler has a symbolic
  view for proving things, runtime has a concrete view for
  executing).

Cons:
- Two AD systems to maintain, with a consistency obligation between
  them.
- User-facing story more complex ("why do I get two different
  gradients?" — they won't differ in math, but implementation paths
  diverge).

Selected: Option C. This matches Myco's broader
symbolic-analysis-plus-concrete-execution pattern. The compiler keeps
the derivative structure it can inspect for conditioning, envelopes,
rewrite eligibility, diagnostics, and provenance. The backend handles
runtime gradient values where mature AD systems are the right
execution machinery. Opaque runtime gradients stay opaque to the
symbolic layers unless separately certified.

### 4.4 PPL backend protocol (was B3) — RESOLVED

Decision (2026-04-24): Tier C uses whole-stochastic-SCC handoff after
Tier A exact rewrites and authorized Tier B approximate rewrites have
run to exhaustion. The compiler serializes each unresolved stochastic
SCC as one `InferenceTask`; the backend runs inference and returns an
`InferenceResult`. Per-factor handoff is not the v2.1 protocol.

Concrete handoff for Tier C distributional inference:

- **Compiler emits:**
  - Stochastic SCC identity and stochastic e-class identities
  - Latent nodes, observed nodes / data, and visible deterministic
    dependency terms
  - Envelope metadata (layer-2 facts: family, parameters, shape,
    bounds)
  - Support / refinement constraints and capability requirements
  - Structural declarations / coupling metadata when visible
    (independence claims, copula structure, joint declarations)
  - Log-density assembly recipe for the whole unresolved SCC
  - Requested inference kind (`hmc`, `nuts`, `vi`, `importance`,
    backend-specific extensions behind capability flags)
- **Backend returns:**
  - Posterior draws / sample values with shape and provenance
    metadata
  - Optional log-density evaluations
  - Gradient estimates where requested (score function,
    reparameterized, or via
    backend AD per §4.3)
  - MCMC traces (chains, acceptance stats, convergence diagnostics)
  - Diagnostic metadata (effective sample size, R-hat, divergence
    warnings)
- **Returned-value semantics:** returned samples are opaque draws or
  empirical summaries with provenance. They are not new parametric
  envelope facts and do not create observation-style equalities.
- **Framework-specific adapters:** NumPyro-style, Pyro-style,
  Turing.jl-style, Stan-style, custom. Each wraps the same protocol
  differently.

Whole-SCC handoff is what lets HMC / NUTS / VI see shared latents,
posterior geometry, support constraints, and deterministic transforms.
Per-factor handoff would hide the joint problem from the backend and
is retired for v2.1.

### 4.5 Opaque callable protocol — RESOLVED

Decision (2026-04-24): `Controller` sources execute in the selected
run backend context by default. A callable may be used as a fixed
opaque source with `opaque_callable_runtime`; it may participate in
training gradients only when the callable and backend jointly
advertise differentiability / AD support. Silent gradient stops are
not allowed.

Committed semantics:

- **Runtime context.** Opaque callables run in the same backend
  context as the rest of the run. Cross-backend callable interop is
  not a v2.1 guarantee; workflows that need it isolate the callable
  into a separate run and pass outputs back as sources.
- **Fixed opaque callable.** A non-trainable controller may execute
  without AD when it satisfies its input / output contracts and the
  backend advertises `opaque_callable_runtime`.
- **Trainable differentiable callable.** A trainable controller
  requires a differentiable callable contract, `opaque_callable_ad`,
  and a compatible runtime AD profile such as `RuntimeADReverse`.
  The callable must live in the selected backend's AD frame.
- **Required gradient path.** If a training objective requires
  gradients through a controller and the callable or backend cannot
  provide them, workflow composition errors with a differentiability
  / capability diagnostic.
- **Explicit gradient stop.** A workflow may explicitly mark the
  controller boundary as a gradient stop. Then the controller may
  influence downstream values, but its internal parameters are not
  learned in the current run and the stop is recorded in gradient
  provenance.

This keeps fixed heuristics and non-differentiable services usable
without letting an accidental black box silently sever training.

### 4.6 Mixed-backend policy

Options:

- **Single-backend-per-run.** Workflow sets one backend; everything
  runs there. Simplest.
- **SCC-level dispatch.** Different SCCs can be annotated with
  different backends (e.g., sparse linear solve on CPU, dense math
  on GPU). Requires SCC-boundary data movement.
- **Op-level dispatch.** Fine-grained per-op backend selection.
  Maximum flexibility; maximum complexity.

Lean: v2.1 commits to single-backend-per-run. SCC-level is
v2.2. Op-level probably never ships.

### 4.7 Backend versioning

`.myco` files are long-lived world-claims; backends evolve rapidly.

- Backends advertise semantic version.
- `run.config.backend` includes version pin option.
- Compiler warns on major-version mismatch; errors on incompatible
  trait-surface change.

Probably a small design item. Worth committing to a versioning
policy early, not late.

---

## 5. Interactions with other chunks

- **Chunk 02 (collections).** Aggregation ops lower to backend
  kernels; masked iteration interacts with backend shape-handling
  (static vs dynamic shapes across JAX / PyTorch / burn).
- **Chunk 03 (kernels).** Kernel evaluation lowers through the
  backend; low-rank kernel approximations (K3: SVD, Nyström,
  random Fourier features) require backend SVD capability.
- **Chunk 04 (e-graph / distributions).** CC4 Tier C → backend;
  Level III `condition_of` → backend `condest`; stochastic
  inference → backend PPL adapter. B3 absorbed into this chunk.
- **Chunk 05 (matrices).** All linear-algebra primitives in
  chunk 05's stdlib list (§4) lower through the backend. Chunk 05
  explicitly defers backend-specific choices here.
- **Spec `Controller` source section.** Opaque callable protocol
  (§4.5 above).
- **Workflow surface.** `run.config.backend`, capability probing,
  fallback policy, version pinning — all new workflow verbs.

---

## 6. Downstream unblocks

With this chunk locked:

- Chunk 05's numerical primitives have a concrete target to lower to.
- Chunk 04's Tier C distributional inference has a concrete protocol.
- `condition_of` Level III has a concrete `condest` target.
- `Controller` source gradient-flow semantics lock.
- GPU lowering for collections and matrices unifies under one path.
- Enzyme-via-LLVM direction (from `v2.1_in_progress.md:1789`) can
  be framed as "one possible backend implementation," not a
  committed architecture.
- The first conformance implementation target is clear: a
  semantics-complete CPU reference backend.

---

## 7. Final v2.1 commitment

1. Myco targets a backend trait surface, not a privileged runtime.
   The mandatory surface is `CoreBackend`; richer functionality is
   advertised through capabilities and capability profiles.
2. Capability mismatch defaults to `error`. `host` and `emulate` are
   explicit workflow policies, never silent fallbacks.
3. AD has a hybrid boundary: Myco owns visible symbolic /
   algorithmic derivative structure; backends own runtime AD over
   emitted kernels and opaque callables.
4. Tier C hands each unresolved stochastic SCC to the backend as an
   `InferenceTask`, after Tier A exact rewrites and authorized Tier B
   approximations have run.
5. Opaque callables run in the selected backend context by default.
   Fixed callables require runtime support; trainable callables
   additionally require explicit opaque-callable AD support. Gradient
   stops are workflow declarations, not inferred repairs.
6. A run selects one backend. Future SCC-level mixed-backend
   execution is a follow-on item, not v2.1 semantics.
7. Myco versions the trait surface; backend implementations advertise
   compatible trait versions; plan cache keys include backend
   identity.
8. No backend is primary. JAX-, PyTorch-, Burn-, GPU-, PPL-oriented,
   Rust CPU, and reference CPU implementations are peer backends
   against the same trait.
9. The first conformance target is a semantics-complete CPU reference
   backend: Python-hosted in the workflow layer, CPU-executed,
   vectorized through NumPy / SciPy where semantics allow, and
   explicit about slower reference paths where they are required.
10. Remaining backend work is implementation-facing spelling: exact
   PPL message schema, inference-kind enumeration, workflow syntax
   for explicit gradient stops and capability-scoped fallback, future
   mixed-backend execution, and concrete trait method signatures.

Chunk 05 (matrices) is closed on source semantics; its primitive list
now supplies concrete lowering requirements for this chunk's backend
trait surface. Chunk 06 is closed on semantics; implementation detail
continues under backend follow-on items.

---

## 8. Closed questions (consolidated)

- **Q1.** AD ownership. RESOLVED in canonical spec §31: hybrid
  boundary. Myco owns visible symbolic / algorithmic derivative
  structure; backend owns runtime AD over emitted kernels and opaque
  callables.
- **Q2.** Minimum backend trait API vs. capability-advertised
  optional. RESOLVED 2026-04-24: small mandatory `CoreBackend`;
  richer operations are advertised capabilities / profiles (§4.1).
- **Q3.** Default fallback policy. RESOLVED in canonical spec §31.1:
  `error` by default; `host` and `emulate` require explicit workflow
  authorization.
- **Q4.** PPL backend protocol concrete form. RESOLVED 2026-04-24:
  whole unresolved stochastic SCC handoff after Tier A/B exhaustion;
  backend returns opaque draws / diagnostics, not parametric facts.
- **Q5.** Opaque callable gradient-flow semantics. RESOLVED
  2026-04-24: fixed opaque callables may run without AD; trainable
  callables require compatible opaque-callable AD capabilities;
  non-differentiable callables on required gradient paths error
  unless the workflow explicitly marks a gradient stop.
- **Q6.** Mixed-backend policy for v2.1. RESOLVED in canonical spec
  §32.1: single-backend-per-run; SCC-level cross-backend handoff is
  future work.
- **Q7.** Versioning strategy. RESOLVED in canonical spec §31.4:
  Myco versions the trait surface; backend implementations advertise
  compatible trait versions; plan cache keys include backend identity.
- **Q8.** First concrete backend to implement against. RESOLVED
  2026-04-24: semantics-complete CPU reference backend first:
  Python-hosted in the workflow layer, CPU-executed, vectorized
  through NumPy / SciPy where semantics allow, and explicit about
  slower reference paths where required. This is a conformance /
  debugging target, not a primary backend commitment; a Rust CPU
  backend remains a later performance-oriented peer implementation.
