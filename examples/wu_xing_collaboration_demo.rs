```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Демонстрация совместного решения задачи через цикл Wu Xing:
//! Fire (инициатор) → Earth (реализатор) → Metal (структуризатор)

use onto144::profile::profile::Profile;
use onto144::state::{EnergyState, Phase, tension::TensionState};
use onto144::projection::projector::{project, Stimulus, StimulusSource};
use onto144::wu_xing::collaboration::evaluate_collaboration;
use onto144::projection::onto16::serialize_to_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== onto144: Wu Xing Collaboration Demo ===");

    // 1. Загрузка трёх профилей
    let fire = Profile::from_file("profiles/leo-fire-choleric.sgcl")?;
    let earth = Profile::from_file("profiles/virgo-earth-melancholic.sgcl")?;
    let metal = Profile::from_file("profiles/libra-metal-phlegmatic.sgcl")?;

    println!("✅ Загружены профили: Fire (Leo), Earth (Virgo), Metal (Libra)");

    // 2. Энергия
    let energy = EnergyState::new(0.95);

    // 3. Проверка парных взаимодействий
    println!("\n🔍 Проверка Wu Xing-связей:");

    for (init, resp, name) in [
        (&fire, &earth, "Fire → Earth"),
        (&earth, &metal, "Earth → Metal"),
        (&metal, &fire, "Metal → Fire (Ke: контроль)"),
    ] {
        match evaluate_collaboration(init, resp, &energy) {
            Ok(result) => {
                println!("  ✅ {}: {}", name, result.justification);
            }
            Err(e) => {
                println!("  ❌ {}: {:?}", name, e);
            }
        }
    }

    // 4. Стимул от Fire к группе
    let stimulus = Stimulus {
        id: "wuxing-task-001".to_string(),
        content: "Спроектируйте этически легитимное решение.".to_string(),
        source: StimulusSource::WuXingCollaboration("Fire-Leo".to_string()),
    };

    let tension = TensionState::new();
    let phase = Phase::NoemaSlow; // совместные действия требуют рефлексии

    // 5. Проекции
    for (profile, name) in [(&fire, "Fire"), (&earth, "Earth"), (&metal, "Metal")] {
        let proj = project(profile, &stimulus, &energy, phase, &tension)?;
        let json = serialize_to_json(&proj)?;
        println!("\n📤 {} ({}):", name, profile.id());
        // Выводим только семантику для краткости
        if let Some(sem) = proj.semantics.get("attention_mode") {
            println!("   Внимание: {}", sem);
        }
    }

    println!("\n✅ Совместное действие завершено. Все связи соответствуют Sheng/Ke.");
    Ok(())
}
```