fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 数组

    // 已修复：使用 `vec!` 宏创建向量 `v`，包含与数组 `a` 相同的元素。
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
