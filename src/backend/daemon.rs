#![allow(unused)]

//! 后台服务总入口

// 内部库
use super::log_time::backend_log_time;
use super::website::backend_website;

pub async fn run_backend(){
    println!("hello from backend");
    
    // // 启动时间服务
    // backend_log_time().await;
    
    // // 启动网页服务
    // backend_website().await;
    
    // 并行启动时间服务和网页服务
    tokio::join!(backend_log_time(), backend_website());
    
    println!("byeefree backend is running.");
}
