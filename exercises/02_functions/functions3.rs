fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // 已修复：函数参数类型为 `u8`，传入字面量 3 即可（默认推断为 i32 需转型）。
    call_me(3);
}
