import pytest
import yaml
from pathlib import Path

MED_PROFILE_PATH = Path(__file__).parent.parent.parent / "profiles" / "domain" / "medical" / "med-ai-2026.yaml"
MED_LEXICON_PATH = Path(__file__).parent.parent.parent / "lexicons" / "medical.txt"

def load_lexicon(path):
    with open(path, encoding="utf-8") as f:
        return {line.strip() for line in f if line.strip() and not line.startswith("#")}

def extract_terms_from_profile(profile):
    """Рекурсивно извлекает все строковые значения из профиля для проверки."""
    terms = set()
    def _walk(obj):
        if isinstance(obj, dict):
            for v in obj.values():
                _walk(v)
        elif isinstance(obj, list):
            for item in obj:
                _walk(item)
        elif isinstance(obj, str):
            # Предполагаем, что термины либо отдельные слова, либо в формате snake_case
            # В реальной системе может быть строже, но здесь — упрощённо
            terms.add(obj)
    _walk(profile)
    return terms

def test_medical_profile_compliance():
    with open(MED_PROFILE_PATH, encoding="utf-8") as f:
        profile = yaml.safe_load(f)

    allowed_terms = load_lexicon(MED_LEXICON_PATH)
    profile_terms = extract_terms_from_profile(profile)

    # Фильтруем служебные поля (например, метаданные), которые не являются терминами
    # В реальной системе такие поля должны быть чётко определены (например, `id`, `license`, `domain`)
    non_lexical_fields = {"id", "name", "license", "domain", "version", "author", "description"}
    lexical_terms = {t for t in profile_terms if t not in non_lexical_fields and not t.startswith("GPL")}

    illegal_terms = lexical_terms - allowed_terms
    assert not illegal_terms, f"Профиль использует термины вне medical.txt: {illegal_terms}"