// 05_ownership_and_borrowing.rs
// 主题：所有权与借用（Ownership & Borrowing）
//
// 所有权（Ownership）是 Rust 最核心的特性，它让 Rust 无需垃圾回收器
// 就能保证内存安全。
//
// 所有权三条规则：
//   1. Rust 中每个值都有一个所有者（owner）。
//   2. 同一时刻，值只能有一个所有者。
//   3. 当所有者离开作用域，值会被自动丢弃（drop）。
//
// 借用（Borrowing）：通过引用（&）使用值而不获取所有权。
//   - 不可变引用 &T：可以同时存在多个。
//   - 可变引用 &mut T：同一时刻只能存在一个，且不能与不可变引用共存。

fn main() {
    // ==== 所有权基础 ====

    // 栈上数据（如整数）实现了 Copy trait，赋值时会复制
    let x = 5;
    let y = x; // x 的值被复制到 y，x 仍然有效
    println!("Copy 类型：x={}, y={}", x, y);

    // 堆上数据（如 String）赋值时会发生所有权转移（move）
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权转移给 s2，s1 不再有效
    // println!("{}", s1); // 编译错误：s1 已被移动（moved）
    println!("移动后 s2 = {}", s2);

    // 使用 clone() 进行深拷贝（复制堆上数据）
    let s3 = String::from("world");
    let s4 = s3.clone(); // 深拷贝，s3 和 s4 都有效
    println!("clone：s3={}, s4={}", s3, s4);

    // ==== 函数与所有权 ====

    // 将值传给函数会发生所有权转移（对堆数据）或复制（对 Copy 类型）
    let s = String::from("ownership");
    takes_ownership(s); // s 的所有权移入函数，之后 s 无效
    // println!("{}", s); // 编译错误

    let n = 5;
    makes_copy(n); // i32 是 Copy 类型，n 之后仍然有效
    println!("n 仍然有效：{}", n);

    // 函数返回值也可以转移所有权
    let s5 = gives_ownership(); // 函数将所有权转移给 s5
    println!("从函数获得所有权：{}", s5);

    let s6 = String::from("hello");
    let s7 = takes_and_gives_back(s6); // s6 移入函数，函数再返回所有权给 s7
    println!("转移并归还所有权：{}", s7);

    // ==== 借用（Borrowing）====
    // 通过引用（&）借用值，不转移所有权

    let s8 = String::from("hello");
    let len = calculate_length(&s8); // 传递引用，s8 所有权不变
    println!("'{}' 的长度是 {}", s8, len); // s8 在这里仍然有效

    // ---- 不可变引用（Immutable Reference）----
    // 同一时刻可以有任意多个不可变引用
    let s9 = String::from("Rust");
    let r1 = &s9;
    let r2 = &s9;
    println!("多个不可变引用：r1={}, r2={}", r1, r2);

    // ---- 可变引用（Mutable Reference）----
    // 同一时刻只能有一个可变引用（防止数据竞争）
    let mut s10 = String::from("hello");
    {
        let r3 = &mut s10; // 可变引用
        r3.push_str(", world");
        println!("可变引用修改后：{}", r3);
    } // r3 在此离开作用域，可以再次借用

    let r4 = &mut s10; // 可以再次创建可变引用
    println!("再次可变借用：{}", r4);

    // 可变引用与不可变引用不能同时存在
    // （在不可变引用最后一次使用之后，可以创建可变引用）
    let mut s11 = String::from("hello");
    let imm_ref = &s11;
    println!("不可变引用：{}", imm_ref); // imm_ref 最后一次使用
    // 不可变引用的生命周期在此结束（NLL: Non-Lexical Lifetimes）
    let mut_ref = &mut s11; // 现在可以创建可变引用
    mut_ref.push_str("!");
    println!("可变引用修改后：{}", mut_ref);

    // ==== 悬垂引用（Dangling Reference）——Rust 防止这种情况 ====
    // 在 Rust 中，编译器确保引用永远不会成为悬垂引用。
    // 以下代码会导致编译错误（已注释掉）：
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // s 在函数结束时被 drop，引用将悬垂
    // }

    println!("\n所有权与借用示例完成！");
}

// 接受 String 所有权（所有权被转移进来，函数结束时字符串被 drop）
fn takes_ownership(some_string: String) {
    println!("takes_ownership 获得：{}", some_string);
} // some_string 在此被 drop

// 接受 i32（Copy 类型，只复制值）
fn makes_copy(some_integer: i32) {
    println!("makes_copy 获得：{}", some_integer);
}

// 返回 String，将所有权转移给调用者
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回，所有权转移
}

// 接受并归还 String 所有权
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 直接返回，所有权转移回调用者
}

// 通过引用借用 String（不转移所有权）
fn calculate_length(s: &String) -> usize {
    s.len()
} // s 只是引用，不 drop 实际数据
