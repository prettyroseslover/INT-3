# INT-3
> Многопоточный сервер и однопоточный клиент, реализованные на Rust

Запуск сервера:
```bash
cargo run --bin server -- -q [путь/к/директрии/карантина]
```

Запуск Rust-клиента можно производить в двух режимах, в зависимости от выбранной команды:
```bash
# CheckLocalFileParams
cargo run --bin client -- check-local-file -r [путь/к/файлу] < [файл/с/сигнатурой]
# ИЛИ
echo -n -e '\x[сигнатура]' | cargo run --bin client -- check-local-file -r [путь/к/файлу]

# QuarantineLocalFile
cargo run --bin client -- quarantine-local-file -r [путь/к/файлу]
```