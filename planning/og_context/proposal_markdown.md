#import "@preview/charged-ieee:0.1.3": ieee

#show: ieee.with(
  title: [Dissertation Proposal],
  authors: (
    (
      name: "Riley Leff",
      department: [Ph.D. Candidate, Biological Sciences],
      organization: [George Washington University],
      location: [Washington, DC],
      email: "rileyleff@gmail.com"
    ),
  ),
  bibliography: bibliography("references.yml", style: "ieee"),
  figure-supplement: [Fig.],
)

#show link: set text(fill: blue)

= Introduction

Accelerating sea level rise is threatening coastal forests worldwide, leading to tree mortality and the formation of "ghost forests" — landscapes of dead trees and stumps surrounded by encroaching marshlands @kirwanSealevelDrivenLand2019. These coastal forests play critical roles in carbon sequestration, storm mitigation, and habitat provision @smithSeaLevelDrivenMarsh2021, and their loss can negatively impact both local ecosystem function and global climate regimes @tagestadSmallStreamsDominate2021 @wardRepresentingFunctionSensitivity2020 @smithSeaLevelDrivenMarsh2021.

The future extent of these changes remains uncertain, largely due to our limited ability to predict tree mortality across space and time. While researchers have identified several factors that predispose trees to mortality risk @molinoBiophysicalDriversCoastal2023 @mcdowellProcessesMechanismsCoastal2022, both explaining past mortality events and predicting future ones has proven challenging. Significant advances have been made in three key areas: relating environmental conditions to tree mortality, representing plant hydraulic and photosynthetic processes, and integrating those plant dynamics into larger-scale environmental models @dingModelingMechanismsConifer2023a @fieldCreationGhostForests2021 @liInfluenceIncreasingAtmospheric2022. Despite these advances, we still lack a robust, generalized framework for predicting when and why trees die.

The central challenge in quantifying tree mortality is that trees are moving targets. Not literally -- they can't move, at least to my knowledge. They can, however, alter their internal distribution of assets and liabilities to respond to changing external conditions. A plant can achieve this via two distinct pathways. First, a plant may preferentially take on stress in one facet of its physiological state so as to ameliorate stress in another. For example, by closing its stomata, a plant takes on additional risk of carbon starvation but mitigates its risk of hydraulic failure, which may represent a safer overall state under dry or saline conditions if that plant has ample carbon reserves @sperryWhatPlantHydraulics2015. Second, a plant may alter one or more of its traits to better suit current and future environmental conditions. For example, by growing denser sapwood, a plant improves its ability to continue conducting water under severe drought stress @hackeTrendsWoodDensity2001. The per-unit carbon cost of building new sapwood increases with density, but paying that higher price may be a better investment during drought conditions than alternative uses of the carbon, such as growing a greater amount of lower-density (more vulnerable) sapwood.

#figure(
  image("images/death_spiral.png"),
  caption: [
    Recent conceptual models of tree mortality like this one reproduced from @mcdowellProcessesMechanismsCoastal2022 recognize the interconnectedness of various environmental and physiological processes in response to changing climate. These models, however, often commingle adaptive physiological adjustments with detrimental external circumstances. For example, while stomatal closure and reduced leaf area impose opportunity costs, they also save water, lower maintenance respiration, and may confer net benefits to trees in changing markets. Quantifying these distinctions is crucial for accurately predicting forest responses to sea level rise, where apparent symptoms of decline may sometimes represent beneficial acclimation to new circumstances rather than a funnel towards mortality.
  ]
)

This dynamism is, in my view, what makes tree mortality so difficult to predict. Models that seek to predict tree mortality based on some threshold environmental condition (a particular value of soil water potential, site inundation frequency, groundwater salinity, or otherwise) silently depend on externalities, like carbon storage status and trait plasticity, that often don't generalize well across species or sites. Process-based models, likewise, can't capture _every_ process, nor do they have the means to generalize the relationships between constituent processes across contexts (at least not yet, see @wdttk).

These fundamental limitations in modeling approaches du jour suggest a critical knowledge gap: despite extensive study of how environmental conditions influence tree mortality rates, and how physiological states correlate with mortality risk, we lack a comprehensive understanding of how trees dynamically adjust their physiological states in response to changing environmental conditions, and how much additional resilience those adjustments can confer to them.

Coastal forests, particularly in the shallow slopes of the US's mid-atlantic region, provide an ideal system for testing these ideas. The gradual increase in salinity and inundation frequency creates environmental stress gradients analogous to those experienced during drought @allenGlobalOverviewDrought2010, but with several key advantages for study. First, the rate of change is relatively predictable compared to drought events. Second, the spatial gradient from upland to shoreline provides a natural laboratory for studying how trees respond to different levels of stress. Third, because these systems are experiencing novel combinations of stressors, they may reveal the limits of trees' adaptive capabilities.

To address these critical knowledge gaps in our understanding of coastal forest dynamics, my dissertation integrates empirical research with quantitative modeling approaches. This work aims to advance our ability to predict coastal forest responses to accelerating sea level rise and provide insights applicable to other ecosystems experiencing novel environmental stresses.

#figure(image("images/ghost_forest.jpg"),
caption: [
  A ghost forest on the Chesapeake Bay, near Cambridge, Maryland.
])



= Research Aims

+ _Improve the precision, scalability, and cost-effectiveness of physiological and environmental data generation._

  #figure(
    image( "images/concept_fig_measurement_precision.png", width: 100%),
    caption: [Early detection of subtle physiological changes is critical for understanding tree responses to environmental stress. By developing higher-precision measurement techniques, we can identify emerging trends in plant responses before they manifest (or fail to manifest) as visible symptoms of decline. Small physiological adjustments that appear subtle in isolation may compound over time, potentially revealing fundamental differences in resilience or vulnerability between individuals or populations experiencing similar stressors.]
  )<measurement_fig>

    Improving the accuracy of empirical measurements can drastically narrow confidence intervals around long-term projections of tree mortality (@measurement_fig). Further, by developing open-source, low-cost tools, we can expand capacity for data collection, foster collaborative improvement, and reduce financial barriers to participation in science.

  \
+ _Derive mechanistic explanations for plant responses to climate change through empirical research._

  Plant responses to environmental stress involve complex interactions among multiple physiological systems. By empirically measuring how these mechanisms function together across environmental gradients, we can develop more accurate models to describe plants' vulnerability or resilience. When integrated with process-based models, these empirical findings allow us to test hypothesized causal relationships and attribute observed responses to specific physiological mechanisms, establishing clear links between environmental drivers and plant outcomes.

  #figure(
    image("images/concept_fig_interaction_of_resilience.png"),
    caption: [
      Plants may employ multiple adaptive mechanisms that interact to determine their overall resilience to climate stressors. This conceptual model illustrates how various combinations of physiological responses (dashed lines) can alter a plant's trajectory compared to a fixed-trait model (solid line). As environmental stress increases over time (x-axis), well-coordinated sets of adaptive strategies may help trees survive longer than staying put. Through empirical research measuring these responses in field settings, we can quantify how different response combinations determine survival thresholds under changing climate conditions, revealing both the adaptive capacity and fundamental limitations of plant acclimation strategies.
    ]
  )<fig_interaction_of_resilience>
  \
+ _Build a quantitative framework to generalize plant physiological responses to the environment around a unifying incentive._

Plant responses to environmental stressors like sea level rise and salinity can be understood through an economic lens where physiological decisions reflect adaptive tradeoffs. Current models often hold plants to particular fixed strategies that may not stay accurate across changing contexts. By building a more flexible model framework that allows plants to update their strategies given new information about the environment and coordinate their decision-making across multiple facets of their physiology over time, I aim to provide more accurate predictions of tree mortality suitable for a wide range of ecological circumstances.

= A Sap Flux Methodology Enabling Direct Calculation of Plant Thermal Parameters and Probe Spacing

== Abstract
Sap flux sensors, sensors that estimate water movement through plants by tracking heat propagation through sapwood, are widely used but rely on mathematical models with unverified assumptions about sapwood thermal properties and probe positioning. Current methods treat these parameters as constants, leading to unquantified uncertainties that limit their application in precise physiological studies. Here we show that changes in temperature ratios between sensor probes following a heat pulse, previously treated as noise, contain sufficient information to directly solve simultaneously for thermal conductivity, heat velocity, and probe positions. Using an original open-source simulation framework, we demonstrate that this novel analytical approach can nearly eliminate measurement error, compared to conventional methods that can generate errors in excess of 70% under realistic measurement conditions. Our method eliminates the need for assumption and unrepresentative empirical sampling to estimate key parameters. These advances establish a theoretical foundation for more reliable sap flux measurements and reveal significant limitations in current applications of heat-pulse methods. We anticipate this work will lead to the development of more accurate sensor designs and enable proper statistical treatment of measurement uncertainty in plant hydraulic studies.

== Background <sapflux-bg>
Accurately quantifying plant hydraulic function is fundamental to understanding responses to environmental stress at the organism and ecosystem scales. Sap flux density – the rate at which water moves through plants, normalized for cross-sectional area - is a useful indicator of both water availability and utilization in forest ecosystems. Researchers commonly refer to quantities of sap flux density as “measurements”, but that is a misnomer. Directly measuring sap flux density is both destructive and impractical in the field. Sap flux densities are typically estimates derived from mathematical models. Popular methods estimate sap flux rates by measuring rates of heat propagation through a tree's sapwood @marshallMeasurementSapFlow1958.

