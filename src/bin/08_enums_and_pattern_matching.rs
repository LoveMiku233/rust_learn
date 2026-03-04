// 08_enums_and_pattern_matching.rs
// 主题：枚举（Enum）与模式匹配（Pattern Matching）
//
// 枚举（enum）允许定义一个可以是多种变体之一的类型。
// 每个变体可以附带不同类型的数据（无数据、元组、具名字段）。
//
// 模式匹配（match）是 Rust 处理枚举的核心手段，
// 要求覆盖所有可能的情况（穷尽性检查）。
//
// Option<T>：Rust 标准库中内置的枚举，用于表示"可能不存在"的值，
//             替代其他语言中的 null。

// ==== 基本枚举定义 ====
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ==== 带数据的枚举 ====
// 变体可以携带不同类型和数量的数据
#[derive(Debug)]
enum Shape {
    Circle(f64),             // 元组变体：半径
    Rectangle(f64, f64),     // 元组变体：宽、高
    Triangle {               // 结构体变体：三边
        a: f64,
        b: f64,
        c: f64,
    },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { a, b, c } => {
                // 海伦公式
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "圆形",
            Shape::Rectangle(_, _) => "矩形",
            Shape::Triangle { .. } => "三角形",
        }
    }
}

// ==== Option<T> ====
// enum Option<T> { Some(T), None }
// 用于表示值可能存在也可能不存在，避免空指针错误

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // 除数为零时返回 None
    } else {
        Some(a / b) // 有值时返回 Some
    }
}

// ==== Result<T, E> ====
// enum Result<T, E> { Ok(T), Err(E) }
// 用于可能失败的操作（详见第11章错误处理）
fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| e.to_string())
}

fn main() {
    // ==== 基本枚举使用 ====
    let dir = Direction::North;
    println!("方向：{:?}", dir);

    // match 表达式：穷尽匹配（必须覆盖所有变体）
    match dir {
        Direction::North => println!("向北走"),
        Direction::South => println!("向南走"),
        Direction::East => println!("向东走"),
        Direction::West => println!("向西走"),
    }

    // ==== 带数据的枚举匹配 ====
    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Rectangle(4.0, 5.0),
        Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 },
    ];

    for shape in &shapes {
        println!("{}: 面积 = {:.2}", shape.name(), shape.area());
    }

    // ==== Option 的使用 ====
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    // match 处理 Option
    match result1 {
        Some(val) => println!("10 / 2 = {}", val),
        None => println!("除法失败"),
    }

    // if let：只关心一种情况时更简洁
    if let Some(val) = result2 {
        println!("结果：{}", val);
    } else {
        println!("除以零，无结果");
    }

    // Option 的常用方法
    let opt: Option<i32> = Some(42);
    println!("unwrap_or：{}", opt.unwrap_or(0));           // 有值取值，无值取默认
    println!("map：{:?}", opt.map(|x| x * 2));             // 有值时变换
    println!("is_some：{}", opt.is_some());
    println!("is_none：{}", opt.is_none());

    let none_opt: Option<i32> = None;
    println!("None.unwrap_or：{}", none_opt.unwrap_or(-1));

    // ==== 模式匹配高级用法 ====

    // 1. 绑定变量（binding）：@ 运算符
    let num = 7;
    match num {
        n @ 1..=5 => println!("{} 在 1-5 范围内", n),
        n @ 6..=10 => println!("{} 在 6-10 范围内", n),
        _ => println!("其他"),
    }

    // 2. 守卫条件（match guard）
    let pair = (2, -3);
    match pair {
        (x, y) if x == y => println!("相等"),
        (x, y) if x + y == 0 => println!("互为相反数"),
        (x, _) if x > 0 => println!("x 是正数"),
        _ => println!("其他情况"),
    }

    // 3. 解构结构体变体
    let shape = Shape::Rectangle(3.0, 4.0);
    if let Shape::Rectangle(w, h) = shape {
        println!("矩形：{}×{}", w, h);
    }

    // 4. 忽略部分字段 ..
    let circle = Shape::Circle(5.0);
    match circle {
        Shape::Circle(r) if r > 3.0 => println!("大圆，半径 {}", r),
        Shape::Circle(_) => println!("小圆"),
        _ => {}
    }

    // 5. 多重模式（|）
    let c = 'A';
    match c {
        'a'..='z' | 'A'..='Z' => println!("'{}' 是字母", c),
        '0'..='9' => println!("'{}' 是数字", c),
        _ => println!("'{}' 是其他字符", c),
    }

    // ==== Result 处理 ====
    match parse_int("42") {
        Ok(n) => println!("解析成功：{}", n),
        Err(e) => println!("解析失败：{}", e),
    }
    match parse_int("abc") {
        Ok(n) => println!("解析成功：{}", n),
        Err(e) => println!("解析失败：{}", e),
    }

    // ==== 枚举作为状态机 ====
    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    impl TrafficLight {
        fn duration_secs(&self) -> u32 {
            match self {
                TrafficLight::Red => 60,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 45,
            }
        }
        fn next(&self) -> TrafficLight {
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Green => TrafficLight::Yellow,
                TrafficLight::Yellow => TrafficLight::Red,
            }
        }
    }

    let mut light = TrafficLight::Red;
    for _ in 0..4 {
        println!("{:?} 持续 {} 秒", light, light.duration_secs());
        light = light.next();
    }
}
