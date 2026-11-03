---
theme: default
themeName: "默认主题"
title: "07Rust结构体入门指南"
---

# Rust 结构体（Structs）入门指南

---

## 什么是结构体？

**结构体（Struct）是 Rust 中自定义数据类型的核心方式，用于将多个相关的值组合成一个有意义的整体。**

现实生活中，很多数据是相互关联的。比如一个"包裹"包含发送国、接收国、重量等多个信息；一个"颜色"包含红、绿、蓝三个分量。如果把这些信息分散存储为独立变量，既不方便也容易出错。结构体就像一个"容器"，把这些相关数据打包在一起，用一个名称来代表这个整体。

### 结构体的作用

1. **组织数据**：将多个相关字段组合为一个整体，比散落的变量更清晰、更安全
2. **类型安全**：给数据组合赋予一个明确的类型名，编译器可以检查类型匹配
3. **封装逻辑**：在 `impl` 块中为结构体定义方法，数据和行为绑定在一起
4. **代码复用**：通过结构体更新语法 `..` 快速创建相似实例

### 与其他语言的对比

| 语言 | 类似概念 | 示例 |
|------|---------|------|
| C/C++ | struct / class | `struct Point { int x; int y; }` |
| Java | class | `class Point { int x; int y; }` |
| Python | class / dataclass | `@dataclass class Point: x: int; y: int` |
| Go | struct | `type Point struct { X, Y int }` |

Rust 的结构体与 C 的 struct 最接近，但 Rust 还能在 `impl` 块中为结构体添加方法（类似 class 的方法），不过 Rust **没有继承**，这是与其他语言 class 的关键区别。

关于`impl` 块中下面会有详细解释，你先不要急。

---

## 一、三种结构体——普通结构体、元组结构体、单元结构体

**知识点：Rust 有三种结构体形式：普通结构体（命名字段）、元组结构体（匿名字段）、单元结构体（无字段）**

### 1.1 普通结构体（Named-field Struct）

这是最常见的结构体形式，每个字段都有名称。通过 `结构体名 { 字段名: 值 }` 创建，通过 `实例.字段名` 访问。

```rust
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}
```

### 1.2 元组结构体（Tuple Struct）

这种结构体的字段没有名称，只有类型。看起来像元组但带有结构体名。通过 `结构体名(值1, 值2, ...)` 创建，通过 `实例.0`、`实例.1` 等索引访问。

```rust
struct ColorTupleStruct(u8, u8, u8);
```

> 元组结构体适合字段含义明确、不需要命名的情况，如坐标 `(x, y)` 用 `Point(i32, i32)` 表示。

### 1.3 单元结构体（Unit Struct）

这是没有任何字段的结构体。本身也不存储数据，通常用于实现某个 trait（特征）或作为标记类型。

```rust
#[derive(Debug)]
struct UnitStruct;
```
tips:
> `#[derive(Debug)]` 是派生宏，让结构体自动实现 `Debug` trait，这样可以用 `{:?}` 格式化打印。

### 完整代码

```rust
// 普通结构体——每个字段有名称
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// 元组结构体——字段无名称，只有类型
struct ColorTupleStruct(u8, u8, u8);

// 单元结构体——无字段，常作为标记类型
#[derive(Debug)]
struct UnitStruct;

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // 普通结构体：用字段名创建和访问
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // 元组结构体：用索引创建和访问
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // 单元结构体：无字段，直接创建
        let unit_struct = UnitStruct;
        // 需要 #[derive(Debug)] 才能用 {:?} 格式化打印
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
```

---

## 二、结构体更新语法（Struct Update Syntax）

**知识点：使用 `..旧实例` 语法可以从已有实例中批量复制字段，只需显式写出不同的字段**

当你需要创建一个与现有实例大部分字段相同、只有少数字段不同的新实例时，可以使用结构体更新语法 `..旧实例`。它会从旧实例中自动填充那些没有显式指定的字段。

关键要点：
- `..` 必须放在结构体初始化的**最后**
- `..` 会**移动**旧实例中被复制的字段的所有权（如果字段类型实现了 `Copy` trait 则是复制而非移动）
- 使用 `..` 后，旧实例中所有权被移动的字段不再可用