The relationship between heat and sap movement is complex. A key challenge in generating precise and accurate estimates lies in separating two modes of heat transfer: conduction (heat traversing the sapwood itself) and convection (heat carried by moving sap). Sap flux is only responsible for the convection component of heat transfer. While the theoretical basis for common models accounts for both modes of heat transfer separately, current implementations of those models oversimplify by treating a plant’s thermal conductivity as a constant value. This ignores an important reality: a plant's thermal conductivity changes with its water content, varying daily, seasonally, and in response to changing environmental conditions. This assumption attributes all variation in heat velocity to variation in sap flux rate, even though some portion of that variation is attributable to variation in thermal conductivity. This assumption is particularly dangerous considering that plant water content, the primary source of variation of plant thermal conductivity, may be causally related to a researcher’s variables of interest, potentially confounding experimental results.

Some researchers attempt to address this by measuring sapwood thermal conductivity directly in the field, but this approach faces significant limitations. Sap flux sensors typically collect continuous data, but empirical measurements only represent one time point. Thus, point measurements of water content quickly become unrepresentative, often within hours. The magnitude of this variation can be substantial – in an experiment on European beech (Fagus sylvatica), relative water content ranged from 0.47 to 0.90, corresponding to thermal diffusivity variations between 2.2e-7 and 2.7e-7 square meters per second, with average errors of 22% @vandegehuchteTripleprobeHeatpulseMethod2012. Moreover, common field techniques for estimating sapwood moisture content cannot distinguish between bound and unbound water, an oversimplification that contributes more than 10% error to sap flux estimates @vandegehuchteImprovingSapFlux2012a. Water content measurements are further complicated by the local effects of sensor installation, such as plant wounding responses. Sensor installation may alter local thermal and hydraulic properties, rendering measurements taken elsewhere on the plant potentially unrepresentative @burgessImprovedHeatPulse2001.

Probe misalignment represents another significant source of error in sap flux measurements. Heat-pulse sap flux sensors consist of two key components: a needle-like heater and one or more needle-like thermistors, which must be installed by drilling holes into the tree's sapwood using a dremel. Accurate sap flux calculations depend on precise knowledge of both the distance between the heater and thermistors as well as the time required for heat to traverse that distance. However, the installation process can inadvertently alter the critically important spacing between sensor components. Even when using drill guides to improve precision, imperfect angles can introduce errors. The magnitude of these installation-induced errors can be surprisingly large: a vertical misalignment of just 1 millimeter can produce errors exceeding 50% in some commercial sensor systems.

Alternative sensor designs attempt to address these challenges but face their own limitations. Non-invasive approaches such as external collar-like sensors @lascanoStemHeatBalance2016 can rely heavily on empirically derived parameters that may not accurately represent the measured plant. Methods based on heat field deformation require a continuously-powered heater, which can pose challenges for remote sensor installations due to their energy demands @nadezhdinaSapFluxDensity2012. Recent innovations such as miniaturized silicon designs @kimMicromachinedNeedlelikeCalorimetric2024 are promising, but their current applications are restricted to non-woody vegetation like tomatoes.

Current heat-pulse methods can estimate ecosystem-scale water usage when supported by sufficient replication and the assumption that multiple sources of error converge toward a central tendency. However, these methods face significant limitations when applied to the study of individual plant responses to environmental change. One critical weakness is the methods' inability to quantify uncertainty around difficult-to-measure parameters, preventing the calculation of confidence intervals or probability distributions for our estimates. To enable more rigorous study of plant physiological responses using sap flux measurements, we must develop more robust methods for measuring or estimating key parameters, particularly sapwood thermal conductivity and probe position.

== Derivation <sapflux-mt>

Here, we demonstrate an analytical solution that enables the direct computation of key parameters from existing thermal models and sap flux methodologies. Our approach leverages previously ignored information in the variation in probe temperature ratio following a heat pulse to solve for thermal conductivity, heat velocity, and probe positions without assuming fixed values for any of these parameters. We derive a system of equations that isolates and resolves each parameter independently. This mathematical framework eliminates the need for empirical calibration or assumed constants while maintaining compatibility with existing sensor designs.

Heat pulse sap flux methods are usually based on Marshall's 1958 model @marshallMeasurementSapFlow1958 of temperature change $Delta T$ in sapwood following an energy input. Refer to @sapflux-symbols for an overview of the symbols used.

$ Delta T = q/(4 pi k t) exp(-1  * (x - V_h t)^2 / (4 k t)) $

#figure(
  table(
    columns: 4,
    [Symbol], [Parameter], [Example Unit], [Notes],
    [q], [Energy input], [W/m], [Heat pulse.],
    [k], [Thermal diffusivity], [m#super[2]/s], [Heat conduction, depends on sapwood moisture content.],
    [V#sub[h]], [Heat velocity], [m/s], [Heat convection, depends on sap flux rate.],
    [t], [Time elapsed since heat pulse], [s], [],
    [x], [Probe position relative to heat source],[m], [Usually on y-plane, not x or z.]
  ),
  caption:[Sap Flux Equation Symbols]
)<sapflux-symbols>

One such technique is the Heat Ratio Method (HRM), which measures temperature differences $Δ T$ at two points some distance from a central heat source. While these measurement points are traditionally labeled as "upstream" and "downstream" probes, such terminology implies unidirectional flow. Here, I'll denote the measurement positions using numerical subscripts _0_ and _1_.

To estimate sap flux density using the HRM, we first compute $alpha$, the log ratio of the pair of $Delta T$ values.

$ alpha = ln((Delta T_0) / (Delta T_1)) $

#figure(
  image("images/sapflux_normal3.png", width: 100%),
  caption: [Typical sap flux heat pulse with no probe misalignment. Note that alpha (red line, the log ratio of the green downstream and blue upstream temperatures) is constant over time. Assumes instantaneous heat pulse.]
)

Given that the log of a quotient is equal to the difference of the logs, we can substitute Marshall's equation for both instances of $Delta T$ and simplify the equation to a solution for $V_h$, the velocity of the heat pulse.

$ V_h = (2 k alpha) / (x_0 - x_1) + (x_0 + x_1)/(2t) $ <sapflux-vh-from-alpha>

In the case that probes 0 and 1 are on opposite sides of the heat source at an equal distance apart (e.g. $x_0 = -1 "cm"$ and $x_1 = +1 "cm"$), @sapflux-vh-from-alpha further simplifies to $ V_h = (a k) / x $

Thus, $alpha$ only depends on $t$ if $x_0 != -x_1$. One would expect $alpha$ to remain constant as long as $k $ and $v$ are constant, which should be a reasonable assumption over the approximately two minute-long measurement cycle.

Commercial sap flux sensors are often designed with equidistant probe spacing to leverage this property of the model @forsterImportanceConductionConvection2020. _However, contrary to what one would expect, alpha often varies over the duration of the temperature measurement on these sensors_. Sensor designers and researchers alike often choose to ignore this variation by taking the average value of $alpha$ between 60 and 80 seconds after the heat pulse as the true value of alpha @burgessImprovedHeatPulse2001 @forsterImportanceConductionConvection2020. Here, I propose that instead of ignoring $Delta alpha$, *we can utilize the information in $Delta alpha$ to directly measure our parameters of interest*.

#figure(
  image("images/sapflux_misaligned3.png", width: 100%),
  caption: [Sap flux heat pulse with merely 1mm probe misalignment. Note that alpha is not constant, and any resulting estimate of V#sub[h] will vary depending on time of measurement.]
)

The derivative of $alpha$ with respect to time, shown below, has an extremely useful property: it depends on $k$, but not on $V_h$.
$ (delta alpha)/(delta t) = (x_0^2 - x_1^2)/(4 k t^2) $

Trivially solving for $k$:

$ k = (x_0^2 - x_1^2) / (4 t^2 ((delta alpha) / (delta t))) $

Likewise, the ratio of any two alphas $alpha_a$ and $alpha_b$ recorded at times $t_a$ and $t_b$ depends on $V_h$ but not on $k$. 

$ alpha_a / alpha_b =  (t_b (-x_0 + 2 t_a V_h - x_1))/(t_a (-x_0 + 2 t_b V_h - x_1)) $

Trivially solving for $V_h$:
 
$ V_h = ((x_0 + x_1) (t_a (alpha_a / alpha_b) - t_b)) / (2 t_a t_b (alpha_a / alpha_b - 1)) $

By separating these variables, we no longer need to make assumptions about one to calculate the other. We're left with one unsolved parameter: the temperature probe distance(s). Clearly, since alpha is not constant on field-deployed sensors with allegedly equidistant probes, it would be deleterious to make assumptions regarding the probe spacing. We can directly compute probe distances by combining our equations for $k$ and $V_h$ derived here with estimates of probe distance derived from another method: the T#sub[max] Method (TMM).

The TMM relies on the time a probe takes to reach its maximum temperature after a heat pulse. Using a combination of the TMM, my new equation for $k$, and my new equation for $V_h$, we can solve for both probe positions directly from measurements of temperature changes.

== Results and Discussion

By providing analytical solutions for $k$, $V_h$, $x_0$, and $x_1$ that are easy to calculate from real-world temperature sensors, we’ve eliminated the most prominent sources of error from a popular style of sap flux sensors. This new approach facilitates more precise and accurate study of tree physiology, and establishes a pathway towards the proper statistical handling of sap flux model parameters. Further, it calls into question the tractability of some existing sap flux results.

We will demonstrate the improved efficacy of these sap flux methods in a set of in silico experiments run in the sap flux simulator, a program I wrote to assist the development of and test the methods described above. The sap flux simulator will itself be released as open-source software on github, ideally with an accompanying paper in the #link("https://joss.theoj.org/")[Journal of Open Source Software]. Areas that stand to improve the most from this improved methodology include sensitive overnight measurements, distinguishing between low, zero, and negative flow, and in measuring individual responses to environmental phenomena.

Additionally, we provide a set of design principles for future sap flux sensors to adhere to in order to maximize the quality and efficacy of data collected. Recommendations include staggering the sensor's probe distances to intentionally generate large values of $Delta alpha$, which mitigates the potential for measurement error from the underlying temperature sensors.

