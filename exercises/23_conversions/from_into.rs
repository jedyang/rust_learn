// The `From` trait is used for value-to-value conversions. If `From` is
// implemented, an implementation of `Into` is automatically provided.
// You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// We implement the Default trait to use it as a fallback when the provided
// string is not convertible into a `Person` object.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 实现 From trait，从 "Mark,20" 格式的字符串解析出 Person
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        // 1. 按逗号分割字符串
        let mut parts = s.split(',');
        let name = parts.next();
        let age = parts.next();

        // 2. 如果分割后元素数量不等于 2（第一个或第二个不存在，或还有更多），返回默认值
        // 3. 如果名字为空，返回默认值
        // 6. 如果年龄解析失败，返回默认值
        match (name, age, parts.next()) {
            (Some(name), Some(age_str), None) if !name.is_empty() => {
                match age_str.parse::<u8>() {
                    Ok(age) => Person {
                        name: String::from(name),
                        age,
                    },
                    Err(_) => Person::default(),
                }
            }
            _ => Person::default(),
        }
    }
}

fn main() {
    // Use the `from` function.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // Since `From` is implemented for Person, we are able to use `Into`.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
