// 这家商店正在促销：如果价格是偶数，减 10 Rustbucks；
// 如果是奇数，减 3 Rustbucks。
// 不用关心函数体本身，我们只对函数签名感兴趣。

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// 已修复：函数签名中添加了返回值类型 `-> i64`。
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
