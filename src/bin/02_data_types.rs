// 02_data_types.rs
// 主题：数据类型——标量类型与复合类型
//
// Rust 是静态类型语言，编译时必须知道所有变量的类型。
// 标量类型（Scalar）：表示单个值，包括整数、浮点数、布尔值、字符。
// 复合类型（Compound）：将多个值组合成一个类型，包括元组（tuple）和数组（array）。

fn main() {
    // ==== 标量类型 ====

    // ---- 整数类型 ----
    // 有符号整数：i8, i16, i32, i64, i128, isize
    // 无符号整数：u8, u16, u32, u64, u128, usize
    // 默认推断为 i32
    let a: i32 = -42;
    let b: u64 = 100;
    let c: i8 = 127; // i8 范围：-128 ~ 127
    println!("整数：a={}, b={}, c={}", a, b, c);

    // 不同进制的整数字面量
    let decimal = 98_222;       // 十进制（下划线分隔提高可读性）
    let hex = 0xff;             // 十六进制
    let octal = 0o77;           // 八进制
    let binary = 0b1111_0000;   // 二进制
    let byte: u8 = b'A';        // 字节（仅限 u8）
    println!("不同进制：{}, {}, {}, {}, {}", decimal, hex, octal, binary, byte);

    // ---- 浮点类型 ----
    // f32（单精度）和 f64（双精度），默认推断为 f64
    let f1: f64 = 2.0;
    let f2: f32 = 3.14;
    println!("浮点数：f1={}, f2={:.2}", f1, f2);

    // 基本数值运算
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("运算：sum={}, diff={:.2}, prod={}, quot={:.4}, rem={}", 
             sum, difference, product, quotient, remainder);

    // ---- 布尔类型 ----
    // bool 类型，取值 true 或 false，占 1 个字节
    let t: bool = true;
    let f: bool = false;
    println!("布尔值：t={}, f={}", t, f);

    // ---- 字符类型 ----
    // char 类型表示单个 Unicode 字符，用单引号，占 4 个字节
    let letter: char = 'A';
    let emoji: char = '😊';
    let chinese: char = '中';
    println!("字符：letter={}, emoji={}, chinese={}", letter, emoji, chinese);

    // ==== 复合类型 ====

    // ---- 元组（Tuple）----
    // 元组可以组合多种不同类型，长度固定，用圆括号表示
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 通过索引访问元组元素（从 0 开始）
    println!("元组第一个元素：{}", tup.0);
    println!("元组第二个元素：{}", tup.1);
    // 解构元组
    let (x, y, z) = tup;
    println!("解构元组：x={}, y={}, z={}", x, y, z);

    // 单元类型（unit）：() 表示空值，函数无返回值时隐式返回 ()
    let unit: () = ();
    println!("单元类型：{:?}", unit);

    // ---- 数组（Array）----
    // 数组中所有元素类型必须相同，长度固定（编译期确定），存储在栈上
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组：{:?}", arr);
    println!("数组第一个元素：{}", arr[0]);
    println!("数组长度：{}", arr.len());

    // 快速初始化：创建含 5 个 3 的数组
    let repeated = [3; 5]; // 等价于 [3, 3, 3, 3, 3]
    println!("重复元素数组：{:?}", repeated);

    // 数组切片（slice）：对数组一部分的引用
    let slice = &arr[1..3]; // 取索引 1 和 2 的元素
    println!("数组切片：{:?}", slice);
}
