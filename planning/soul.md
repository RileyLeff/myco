# The Soul of Myco

Myco is a compiler that separates what is true about the world from how you use
it.

1. **The model is the science.** A `.myco` file should read like a description
   of what relationships hold in reality — not like a program that computes
   something. If you have to think about execution order while writing the
   model, the abstraction is leaking.

2. **The workflow is a separate concern.** What you assume, observe, and learn
   is not a property of the world. It's a property of your experiment. The same
   world supports many experiments.

3. **The compiler does the work.** Causal ordering, solver selection, code
   generation, algebraic inversion, loop detection — all of this is the
   compiler's job. The user declares truth; the compiler figures out
   computation.

4. **Structure is the regularizer.** Overdetermined relations, constraints,
   temporal dynamics, and cross-quantity coupling are not inconveniences to be
   simplified away. They are the signal. The more structure the model encodes,
   the less data you need.

5. **Generated code is the product.** Myco emits ordinary source code, not a
   hidden runtime. The user owns the output — they can inspect it, import it,
   and run it with standard tools. But the source of truth is the `.myco` model
   and the binding, not the generated artifact. Reproducibility comes from
   recompilation, not from hand-editing outputs.

6. **The shortest faithful model should feel natural.** Myco should not make
   modelers pay boilerplate tax for ordinary scientific claims. Complexity belongs
   in the source only when it expresses a real distinction in the world; execution
   ceremony belongs to the compiler and workflow.

7. **Myco itself has absolutely nothing to do with ecosystem simulation or plant physiology.** This is a general-purpose tool for scientific research and simulation, all the plant stuff is just how the author is thinking about his own work and stress-testing the framework. All plant physio-specific stuff will live outside of this repo in implementation, only in here as an example.
