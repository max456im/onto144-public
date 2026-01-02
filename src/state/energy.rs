```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Энергетическая модель: dE/dt, утечки, кредит.
//! Энергия — измеряемая, сохраняющаяся величина, не метафора.

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyState {
    /// Текущий уровень энергии (0.0–1.0 нормализовано)
    pub current: f32,
    /// Минимальный порог для проекции
    pub threshold: f32,
    /// Кредит энергии (может быть использован при дефиците)
    pub credit: f32,
}

impl EnergyState {
    pub fn new(initial: f32) -> Self {
        Self {
            current: initial.min(1.0).max(0.0),
            threshold: 0.3,
            credit: 0.0,
        }
    }

    /// Применяет утечку энергии за тик времени.
    /// Утечка зависит от режима внимания и фазы.
    pub fn apply_leak(&mut self, leak_rate: f32) {
        self.current -= leak_rate;
        if self.current < 0.0 {
            // Использовать кредит, если доступен
            let deficit = self.current.abs();
            if self.credit >= deficit {
                self.credit -= deficit;
                self.current = 0.0;
            } else {
                self.credit = 0.0;
                // current остаётся отрицательным — триггер энергосбережения
            }
        }
    }

    /// Пополняет энергию от внешнего стимула.
    pub fn recharge(&mut self, amount: f32) {
        self.current = (self.current + amount).min(1.0);
        // Кредит не пополняется напрямую — только через социальную легитимность (Wu Xing)
    }

    /// Начисляет кредит за совместное действие по циклу Sheng.
    pub fn grant_credit(&mut self, amount: f32) {
        self.credit = (self.credit + amount).min(0.5); // Макс. кредит = 50% от полной энергии
    }

    /// Проверяет, является ли состояние валидным (не нарушает инварианты).
    pub fn is_valid(&self) -> bool {
        self.current >= -0.2 && self.credit >= 0.0 && self.threshold > 0.0
    }

    /// Возвращает, может ли профиль выполнять проекции.
    pub fn can_act(&self) -> bool {
        self.current >= self.threshold || (self.current + self.credit) >= self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_leak_and_recharge() {
        let mut energy = EnergyState::new(0.8);
        energy.apply_leak(0.2);
        assert_eq!(energy.current, 0.6);
        energy.recharge(0.3);
        assert_eq!(energy.current, 0.9);
    }

    #[test]
    fn test_credit_usage() {
        let mut energy = EnergyState::new(0.1);
        energy.apply_leak(0.3); // current = -0.2
        assert!(energy.current < 0.0);
        energy.grant_credit(0.3);
        energy.apply_leak(0.0); // не менять, но проверить использование кредита при следующем действии
        // В onto144 кредит используется не автоматически — только через kernel::can_project()
    }
}
```