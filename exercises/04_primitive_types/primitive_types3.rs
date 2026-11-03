fn main() {
    // 已修复：创建一个包含 100 个元素的数组，使用 `[值; 数量]` 语法。
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