= Physiological Coordination With Changing Environments Makes Forests More Resilient <hydraulics-chapter>

== Background
Rising sea levels threaten coastal forests across the globe @sweetGlobalRegionalSea2022 @mcdowellMechanismsWoodyplantMortality2022. One might expect that as salinity and inundation frequency increase, forests would retreat to higher elevations, with ghost forests—regions of standing dead trees—marking these transitions @kirwanSealevelDrivenLand2019. However, ghost forest formation remains surprisingly rare: between 2000 and 2018, less than 1% of coastal forest patches in the northeastern US experienced detectable losses @fieldCreationGhostForests2021.

This resilience manifests in retreat rates that consistently lag behind changes in sea level rise and soil salinity @williamsSEALEVELRISECOASTAL1999 @clarkCoastalForestTree1986 @kirwanDynamicsEstuarineForest2007 @schiederSealevelDrivenAcceleration2019. Notably, geophysical properties such as slope and elevation explain only about half of the variation in forest retreat patterns @molinoBiophysicalDriversCoastal2023. This phenomenon parallels findings from drought-affected forests, where individuals of the same species subject to similar stressors may die at different times—even physiological thresholds often fail to explain why one tree dies while another survives @trugmanWhyTreeDrought2021 @sevantoMoreAccurateVegetation2016.

#figure( image("images/bville.JPG"), caption: [The Brownsville Preserve, a coastal forest near Nassawadox, Virginia where we're conducting our study. Areas closer to the marsh (near the bottom of the image) are at lower elevations and experience higher salinity. Canopy health and forest diversity improves with elevation, e.g. in the area near the top of the image. Slopes are shallow along the gradient, providing a high-resolution look at how trees respond to changing salinity and inundation frequency.] )

A key pattern emerging from field studies is that mature trees often persist beyond salinity thresholds where seedlings from those same trees cannot establish @woodsSoilSalinityImpacts2020. Even after environmental conditions become unfavorable, established plants may survive on resources accumulated during more favorable conditions. In the Pacific Northwest National Lab's studies of salinity-induced tree mortality at Beaver Creek in Washington state, _Picea sitchensis_ (Sitka spruce) trees drastically reduced stomatal conductance after exposure to elevated salinity and experienced a slow decline in non-structural carbohydrates over several years @zhangDecliningCarbohydrateContent2021 @wangSevereDeclinesHydraulic2022. Notably, these trees did not experience severe hydraulic failure @zhangSeawaterExposureCauses2021.

#figure( image("images/beaver_creek.jpeg"), caption: [Near-complete defoliation of _Picea sitchensis_ (Sitka spruce) at Beaver Creek, July 2019, roughly 5 years after the removal of the causeway.] )<fig_beaver_creek>

These Beaver Creek studies followed an anthropogenic disturbance: a causeway removal that reintroduced tidal seawater to a previously freshwater system. Before the removal, Beaver Creek was a freshwater ecosystem for nearly 100 years. Following the removal, salinity reached up to 15 PSU annually, peaking in summer @yabusakiFloodplainInundationSalinization2020. Many spruces survived about 5 years in these newly-saline conditions before dying.

This pattern of carbon starvation without hydraulic failure is somewhat unusual in the context of existing literature, where hydraulic failure has been described as "ubiquitous" in drought-associated tree mortality @adamsMultispeciesSynthesisPhysiological2017. We might expect responses to salinity to more closely resemble drought responses than we've actually observed, though this comparison is limited by the relative scarcity of empirical studies examining tree mortality in response to salinity, particularly in glycophytes. When a tree succumbs to just one mode of failure while maintaining headroom in another, it suggests potentially suboptimal resource use. Ideally, trees would deplete both carbon reserves and hydraulic capacity to maximize survival under stress.

The sudden change in salinity at Beaver Creek represents a rapid-onset stressor with some parallels to flash droughts. In response to acute drought events, plants have been observed to die from hydraulic failure without concomitant carbon starvation a mirror image of the imbalanced resource depletion seen at Beaver Creek, but with the opposite resource left on the table @hoffmannHydraulicFailureTree2011 @arendRapidHydraulicCollapse2021 @biglerDroughtIncitingMortality2006 @wangMortalityPredispositionsConifers2021. Under such acute stress, trees can only deploy fast-acting responses like stomatal regulation, while longer-term adaptations—such as alterations to sapwood characteristics—require extended implementation periods or significant carbon investment. The speed of environmental change may fundamentally constrain trees' ability to optimize their physiological responses. Although the Beaver Creek spruces exhibited apparently uncoordinated responses, this may not have cost them much in an essentially unwinnable situation. Their high foliar ion concentrations compromised both photosynthesis @liChangesCarbonNitrogen2021 and turgor maintenance @zhangSeawaterExposureCauses2021, suggesting alternative strategies, such as more moderate stomatal regulation to eke out more carbon gains at the expense of their remaining hydraulic safety, wouldn't have significantly extended their lifetimes.

In contrast, slower-changing environments may allow for more coordinated physiological responses, and those responses may play a much larger role in determining outcomes. As conditions gradually become unfavorable, even small adaptive benefits can accumulate over time, allowing trees to remain profitable longer, begin eventual declines from more advantaged positions, and decrease their resource consumption rates to deteriorate more slowly. Gradual environmental changes may facilitate better coordination between fast and slow adaptive responses, potentially creating synergies that improve plant outcomes (@fig_interaction_of_resilience). Our study system on the Delmarva Peninsula offers a unique opportunity to examine trees with ample time to acclimate to slowly changing conditions.

Hydraulic vulnerability segmentation represents a key adaptive trait that may shift across environmental gradients. Plants typically exhibit greater vulnerability in leaves and roots compared to stems, creating "safety valves" that protect carbon-expensive stem tissues @pivovaroffCoordinationStemLeaf2014 @johnsonTestHydraulicVulnerability2016. Given that increased sapwood density is associated with inreased hydraulic resilience, one might expect trees in more saline areas to have denser sapwood. Counter to expectations, our collaborator Stephanie Stotts found that sapwood density in Pinus taeda decreases along increasing salinity gradients. This may indicate strategic resource reallocation. Vulnerability segmentation may function as a "luxury good" with elastic demand: a trait that becomes less affordable or worthwhile to invest in under stressful conditions. By reducing stem density (and consequently, the carbon investment per unit of sapwood), trees may achieve meaningful cost savings that can be redirected toward more critical functions in less productive environments, particularly as declining leaf area decreases hydraulic demand.

Turgor loss point (TLP) represents another critical physiological trait that plants may acclimate across environmental gradients. TLP is the leaf water potential at which cells lose turgor pressure, with profound implications for cellular function and growth @beckettPlasticityBranchWater2024 @bartlettDeterminantsLeafTurgor2012. Plants can adjust their TLP through several mechanisms that operate on different timeframes, including osmotic adjustment and producing new leaves with different characteristics @ellerCloudForestTrees2016. As stomatal closure occurs near the turgor loss point @jiangJiangGuoFengCoordinationHydraulicThresholds2022, improvements to TLP may keep plants profitable for longer in unfavorable conditions.

Photosynthetic acclimation represents a third critical adjustment strategy for trees navigating changing environmental conditions. At our coastal forest site, the salinity gradient coincides with increasing light availability, creating a complex optimization challenge. While salinity generally impairs photosynthetic capacity through reduced mesophyll conductance, ionic toxicity, and damage to photosynthetic machinery @dingModelingMechanismsConifer2023a, increased light availability typically favors greater investment in photosynthetic capacity @lamourEffectVerticalGradients2023. Plants show remarkable plasticity in both structural and biochemical photosynthetic traits, adjusting maximum carboxylation rate (Vcmax) and electron transport capacity (Jmax) to match prevailing conditions @niinemetsWorldwideAnalysisCanopy2015. The extent to which plants at our field site capitalize on increased light availability along the forest dieback gradient remains unclear.

By measuring key variables across the salinity gradient—including growth rates, mortality rates, canopy changes, and physiological indicators like leaf water potential, foliar ion concentration, and non-structural carbohydrate concentrations in both leaves and sapwood, we can quantify how these trait adjustments affect plant vulnerability. Integrating these measurements with environmental data reveals the emergent whole-plant strategies trees employ when navigating increasingly stressful conditions, providing insights into the coordinated physiological responses that determine which trees will live and die in changing coastal forests.

== Methods

_Sampling Design_

We conducted this study at the Brownsville Preserve (37.5°N, -75.8°W) near Nassawadox, Virginia, a mid-Atlantic coastal forest characterized by co-occurring gradients in soil salinity and woody plant species composition. Our sampling design included four evergreen woody species: Gymnosperms _Pinus taeda_ (Loblolly Pine) and _Juniperus virginiana_ (Eastern Red Cedar), and angiosperms _Morella cerifera_ (Wax Myrtle; syn. _Myrica cerifera_), and _Ilex opaca_ (American Holly).

The sampling design reflected each species' distribution along the site's salinity gradient. For _P. taeda_ and _I. opaca_, we selected three individuals at each species' range edge (high and low salinity), while for _J. virginiana_ and _M. cerifera_, we sampled two individuals from each range edge plus two individuals in the center of the range. From each plant, we collected two intact sets of leaves and distal stem segments.

#figure(
  image("images/maggie_coolplotmap.png", width: 100%),
  caption: [
    Species ranges within the Brownsville Preserve for our water potential sampling design.
  ],
)<fig-site_map>

We performed these collections a total of 16 times between June 2023 and September 2024, totaling over 1000 samples. Collections include predawn (one hour before sunrise) and midday (14:00-15:00 local time) sampling periods, with all samples collected within a one-hour window. Predawn and midday collections were paired (e.g. executed within the same 24-hour window) when permitted by weather. Following @rodriguez-dominguezLeafWaterPotential2022, we stored samples in deflated plastic bags with moistened paper towels to prevent desiccation prior to analysis.

