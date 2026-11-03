fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // 已修复：从数组 `a` 中获取切片 `nice_slice`，包含索引 1..4 的元素。
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
