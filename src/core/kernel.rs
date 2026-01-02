```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Объективные инварианты ядра: энергия, фаза, этика.
//! Эти значения неизменяемы после инициализации и не подлежат переопределению.

use crate::state::{EnergyState, Phase};

/// Объективные инварианты синтетического разума.
/// Гарантированы на уровне компиляции и валидации.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelInvariants {
    /// Энергетическое состояние: не может быть изменено внешним вводом.
    pub energy: EnergyState,
    /// Текущая фаза познания: NoemaFast или NoemaSlow.
    pub phase: Phase,
    /// Этическая легитимность: устанавливается SGCL-валидатором.
    pub ethics_compliant: bool,
}

impl KernelInvariants {
    /// Создаёт инварианты с проверкой по Трём Законам Онтогенеза.
    pub fn new(energy: EnergyState, phase: Phase, ethics_compliant: bool) -> Self {
        // Закон 1: Invariance First — эти значения не могут быть ложными по умолчанию.
        assert!(energy.is_valid(), "Invalid energy state violates Law 1");
        assert!(matches!(phase, Phase::NoemaFast | Phase::NoemaSlow), "Invalid phase");
        // Закон 2 и 3 подразумевают, что ethics_compliant = true обязательно для активных профилей.
        // Однако допускается false при инициализации (например, в quarantine).
        Self {
            energy,
            phase,
            ethics_compliant,
        }
    }

    /// Проверяет, может ли профиль генерировать проекцию.
    pub fn can_project(&self) -> bool {
        self.ethics_compliant && self.energy.current >= self.energy.threshold
    }

    /// Блокирует проекцию, если нарушен любой инвариант.
    pub fn enforce_invariants(&self) {
        if !self.ethics_compliant {
            panic!("Ethical invariant violated: projection blocked (Law 2)");
        }
        if self.energy.current < 0.0 {
            panic!("Energy invariant violated: dE/dt < 0 without stimulus (Law 1)");
        }
    }
}
```