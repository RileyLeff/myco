 Let me lay out the problem space first, then the approaches.

  What is a kernel in Myco's context?                                                                                                                                                                           
  
  A kernel is a function that says "how strongly does point A influence point B?" It's the spatial coupling mechanism. Every cross-scale or cross-entity interaction in ecosystem modeling is some flavor of    
  this:                                                                     
                                                                                                                                                                                                                
  - Root-soil coupling: Each root segment draws water from nearby soil, weighted by distance. A root at 30cm depth mostly affects soil at 25-35cm, barely touches soil at 1m.                                   
  - Shading: A tall tree reduces light for shorter neighbors. The "kernel" is crown geometry — how much of tree B's sky does tree A block?
  - Competition: Two root systems overlap. The overlap fraction determines how much they compete for the same water.                                                                                            
  - Dispersal: Seeds from tree A land near A, weighted by a dispersal kernel (maybe Gaussian, maybe fat-tailed).                                                                                                
                                                                                                                                                                                                                
  The mathematical pattern is always one of:                                                                                                                                                                    
                                                                                                                                                                                                                
  // Entity-to-field (root draws from continuous soil):                     
  effect_on_A = integrate(K(A, point) * field(point), over = domain)                                                                                                                                            
  
  // Entity-to-entity (tree shades tree):                                                                                                                                                                       
  effect_on_A = sum(K(A, B) * quantity_on_B for B in collection)            
                                                                                                                                                                                                                
  What the compiler needs to know:
                                                                                                                                                                                                                
  The naive implementation of both patterns is O(n²) or worse — evaluate the kernel for every pair. But most ecological kernels are sparse — they decay with distance. A root at 30cm doesn't affect soil at    
  10m. If the compiler knows the kernel's characteristic length scale, it can use spatial indexing (grid hashing, kd-trees) to skip distant pairs. This turns O(n²) into O(n log n) or even O(n).
                                                                                                                                                                                                                
  So the design question is really two questions:                                                                                                                                                               
  1. How does the user express kernel-weighted coupling?
  2. How does the compiler learn about sparsity?                                                                                                                                                                
                                                                            
  Approach 1: Kernels are just functions. No keyword.                                                                                                                                                           
                                                                                                                                                                                                                
  The most minimal option. A kernel is a regular fn. The compiler detects the coupling pattern when a function appears as a weight inside integrate() or sum().                                                 
                                                                                                                                                                                                                
  fn root_uptake_kernel(                                                                                                                                                                                        
      root_depth: Scalar<m>,                                                                                                                                                                                    
      soil_depth: Scalar<m>,
      sigma: Scalar<m>                                                                                                                                                                                          
  ) -> Scalar<dimensionless> {                                              
      let d = abs(root_depth - soil_depth)
      exp(-d^2 / (2 * sigma^2))
  }                                                                                                                                                                                                             
  
  relation root_uptake for r in roots:                                                                                                                                                                          
      r.water_supply = integrate(                                           
          root_uptake_kernel(r.depth, z, r.sigma) * soil.water_potential,
          over = soil                                                                                                                                                                                           
      )
                                                                                                                                                                                                                
  The compiler sees root_uptake_kernel(...) inside integrate(... over = soil), recognizes it as spatial coupling. Because all Myco functions have transparent bodies, the compiler can analyze the body: it     
  depends on abs(root_depth - soil_depth) and decays as exp(-d²/2σ²). The compiler extracts σ as the characteristic length scale and emits spatial indexing that skips pairs where d > 3σ.
                                                                                                                                                                                                                
  No keyword, no annotation. The compiler is smart.                         

  Approach 2: kernel keyword — dedicated declaration.                                                                                                                                                           
  
  Kernels get their own declaration form with mandatory sparsity metadata:                                                                                                                                      
                                                                            
  kernel root_uptake_kernel(
      root_depth: Scalar<m>,                                                                                                                                                                                    
      soil_depth: Scalar<m>
  ) -> Scalar<dimensionless> {                                                                                                                                                                                  
      characteristic_length: sigma                                          

      sigma: Scalar<m>                                                                                                                                                                                          
      let d = abs(root_depth - soil_depth)
      exp(-d^2 / (2 * sigma^2))                                                                                                                                                                                 
  }                                                                         

  The characteristic_length field explicitly tells the compiler the decay scale. No analysis needed — the user declares it.                                                                                     
  
  Approach 3: Annotation on regular functions.                                                                                                                                                                  
                                                                            
  fn root_uptake_kernel(...) -> Scalar<dimensionless>
      #[spatial(distance = abs(a - b), scale = sigma)]                                                                                                                                                          
  { ... }
                                                                                                                                                                                                                
  Middle ground — it's a function, but annotated with spatial metadata.                                                                                                                                         
  
  Approach 4: No named kernel at all — inline everything.                                                                                                                                                       
                                                                            
  relation root_uptake for r in roots:                                                                                                                                                                          
      r.water_supply = integrate(                                           
          exp(-abs(r.depth - z)^2 / (2 * r.sigma^2)) * soil.water_potential,
          over = soil                                                                                                                                                                                           
      )
                                                                                                                                                                                                                
  The kernel is just an inline expression. The compiler can still analyze it.                                                                                                                                   
  
  ---                                                                                                                                                                                                           
  My recommendation: Approach 1 (kernels are just functions).               
                                                             
  Here's why it's the most Myco approach:
                                                                                                                                                                                                                
  It follows the existing design philosophy. Myco functions are already transparent — the compiler does symbolic differentiation on them, dimensional analysis through them, invertibility analysis on them.    
  Analyzing a function body for distance-dependence and decay rate is the same kind of work. No new concept needed.                                                                                             
                                                                                                                                                                                                                
  It doesn't add a keyword for something that doesn't need one. A kernel is a function. kernel would be a keyword that means "a function the compiler should try to optimize spatially." But the compiler should
   do that optimization whenever it can, not only when the user remembers to ask. Adding a keyword means the user can forget it and get silently slow code.
                                                                                                                                                                                                                
  Entity-to-field and entity-to-entity are naturally unified. Both use the same functions, just in different contexts:                                                                                          
  
  // Entity-to-field: root-soil (integrate over continuous domain)                                                                                                                                              
  relation root_uptake for r in roots:                                                                                                                                                                          
      r.water_supply = integrate(
          K_root(r.depth, z, r.sigma) * soil.water_potential,                                                                                                                                                   
          over = soil                                                                                                                                                                                           
      )
                                                                                                                                                                                                                
  // Entity-to-entity: shading (sum over discrete collection)               
  relation shading for t in trees:
      t.shade_fraction = sum(
          shade_kernel(t, other) * other.crown_opacity                                                                                                                                                          
          for other in trees where other.height > t.height
      )                                                                                                                                                                                                         
                                                                            
  Same function mechanism, different aggregation (integrate vs sum). The compiler recognizes both as spatial coupling patterns.                                                                                 
  
  The shading case is interesting — that where other.height > t.height clause tells the compiler the interaction is triangular (only taller trees shade shorter ones). The compiler can prove this and use an   
  O(n log n) height-ordered sweep instead of O(n²). This optimization falls out of the where clause analysis, not from a kernel annotation.
                                                                                                                                                                                                                
  The one gap: learned kernels. If the kernel is a neural slot, its body is opaque — the compiler can't analyze it for sparsity. Two options:                                                                   
  
  - Conservative default: If the compiler can't determine sparsity, it falls back to global interaction (O(n²)). The user notices it's slow and provides a hint.                                                
  - Workflow-layer hint: Same as discretization config — the .myco file describes the coupling, Python hints the spatial optimization:
                                                                                                                                                                                                                
  experiment.compile(                                                       
      kernel_config={                                                                                                                                                                                           
          "root_uptake_kernel": {"max_radius": 0.5},  # meters                                                                                                                                                  
          "shade_kernel": {"strategy": "height_sweep"},
      }                                                                                                                                                                                                         
  )                                                                         
                                                                                                                                                                                                                
  This is consistent with the existing principle: .myco describes the world, Python configures how to solve it.                                                                                                 
  
  What I'd push back on if someone proposed Approach 2 or 3: The characteristic_length or #[spatial] annotation is just restating information that's already in the function body. For transparent functions,   
  it's redundant. For opaque slots, the workflow layer is the right place. There's no case where the annotation is both necessary and belongs in the .myco file.
                                                                                                                                                                                                                
  What's your gut on this? Does "kernels are just functions, compiler figures it out" feel right, or do you want something more explicit?                      