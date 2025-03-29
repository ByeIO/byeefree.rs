#![allow(unused)]

//! 使用tokio框架进行多线程操作

// 多线程
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 任务1
    let task1 = tokio::spawn(async {
        loop {
            println!("hello");
            sleep(Duration::from_secs(1)).await;
        }
    });

    // 任务2
    let task2 = tokio::spawn(async {
        loop {
            println!("hi");
            sleep(Duration::from_secs(1)).await;
        }
    });

    // 添加到任务调度器
    let _ = tokio::join!(task1, task2);
}
