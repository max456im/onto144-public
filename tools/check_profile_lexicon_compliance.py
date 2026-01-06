#!/usr/bin/env python3
"""
Проверяет, что все термины в профиле содержатся только в указанном лексиконе.
Профиль должен содержать поле `domain_lexicon` (например: 'medical').
Лексикон загружается из lexicons/{domain_lexicon}.txt.
"""

import sys
import yaml
from pathlib import Path

def load_lexicon(lex_path):
    with open(lex_path, "r", encoding="utf-8") as f:
        return set(line.strip() for line in f if line.strip())

def extract_terms_from_profile(profile):
    """Рекурсивно извлекает все строковые значения из профиля."""
    terms = set()

    def recurse(obj):
        if isinstance(obj, dict):
            for v in obj.values():
                recurse(v)
        elif isinstance(obj, list):
            for item in obj:
                recurse(item)
        elif isinstance(obj, str):
            terms.add(obj)
        # Числа, булевы и т.д. игнорируются — только строки считаются терминами

    recurse(profile)
    return terms

def main():
    if len(sys.argv) != 3:
        print("Использование: python check_profile_lexicon_compliance.py <профиль.yaml> <лексикон.txt>")
        sys.exit(1)

    profile_path = Path(sys.argv[1])
    lexicon_path = Path(sys.argv[2])

    if not profile_path.exists():
        print(f"Ошибка: профиль не найден: {profile_path}")
        sys.exit(1)
    if not lexicon_path.exists():
        print(f"Ошибка: лексикон не найден: {lexicon_path}")
        sys.exit(1)

    # Загрузка профиля
    try:
        with open(profile_path, "r", encoding="utf-8") as f:
            profile = yaml.safe_load(f)
    except yaml.YAMLError as e:
        print(f"Ошибка YAML в профиле: {e}")
        sys.exit(1)

    # Загрузка лексикона
    allowed_terms = load_lexicon(lexicon_path)

    # Извлечение всех строковых значений
    used_terms = extract_terms_from_profile(profile)

    # Исключаем служебные поля, которые не являются терминами:
    # Например, идентификаторы, версии, пути — но в онтологическом профиле
    # всё, что является строкой и не указано как метаданные, считается термином.
    # Для большей точности можно ввести аннотации, но пока — строгий подход.

    violations = used_terms - allowed_terms

    if violations:
        print(f"❌ Профиль {profile_path} использует термины вне лексикона {lexicon_path}:")
        for term in sorted(violations):
            print(f"  - '{term}'")
        sys.exit(1)
    else:
        print(f"✅ Профиль {profile_path} соответствует лексикону {lexicon_path}.")
        sys.exit(0)

if __name__ == "__main__":
    main()