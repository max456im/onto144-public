```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Демонстрация работы одного профиля: загрузка, стимуляция, проекция.

use onto144::profile::profile::Profile;
use onto144::core::kernel::KernelInvariants;
use onto144::state::{EnergyState, Phase, tension::TensionState, phase::PhaseManager};
use onto144::projection::projector::{project, Stimulus, StimulusSource};
use onto144::projection::onto16::serialize_to_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== onto144: Single Profile Demo ===");

    // 1. Загрузка профиля
    let profile = Profile::from_file("profiles/aries-fire-choleric.sgcl")?;
    println!("✅ Загружен профиль: {}", profile.id());

    // 2. Инициализация состояния
    let mut energy = EnergyState::new(0.9);
    let mut phase_manager = PhaseManager::new(Phase::NoemaFast);
    let tension = TensionState::new();

    // 3. Ядро: проверка инвариантов
    let kernel = KernelInvariants::new(
        energy.clone(),
        phase_manager.current(),
        true, // ethics_compliant (уже проверено при загрузке профиля)
    );
    kernel.enforce_invariants();
    println!("✅ Ядро: инварианты соблюдены");

    // 4. Стимул
    let stimulus = Stimulus {
        id: "user-query-001".to_string(),
        content: "What is your action in this situation?".to_string(),
        source: StimulusSource::External,
    };

    // 5. Проекция
    let projection = project(
        &profile,
        &stimulus,
        &energy,
        phase_manager.current(),
        &tension,
    )?;

    // 6. Вывод
    let output = serialize_to_json(&projection)?;
    println!("📤 Проекция (NoemaFast, provisional):\n{}", output);

    // 7. Рефлексия: переключение в NoemaSlow
    phase_manager.force_reflective(onto144::state::phase::PhaseTransitionReason::ManualOverride);
    let reflective_projection = project(
        &profile,
        &stimulus,
        &energy,
        phase_manager.current(),
        &tension,
    )?;
    let reflective_output = serialize_to_json(&reflective_projection)?;
    println!("\n🔄 Рефлексивная проекция (NoemaSlow, permanent):\n{}", reflective_output);

    println!("\n✅ Демо завершено. Профиль остался в рамках 144.");
    Ok(())
}
```