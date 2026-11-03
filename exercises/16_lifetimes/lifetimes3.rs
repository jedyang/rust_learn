// 当结构体持有引用时也需要生命周期参数。
// 修复结构体的编译器错误：添加生命周期参数 `'a`。
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
