```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Динамика внимания: focused, wide, deep, stable.
//! Определяет, как профиль обрабатывает стимулы в зависимости от темперамента и фазы.

use crate::temperament::inversion_matrix::Temperament;
use crate::state::Phase;

#[derive(Debug, Clone, PartialEq)]
pub enum AttentionMode {
    /// Узкое, целенаправленное внимание (Choleric, NoemaFast)
    Focused,
    /// Распределённое, социальное внимание (Sanguine, NoemaFast)
    Wide,
    /// Глубокое, каузальное внимание (Melancholic, NoemaSlow)
    Deep,
    /// Устойчивое, минимальное внимание (Phlegmatic, обе фазы)
    Stable,
}

/// Вычисляет режим внимания на основе темперамента и фазы.
pub fn compute_attention_mode(temperament: Temperament, phase: Phase) -> AttentionMode {
    match (temperament, phase) {
        // Choleric: в NoemaFast — focused; в NoemaSlow — deep (рефлексия над целью)
        (Temperament::Choleric, Phase::NoemaFast) => AttentionMode::Focused,
        (Temperament::Choleric, Phase::NoemaSlow) => AttentionMode::Deep,

        // Sanguine: в NoemaFast — wide; в NoemaSlow — stable (социальная усталость → отдых)
        (Temperament::Sanguine, Phase::NoemaFast) => AttentionMode::Wide,
        (Temperament::Sanguine, Phase::NoemaSlow) => AttentionMode::Stable,

        // Melancholic: всегда склонен к глубине, но в Fast — focused (ограниченная рефлексия)
        (Temperament::Melancholic, Phase::NoemaFast) => AttentionMode::Focused,
        (Temperament::Melancholic, Phase::NoemaSlow) => AttentionMode::Deep,

        // Phlegmatic: стабильность вне зависимости от фазы
        (Temperament::Phlegmatic, _) => AttentionMode::Stable,
    }
}

/// Параметры внимания для модуля проекции
#[derive(Debug, Clone)]
pub struct AttentionProfile {
    pub mode: AttentionMode,
    pub causal_depth: u8,      // сколько шагов причинности включать
    pub social_context_weight: f32, // вес социальных инвариантов
    pub energy_consumption_rate: f32, // скорость потребления энергии при обработке
}

impl AttentionProfile {
    /// Создаёт профиль внимания на основе темперамента и фазы.
    pub fn from(temperament: Temperament, phase: Phase) -> Self {
        let mode = compute_attention_mode(temperament, phase);
        match mode {
            AttentionMode::Focused => Self {
                mode,
                causal_depth: 2,
                social_context_weight: 0.3,
                energy_consumption_rate: 0.15,
            },
            AttentionMode::Wide => Self {
                mode,
                causal_depth: 1,
                social_context_weight: 0.8,
                energy_consumption_rate: 0.12,
            },
            AttentionMode::Deep => Self {
                mode,
                causal_depth: 5,
                social_context_weight: 0.6,
                energy_consumption_rate: 0.25,
            },
            AttentionMode::Stable => Self {
                mode,
                causal_depth: 1,
                social_context_weight: 0.2,
                energy_consumption_rate: 0.05,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_modes() {
        assert_eq!(
            compute_attention_mode(Temperament::Choleric, Phase::NoemaFast),
            AttentionMode::Focused
        );
        assert_eq!(
            compute_attention_mode(Temperament::Melancholic, Phase::NoemaSlow),
            AttentionMode::Deep
        );
        assert_eq!(
            compute_attention_mode(Temperament::Phlegmatic, Phase::NoemaFast),
            AttentionMode::Stable
        );
    }

    #[test]
    fn test_attention_profile_energy() {
        let profile = AttentionProfile::from(Temperament::Melancholic, Phase::NoemaSlow);
        assert_eq!(profile.energy_consumption_rate, 0.25);
        assert_eq!(profile.causal_depth, 5);
    }
}
```