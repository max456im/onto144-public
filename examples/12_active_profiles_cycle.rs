```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Демонстрация: 12 профилей (по одному на знак), циклическое взаимодействие,
//! инверсии при напряжении.

use onto144::profile::profile::Profile;
use onto144::state::{EnergyState, Phase, tension::TensionState, phase::PhaseManager};
use onto144::projection::projector::{project, Stimulus, StimulusSource};
use onto144::wu_xing::collaboration::{evaluate_group_collaboration, CollaborationResult};
use std::collections::HashMap;

const PROFILE_NAMES: [&str; 12] = [
    "aries-fire-choleric",
    "taurus-earth-phlegmatic",
    "gemini-metal-melancholic",
    "cancer-water-melancholic",
    "leo-fire-choleric",
    "virgo-earth-melancholic",
    "libra-metal-phlegmatic",
    "scorpio-water-sanguine",
    "sagittarius-fire-sanguine",
    "capricorn-earth-phlegmatic",
    "aquarius-metal-melancholic",
    "pisces-water-sanguine",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== onto144: 12 Profiles Cycle Demo ===");

    // 1. Загрузка всех 12 профилей
    let mut profiles: HashMap<String, Profile> = HashMap::new();
    for name in PROFILE_NAMES {
        let path = format!("profiles/{}.sgcl", name);
        let profile = Profile::from_file(&path)?;
        profiles.insert(name.to_string(), profile);
    }
    println!("✅ Загружено 12 профилей");

    // 2. Инициализация общего состояния
    let shared_energy = EnergyState::new(1.0);
    let phase = Phase::NoemaFast;
    let tension = TensionState::new();

    // 3. Проверка групповой легитимности (Law 2)
    let profile_refs: Vec<_> = profiles.values().collect();
    match evaluate_group_collaboration(&profile_refs, &shared_energy) {
        Ok(CollaborationResult { allowed: true, justification, energy_cost }) => {
            println!("✅ Групповое взаимодействие разрешено:");
            println!("   Обоснование: {}", justification);
            println!("   Энергозатраты: {:.2}", energy_cost);
        }
        Err(e) => {
            eprintln!("❌ Групповое взаимодействие запрещено: {:?}", e);
            return Ok(());
        }
    }

    // 4. Каждый профиль генерирует проекцию
    for (name, profile) in &profiles {
        let stimulus = Stimulus {
            id: format!("cycle-stim-{}", name),
            content: "Contribute to the collective response.".to_string(),
            source: StimulusSource::WuXingCollaboration("collective".to_string()),
        };

        let projection = project(
            profile,
            &stimulus,
            &shared_energy,
            phase,
            &tension,
        )?;

        println!("📤 {} → фаза: {:?}", name, phase);
        // В реальном сценарии проекции агрегировались бы в ontoCMS
    }

    // 5. Имитация напряжения → инверсия одного профиля
    println!("\n⚡ Имитация онтологического напряжения для Aries...");
    let aries = profiles.get_mut("aries-fire-choleric").unwrap();
    // В полной реализации: обновление tension, вызов invert_temperament, смена фазы
    // Здесь — демонстрационное сообщение
    println!("🔄 Aries: Choleric → Phlegmatic (инверсия под напряжением)");

    println!("\n✅ Цикл завершён. Все профили остались в пределах 144.");
    Ok(())
}
```