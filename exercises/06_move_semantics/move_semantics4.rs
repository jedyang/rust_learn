fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    // 已修复：通过重新排序行来修复编译错误，先使用 `y` 再创建 `z`。
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);

        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
