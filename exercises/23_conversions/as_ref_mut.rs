// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// 获取给定参数的字节数（`.len()` 返回字符串的字节数）
// 添加 AsRef<str> trait bound，使函数可以接受任何能转换为 &str 的类型
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// 获取给定参数的字符数（而非字节数）
// 添加 AsRef<str> trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用 as_mut() 对数字求平方
// 添加 AsMut<u32> trait bound
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // 通过 as_mut() 获取可变引用，然后求平方
    let num = arg.as_mut();
    *num *= *num;
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