\
_Water Potential Measurements_

We measured leaf water potential (Ψ) using a pressure chamber (#link("https://www.pmsinstrument.com/products/model-600d/")[Model 600D], PMS Instrument Company) and nitrogen gas. We allowed each sample to equilibrate in its humidified bag for a minimum of 15 minutes. We left approximately 1 cm of stem attached to each leaf sampled to standardize the diameter of the samples for the instrument. We routed the pressurized nitrogen through water-saturated paper towels prior to chamber entry to minimize artifactual tissue dehydration during measurement.

\
_Non-Structural Carbohydrate Quantification_

To assess concentrations of sugars and starches in the collected leaves, we're following the procedures established in @landhausserStandardizedProtocolsProcedures2018. Efforts are underway to process the approximately 1000 sets of pooled samples for analysis.

In addition, this summer, we will sample sapwood cores across the salinity gradient at the end of the growing season to get an estimate of long-term carbon storage. 

\
_Hydraulic Conductivity Measurements And Vulnerability Curves_

I developed low-cost hydraulic conductivity flow meter to measure vulnerability to hydraulic failure across species and spatial gradients. 

#figure(
  image("images/flowmeter_resistors.jpeg", width: 100%),
  caption: [The world's cheapest (to my knowledge) hydraulic conductivity flow meter. I hooked up two #link("https://www.qosina.com/6-gang-stopcock-manifold-7-female-luer-locks-male-luer-with-spin-lock-non-vented-caps-17554-2?VariantID=Version_v-01")[enormous manifolds from Qosina] together so all of the resistor tubes can stay on the device concurrently, opened or closed via a pair of stopcocks. This makes calibration a bit faster and reduces the number of ways bubbles can get into the device. To keep the resistors organized (at least more organized than the electronics in @electronics_fig), I 3D printed some tube wranglers. You can see one of the pressure transducers on the left, adjacent to the syringe near the red and green stopcocks. The line is fed with water from a graduated cylinder elevated on a lab stand, as is typical. The device calculates flow rates by comparing the difference in pressure drop between a plant sample and a calibrated resistor.]
)

Our design builds upon the established apparatus described in #link("https://prometheusprotocols.net/function/water-relations/hydraulic-conductance-and-conductivity/constructing-and-operating-a-hydraulics-flow-meter/")[Prometheus Protocols] and @melcherMeasurementsStemXylem2012, but introduces a few novelties to dramatically reduce the cost of the device:

- A microcontroller-based datalogging system (<\$3) with custom firmware
- Select components replaced with 3D-printed alternatives
- An open-source cross-platform desktop client application for data acquisition and analysis

These modifications reduced the total construction cost by approximately \$680 compared to the original Prometheus design, largely a product of replacing the #link("https://www.newark.com/ni/779026-01/usb-6009-multifunction-i-o-device/dp/14AJ4776")[analog to digital interface] with the microcontroller shown in @electronics_fig. For labs that already have basic equipment like stands and graduated cylinders, the improved flowmeter design only costs about \$350 to build. The complete hardware specifications and software implementation will be published separately as an open-source resource for the scientific community.

This summer, I will use this device to construct segmented hydraulic vulnerability curves following the project's water potential sampling design.

\
_Pressure-Volume Curves_

In order to estimate turgor loss point and changes to capacitance, we will measure pressure-volume curves following a design similar to the water potential sampling. In addition, we will use a "big shot" slingshot to sample additional species with more restricted ranges within the site, such as _Acer Rubrum_, _Liquidambar styraciflua_, and _Nyssa sylvatica_.

\
_Foliar Ion Concentration_

We will ship a subset of the non-structural carbohydrate samples to an analytical lab to measure foliar ion concentration across species and space. 

\
_Forest Censuses_

My laboratory group has maintained a census of the forest at Brownsville since 2020. The census covers 28 plots, 20 by 20 meters in size, distributed across the salinity gradient. Every tree in each plot above 50mm diameter at breast height (dbh) is tagged, identified to the species level, and included in the survey. The census includes a once or twice-annual liveness check, canopy health score, check for new growth above the dbh threshold. Each tree is associated with a GPS point to approximately 3m accuracy.

In addition, we have a separate census of seedling distributions within subsets of these plots.

#figure(
  image("images/flowmeter_electronics.jpeg", width: 100%),
  caption: [Wires are messy but it works! On the left breadboard, there's a Raspberry Pi Pico (\$2.99) microcontroller, with a USB connection to a laptop for both power and serial communication. It runs homemade firmware, written in Rust using the Embassy framework for asynchronous communication. Together, these replace expensive closed-source data loggers and logging software. On the right breadboard, there are three identical INA128PA instrument amplification circuits (\$9.99 each), each connected to a PX26-005GV pressure transducer (\$79 each) on the flowmeter.]
) <electronics_fig>

_Canopy Health Estimates_

Aidan Brown, an undergraduate I've mentored, fine-tuned a model based on DeepForest @weinsteinDeepForestPythonPackage2020 to detect tree canopies from drone footage and classify them as living or dead (@aidanfig). We can leverage this classifier to extend the coverage of the forest census. If the classifier is able to correctly identify mortality in the censused plots, we can measure canopy metrics (e.g. size, greenness) continuously across the entire Brownsville preserve rather than just in the censused plots.

_Photosynthetic Traits_

This summer, following the design of the water potential measurements, we will measure CO2 (assimilation / intracellular carbon concentration) and light response curves. Light availability generally increases along the salinity gradient at our site, and increased photosynethetic efficacy (e.g. via greater investment in photosynhetic machinery per-leaf on a smaller overall canopy size) may present an opportunity for plants to adjust their strategy as their context changes.

#figure(
  image("images/canopy_model.png", width: 100%),
  caption: [A: Canopy recognition and classification model, courtesy of Aidan Brown. Orange squares represent detected live canopies, red squares denote detected dead canopies. B: 3D reconstruction of the Brownsville site generated from a set of 2D images. With continued improvements to our drone flights, we may be able to measure changes in canopy size in individual trees in three dimensions rather than two.
  ]
)<aidanfig>

== Results And Discussion

#figure(
  image("images/water_potential3.png", width: 100%),
  caption: [Left: predawn (blue) and midday (red) water potentials ($psi$), measured in megapascals, by species and range from June 2023 through March 2024. Right: predawn and midday percentage loss of hydraulic conductivity, using leaf vulnerability curves sourced from @johnsonTestHydraulicVulnerability2016 and @shiflettCoordinationLeafAnatomy2014. Note that I'm using the same vulnerability curves across the entire species range. Additionally, note that for both panels, while the x-axis represents time, the space between the sampling points is not proportional to the actual duration of time between them to save some space.]
)

As expected, water potentials were more negative at the more saline end (low range) of each species' range. _Pinus taeda_ and _Morella cerifera_ reached water potentials that correspond to elevated risk of mortality in other studies @hammondDeadDyingQuantifying2019 @johnsonTestHydraulicVulnerability2016 @shiflettCoordinationLeafAnatomy2014. While _Pinus_ and _Morella_ may be range-bound by hydraulic vulnerability, _Ilex_ and _Juniperus_ along the marsh-forest ecotone do not reach severe hydraulic stress (e.g. a percentage loss of conductivity (PLC) greater than 60% @adamsMultispeciesSynthesisPhysiological2017) at any point in the growing season. Note that these PLC values were calculated based on literature vulnerability curves derived from the papers referenced above, and are subject to change as we measure our own vulnerability curves at this site.

Why does their range stop at the marsh, then? Some possibilitites include limitations by foliar ion concentration, turgor loss, or anoxic conditions in the soil, especially with regards to the relatively shallow water table in that area. While we don't have an explicit empirical examination of anoxia planned, we may be able to examine those dynamics with a model (e.g. by imposing a penalty on respiration for the portion of the root network below the groundwater table at any time) if we find that some species ranges remain poorly-explained by foliar ions and turgor.

== Questions
+ Is tree survivorship at Brownsville a product of acclimation or selection?

  If we observe variations in sets of plant traits across the salinity gradient at Brownsville, how can we tell whether those differences are attributable to individual plasticity (acclimation) or selective mortality of plants with less favorable trait combinations? By over-replicating our sampling efforts in the higher-elevation populations, we can quantify whether traits exhibited by plants at the marsh edge represent a subset of the trait diversity found in upland populations (suggesting selection) or whether they form a distinct trait profile (indicating acclimation).

  \
+ Is the discrepancy between seedling range and mature tree range for a given species better explained by carbon storage or trait acclimation?

  By examining growth rates and changes to canopy health in mature trees below elevation thresholds for seedling establishment, we can distinguish whether plants can move the threshold between a positive and negative market: continued growth suggests that trees have successfully acclimated to extract value where unacclimated trees can't, while growth stagnation and canopy dieback indicates reliance on stored reserves.

  \
+ Do trees at Brownsville experience greater degree of coordination between vulnerability to hydraulic failure vulnerability to carbon starvation than trees at Beaver Creek?

  While drought and salinity stress share similarities, they may impose fundamentally different constraints on plant physiology. The gradual pace of environmental change at Brownsville could allow trees to optimally distribute stress across multiple physiological systems, unlike the rapid changes at Beaver Creek. Alternatively, the unique challenges of elevated salinity, particularly ion accumulation in photosynthetic tissues, may drive different tradeoffs between productivity and safety than those documented in drought studies. By comparing physiological coordination across these systems, we can determine whether salt stress represents a qualitatively different challenge requiring novel adaptive strategies beyond those observed in drought responses.

== Acknowledgements

Thanks to Aidan Brown, Maggie Connolly (former GWU undergrad), Jess Liu (former GWU tech), Juan Ignacio Martinez (a tall handsome stranger), Aaron Mendez (former high school student intern), and Keryn Gedan (big boss) for their willingness to wake up at 3:30am. Thanks to Ellyn Kinkel (GWU undergrad) for her help processing the non-structural carbohydrate samples. Thanks to Justus Jobe for lending me his paintball gun for my ridiculous first attempt at getting leaves down from the canopy.

