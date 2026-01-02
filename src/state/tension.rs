```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Онтологическое напряжение → триггер инверсии темперамента.

use crate::state::{EnergyState, Phase};
use crate::temperament::inversion_matrix::{Temperament, invert_temperament};

/// Порог напряжения, при котором срабатывает инверсия.
const TENSION_THRESHOLD: f32 = 0.85;

#[derive(Debug)]
pub struct TensionState {
    pub current: f32,
    pub max: f32,
}

impl TensionState {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            max: TENSION_THRESHOLD,
        }
    }

    /// Обновляет напряжение на основе внешних и внутренних факторов.
    pub fn update(
        &mut self,
        energy: &EnergyState,
        phase: Phase,
        social_isolation: bool,
        attention_depth: u8,
    ) {
        let mut tension = 0.0;

        // Энергетический стресс
        if energy.current < 0.0 {
            tension += 0.5;
        } else if energy.current < energy.threshold {
            tension += 0.3;
        }

        // Фазовый дисбаланс
        if phase.is_reactive() && attention_depth > 3 {
            // Пытается делать глубокую рефлексию в Fast — напряжение
            tension += 0.25;
        }

        // Социальная изоляция (нарушение Law 2)
        if social_isolation {
            tension += 0.4;
        }

        self.current = tension.min(1.0);
    }

    /// Проверяет, превышено ли пороговое значение.
    pub fn exceeds_threshold(&self) -> bool {
        self.current >= self.max
    }

    /// Сбрасывает напряжение после инверсии.
    pub fn reset(&mut self) {
        self.current = 0.0;
    }
}

/// Оценивает, требуется ли инверсия темперамента.
pub fn should_invert(
    tension: &TensionState,
    current_temperament: Temperament,
) -> Option<(Temperament, crate::state::phase::PhaseTransitionReason)> {
    if tension.exceeds_threshold() {
        let new_temp = invert_temperament(current_temperament);
        Some((new_temp, crate::state::phase::PhaseTransitionReason::TensionThresholdExceeded))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tension_triggers_inversion() {
        let mut tension = TensionState::new();
        let energy = EnergyState { current: -0.1, threshold: 0.3, credit: 0.0 };
        tension.update(&energy, Phase::NoemaFast, true, 4);
        assert!(tension.exceeds_threshold());

        let result = should_invert(&tension, Temperament::Choleric);
        assert_eq!(result.unwrap().0, Temperament::Phlegmatic);
    }
}
```