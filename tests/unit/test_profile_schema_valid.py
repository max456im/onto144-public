import pytest
import yaml
import jsonschema
from pathlib import Path

# Предполагаем, что схема находится в корне specs/
SCHEMA_PATH = Path(__file__).parent.parent.parent / "specs" / "profile-schema.yaml"
PROFILE_EXAMPLE = Path(__file__).parent.parent.parent / "examples" / "create_custom_profile_template.yaml"

def test_profile_schema_valid():
    with open(SCHEMA_PATH, encoding="utf-8") as f:
        schema = yaml.safe_load(f)
    with open(PROFILE_EXAMPLE, encoding="utf-8") as f:
        profile = yaml.safe_load(f)

    # Валидация схемы
    jsonschema.validate(instance=profile, schema=schema)