= Foliar Water Uptake Keeps Dying Trees Alive <fwu>

== Background
Water transport in plants is classically explained by the cohesion-tension theory (CTT) @boehmCapillaritatUndSaftsteigen1893 @dixonXIIAscentSap1895. Although the CTT is a simplified model @rennerExperimentelleBeitrageZur1911 and has occasionally been the subject of controversy @zimmermannWaterAscentTall2004, it is widely endorsed by plant physiologists @bentrupWaterAscentTrees2017. According to the CTT, water movement through the soil-plant-air continuum (SPAC) is entirely passive, and thus a product of the relative hydration of a plant's tissues and the surrounding environment. Typically, the atmospheric demand for water exceeds the water potential in a leaf, which itself exceeds the water potential in the soil. This gradient draws continuous chains of water molecules, held together by cohesive forces, upward through the plant's xylem.

While the typical direction of water movement through the SPAC is from soil to atmosphere, water simply moves in the direction where a favorable water potential gradient exists @schreelFoliarWaterUptake2020. When atmospheric water availability exceeds leaf water potential, for example, in foggy conditions at night after a dry day, plants can absorb water directly through their leaves, a process known as foliar water uptake (FWU) @ellerFoliarUptakeFog2013.

FWU has been documented in hundreds of species across diverse biomes. Approximately 90% of studied species were found to be capable of FWU @dawsonValueWetLeaves2018. Recent research indicates that conditions favoring FWU, such as rain, fog, or dew, occur over 100 days per year on average across the globe @dawsonValueWetLeaves2018. Importantly, complete leaf wetting or reaching dew point temperature is not required; FWU can occur whenever a water source and favorable water potential gradient exist.

Multiple pathways facilitate FWU. Water can diffuse directly through the leaf cuticle, with cuticular permeability increasing during periods of high humidity @fernandezPhysicochemicalPropertiesPlant2017. Stomata may contribute to FWU through a process called hydraulic activation, where thin water films connect the leaf surface to internal tissues @burkhardtStomatalPenetrationAqueous2012. Additional pathways include specialized structures such as trichomes and hydathodes, though their relative importance varies among species @berryFoliarWaterUptake2019.

Once water enters leaves through FWU, it follows water potential gradients throughout the plant, potentially flowing from leaves to stems and even to roots @steppeDirectUptakeCanopy2018. This hydraulic redistribution can help maintain tissue hydration when soil water is limited, demonstrating how the CTT's passive water transport mechanism can benefit plants even when flow directions are reversed @schreelInfluenceDroughtFoliar2019. The magnitude of water transport through FWU can be described using the same physical principles as conventional water movement, with flow rate dependent on the water potential gradient and the hydraulic conductance of the uptake pathway @binksFoliarWaterUptake2019.

FWU can provide a source of water during periods of soil water deficit, when conventional root-based water uptake is limited. Research in Amazonian trees has shown that FWU can contribute to carbon assimilation equivalent to more than 8% of gross primary production @binksFoliarWaterUptake2019. As climate change increases the frequency and severity of drought events, understanding the role of FWU in plant water relations becomes increasingly important for predicting vegetation responses to future conditions @schreelFoliarWaterUptake2019. 

FWU's role in plant survival under environmental stress takes on new significance as climate change presents multiple challenges: increased drought frequency and severity @schreelFoliarWaterUptake2019, as well as elevated soil salinity from rising sea levels @kirwanSealevelDrivenLand2019. While FWU's function in drought conditions is increasingly well-documented, its role in saline environments remains understudied despite intriguing parallels - both conditions create strongly negative soil water potentials that challenge plant hydraulic function. Field observations of _Juniperus virginiana_ leaf water potential at the forest-marsh ecotone of my field site in Nassawadox, VA illustrate this paradox, where trees in standing water can experience water potentials below -3 megapascals due to soil salinity. Coastal forests serve as natural laboratories for understanding FWU's role in plant survival under water stress, as their salinity gradients create proximate populations experiencing vastly different levels of hydraulic stress. These landscapes allow for direct comparisons between physiologically stressed and healthy individuals of the same species growing under otherwise similar conditions, providing insights that can inform our understanding of both coastal and drought-stressed systems.

== Methods

_Experiment_

We performed an experiment (@fig-fwu-exp) on Loblolly pine (_Pinus taeda_) saplings at our field station in Oyster, Virginia to confirm that foliar water uptake occurs naturally in our system. Distal pine branches with intact needles were subject to one of three treatments: control, bagged, or sprayed. Sprayed samples were thoroughly doused with water every hour between sunset and predawn (1 hour before sunrise). Bagged samples were sealed in ziplock bags that were then sealed again with duct tape, and covered in aluminum foil. We replicated the set of treatments twice within each individual across three individual pines. At predawn, we collected the samples, stored them briefly, and allowed them to equilibrate in accordance with best practices for water potential measurements established in @rodriguez-dominguezLeafWaterPotential2022. We measured the water potential of one distal stem and two needles per treated sample.

  #figure(
    image("images/fwu_experiment_design.png", width: 100%),
    caption: [Foliar water uptake experiment design. Keen-eyed observers might notice that this figure doesn't actually depict a Loblolly pine (_Pinus taeda_) like we used in the experiment. ]
  )<fig-fwu-exp>


_How Much Water Can Plants Take Up Via FWU?_

To estimate hydraulic conductance from FWU, we will conduct a laboratory experiment where we progressively dry down leaves and rehydrate them in both humidified air and in simulated rainfall, measuring their rehydration rates over time. Crucially, we'll use water potential and a pressure-volume curve to estimate their rehydration rates, to avoid error due to condensation on the surface of the leaf that hasn't yet been taken up. This will provide meaningful hydraulic parameters and shed light on whether we can use a simple conductance relationship to estimate differences in FWU efficacy across the salinity gradient, as in @binksFoliarWaterUptake2019, or whether we need to use any sort of scaling for conductance by water potential, as in a typical hydraulic vulnerability curve.

\
_Integration With Hydraulic Model_
To estimate the relative contribution of FWU to plant hydraulic and carbon regimes, I will use the empirical parameters measured above to integrate FWU into an assimilation optimization stomatal model, based on @sperryPredictingStomatalResponses2017. This can better contextualize the net benefits of water acquired with respect to environmental conditions, plant hydraulic safety, capacitance, and traits that change across the gradient, like leaf area to sapwood area ratio.

== Results and Brief Discussion

  #figure(
    image("images/fwu_experiment.png", width: 100%),
    caption: [Foliar water uptake experiment results.]
  )<fig-fwu-exp-results>

We confirmed experimentally that _Pinus taeda_ (Loblolly pine) saplings in Oyster, VA experienced FWU (@fig-fwu-exp-results). In bagged tissues, which were blocked from the possibility of performing FWU, needles were found to be less hydrated than the stems they were attached to. In unbagged tissues, we found the opposite: _needles were more hydrated than the stems they were attached to_, strong evidence for FWU. In the spray treatment, needles and their stems were equilibrated, but at a more hydrated water potential than either of the other two treatments, suggesting that the very light rain that occurred naturally during the experiment did not saturate the needles' capacity for FWU. Further, it seems that FWU has sufficient conductance to recharge at least some of the capacitance in the stem during heavy rain events, an effect we hope to quantify in a detailed modeling approach.

Interestingly, though the unbagged treatment had better-hydrated needles than stems, both tissues had more negative water potentials on average than the stems in the bagging treatment. This suggests the bagging treatment may have prevented some nighttime transpiration earlier in the evening that occurs naturally.

== Questions

+ How much does FWU improve hydraulic safety? Do worse-off plants benefit more from FWU than healthy plants?

  #figure(
    image("images/fwu_hsm.png", width: 100%),
    caption: [Rough initial estimate of hydraulic safety margin (HSM) improvements from foliar water uptake. Given the approximately 0.2 MPa improvement in needle water potential between the sprayed and unbagged treatments in my FWU experiment, the water potentials I measured in the field in @hydraulics-chapter, and a _Pinus taeda_ needle vulnerability curve from @johnsonTestHydraulicVulnerability2016, stressed pines at the marsh-forest ecotone stand to improve their HSMs nearly twice as much as healthy upland pines by percentage. I expect this differential to be even greater in a more realistic model, where relatively larger pressure gradients between stressed needles and the air drive faster flow rates.]
  ) <fig-fwu-hsm>


  Although FWU usually provides relatively small volumes of water, it may serve as a lifeline for stressed plants. In the steepest regions of the hydraulic vulnerability curve, improvements in water potential yield superlinear improvements in hydraulic safety margin @fig-fwu-hsm. Given that we've observed a correlation between hydraulic stress and salinity, dying plants may benefit disproportionately from FWU, delaying mortality.

  Additionally, stressed plants may also be more effective at acquiring water via FWU than their healthy counterparts. Due to steeper water potential gradients, plants under greater hydraulic stress may encounter conditions for FWU more frequently and for longer durations. Further, they may experience enhanced uptake rates during each FWU event @binksFoliarWaterUptake2019. However, several factors complicate this hypothesis. Reduced leaf area associated with high salinity could limit a plant's capacity to capture and redistribute foliar water despite those stronger gradients, especially during fog or rain events that last long enough for FWU-acquired water to reach large capacitors in stems and roots. Additionally, salinity-induced changes in leaf traits critical to FWU, including stomatal characteristics (Emery & Martinez, in progress) and cuticular properties may alter uptake capacity across forest dieback gradients.

  \
+ Do plants know to expect FWU?

