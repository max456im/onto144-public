```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Управление фазой: NoemaFast (реактивный) ↔ NoemaSlow (рефлексивный).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    NoemaFast,
    NoemaSlow,
}

impl Phase {
    /// Принудительно переключает в NoemaSlow — требуется для инверсии или валидации.
    pub fn force_reflective(&mut self) {
        *self = Phase::NoemaSlow;
    }

    /// Возвращает, является ли фаза рефлексивной.
    pub fn is_reflective(&self) -> bool {
        matches!(self, Phase::NoemaSlow)
    }

    /// Возвращает, является ли фаза реактивной.
    pub fn is_reactive(&self) -> bool {
        matches!(self, Phase::NoemaFast)
    }
}

/// Контекст фазового перехода.
#[derive(Debug)]
pub struct PhaseTransition {
    pub from: Phase,
    pub to: Phase,
    pub reason: PhaseTransitionReason,
}

#[derive(Debug)]
pub enum PhaseTransitionReason {
    TensionThresholdExceeded,
    InversionRequired,
    ManualOverride,
    EnergyConservation,
}

/// Менеджер фазы с логированием переходов.
pub struct PhaseManager {
    current: Phase,
    history: Vec<PhaseTransition>,
}

impl PhaseManager {
    pub fn new(initial: Phase) -> Self {
        Self {
            current: initial,
            history: Vec::new(),
        }
    }

    pub fn current(&self) -> Phase {
        self.current
    }

    pub fn switch_to(&mut self, target: Phase, reason: PhaseTransitionReason) {
        if self.current != target {
            let transition = PhaseTransition {
                from: self.current,
                to: target,
                reason,
            };
            self.history.push(transition);
            self.current = target;
        }
    }

    pub fn force_reflective(&mut self, reason: PhaseTransitionReason) {
        self.switch_to(Phase::NoemaSlow, reason);
    }

    pub fn history(&self) -> &[PhaseTransition] {
        &self.history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_forced_reflection() {
        let mut manager = PhaseManager::new(Phase::NoemaFast);
        manager.force_reflective(PhaseTransitionReason::InversionRequired);
        assert!(manager.current().is_reflective());
        assert_eq!(manager.history().len(), 1);
        assert_eq!(manager.history()[0].reason, PhaseTransitionReason::InversionRequired);
    }
}
```