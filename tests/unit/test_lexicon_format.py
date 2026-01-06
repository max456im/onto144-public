import pytest
from pathlib import Path

LEXICON_EXAMPLE = Path(__file__).parent.parent.parent / "lexicons" / "medical.txt"

def test_lexicon_format():
    with open(LEXICON_EXAMPLE, encoding="utf-8") as f:
        lines = [line.strip() for line in f if line.strip() and not line.startswith("#")]

    # Проверка: одна лексема на строку
    assert all("\t" not in line and "," not in line and " " not in line for line in lines), \
        "Лексема должна быть однословной или составной без пробелов (например, snake_case)"

    # Проверка: без дублей
    assert len(lines) == len(set(lines)), "Обнаружены дублирующиеся лексемы"