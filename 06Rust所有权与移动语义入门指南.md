# Rust 所有权与移动语义（Move Semantics）入门指南

---

## 一、所有权转移（Move）——变量被移动后不可再访问

**知识点：当变量的值被传递给函数时，所有权（ownership）会被转移（move），原变量将不可再使用**

Rust 中每个值都有唯一一个"所有者"（owner）。当把变量传给函数时，所有权会被转移（move）到函数内部，原变量就不再拥有这个值，编译器会阻止你再次访问它。这是 Rust 内存安全的核心机制——不需要垃圾回收器，也不需要手动释放内存，通过所有权规则在编译期就能保证内存安全。

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // 将传入的 vec 声明为可变变量，才能调用 push
    // 这利用了变量遮蔽（shadowing）：新的 mut vec 遮蔽了旧的不可变参数 vec
    // 更推荐的做法是直接在参数上声明 mut，见第三节
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // vec0 不能再被访问，因为它的所有权已经移动到 fill_vec 中了
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
```

**测试结果：**
```
running 1 test
test tests::move_semantics1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

解析：`vec0` 的所有权通过 `fill_vec(vec0)` 被移动到函数内部，之后 `vec0` 不再可用。函数修改向量后将其返回，所有权转移到 `vec1`。

---

## 二、克隆（Clone）——深拷贝保留原变量的访问权

**知识点：使用 `.clone()` 可以深拷贝一个值，传给函数的是克隆副本而非原始值，原变量仍然可用**

当你需要同时保留原变量和传给函数的值时，可以在传递前调用 `.clone()` 进行深拷贝。克隆副本拥有独立的所有权，原变量不受影响。

> **何时使用 clone？** 当你需要在不同地方同时使用同一个值时。clone 会复制整个数据，有性能开销，所以不要滥用。

```rust
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // 利用变量遮蔽（shadowing）将不可变参数重新绑定为可变变量
    // 更推荐的做法是直接在参数上声明 mut，见第三节
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // 克隆 vec0，这样传入的是克隆副本而非 vec0 本身
        let vec1 = fill_vec(vec0.clone());

        // 原变量 vec0 仍然可用
        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
```

**测试结果：**
```
running 1 test
test tests::move_semantics2 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

解析：`vec0.clone()` 创建了深拷贝副本传给 `fill_vec`，原变量 `vec0` 的所有权未被移动，所以仍然可以访问。

---

## 三、函数参数直接声明为 `mut`——更简洁的方式

**知识点：函数参数可以直接用 `mut` 声明为可变，省去在函数体内再 `let mut` 的步骤**

前面的示例中，函数内部用 `let mut vec = vec;` 重新绑定变量使其可变。其实可以直接在参数列表中加 `mut`，这样更简洁：

```rust
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    // 参数直接声明为 mut，无需在函数体内再 let mut
    vec.push(88);

    vec
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
```

**测试结果：**
```
running 1 test
test tests::move_semantics3 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

解析：`fn fill_vec(mut vec: Vec<i32>)` 直接在参数上声明 `mut`，函数内可直接 `vec.push(88)`，省去 `let mut vec = vec;` 这一步。

---

## 四、引用基础——不可变引用（`&T`）与可变引用（`&mut T`）

**知识点：引用是借用数据的方式，不转移所有权；不可变引用只读，可变引用可读写**

### 4.1 什么是引用？

引用（reference）让你可以**借用**一个值而不获取其所有权。创建引用使用 `&` 符号，就像现实生活中"借书"一样——你只是借来看看，书还是原主人的。

### 4.2 不可变引用 `&T`

不可变引用允许你**读取**数据，但不能修改。就像借了一本图书馆的书，只能看，不能在上面写字。

```rust
fn main() {
    let s = String::from("hello");

    // 创建不可变引用，可以同时存在多个
    let r1 = &s;
    let r2 = &s;

    // r1 和 r2 都可以读取 s 的值
    println!("r1: {r1}, r2: {r2}");
}
```

**执行结果：**
```
r1: hello, r2: hello
```

### 4.3 可变引用 `&mut T`

可变引用允许你**读取并修改**数据。但为了防止数据竞争，Rust 规定：**同一时间只能有一个可变引用**指向同一数据。

```rust
fn main() {
    let mut s = String::from("hello");

    // 创建可变引用，同一时间只能有一个
    let r = &mut s;

    // 通过可变引用修改数据
    r.push_str(", world");

    println!("{r}");
}
```

**执行结果：**
```
hello, world
```

### 4.4 借用规则总结

