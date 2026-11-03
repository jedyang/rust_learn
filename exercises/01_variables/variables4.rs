// 已修复：Rust 中变量默认不可变，添加 `mut` 关键字使其可变。
fn main() {
    let mut x = 3;
    println!("Number {x}");

    x = 5; // 不要修改这一行
    println!("Number {x}");
}
