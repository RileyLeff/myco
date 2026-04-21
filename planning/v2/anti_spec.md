# Myco — Anti-Spec

Things that are retired, dropped, or wrong. Consult before re-litigating.
Terse by design. Sources: 5-reviewer consolidation audit (2026-04-20),
gap-review stale list, subsequent design locks.

## Retired keywords / syntax

| retired | replacement | why |
|---|---|---|
| `dyn` | `impl Contract` (static monomorph) + `some` (runtime sizing) | clean split of compile-time vs runtime heterogeneity |
| `param` | workflow-bound typed fields | CC1: all values enter from workflow |
| `slot` / `learn_slot` / `bind_slot` / `bind_slot_metadata` | `bind_controller(path, fn, input_contract)` | controller is workflow-only, no `.myco` kind |
| `[*]` wildcard slot inputs | controller data contract | explicit I/O spec |
| transparent-heuristic ABI | unified `bind_controller` | one mechanism for pluggable behavior |
| structural introspection (`<:` predicate, §5.5/§8.5) | nothing | closure policies see values + hyperparameters only |
| `[t+1]` / `[t]` temporal subscripts | `d(x) = expr` (ODE) / `step(x) = expr` (discrete) | subscripts conflated kinds |
| `rate()` | `d(x) = expr` | same |
| `rule` keyword | `event` | disambiguate from rewrite rules |
| module-scope `initial:` / `temporal:` per-type | in-type-body `initial:` / `temporal:` | module-scope kept only for truly cross-entity relations |
| `const N: usize` | `N: val` | cleaner val-generic spelling |
| `assume_topology` | `bind_topology` | 8-verb taxonomy |
| `has`-style field-presence filtering | `where x is T` narrowing | type-based narrowing |
| `property` declarations (`property sigma is PositiveDefinite`) | refinement types + capability contracts (`Invertible<_>`, `Differentiable`, `Monotone`) + `constraint` blocks | redundant with existing machinery; spec_new.md §6 already forbids user property-declaration surface. mock_sperry.myco flagged for rewrite |
| `DataContract` / "data contract" as distinct contract kind | plain contracts satisfied by a type's output fields | workflow-layer visibility (`bind_controller(path, fn, contract)`) enforces access; no failure case found where a plain contract + output-type annotation is insufficient |

## Retired annotations / attributes

| retired | replacement | why |
|---|---|---|
| `#[verified_externally]` | nothing | no proof-escape-hatch annotations |
| `#[inverse]` | capability contract (`Invertible<_>`) on stdlib fn | unified contract machinery |
| four-class invertibility metadata (`bijective` / `injective_restricted` / `lossy` / `opaque`) | capability contracts on fns | same |
| all `#[...]` attribute annotations | nothing | `.myco` has no annotation surface |
| user-declared fn invertibility / differentiability / domain | compiler derives from body composition + stdlib atom contracts | no user property-declaration surface; refactor fn if compiler can't derive |

## Dropped features

| retired | status | why |
|---|---|---|
| macros (declarative + derive) | deferred post-v2.1 | generics + contracts + refinements + `{conserved}` + `impl`/`some` cover the boilerplate use cases |
| homotopy continuation as language feature | workflow Python recipe | belongs on workflow side |
| stdlib physical constants (`R`, `Avogadro`, etc.) | workflow-injected via `assume_constant` | physical constants are values; values live workflow-side |
| literal numerics in `.myco` value position | CC1: banned except in unit defs, affine conversion bodies, structural positions (shape tuples / indices / arity) | no two-trust-posture split; all `.myco` files obey one rule |
| universals carrying values (`universal R: Scalar<U> = 8.314`) | `universal R: Scalar<U>` declaration only; value from workflow | CC1 scope |
| contract composition alias (`contract C := A + B`) | nothing | multi-contract satisfaction (`: A + B + C`) + supertraits already cover the bundle case; alias adds a second spelling with no new expressive power |
| user-facing `Dual` numeric representation | backend-owned AD | Part V commits backend-delegated AD (burn-style tensor tracks operations); user-facing `Dual` would duplicate backend machinery and risks conflicting with backend AD representation. Forward-mode AD is a backend concern, not a user-facing scalar type |

## Retired architectural framing

| retired | replacement | why |
|---|---|---|
| JAX-as-primary emitter | backend trait (burn-style) with capability advertising | no primary backend; trait-based |
| PyTorch-as-primary emitter | same | same |
| residual graph as core semantic object | e-graph three-layer split (equational core / envelope metadata / adjacent keyed state); residual = user-facing projection | chunk 04 recommitment |
| compiler auto-emitted admissibility projections | workflow picks projection flavor (`hard_clip` / `sigmoid` / `soft_clip`) | projection-free-compiler principle |
| compiler auto-selected solver | workflow selects | same principle |
| controller as `.myco` construct | workflow-only concept | strict `.myco` / Python split |
| "slot is gone" narrative / "v2.0 had X" retirement prose | none — use anti_spec.md instead of in-spec versioning | consolidation strips versioning prose |

## Retired open questions (closed or structurally void)

| item | status |
|---|---|
| `dyn` trait-object semantics vs sized | void — `dyn` retired |
| `rule` keyword semantics for topology change | void — renamed to `event` |
| wildcard-slot / slot-declaration / slot-ABI questions | void — slot construct retired |
| structural-introspection iteration | void — introspection retired |
| `[*]` wildcard reachability | void — slot retired |
| homotopy continuation | void — not a language feature |
| `condition_weighted` deferred | resolved — ships via `condition_of` Levels I-III (chunk 04 O4.5) |
| `~` stochastic as e-graph merge | resolved — `~` is layer-2 distributional metadata, not a merge |
| MVN "deferred pending vector/matrix story" | reframed — gated on B5 heterogeneous-unit resolution |
| `d(x)` vs `step(x)` | resolved — both ship |

## Stale in legacy docs (do not import)

- `spec.md` §2.5, §4.11, §7, §5.5, §8.5, §6.3, §12, §13.2-13.3, §14.6, Appendix A/C — supersede wholesale
- `v2.1_in_progress.md` internal `rule`/`event` contradiction (§984-988 vs §1795-1800)
- `v2.1_in_progress.md` "NEW:" / "renamed from" / "API-break note" / "ships in v2.1" versioning prose
- `chunk 01` `assume_topology` occurrences (10 locations) — pre-verb-lock
- `chunk 03` §8 `condition_weighted` deferral — pre chunk-04
- `mock_potkay.myco` — uses `slot` + `[t+1]` + universals-with-values; full rewrite pending
- `mock_sperry.myco` — uses `property monotone: ...` (retired); rewrite to capability contracts pending
- `open_questions.md` §Spec Maintenance section — migration checklist, not spec prose
