---
theme: default
themeName: "默认主题"
title: "03Rust条件判断if入门指南"
---

# Rust 条件判断（if）入门指南

---

## 一、`if` 表达式可以作为值使用

**知识点：Rust 中 `if` 是表达式而非语句，它会产生一个值，可以直接赋给变量或作为函数返回值**

与许多语言不同，Rust 的 `if` 是**表达式**（expression），而不是**语句**（statement）。这意味着 `if` 的每个分支都会产生一个值，整个 `if` 表达式的结果就是匹配分支的值。

关键规则：
- `if` 的每个分支必须返回**相同类型**的值，否则编译报错
- `if` 表达式不需要用 `return` 返回值，只需在分支末尾写一个不带分号的表达式
- 可以用 `if` 表达式直接赋值给变量，如 `let x = if condition { a } else { b };`

```rust
fn bigger(a: i32, b: i32) -> i32 {
    // if 表达式直接返回值，不需要 return 关键字
    if a > b { a } else { b }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
```

### 补充：Rust 单元测试的基本用法

上面的代码中包含了 Rust 单元测试的标准写法，下面拆解各个部分的作用：

**1. `#[cfg(test)]` —— 测试模块的"开关"**

`cfg(test)` 是条件编译标注，意思是：这段代码只在运行 `cargo test` 时才会被编译进程序。平时 `cargo build` 生成的正式程序中完全不包含测试代码，零开销。

**2. `mod tests` —— 测试模块**

习惯上把所有测试函数放在一个名为 `tests` 的模块中。模块内部可以定义只供测试使用的辅助函数，不会污染正式代码。

**3. `use super::*;` —— 引入被测试的代码**

`super` 指父模块（即外层的代码），`*` 表示全部引入。这样测试模块内就能直接调用 `bigger` 这样的外部函数。注意：如果要测试的函数是私有的（没有 `pub`），这种写法也能访问到。

**4. `#[test]` —— 标记测试函数**

每个测试函数都要加上 `#[test]` 属性，`cargo test` 才会识别并执行它。测试函数必须没有任何参数和返回值，函数名习惯上用能描述场景的小写英文，如 `ten_is_bigger_than_eight`。

**5. `assert_eq!` —— 断言宏**

断言是测试的核心，常用断言宏有三个：

| 宏 | 作用 | 示例 |
|----|------|------|
| `assert_eq!(a, b)` | 断言 `a == b`，不等则测试失败 | `assert_eq!(10, bigger(10, 8));` |
| `assert_ne!(a, b)` | 断言 `a != b`，相等则测试失败 | `assert_ne!(0, bigger(10, 8));` |
| `assert!(表达式)` | 断言表达式为 `true`，为 `false` 则失败 | `assert!(bigger(10, 8) > 0);` |

断言失败时会打印出两边的值（前提是类型实现了 `Debug` 特征），方便定位问题。还可以附加自定义提示信息：

```rust
assert_eq!(bigger(10, 8), 10, "bigger 应该返回较大的那个数");
```

**6. 运行测试的常用命令**

```bash
cargo test              # 运行所有测试
cargo test bigger       # 只运行名称中包含 "bigger" 的测试
cargo test -- --nocapture  # 显示测试中 println! 打印的内容
```

> 顺带一提：`#[cfg(test)]` 模块里可以写 `#[test]` 之外的非测试函数（如 `setup()` 辅助函数），它们不会被执行。如果某个测试暂时不想跑，可以标注 `#[ignore]`，之后用 `cargo test -- --ignored` 单独运行被忽略的测试。

**测试结果：**
```
running 3 tests
test tests::ten_is_bigger_than_eight ... ok
test tests::fortytwo_is_bigger_than_thirtytwo ... ok
test tests::equal_numbers ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 二、`if / else if / else` 多分支条件

**知识点：`if` 可以搭配 `else if` 和 `else` 处理多种条件，所有分支仍必须返回相同类型**

Rust 支持 `if / else if / else` 的多分支结构。与单分支 `if` 一样，所有分支返回的值必须是同一类型。`else` 分支是必须的（当 `if` 作为表达式使用时），因为编译器需要确保每种情况都有一个返回值，否则会报编译错误。

```rust
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else {
        "No thanks!"
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
```

**测试结果：**
```
running 3 tests
test tests::yummy_food ... ok
test tests::neutral_food ... ok
test tests::default_disliked_food ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 三、`if` 分支类型必须一致——编译器的严格检查

**知识点：`if` 各分支返回的类型必须完全一致，否则编译器报错；需要注意条件中不同类型之间的比较**

这是 `if` 表达式最容易出错的地方：**所有分支必须返回相同类型**。如果某个分支返回整数，另一个分支返回字符串，编译器会直接报错，不会自动转换。

本例还展示了另一个常见错误：将字符串类型的值与整数比较。`identifier` 通过 `if` 表达式赋值，其类型由各分支的值决定。如果分支返回整数（如 `1`、`2`、`3`），`identifier` 就是整数类型；如果分支返回字符串（如 `"Beach"`），`identifier` 就是字符串类型。后续再用 `identifier == 1` 去比较时，类型必须匹配。

> 在实际 Rust 开发中，这类"用整数标识不同状态"的模式通常应该用**枚举（enum）**来实现，枚举更安全、更清晰。但初学阶段先用整数理解即可。

```rust
fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        // Any unused identifier.
        4
    };

    // Instead of such an identifier, you would use an enum in Rust.
    // But we didn't get into enums yet.
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
```

**测试结果：**
```
running 4 tests
test tests::crab_lives_on_beach ... ok
test tests::gopher_lives_in_burrow ... ok
test tests::snake_lives_in_desert ... ok
test tests::unknown_animal ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 知识点总览

| 章节 | 核心知识点 |
|------|-----------|
| 一 | `if` 是表达式而非语句，可以直接产生值并赋给变量或作为返回值 |
| 二 | `if / else if / else` 多分支条件，`else` 分支在表达式场景中不可省略 |
| 三 | `if` 各分支必须返回相同类型；注意类型匹配，字符串不能和整数比较 |
