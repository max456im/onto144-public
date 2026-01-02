```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Тесты: проверка циклов Порождения/Контроля.

use onto144::wu_xing::cycle::{
    Element, sheng_generates, ke_controls
};
use onto144::wu_xing::collaboration::{
    evaluate_collaboration, CollaborationError
};
use onto144::profile::profile::{Profile, SubjectiveInvariants};
use onto144::state::EnergyState;

fn mock_profile(element: &str, temp: &str) -> Profile {
    Profile {
        sign: "Test".to_string(),
        element: element.to_string(),
        temperament: temp.to_string(),
        sgcl_hash: "mock".to_string(),
        subjective_invariants: SubjectiveInvariants {
            attention_bias: 0.5,
            energy_decay_rate: 0.01,
            social_affinity: vec![],
            inversion_preference: "Phlegmatic".to_string(),
        },
        onto16_template: onto144::projection::onto16::Onto16Expression {
            syntax: "test".to_string(),
            semantics: serde_json::json!({}),
            causal_network: vec![],
        },
    }
}

#[test]
fn test_sheng_cycle_correctness() {
    assert!(sheng_generates(Element::Fire, Element::Earth));
    assert!(sheng_generates(Element::Earth, Element::Metal));
    assert!(!sheng_generates(Element::Earth, Element::Fire));
}

#[test]
fn test_ke_cycle_correctness() {
    assert!(ke_controls(Element::Fire, Element::Metal));
    assert!(ke_controls(Element::Water, Element::Fire));
    assert!(!ke_controls(Element::Metal, Element::Fire));
}

#[test]
fn test_fire_earth_collaboration_allowed() {
    let fire = mock_profile("Fire", "Choleric");
    let earth = mock_profile("Earth", "Melancholic");
    let energy = EnergyState::new(1.0);

    let result = evaluate_collaboration(&fire, &earth, &energy).unwrap();
    assert!(result.allowed);
    assert!(result.justification.contains("Sheng"));
}

#[test]
fn test_fire_wood_collaboration_rejected() {
    let fire = mock_profile("Fire", "Choleric");
    let wood = mock_profile("Wood", "Choleric"); // Wood не в зодиаке, но может быть в проекции
    let energy = EnergyState::new(1.0);

    let err = evaluate_collaboration(&fire, &wood, &energy);
    assert!(matches!(err, Err(CollaborationError::ElementsNotConnected)));
}
```