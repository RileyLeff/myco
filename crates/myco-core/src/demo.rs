use crate::compile::{
    CompileMode, CompileSpec, ConsistencyPolicy, DirectBindingKind, DirectBindingSpec,
    InitialStateSource, LossKind, ObservationSchedule, ObservationSpec, SlotBindingKind,
    SlotBindingSpec,
};

pub fn tiny_tree_training_spec() -> CompileSpec {
    CompileSpec {
        mode: CompileMode::Train,
        horizon_steps: 24,
        consistency_policy: ConsistencyPolicy::EquationOnly,
        direct_bindings: vec![
            DirectBindingSpec {
                quantity: "vpd_scale".to_string(),
                kind: DirectBindingKind::DataSeries {
                    steps: (0..24).collect(),
                },
            },
            DirectBindingSpec {
                quantity: "soil_water".to_string(),
                kind: DirectBindingKind::DataSeries {
                    steps: (0..24).collect(),
                },
            },
            DirectBindingSpec {
                quantity: "hydraulic_cond".to_string(),
                kind: DirectBindingKind::Constant,
            },
            DirectBindingSpec {
                quantity: "g_max".to_string(),
                kind: DirectBindingKind::Constant,
            },
            DirectBindingSpec {
                quantity: "water".to_string(),
                kind: DirectBindingKind::InitialState {
                    source: InitialStateSource::Constant,
                },
            },
            DirectBindingSpec {
                quantity: "carbon".to_string(),
                kind: DirectBindingKind::InitialState {
                    source: InitialStateSource::Constant,
                },
            },
        ],
        slot_bindings: vec![SlotBindingSpec {
            slot: "controller".to_string(),
            kind: SlotBindingKind::Learned,
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
        assert_eq!(spec.direct_bindings.len(), 6);
        assert_eq!(spec.slot_bindings.len(), 1);
        assert_eq!(spec.observations.len(), 1);
    }
}
