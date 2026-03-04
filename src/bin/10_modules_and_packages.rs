// 10_modules_and_packages.rs
// 主题：模块系统与包管理
//
// Rust 的代码组织系统：
//   - 包（Package）：一个 Cargo 项目，包含 Cargo.toml。
//   - Crate：编译单元。二进制 crate（有 main）或库 crate（无 main）。
//   - 模块（Module）：用 mod 关键字定义，控制代码的组织和私有性。
//   - 路径（Path）：用 :: 分隔的模块访问路径。
//   - use：将路径引入作用域。
//   - pub：使项目对外可见（默认私有）。

// ==== 在同一文件中定义模块（内联模块）====
// 模块可以嵌套
mod geometry {
    // 默认私有：只有本模块和子模块可以访问
    // 使用 pub 使其对外可见

    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }

        pub fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }

    pub mod shapes {
        use super::Point; // super 指向父模块（geometry）

        pub struct Circle {
            pub center: Point,
            pub radius: f64,
        }

        impl Circle {
            pub fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle {
                    center: Point::new(x, y),
                    radius,
                }
            }

            pub fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }

            // 私有方法：只在模块内部使用
            fn _internal_check(&self) -> bool {
                self.radius > 0.0
            }
        }

        pub struct Rectangle {
            pub top_left: Point,
            pub width: f64,
            pub height: f64,
        }

        impl Rectangle {
            pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rectangle {
                Rectangle {
                    top_left: Point::new(x, y),
                    width: w,
                    height: h,
                }
            }

            pub fn area(&self) -> f64 {
                self.width * self.height
            }
        }
    }
}

// ==== 嵌套模块的访问 ====
// 使用绝对路径（从 crate 根开始）
// 使用相对路径（从当前模块开始）

// use 将路径引入作用域
use geometry::Point;
use geometry::shapes::{Circle, Rectangle};
// 也可以用 glob 引入全部公有项（不推荐，可能导致命名冲突）
// use geometry::shapes::*;

// ==== 使用 as 重命名 ====
use std::collections::HashMap as Map;
use std::fmt::Display as Fmt;

// ==== 嵌套 use（同一模块下的多个项）====
use std::io::{self, Write}; // std::io 和 std::io::Write

// ==== 标准库中的模块示例 ====
use std::collections::HashSet;

fn demonstrate_std_modules() {
    // std::env：环境变量
    let _args: Vec<String> = std::env::args().collect();

    // std::fs：文件系统（这里不实际操作文件，仅展示引用方式）
    // use std::fs;

    // std::time：时间
    let now = std::time::Instant::now();
    let _elapsed = now.elapsed();
}

// ==== 重导出（pub use）====
// 使外部代码可以通过更短的路径访问内部项
mod math {
    pub mod algebra {
        pub fn add(a: i32, b: i32) -> i32 { a + b }
        pub fn mul(a: i32, b: i32) -> i32 { a * b }
    }
}
// 将 math::algebra 的函数重导出到 math 级别
pub use math::algebra::add as math_add;

fn main() {
    // ==== 使用模块中的类型 ====
    // 通过完整路径
    let p1 = geometry::Point::new(0.0, 0.0);
    let p2 = geometry::Point::new(3.0, 4.0);
    println!("距离：{}", p1.distance(&p2));

    // 通过 use 引入后直接使用
    let p3 = Point::new(1.0, 1.0);
    println!("p3: ({}, {})", p3.x, p3.y);

    let c = Circle::new(0.0, 0.0, 5.0);
    println!("圆面积：{:.2}", c.area());

    let r = Rectangle::new(0.0, 0.0, 4.0, 3.0);
    println!("矩形面积：{:.2}", r.area());

    // ==== 使用重命名后的类型 ====
    let mut map: Map<String, i32> = Map::new();
    map.insert(String::from("one"), 1);
    map.insert(String::from("two"), 2);
    println!("HashMap（重命名为 Map）：{:?}", map);

    // ==== HashSet ====
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    println!("HashSet：{:?}", set);

    // ==== std::io 使用 ====
    // 通过 io::stdout() 直接访问 std::io::stdout
    let stdout = io::stdout();
    let _ = stdout; // 避免未使用警告

    // flush stdout（确保缓冲区内容被写出）
    print!("标准输出刷新：");
    io::stdout().flush().unwrap();
    println!("完成");

    // ==== 重导出的函数 ====
    println!("重导出函数 math_add(3, 4) = {}", math_add(3, 4));

    demonstrate_std_modules();

    // ==== 模块路径总结 ====
    // crate::     绝对路径（从 crate 根）
    // super::     父模块
    // self::      当前模块
    println!("\n模块系统示例完成！");
}

// ==== 函数使用 Display trait ====
fn print_value(value: impl Fmt) {
    println!("值：{}", value);
}
