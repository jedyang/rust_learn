// 这个强大的包装器可以存储正值。
// 使用泛型重写，使其可以包装任意类型。
struct Wrapper<T> {
    value: T,
}

// 适配结构体的实现，使其对包装值使用泛型。
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
