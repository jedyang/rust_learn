# Rust 模块系统知识点总结

Rust 的模块系统用于组织和控制代码的可见性。核心概念包括：`mod` 定义模块、`pub` 控制可见性、`use` 引入路径、`as` 重命名。

---

## 1. 模块可见性与 `pub` 关键字

> 文件：`modules1.rs`

### 知识点
- 模块内的项（函数、结构体、常量等）默认是**私有**的，只能在同一模块内访问。
- 用 `pub` 关键字将项标记为**公开**，模块外部才能访问。
- 模块可以有意将内部实现细节保持私有，只公开必要的接口（封装原则）。

### 代码

```rust
mod sausage_factory {
    // 这个函数保持私有，模块外部无法调用
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 加上 pub 使其可被模块外部访问
    pub fn make_sausage() {
        get_secret_recipe(); // 模块内部可以调用私有函数
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
```

### 执行结果

```
sausage!
```

---

## 2. `use` 与 `as` 引入并重命名模块路径

> 文件：`modules2.rs`

### 知识点
- `use` 可以将模块路径引入当前作用域，避免每次写完整路径。
- `as` 可以为引入的项指定别名，解决命名冲突或简化名称。
- `pub use` 可以将内部项**重导出**，使外部模块可以通过新名称访问。
- `self` 在 `use` 语句中指代当前模块。

### 代码

```rust
mod delicious_snacks {
    // 使用 pub use 将内部常量重导出为更短的别名
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
```

### 执行结果

```
favorite snacks: Pear and Cucumber
```

---

## 3. 从标准库引入模块路径

> 文件：`modules3.rs`

### 知识点
- `use` 不仅用于自定义模块，也用于引入标准库（`std`）或其他外部 crate 的路径。
- 可以用花括号 `{}` 在一行中引入多个项：`use path::{Item1, Item2}`。
- 这种写法比分别写两行 `use` 更简洁，是 Rust 的惯用风格。

### 代码

```rust
// 一行引入标准库中的两个项
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
```

### 执行结果

```
1970-01-01 00:00:00 UTC was 1779345600 seconds ago!
```

（具体秒数取决于运行时间）

---

## 总结

| 知识点 | 要点 |
|---|---|
| 模块定义 | 用 `mod name { ... }` 定义模块，模块可以嵌套 |
| 默认私有 | 模块内的项默认私有，只有同模块内可访问 |
| `pub` 关键字 | 标记项为公开，外部模块才能访问 |
| `use` 引入路径 | 将长路径引入作用域，简化代码 |
| `as` 重命名 | 为引入的项指定别名 |
| `pub use` 重导出 | 将内部项以新名称暴露给外部 |
| `use path::{A, B}` | 花括号语法一行引入多个项 |
| 标准库引入 | `use std::...` 引入标准库模块 |
