// 已修复：为参数 `num` 添加类型标注 `i32`。
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
