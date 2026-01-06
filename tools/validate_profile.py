#!/usr/bin/env python3
"""
Валидация онтологического профиля по YAML-схеме.
Использует specs/profile-schema.yaml.
"""

import sys
import yaml
from jsonschema import validate, ValidationError
from pathlib import Path

def main():
    if len(sys.argv) != 2:
        print("Использование: python validate_profile.py <путь_к_профилю.yaml>")
        sys.exit(1)

    profile_path = Path(sys.argv[1])
    schema_path = Path(__file__).parent.parent / "specs" / "profile-schema.yaml"

    if not profile_path.exists():
        print(f"Ошибка: файл профиля не найден: {profile_path}")
        sys.exit(1)
    if not schema_path.exists():
        print(f"Ошибка: схема не найдена: {schema_path}")
        sys.exit(1)

    try:
        with open(profile_path, "r", encoding="utf-8") as f:
            profile = yaml.safe_load(f)
        with open(schema_path, "r", encoding="utf-8") as f:
            schema = yaml.safe_load(f)
    except yaml.YAMLError as e:
        print(f"Ошибка YAML: {e}")
        sys.exit(1)

    try:
        validate(instance=profile, schema=schema)
        print(f"✅ Профиль {profile_path} прошёл валидацию по схеме.")
    except ValidationError as e:
        print(f"❌ Ошибка валидации профиля {profile_path}:")
        print(f"Путь: {'.'.join(str(p) for p in e.path)}")
        print(f"Сообщение: {e.message}")
        sys.exit(1)

if __name__ == "__main__":
    main()