// 已修复：在函数体中返回 `num * num` 的结果。
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
