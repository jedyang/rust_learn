fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // 已修复：将 `input` 切片中每个元素乘以 2，然后推入 `output` 向量。
        output.push(element * 2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 使用映射后收集为向量的示例。
    // 我们将 `input` 切片中的每个元素加 1。
    // 如果输入是 `[1, 2, 3]`，输出是 `[2, 3, 4]`。
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // 已修复：使用迭代器的 `map` 方法将每个元素乘以 2，然后收集为向量。
    // 参考上面的 `vec_map_example` 函数。
    input
        .iter()
        .map(|element| {
            element * 2
        })
        .collect()
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
