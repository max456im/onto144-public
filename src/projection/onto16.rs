```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Сериализация в формат onto16.

use serde::{Serialize, Deserialize};

/// Полная onto16-проекция с причинной сетью.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Onto16Expression {
    pub syntax: String,
    pub semantics: serde_json::Value,
    #[serde(rename = "causal_network")]
    pub causal_network: Vec<Onto16CausalNode>,
}

/// Узел причинной сети.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Onto16CausalNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub content: String,
    #[serde(rename = "depends_on")]
    pub depends_on: Vec<String>,
}

/// Сериализует проекцию в YAML (для совместимости с SGCL-экосистемой).
pub fn serialize_to_yaml(expr: &Onto16Expression) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(expr)
}

/// Сериализует проекцию в JSON (для API и транспондеров).
pub fn serialize_to_json(expr: &Onto16Expression) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onto16_serialization() {
        let node = Onto16CausalNode {
            id: "test".to_string(),
            node_type: "stimulus".to_string(),
            content: "Hello".to_string(),
            depends_on: vec!["identity".to_string()],
        };
        let expr = Onto16Expression {
            syntax: "onto16/1.0".to_string(),
            semantics: serde_json::json!({ "test": true }),
            causal_network: vec![node],
        };

        let yaml = serialize_to_yaml(&expr).unwrap();
        assert!(yaml.contains("id: test"));
        assert!(yaml.contains("type: stimulus"));

        let json = serialize_to_json(&expr).unwrap();
        assert!(json.contains("\"id\": \"test\""));
    }
}
```