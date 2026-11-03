// 该函数在传入空字符串时拒绝生成铭牌文本。
// 使用 `Result<String, String>` 代替 `Option<String>` 来表达错误条件，
// 这样可以返回错误信息说明问题所在。
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 空名字不被允许
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
