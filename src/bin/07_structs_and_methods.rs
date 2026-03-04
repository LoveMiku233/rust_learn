// 07_structs_and_methods.rs
// 主题：结构体（Struct）与方法（Methods）
//
// 结构体是将多个相关数据字段组合在一起的自定义类型。
// 有三种结构体形式：
//   1. 命名字段结构体（Named Field Struct）
//   2. 元组结构体（Tuple Struct）
//   3. 单元结构体（Unit-like Struct）
//
// 方法通过 impl 块定义在结构体上：
//   - &self 方法：不可变借用 self，只读方法
//   - &mut self 方法：可变借用 self，修改方法
//   - 关联函数（Associated Function）：不以 self 为第一个参数（类似"静态方法"）

// ==== 1. 命名字段结构体 ====
// 使用 #[derive(Debug)] 自动实现 Debug trait，允许用 {:?} 打印
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// 为 Rectangle 实现方法
impl Rectangle {
    // 关联函数（构造器惯例命名 new）：不接受 self 参数
    // 通过 Rectangle::new(...) 调用
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height } // 字段名与变量名相同时可简写
    }

    // &self 方法：不可变借用，计算面积
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // &self 方法：计算周长
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // &self 方法：判断能否容纳另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // &mut self 方法：修改结构体字段
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // 方法可以返回 self 以支持链式调用（这里返回新结构体）
    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// 可以有多个 impl 块（通常用于组织代码）
impl Rectangle {
    fn describe(&self) -> String {
        format!("Rectangle({}×{})", self.width, self.height)
    }
}

// ==== 2. 元组结构体（Tuple Struct）====
// 字段无命名，通过索引访问，常用于创建区分意义的新类型
#[derive(Debug)]
struct Point(f64, f64);

#[derive(Debug)]
struct Color(u8, u8, u8); // RGB

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

// ==== 3. 单元结构体（Unit-like Struct）====
// 没有任何字段，常用于实现 trait 而不需要存储数据
struct AlwaysEqual;

// ==== 具有引用字段的结构体（需要生命周期，见第14章）====
// 这里先用 String 拥有数据的方式
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u64,
}

impl User {
    fn new(username: &str, email: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            active: true,
            login_count: 0,
        }
    }
}

fn main() {
    // ==== 命名字段结构体 ====
    let rect1 = Rectangle::new(10.0, 5.0);
    println!("rect1 = {:?}", rect1);        // Debug 输出
    println!("rect1 = {:#?}", rect1);       // 格式化 Debug 输出
    println!("面积：{}", rect1.area());
    println!("周长：{}", rect1.perimeter());
    println!("描述：{}", rect1.describe());

    let rect2 = Rectangle::new(8.0, 3.0);
    println!("rect1 能容纳 rect2：{}", rect1.can_hold(&rect2));

    // 可变结构体：使用 &mut self 方法
    let mut rect3 = Rectangle::new(4.0, 2.0);
    println!("缩放前：{}", rect3.describe());
    rect3.scale(2.0);
    println!("缩放后：{}", rect3.describe());

    // 关联函数创建正方形
    let sq = Rectangle::square(5.0);
    println!("正方形：{}", sq.describe());

    // ==== 结构体更新语法 ====
    // 使用 ..base 语法复用另一个结构体的剩余字段
    let user1 = User::new("alice", "alice@example.com");
    let user2 = User {
        email: String::from("bob@example.com"),
        username: String::from("bob"),
        ..user1 // 其余字段从 user1 复制（注意：会移动不实现 Copy 的字段）
    };
    println!("user2: {:?}", user2);

    // ==== 元组结构体 ====
    let p = Point(3.0, 4.0);
    println!("点坐标：({}, {})", p.0, p.1);

    let red = Color::new(255, 0, 0);
    println!("颜色：{:?}，十六进制：{}", red, red.to_hex());

    // 解构元组结构体
    let Color(r, g, b) = red;
    println!("R={}, G={}, B={}", r, g, b);

    // ==== 单元结构体 ====
    let _marker = AlwaysEqual;
    println!("单元结构体已创建（无字段）");

    // ==== dbg! 宏（调试输出，打印到 stderr 并返回值）====
    let rect4 = Rectangle::new(dbg!(3.0 * 2.0), 4.0);
    dbg!(&rect4); // dbg! 借用并打印
}