Preliminary evidence suggests plants experience nighttime water losses prior to foliar water uptake events. Nighttime stomatal conductance, observed in virtually all C3 and C4 plants at rates averaging 12% of daytime values @rescodediosAssessingPotentialFunctions2019, may serve an unrecognized adaptive role in coastal forests. This transpiration creates steeper water potential gradients that serve dual purposes: initially accelerating upward water flow to recharge stem capacitance, then enhancing foliar water uptake efficiency when humidity peaks. Given the predictable diurnal and seasonal patterns of foliar water uptake opportunities in our system, this strategy may represent a calculated investment where plants "spend" water early to recover more later through both foliar and root pathways. Process-based models could help determine whether this apparent bet on future conditions provides net hydraulic advantages in increasingly saline coastal forests.

== Acknowledgements

Thanks to Kaeden Rippon (UVA undergrad) for staying up all night with me a few times hunting for the right conditions, and to Dr. Alex Pivovaroff for helpful design suggestions.

= Hydraulic Capacitance And Foliar Water Uptake Buffer Coastal Forests Against Seawater Inundation

== Background
Sea level rise is often discussed in terms of changes to the mean sea level over time, typically measured in millimeters per year. However, this is a simplification. Sea levels follow a tidal distribution, and the entire tidal distribution is shifting higher over time. For elevations at the tails of tidal distributions, high-tide flood frequencies are expected to increase exponentially over time as the center of the tidal distribution moves closer @sweetGlobalRegionalSea2022.

This exponential increase in inundation frequency with linear sea level rise creates an urgent challenge for coastal ecosystems. As the distribution of high-water events shifts upward, even modest increases in mean sea level can dramatically increase the exposure of coastal forests to saltwater stress, potentially driving major ecological transitions from forest to marsh landscapes.

Coastal plant communities face a dual challenge from these changing environmental regimes: "pulse" stressors — discrete flooding events that shock ecosystems with sudden exposure to extreme conditions — and "press" stressors—the gradual but persistent increase in average water level and salinity over extended periods. While pulse events may trigger immediate physiological responses or even mortality, press stressors can progressively alter habitat suitability and ecosystem composition over time.

#figure( image("images/sweet2022_fig1-3.png", width: 100%), caption: [ a) Annual probability density and b) annual expected exceedances for daily highest water levels relative to the 1983–2001 mean higher high water (MHHW) tidal datum showing increases in NOAA minor, moderate, and major high tide flooding (HTF) probabilities/frequencies due to relative sea level (RSL) rise at the NOAA tide gauge in Charleston, South Carolina. Reproduced from @sweetGlobalRegionalSea2022. ], )<sweetfig_htf>

@sweetfig_htf shows these sharp increases in the frequency of minor high-tide flooding in Charleston, South Carolina over recent decades. This phenomenon is widespread: high-tide flooding is more than twice as frequent across the United States in 2020 compared to 2000, and projections estimate another doubling by 2030.

This exponential increase in inundation frequency with linear sea level rise creates an urgent challenge for coastal ecosystems. As the distribution of high-water events shifts upward, even modest increases in mean sea level can dramatically increase the exposure of coastal forests to saltwater stress, potentially driving major ecological transitions from forest to marsh landscapes.

Coastal plant communities face a dual challenge from these changing environmental regimes: "pulse" stressors — discrete flooding events that shock ecosystems with sudden exposure to extreme conditions — and "press" stressors—the gradual but persistent increase in average water level and salinity over extended periods. While pulse events may trigger immediate physiological responses or even mortality, press stressors can progressively alter habitat suitability and ecosystem composition over time.

In spring 2021, we documented one such pulse stressor: a seawater inundation event at the Brownsville Preserve near Nassawadox, VA that induced reverse sap flux in loblolly pines across a salinity gradient. Floodwaters peaked at 1:00am on May 30 in the lowest elevation plots adjacent to the salt marsh (mean elevation 0.97 m above MSL), and progressed to higher elevation plots (mean elevation 1.05 m above MSL) two hours later at 3:00am.

Although surface flooding subsided relatively quickly (within 5-30 hours), the event's impacts persisted in the soil for several weeks. Groundwater salinity increased dramatically across the study area, rising from baseline levels of 1 ppt to 19 ppt in the highest elevation plots and from 11 ppt to 30 ppt in the lowest elevation plots (herein referred to as "low forest"). Both soil moisture and soil salinity remained elevated for weeks following the initial inundation, creating prolonged stressful conditions for the plant community.

#figure( image("images/revsap_env.png", width: 100%), caption: [ Hydrologic conditions in plots during the period of May 17-June 7, 2021, two weeks prior and one week following the flood event that began on May 30, 2021. a) Groundwater level in cm below ground surface (b.g.s.). The dashed line represents the soil surface at 0 cm. b) Groundwater salinity, c) Soil water content, and d) Soil specific conductivity (at 25 °C) in the five instrumented plots. ] )<revsap_env>

This inundation event provided an opportunity to examine how trees respond to and recover from pulse stressors. Understanding this response requires considering the suite of physiological mechanisms that coastal trees may employ to maintain function despite osmotic shock. 

Plants employ several adaptive mechanisms to withstand these challenging conditions. Non-structural carbohydrate reserves serve as critical buffers, providing energy when photosynthesis is limited and supporting osmotic adjustment @zhangDecliningCarbohydrateContent2021. Hydraulic capacitance plays a particularly important role by buffering plants against spikes in osmotic tension, such as those imposed by saline flood events. Recently, capacitance was shown to play a significant role in mangrove acclimation to elevated salinity @beckettPlasticityBranchWater2024. Interestingly, this leads to a physiological tradeoff in stressed coastal forests: canopy dieback reduces overall hydraulic capacitance, potentially increasing vulnerability to hydraulic failure during "pulse" events, yet simultaneously decreases respiratory costs and hydraulic demand, potentially improving carbon balance and reducing osmotic stress on the root network. As pulse events increase in frequency, duration, and intensity over time, does the adaptive benefit of a reduced canopy diminish, or even become a net detriment?

Additional resilience mechanisms include foliar water uptake (detailed in section @fwu), which provides an alternative freshwater source during periods when soil water is inaccessible or osmotically unfavorable. Given that seawater surges often coincide with rainstorms, foliar water uptake could play a role in keeping canopies hydrated while water potential gradients between roots and soil are inverted. At the cellular level, aquaporins—water channel proteins small enough to exclude sodium ions—provide a sophisticated regulatory system for water transport under saline conditions @domecAquaporinsNotChanges2021. These transmembrane proteins can rapidly adjust hydraulic conductivity, with plants capable of reducing root conductance by up to 70% within 45 minutes of salt exposure as a protective response @boursiacEarlyEffectsSalinity2005. Aquaporins contribute significantly to total hydraulic conductance: between 35-55% in roots and 10-30% in stems. They also operate across multiple timescales. Short-term regulation occurs through post-translational modifications and protein trafficking, while longer-term responses (hours to days) involve coordinated changes in gene expression and protein abundance. This system allows plants to dynamically manage water relations during both pulse and press salinity stress.

Understanding the interaction between pulse and press stressors, and thus when, where, and why trees are vulnerable, requires integrating these physiological mechanisms into process-based models. By representing the causal relationships between salinity exposure, inundation frequency, and plant responses, such models can generate mechanistically interpretable predictions about coastal forest persistence or mortality under various sea level rise scenarios.

== Methods

_Sap Flux_

In 2020, we instrumented 4 _Pinus taeda_ with heat pulse sap flux sensors at breast height across 3 zones of forest dieback at the Brownsville Preserve: Low Forest (significant canopy dieback), Mid Forest (moderate canopy dieback), and High Forest (little to no canopy dieback). Note that this project predates the existence of the "Reference Forest" zone, as the "High Forest" was healthy at this time. Measurements were collected continuously on 30-minute intervals.

_Environmental Data_

Meteorological and hydrological data were provided by collaborators in the Virginia Coastal Reserve Long-Term Ecological Research (LTER) project and the Coastal Critical Zone Network (CCZN) project.

_Process Model_

My initial approach at modeling involved parameterizing a stomatal optimization model @sperryPredictingStomatalResponses2017 with morphological measurements from our forest census and physiological traits (e.g. hydraulic vulnerability curves @johnsonTestHydraulicVulnerability2016 and photosynthetic parameters @myersPhotosyntheticCapacityLoblolly1999) from the literature, and meteorological and hydrogeological characteristics from our collaborators. Our model design was intended to simulate the pines' hydraulic function as if the seawater inundation had not occurred, but the other climatic factors were held exactly the same. This provided a pseudoexperimental counterfactual to our observational study in the field to try to isolate the effects of the event. We ran a model starting before the event and let it continue for weeks after the event, to test if the agreement between the model and reality changed as a function of the inundation. For details on next steps to evaluate the impacts of physiological context on these flood responses, see Question 2.

== Results

Surprisingly, we found no significant effect of the flood event on sap flux density in the weeks following the inundation, despite persistently elevated salinity levels. To investigate this apparent resilience, we parameterized a stomatal optimization model @sperryPredictingStomatalResponses2017 representative of loblolly pines across our site. The model, which had no information about the flood event, maintained strong predictive power before and after the inundation. While observed sap flux rates were generally lower post-flood, the model attributed this decline to less favorable meteorological conditions rather than to flood-induced physiological damage (@revsap_models).

#figure(
  image("images/revsap_models.png", width: 100%),
  caption: [
    Modeled and measured sap flux density from Brownsville Forest, before and after the saltwater flooding event in 2021. Each colored measured trace is an average of the instrumented Pinus taeda trees from a forest level for two weeks prior and one week after the event. Each gray line represents a modeled expected sap flux density value for each level had the flood event not occurred.
  ]
)<revsap_models>

