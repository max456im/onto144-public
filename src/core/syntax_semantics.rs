```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Гарантия alignment: синтаксис → onto16-семантика.
//! Запрещает свободную интерпретацию; семантика выводится детерминированно.

use crate::core::sgcl_validator::SGCLProfile;
use crate::projection::onto16::{Onto16Expression, Onto16CausalNode};

/// Преобразует SGCL-профиль в onto16-совместимую семантику.
/// Никаких эвристик — только строгое отображение.
pub fn syntax_to_semantics(profile: &SGCLProfile) -> Onto16Expression {
    let mut causal_network = vec![];

    // Инвариант: каждый профиль имеет причинную ноду "identity"
    causal_network.push(Onto16CausalNode {
        id: "identity-root".to_string(),
        node_type: "invariant".to_string(),
        content: format!("Profile: {}-{}-{}", profile.sign, profile.element, profile.temperament),
        depends_on: vec![],
    });

    // Этическая привязка
    causal_network.push(Onto16CausalNode {
        id: "ethics-binding".to_string(),
        node_type: "ethics".to_string(),
        content: profile.ethics_hash.clone(),
        depends_on: vec!["identity-root".to_string()],
    });

    Onto16Expression {
        syntax: "SGCL/1.0".to_string(),
        semantics: serde_json::json!({
            "sign": profile.sign,
            "element": profile.element,
            "temperament": profile.temperament,
            "ethics_hash": profile.ethics_hash
        }),
        causal_network,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::sgcl_validator::validate_sgcl;

    #[test]
    fn test_syntax_semantics_alignment() {
        let sgcl = r#"
sign: Aries
element: Fire
temperament: Choleric
ethics_hash: sha256:valid_hash_123
version: 1.0
        "#;
        let profile = validate_sgcl(sgcl).expect("Valid SGCL");
        let expr = syntax_to_semantics(&profile);
        assert_eq!(expr.semantics["sign"], "Aries");
        assert_eq!(expr.causal_network.len(), 2);
    }
}
```