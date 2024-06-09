# INT-3

Запуск сервера:
```bash
cargo run --bin server
```

Запуск Rust-клиента можно производить в двух режимах, в зависимости от выбранной команды:
```bash
# CheckLocalFileParams
cargo run --bin client -- check-local-file -r <ПУТЬ К ФАЙЛУ> < <ФАЙЛ С СИГНАТУРОЙ>
# ИЛИ
echo -n -e '\xСИГНАТУРА' | cargo run --bin client -- check-local-file -r <ПУТЬ К ФАЙЛУ>

# QuarantineLocalFile
cargo run --bin client -- quarantine-local-file -r <ПУТЬ К ФАЙЛУ>
```