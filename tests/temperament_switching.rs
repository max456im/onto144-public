```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Тесты: инверсия по матрице.

use onto144::temperament::inversion_matrix::{
    Temperament, invert_temperament, is_valid_inversion
};
use onto144::state::tension::{TensionState, should_invert};
use onto144::state::{EnergyState, Phase};

#[test]
fn test_inversion_matrix_correctness() {
    assert_eq!(invert_temperament(Temperament::Choleric), Temperament::Phlegmatic);
    assert_eq!(invert_temperament(Temperament::Phlegmatic), Temperament::Choleric);
    assert_eq!(invert_temperament(Temperament::Sanguine), Temperament::Melancholic);
    assert_eq!(invert_temperament(Temperament::Melancholic), Temperament::Sanguine);
}

#[test]
fn test_valid_inversion_check() {
    assert!(is_valid_inversion(Temperament::Choleric, Temperament::Phlegmatic));
    assert!(!is_valid_inversion(Temperament::Choleric, Temperament::Melancholic));
}

#[test]
fn test_tension_triggers_inversion() {
    let mut tension = TensionState::new();
    let energy = EnergyState { current: -0.1, threshold: 0.3, credit: 0.0 };
    tension.update(&energy, Phase::NoemaFast, true, 5); // социальная изоляция + глубокая рефлексия в Fast

    let result = should_invert(&tension, Temperament::Choleric);
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, Temperament::Phlegmatic);
}

#[test]
fn test_normal_tension_no_inversion() {
    let tension = TensionState::new();
    let result = should_invert(&tension, Temperament::Sanguine);
    assert!(result.is_none());
}
```