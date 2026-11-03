// 已修复：函数的 if/else 分支都返回 `&str` 类型，类型一致即可编译通过。
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else {
        "No thanks!"
    }
}

fn main() {
    // 你可以在这里进行可选的实验。
    let answer = picky_eater("strawberry");
    println!("I like {answer}");
}

// 已修复：阅读测试用例理解预期行为，确保所有测试通过且不修改测试代码。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // 这意味着调用 `picky_eater` 并传入 "strawberry" 应返回 "Yummy!"。
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
