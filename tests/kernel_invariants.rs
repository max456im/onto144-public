```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Тесты: объективные инварианты не нарушены.

use onto144::core::kernel::KernelInvariants;
use onto144::state::{EnergyState, Phase};

#[test]
fn test_valid_kernel_invariants() {
    let energy = EnergyState::new(0.8);
    let phase = Phase::NoemaFast;
    let ethics = true;

    let kernel = KernelInvariants::new(energy, phase, ethics);
    assert!(kernel.can_project());
    kernel.enforce_invariants(); // не должно паниковать
}

#[test]
#[should_panic(expected = "Ethical invariant violated")]
fn test_ethics_violation_panic() {
    let energy = EnergyState::new(0.8);
    let phase = Phase::NoemaFast;
    let ethics = false;

    let kernel = KernelInvariants::new(energy, phase, ethics);
    kernel.enforce_invariants(); // должно вызвать панику
}

#[test]
#[should_panic(expected = "Energy invariant violated")]
fn test_negative_energy_panic() {
    let mut energy = EnergyState::new(0.0);
    energy.current = -0.1; // нарушение
    let phase = Phase::NoemaFast;
    let ethics = true;

    let kernel = KernelInvariants::new(energy, phase, ethics);
    kernel.enforce_invariants(); // должно вызвать панику
}

#[test]
fn test_low_energy_cannot_project() {
    let energy = EnergyState { current: 0.1, threshold: 0.3, credit: 0.0 };
    let phase = Phase::NoemaFast;
    let ethics = true;

    let kernel = KernelInvariants::new(energy, phase, ethics);
    assert!(!kernel.can_project());
}
```