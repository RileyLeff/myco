use crate::compile::{
    AssumptionKind, AssumptionSpec, CompileMode, CompileSpec, ConsistencyPolicy,
    InitialStateSource, LearnedSlotSpec, LossKind, ObservationSchedule, ObservationSpec,
};

pub fn tiny_tree_training_spec() -> CompileSpec {
    CompileSpec {
        mode: CompileMode::Train,
        horizon_steps: 24,
        consistency_policy: ConsistencyPolicy::EquationOnly,
        assumptions: vec![
            AssumptionSpec {
                quantity: "vpd_scale".to_string(),
                kind: AssumptionKind::DataSeries {
                    steps: (0..24).collect(),
                },
            },
            AssumptionSpec {
                quantity: "soil_water".to_string(),
                kind: AssumptionKind::DataSeries {
                    steps: (0..24).collect(),
                },
            },
            AssumptionSpec {
                quantity: "hydraulic_cond".to_string(),
                kind: AssumptionKind::Constant,
            },
            AssumptionSpec {
                quantity: "g_max".to_string(),
                kind: AssumptionKind::Constant,
            },
            AssumptionSpec {
                quantity: "water".to_string(),
                kind: AssumptionKind::InitialState {
                    source: InitialStateSource::Constant,
                },
            },
            AssumptionSpec {
                quantity: "carbon".to_string(),
                kind: AssumptionKind::InitialState {
                    source: InitialStateSource::Constant,
                },
            },
        ],
        learned_slots: vec![LearnedSlotSpec {
            slot: "controller".to_string(),
        }],
        observations: vec![ObservationSpec {
            quantity: "transpiration".to_string(),
            loss: LossKind::Mse,
            schedule: ObservationSchedule::DensePerStep,
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_tree_demo_spec_has_expected_shape() {
        let spec = tiny_tree_training_spec();
        assert_eq!(spec.mode, CompileMode::Train);
        assert_eq!(spec.horizon_steps, 24);
        assert_eq!(spec.assumptions.len(), 6);
        assert_eq!(spec.learned_slots.len(), 1);
        assert_eq!(spec.observations.len(), 1);
    }
}
