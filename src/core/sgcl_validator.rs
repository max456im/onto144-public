```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Валидатор синтаксиса SGCL (Synthetic Governance Constraint Language).
//! SGCL — строгий DSL для описания профилей с фиксированными полями.

use std::collections::HashMap;

#[derive(Debug)]
pub struct SGCLParseError(pub String);

/// Минимальная структура SGCL-профиля.
#[derive(Debug, Clone, PartialEq)]
pub struct SGCLProfile {
    pub sign: String,
    pub element: String,
    pub temperament: String,
    pub ethics_hash: String,
    pub version: String,
}

/// Проверяет SGCL-текст на соответствие грамматике.
pub fn validate_sgcl(sgcl_content: &str) -> Result<SGCLProfile, SGCLParseError> {
    let mut lines = sgcl_content.lines().filter(|l| !l.trim().starts_with('#'));
    let mut fields: HashMap<String, String> = HashMap::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() { continue; }
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().to_string();
            if ["sign", "element", "temperament", "ethics_hash", "version"].contains(&key.as_str()) {
                fields.insert(key, value);
            } else {
                return Err(SGCLParseError(format!("Unknown field: {}", key)));
            }
        } else {
            return Err(SGCLParseError("Invalid line format".to_string()));
        }
    }

    // Обязательные поля
    let sign = fields.remove("sign").ok_or_else(|| SGCLParseError("Missing 'sign'".into()))?;
    let element = fields.remove("element").ok_or_else(|| SGCLParseError("Missing 'element'".into()))?;
    let temperament = fields.remove("temperament").ok_or_else(|| SGCLParseError("Missing 'temperament'".into()))?;
    let ethics_hash = fields.remove("ethics_hash").ok_or_else(|| SGCLParseError("Missing 'ethics_hash'".into()))?;
    let version = fields.remove("version").unwrap_or_else(|| "1.0".to_string());

    // Проверка значений
    if !valid_zodiac_signs().contains(&sign.as_str()) {
        return Err(SGCLParseError(format!("Invalid zodiac sign: {}", sign)));
    }
    if !valid_elements().contains(&element.as_str()) {
        return Err(SGCLParseError(format!("Invalid element: {}", element)));
    }
    if !valid_temperaments().contains(&temperament.as_str()) {
        return Err(SGCLParseError(format!("Invalid temperament: {}", temperament)));
    }

    Ok(SGCLProfile {
        sign,
        element,
        temperament,
        ethics_hash,
        version,
    })
}

fn valid_zodiac_signs() -> Vec<&'static str> {
    vec![
        "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
        "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
    ]
}

fn valid_elements() -> Vec<&'static str> {
    vec!["Wood", "Fire", "Earth", "Metal", "Water"]
}

fn valid_temperaments() -> Vec<&'static str> {
    vec!["Choleric", "Sanguine", "Melancholic", "Phlegmatic"]
}
```