fn factorial(num: u64) -> u64 {
    // 完成此函数，返回 `num` 的阶乘，定义为 `1 * 2 * 3 * … * num`。
    // 不使用：
    // - 提前返回（显式使用 `return` 关键字）
    // 尽量不使用：
    // - 命令式循环（for/while）
    // - 额外变量
    // 额外挑战，不使用：
    // - 递归
    (1..=num).product()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
