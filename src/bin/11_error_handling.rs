// 11_error_handling.rs
// 主题：错误处理
//
// Rust 将错误分为两类：
//   1. 不可恢复错误（Unrecoverable）：使用 panic! 宏终止程序。
//   2. 可恢复错误（Recoverable）：使用 Result<T, E> 枚举处理。
//
// Result<T, E> 枚举：
//   enum Result<T, E> { Ok(T), Err(E) }
//
// ? 运算符：在函数中传播错误，是处理 Result/Option 的简洁语法糖。

use std::fmt;
use std::num::ParseIntError;

// ==== 自定义错误类型 ====
// 良好的错误处理通常需要自定义错误类型
#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    DivisionByZero,
    NegativeNumber(i32),
    Custom(String),
}

// 实现 Display trait 用于友好显示
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ParseError(e) => write!(f, "解析错误：{}", e),
            AppError::DivisionByZero => write!(f, "除以零错误"),
            AppError::NegativeNumber(n) => write!(f, "负数错误：{}", n),
            AppError::Custom(msg) => write!(f, "自定义错误：{}", msg),
        }
    }
}

// 实现 std::error::Error trait（标准错误特征）
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

// From trait：允许使用 ? 运算符自动转换错误类型
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> AppError {
        AppError::ParseError(e)
    }
}

// ==== 使用 ? 运算符的函数 ====
// ? 运算符：如果 Result 是 Err，自动返回 Err；如果是 Ok，解包值
fn parse_and_double(s: &str) -> Result<i32, AppError> {
    let n: i32 = s.parse()?; // ParseIntError 自动转为 AppError::ParseError
    if n < 0 {
        return Err(AppError::NegativeNumber(n));
    }
    Ok(n * 2)
}

fn safe_divide(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        Err(AppError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

// 链式使用 ? 运算符
fn complex_operation(input: &str, divisor_str: &str) -> Result<i32, AppError> {
    let a = parse_and_double(input)?;      // 如果失败，提前返回 Err
    let b: i32 = divisor_str.parse()?;    // 自动转换 ParseIntError
    let result = safe_divide(a, b)?;       // 如果失败，提前返回 Err
    Ok(result)
}

// Option 中的 ? 运算符（需要函数返回 Option<T>）
fn get_first_char(s: &str) -> Option<char> {
    let first = s.chars().next()?; // None 时提前返回 None
    Some(first)
}

fn main() {
    // ==== panic!：不可恢复错误 ====
    // panic! 会终止程序并打印错误信息（通常在调试或不可修复的逻辑错误时使用）
    // 以下代码会触发 panic（已注释掉）：
    // panic!("程序崩溃！");
    // let v = vec![1, 2, 3];
    // v[99]; // 越界访问，触发 panic

    println!("==== Result 错误处理 ====");

    // ==== 方法1：match 处理 Result ====
    match safe_divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误：{}", e),
    }
    match safe_divide(10, 0) {
        Ok(result) => println!("结果：{}", result),
        Err(e) => println!("错误：{}", e),
    }

    // ==== 方法2：unwrap 和 expect（会在 Err 时 panic）====
    // unwrap()：Ok 时返回值，Err 时 panic
    // expect("msg")：类似 unwrap，但 panic 时显示自定义消息
    // 建议只在确定不会出错或原型开发时使用
    let result = safe_divide(20, 4).unwrap();
    println!("unwrap 结果：{}", result);

    let result2 = safe_divide(15, 3).expect("除法操作失败");
    println!("expect 结果：{}", result2);

    // ==== 方法3：unwrap_or / unwrap_or_else ====
    let r1 = safe_divide(10, 0).unwrap_or(0); // 出错时返回默认值
    println!("unwrap_or(0)：{}", r1);

    let r2 = safe_divide(10, 0).unwrap_or_else(|e| {
        println!("处理错误：{}，返回 -1", e);
        -1
    });
    println!("unwrap_or_else 结果：{}", r2);

    // ==== 方法4：? 运算符传播错误 ====
    println!("\n==== ? 运算符 ====");
    match parse_and_double("21") {
        Ok(n) => println!("parse_and_double('21') = {}", n),
        Err(e) => println!("错误：{}", e),
    }
    match parse_and_double("abc") {
        Ok(n) => println!("结果：{}", n),
        Err(e) => println!("错误：{}", e),
    }
    match parse_and_double("-5") {
        Ok(n) => println!("结果：{}", n),
        Err(e) => println!("错误：{}", e),
    }

    // 链式 ? 使用
    println!("\n==== 链式错误传播 ====");
    match complex_operation("10", "4") {
        Ok(n) => println!("complex_operation('10', '4') = {}", n),
        Err(e) => println!("错误：{}", e),
    }
    match complex_operation("abc", "4") {
        Ok(n) => println!("结果：{}", n),
        Err(e) => println!("错误：{}", e),
    }

    // ==== Result 的常用方法 ====
    println!("\n==== Result 方法 ====");
    let ok_val: Result<i32, AppError> = Ok(42);
    let err_val: Result<i32, AppError> = Err(AppError::DivisionByZero);

    println!("is_ok：{}", ok_val.is_ok());
    println!("is_err：{}", err_val.is_err());

    // map：对 Ok 值进行变换（as_ref() 避免移动所有权）
    let doubled = ok_val.as_ref().map(|&x| x * 2);
    println!("map *2：{:?}", doubled);

    // map_err：对 Err 进行变换
    let mapped_err = err_val.map_err(|e| format!("错误：{}", e));
    println!("map_err：{:?}", mapped_err);

    // and_then：链式操作（flatMap）
    let chained = ok_val.and_then(|x| {
        if x > 0 { Ok(x + 10) } else { Err(AppError::NegativeNumber(x)) }
    });
    println!("and_then：{:?}", chained);

    // ==== Option 中的 ? 运算符 ====
    println!("\n==== Option ? 运算符 ====");
    println!("get_first_char('hello')：{:?}", get_first_char("hello"));
    println!("get_first_char('')：{:?}", get_first_char(""));

    // ==== 自定义错误类型 ====
    println!("\n==== 自定义错误类型 ====");
    let custom_err = AppError::Custom(String::from("某些操作失败"));
    println!("Display：{}", custom_err);
    println!("Debug：{:?}", custom_err);

    println!("\n错误处理示例完成！");
}
