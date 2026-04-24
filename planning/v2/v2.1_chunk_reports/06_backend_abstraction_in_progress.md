# Myco v2.1 — Backend Abstraction Design Report (IN PROGRESS — STUB)

**Date:** 2026-04-20 (stub created)
**Authors:** Riley Leff, Claude (Opus 4.7)
**Reviewers:** None yet
**Status:** STUB. Captures the scope and initial framing of backend
abstraction. Several chunks converged on "this is really one backend-
routing problem, not several." Factored here so those chunks don't all
try to solve it in-place.

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
- **Opaque callables / neural controllers** — `bind_controller`
  attaches Python/PyTorch/JAX functions as opaque factors in the
  graph. The callable runs in some backend's runtime; gradient flow
  through it depends on backend AD.
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

**Pattern to commit (direction, not detail):** burn-style
`trait Backend { type Tensor; type Distribution; fn matmul(...); fn
sample(...); ... }`. Every backend-dependent op routes through the
trait. Workflow selects a backend at run-time configuration. `.myco`
stays backend-agnostic.

Riley's confirmation: *"we'll be able to handle this backend-
agnostic, sort of like how burn in rust is impl backend for
different gpu apis? i like your proposals overall."* (2026-04-20)

---

## 2. Current state in v2.1

Nothing is formalized. Ad-hoc mentions scattered across:

- **PPL backend** — chunk 04 §11 CC4 resolution commits Tier C
  distributions to "route to a PPL backend" but leaves the protocol
  undesigned. Blocker B3 in chunk 04 §11 tracked this; B3 absorbed
  here.
- **Enzyme + Rust** — `v2.1_in_progress.md:1789` mentions "long-term
  Enzyme + Rust path for LLVM-level AD" as an implementation
  direction. No interface lock.
- **`bind_controller`** — `v2.1_in_progress.md:867, 880-909`
  attaches opaque callables. No statement about which backend runs
  them or how gradients flow back.
- **GPU lowering for collections** — `02_collections_iteration_
  report.md:233-242` has a lowering table from collection ops to
  device primitives, but without a backend trait the lowering is
  target-hardcoded.
- **`condition_of` Level III** — chunk 04 §11 O2.4 Level III commits
  to runtime `condest` on assembled matrices; `condest` is a
  backend primitive.

No single surface ties these together; this chunk is that surface.

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
5. **Opaque callable protocol.** How `bind_controller` attaches
   callables; how gradient flow works; whether the callable runs in
   the same backend as the rest of the graph or in a separate one.
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

### 4.4 PPL backend protocol (was B3)

Concrete handoff for Tier C distributional inference (chunk 04
CC4 locked Tier C routes through a backend; protocol lives here):

- **Compiler emits:**
  - Envelope metadata (layer-2 facts: family, parameters, bounds)
  - Structural declarations (joint syntax from chunk 06 B2/B4
    absorption or its successor)
  - Coupling annotations (independence claims, copula structure)
  - Log-density assembly recipe (how to build `log_pdf` from parts
    when parts span Tier A / B / C)
- **Backend returns:**
  - Sample values (with shape and provenance metadata)
  - Gradient estimates (score function, reparameterized, or via
    backend AD per §4.3)
  - MCMC traces (chains, acceptance stats, convergence diagnostics)
  - Diagnostic metadata (effective sample size, R-hat, divergence
    warnings)
- **Serialization:** how stochastic e-classes serialize to backend
  primitives; how returned values flow back into the e-graph (as
  new envelope facts? as observation-style equalities?).
- **Framework-specific adapters:** NumPyro-style, Pyro-style,
  Turing.jl-style, Stan-style, custom. Each wraps the same protocol
  differently.

Open questions:
- Does the backend see the whole stochastic model at once, or
  per-factor? (Affects what optimizations the backend can do —
  JIT-compile the full model vs. build it incrementally.)
- How do backend-returned samples participate in further graph
  computation? (Clean answer: they enter as new envelope facts on
  existing e-classes, not as new merges.)

### 4.5 Opaque callable protocol

`bind_controller(path, fn, input_contract)` currently says "attach
a callable." Unsaid:

- Which backend runs the callable (same as the rest of the graph,
  or separate)?
- How does gradient flow work when the callable is inside a
  training-time SCC? Backend AD through the callable requires the
  callable to live in the same AD frame as the rest of the
  computation.
- Can a neural controller with Matrix/Tensor I/O use a different
  backend than the main numerical workload?
- Portability: can a callable trained against one backend run
  against another?

Lean: v2.1 commits to same-backend-per-run for simplicity.
Cross-backend callable interop is v2.2+.

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
- **Spec `bind_controller` section.** Opaque callable protocol
  (§4.5 above).
- **Workflow surface.** `run.config.backend`, capability probing,
  fallback policy, version pinning — all new workflow verbs.

---

## 6. Downstream unblocks

With this chunk locked:

- Chunk 05's numerical primitives have a concrete target to lower to.
- Chunk 04's Tier C distributional inference has a concrete protocol.
- `condition_of` Level III has a concrete `condest` target.
- `bind_controller` gradient-flow semantics lock.
- GPU lowering for collections and matrices unifies under one path.
- Enzyme-via-LLVM direction (from `v2.1_in_progress.md:1789`) can
  be framed as "one possible backend implementation," not a
  committed architecture.

---

## 7. Return path

1. AD ownership is resolved in the canonical spec (§31): hybrid
   boundary. This report should mirror it.
2. Minimum backend trait API vs. advertised capabilities is resolved
   (§4.1): small `CoreBackend` plus capability profiles.
3. Default fallback policy is resolved in the canonical spec (§31.1):
   `error`, with `host` / `emulate` as explicit workflow choices.
4. Draft PPL backend protocol (§4.4) — absorbs chunk 04 blocker B3.
5. Draft opaque callable protocol (§4.5).
6. Single-backend-per-run is resolved in the canonical spec (§32.1).
7. Backend versioning is resolved in the canonical spec (§31.4).
8. Write the v2.1 commitment text into the spec.

Chunk 05 (matrices) is closed on source semantics; its primitive
list (§4) now supplies concrete lowering requirements for this
chunk's backend trait surface.

---

## 8. Open questions (consolidated)

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
- **Q4.** PPL backend protocol concrete form?
- **Q5.** Opaque callable gradient-flow semantics?
- **Q6.** Mixed-backend policy for v2.1. RESOLVED in canonical spec
  §32.1: single-backend-per-run; SCC-level cross-backend handoff is
  future work.
- **Q7.** Versioning strategy. RESOLVED in canonical spec §31.4:
  Myco versions the trait surface; backend implementations advertise
  compatible trait versions; plan cache keys include backend identity.
- **Q8.** First concrete backend to implement against (burn?
  NumPy-on-CPU reference? JAX?) — not a design question strictly,
  but affects trait shape.
