// 06_references_and_slices.rs
// 主题：引用与切片（References & Slices）
//
// 引用（Reference）：通过 & 符号创建，允许在不获取所有权的情况下使用值。
//   - &T 不可变引用
//   - &mut T 可变引用
//
// 切片（Slice）：对集合（数组、字符串、Vec 等）一部分的引用，不拥有数据。
//   - 字符串切片：&str
//   - 数组切片：&[T]
//
// 切片保存了起始位置和长度，是一种"胖指针"（fat pointer）。

fn main() {
    // ==== 引用回顾 ====

    let s1 = String::from("hello");
    let len = get_length(&s1); // 传不可变引用
    println!("'{}' 的长度：{}", s1, len);

    let mut s2 = String::from("hello");
    append_world(&mut s2); // 传可变引用
    println!("追加后：{}", s2);

    // ==== 字符串切片（&str）====
    // 字符串切片是对 String 或字符串字面量中一部分的引用
    // 语法：&s[start..end]，包含 start，不含 end

    let s = String::from("hello world");

    // 创建字符串切片
    let hello = &s[0..5];  // "hello"
    let world = &s[6..11]; // "world"
    println!("切片：'{}' 和 '{}'", hello, world);

    // 简写：省略起始或结束索引
    let from_start = &s[..5];  // 等同于 &s[0..5]
    let to_end = &s[6..];      // 等同于 &s[6..s.len()]
    let whole = &s[..];        // 整个字符串的切片
    println!("简写切片：'{}', '{}', '{}'", from_start, to_end, whole);

    // 字符串字面量本身就是切片（&str 类型）
    let literal: &str = "hello, world"; // 存储在程序的只读数据段
    println!("字面量切片：{}", literal);

    // 函数参数使用 &str 比 &String 更通用
    // （&String 可以自动转为 &str，但反过来不行）
    println!("第一个单词：{}", first_word(&s));
    println!("第一个单词（字面量）：{}", first_word("hello world"));

    // ==== 数组切片（&[T]）====
    // 对数组或 Vec 一部分的引用

    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..3]; // [2, 3]
    println!("数组切片：{:?}", slice);
    println!("切片长度：{}", slice.len());

    // 对 Vec 的切片
    let v = vec![10, 20, 30, 40, 50];
    let v_slice = &v[2..4]; // [30, 40]
    println!("Vec 切片：{:?}", v_slice);

    // 使用切片作为函数参数（&[i32] 比 &Vec<i32> 更通用）
    println!("数组之和：{}", sum_slice(&arr));
    println!("Vec 之和（切片）：{}", sum_slice(&v));

    // ==== 切片的安全性 ====
    // 编译器确保切片引用在有效期间数据不被修改（借用规则）
    let mut data = vec![1, 2, 3, 4, 5];
    {
        let s = &data[1..3]; // 不可变借用
        println!("借用期间的切片：{:?}", s);
        // data.push(6); // 编译错误：不能在不可变借用存在时修改
    } // 借用在此结束
    data.push(6); // 现在可以修改
    println!("修改后的 vec：{:?}", data);

    // ==== 多维数组切片 ====
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for row in matrix.iter() {
        println!("行切片：{:?}", row);
    }
}

// 接受不可变引用，计算长度
fn get_length(s: &String) -> usize {
    s.len()
}

// 接受可变引用，修改字符串
fn append_world(s: &mut String) {
    s.push_str(", world");
}

// 返回字符串切片（第一个单词）
// 参数类型 &str 可同时接受 &String 和字符串字面量
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // 找到空格，返回第一个单词的切片
        }
    }
    &s[..] // 没有空格，整个字符串就是第一个单词
}

// 接受切片参数，比接受 &Vec<i32> 更通用
fn sum_slice(arr: &[i32]) -> i32 {
    arr.iter().sum()
}
