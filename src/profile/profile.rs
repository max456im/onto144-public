```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Структура профиля: (sign, element, temperament)
//! Профиль — неизменяемая сущность после валидации.

use std::path::Path;
use crate::core::sgcl_validator::{validate_sgcl, SGCLProfile};
use crate::core::syntax_semantics::syntax_to_semantics;
use crate::projection::onto16::Onto16Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub sign: String,
    pub element: String,
    pub temperament: String,
    pub sgcl_hash: String,
    pub subjective_invariants: SubjectiveInvariants,
    pub onto16_template: Onto16Expression,
}

impl Profile {
    /// Загружает и валидирует профиль из SGCL-файла.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let sgcl = validate_sgcl(&content)?;
        let hash = crate::profile::generator::compute_sgcl_hash(&content);
        let subjective = SubjectiveInvariants::load(&sgcl.element, &sgcl.temperament)?;
        let onto16_template = syntax_to_semantics(&sgcl);

        Ok(Self {
            sign: sgcl.sign,
            element: sgcl.element,
            temperament: sgcl.temperament,
            sgcl_hash: hash,
            subjective_invariants: subjective,
            onto16_template,
        })
    }

    /// Возвращает уникальный идентификатор профиля.
    pub fn id(&self) -> String {
        format!("{}-{}-{}", self.sign, self.element, self.temperament)
    }
}

/// Субъективные инварианты, загружаемые из YAML-конфигураций.
/// Они определяют поведенческие предпочтения, но не нарушают ядерных инвариантов.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SubjectiveInvariants {
    pub attention_bias: f32,
    pub energy_decay_rate: f32,
    pub social_affinity: Vec<String>, // список совместимых элементов
    pub inversion_preference: String,  // предпочтительный темперамент при инверсии
}
```