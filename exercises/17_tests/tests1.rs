// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // 导入 `is_even`。可以使用通配符导入外部模块的所有内容。
    use super::*;

    #[test]
    fn you_can_assert() {
        // 测试 `is_even` 函数的一些值
        assert!(is_even(2));
        assert!(!is_even(3));
    }
}
