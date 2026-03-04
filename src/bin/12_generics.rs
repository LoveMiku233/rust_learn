// 12_generics.rs
// 主题：泛型（Generics）
//
// 泛型允许我们编写可复用的代码，适用于多种数据类型，
// 而不需要重复实现同样的逻辑。
//
// Rust 中泛型出现在：
//   - 函数签名（fn foo<T>(x: T) -> T）
//   - 结构体定义（struct Point<T>）
//   - 枚举定义（enum Option<T>）
//   - impl 块（impl<T> Point<T>）
//
// 泛型是零成本抽象：Rust 在编译时进行"单态化"（monomorphization），
// 为每种具体类型生成对应的代码，运行时无额外开销。

// ==== 泛型函数 ====
// T 是类型参数，可以是任意名称，惯例用大写字母
// 需要 T: PartialOrd 约束，因为 > 操作需要类型支持比较
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 多个泛型类型参数
fn first_and_last<T>(list: &[T]) -> Option<(&T, &T)> {
    if list.is_empty() {
        None
    } else {
        Some((&list[0], &list[list.len() - 1]))
    }
}

// ==== 泛型结构体 ====
// 结构体中的每个字段都可以有不同类型参数
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// T 和 U 是不同类型参数
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// ==== impl 块中的泛型 ====
// 为所有类型 T 实现方法（需要在 impl 后声明 <T>）
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// 只为特定类型实现方法（具体类型的 impl，不需要 <T>）
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// 约束泛型（T 必须实现 std::fmt::Display）
impl<T: std::fmt::Display> Point<T> {
    fn describe(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

// ==== 泛型枚举（标准库中的 Option 和 Result 就是泛型枚举）====
// 自定义泛型枚举示例
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: std::fmt::Display, R: std::fmt::Display> Either<L, R> {
    fn value_str(&self) -> String {
        match self {
            Either::Left(l) => format!("Left({})", l),
            Either::Right(r) => format!("Right({})", r),
        }
    }
}

// ==== 泛型结构体与方法返回不同类型 ====
impl<T, U> MixedPoint<T, U> {
    fn new(x: T, y: U) -> MixedPoint<T, U> {
        MixedPoint { x, y }
    }
}

// ==== 泛型与标准库 ====
// 标准库大量使用泛型：Vec<T>, HashMap<K,V>, Option<T>, Result<T,E>

// 泛型函数：交换两个 Vec 的第一个元素
fn swap_first<T: Clone>(v1: &mut Vec<T>, v2: &mut Vec<T>) {
    if !v1.is_empty() && !v2.is_empty() {
        let temp = v1[0].clone();
        v1[0] = v2[0].clone();
        v2[0] = temp;
    }
}

// ==== 泛型与 where 子句 ====
// 当约束复杂时，可以用 where 子句使签名更清晰
fn print_pair<T, U>(t: T, u: U)
where
    T: std::fmt::Display + std::fmt::Debug,
    U: std::fmt::Display + Clone,
{
    println!("Display：{} | Debug：{:?} | Pair：{}", t, t, u);
}

fn main() {
    // ==== 泛型函数 ====
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大整数：{}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符：{}", largest(&chars));

    let floats = [1.1, 2.2, 0.5, 3.3];
    println!("最大浮点数：{}", largest(&floats));

    // first_and_last
    if let Some((first, last)) = first_and_last(&numbers) {
        println!("第一个：{}，最后一个：{}", first, last);
    }

    // ==== 泛型结构体 ====
    let int_point = Point::new(5, 10);
    println!("整数点：{:?}", int_point);
    println!("x = {}", int_point.x());
    println!("{}", int_point.describe());

    let float_point = Point::new(1.0_f64, 4.0_f64);
    println!("浮点点：{:?}", float_point);
    println!("到原点距离：{:.4}", float_point.distance_from_origin());
    println!("{}", float_point.describe());

    // 混合类型点
    let mixed = MixedPoint::new(5, 3.14);
    println!("混合点：{:?}", mixed);

    // ==== 泛型枚举 ====
    let left: Either<i32, &str> = Either::Left(42);
    let right: Either<i32, &str> = Either::Right("hello");
    println!("Either：{}", left.value_str());
    println!("Either：{}", right.value_str());

    // ==== 泛型与集合操作 ====
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];
    println!("交换前：v1={:?}, v2={:?}", v1, v2);
    swap_first(&mut v1, &mut v2);
    println!("交换后：v1={:?}, v2={:?}", v1, v2);

    // ==== where 子句 ====
    print_pair(42, "hello");
    print_pair("world", 3.14);

    // ==== 标准库的泛型 ====
    // Vec<T>
    let mut generic_vec: Vec<String> = Vec::new();
    generic_vec.push(String::from("泛型"));
    generic_vec.push(String::from("是"));
    generic_vec.push(String::from("强大的"));
    println!("泛型 Vec：{:?}", generic_vec);

    // Option<T>
    let some_int: Option<i32> = Some(42);
    let no_int: Option<i32> = None;
    println!("Option<i32>：{:?}, {:?}", some_int, no_int);

    // Result<T, E>
    let ok_result: Result<i32, String> = Ok(100);
    let err_result: Result<i32, String> = Err(String::from("出错了"));
    println!("Result：{:?}, {:?}", ok_result, err_result);

    println!("\n泛型示例完成！");
}
