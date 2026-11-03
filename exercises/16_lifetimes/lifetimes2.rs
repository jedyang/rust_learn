// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 修复编译器错误：通过移动一行代码。
    // 将 `println!` 移入内部作用域中，确保 `string2` 在使用 `result` 时仍然存活。

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is '{result}'");
    }
}
