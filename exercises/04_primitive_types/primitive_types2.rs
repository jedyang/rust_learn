// Characters (`char`)

fn main() {
    // 注意使用的是 _单引号_，这与之前看到的双引号不同。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // 已修复：声明变量 `your_character`，赋值为一个字符（如数字 '1'）。
    let your_character = '1';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
