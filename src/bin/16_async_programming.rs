// 16_async_programming.rs
// 主题：Rust 中的异步编程（async/await）
//
// 异步编程允许程序在等待 I/O 等操作时不阻塞线程，
// 从而高效地处理大量并发任务。
//
// Rust 的 async/await 基础概念：
//   - async fn：声明一个异步函数，返回 impl Future<Output = T>
//   - .await：在异步上下文中等待 Future 完成
//   - Future：代表一个可能尚未完成的值（惰性求值）
//   - Runtime：异步运行时（如 tokio），负责调度和执行 Future
//
// 本文件使用 tokio 作为异步运行时。
// 需要在 Cargo.toml 中添加：
//   [dependencies]
//   tokio = { version = "1", features = ["full"] }
//
// 运行方式：cargo run --bin 16_async_programming

use std::time::Duration;
use tokio::time::sleep;

// ==== 基本 async fn ====
// async fn 声明一个异步函数，返回 Future，在 .await 时执行
async fn say_hello() {
    println!("你好，异步 Rust！");
}

// ==== 模拟异步 I/O 操作 ====
// 实际应用中这里可能是网络请求、文件读取等
async fn fetch_data(id: u32) -> String {
    // tokio::time::sleep 是异步的，不会阻塞线程
    sleep(Duration::from_millis(100)).await;
    format!("数据-{}", id)
}

// ==== 返回 Result 的异步函数 ====
async fn parse_number(s: &str) -> Result<i32, String> {
    sleep(Duration::from_millis(10)).await; // 模拟异步延迟
    s.parse::<i32>().map_err(|e| e.to_string())
}

// ==== 顺序执行多个异步任务 ====
async fn sequential_tasks() {
    println!("\n-- 顺序执行 --");
    let start = std::time::Instant::now();

    let d1 = fetch_data(1).await; // 等待第一个完成，再执行第二个
    let d2 = fetch_data(2).await;
    let d3 = fetch_data(3).await;

    println!("顺序结果：{}, {}, {}", d1, d2, d3);
    println!("顺序耗时：{:?}（约 300ms）", start.elapsed());
}

// ==== 并发执行多个异步任务（tokio::join!）====
// join! 宏同时启动多个 Future，等待全部完成
// 比顺序执行快得多（类似并发，但在同一线程）
async fn concurrent_tasks() {
    println!("\n-- 并发执行（join!）--");
    let start = std::time::Instant::now();

    // tokio::join! 并发等待所有 future 完成
    let (d1, d2, d3) = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
    );

    println!("并发结果：{}, {}, {}", d1, d2, d3);
    println!("并发耗时：{:?}（约 100ms）", start.elapsed());
}

// ==== tokio::spawn：在后台生成异步任务 ====
// spawn 将任务提交给运行时在后台执行，立即返回 JoinHandle
async fn spawned_tasks() {
    println!("\n-- spawn 后台任务 --");

    // spawn 创建独立任务，可在多线程运行时中并行执行
    let handle1 = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        println!("  后台任务 1 完成");
        "任务1结果"
    });

    let handle2 = tokio::spawn(async {
        sleep(Duration::from_millis(30)).await;
        println!("  后台任务 2 完成");
        "任务2结果"
    });

    // 等待任务完成并获取结果（Result<T, JoinError>）
    let r1 = handle1.await.unwrap();
    let r2 = handle2.await.unwrap();
    println!("spawn 结果：{}, {}", r1, r2);
}

// ==== tokio::select!：等待多个 Future，取最先完成的那个 ====
async fn select_example() {
    println!("\n-- select!（竞争执行）--");

    // 两个竞争的 Future，只取最先完成的
    tokio::select! {
        result = fetch_data(10) => {
            println!("fetch_data 先完成：{}", result);
        }
        _ = sleep(Duration::from_millis(50)) => {
            println!("超时！（50ms 内未完成）");
        }
    }
}

// ==== async 闭包与 async 块 ====
async fn async_blocks() {
    println!("\n-- async 块 --");

    // async 块：创建一个匿名的 Future
    let future = async {
        sleep(Duration::from_millis(10)).await;
        42 // 返回值
    };

    let result = future.await; // 执行并等待 async 块
    println!("async 块结果：{}", result);
}

// ==== 错误处理 in async ====
async fn async_error_handling() {
    println!("\n-- async 错误处理 --");

    // async 函数可以返回 Result，使用 ? 传播错误
    async fn risky_op(input: &str) -> Result<i32, String> {
        let n = parse_number(input).await?; // ? 在 async 中同样适用
        Ok(n * 2)
    }

    match risky_op("21").await {
        Ok(n) => println!("risky_op('21') = {}", n),
        Err(e) => println!("错误：{}", e),
    }
    match risky_op("abc").await {
        Ok(n) => println!("结果：{}", n),
        Err(e) => println!("错误：{}", e),
    }
}

// ==== tokio::time::timeout：为 Future 设置超时 ====
async fn timeout_example() {
    println!("\n-- 超时控制 --");

    let result = tokio::time::timeout(
        Duration::from_millis(50),
        fetch_data(99), // 这个会在 100ms 后完成，超过超时时间
    )
    .await;

    match result {
        Ok(data) => println!("在超时前完成：{}", data),
        Err(_) => println!("操作超时！"),
    }
}

// ==== 主函数：#[tokio::main] 属性宏 ====
// #[tokio::main] 将 main 函数变成异步函数，并启动 tokio 运行时
// 等价于：fn main() { tokio::runtime::Runtime::new().unwrap().block_on(async { ... }) }
#[tokio::main]
async fn main() {
    println!("==== Rust 异步编程示例 ====\n");

    // 基本 async/await
    say_hello().await;

    // 顺序 vs 并发
    sequential_tasks().await;
    concurrent_tasks().await;

    // 后台任务（spawn）
    spawned_tasks().await;

    // select!（竞争）
    select_example().await;

    // async 块
    async_blocks().await;

    // 错误处理
    async_error_handling().await;

    // 超时控制
    timeout_example().await;

    println!("\n==== 异步编程总结 ====");
    println!("1. async fn 声明异步函数，返回 Future");
    println!("2. .await 在异步上下文中等待 Future 完成");
    println!("3. tokio::join! 并发执行多个任务");
    println!("4. tokio::spawn 在后台生成独立任务");
    println!("5. tokio::select! 等待多个 Future，取最先完成的");
    println!("6. tokio::time::timeout 为任务设置超时");
    println!("7. async 代码中可以正常使用 ? 进行错误传播");
}
