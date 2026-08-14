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

## 二、遍历 Vector 与 `push`、`iter().map().collect()` 两种操作方式

**知识点：遍历 Vector 有两种常见模式——循环 + `push`（命令式）和 `iter().map().collect()`（函数式）**

### 2.1 命令式方式：循环 + `push`

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

### 2.2 函数式方式：`iter().map().collect()`

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
| 二 | 命令式：`for` 循环 + `push`；函数式：`iter().map().collect()`；闭包语法 `|element| ...` |
