---
theme: default
themeName: "默认主题"
title: "05Rust向量Vec入门指南"
---

# Rust 向量（Vec）入门指南

---

## 一、创建 Vector——`vec!` 宏与 `Vec::new()`

**知识点：Vector 是可变长度的动态数组，使用 `vec!` 宏快速创建，或用 `Vec::new()` 创建空向量后逐步添加元素**

Vector（`Vec<T>`）是 Rust 中最常用的集合类型，与数组的关键区别：

| 特性 | 数组 `[T; N]` | Vector `Vec<T>` |
|------|--------------|-----------------|
| 长度 | 固定（编译期确定） | 可变（运行时动态增减） |
| 内存 | 栈上分配 | 堆上分配 |
| 创建 | `[10, 20, 30, 40]` | `vec![10, 20, 30, 40]` 或 `Vec::new()` |
| 添加元素 | 不支持 | `push()` 方法 |

创建 Vector 的两种方式：
1. **`vec!` 宏**：直接列出初始值，如 `vec![10, 20, 30, 40]`，最常用
2. **`Vec::new()`**：创建空向量，之后用 `.push()` 逐个添加元素

> 注意：`Vec::new()` 创建空向量时，如果后续没有 `push` 操作来推断类型，则需要显式标注类型，如 `let v: Vec<i32> = Vec::new();`。

```rust
fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 数组

    // 使用 vec! 宏创建向量
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
```

**测试结果：**
```
running 1 test
test tests::test_array_and_vec_similarity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

解析：数组 `[10, 20, 30, 40]` 和向量 `vec![10, 20, 30, 40]` 包含相同的元素值。`assert_eq!(a, *v)` 中 `*v` 解引用向量使其可以与数组比较——这说明相同内容的数组和向量在值比较上是等价的。

---

## 二、遍历 Vector——`for` 循环的三种方式

**知识点：遍历 `Vec<T>` 有三种常见方式——借用 `&vec`（只读）、可变借用 `&mut vec`（可修改）、消费 `vec`（转移所有权）**

遍历 Vector 时，根据是否需要修改元素、是否还要继续使用原向量，选择不同的遍历方式：

| 方式 | 语法 | 能否修改元素 | 是否获取所有权 | 遍历后原向量 |
|------|------|------------|--------------|-------------|
| 借用遍历 | `for v in &vec` | 否（只读 `&T`） | 否 | 仍可用 |
| 可变借用遍历 | `for v in &mut vec` | 是（`&mut T`） | 否 | 仍可用（已被修改） |
| 消费遍历 | `for v in vec` | — | 是（值 `T`） | 不可再用 |

> 提示：`for v in &vec` 等价于 `for v in vec.iter()`，`for v in vec` 等价于 `for v in vec.into_iter()`。日常只读遍历推荐直接写 `&vec`，更简洁。

```rust
fn sum_vec(vec: &[i32]) -> i32 {
    // 借用遍历：只读取元素，不获取所有权
    let mut sum = 0;
    for v in vec {
        sum += v;
    }
    sum
}

fn double_vec(mut vec: Vec<i32>) -> Vec<i32> {
    // 可变借用遍历：原地修改每个元素
    for v in &mut vec {
        *v *= 2;
    }
    vec
}

fn consume_vec(vec: Vec<i32>) -> i32 {
    // 消费遍历：获取每个元素的所有权，vec 之后不可再用
    let mut last = 0;
    for v in vec {
        last = v;
    }
    last
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vec() {
        assert_eq!(sum_vec(&[10, 20, 30, 40]), 100);
    }

    #[test]
    fn test_double_vec() {
        assert_eq!(double_vec(vec![10, 20, 30, 40]), [20, 40, 60, 80]);
    }

    #[test]
    fn test_consume_vec() {
        assert_eq!(consume_vec(vec![10, 20, 30, 40]), 40);
    }
}
```

**测试结果：**
```
running 3 tests
test tests::test_consume_vec ... ok
test tests::test_double_vec ... ok
test tests::test_sum_vec ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 三、从切片转换到 Vec——`push` 与 `iter().map().collect()` 两种方式

**知识点：将切片 `&[T]` 转换为 `Vec<T>` 有两种常见模式——循环 + `push`（命令式）和 `iter().map().collect()`（函数式）**

### 3.1 命令式方式：循环 + `push`

用 `for element in input` 遍历切片的每个元素，对每个元素做变换后用 `.push()` 逐个添加到结果向量中。这是最直观的方式。

```rust
fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(2 * element);
    }

    output
}
```

- `Vec::new()` 创建空向量
- `.push(value)` 在向量末尾添加一个元素
- 函数末尾的 `output` 不加分号，作为隐式返回值

### 3.2 函数式方式：`iter().map().collect()`

Rust 提供了更简洁的函数式风格来处理集合变换：

- `.iter()` — 创建迭代器，逐个访问元素
- `.map(|element| ...)` — 对每个元素应用变换（闭包/lambda）
- `.collect()` — 将迭代结果收集为新的 `Vec`

```rust
fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 使用 map 对切片中的每个元素加 1，然后收集为新向量
    // 如果输入是 [1, 2, 3]，输出就是 [2, 3, 4]
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // 后续会深入学习迭代器，目前只需这样写即可！
    // 进阶说明：此方法更高效，因为它会自动预分配足够的容量。
    // 在 vec_loop 中可以用 Vec::with_capacity(input.len()) 替代 Vec::new() 来手动预分配。
    input.iter().map(|element| 2 * element).collect()
}
```

> **进阶提示**：`iter().map().collect()` 方法更高效，因为它会自动预分配足够的容量。在命令式方式中，可以用 `Vec::with_capacity(input.len())` 替代 `Vec::new()` 来手动预分配容量，避免多次扩容。

### 完整代码

```rust
fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(2 * element);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 使用 map 对切片中的每个元素加 1，然后收集为新向量
    // 如果输入是 [1, 2, 3]，输出就是 [2, 3, 4]
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // 后续会深入学习迭代器，目前只需这样写即可！
    // 进阶说明：此方法更高效，因为它会自动预分配足够的容量。
    // 在 vec_loop 中可以用 Vec::with_capacity(input.len()) 替代 Vec::new() 来手动预分配。
    input.iter().map(|element| 2 * element).collect()
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
```

**测试结果：**
```
running 3 tests
test tests::test_vec_map ... ok
test tests::test_vec_map_example ... ok
test tests::test_vec_loop ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 知识点总览

| 章节 | 核心知识点 |
|------|-----------|
| 一 | `vec!` 宏快速创建向量，`Vec::new()` 创建空向量，`push()` 添加元素；向量与数组的对比 |
| 二 | 遍历 `Vec<T>` 的三种方式：借用 `&vec`（只读）、可变借用 `&mut vec`（可修改）、消费 `vec`（转移所有权） |
| 三 | 从切片 `&[T]` 转换为 `Vec<T>`：命令式 `for` 循环 + `push`，函数式 `iter().map().collect()`；闭包语法 `|element| ...` |
