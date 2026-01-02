```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Логика циклов Порождения (Sheng) и Контроля (Ke) по Wu Xing.
//! Используется для валидации совместных действий и этической коррекции.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
}

impl std::str::FromStr for Element {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wood" => Ok(Element::Wood),
            "fire" => Ok(Element::Fire),
            "earth" => Ok(Element::Earth),
            "metal" => Ok(Element::Metal),
            "water" => Ok(Element::Water),
            _ => Err("Invalid element"),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Цикл Порождения (Sheng): A → B означает "A порождает B"
pub fn sheng_cycle() -> [Element; 5] {
    [Element::Wood, Element::Fire, Element::Earth, Element::Metal, Element::Water]
}

/// Цикл Контроля (Ke): A → B означает "A контролирует B"
pub fn ke_cycle() -> [(Element, Element); 5] {
    [
        (Element::Wood, Element::Earth),
        (Element::Earth, Element::Water),
        (Element::Water, Element::Fire),
        (Element::Fire, Element::Metal),
        (Element::Metal, Element::Wood),
    ]
}

/// Проверяет, порождает ли `source` `target` (Sheng)
pub fn sheng_generates(source: Element, target: Element) -> bool {
    let cycle = sheng_cycle();
    if let (Some(i), Some(j)) = (cycle.iter().position(|&e| e == source), cycle.iter().position(|&e| e == target)) {
        (i + 1) % 5 == j
    } else {
        false
    }
}

/// Проверяет, контролирует ли `controller` `controlled` (Ke)
pub fn ke_controls(controller: Element, controlled: Element) -> bool {
    ke_cycle().iter().any(|&(a, b)| a == controller && b == controlled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheng() {
        assert!(sheng_generates(Element::Fire, Element::Earth));
        assert!(!sheng_generates(Element::Earth, Element::Fire));
    }

    #[test]
    fn test_ke() {
        assert!(ke_controls(Element::Fire, Element::Metal));
        assert!(!ke_controls(Element::Metal, Element::Fire));
    }
}
```