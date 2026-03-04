// 15_standard_library_and_macros.rs
// 主题：常用标准库函数与实用宏
//
// 本文件介绍 Rust 标准库中常用的工具与宏：
//   - 迭代器（Iterator）适配器与消费者
//   - 字符串处理
//   - 数学函数
//   - 常用宏：println!, print!, format!, vec!, assert!, panic!, todo!, dbg!
//   - 闭包（Closure）
//   - std::mem, std::cmp 等实用工具

use std::collections::HashMap;

fn main() {
    // ======================================================
    // ==== 常用宏 ====
    // ======================================================
    println!("==== 常用宏 ====");

    // println! / print!：格式化输出到标准输出
    // 支持多种格式化说明符：
    println!("{}", 42);            // 默认格式
    println!("{:?}", vec![1,2,3]); // Debug 格式
    println!("{:#?}", ("a", 1));   // 美化 Debug 格式
    println!("{:5}", 42);          // 宽度为 5（右对齐）
    println!("{:<5}", 42);         // 左对齐
    println!("{:^5}", 42);         // 居中对齐
    println!("{:0>5}", 42);        // 用 0 填充，右对齐
    println!("{:.3}", 3.14159);    // 小数点后 3 位
    println!("{:08.3}", 3.14159);  // 总宽度 8，小数 3 位，0 填充
    println!("{:b}", 255);         // 二进制
    println!("{:o}", 255);         // 八进制
    println!("{:x}", 255);         // 十六进制（小写）
    println!("{:X}", 255);         // 十六进制（大写）
    println!("{:e}", 1_000_000.0); // 科学计数法

    // format!：格式化为 String
    let s = format!("{}+{}={}", 1, 2, 3);
    println!("format! 结果：{}", s);

    // vec!：创建 Vec
    let v = vec![0; 5]; // [0, 0, 0, 0, 0]
    println!("vec! 宏：{:?}", v);

    // assert! / assert_eq! / assert_ne!：断言（测试中常用）
    assert!(2 + 2 == 4, "数学错误！");
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
    println!("assert 通过！");

    // dbg!：调试输出（输出到 stderr，返回值）
    let a = dbg!(2 + 3); // 打印 "[src/...] 2 + 3 = 5" 并返回 5
    println!("dbg! 返回值：{}", a);

    // todo! / unimplemented!：标记未实现的代码
    fn _unfinished_function() -> i32 {
        todo!("还没实现这个功能")
    }

    // eprintln!：输出到标准错误
    eprintln!("这是错误输出（显示在 stderr）");

    // ======================================================
    // ==== 迭代器（Iterator）====
    // ======================================================
    println!("\n==== 迭代器 ====");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // ---- 迭代器适配器（惰性，不消费，返回新迭代器）----
    // map：变换每个元素
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("map（*2）：{:?}", doubled);

    // filter：保留满足条件的元素
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter（偶数）：{:?}", evens);

    // filter_map：同时过滤和变换
    let parsed: Vec<i32> = vec!["1", "two", "3", "four", "5"]
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("filter_map：{:?}", parsed);

    // flat_map / flatten
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten：{:?}", flat);

    // enumerate：带索引遍历
    for (i, val) in numbers.iter().enumerate().take(3) {
        println!("  [{i}] = {val}");
    }

    // zip：将两个迭代器合并为元组迭代器
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![90, 85, 78];
    let pairs: Vec<(&&str, &i32)> = names.iter().zip(scores.iter()).collect();
    println!("zip：{:?}", pairs);

    // chain：连接两个迭代器
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("chain：{:?}", chained);

    // take / skip：截取和跳过
    let first3: Vec<&i32> = numbers.iter().take(3).collect();
    let skip3: Vec<&i32> = numbers.iter().skip(7).collect();
    println!("take(3)：{:?}", first3);
    println!("skip(7)：{:?}", skip3);

    // ---- 迭代器消费者（消耗迭代器，产生最终值）----
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();
    let count = numbers.iter().count();
    println!("sum={}, product={}, max={}, min={}, count={}", sum, product, max, min, count);

    // fold：自定义累积操作
    let sum_fold = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("fold（累加）：{}", sum_fold);

    // any / all：检查条件
    let any_gt5 = numbers.iter().any(|&x| x > 5);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("any(>5)：{}, all(>0)：{}", any_gt5, all_positive);

    // find / position：查找元素
    let first_gt5 = numbers.iter().find(|&&x| x > 5);
    let pos = numbers.iter().position(|&x| x == 5);
    println!("find(>5)：{:?}, position(5)：{:?}", first_gt5, pos);

    // collect 到 HashMap
    let map: HashMap<&str, i32> = names.iter().zip(scores.iter()).map(|(&k, &v)| (k, v)).collect();
    println!("collect 到 HashMap：{:?}", map);

    // ======================================================
    // ==== 闭包（Closure）====
    // ======================================================
    println!("\n==== 闭包 ====");

    // 闭包可以捕获环境中的变量
    let base = 10;
    let add_base = |x| x + base; // 捕获 base
    println!("闭包（捕获 base={}）：{}", base, add_base(5));

    // move 闭包：强制获取捕获变量的所有权
    let text = String::from("hello");
    let greeting = move || println!("move 闭包：{}", text);
    greeting();
    // println!("{}", text); // 编译错误：text 已被移动到闭包

    // 闭包作为高阶函数参数
    fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(f(x)) }
    println!("apply_twice(|x| x*2, 3) = {}", apply_twice(|x| x * 2, 3));

    // FnMut：可以修改捕获的变量
    let mut counter = 0;
    let mut increment = || { counter += 1; counter };
    println!("FnMut: {}, {}, {}", increment(), increment(), increment());

    // ======================================================
    // ==== std::cmp 工具 ====
    // ======================================================
    println!("\n==== std::cmp ====");
    use std::cmp;
    println!("max(3, 7) = {}", cmp::max(3, 7));
    println!("min(3, 7) = {}", cmp::min(3, 7));
    println!("clamp(15, 0, 10) = {}", 15_i32.clamp(0, 10)); // 夹紧到范围

    // ======================================================
    // ==== std::mem 工具 ====
    // ======================================================
    println!("\n==== std::mem ====");
    use std::mem;
    println!("size_of::<i32> = {}", mem::size_of::<i32>());
    println!("size_of::<f64> = {}", mem::size_of::<f64>());
    println!("size_of::<String> = {}", mem::size_of::<String>());

    // swap：交换两个变量的值
    let mut x = 5;
    let mut y = 10;
    mem::swap(&mut x, &mut y);
    println!("swap 后：x={}, y={}", x, y);

    // replace：替换值并返回旧值
    let mut s = String::from("旧值");
    let old = mem::replace(&mut s, String::from("新值"));
    println!("replace：old='{}', s='{}'", old, s);

    // ======================================================
    // ==== 数学函数 ====
    // ======================================================
    println!("\n==== 数学函数 ====");
    let n: f64 = 2.0;
    println!("sqrt(2) = {:.6}", n.sqrt());
    println!("powi(2, 10) = {}", 2_i32.pow(10));
    println!("powf(2.0, 0.5) = {:.6}", n.powf(0.5));
    println!("abs(-5) = {}", (-5_i32).abs());
    println!("floor(3.7) = {}", 3.7_f64.floor());
    println!("ceil(3.2) = {}", 3.2_f64.ceil());
    println!("round(3.5) = {}", 3.5_f64.round());
    println!("ln(e) = {:.6}", std::f64::consts::E.ln());
    println!("log2(8) = {:.6}", 8.0_f64.log2());
    println!("sin(π/2) = {:.6}", (std::f64::consts::PI / 2.0).sin());

    // ======================================================
    // ==== 字符串工具 ====
    // ======================================================
    println!("\n==== 字符串工具 ====");
    let text = "  Hello, Rust World!  ";
    println!("trim：'{}'", text.trim());
    println!("split_whitespace：{:?}", text.split_whitespace().collect::<Vec<_>>());
    println!("to_uppercase：{}", text.trim().to_uppercase());
    println!("starts_with：{}", text.trim().starts_with("Hello"));
    println!("ends_with：{}", text.trim().ends_with("!"));
    println!("find('R')：{:?}", text.find('R'));
    println!("replacen：{}", text.replacen("o", "0", 1));
    println!("repeat：{}", "ab".repeat(3));
    // 字符串转数字
    let n: i32 = "42".parse().unwrap();
    println!("parse：{}", n);
    // 数字转字符串
    println!("to_string：{}", 42.to_string());

    println!("\n标准库与宏示例完成！");
}
