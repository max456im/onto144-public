#!/usr/bin/env python3
"""
Проверка формата лексикона:
- Одна лексема на строку
- Без пустых строк
- Без дубликатов
- Только непустые строки без ведущих/завершающих пробелов
"""

import sys
from pathlib import Path

def main():
    if len(sys.argv) != 2:
        print("Использование: python validate_lexicon.py <путь_к_лексикону.txt>")
        sys.exit(1)

    lex_path = Path(sys.argv[1])
    if not lex_path.exists():
        print(f"Ошибка: лексикон не найден: {lex_path}")
        sys.exit(1)

    with open(lex_path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    seen = set()
    errors = []

    for i, line in enumerate(lines, start=1):
        original = line
        stripped = line.strip()

        if not stripped:
            errors.append(f"Строка {i}: пустая строка")
            continue

        if stripped != original.rstrip('\n'):
            errors.append(f"Строка {i}: недопустимые пробелы в начале или конце: '{original.rstrip()}'")
            continue

        if stripped in seen:
            errors.append(f"Строка {i}: дубликат: '{stripped}'")
        else:
            seen.add(stripped)

    if errors:
        print(f"❌ Ошибки в лексиконе {lex_path}:")
        for err in errors:
            print(f"  - {err}")
        sys.exit(1)
    else:
        print(f"✅ Лексикон {lex_path} корректен: {len(seen)} уникальных терминов.")
        sys.exit(0)

if __name__ == "__main__":
    main()