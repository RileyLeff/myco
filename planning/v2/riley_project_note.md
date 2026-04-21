## Scope boundary — Myco vs Riley's project

**No Riley-project-specific stuff in the Myco core repo.** Plant
ecophysiology-specific content — the ecophys unit library
(water potential, gas-exchange rates, PPFD, LAI, soil-water
conversions, species-trait inputs, vulnerability curves, Jmax /
Vmax, Q10-style corrections, etc.), model-family shapes (Sperry /
Potkay / Farquhar variants), ecosystem-simulation scaffolding,
FiLM-style taxonomic embeddings, and so on — lives in a dedicated
external library (tentatively called a **"spore"**: a distributable
Myco package). The spore is a consumer of Myco, not part of it.

Things that DO belong in Myco core: general language surface
(types, contracts, relations, events, loci, workflow verbs), the
general units machinery (SI base + derived, the affine-conversion
facility), the distribution catalog and capability contracts, the
backend trait, etc. Things that do NOT belong in Myco core: any
unit, function, model shape, or stdlib item that only makes sense
in a plant-physiology / ecology / Riley's-research context.

If a tracker item or writeup starts talking about respiration,
photosynthesis, stomata, canopies, leaves, soil water, trees,
species, etc. as language / stdlib content — that's the
spore-project, not Myco core.

---

let's worry less about "what we can implement now" and worry more about "what's the right thing for myco". i find that if I take shortcuts to    
  get to products faster with LLMs it becomes harder, not easier, to build the right thing later on. so let's try to get it right. if we disregard  
  tiers, what's the right version of this look like? i think from my perspective, the ux i want is to be able to add data and assumptions on the    
  python side in stages or steps -- like i might take my .myco -> add a python script that furnishes my data and computes it -> it returns me       
  another myco graph i can add some assumptions to etc. so symbolics and proof-like semantics would be useful here. for training, i actually think  
  it's very important how we determine how to handle unobserved/underdetermined nodes. let me tell you more about my exact use case so i can make   
  this clear. i'm a plant ecophysiologist. for decades, we've basically been following farquhar's insight: some systems, like stomata (also         
  extends to carbon allocation, phenology, etc) are too complex to model mechanistically from how they're built, perhaps those things are           
  regulated by stuff that happens at a cellular level and we can't build a tree from scratch out of its genes or molecule. so you have to have      
  some compression to describe the stuff that you can't see. farquhar's approach was teleology: he said the *purpose* of stomata is to maximize     
  carbon gain relative to water loss. the field has been running with heuristics like that for decades: sperry extends his optimization criterion   
  to operate on a mechanistic body of a plant, with descriptions of hydraulics and photosynthesis. potkay moved the optimization criterion to       
  growth, which lets you integrate some carbon dynamics etc. but we're still governed by a behavioral heuristic, which is basically imposing a few  
  assumptions: 1.) plants have some purpose 2.) we, the observers, are capable of determining exactly what it is and 3.) plants are perfectly       
  optimized in pursuit of that purpose. all of those assumptions are weak. think rubisco, recurrent laryngeal nerve, and so on. i see evolution as  
  more a process of "whatever is reachable and whatever persists" not one of optimization. and while you can get your hueristics to behave          
  realistically in typical conditions for a few timesteps, it doesn't allow us to simulate a tree over long timescales, especially in non-standard  
  or weird conditions, and it doesn't generalize well across plant diversity and environmental conditions etc. you end up with these weird hacks    
  in the literature where you ask 'what's going to happen to trees under future climates' and you restart the simulation every year so it doesn't   
  accumulate error. seems like it defeats the point to me? and in models that do run over long time scales, think zoomed out ones like earth        
  systems etc, you lose all of the complexity and dynamism of plants. my view is that the different components of a complex organism may be net     
  synergetic or net (whatever the opposite of synergy is), and you can get things like rapid crashes to death with slight disturbance or            
  surprising resilience, where the whole system is more than the sum of its parts, and you lose that resolution when you zoom out. so with myco,    
  here's what i want to do: i want to learn the unobservable stuff from data, no assumptions. i want to have a mechanistic graph that describes     
  how plants work. i want a neural controller that outputs "decisions" about anything we don't have a mechanistic model for -- stomata,             
  allocation, phenology, aquaporins, etc. and the mechanistic graph should constrain the decision space to realistic ranges, and because we have    
  some of the key systems represented outside of the neural net that drastically reduces the space of things the model has to learn -- e.g. it      
  doesn't have to learn hydraulic physics or anything. but here's the thing: this kind of approach requires lots and lots of data, and plant        
  ecophysiology has lots of field study, but it's all pretty sparse, irregular, and not organized well. so here's my thought: basically i want to   
  take one mechanistic model -> scrape many papers for many of these small irregular datasets -> and train on the whole set. each dataset might     
  require me to configure the model with different inputs and outputs. but if i can get all of the training runs to live in the same space at the   
  same time, i think i can get identifiability of the control parameters to emerge from each indiviudal study's unmeasured parameters. like if      
  study 24 says here's this dataset on non-structral carbohydrate concentrations and canopy dieback, i can use that, leave the rest symbolic,       
  constrain with what i can from the types, mechanisms, constraints, etc, and for that particular study you might not be able to tell stomatal      
  control from root depth, but as long as i have more papers that have root depth in them, i can still get to identifiability. does that make       
  sense? so i see my path here as follows: 1.) build myco, properly. 2.) use myco to implement the sperry model, potkay model, etc. 3.) use those   
  to generate synthetic datasets. 4.) remove the optimization crtierion from those models and replace them with neural net, train on the synthetic  
  data, see if i can recover the controller. good test. 5.) start systematically removing data from my synthesized experiments. see if i can        
  still train and recover the controller. 6.) ideally i should be able to delete so much data that each indivudal synthetic "study" doesn't         
  contain sufficient information to make the controller identifiable on its own, but across the synthetic studies, we can recover the known         
  controller. this would be evidence to me that the idea works. 7.) use LLMs to scrape both phyisogical and community level data on trees, build a  
  training run, try to recover a controller that both A.) describes plant behavior B.) diverges from the heuristics. should recover more stuff      
  than a heuristic can specify. i expect this one to be maybe worse at describing plants than a heuristic at first, and then i see it as my job to  
  scale the shit out of the data to make my method win on scale. 8.) other notes: i see it that i need to do my training runs in an ecosystem       
  simulation, not just a single tree simulation. if i train a tree where it's the only tree in the universe, it's not going to learn realistic      
  things about like competition for light etc. another thing is that i need to figure out how to handle species etc. with things like traits that   
  vary, i see that as data inputs to a generic mechanistic model, where we have these broadly applicable compressions like a vulnerability curve    
  and a jmax and vmax and that stuff. I think we might want to use something like FiLM to do taxonomic embeddings on top of a foundation/base       
  model that just knows everything about trees broadly, and then you can specify to something like "conifer" or "loblolly pine" if you want. and    
  some of those will be better identified than others. this is what a different claude and a gemini instance said about film in this context:       
  [Pasted text #2 +4 lines] . one more thing, is i want to be clear that i see this project as a consumer of myco, not something to throw in this   
  repo for the tool. does all that make sense to you? do you see the vision?        