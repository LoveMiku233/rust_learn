// 09_collections.rs
// 主题：常见集合类型及常用操作
//
// Rust 标准库提供了几种常用的集合类型，数据存储在堆上：
//   - Vec<T>：动态数组，按顺序存储同类型元素
//   - String：UTF-8 编码的可增长字符串
//   - HashMap<K, V>：键值对哈希映射
//   - HashSet<T>：不重复元素的集合（基于 HashMap 实现）

use std::collections::{HashMap, HashSet};

fn main() {
    // ======================================================
    // Vec<T>：动态数组
    // ======================================================

    // ---- 创建 Vec ----
    let mut v: Vec<i32> = Vec::new(); // 空 Vec
    let v2 = vec![1, 2, 3]; // vec! 宏快速创建
    println!("v2 = {:?}", v2);

    // ---- 添加元素 ----
    v.push(1);
    v.push(2);
    v.push(3);
    println!("push 后：{:?}", v);

    // ---- 访问元素 ----
    // 方法1：索引（越界会 panic）
    println!("v[0] = {}", v[0]);

    // 方法2：get 方法（返回 Option，安全）
    match v.get(10) {
        Some(val) => println!("找到：{}", val),
        None => println!("索引越界，返回 None"),
    }

    // ---- 遍历 Vec ----
    for x in &v {
        print!("{} ", x);
    }
    println!();

    // 遍历并修改（可变引用）
    let mut nums = vec![1, 2, 3, 4, 5];
    for x in &mut nums {
        *x *= 2; // 解引用后修改
    }
    println!("乘 2 后：{:?}", nums);

    // ---- 常用 Vec 方法 ----
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    println!("长度：{}", data.len());
    println!("是否为空：{}", data.is_empty());
    data.sort();                          // 排序（就地）
    println!("排序后：{:?}", data);
    data.dedup();                         // 去除相邻重复
    println!("去重后：{:?}", data);
    data.retain(|&x| x > 3);             // 保留满足条件的元素
    println!("保留 >3：{:?}", data);

    let popped = data.pop();              // 移除并返回末尾元素
    println!("pop：{:?}，剩余：{:?}", popped, data);

    data.insert(1, 99);                   // 在索引 1 处插入 99
    println!("insert(1, 99)：{:?}", data);

    data.remove(1);                       // 移除索引 1 处的元素
    println!("remove(1)：{:?}", data);

    // 切片与迭代器操作
    let v3 = vec![1, 2, 3, 4, 5];
    let sum: i32 = v3.iter().sum();
    let max = v3.iter().max().unwrap();
    let doubled: Vec<i32> = v3.iter().map(|&x| x * 2).collect();
    let evens: Vec<&i32> = v3.iter().filter(|&&x| x % 2 == 0).collect();
    println!("sum={}, max={}, doubled={:?}, evens={:?}", sum, max, doubled, evens);

    // ======================================================
    // String：UTF-8 字符串
    // ======================================================

    // ---- 创建 String ----
    let mut s = String::new();
    let s2 = String::from("hello");
    let s3 = "world".to_string();
    println!("s2={}, s3={}", s2, s3);

    // ---- 追加内容 ----
    s.push_str("hello");  // 追加字符串切片
    s.push(' ');          // 追加单个字符
    s.push_str("world");
    println!("拼接：{}", s);

    // 用 + 运算符连接（会移动第一个 String）
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // s4 被移动，s5 仍有效
    println!("+ 连接：{}", s6);

    // 用 format! 宏连接（不移动所有权）
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let combined = format!("{}-{}-{}", s7, s8, s9);
    println!("format! 连接：{}", combined);

    // ---- String 常用方法 ----
    let text = String::from("  Hello, Rust!  ");
    println!("trim：'{}'", text.trim());
    println!("to_uppercase：{}", text.trim().to_uppercase());
    println!("to_lowercase：{}", text.trim().to_lowercase());
    println!("contains 'Rust'：{}", text.contains("Rust"));
    println!("replace：{}", text.trim().replace("Rust", "World"));
    println!("len（字节数）：{}", text.len());
    println!("chars 数：{}", text.chars().count()); // 字符数

    // 遍历字符
    for c in "你好，世界！".chars() {
        print!("{} ", c);
    }
    println!();

    // 字符串分割
    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("分割：{:?}", fruits);

    // ======================================================
    // HashMap<K, V>：哈希映射
    // ======================================================

    // ---- 创建 HashMap ----
    let mut scores: HashMap<String, i32> = HashMap::new();

    // ---- 插入键值对 ----
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 78);
    println!("scores：{:?}", scores);

    // ---- 访问值 ----
    let name = String::from("Alice");
    if let Some(score) = scores.get(&name) {
        println!("{} 的分数：{}", name, score);
    }

    // ---- 遍历 HashMap ----（顺序不固定）
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // ---- 只在键不存在时插入（entry API）----
    scores.entry(String::from("Dave")).or_insert(70);   // Dave 不存在，插入 70
    scores.entry(String::from("Alice")).or_insert(100); // Alice 已存在，不覆盖
    println!("entry 后：{:?}", scores);

    // ---- 基于旧值更新 ----
    let text_count = "hello world hello rust hello";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text_count.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1; // 解引用修改计数
    }
    println!("词频统计：{:?}", word_count);

    // ---- 其他常用方法 ----
    println!("包含 'Bob'：{}", scores.contains_key("Bob"));
    scores.remove("Bob");
    println!("移除 Bob 后：{:?}", scores);
    println!("HashMap 长度：{}", scores.len());

    // ======================================================
    // HashSet<T>：集合（无重复元素）
    // ======================================================

    let mut set1: HashSet<i32> = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set1.insert(2); // 重复元素不会被插入
    println!("HashSet：{:?}", set1);
    println!("包含 2：{}", set1.contains(&2));
    set1.remove(&2);
    println!("移除 2 后：{:?}", set1);

    // 集合操作：交集、并集、差集
    let set2: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set3: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

    let intersection: HashSet<&i32> = set2.intersection(&set3).collect();
    let union: HashSet<&i32> = set2.union(&set3).collect();
    let difference: HashSet<&i32> = set2.difference(&set3).collect();
    println!("交集：{:?}", intersection);
    println!("并集：{:?}", union);
    println!("差集（set2 - set3）：{:?}", difference);
}
