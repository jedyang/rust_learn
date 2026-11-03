// 在模块上添加 #[macro_use] 属性，使宏在模块外部也可用
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
