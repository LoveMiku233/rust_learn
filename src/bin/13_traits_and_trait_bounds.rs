// 13_traits_and_trait_bounds.rs
// 主题：Trait 与 Trait Bound
//
// Trait 定义了一组类型可以共享的行为（类似其他语言的接口）。
// Trait Bound 约束泛型类型必须实现某些 trait。
//
// 关键概念：
//   - trait 定义：定义抽象行为（方法签名 + 可选默认实现）
//   - impl Trait for Type：为类型实现 trait
//   - Trait Bound：fn foo<T: TraitA + TraitB>(x: T)
//   - impl Trait 语法：fn foo(x: impl TraitA) -> impl TraitB
//   - Trait 对象（dyn Trait）：动态分发，运行时多态

use std::fmt;

// ==== 定义 Trait ====
trait Animal {
    // 必须实现的方法（无默认实现）
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    // 带默认实现的方法（可被覆盖）
    fn describe(&self) -> String {
        format!("{} 发出 '{}' 的声音", self.name(), self.sound())
    }

    // 默认实现可以调用其他方法
    fn introduce(&self) -> String {
        format!("我是一只 {}，{}", self.name(), self.describe())
    }
}

// ==== 为不同类型实现 Trait ====
struct Dog {
    name: String,
    breed: String,
}

struct Cat {
    name: String,
}

struct Duck;

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "汪汪" }
    // 覆盖默认实现
    fn describe(&self) -> String {
        format!("{}（{}犬）发出 '{}' 的声音", self.name, self.breed, self.sound())
    }
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "喵喵" }
    // 使用默认的 describe 实现
}

impl Animal for Duck {
    fn name(&self) -> &str { "鸭子" }
    fn sound(&self) -> &str { "嘎嘎" }
}

// ==== Trait 作为函数参数（impl Trait 语法）====
// 任何实现了 Animal trait 的类型都可以作为参数传入
fn make_sound(animal: &impl Animal) {
    println!("{}", animal.describe());
}

// ==== Trait Bound 语法（等价于 impl Trait，更明确）====
fn introduce_animal<T: Animal>(animal: &T) {
    println!("{}", animal.introduce());
}

// ==== 多个 Trait Bound ====
// 类型必须同时实现多个 trait（用 + 连接）
fn print_and_display<T: fmt::Display + fmt::Debug>(value: T) {
    println!("Display：{}", value);
    println!("Debug：{:?}", value);
}

// ==== where 子句（复杂 Trait Bound 的清晰写法）====
fn compare_and_display<T, U>(t: &T, u: &U)
where
    T: fmt::Display + PartialOrd,
    U: fmt::Display,
{
    println!("T={}, U={}", t, u);
}

// ==== 返回 impl Trait ====
// 函数可以返回实现了某 Trait 的类型，调用者不需要知道具体类型
fn new_animal() -> impl Animal {
    Duck // 返回具体类型，但调用者只看到 Animal trait
}

// ==== Trait 对象（dyn Trait）：运行时多态 ====
// 使用 &dyn Trait 或 Box<dyn Trait> 存储不同类型的 trait 对象
// 适用于需要在运行时确定具体类型的场景（如异构集合）
fn loudest_animal(animals: &[Box<dyn Animal>]) -> &dyn Animal {
    // 通过 trait 对象调用方法（动态分发）
    // 这里简单返回第一个
    animals[0].as_ref()
}

// ==== 标准库常用 Trait ====

// 1. Display（自定义格式化输出）
#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

// 2. Clone + Copy
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 3. PartialEq 和 Eq（相等性比较）
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 4. 运算符重载（通过 Trait 实现）
use std::ops::Add;

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

// 5. Default Trait
#[derive(Debug, Default)]
struct Config {
    width: u32,
    height: u32,
    title: String,
}

fn main() {
    // ==== 基本 Trait 使用 ====
    let dog = Dog { name: String::from("旺财"), breed: String::from("拉布拉多") };
    let cat = Cat { name: String::from("咪咪") };
    let duck = Duck;

    make_sound(&dog);
    make_sound(&cat);
    make_sound(&duck);
    introduce_animal(&duck);

    // ==== Trait 对象（动态分发）====
    println!("\n==== Trait 对象（dyn Trait）====");
    // Box<dyn Animal> 可以存储不同的 Animal 类型
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: String::from("小黑"), breed: String::from("哈士奇") }),
        Box::new(Cat { name: String::from("橘猫") }),
        Box::new(Duck),
    ];

    for animal in &animals {
        println!("{}", animal.describe());
    }

    let first = loudest_animal(&animals);
    println!("第一只：{}", first.name());

    // ==== impl Trait 返回值 ====
    let new = new_animal();
    println!("新动物：{}", new.describe());

    // ==== 标准库 Trait ====
    println!("\n==== 标准库 Trait ====");

    // Display 和 Debug
    let color = Color { r: 255, g: 128, b: 0 };
    println!("Display：{}", color);
    println!("Debug：{:?}", color);

    // Clone 和 Copy
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = v1; // Copy：v1 仍然有效（因为实现了 Copy）
    println!("v1={}, v2={}", v1, v2);

    // 运算符重载（Add）
    let v3 = v1 + v2;
    println!("v1 + v2 = {}, 长度 = {:.4}", v3, v3.length());

    // PartialEq
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };
    println!("p1 == p2：{}", p1 == p2);
    println!("p1 == p3：{}", p1 == p3);

    // Default
    let config = Config::default();
    println!("默认 Config：{:?}", config);
    let custom_config = Config {
        width: 1920,
        height: 1080,
        title: String::from("我的窗口"),
        ..Config::default()
    };
    println!("自定义 Config：{:?}", custom_config);

    // print_and_display（多 Trait Bound）
    println!("\n多 Trait Bound：");
    print_and_display(42);
    print_and_display("hello");

    compare_and_display(&10, &"world");

    println!("\nTrait 示例完成！");
}
