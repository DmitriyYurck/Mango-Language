![mango](https://free-png.ru/wp-content/uploads/2022/02/free-png.ru-494-370x364.png)
Mango Language

Mango — минималистичный интерпретируемый язык программирования, созданный для изучения архитектуры интерпретаторов.  
Mango is a minimalist interpreted programming language designed to explore interpreter internals and execution flow.

---

Пример кода / Example Code

`mango
let a = 10;
let b = 5;
let sum = a + b
print sum;

if sum > 20 {
    print "Sum is large";
} else {
    print "Sum is small";
}

func multiply(x, y) {
    return x * y;
}

let result = multiply(3, 4);
print result;
`

---

Установка и запуск / Installation & Execution

Требования / Requirements
- Rust ≥ 1.70
- Git

Инструкция / Steps

`bash

Клонируйте репозиторий
git clone https://github.com/DmitriyYurck/Mango-Language.git
cd Mango-Language

Сборка
cargo build --release

Запуск скрипта
cargo run -- examples/demo.mgo
`

Альтернативный запуск:
`bash
./target/release/mango examples/demo.mgo
`

---

Структура проекта / Project Structure

`text
Mango-Language/
├── src/
│   ├── main.rs          // Точка входа
│   ├── lexer.rs         // Лексический анализ
│   ├── parser.rs        // Синтаксический анализ
│   ├── ast.rs           // Структуры AST
│   ├── env.rs           // Окружение переменных
│   ├── interpreter.rs   // Исполнение кода
├── examples/
│   └── demo.mgo         // Пример скрипта
├── Cargo.toml           // Конфигурация Cargo
└── README.md            // Документация
`

---

Архитектура исполнения / Execution Pipeline

`text
┌────────────┐
│  .mgo файл │
└────┬───────┘
     ↓
┌────────────┐
│   Lexer    │  → токенизация
└────┬───────┘
     ↓
┌────────────┐
│   Parser   │  → построение AST
└────┬───────┘
     ↓
┌────────────┐
│    AST     │  → структура программы
└────┬───────┘
     ↓
┌────────────┐
│ Interpreter│  → выполнение
└────┬───────┘
     ↓
┌────────────┐
│ Environment│  → переменные и область видимости
└────────────┘
`

---

Вывод интерпретатора / Sample Output

`bash
$ cargo run -- examples/demo.mgo
15
Sum is small
12
`

---

Расширение / Extensibility

- Поддержка строк и операций над ними
- Расширяемый синтаксис функций
- Возможность подключения модулей
- Планируется REPL-режим и отладчик

---

Лицензия / License

Проект распространяется под лицензией MIT.  
This project is licensed under the MIT License.
