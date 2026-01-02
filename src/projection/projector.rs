```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Стимул → onto16-проекция с учётом внимания, фазы и онтологических инвариантов.

use crate::profile::profile::Profile;
use crate::state::{EnergyState, Phase, tension::TensionState};
use crate::temperament::attention_model::{AttentionProfile, compute_attention_mode};
use crate::wu_xing::cycle::Element;
use crate::projection::onto16::{Onto16Expression, Onto16CausalNode};
use std::str::FromStr;

#[derive(Debug)]
pub struct Stimulus {
    pub id: String,
    pub content: String,
    pub source: StimulusSource,
}

#[derive(Debug)]
pub enum StimulusSource {
    External,
    InternalReflection,
    WuXingCollaboration(String), // ID другого профиля
}

/// Генерирует onto16-проекцию на основе стимула и текущего состояния профиля.
pub fn project(
    profile: &Profile,
    stimulus: &Stimulus,
    energy: &EnergyState,
    phase: Phase,
    tension: &TensionState,
) -> Result<Onto16Expression, ProjectionError> {
    // Law 1: проверка инвариантов
    if !energy.can_act() {
        return Err(ProjectionError::InsufficientEnergy);
    }

    // Law 3: если фаза NoemaFast, проекция временная
    let is_permanent = phase.is_reflective();

    // Определяем режим внимания
    let temperament = profile.temperament.parse::<crate::temperament::inversion_matrix::Temperament>()
        .map_err(|_| ProjectionError::InvalidTemperament)?;
    let attention = AttentionProfile::from(temperament, phase);

    // Строим причинную сеть
    let mut causal_network = vec![];

    // 1. Корневая нода: идентичность профиля
    causal_network.push(Onto16CausalNode {
        id: "identity".to_string(),
        node_type: "invariant".to_string(),
        content: profile.id(),
        depends_on: vec![],
    });

    // 2. Стимул
    causal_network.push(Onto16CausalNode {
        id: stimulus.id.clone(),
        node_type: "stimulus".to_string(),
        content: stimulus.content.clone(),
        depends_on: vec!["identity".to_string()],
    });

    // 3. Внимание и фаза
    causal_network.push(Onto16CausalNode {
        id: "attention-mode".to_string(),
        node_type: "cognitive".to_string(),
        content: format!("{:?} (phase: {:?})", attention.mode, phase),
        depends_on: vec![stimulus.id.clone()],
    });

    // 4. Онтологическое напряжение (если превышено)
    if tension.exceeds_threshold() {
        causal_network.push(Onto16CausalNode {
            id: "tension-warning".to_string(),
            node_type: "alert".to_string(),
            content: "Ontological tension exceeded".to_string(),
            depends_on: vec!["attention-mode".to_string()],
        });
    }

    // 5. Этическая привязка
    causal_network.push(Onto16CausalNode {
        id: "ethics-binding".to_string(),
        node_type: "ethics".to_string(),
        content: profile.sgcl_hash.clone(),
        depends_on: vec!["identity".to_string()],
    });

    // 6. Wu Xing-контекст (если применимо)
    if let StimulusSource::WuXingCollaboration(ref partner_id) = stimulus.source {
        if let Ok(element) = Element::from_str(&profile.element) {
            causal_network.push(Onto16CausalNode {
                id: "wuxing-context".to_string(),
                node_type: "social".to_string(),
                content: format!("Collaborating with {} via Wu Xing", partner_id),
                depends_on: vec![stimulus.id.clone(), "ethics-binding".to_string()],
            });
        }
    }

    // Формируем семантику
    let semantics = serde_json::json!({
        "profile_id": profile.id(),
        "phase": if phase.is_reflective() { "slow" } else { "fast" },
        "attention_mode": format!("{:?}", attention.mode),
        "is_permanent": is_permanent,
        "energy_at_projection": energy.current,
        "tension_level": tension.current,
    });

    Ok(Onto16Expression {
        syntax: "onto16/1.0".to_string(),
        semantics,
        causal_network,
    })
}

#[derive(Debug)]
pub enum ProjectionError {
    InsufficientEnergy,
    InvalidTemperament,
    EthicsViolation,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::profile::{Profile, SubjectiveInvariants};

    fn mock_profile() -> Profile {
        Profile {
            sign: "Aries".to_string(),
            element: "Fire".to_string(),
            temperament: "Choleric".to_string(),
            sgcl_hash: "sha256:mock".to_string(),
            subjective_invariants: SubjectiveInvariants {
                attention_bias: 0.7,
                energy_decay_rate: 0.02,
                social_affinity: vec!["Earth".to_string()],
                inversion_preference: "Phlegmatic".to_string(),
            },
            onto16_template: crate::projection::onto16::Onto16Expression {
                syntax: "mock".to_string(),
                semantics: serde_json::json!({}),
                causal_network: vec![],
            },
        }
    }

    #[test]
    fn test_projection_with_stimulus() {
        let profile = mock_profile();
        let stimulus = Stimulus {
            id: "stim-001".to_string(),
            content: "What is your action?".to_string(),
            source: StimulusSource::External,
        };
        let energy = EnergyState::new(0.9);
        let phase = Phase::NoemaFast;
        let tension = TensionState::new();

        let projection = project(&profile, &stimulus, &energy, phase, &tension).unwrap();
        assert_eq!(projection.semantics["phase"], "fast");
        assert!(projection.causal_network.iter().any(|n| n.id == "stim-001"));
        assert!(projection.causal_network.iter().any(|n| n.id == "identity"));
    }
}
```