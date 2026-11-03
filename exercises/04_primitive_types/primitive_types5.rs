fn main() {
    let cat = ("Furry McFurson", 3.5);

    // 已修复：解构元组 `cat`，将两个值绑定到 `name` 和 `age`。
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
