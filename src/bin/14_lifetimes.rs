// 14_lifetimes.rs
// 主题：生命周期（Lifetimes）
//
// 生命周期是 Rust 所有权系统的一部分，用于确保引用在其所指向的数据有效时才能使用。
// 大多数情况下生命周期是隐式推断的，但有时需要显式标注。
//
// 生命周期标注不改变引用的实际生命周期，
// 只是告诉编译器多个引用的生命周期之间的关系。
//
// 语法：'a（单引号加标识符）
//   - 函数：fn foo<'a>(x: &'a str, y: &'a str) -> &'a str
//   - 结构体：struct Foo<'a> { part: &'a str }

// ==== 为什么需要生命周期？====
// 防止悬垂引用（dangling reference）：
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // 错误：s 在函数结束时被 drop，引用将悬垂
// }

// ==== 函数中的生命周期标注 ====
// 返回的引用与输入引用中生命周期较短的那个相同
// 'a 表示：返回的 &str 与 x、y 中寿命较短的那个一样长
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 当参数之一不参与返回值时，不需要相同生命周期标注
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[..i]; }
    }
    s
}

// ==== 结构体中的生命周期标注 ====
// 当结构体持有引用时，必须标注生命周期
// 'a 表示：结构体的实例不能比字段 part 指向的数据存活更长
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str, // 持有一个字符串切片引用
}

// 为持有引用的结构体实现方法
impl<'a> Excerpt<'a> {
    // 方法的生命周期通常由省略规则自动推断
    fn level(&self) -> i32 { 3 }

    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self.part // 返回 self 的引用，生命周期与 self 相同
    }
}

// ==== 生命周期省略规则（Lifetime Elision Rules）====
// Rust 编译器有三条规则自动推断生命周期，不需要手动标注：
//   规则1：每个引用参数都有自己独立的生命周期。
//   规则2：如果只有一个输入生命周期参数，它被赋给所有输出生命周期。
//   规则3：如果方法有 &self 或 &mut self 参数，self 的生命周期被赋给所有输出生命周期。

// 下面这个函数签名根据规则2可以省略生命周期标注：
fn first_char(s: &str) -> &str { // 等价于 fn first_char<'a>(s: &'a str) -> &'a str
    if s.is_empty() { "" } else { &s[..1] }
}

// ==== 'static 生命周期 ====
// 'static 表示整个程序运行期间都有效（如字符串字面量）
fn get_static() -> &'static str {
    "我是字面量，生命周期是 'static"
}

// ==== 泛型 + Trait Bound + 生命周期（组合使用）====
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    announcement: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("公告：{}", announcement);
    if x.len() > y.len() { x } else { y }
}

// ==== 多个生命周期参数 ====
// 'b: 'a 表示 'b 的生命周期至少和 'a 一样长
struct Manager<'a, 'b: 'a> {
    important: &'a str,
    extra: &'b str,
}

impl<'a, 'b: 'a> Manager<'a, 'b> {
    fn new(important: &'a str, extra: &'b str) -> Manager<'a, 'b> {
        Manager { important, extra }
    }
    fn important(&self) -> &'a str {
        self.important
    }
}

fn main() {
    // ==== 基本生命周期示例 ====
    println!("==== longest 函数 ====");
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        // result 在这里有效，因为两个字符串都在这个作用域内存活
        println!("最长的字符串：{}", result);
    }
    // 注意：result 借用自 string2，不能在 string2 的作用域外使用
    // 以下代码会编译错误：
    // println!("{}", result); // string2 已经 drop

    // ==== 结构体生命周期 ====
    println!("\n==== 结构体生命周期 ====");
    let novel = String::from("从前，在很远的地方。这是一个故事。");
    let first_sentence;
    {
        let i = novel.find('。').unwrap_or(novel.len());
        first_sentence = &novel[..i];
    }
    let excerpt = Excerpt { part: first_sentence };
    println!("摘录：{:?}", excerpt);
    println!("级别：{}", excerpt.level());
    let returned = excerpt.announce_and_return("重要更新");
    println!("返回：{}", returned);

    // ==== 省略的生命周期 ====
    println!("\n==== 省略的生命周期 ====");
    let s = String::from("hello world");
    println!("first_word：{}", first_word(&s));
    println!("first_char：{}", first_char(&s));

    // ==== 'static 生命周期 ====
    println!("\n==== 'static 生命周期 ====");
    let static_str = get_static();
    println!("{}", static_str);

    // 字符串字面量都是 'static
    let s: &'static str = "我也是 'static 生命周期";
    println!("{}", s);

    // ==== 泛型 + 生命周期组合 ====
    println!("\n==== 泛型 + 生命周期 ====");
    let s1 = String::from("hello, world");
    let s2 = String::from("hi");
    let result2 = longest_with_announcement(&s1, &s2, "比较两个字符串");
    println!("最长：{}", result2);

    // ==== 多个生命周期参数 ====
    println!("\n==== 多个生命周期参数 ====");
    let important_data = String::from("重要数据");
    let extra_data = String::from("附加数据");
    let manager = Manager::new(&important_data, &extra_data);
    println!("重要信息：{}", manager.important());
    println!("附加信息：{}", manager.extra);

    // ==== 生命周期与迭代器 ====
    println!("\n==== 常见生命周期场景 ====");
    // 返回字符串切片的函数必须确保切片的数据仍然有效
    let data = vec!["apple", "banana", "cherry"];
    let longest_fruit = data.iter().max_by_key(|s| s.len()).unwrap();
    println!("最长水果名：{}", longest_fruit);

    println!("\n生命周期示例完成！");
}
