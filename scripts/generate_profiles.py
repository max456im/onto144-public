```python
#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
"""
Генератор профилей onto144.
Создаёт 24 SGCL-файла в директории profiles/ на основе config/zodiac.yaml.
Каждый файл содержит:
  - sign
  - element
  - temperament
  - ethics_hash (SHA256 от нормализованного содержимого)
  - version

Профили соответствуют правилам из PROFILE_GENERATION_RULES.md.
"""

import os
import yaml
import hashlib
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
ROOT_DIR = SCRIPT_DIR.parent
CONFIG_DIR = ROOT_DIR / "config"
PROFILES_DIR = ROOT_DIR / "profiles"

def normalize_sgcl(content: str) -> str:
    """Нормализация SGCL для хеширования: удаляет комментарии и лишние пробелы."""
    lines = []
    for line in content.strip().splitlines():
        stripped = line.strip()
        if stripped and not stripped.startswith('#'):
            lines.append(stripped)
    return '\n'.join(lines)

def compute_ethics_hash(sgcl_content: str) -> str:
    """Вычисляет SHA256 хеш нормализованного SGCL."""
    normalized = normalize_sgcl(sgcl_content)
    return 'sha256:' + hashlib.sha256(normalized.encode('utf-8')).hexdigest()

def load_zodiac_config():
    with open(CONFIG_DIR / "zodiac.yaml", 'r', encoding='utf-8') as f:
        return yaml.safe_load(f)['zodiac']

def ensure_profiles_dir():
    PROFILES_DIR.mkdir(exist_ok=True)

def main():
    ensure_profiles_dir()
    zodiac_entries = load_zodiac_config()
    generated_count = 0

    for entry in zodiac_entries:
        sign = entry['sign']
        element = entry['element']
        temperaments = entry['base_temperaments']

        for temp in temperaments:
            # Формируем имя файла в нижнем регистре
            filename = f"{sign.lower()}-{element.lower()}-{temp.lower()}.sgcl"
            filepath = PROFILES_DIR / filename

            # Создаём SGCL-контент (без ethics_hash — он будет добавлен после)
            sgcl_lines = [
                f"sign: {sign}",
                f"element: {element}",
                f"temperament: {temp}",
                "version: 1.0"
            ]
            sgcl_content = '\n'.join(sgcl_lines) + '\n'

            # Вычисляем хеш и добавляем его
            ethics_hash = compute_ethics_hash(sgcl_content)
            full_sgcl = sgcl_content.replace("version: 1.0", f"ethics_hash: {ethics_hash}\nversion: 1.0")

            # Записываем файл
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(full_sgcl)
            
            print(f"✅ Сгенерирован: {filename}")
            generated_count += 1

    print(f"\nВсего сгенерировано профилей: {generated_count}")
    assert generated_count == 24, f"Ожидалось 24 профиля, получено {generated_count}"

if __name__ == "__main__":
    main()
```