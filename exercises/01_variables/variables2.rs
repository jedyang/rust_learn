fn main() {
    // 已修复：为变量 `x` 初始化一个整数值，Rust 会将其类型推断为 `i32`。
    let x = 10;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
