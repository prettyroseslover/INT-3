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

## Дополнительные тесты 

Запускаю сервер со следующей конфигурацией (12 потоков):
```bash 
cargo run --bin server -- -a 192.168.0.105:3000 -t 12 -q ./quarantine
```

Для отправки нагрузки я использую утилиту [oha](https://github.com/hatoo/oha). Запускаю ее со следующими параметрами (приостановила тестирование чуть раньше, на 7 миллионах, а не 10):
```bash
$ oha -c 1000 -n 10000000 -m POST  -d '{"command1": "CheckLocalFile","params": {"path": "test.txt","signature": [108,108]}}' -H 'Content-Type:application/json' http://192.168.0.105:3000

Summary:
  Success rate: 100.00%
  Total:        486.4772 secs
  Slowest:      1.1916 secs
  Fastest:      0.0004 secs
  Average:      0.0693 secs
  Requests/sec: 14434.3795

  Total data:   93.75 MiB
  Size/request: 14
  Size/sec:     197.34 KiB

Response time histogram:
  0.000 [1]       |
  0.120 [4517938] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.239 [2464602] |■■■■■■■■■■■■■■■■■
  0.358 [37681]   |
  0.477 [1303]    |
  0.596 [226]     |
  0.715 [47]      |
  0.834 [0]       |
  0.953 [196]     |
  1.072 [0]       |
  1.192 [3]       |

Response time distribution:
  10.00% in 0.0126 secs
  25.00% in 0.0156 secs
  50.00% in 0.0226 secs
  75.00% in 0.1282 secs
  90.00% in 0.1414 secs
  95.00% in 0.1759 secs
  99.00% in 0.2220 secs
  99.90% in 0.2827 secs
  99.99% in 0.4271 secs


Details (average, fastest, slowest):
  DNS+dialup:   0.0380 secs, 0.0002 secs, 0.1950 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0070 secs

Status code distribution:
  [200] 7021997 responses

```

Проверяю нагрузку по всем 12-ти потокам, используя простой `top`. До запуска нагрузки все потоки были в `S`-состоянии. После запуска нагрузки они переключились в состояние `R`-состояние, а нагрузка распространялась достаточно равномерно между всеми потоками. Спустя какое-то время нагрузка стабилизировалась по ~50% на поток.

```bash 
$ top -b -H | ts | grep tokio 

Jun 15 14:28:13    6486 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:42.02 tokio-r+
Jun 15 14:28:13    6487 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:40.49 tokio-r+
Jun 15 14:28:13    6488 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:41.93 tokio-r+
Jun 15 14:28:13    6489 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:40.66 tokio-r+
Jun 15 14:28:13    6490 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:41.27 tokio-r+
Jun 15 14:28:13    6491 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:39.98 tokio-r+
Jun 15 14:28:13    6492 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:41.13 tokio-r+
Jun 15 14:28:13    6493 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:40.65 tokio-r+
Jun 15 14:28:13    6494 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:40.73 tokio-r+
Jun 15 14:28:13    6495 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:41.77 tokio-r+
Jun 15 14:28:13    6496 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:41.24 tokio-r+
Jun 15 14:28:13    6497 prettyr+  20   0  829048  50516   8376 S   0.0   0.2   0:41.37 tokio-r+

Jun 15 14:28:16    6490 prettyr+  20   0  829048  50516   8376 R  40.3   0.2   0:42.49 tokio-r+
Jun 15 14:28:16    6497 prettyr+  20   0  829048  50516   8376 R  40.3   0.2   0:42.59 tokio-r+
Jun 15 14:28:16    6486 prettyr+  20   0  829048  50516   8376 R  39.9   0.2   0:43.23 tokio-r+
Jun 15 14:28:16    6488 prettyr+  20   0  829048  50516   8376 R  39.9   0.2   0:43.14 tokio-r+
Jun 15 14:28:16    6493 prettyr+  20   0  829048  50516   8376 R  39.9   0.2   0:41.86 tokio-r+
Jun 15 14:28:16    6495 prettyr+  20   0  829048  50516   8376 R  39.9   0.2   0:42.98 tokio-r+
Jun 15 14:28:16    6487 prettyr+  20   0  829048  50516   8376 R  38.9   0.2   0:41.67 tokio-r+
Jun 15 14:28:16    6496 prettyr+  20   0  829048  50516   8376 R  38.6   0.2   0:42.41 tokio-r+
Jun 15 14:28:16    6492 prettyr+  20   0  829048  50516   8376 R  37.3   0.2   0:42.26 tokio-r+
Jun 15 14:28:16    6491 prettyr+  20   0  829048  50516   8376 R  37.0   0.2   0:41.10 tokio-r+
Jun 15 14:28:16    6489 prettyr+  20   0  829048  50516   8376 R  36.3   0.2   0:41.76 tokio-r+
Jun 15 14:28:16    6494 prettyr+  20   0  829048  50516   8376 R  36.0   0.2   0:41.82 tokio-r+

Jun 15 14:28:19    6486 prettyr+  20   0  829048  50516   8376 R  83.3   0.2   0:45.78 tokio-r+
Jun 15 14:28:19    6490 prettyr+  20   0  829048  50516   8376 R  82.7   0.2   0:45.02 tokio-r+
Jun 15 14:28:19    6491 prettyr+  20   0  829048  50516   8376 R  81.7   0.2   0:43.60 tokio-r+
Jun 15 14:28:19    6488 prettyr+  20   0  829048  50516   8376 R  80.7   0.2   0:45.61 tokio-r+
Jun 15 14:28:19    6492 prettyr+  20   0  829048  50516   8376 R  80.7   0.2   0:44.73 tokio-r+
Jun 15 14:28:19    6493 prettyr+  20   0  829048  50516   8376 R  80.7   0.2   0:44.33 tokio-r+
Jun 15 14:28:19    6487 prettyr+  20   0  829048  50516   8376 R  80.4   0.2   0:44.13 tokio-r+
Jun 15 14:28:19    6495 prettyr+  20   0  829048  50516   8376 R  78.8   0.2   0:45.39 tokio-r+
Jun 15 14:28:19    6496 prettyr+  20   0  829048  50516   8376 R  78.8   0.2   0:44.82 tokio-r+
Jun 15 14:28:19    6497 prettyr+  20   0  829048  50516   8376 R  77.8   0.2   0:44.97 tokio-r+
Jun 15 14:28:19    6489 prettyr+  20   0  829048  50516   8376 R  76.8   0.2   0:44.11 tokio-r+
Jun 15 14:28:19    6494 prettyr+  20   0  829048  50516   8376 R  74.8   0.2   0:44.11 tokio-r+

<...>

Jun 15 14:36:00    6494 prettyr+  20   0  829048  51148   8648 R  54.9   0.2   5:04.61 tokio-r+
Jun 15 14:36:00    6487 prettyr+  20   0  829048  51148   8648 S  54.6   0.2   5:04.61 tokio-r+
Jun 15 14:36:00    6488 prettyr+  20   0  829048  51148   8648 R  54.6   0.2   5:06.31 tokio-r+
Jun 15 14:36:00    6486 prettyr+  20   0  829048  51148   8648 R  54.2   0.2   5:05.80 tokio-r+
Jun 15 14:36:00    6495 prettyr+  20   0  829048  51148   8648 S  53.9   0.2   5:06.15 tokio-r+
Jun 15 14:36:00    6497 prettyr+  20   0  829048  51148   8648 S  53.9   0.2   5:04.07 tokio-r+
Jun 15 14:36:00    6489 prettyr+  20   0  829048  51148   8648 R  52.9   0.2   5:03.33 tokio-r+
Jun 15 14:36:00    6493 prettyr+  20   0  829048  51148   8648 R  52.6   0.2   5:05.16 tokio-r+
Jun 15 14:36:00    6496 prettyr+  20   0  829048  51148   8648 R  49.3   0.2   5:06.11 tokio-r+
Jun 15 14:36:03    6488 prettyr+  20   0  829048  51148   8648 S  57.0   0.2   5:08.05 tokio-r+
Jun 15 14:36:03    6493 prettyr+  20   0  829048  51148   8648 S  56.4   0.2   5:06.88 tokio-r+
Jun 15 14:36:03    6495 prettyr+  20   0  829048  51148   8648 S  55.7   0.2   5:07.85 tokio-r+

```