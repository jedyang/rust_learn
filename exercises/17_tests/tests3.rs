struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Returning a `Result` would be better here. But we want to learn
            // how to test functions that can panic.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 这个测试应该检查矩形是否具有我们传递给构造函数的尺寸。
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 检查宽度
        assert_eq!(rect.height, 20); // 检查高度
    }

    // 这个测试应该检查当我们尝试创建一个宽度为负的矩形时程序是否会 panic。
    #[test]
    #[should_panic]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // 这个测试应该检查当我们尝试创建一个高度为负的矩形时程序是否会 panic。
    #[test]
    #[should_panic]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
