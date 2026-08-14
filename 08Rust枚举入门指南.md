# Rust 枚举（Enum）入门指南

---

## 什么是枚举？

**枚举（Enum）是一种可以定义一个类型为若干个变体（variant）之一的自定义数据类型。**

与结构体将多个字段组合在一起不同，枚举表达的是"这个值是这些可能性中的一种"。比如一条消息可能是"调整大小"、"移动"、"回显"、"改变颜色"或"退出"中的某一种，用枚举来描述非常自然。

枚举的每个变体可以：
- **不带数据**：只是一个标签
- **携带元组式数据**：如 `ChangeColor(u8, u8, u8)`
- **携带结构体式数据**：如 `Resize { width: u64, height: u64 }`

### 枚举与结构体的对比

| 特性 | 结构体 | 枚举 |
|------|--------|------|
| 语义 | "并且"（AND） | "或者"（OR） |
| 字段关系 | 所有字段同时存在 | 同一时间只有一种变体 |
| 适用场景 | 描述一个事物的多个属性 | 描述一个事物的多种可能状态 |

---

## 一、无数据枚举变体

**知识点：枚举变体可以不携带任何数据，仅作为命名标签使用**

最简单的枚举形式：每个变体只是一个名字，不携带额外数据。适合表达一组互斥的状态或选项。

```rust
// #[derive(Debug)] 让枚举可以用 {:?} 格式化打印
#[derive(Debug)]
enum Message {
    Resize,       // 调整大小
    Move,         // 移动
    Echo,         // 回显
    ChangeColor,  // 改变颜色
    Quit,         // 退出
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
```

**执行结果：**
```
Resize
Move
Echo
ChangeColor
Quit
```

解析：用 `枚举名::变体名` 访问枚举值，`{:?}` 打印出变体的名称字符串。

---

## 二、带数据的枚举变体——三种形式

**知识点：枚举变体可以携带数据，有三种形式：结构体式、元组式、无数据**

这是枚举真正强大的地方——每个变体可以携带不同类型和数量的数据：

| 变体形式 | 语法 | 示例 |
|---------|------|------|
| 无数据 | `Quit` | 不携带任何值 |
| 元组式 | `ChangeColor(u8, u8, u8)` | 像元组一样，用索引访问 |
| 结构体式 | `Resize { width: u64, height: u64 }` | 像结构体一样，用字段名访问 |

```rust
#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // 结构体式变体：带命名字段
    Resize { width: u64, height: u64 },
    // 元组式变体：包裹一个 Point 结构体
    Move(Point),
    // 元组式变体：包裹一个 String
    Echo(String),
    // 元组式变体：包裹三个 u8 值
    ChangeColor(u8, u8, u8),
    // 无数据变体
    Quit,
}

impl Message {
    fn call(&self) {
        // 用 {:?} 打印枚举值，会显示变体名和携带的数据
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        // 结构体式变体：用字段名初始化
        Message::Resize {
            width: 10,
            height: 30,
        },
        // 元组式变体：用括号初始化
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        // 无数据变体：直接使用名称
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
```

**执行结果：**
```
Resize { width: 10, height: 30 }
Move(Point { x: 10, y: 15 })
Echo("hello world")
ChangeColor(200, 255, 255)
Quit
```

解析：
- `Resize` 变体携带了命名字段 `width` 和 `height`，打印为 `Resize { width: 10, height: 30 }`
- `Move` 变体包裹了 `Point` 结构体，打印为 `Move(Point { x: 10, y: 15 })`
- `Echo` 变体包裹了 `String`，打印为 `Echo("hello world")`
- `ChangeColor` 变体包裹了三个 `u8`，打印为 `ChangeColor(200, 255, 255)`
- `Quit` 不携带数据，只打印变体名

---

## 三、`match` 模式匹配——处理枚举变体

**知识点：使用 `match` 对枚举进行模式匹配，根据不同变体执行不同逻辑，可以解构出变体携带的数据**

`match` 是 Rust 中处理枚举的核心方式，它**必须穷举所有变体**（编译器强制检查）。通过模式匹配，可以解构出变体中携带的数据并使用。

```rust
struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // 由红、绿、蓝组成的 RGB 颜色
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    // 使用 match 对枚举进行模式匹配，处理不同变体
    fn process(&mut self, message: Message) {
        match message {
            // 解构结构体式变体，提取 width 和 height
            Message::Resize { width, height } => self.resize(width, height),
            // 解构元组式变体，提取 Point
            Message::Move(point) => self.move_position(point),
            // 解构元组式变体，提取 String
            Message::Echo(string) => self.echo(string),
            // 解构元组式变体，提取 red、green、blue
            Message::ChangeColor(red, green, blue) => self.change_color(red, green, blue),
            // 无数据变体，直接调用方法
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
```

**测试结果：**
```
running 1 test
test tests::test_match_message_call ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

解析：
- `Message::Resize { width, height }` 解构出 `width` 和 `height`，传给 `self.resize()`
- `Message::Move(point)` 解构出 `Point`，传给 `self.move_position()`
- `Message::Echo(string)` 解构出 `String`，传给 `self.echo()`
- `Message::ChangeColor(red, green, blue)` 解构出三个 `u8`，传给 `self.change_color()`
- `Message::Quit` 无数据，直接调用 `self.quit()`

> **关键**：`match` 必须穷举所有变体，否则编译报错。如果不需要处理某个变体，可以用 `_` 通配符（类似 switch 中的 default），但显式列出所有变体更安全。

---

## 知识点总览

| 章节 | 核心知识点 |
|------|-----------|
| 一 | 无数据枚举变体：仅作为命名标签，用 `枚举名::变体名` 访问 |
| 二 | 带数据枚举变体：结构体式 `{ field: T }`、元组式 `(T)` 、无数据；`#[derive(Debug)]` 打印枚举值 |
| 三 | `match` 模式匹配：必须穷举所有变体，解构出变体携带的数据，根据变体执行不同逻辑 |
