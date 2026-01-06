#!/usr/bin/env python3
"""
Пример загрузки и валидации медицинского профиля
с использованием соответствующего лексикона.

Этот скрипт демонстрирует:
- Загрузку профиля из YAML
- Загрузку доменного лексикона
- Простейшую проверку, что все термины в профиле
  присутствуют в разрешённом лексиконе

Требования: см. tools/requirements.txt
"""

import yaml
import sys
from pathlib import Path

def load_lexicon(path: Path) -> set:
    """Загружает лексикон как множество терминов (один термин на строку, без дублей)."""
    with open(path, 'r', encoding='utf-8') as f:
        return {line.strip() for line in f if line.strip() and not line.startswith('#')}

def load_profile(path: Path) -> dict:
    """Загружает профиль в формате YAML."""
    with open(path, 'r', encoding='utf-8') as f:
        return yaml.safe_load(f)

def main():
    profile_path = Path(__file__).parent.parent / "profiles" / "domain" / "medical" / "med-ai-2026.yaml"
    lexicon_path = Path(__file__).parent.parent / "lexicons" / "medical.txt"

    if not profile_path.exists():
        print(f"Ошибка: профиль не найден: {profile_path}", file=sys.stderr)
        sys.exit(1)
    if not lexicon_path.exists():
        print(f"Ошибка: лексикон не найден: {lexicon_path}", file=sys.stderr)
        sys.exit(1)

    profile = load_profile(profile_path)
    lexicon = load_lexicon(lexicon_path)

    # Извлекаем все строковые значения из профиля (рекурсивно)
    def extract_strings(obj, results=None):
        if results is None:
            results = []
        if isinstance(obj, str):
            results.append(obj)
        elif isinstance(obj, dict):
            for v in obj.values():
                extract_strings(v, results)
        elif isinstance(obj, list):
            for item in obj:
                extract_strings(item, results)
        return results

    profile_terms = set(extract_strings(profile))
    unknown_terms = profile_terms - lexicon

    if unknown_terms:
        print("❌ Найдены термины вне разрешённого лексикона:")
        for term in sorted(unknown_terms):
            print(f"  - {term}")
        sys.exit(1)
    else:
        print("✅ Профиль med-ai-2026.yaml полностью соответствует medical.txt")
        print(f"   Всего терминов в профиле: {len(profile_terms)}")
        print(f"   Размер лексикона: {len(lexicon)} терминов")

if __name__ == "__main__":
    main()