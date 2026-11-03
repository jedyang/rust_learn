// 布尔类型（`bool`）

fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // 已修复：定义布尔变量 `is_evening`，值为 `is_morning` 的取反。
    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
