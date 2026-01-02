```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Отправка решений через emitter.
//! Все исходящие проекции сериализуются в onto16 и помечаются фазой.

use crate::projection::onto16::{Onto16Expression, serialize_to_json};
use crate::state::Phase;

#[derive(Debug)]
pub enum OutputError {
    SerializationFailed,
    NonReflectiveProjectionBlocked, // если попытка отправить NoemaFast как окончательное решение
}

/// Подготавливает проекцию к отправке.
/// Если фаза — NoemaFast, добавляется пометка "provisional".
/// Если фаза — NoemaSlow, разрешена отправка как окончательного решения.
pub fn emit_projection(
    projection: &Onto16Expression,
    phase: Phase,
    allow_fast: bool, // разрешать ли отправку fast-проекций (для отладки или внутреннего использования)
) -> Result<String, OutputError> {
    if !allow_fast && phase.is_reactive() {
        // Law 3: NoemaFast не может быть окончательным
        return Err(OutputError::NonReflectiveProjectionBlocked);
    }

    let mut output = serde_json::json!({
        "onto16_projection": projection,
        "metadata": {
            "phase": if phase.is_reflective() { "slow" } else { "fast" },
            "provisional": !phase.is_reflective(),
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    });

    // Добавляем временну́ю метку в JSON
    if let Ok(ts) = output["metadata"]["timestamp"].as_u64() {
        output["metadata"]["timestamp"] = serde_json::Value::Number(ts.into());
    }

    serde_json::to_string(&output)
        .map_err(|_| OutputError::SerializationFailed)
}

/// Упрощённая отправка: только JSON onto16 (для транспондеров, которые сами обрабатывают метаданные)
pub fn emit_raw_onto16(projection: &Onto16Expression) -> Result<String, OutputError> {
    serialize_to_json(projection).map_err(|_| OutputError::SerializationFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::onto16::{Onto16Expression, Onto16CausalNode};

    fn mock_projection() -> Onto16Expression {
        Onto16Expression {
            syntax: "onto16/1.0".to_string(),
            semantics: serde_json::json!({ "test": true }),
            causal_network: vec![Onto16CausalNode {
                id: "test".to_string(),
                node_type: "stimulus".to_string(),
                content: "test".to_string(),
                depends_on: vec![],
            }],
        }
    }

    #[test]
    fn test_emit_slow_projection() {
        let proj = mock_projection();
        let output = emit_projection(&proj, Phase::NoemaSlow, false).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(json["metadata"]["phase"], "slow");
        assert_eq!(json["metadata"]["provisional"], false);
    }

    #[test]
    fn test_emit_fast_blocked_by_default() {
        let proj = mock_projection();
        let err = emit_projection(&proj, Phase::NoemaFast, false);
        assert!(matches!(err, Err(OutputError::NonReflectiveProjectionBlocked)));
    }

    #[test]
    fn test_emit_fast_allowed_for_debug() {
        let proj = mock_projection();
        let output = emit_projection(&proj, Phase::NoemaFast, true).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(json["metadata"]["provisional"], true);
    }
}
```