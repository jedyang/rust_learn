// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // 标注向量类型为 `i32`，因为 `u8` 和 `i8` 都可以转换成 `i32`
    let mut numbers: Vec<i32> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
