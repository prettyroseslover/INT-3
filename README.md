# INT-3
> Многопоточный сервер и однопоточный клиент, реализованные на Rust

Запуск сервера (по умолчанию на 0.0.0.0:3000):
```bash
cargo run --bin server -- -q [путь/к/директории/карантина]
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

### Опции сервера
```bash
$ cargo run --bin server -- -h

Usage: server [OPTIONS] --quarantine <QUARANTINE>

Options:
  -a, --address <ADDRESS>        [default: 0.0.0.0:3000]
  -t, --threads <THREADS>        [default: 10]
  -q, --quarantine <QUARANTINE>  
  -h, --help                     Print help
  -V, --version                  Print version
```

### Опции клиента
```bash
$ cargo run --bin client -- -h

Usage: client [OPTIONS] <COMMAND>

Commands:
  check-local-file       
  quarantine-local-file  
  help                   Print this message or the help of the given subcommand(s)

Options:
  -u, --url <URL>  server address [default: http://127.0.0.1:3000/]
  -h, --help       Print help
```