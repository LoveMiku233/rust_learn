// 03_functions_and_scope.rs
// 主题：函数与作用域
//
// Rust 使用 fn 关键字声明函数。
// 函数参数必须标注类型，返回值类型用 `->` 指定。
// 函数体由语句（statement）和表达式（expression）组成：
//   - 语句：执行操作但不返回值（如 let 绑定）。
//   - 表达式：求值并返回结果（如代码块末尾不带分号的一行）。
// 作用域（scope）决定变量的生命周期和可见范围。

// ---- 基本函数声明 ----
// 函数可以在文件任意位置声明（Rust 不要求先声明后使用）
fn greet(name: &str) {
    println!("你好，{}！", name);
}

// ---- 带返回值的函数 ----
// 用 -> 指定返回类型，函数体最后一个表达式（无分号）作为返回值
// 也可以使用 return 关键字提前返回
fn add(x: i32, y: i32) -> i32 {
    x + y // 表达式，作为返回值
}

// ---- 多返回值（通过元组） ----
fn min_max(arr: &[i32]) -> (i32, i32) {
    let mut min = arr[0];
    let mut max = arr[0];
    for &val in arr.iter() {
        if val < min { min = val; }
        if val > max { max = val; }
    }
    (min, max) // 返回元组
}

// ---- 嵌套函数（函数内定义函数） ----
fn outer() {
    fn inner(x: i32) -> i32 {
        x * 2
    }
    println!("内部函数结果：{}", inner(5));
}

// ---- 无返回值（返回单元类型 ()）----
// 不写返回类型时，默认返回 ()
fn print_message(msg: &str) {
    println!("消息：{}", msg);
    // 隐式返回 ()
}

fn main() {
    // ---- 调用基本函数 ----
    greet("Rust 学习者");

    // ---- 调用带返回值的函数 ----
    let result = add(3, 5);
    println!("3 + 5 = {}", result);

    // ---- 调用多返回值函数 ----
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&numbers);
    println!("最小值={}, 最大值={}", min, max);

    // ---- 嵌套函数 ----
    outer();

    // ---- 表达式 vs 语句 ----
    // 语句：let 绑定，不返回值
    let _s = 5; // 语句，不能赋给另一个变量

    // 表达式：代码块可以是表达式，返回最后一行（无分号）的值
    let y = {
        let x = 3;
        x * x + 1 // 表达式：返回 10
    };
    println!("代码块表达式结果 y = {}", y);

    // ---- 作用域（Scope）----
    // 变量只在声明它的作用域（花括号 {}）内有效
    let outer_var = 10;
    {
        let inner_var = 20;
        // 在内部作用域可以访问外部变量
        println!("内部作用域：outer_var={}, inner_var={}", outer_var, inner_var);
    }
    // inner_var 在此已不可访问，以下代码会编译错误：
    // println!("{}", inner_var);
    println!("外部作用域：outer_var={}", outer_var);

    // ---- 提前返回（early return）----
    fn check_positive(n: i32) -> &'static str {
        if n < 0 {
            return "负数"; // 提前返回
        }
        "非负数"
    }
    println!("check_positive(-1)：{}", check_positive(-1));
    println!("check_positive(5)：{}", check_positive(5));

    // ---- 函数作为值（闭包初步）----
    // fn 函数指针可以作为参数传递
    fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }
    fn double(x: i32) -> i32 { x * 2 }
    println!("apply(double, 4) = {}", apply(double, 4));

    // ---- 打印消息 ----
    print_message("函数与作用域示例完成！");
}
