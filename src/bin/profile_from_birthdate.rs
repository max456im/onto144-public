```rust
// SPDX-License-Identifier: GPL-3.0-only
//! Утилита: генерация онтологически легитимного профиля на основе даты рождения.
//! Использование: cargo run --bin profile_from_birthdate -- 1985-05-15 choleric

use chrono::{NaiveDate, Datelike};
use std::collections::HashMap;

#[derive(Debug)]
struct ZodiacProfile {
    sign: &'static str,
    element: &'static str,
    allowed_temperaments: Vec<&'static str>,
}

fn zodiac_map() -> Vec<ZodiacProfile> {
    vec![
        ZodiacProfile { sign: "Aries",      element: "Fire",    allowed_temperaments: vec!["Choleric", "Sanguine"] },
        ZodiacProfile { sign: "Taurus",     element: "Earth",   allowed_temperaments: vec!["Phlegmatic", "Melancholic"] },
        ZodiacProfile { sign: "Gemini",     element: "Metal",   allowed_temperaments: vec!["Melancholic", "Phlegmatic"] },
        ZodiacProfile { sign: "Cancer",     element: "Water",   allowed_temperaments: vec!["Melancholic", "Sanguine"] },
        ZodiacProfile { sign: "Leo",        element: "Fire",    allowed_temperaments: vec!["Choleric", "Sanguine"] },
        ZodiacProfile { sign: "Virgo",      element: "Earth",   allowed_temperaments: vec!["Phlegmatic", "Melancholic"] },
        ZodiacProfile { sign: "Libra",      element: "Metal",   allowed_temperaments: vec!["Melancholic", "Phlegmatic"] },
        ZodiacProfile { sign: "Scorpio",    element: "Water",   allowed_temperaments: vec!["Melancholic", "Sanguine"] },
        ZodiacProfile { sign: "Sagittarius",element: "Fire",    allowed_temperaments: vec!["Choleric", "Sanguine"] },
        ZodiacProfile { sign: "Capricorn",  element: "Earth",   allowed_temperaments: vec!["Phlegmatic", "Melancholic"] },
        ZodiacProfile { sign: "Aquarius",   element: "Metal",   allowed_temperaments: vec!["Melancholic", "Phlegmatic"] },
        ZodiacProfile { sign: "Pisces",     element: "Water",   allowed_temperaments: vec!["Melancholic", "Sanguine"] },
    ]
}

fn get_zodiac_profile(month: u32, day: u32) -> Option<ZodiacProfile> {
    match (month, day) {
        (3, 21..=31) | (4, 1..=19) => Some(zodiac_map()[0].clone()),
        (4, 20..=30) | (5, 1..=20) => Some(zodiac_map()[1].clone()),
        (5, 21..=31) | (6, 1..=20) => Some(zodiac_map()[2].clone()),
        (6, 21..=30) | (7, 1..=22) => Some(zodiac_map()[3].clone()),
        (7, 23..=31) | (8, 1..=22) => Some(zodiac_map()[4].clone()),
        (8, 23..=31) | (9, 1..=22) => Some(zodiac_map()[5].clone()),
        (9, 23..=30) | (10, 1..=22) => Some(zodiac_map()[6].clone()),
        (10, 23..=31) | (11, 1..=21) => Some(zodiac_map()[7].clone()),
        (11, 22..=30) | (12, 1..=21) => Some(zodiac_map()[8].clone()),
        (12, 22..=31) | (1, 1..=19) => Some(zodiac_map()[9].clone()),
        (1, 20..=31) | (2, 1..=18) => Some(zodiac_map()[10].clone()),
        (2, 19..=29) | (3, 1..=20) => Some(zodiac_map()[11].clone()),
        _ => None,
    }
}

fn normalize_temperament(s: &str) -> Option<&'static str> {
    match s.to_lowercase().as_str() {
        "choleric" => Some("Choleric"),
        "sanguine" => Some("Sanguine"),
        "melancholic" => Some("Melancholic"),
        "phlegmatic" => Some("Phlegmatic"),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Использование: profile_from_birthdate <YYYY-MM-DD> [temperament]");
        eprintln!("Пример: profile_from_birthdate 1985-05-15 choleric");
        std::process::exit(1);
    }

    let date_str = &args[1];
    let user_temperament = args.get(2).map(|s| s.as_str());

    // Парсинг даты
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|_| "Неверный формат даты. Используйте YYYY-MM-DD")?;

    // Определение знака
    let profile = get_zodiac_profile(date.month(), date.day())
        .ok_or("Не удалось определить зодиакальный знак")?;

    // Определение темперамента
    let final_temperament = if let Some(temp_str) = user_temperament {
        if let Some(normalized) = normalize_temperament(temp_str) {
            if profile.allowed_temperaments.contains(&normalized) {
                normalized.to_string()
            } else {
                // Коррекция: выбираем первый допустимый темперамент
                eprintln!(
                    "⚠️  '{}' не совместим с {}. Выбран '{}'.",
                    normalized, profile.element, profile.allowed_temperaments[0]
                );
                profile.allowed_temperaments[0].to_string()
            }
        } else {
            eprintln!("⚠️  Неизвестный темперамент '{}'. Используется '{}'.", temp_str, profile.allowed_temperaments[0]);
            profile.allowed_temperaments[0].to_string()
        }
    } else {
        // Без указания темперамента — используем первый по умолчанию
        profile.allowed_temperaments[0].to_string()
    };

    // Формирование имени файла
    let filename = format!(
        "{}-{}-{}.sgcl",
        profile.sign.to_lowercase(),
        profile.element.to_lowercase(),
        final_temperament.to_lowercase()
    );

    println!("sign: {}", profile.sign);
    println!("element: {}", profile.element);
    println!("temperament: {}", final_temperament);
    println!("profile_file: {}", filename);
    println!("ontologically_valid: true");

    Ok(())
}
```