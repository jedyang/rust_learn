#![allow(clippy::ptr_arg)]

// 已修复：通过添加或移除引用（字符 `&`）来修复编译错误。

// 不应该获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应该获取所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
