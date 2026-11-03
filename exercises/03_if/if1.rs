fn bigger(a: i32, b: i32) -> i32 {
    // 已修复：使用 if 表达式返回较大的数字。
    // 如果两个数字相等，返回任意一个都可以。
    // 不使用：
    // - 其他函数调用
    // - 额外的变量
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    // You can optionally experiment here.
    let answer = bigger(10, 8);
    println!("The bigger number is {answer}");
}

// 现在不用管这个 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
