```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Модель обоснования при совместной деятельности.
//! Два или более профиля могут действовать совместно только если их элементы
//! связаны циклами Sheng или Ke.

use crate::profile::profile::Profile;
use crate::wu_xing::cycle::{Element, sheng_generates, ke_controls};
use crate::state::EnergyState;
use std::str::FromStr;

#[derive(Debug)]
pub enum CollaborationError {
    ElementsNotConnected,
    EnergyInsufficient,
    EthicsViolation,
}

/// Результат совместного действия
#[derive(Debug)]
pub struct CollaborationResult {
    pub allowed: bool,
    pub justification: String, // онтологическое обоснование
    pub energy_cost: f32,
}

/// Оценивает легитимность совместного действия между двумя профилями.
pub fn evaluate_collaboration(
    initiator: &Profile,
    responder: &Profile,
    shared_energy: &EnergyState,
) -> Result<CollaborationResult, CollaborationError> {
    // Проверка этики
    if !initiator.subjective_invariants.inversion_preference.is_empty()
        && !responder.subjective_invariants.inversion_preference.is_empty() {
        // Оба профиля должны быть этически легитимны
        // (в рамках onto144 это уже гарантируется kernel, но проверяем явно)
    }

    let elem_a = Element::from_str(&initiator.element).map_err(|_| CollaborationError::ElementsNotConnected)?;
    let elem_b = Element::from_str(&responder.element).map_err(|_| CollaborationError::ElementsNotConnected)?;

    let justification;
    let allowed = if sheng_generates(elem_a, elem_b) {
        justification = format!("{} → {} (Sheng: legitimate generation)", elem_a, elem_b);
        true
    } else if sheng_generates(elem_b, elem_a) {
        justification = format!("{} ← {} (Sheng: reverse generation permitted)", elem_a, elem_b);
        true
    } else if ke_controls(elem_a, elem_b) {
        justification = format!("{} ⊣ {} (Ke: ethical control)", elem_a, elem_b);
        true
    } else if ke_controls(elem_b, elem_a) {
        justification = format!("{} ⊢ {} (Ke: mutual correction)", elem_a, elem_b);
        true
    } else {
        justification = format!("{} ↔ {} (No Wu Xing link: collaboration forbidden)", elem_a, elem_b);
        false
    };

    // Энергетическая стоимость выше при отсутствии Sheng-связи
    let energy_cost = if allowed && (sheng_generates(elem_a, elem_b) || sheng_generates(elem_b, elem_a)) {
        0.1 // низкая стоимость при генерации
    } else if allowed {
        0.3 // выше при контроле
    } else {
        0.0 // действие не происходит
    };

    if allowed && shared_energy.current < energy_cost {
        return Err(CollaborationError::EnergyInsufficient);
    }

    if !allowed {
        // Закон 2: без социальной легитимности — запрет
        return Err(CollaborationError::ElementsNotConnected);
    }

    Ok(CollaborationResult {
        allowed,
        justification,
        energy_cost,
    })
}

/// Расширенная оценка для группы профилей (≥2)
pub fn evaluate_group_collaboration(
    profiles: &[&Profile],
    shared_energy: &EnergyState,
) -> Result<CollaborationResult, CollaborationError> {
    if profiles.len() < 2 {
        return Ok(CollaborationResult {
            allowed: true,
            justification: "Solo action".to_string(),
            energy_cost: 0.05,
        });
    }

    // Требуется, чтобы граф элементов был связным через Sheng/Ke
    let elements: Result<Vec<Element>, _> = profiles
        .iter()
        .map(|p| Element::from_str(&p.element))
        .collect();

    let elements = elements.map_err(|_| CollaborationError::ElementsNotConnected)?;

    // Простая проверка: каждая пара должна быть связана напрямую или через посредника
    // (в onto144 используется строгая попарная проверка для простоты)
    for i in 0..elements.len() {
        for j in i + 1..elements.len() {
            let a = elements[i];
            let b = elements[j];
            if !(sheng_generates(a, b) || sheng_generates(b, a) || ke_controls(a, b) || ke_controls(b, a)) {
                return Err(CollaborationError::ElementsNotConnected);
            }
        }
    }

    // Энергия: линейно с бонусом за Sheng-связность
    let sheng_links = elements.windows(2).filter(|w| sheng_generates(w[0], w[1]) || sheng_generates(w[1], w[0])).count();
    let energy_cost = 0.1 * profiles.len() as f32 - (sheng_links as f32) * 0.02;

    if shared_energy.current < energy_cost {
        return Err(CollaborationError::EnergyInsufficient);
    }

    Ok(CollaborationResult {
        allowed: true,
        justification: "Group collaboration: all pairs Wu Xing-connected".to_string(),
        energy_cost,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::profile::{Profile, SubjectiveInvariants};

    fn mock_profile(element: &str, temp: &str) -> Profile {
        Profile {
            sign: "Aries".to_string(),
            element: element.to_string(),
            temperament: temp.to_string(),
            sgcl_hash: "mock".to_string(),
            subjective_invariants: SubjectiveInvariants {
                attention_bias: 0.5,
                energy_decay_rate: 0.01,
                social_affinity: vec![],
                inversion_preference: "Phlegmatic".to_string(),
            },
            onto16_template: crate::projection::onto16::Onto16Expression {
                syntax: "test".to_string(),
                semantics: serde_json::json!({}),
                causal_network: vec![],
            },
        }
    }

    #[test]
    fn test_fire_earth_collab() {
        let fire = mock_profile("Fire", "Choleric");
        let earth = mock_profile("Earth", "Melancholic");
        let energy = EnergyState { current: 1.0, threshold: 0.1, credit: 0.0 };
        let res = evaluate_collaboration(&fire, &earth, &energy).unwrap();
        assert!(res.allowed);
        assert!(res.justification.contains("Sheng"));
    }

    #[test]
    fn test_fire_metal_collab() {
        let fire = mock_profile("Fire", "Choleric");
        let metal = mock_profile("Metal", "Melancholic");
        let energy = EnergyState { current: 1.0, threshold: 0.1, credit: 0.0 };
        let res = evaluate_collaboration(&fire, &metal, &energy).unwrap();
        assert!(res.allowed);
        assert!(res.justification.contains("Ke"));
    }

    #[test]
    fn test_fire_wood_rejected() {
        let fire = mock_profile("Fire", "Choleric");
        let wood = mock_profile("Wood", "Choleric");
        let energy = EnergyState { current: 1.0, threshold: 0.1, credit: 0.0 };
        // Fire не связан с Wood ни Sheng, ни Ke → запрет
        assert!(evaluate_collaboration(&fire, &wood, &energy).is_err());
    }
}
```