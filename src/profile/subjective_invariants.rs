```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Загрузка субъективных инвариантов из конфигураций стихий и темпераментов.
//! Эти данные влияют на поведение, но не нарушают ядерных инвариантов.

use std::collections::HashMap;
use std::fs;

impl crate::profile::profile::SubjectiveInvariants {
    /// Загружает инварианты из config/elements/{element}.yaml и config/temperaments/{temperament}.yaml
    pub fn load(element: &str, temperament: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let elem_path = format!("config/elements/{}.yaml", element.to_lowercase());
        let temp_path = format!("config/temperaments/{}.yaml", temperament.to_lowercase());

        let elem_data: serde_yaml::Value = serde_yaml::from_str(&fs::read_to_string(&elem_path)?)?;
        let temp_data: serde_yaml::Value = serde_yaml::from_str(&fs::read_to_string(&temp_path)?)?;

        // Извлечение значений с безопасной обработкой отсутствующих ключей
        let energy_decay_rate = elem_data.get("energy_decay_rate")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.01) as f32;

        let attention_bias = temp_data.get("attention_bias")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5) as f32;

        let social_affinity = elem_data.get("social_affinity")
            .and_then(|v| v.as_sequence())
            .map(|seq| seq.iter().filter_map(|s| s.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        let inversion_preference = temp_data.get("inversion_preference")
            .and_then(|v| v.as_str())
            .unwrap_or("Phlegmatic")
            .to_string();

        Ok(Self {
            attention_bias,
            energy_decay_rate,
            social_affinity,
            inversion_preference,
        })
    }
}
```