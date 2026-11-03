---
theme: default
themeName: "默认主题"
title: "04Rust基本类型入门指南"
---

# Rust 基本类型（Primitive Types）入门指南

---

## 一、布尔类型 `bool`

**知识点：Rust 的布尔类型只有两个值 `true` 和 `false`，可以用 `!` 取反**

布尔类型是最简单的基本类型，常用于条件判断。Rust 中布尔类型的关键字是 `bool`，值只有 `true` 或 `false`。使用 `!`（逻辑非）运算符可以对布尔值取反。

```rust
fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
```

**执行结果：**
```
Good morning!
```

解析：`is_morning` 为 `true`，打印 "Good morning!"；`!is_morning` 为 `false`，所以不打印 "Good evening!"。

---

## 二、字符类型 `char`

**知识点：Rust 的 `char` 是 4 字节 Unicode 字符，支持中文、emoji 等，并提供丰富的判断方法**

Rust 的 `char` 类型与许多语言的"字符"不同：
- 它是**4 字节**的 Unicode 标量值，而不是 1 字节的 ASCII
- 可以表示任何 Unicode 字符，包括中文、日文、emoji 等
- 使用**单引号**包裹，如 `'中'`、`'🦀'`（字符串用双引号）

`char` 提供了很多实用方法，如 `is_alphabetic()`（是否字母）、`is_numeric()`（是否数字）等。

```rust
fn main() {
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // Example with an emoji.
    let your_character = '🦀';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
```

**执行结果：**
```
Alphabetical!
Neither alphabetic nor numeric!
```

解析：`'C'` 是字母，打印 "Alphabetical!"；emoji `'🦀'` 既不是字母也不是数字，打印 "Neither alphabetic nor numeric!"。

---

## 三、数组（Array）——固定长度、相同类型

**知识点：数组是固定长度、相同类型元素的集合，可以用 `[值; 重复次数]` 语法快速创建重复元素的数组**

Rust 数组的两个核心特征：
- **固定长度**：数组一旦创建，长度不可改变
- **相同类型**：所有元素必须是同一类型

创建数组有两种方式：
1. `[1, 2, 3, 4, 5]` — 列出每个元素
2. `[42; 100]` — 创建 100 个值为 42 的元素（重复语法），适合创建大型数组

数组的 `.len()` 方法返回其长度。

```rust
fn main() {
    // An array with 100 elements of the value 42.
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
```

**执行结果：**
```
Wow, that's a big array!
```

---

## 四、数组切片（Slice）—— `..` 与 `..=` 范围语法

**知识点：可以从数组中取出切片（Slice），使用 `..`（排除上界）和 `..=`（包含上界）两种范围语法**

切片是对数组（或其他集合）中一段连续元素的引用，不拥有所有权。创建切片的语法是 `&a[起始..结束]`。

两种范围语法：

| 语法 | 含义 | 示例 `&a[1..4]` | 示例 `&a[1..=3]` |
|------|------|-----------------|-----------------|
| `起始..结束` | 排除上界（不含结束索引） | 包含索引 1、2、3 | — |
| `起始..=结束` | 包含上界（含结束索引） | — | 包含索引 1、2、3 |

对数组 `a = [1, 2, 3, 4, 5]`：
- `&a[1..4]` → 索引 1、2、3 → 值 `[2, 3, 4]`
- `&a[1..=3]` → 索引 1、2、3 → 值 `[2, 3, 4]`

两种写法在本例中结果相同，但含义不同：`..4` 排除索引 4，`..=3` 包含索引 3。

```rust
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4  <- indices
        //          -------
        //             |
        //             +--- slice

        // Note that the upper index 4 is excluded.
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // The upper index can be included by using the syntax `..=` (with `=` sign)
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
```

**测试结果：**
```
running 1 test
test tests::slice_out_of_array ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 五、元组（Tuple）——解构赋值

**知识点：元组可以包含不同类型的元素，通过解构赋值（destructuring）可以方便地取出各元素**

元组与数组的关键区别：
- **数组**：固定长度，所有元素**相同类型**
- **元组**：固定长度，元素可以是**不同类型**

解构赋值是提取元组元素的优雅方式，语法为 `let (变量1, 变量2, ...) = 元组;`，一次性将元组的每个元素绑定到对应变量。

```rust
fn main() {
    let cat = ("Furry McFurson", 3.5);

    // Destructuring the tuple.
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
```

**执行结果：**
```
Furry McFurson is 3.5 years old
```

---

## 六、元组（Tuple）——索引访问

**知识点：元组元素可以通过 `.索引` 语法访问，索引从 0 开始**

除了解构赋值，元组还可以通过 `.0`、`.1`、`.2` 等索引方式访问单个元素。注意：**元组用点号加数字索引**，而数组用方括号 `[索引]`。

```rust
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Tuple indexing syntax.
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
```

**测试结果：**
```
running 1 test
test tests::indexing_tuple ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

解析：`numbers.1` 访问元组的第 2 个元素（索引为 1），值为 `2`。

---

## 知识点总览

| 章节 | 核心知识点 |
|------|-----------|
| 一 | `bool` 类型只有 `true`/`false`，`!` 取反 |
| 二 | `char` 是 4 字节 Unicode 字符，单引号包裹，提供 `is_alphabetic()` 等方法 |
| 三 | 数组：固定长度、相同类型，`[值; 重复次数]` 创建重复元素数组 |
| 四 | 切片：`..` 排除上界，`..=` 包含上界，`&a[起始..结束]` 取子序列 |
| 五 | 元组解构：`let (a, b) = tuple` 一次性取出各元素，元素可以不同类型 |
| 六 | 元组索引：`tuple.0`、`tuple.1` 用点号加数字访问单个元素 |
