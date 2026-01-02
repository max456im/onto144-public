```bash
#!/bin/bash
# SPDX-License-Identifier: GPL-3.0-only
# Валидация всех профилей через Rust-валидатор SGCL.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PROFILES_DIR="$ROOT_DIR/profiles"

if [ ! -d "$PROFILES_DIR" ] || [ -z "$(ls -A "$PROFILES_DIR")" ]; then
    echo "❌ Директория profiles/ пуста или отсутствует. Запустите сначала: python scripts/generate_profiles.py"
    exit 1
fi

# Собираем утилиту валидации (если не существует)
# Предполагается, что в проекте есть бинарник `sgcl-validator` или используется integration test
# Но для простоты — запускаем через cargo test с флагом --no-run, затем отдельный бинарник.

# Альтернатива: используем существующий тест как валидатор
# Создадим временный бинарник для валидации

VALIDATOR_SRC="$ROOT_DIR/src/bin/sgcl_validator_cli.rs"

# Создаём CLI-валидатор на лету (если его нет)
if [ ! -f "$VALIDATOR_SRC" ]; then
    cat > "$VALIDATOR_SRC" << 'EOF'
// SPDX-License-Identifier: GPL-3.0-only
// Временный CLI-валидатор для скриптов.

use std::env;
use std::fs;
use onto144::core::sgcl_validator::validate_sgcl;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Использование: sgcl_validator_cli <файл.sgcl>");
        std::process::exit(1);
    }
    let content = fs::read_to_string(&args[1]).expect("Не удалось прочитать файл");
    match validate_sgcl(&content) {
        Ok(_) => {
            println!("✅ {}", args[1]);
        }
        Err(e) => {
            eprintln!("❌ {}: {}", args[1], e);
            std::process::exit(1);
        }
    }
}
EOF
fi

echo "🛠 Сборка валидатора..."
cargo build --bin sgcl_validator_cli --quiet

VALIDATOR_BIN="$ROOT_DIR/target/debug/sgcl_validator_cli"

echo "🔍 Валидация всех профилей в $PROFILES_DIR..."
FAILED=0
TOTAL=0

for profile in "$PROFILES_DIR"/*.sgcl; do
    if [ -f "$profile" ]; then
        TOTAL=$((TOTAL + 1))
        if ! "$VALIDATOR_BIN" "$profile"; then
            FAILED=$((FAILED + 1))
        fi
    fi
done

echo
if [ $FAILED -eq 0 ]; then
    echo "✅ Все $TOTAL профилей валидны."
    # Удаляем временный исходник (опционально)
    # rm -f "$VALIDATOR_SRC"
else
    echo "❌ $FAILED из $TOTAL профилей не прошли валидацию."
    exit 1
fi
```