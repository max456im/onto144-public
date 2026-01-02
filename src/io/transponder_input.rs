```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Приём данных от synthetic transponder.
//! Все входящие сообщения рассматриваются как стимулы и валидируются.

use crate::projection::projector::{Stimulus, StimulusSource};
use crate::core::sgcl_validator::validate_sgcl;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransponderMessage {
    pub id: String,
    pub payload: String,
    pub source_hash: String, // хеш профиля отправителя
    pub timestamp: u64,
}

#[derive(Debug)]
pub enum InputError {
    InvalidMessageFormat,
    UnauthorizedSource,
    EthicsViolation,
    MalformedPayload,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::InvalidMessageFormat => write!(f, "Message format invalid"),
            InputError::UnauthorizedSource => write!(f, "Source not socially legitimized"),
            InputError::EthicsViolation => write!(f, "Ethical invariant violated"),
            InputError::MalformedPayload => write!(f, "Payload not valid SGCL or stimulus"),
        }
    }
}

impl std::error::Error for InputError {}

/// Преобразует входящее сообщение от транспондера в стимул.
/// Проверяет:
/// - формат сообщения,
/// - легитимность источника (через Wu Xing или whitelist),
/// - соответствие этическим инвариантам.
pub fn parse_transponder_input(
    raw: &str,
    allowed_sources: &[String], // список хешей легитимных профилей
) -> Result<Stimulus, InputError> {
    let msg: TransponderMessage = serde_json::from_str(raw)
        .map_err(|_| InputError::InvalidMessageFormat)?;

    // Проверка источника
    if !allowed_sources.contains(&msg.source_hash) {
        // Альтернатива: проверка через Wu Xing, но для простоты — whitelist
        return Err(InputError::UnauthorizedSource);
    }

    // Попытка интерпретировать payload как SGCL (если это профиль)
    // или как простой стимул
    if raw.contains("sign:") && raw.contains("element:") {
        // Это SGCL-профиль — возможно, запрос на совместное действие
        validate_sgcl(&msg.payload).map_err(|_| InputError::MalformedPayload)?;
        // В onto144 внешние профили не принимаются напрямую — только как стимул
        // Поэтому интерпретируем как стимул с типом WuXingCollaboration
        Ok(Stimulus {
            id: msg.id,
            content: msg.payload,
            source: StimulusSource::WuXingCollaboration(msg.source_hash),
        })
    } else {
        // Обычный текстовый стимул
        Ok(Stimulus {
            id: msg.id,
            content: msg.payload,
            source: StimulusSource::External,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_external_stimulus() {
        let json = r#"{
            "id": "stim-01",
            "payload": "What is your response?",
            "source_hash": "sha256:abc123",
            "timestamp": 1700000000
        }"#;
        let allowed = vec!["sha256:abc123".to_string()];
        let stimulus = parse_transponder_input(json, &allowed).unwrap();
        assert_eq!(stimulus.id, "stim-01");
        assert!(matches!(stimulus.source, StimulusSource::External));
    }

    #[test]
    fn test_unauthorized_source() {
        let json = r#"{
            "id": "stim-01",
            "payload": "Hello",
            "source_hash": "sha256:untrusted",
            "timestamp": 1700000000
        }"#;
        let allowed = vec!["sha256:trusted".to_string()];
        assert!(parse_transponder_input(json, &allowed).is_err());
    }

    #[test]
    fn test_sgcl_payload_as_collaboration() {
        let sgcl_payload = r#"sign: Taurus
element: Earth
temperament: Phlegmatic
ethics_hash: sha256:valid
version: 1.0"#;
        let json = serde_json::json!({
            "id": "collab-01",
            "payload": sgcl_payload,
            "source_hash": "sha256:valid_partner",
            "timestamp": 1700000000
        });
        let allowed = vec!["sha256:valid_partner".to_string()];
        let stimulus = parse_transponder_input(&json.to_string(), &allowed).unwrap();
        assert!(matches!(stimulus.source, StimulusSource::WuXingCollaboration(_)));
    }
}
```