The model demonstrated exceptional explanatory power across the site's elevation gradient, with R#super[2] values of 0.85, 0.89, and 0.75 in the High Forest, Mid Forest, and Low Forest zones, respectively. Notably, while the explanation of variance was strong, model predictions did not fit the real data 1:1. Modeled trees generally transpired more than their real-life counterparts, but differences between the model and reality were consistent before and after the event (@revsap_modelfit).

#figure(
  image("images/revsap_modelfit.png", width: 100%),
    caption: [
      Comparison of modeled and measured values, before (blue) and after (red) the event. The dashed one-to-one line represents perfect model accuracy.
    ]
)<revsap_modelfit>

== Questions and Ongoing Work

+ What mechanisms explain how reverse sap flux stopped after merely a few hours after the flood onset despite persistently high soil salinity? What mechanisms explain the apparent lack of a lasting impact on plant hydraulic function?

  We suspect that aquaporin regulation, hydraulic capacitance, and foliar water uptake contribute to the observed plant responses. Aquaporins can be altered on short timescales, perhaps allowing trees to maintain sufficient conductance to supply their canopies following an event that interferes with other modes of hydraulic transport. Water from the canopy and sapwood is distributed to the roots during the event (the negative or reverse sap flux we observed at the sensor on the trunk), reducing the impacts of the sudden changes in osmotic potential by distributing them around the plant. Foliar water uptake may further reduce the impact of those hydraulic redistributions by adding more freshwater into the system. Inundation events often co-occur with heavy rains, favorable conditions for foliar water uptake to occur in.

  \
+ Can we reconstruct the pines' response to these environmental conditions in a process model?

  To demonstrate how those mechanisms interact in the context of the plant and the environment, we will construct a stomatal optimization model capable of representing reverse flows, foliar water uptake, and multiple pathways of root water uptake. This essentially serves as a proof by construction -- if those are really the mechanisms that generate the observed outcomes, we should be able to construct them ourselves to demonstrate causality, e.g. by showing declining plant hydraulic safety if we "turn off" foliar water uptake, by demonstrating the relationship between canopy capacitance and whole-plant hydraulic safety, and so on. Fortunately, we can leverage the data collected in @hydraulics-chapter and @fwu to parameterize these models, with one addition: we will perform an experiment to determine the contribution of aquaporins to root conductance across the salinity gradient following methods from @domecAquaporinsNotChanges2021.

  \
+ Do dying trees (e.g. those with smaller canopies) become more vulnerable to death by pulse event as they lose canopy capacitance?

  I'm particularly interested in this canopy size tradeoff. While the literature often treats foliar dieback as a symptom of decline and an inherently negative process, I'm not so convinced. In particular, the declining respiration costs and reduced hydraulic demand associated with smaller canopies may be better suited to a shrinking (e.g. higher salinity or drier) market, and may permit a longer runway for a tree to persist, essentially by running a smaller businesses with a lower cost basis. While there are obvious photosynthetic opportunity costs associated with smaller canopies, it's important to quantify other potentially negative feedbacks to the hydraulic network to get a more complete picture of how we should price canopy dieback in models.

  \
+ How much does pulse event timing matter? Are trees at the end of the growing season more vulnerable than at the start?

  I have evidence from my water potential measurements that coastal pines have much lower hydraulic safety margins at the end of the growing season, especially at the saline edge of their range. Did the June 2021 inundation event have such a muted effect because it happened to catch those trees at a time when they had ample hydraulic safety margins? We can leverage our ongoing sap flux data collections to examine more inundation events and their timing, and evaluate whether the same physiological models can describe responses to inundation events distributed around the clock and around the calendar. 

= What Do The Trees Know? <wdttk>

Empirical study of tree mortality over the last several decades has produced an astonishingly large body of research @mcdowellMechanismsWoodyplantMortality2022. Despite this extensive work, we face the challenge of organizing our findings into a coherent quantitative framework—one flexible enough to integrate the numerous mechanisms that govern whether and for how long trees survive in a changing climate.

#figure( image("images/death_spiral_other.png"), caption:[The interconnected mortality process. A hypothetical representation of the mortality processes from predisposing factors to death. Predisposing factors are linked to mortality via the mechanisms in the second innermost ring, which subsequently cause a plant to pass a threshold beyond which mortality is inevitable. The death spiral results from the interaction of external drivers, the processes of hydraulic failure and carbon starvation, and their underlying, interdependent mechanisms. VPD is short for vapor pressure deficit. The figure is sourced from @mcdowellMechanismsWoodyplantMortality2022 and was originally inspired by @brodribbLearningCenturyDroughts2020.] )

A litany of factors may influence tree mortality, with relevant factors varying by local context. These factors often interact, causing the number of relationships to examine to grow exponentially beyond what researchers can feasibly study case-by-case. When measuring plant traits in situ, researchers could explain roughly 60% of the variance in observed mortality rates @powersCatastrophicTropicalDrought2020, yet a meta-analysis using mean trait values from a database explained only about 30% of this variance across locales @andereggMetaanalysisRevealsThat2016 @trugmanWhyTreeDrought2021. While cross-cutting trends emerge, we lose specificity when we zoom out. If we can't measure everything everywhere, how can we build models that work well across contexts?

In 1977, Graham Farquhar tackled a similar problem: modeling stomatal behavior, which is controlled by several complex interacting processes occurring at scales inconvenient to measure directly. Rather than attempting to map every mechanism, Farquhar took a creative approach:

#quote(block: true, attribution:[Graham Farquhar @farquharStomatalFunctionRelation1977])[It is proper to enquire, first, what stomata do; secondly, how they do it; and only then, if the question is allowed at all, why they do it. There are excellent reviews (e.g. @raschkeStomatalAction1975) which deal with stomatal physiology in this way. Here we propose to adopt a different sequence: first, to make an assumption about the role of stomata, to explore the implications of that assumption in terms of stomatal behaviour, and then to enquire whether and how that behaviour is realised in practice.]

Farquhar's insight was to sidestep the complex mechanisms driving stomatal closure and instead focus on the incentives guiding the decision to open or close. He formulated stomatal regulation as a constrained-optimization problem:

$ (delta E) / (delta A) = lambda $

where $E$ represents water transpired, $A$ is carbon assimilated, and $lambda$ is a Lagrangian multiplier. This equation formalizes a simple principle: losing water is costly, while gaining carbon is beneficial. For a given water expenditure, plants must assimilate enough carbon to justify that cost.

Building on Farquhar's foundation, researchers like John Sperry have translated these incentives into physiologically meaningful terms. Sperry's 2017 model @sperryPredictingStomatalResponses2017 incorporates plant hydraulic vulnerability curves and photosynthetic traits to better represent how water becomes increasingly "expensive" as xylem cavitation progresses:

