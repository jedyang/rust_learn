# Rust HashMap 知识点总结

HashMap 是键值对集合，类似 C++ 的 `unordered_map`、Python 的 `dict`。通过键快速查找对应的值。

---

## 1. 创建 HashMap 与 `insert` 插入键值对

> 文件：`hashmaps1.rs`

### 知识点
- 使用 `HashMap::new()` 创建空的 HashMap。
- 使用 `insert(key, value)` 插入键值对，如果键已存在会**覆盖**旧值。
- `HashMap` 需要引入 `std::collections::HashMap`。
- `basket.len()` 返回键的种类数，`basket.values().sum::<u32>()` 对所有值求和。

### 代码

```rust
use std::collections::HashMap;

// 用 HashMap 表示水果篮，键是水果名，值是数量
fn fruit_basket() -> HashMap<String, u32> {
    // 声明 HashMap
    let mut basket = HashMap::new();

    // 已给出两个香蕉
    basket.insert(String::from("banana"), 2);

    // 添加更多水果，至少 3 种、总数至少 5 个
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);

    basket
}

fn main() {
    // 可以在这里实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
```

### 执行结果

```
running 2 tests
test tests::at_least_three_types_of_fruits ... ok
test tests::at_least_five_fruits ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 2. `entry` 与 `or_insert` —— 只在键不存在时插入

> 文件：`hashmaps2.rs`

### 知识点
- `insert` 会覆盖已有值，但有时我们只想在键**不存在**时才插入。
- `entry(key)` 返回一个 `Entry` 枚举，表示键可能存在或不存在。
- `or_insert(value)`：如果键不存在则插入该值并返回可变引用；如果键已存在则直接返回已有值的可变引用，**不会覆盖**。
- 这是 HashMap 中非常重要的模式，避免了先 `contains_key` 再 `insert` 的冗余写法。
- 用作键的类型必须实现 `Hash`、`Eq` 等 trait（枚举需派生 `Hash`、`PartialEq`、`Eq`）。

### 代码

```rust
use std::collections::HashMap;

// 水果枚举，用作 HashMap 的键需要实现 Hash + Eq
#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // 只在篮子中不存在该水果时才插入，已有水果的数量不会被修改
        basket.entry(fruit).or_insert(5);
    }
}

fn main() {
    // 可以在这里实验
}

#[cfg(test)]
mod tests {
    use super::*;

    // 不要修改这个函数！
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        // 已有的 Apple(4)、Mango(2)、Lychee(5) 数量不变
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Fruit kind {fruit_kind:?} was not found in basket");
            };
            assert!(*amount > 0);
        }
    }
}
```

### 执行结果

```
running 4 tests
test tests::test_given_fruits_are_not_modified ... ok
test tests::at_least_five_types_of_fruits ... ok
test tests::greater_than_eleven_fruits ... ok
test tests::all_fruit_types_in_basket ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 3. `entry` 与 `or_default` —— 累加统计实战

> 文件：`hashmaps3.rs`

### 知识点
- `or_default()` 等价于 `or_insert(Default::default())`，要求值类型实现 `Default` trait。
- 对于自定义结构体，派生 `#[derive(Default)]` 即可自动实现，所有字段初始化为零值。
- `entry(key).or_default()` 返回值的**可变引用**，可以直接修改字段，非常适合累加统计场景。
- 在本题中，每行比赛结果需要同时更新两支队伍的进球数和失球数，`entry` + `or_default` 模式使代码简洁高效。

### 代码

```rust
use std::collections::HashMap;

// 存储队伍进球和失球的结构体，派生 Default 使 or_default() 可用
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // 如果队伍不存在，用默认值（0, 0）插入；然后累加数据
        let team_1 = scores.entry(team_1_name).or_default();
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        // 同样处理第二支队伍
        let team_2 = scores.entry(team_2_name).or_default();
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }

    scores
}

fn main() {
    // 可以在这里实验
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(
            ["England", "France", "Germany", "Italy", "Poland", "Spain"]
                .into_iter()
                .all(|team_name| scores.contains_key(team_name))
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        // England: 进球 4+1=6，失球 2+2=4
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        // Spain: 进球 0+0=0，失球 2+1=3
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
```

### 执行结果

```
running 3 tests
test tests::build_scores ... ok
test tests::validate_team_score_1 ... ok
test tests::validate_team_score_2 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 总结

| 知识点 | 要点 |
|---|---|
| 创建 HashMap | `HashMap::new()` 创建空 HashMap |
| 插入键值对 | `insert(key, value)` 插入，键已存在则**覆盖**旧值 |
| 条件插入 | `entry(key).or_insert(value)` 键不存在时插入，存在时不覆盖 |
| 默认值插入 | `entry(key).or_default()` 键不存在时插入 `Default` 值，需类型实现 `Default` |
| 获取可变引用 | `entry().or_insert()` / `or_default()` 返回可变引用，可直接修改 |
| 键的要求 | 键类型必须实现 `Hash` + `Eq`，枚举需派生 `Hash`、`PartialEq`、`Eq` |
| 值的默认实现 | `#[derive(Default)]` 为结构体自动生成零值默认 |
| 常用方法 | `len()` 键数、`values()` 值迭代器、`get()` 按键查询、`contains_key()` 判断存在 |
| `entry` 模式 | 避免先 `contains_key` 再 `insert`，一行完成"查询或插入"，是 Rust 惯用写法 |
