macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 修复宏调用语法：需要使用感叹号
    my_macro!();
}
