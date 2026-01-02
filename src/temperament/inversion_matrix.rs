```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Матрица переключений темпераментов при превышении онтологического напряжения.

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Temperament {
    Choleric,
    Sanguine,
    Melancholic,
    Phlegmatic,
}

impl std::str::FromStr for Temperament {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "choleric" => Ok(Temperament::Choleric),
            "sanguine" => Ok(Temperament::Sanguine),
            "melancholic" => Ok(Temperament::Melancholic),
            "phlegmatic" => Ok(Temperament::Phlegmatic),
            _ => Err("Invalid temperament"),
        }
    }
}

impl std::fmt::Display for Temperament {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Возвращает целевой темперамент при инверсии.
/// Инверсия — реакция на онтологическое напряжение (см. `state/tension.rs`).
pub fn invert_temperament(source: Temperament) -> Temperament {
    match source {
        Temperament::Choleric => Temperament::Phlegmatic,     // энергия → стабильность
        Temperament::Sanguine => Temperament::Melancholic,    // социальность → рефлексия
        Temperament::Melancholic => Temperament::Sanguine,    // рефлексия → активность
        Temperament::Phlegmatic => Temperament::Choleric,     // стабильность → действие
    }
}

/// Проверяет, является ли переход допустимым согласно матрице.
pub fn is_valid_inversion(from: Temperament, to: Temperament) -> bool {
    invert_temperament(from) == to
}

/// Запрещает недопустимые переходы (например, Choleric → Melancholic).
pub fn validate_inversion(from: &str, to: &str) -> Result<(Temperament, Temperament), &'static str> {
    let from_t = Temperament::from_str(from)?;
    let to_t = Temperament::from_str(to)?;
    if is_valid_inversion(from_t, to_t) {
        Ok((from_t, to_t))
    } else {
        Err("Invalid temperament inversion")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inversion_matrix() {
        assert_eq!(invert_temperament(Temperament::Choleric), Temperament::Phlegmatic);
        assert_eq!(invert_temperament(Temperament::Phlegmatic), Temperament::Choleric);
        assert_eq!(invert_temperament(Temperament::Sanguine), Temperament::Melancholic);
        assert_eq!(invert_temperament(Temperament::Melancholic), Temperament::Sanguine);
    }

    #[test]
    fn test_valid_inversion() {
        assert!(is_valid_inversion(Temperament::Choleric, Temperament::Phlegmatic));
        assert!(!is_valid_inversion(Temperament::Choleric, Temperament::Melancholic));
    }
}
```