// Here are some more easy Clippy fixes so you can see its utility 📎

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Assume that you don't know the value of `my_option`.
    // In the case of `Some`, we want to print its value.
    // 使用 if let 安全地处理 Option，避免在 None 时 unwrap 导致 panic
    if let Some(value) = my_option {
        println!("{value}");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // 直接创建空 Vec，而不是先创建再 resize
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // 使用元组解构安全地交换两个变量的值
    (value_a, value_b) = (value_b, value_a);
    println!("value a: {value_a}; value b: {value_b}");
}
