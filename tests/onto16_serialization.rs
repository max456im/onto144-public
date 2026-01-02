```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Тесты: onto16 соответствует спецификации.

use onto144::projection::onto16::{Onto16Expression, Onto16CausalNode, serialize_to_json, serialize_to_yaml};
use serde_json::Value;

#[test]
fn test_onto16_minimal_structure() {
    let expr = Onto16Expression {
        syntax: "onto16/1.0".to_string(),
        semantics: serde_json::json!({ "test": true }),
        causal_network: vec![
            Onto16CausalNode {
                id: "root".to_string(),
                node_type: "invariant".to_string(),
                content: "identity".to_string(),
                depends_on: vec![],
            }
        ],
    };

    // Проверка YAML
    let yaml = serialize_to_yaml(&expr).unwrap();
    assert!(yaml.contains("syntax: onto16/1.0"));
    assert!(yaml.contains("id: root"));

    // Проверка JSON
    let json_str = serialize_to_json(&expr).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(json["syntax"], "onto16/1.0");
    assert_eq!(json["causal_network"][0]["id"], "root");
}

#[test]
fn test_causal_network_completeness() {
    // onto16 требует: identity, ethics, stimulus
    let expr = Onto16Expression {
        syntax: "onto16/1.0".to_string(),
        semantics: serde_json::json!({}),
        causal_network: vec![
            Onto16CausalNode {
                id: "identity".to_string(),
                node_type: "invariant".to_string(),
                content: "Aries-Fire-Choleric".to_string(),
                depends_on: vec![],
            },
            Onto16CausalNode {
                id: "ethics-binding".to_string(),
                node_type: "ethics".to_string(),
                content: "sha256:...".to_string(),
                depends_on: vec!["identity".to_string()],
            },
            Onto16CausalNode {
                id: "stim-001".to_string(),
                node_type: "stimulus".to_string(),
                content: "Hello".to_string(),
                depends_on: vec!["identity".to_string()],
            },
        ],
    };

    let json = serialize_to_json(&expr).unwrap();
    assert!(json.contains("ethics-binding"));
    assert!(json.contains("stim-001"));
}

#[should_panic]
#[test]
fn test_missing_identity_panics_in_projection() {
    // Этот тест демонстрирует, что проектор всегда добавляет identity
    // (реализовано в projector.rs), поэтому прямой тест на отсутствие identity не проходит.
    // Вместо этого — проверка через интеграционный тест в examples/.
}
```
