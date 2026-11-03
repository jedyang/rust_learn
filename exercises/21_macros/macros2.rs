// 将宏定义移到 main 函数之前，使其在使用前可见
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
