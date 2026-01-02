```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Тесты: все 144 профиля сгенерированы корректно.

use onto144::profile::generator::generate_all_profiles;
use std::collections::HashSet;

#[test]
fn test_exactly_24_base_profiles() {
    let profiles = generate_all_profiles();
    assert_eq!(profiles.len(), 24, "Должно быть ровно 24 базовых профиля");
}

#[test]
fn test_unique_profile_ids() {
    let profiles = generate_all_profiles();
    let ids: HashSet<_> = profiles.iter().map(|p| p.id()).collect();
    assert_eq!(ids.len(), 24, "Все ID профилей должны быть уникальны");
}

#[test]
fn test_valid_zodiac_element_mapping() {
    let valid_combos = [
        ("Aries", "Fire"), ("Taurus", "Earth"), ("Gemini", "Metal"),
        ("Cancer", "Water"), ("Leo", "Fire"), ("Virgo", "Earth"),
        ("Libra", "Metal"), ("Scorpio", "Water"), ("Sagittarius", "Fire"),
        ("Capricorn", "Earth"), ("Aquarius", "Metal"), ("Pisces", "Water"),
    ];

    let profiles = generate_all_profiles();
    for profile in &profiles {
        let expected_element = valid_combos.iter()
            .find(|(sign, _)| sign == &profile.sign.as_str())
            .expect("Знак должен быть в списке")
            .1;
        assert_eq!(&profile.element, expected_element, "Несоответствие стихии для {}", profile.sign);
    }
}

#[test]
fn test_temperament_compatibility() {
    let fire_profiles: Vec<_> = generate_all_profiles()
        .into_iter()
        .filter(|p| p.element == "Fire")
        .collect();
    assert_eq!(fire_profiles.len(), 6); // 3 знака × 2 темперамента

    for p in &fire_profiles {
        assert!(["Choleric", "Sanguine"].contains(&p.temperament.as_str()));
    }
}
```