```rust
#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // 只修改 name 和 count，其余字段从 order_template 复制
        let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            // 结构体更新语法：从 order_template 复制剩余字段
            ..order_template
        };

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
```

**测试结果：**
```
running 1 test
test tests::your_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

解析：`Order { name: ..., count: 1, ..order_template }` 中，`name` 和 `count` 显式指定了新值，其余 5 个字段（`year`、`made_by_phone` 等）通过 `..order_template` 自动从模板复制。

> 注意：`name` 是显式赋值 `String::from("Hacker in Rust")`，**不从 `order_template` 移动**。`..order_template` 只涉及未被显式指定的 5 个字段，它们都是 `u32` 或 `bool`（实现了 `Copy` trait），所以是**复制而非移动**。因此 `order_template` 完全没有被消耗，测试中仍然可以访问 `order_template.year` 等字段。

> 如果 `..` 涉及的字段中包含非 `Copy` 类型（如 `String`），那么该字段会被移动，之后 `order_template` 中对应的字段就不可再访问了，整个 `order_template` 也会被视为部分移动。

---

## 三、结构体方法（Methods）——`impl` 块与 `&self`

**知识点：结构体可以在 `impl` 块中定义方法，使用 `&self` 访问实例自身的数据**

### 3.1 `impl` 块-方法

Rust 使用 `impl`（implementation）块为结构体添加方法。方法与函数类似，但第一个参数总是 `self`（或 `&self`、`&mut self`），代表调用方法的结构体实例本身。

| 参数形式 | 含义 | 调用方式 |
|---------|------|---------|
| `&self` | 不可变借用，只读取 | `instance.method()` |
| `&mut self` | 可变借用，可修改 | `instance.method()`（实例需 `mut`） |
| `self` | 获取所有权，消费实例 | `instance.method()`（调用后实例不可用） |

### 3.2 关联函数（Associated Function）

不以 `self` 为第一个参数的函数称为**关联函数**（类似其他语言的"静态方法"），通常用于构造实例。调用方式是 `结构体名::函数名()`，如 `Package::new()`。

### 3.3 完整代码

```rust
#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    // 关联函数：构造新实例，不以 self 为参数
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        // Self 是当前结构体类型的别名，这里等于 Package
        if weight_in_grams < 10 {
            // 这不是 Rust 推荐的错误处理方式，后续会学习 error handling
            panic!("Can't ship a package with weight below 10 grams");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // 方法：判断是否国际包裹，不可变借用 self（只读取）
    fn is_international(&self) -> bool {
        // 发送国和接收国不同即为国际包裹
        self.sender_country != self.recipient_country
    }

    // 方法：计算运费，不可变借用 self（只读取）
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // 运费 = 重量 × 每克单价
        self.weight_in_grams * cents_per_gram
    }
}

fn main() {
    // 你可以在这里进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        // #[should_panic] 表示测试预期会 panic
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        // 重量 5 克小于 10 克，应该 panic
        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        // 发送国和接收国不同，是国际包裹
        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        // 发送国和接收国相同，不是国际包裹
        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        // 1500 × 3 = 4500
        assert_eq!(package.get_fees(cents_per_gram), 4500);
        // 1500 × 6 = 9000
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
```

**测试结果：**
```
running 4 tests
test tests::calculate_transport_fees ... ok
test tests::create_international_package ... ok
test tests::create_local_package ... ok
test tests::fail_creating_weightless_package - should panic ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 知识点总览

| 章节 | 核心知识点 |
|------|-----------|
| 一 | 三种结构体：普通结构体（命名字段）、元组结构体（索引访问）、单元结构体（无字段）；`#[derive(Debug)]` 派生宏 |
| 二 | 结构体更新语法 `..旧实例`：批量复制字段，只需显式写出不同的字段；`..` 会移动所有权 |
| 三 | `impl` 块定义方法：`&self` 读取、`&mut self` 修改、`self` 消费；关联函数用 `Self` 和 `::` 调用 |