| 规则 | 说明 |
|------|------|
| 多个 `&T` | 可以同时存在多个不可变引用，只读不冲突 |
| 一个 `&mut T` | 同一时间只能有一个可变引用，防止数据竞争 |
| `&T` 和 `&mut T` 不能共存 | 不能在有不可变引用的同时创建可变引用，反之亦然 |
| 引用必须始终有效 | 不能创建指向已销毁数据的引用（悬垂引用），Rust 编译器会阻止 |

> **为什么要有这些规则？** Rust 在编译期就能防止数据竞争（data race）——两个指针同时访问同一数据，其中至少一个在写入，且没有同步机制。这是大多数语言只能在运行时检测的问题，而 Rust 在编译期就杜绝了。

---

## 五、可变引用（`&mut`）与借用规则——NLL 详解

**知识点：同一时间只能有一个可变引用（`&mut`）指向同一数据，但可变引用不再使用后可以创建新的可变引用**

Rust 的借用规则（Borrowing Rules）是所有权系统的重要组成部分：

1. **同一时间，要么有多个不可变引用（`&T`），要么只有一个可变引用（`&mut T`）**，两者不能同时存在
2. **引用的作用域从创建到最后一次使用为止**（Non-Lexical Lifetimes，NLL）

这意味着：如果可变引用已经不再被使用，就可以创建新的可变引用。本例中 `y` 在 `y.push(42)` 之后不再使用，所以可以创建新的可变引用 `z`。

```rust
fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        // y 在这里被使用
        y.push(42);
        // 可变引用 y 此后不再使用，因此可以创建新的可变引用
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
```

**测试结果：**
```
running 1 test
test tests::move_semantics4 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

解析：`y.push(42)` 之后 `y` 不再使用，NLL 规则允许此时创建新的可变引用 `z`。如果 `y` 在 `z` 创建后仍被使用，编译器会报错——因为不能同时存在两个可变引用。

---

## 六、所有权转移 vs 借用——什么时候用 `&` 什么时候不用

**知识点：函数需要修改并"消费"数据时用所有权转移（不加 `&`），只读取数据时用借用（加 `&`）**

这是所有权系统中最实用的规则：

| 场景 | 参数类型 | 调用方式 | 说明 |
|------|---------|---------|------|
| 只读取，不修改 | `&T`（不可变引用） | `func(&data)` | 借用数据，原变量仍可用 |
| 读取并修改，但不消费 | `&mut T`（可变引用） | `func(&mut data)` | 借用并修改，原变量仍可用 |
| 完全消费数据 | `T`（所有权转移） | `func(data)` | 数据被移动，原变量不可再用 |

```rust
#![allow(clippy::ptr_arg)]

// 借用而非获取所有权
// 目前建议使用 &str 而非 &String，但现阶段这样写已足够
fn get_char(data: &String) -> char {
    // data.chars(): 将字符串拆成字符迭代器
    // .last(): 获取最后一个字符，返回 Option<char>（可能为 None）
    // .unwrap(): 从 Option 中取出值，如果为 None 则 panic
    data.chars().last().unwrap()
}

// 获取所有权而非借用
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    // 只读取，借用 data
    get_char(&data);

    // 需要消费 data，转移所有权
    string_uppercase(data);
}
```

**执行结果：**
```
RUST IS GREAT!
```

解析：
- `get_char(&data)` — 传入 `&data`（不可变引用），只读取字符，`data` 之后仍可用
- `string_uppercase(data)` — 直接传入 `data`（所有权转移），函数可以修改并消费数据。调用后 `data` 不可再使用
- `string_uppercase` 的参数用 `mut data: String` 而不是 `&mut String`，因为 `data.to_uppercase()` 返回新的 `String`，需要所有权才能赋值

---

## 知识点总览

| 章节 | 核心知识点 |
|------|-----------|
| 一 | 所有权转移（Move）：变量传给函数后所有权被移动，原变量不可再访问 |
| 二 | 克隆（Clone）：`.clone()` 深拷贝数据，传副本而非原值，原变量仍可用 |
| 三 | `mut` 参数：函数参数直接声明 `mut` 更简洁，省去 `let mut` 重绑定 |
| 四 | 引用基础：不可变引用 `&T` 只读，可变引用 `&mut T` 可读写，借用规则总结 |
| 五 | 可变引用（`&mut`）：同一时间只能有一个可变引用，NLL 规则允许引用不再使用后创建新的 |
| 六 | 借用 vs 转移：读取用 `&`（借用），修改并消费用所有权转移（不加 `&`） |
