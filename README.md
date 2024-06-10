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

### Запуск unit-тестов
```bash
$ cargo test -q

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Bonus: Python-клиент
> Дополнительный клиент, реализованный на Python

После просмотра вводного интенсива, где раскрывались особенности команды, я обратила внимание на то, что ваша команда использует Python для простоты и быстроты реализации бизнес-логики и тестирования, а в моментах, где производительности Python не достаточно - Rust. Это натолкнуло меня на идею не просто реализовать Rust клиент-сервер, но и попробовать создать дополнительный клиент на Python, чтобы он интегрировался (и переиспользовал) часть логики, уже реализованной на Rust. Для поддержания единой точки контракта я переиспользую `lib.rs`, а точнее сериализацию json'а. Такая интеграция Rust-кода в Python-код реализована через библиотеку [PyO3](https://pyo3.rs/v0.21.2/). Кроме того, такой выбор языков для клиента и сервера отражает типичные требования производительности подобного ПО.

### Подготовка среды и использование

Для менеджмента зависимостей использую [Poetry](https://python-poetry.org/), поскольку он не только является соверменным и удобным инструментом для Python-разработки, но и интеграируется с [maturin](https://www.maturin.rs/), который используется для интеграции с Rust. Перед первым запуском необходимо подготовить среду:
```bash
poetry install
poetry run maturin develop # сборка модуля Rust
```

Опции Python-клиента подобны клиенту, реализованному через Rust. Пример: 

```bash
$ poetry run python py-client/py-client.py --help

usage: py-client.py [-h] [-u URL] {check-local-file,quarantine-local-file} ...

Single-threaded Python client

positional arguments:
  {check-local-file,quarantine-local-file}

options:
  -h, --help            show this help message and exit
  -u URL, --url URL     URL of the server. Default http://127.0.0.1:3000
```

Таким образом, после запуска сервера и удачной сборки Python-клиента им можно пользоваться:
```bash
# CheckLocalFileParams
poetry run python py-client/py-client.py check-local-file -r [путь/к/файлу] < [файл/с/сигнатурой]
# ИЛИ
echo -n -e '\x[сигнатура]' | poetry run python py-client/py-client.py check-local-file -r [путь/к/файлу]

# QuarantineLocalFile
poetry run python py-client/py-client.py quarantine-local-file -r [путь/к/файлу]
```

### Запуск unit-тестов
```bash
$ poetry run pytest py-client/test.py -q
.. [100%]                                                                                                                                                                                                                                    
2 passed in 0.01s
```