// 04_control_flow.rs
// 主题：流程控制
//
// Rust 提供以下流程控制结构：
//   - if / else if / else：条件分支
//   - loop：无限循环（可用 break 返回值）
//   - while：条件循环
//   - for：遍历迭代器或范围
//   - match：模式匹配（强大的分支控制，见第8章详细介绍）

fn main() {
    // ==== if 表达式 ====
    // Rust 中 if 是一个表达式，可以返回值
    // 条件必须是 bool 类型（不会自动转换，如 `if 1` 是错误的）
    let number = 7;

    if number < 5 {
        println!("{} 小于 5", number);
    } else if number == 5 {
        println!("{} 等于 5", number);
    } else {
        println!("{} 大于 5", number);
    }

    // if 作为表达式赋值（两个分支必须返回相同类型）
    let condition = true;
    let value = if condition { 100 } else { 200 };
    println!("if 表达式结果：{}", value);

    // ==== loop 循环 ====
    // loop 创建无限循环，使用 break 退出
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10; // break 可以返回一个值
        }
    };
    println!("loop 退出时 counter={}, result={}", counter, result);

    // 嵌套循环与循环标签（label）
    // 使用标签可以 break 或 continue 外层循环
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                println!("跳出外层循环，i={}, j={}", i, j);
                break 'outer; // 跳出带标签的外层循环
            }
            println!("i={}, j={}", i, j);
        }
    }

    // ==== while 循环 ====
    // 在条件为 true 时持续执行
    let mut n = 3;
    while n > 0 {
        println!("while 倒计时：{}", n);
        n -= 1;
    }
    println!("while 结束！");

    // ==== for 循环 ====
    // 推荐用法：遍历集合或范围，更安全（不存在越界）

    // 遍历数组
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        print!("{} ", element);
    }
    println!();

    // 遍历范围（Range）
    // 1..5 表示 [1, 5)，不含 5
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    // 1..=5 表示 [1, 5]，含 5
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // 带索引遍历（使用 enumerate）
    for (index, value) in arr.iter().enumerate() {
        println!("arr[{}] = {}", index, value);
    }

    // 倒序遍历（使用 rev）
    print!("倒序：");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    // ==== continue 关键字 ====
    // 跳过本次迭代，进入下一次
    print!("跳过偶数：");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        print!("{} ", i);
    }
    println!();

    // ==== 简单 match 示例 ====
    // match 是 Rust 最强大的流程控制结构，详见第8章
    let x = 3;
    match x {
        1 => println!("x 是 1"),
        2 => println!("x 是 2"),
        3 | 4 => println!("x 是 3 或 4"),   // 多个模式
        5..=9 => println!("x 在 5 到 9 之间"), // 范围模式
        _ => println!("其他值"),              // 通配符，匹配任意值
    }

    // ==== if let（单模式匹配的简洁写法）====
    // 当只关心一种 match 情况时，可以用 if let 代替 match
    let some_value: Option<i32> = Some(42);
    if let Some(v) = some_value {
        println!("if let 匹配到值：{}", v);
    }

    // ==== while let（条件模式循环）====
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("弹出：{} ", top);
    }
    println!();
}
