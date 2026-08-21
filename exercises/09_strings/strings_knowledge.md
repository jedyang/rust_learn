# Rust 字符串知识点总结

Rust 有两种字符串类型：**字符串切片 `&str`** 和 **堆分配字符串 `String`**。

---

## 1. `&str` 与 `String` 的区别与转换

> 文件：`strings1.rs`

### 知识点
- 字符串字面量（如 `"blue"`）的类型是 `&str`，它是不可变的引用，通常指向程序的只读数据段。
- `String` 是堆分配的、可增长的字符串类型。
- 当函数签名要求返回 `String` 时，不能直接返回 `&str`，需要用 `.to_string()` 或 `String::from()` 进行转换。

### 代码

```rust
// 修复编译错误，不能修改函数签名
fn current_favorite_color() -> String {
    // "blue" 是 &str 类型，不能直接返回，需要转为 String
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
```

### 执行结果

```
My current favorite color is blue
```

---

## 2. `String` 到 `&str` 的自动解引用（Deref）

> 文件：`strings2.rs`

### 知识点
- `String` 实现了 `Deref<Target = str>`，因此在需要 `&str` 的地方可以传 `&String`。
- 但 Rust **不会自动借用**，必须显式加 `&`：`&word` 将 `String` 借用为 `&str`。
- 如果直接传 `word`（`String`），编译器会报类型不匹配错误。

### 代码

```rust
// 不要修改这个函数
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // 不要修改这行

    // word 是 String 类型，is_a_color_word 需要 &str
    // 必须显式借用：&word 将 String 借用为 &str
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
```

### 执行结果

```
That is a color word I know!
```

---

## 3. 常用字符串操作方法

> 文件：`strings3.rs`

### 知识点
- `trim()`：去除字符串两端空白，返回 `&str`。
- `format!` 宏：格式化拼接字符串，返回 `String`。
- `replace()`：替换子串，返回 `String`。

### 代码

```rust
// 去除字符串两端空白
fn trim_me(input: &str) -> &str {
    input.trim()
}

// 在字符串末尾追加 " world!"
fn compose_me(input: &str) -> String {
    format!("{input} world!")
}

// 将字符串中的 "cars" 替换为 "balloons"
fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

fn main() {
    // 你可以在这里实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
```

### 执行结果

```
running 3 tests
test tests::trim_a_string ... ok
test tests::compose_a_string ... ok
test tests::replace_a_string ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 4. 判断表达式返回 `&str` 还是 `String`

> 文件：`strings4.rs`

### 知识点

这是最综合的练习，需要判断每个表达式产生的是 `&str` 还是 `String`：

| 表达式 | 返回类型 | 说明 |
|---|---|---|
| `"blue"` | `&str` | 字符串字面量始终是 `&str` |
| `"red".to_string()` | `String` | `.to_string()` 将 `&str` 转为 `String` |
| `String::from("hi")` | `String` | 直接构造 `String` |
| `"rust is fun!".to_owned()` | `String` | `.to_owned()` 将 `&str` 转为 `String` |
| `"nice weather".into()` | 看上下文 | `.into()` 根据目标类型转换，传给 `string()` 则为 `String`，传给 `string_slice()` 则为 `&str` |
| `format!("Interpolation {}", "Station")` | `String` | `format!` 宏返回 `String` |
| `&String::from("abc")[0..1]` | `&str` | 对 String 切片得到 `&str`（注意是字节索引，不是字符索引） |
| `"  hello there ".trim()` | `&str` | `trim()` 返回 `&str` |
| `"Happy Monday!".replace("Mon", "Tues")` | `String` | `replace()` 返回 `String` |
| `"mY sHiFt KeY iS sTiCkY".to_lowercase()` | `String` | `to_lowercase()` 返回 `String` |

### 代码

```rust
fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    // 字符串字面量 -> &str
    string_slice("blue");

    // .to_string() -> String
    string("red".to_string());

    // String::from() -> String
    string(String::from("hi"));

    // .to_owned() -> String
    string("rust is fun!".to_owned());

    // .into() 根据目标类型推断，这里期望 String，所以转为 String
    string("nice weather".into());
    // .into() 也可以用于 &str 的上下文，此时不做转换
    #[allow(clippy::useless_conversion)]
    string_slice("nice weather".into());

    // format! 宏 -> String
    string(format!("Interpolation {}", "Station"));

    // 对 String 切片 -> &str（注意：这是字节索引，不是字符索引）
    string_slice(&String::from("abc")[0..1]);

    // .trim() -> &str
    string_slice("  hello there ".trim());

    // .replace() -> String
    string("Happy Monday!".replace("Mon", "Tues"));

    // .to_lowercase() -> String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
```

### 执行结果

```
blue
red
hi
rust is fun!
nice weather
nice weather
Interpolation Station
a
hello there
Happy Tuesday!
my shift key is sticky
```

---

## 总结

| 知识点 | 要点 |
|---|---|
| 两种字符串类型 | `&str` 是借用切片，`String` 是堆分配的拥有所有权类型 |
| `&str` → `String` | `.to_string()`、`String::from()`、`.to_owned()`、`.into()`、`format!` |
| `String` → `&str` | 显式借用 `&s`，或对 String 切片 `&s[..]` |
| 方法返回类型 | `trim()` → `&str`；`replace()`、`to_lowercase()` → `String` |
| 自动解引用 | `String` 实现了 `Deref<Target=str>`，但 Rust 不会自动借用，需手动加 `&` |
| 字符串切片索引 | 是**字节索引**而非字符索引，中文字符等多字节字符需小心 |
