fn main() {
    let number = "T-H-R-E-E"; // 不要修改这一行
    println!("Spell a number: {number}");

    // 已修复：使用变量遮蔽（shadowing），用同名变量重新声明为整数类型。
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