$ (delta beta) / (delta psi#sub[canopy]) = (delta theta) / (delta psi#sub[canopy]) $

where $beta(psi#sub[canopy])$ represents normalized photosynthetic gain and $theta(psi#sub[canopy])$ represents normalized hydraulic cost.

These stomatal optimization models have been applied to forecast tree mortality by establishing specific thresholds of hydraulic failure or carbon starvation that signify death @sperryImpactRisingCO2019 @liInfluenceIncreasingAtmospheric2022. However, while these models provide effective heuristics for instantaneous stomatal regulation, they face critical limitations when applied across longer timeframes and diverse contexts.

Other researchers have built more integrated models around unifying incentives. Aaron Potkay and Xue Feng's excellent model maximizes growth @potkayStomataOptimizeTurgordriven2023:

$ max_(g#sub[c]) integral G(C, E) delta t $

where g#sub[c] is conductance to CO#sub[2], G is growth rate, C is non-structural carbohydrate storage, and E is transpiration rate. This approach has the advantage of unifying multiple functions across the plant around a single incentive that operates at the whole-plant level.

Despite this improvement, growth optimization stomatal models (GOSMs, Potkay-like models) share some limitations with assimilation optimization stomatal models (AOSMs, Sperry-like models). First, they both evaluate the decisions that plants make on an instantaneous basis.

Trees make physiological decisions across dramatically different timescales: stomata open and close within minutes, leaf area adjusts seasonally, and structural traits like sapwood density may take years to meaningfully alter. How do trees align rapid responses with slow-changing traits to maximize fitness? A tree investing in denser, more drought-resistant sapwood over years is incentivized to coordinate this long-term strategy with its day-to-day stomatal regulation. Current models struggle to represent this inter-temporal coordination because they typically operate at a single timescale.

I propose that we can understand how trees coordinate (or fail to coordinate) decisions across these disparate timeframes by modeling trees as if they have implicit expectations about future conditions. While trees do not literally "know" things, they have clearly evolved to exploit asynchronous resource distributions in their environments. For example, deciduous trees shed their leaves before winter, "expecting" conditions where the carbon cost of maintaining those leaves would exceed their photosynthetic benefit. Plants accept hydraulic risk during daylight hours, "anticipating" nighttime recovery periods with more favorable hydraulic conditions. These behaviors allow trees to take on additional or imbalanced short-term risks when those risks are expected to yield long-term benefits. For instance, a tree might maintain higher sapwood density than immediately optimal if it "expects" periodic drought based on evolutionary history.

This capacity to "look ahead" may relate to climate predictability in a given region and the population's residence time in its current location, with more generations in a particular locale potentially providing more fine-tuned behaviors.

Second, implementations of both GOSMs and AOSMs employ fixed strategies that cannot adapt across changing environments. Maximizing lifetime fitness requires balancing two interconnected variables: longevity and productivity. The relative importance of each depends on environmental context. In resource-rich environments, fitness is primarily constrained by productive capacity. In deteriorating conditions, fitness becomes increasingly dependent on an organism's ability to stay alive. When environmental returns become negative, success may hinge on efficiently scaling down metabolic costs. This represents a fundamental strategic spectrum from opportunistic growth to conservative survival—a spectrum along which individual plants may reposition themselves as climate conditions shift.

While evolution incentivizes plants to coordinate all their decisions under some context-dependent strategy, we should not expect perfect optimization. Evolution selects for optimum internal coordination of physiological systems. Despite this strong incentive, plants cannot anticipate unprecedented conditions, and evolutionary history constrains adaptation. Once physiological systems become deeply integrated into a plant's function, they can be difficult to replace even when suboptimal. RuBisCO is the classic example: this enzyme evolved in an oxygen-poor atmosphere and now wastes energy through photorespiration in our oxygen-rich atmosphere @erbShortHistoryRubisCO2018. Similar constraints appear at the whole-plant level, such as structural overshoot in mangroves @jumpStructuralOvershootTree2017, where nutrient enrichment paradoxically increases vulnerability by triggering maladaptive growth patterns.

Trees' immobility and longevity make them ideal systems for studying how organisms coordinate decision-making across timescales. Climate change, while rapid geologically, unfolds relatively slowly compared to a tree's lifespan. The relative contribution of optimal strategic decision-making (acclimation) in this system is magnified compared to plants with faster generation times, such as marsh grasses, where rapid evolution alters responses to climate change @vahsenRapidPlantTrait2023.

The advantage extends beyond convenient timescales. The costs and benefits of tree physiological decisions can be quantified in direct physical and chemical terms, often measurable in the field. While the underlying mechanisms linking physiological systems may be too complex to measure comprehensively, we can approach this complexity by modeling trees as decision-making agents rather than attempting to document every particular interaction.

By modeling trees as decision-making agents that maintain dynamic expectations about their environment, we can make progress towards the challenge of predicting mortality across diverse contexts, addressing the fundamental knowledge gap identified at the beginning of this proposal.

== Methods

=== Acausal Modeling: Separating Mechanism from Strategy

To overcome limitations in current plant models, I propose a computational framework that cleanly separates physiological mechanisms from decision-making strategies. This separation of concerns addresses a fundamental challenge in existing implementations: the conflation of "how plants work" with "how plants decide."

The framework represents adaptive agents making coordinated decisions across multiple timescales within the constraints of mechanistic systems. These systems are implemented as a recursively structured graph where:

+ *Resources* are nodes representing physical quantities or states (e.g., water potential, carbon reserves) with associated numerical bounds and maximum response rates. To enforce these bounds, I developed Floco, an open-source library that provides bounded numeric types for computational models #link("https://crates.io/crates/floco/0.1.3")[available here].

+ *Agents* are nodes that contain other nodes, make concurrent sets of decisions that influence resource states, and maintain expectations about future conditions that guide their decision-making processes.

+ *Edges* are pairs of inverse functions ($f, g$) where $f(g(x)) = g(f(x)) = x$. These represent bidirectional relationships between resources, with an assigned directionality when equality constraints need to be enforced. This bidirectionality allows efficient propagation of constraints throughout the system.

While I am applying this framework to plant ecophysiology, the architecture is general enough to model any decision-making system, and was partially inspired by models of human social learning and inequality @kempInformationSynergyMaximizes2024 @kempLearningIncreasesGrowth2023. This year, I will implement common physiological models (e.g., hydraulic vulnerability curves, photosynthetic models, and the like) as reusable graph components. By representing these processes as acausal graphs, we avoid embedding assumptions about causality in our implementations, facilitating component reuse and allowing efficient traversal between nodes without manually specifying solutions for every combination of constituent equations.

=== Resource Constraints and Valid Decision Space

A key challenge in modeling plant responses is representing how resources constrain each other across multiple dimensions, particularly time. Our framework captures critical constraints:

- *Temporal response rates*: Different plant processes operate at vastly different timescales, from stomatal responses (minutes) to wood density changes (years), creating temporal dependencies between decisions.

- *Reciprocal constraints*: When multiple processes draw from the same resource pool, their combined consumption cannot exceed available supply, enforcing conservation laws across the system.

These constraints define a valid decision space $V(S_t)$ at any given moment:

$ V(S_t) = {D_a^t | forall c_i in C, g_i(S_t, D_a^t) <= 0} $

where $S_t$ is the system state, $D_a^t$ represents potential decisions, $C$ is the set of constraints, and $g_i$ evaluates constraint satisfaction. Decisions must fall within this space to be physically realizable, but the specific path taken through this space over time defines the agent's strategy, a trajectory we deliberately leave undetermined rather than imposing a priori assumptions about optimality.

=== Equality Saturation for Model Integration

Existing approaches to acausal modeling, such as #link("https://modelica.org/")[Modelica], are often incompatible with underdetermined or overdetermined systems: cases where there are too few or too many constraints to uniquely determine a variable. Our framework embraces these situations as features rather than limitations through equality saturation, a concept borrowed from compiler optimization @zhangBetterTogetherUnifying2023 @tateEqualitySaturationNew2009.

When multiple equations describe the same variable from different perspectives, the framework autonomously connects these perspectives through shared variables. This allows us to:

- Represent uncertainty explicitly in underdetermined cases rather than making arbitrary assumptions
- Quantify agreement between constituent models in overdetermined cases
- Improve robustness against missing data through redundant pathways

This approach differs from traditional simulation methods by maintaining equivalent representations of relationships simultaneously, selecting the appropriate pathway for calculation dynamically based on available information and constraints.

=== Reinforcement Learning & Discovering Paths Through Decision Space

Even with a rigorous model of what plants are capable of deciding and a reliable physiological definition of benefits and costs, we face a familiar challenge: determining which specific decisions a plant will make among all valid possibilities. Rather than imposing _a priori_ strategies (akin to the problem of picking a $lambda$ value in early stomatal models), we employ reinforcement learning (RL) to discover effective strategies.

RL is particularly well-suited to this problem because it can navigate high-dimensional decision spaces with complex constraints, account for delayed consequences of actions across multiple timescales, and balance exploration of novel strategies against exploitation of known effective behaviors @shakyaReinforcementLearningAlgorithms2023.

By training RL agents in simulated plant environments, we have several means to derive strategy from data or from hueristics. RL can identify optimal strategies by rewarding fitness-relevant outcomes like maximizing cumulative carbon sequestration, or approximate realistic behavior by rewarding models for producing outcomes similar to real data from the field.

=== Modeling Internal Expectations

A distinctive feature of our framework is the explicit representation of agent expectations about future conditions. These expectations may diverge from reality in important ways that influence decision-making. To model this, we maintain parallel system representations:

+ The external system representing actual environmental and physiological states
+ An internal system representing the agent's expectations and beliefs

At each timestep, we update both systems: the external system according to physical laws and actual environmental conditions, and the internal system according to the agent's beliefs and prediction models. Decisions are made based on the internal model but enacted in the external system, creating a feedback loop that can reveal how misaligned expectations affect outcomes.

This dual-system approach allows us to explore how plants might respond to novel environmental conditions based on expectations derived from evolutionary history or individual experience, helping us understand potential maladaptation or latent resilience under rapid climate change.

== Questions

+ What do the trees know? Can trees learn, and update their expectations?

  Trees clearly make decisions based on expectations about future conditions, but the nature of these expectations remains unclear. When trees experience novel climate conditions, do they: (1) adapt to these as a "new normal," (2) anticipate a return to historically familiar conditions, or (3) anticipate continued directional change? This question is particularly interesting for slow-adapting traits like sapwood characteristics, which may reveal long-term strategic intentions. When a tree experiences climate misalignment, does it adjust to current conditions or "anticipate" future conditions based on observed trends?

  We can test this using our modeling framework by comparing three scenarios: trees with perfect climate foresight, trees with fixed historical climate expectations, and trees that update expectations based on recent experience without extrapolating future changes. By comparing these model outputs against empirical sapwood core data, we can determine which expectation model best explains observed tree behavior.

  \
+ To what degree do trees achieve strategic coherence across physiological systems?

  We define strategic coherence as the extent to which plants optimize decisions toward a unified goal (e.g., maximizing lifetime fecundity or carbon assimilation) across all decision points. While evolution should theoretically favor perfect within-organism coordination, evolutionary constraints may limit this optimization. Where do real trees fall on the spectrum from perfect strategic coherence to random or even maladaptive decisions?

  By comparing models trained on empirical data against both optimal strategy models and random decision models, we can quantify how effectively trees coordinate their physiological responses toward unified goals.

  \
+ Can we model trees as physiological agents alone or are they necessarily a part of an ecological community?

  Trees exist as members of ecological communities where competitive pressures may significantly alter optimal strategies. For example, saplings competing for canopy gaps may prioritize rapid height growth over other optimization criteria, as survival itself depends on outcompeting neighbors. This introduces a layer of complexity where trees must maintain expectations not just about environmental conditions but also about other decision-making agents in their vicinity.

  While our initial modeling will focus on individual trees responding to environmental variables, future work could explore how tree-tree interactions and community dynamics affect physiological decision-making. This ecological context might prove crucial for understanding mortality risks from biotic stressors (insect outbreaks, disease) or shifts in community composition under climate change. Such multi-agent physiological modeling could reveal important game-theoretical dynamics, such as how resource availability changes in thinning forest stands influence survival strategies @thomasMitigatingDroughtMortality2024.

  \
+ Why does this matter?

The effectiveness with which plants coordinate decisions across their physiological systems may significantly determine their survival and productivity under gradually changing climate conditions. By integrating empirically measured adaptive responses into a quantitative framework, we can organize field findings into a format that enables direct testing of strategic hypotheses and causal relationships.

This modeling approach could serve as a crucial bridge between organism-scale physiological studies and planet-scale climate models @wardRepresentingFunctionSensitivity2020. While simpler modeling approaches are often advocated for climate science, our approach emphasizes flexibility, allowing complex physiological dynamics to be represented authentically, then distilled into simpler formulations when appropriate. The insights gained could significantly improve our ability to predict forest responses to novel climate conditions and inform conservation and management strategies.
