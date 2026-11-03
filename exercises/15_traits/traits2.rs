trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为字符串向量实现 `AppendBar` trait。
// `append_bar` 将字符串 "Bar" 推入向量末尾。
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
