#![allow(unused)]

//! 后台服务总入口

// 内部库
use super::log_time::backend_log_time;

pub fn run_backend(){
    println!("hello from backend");
    
    // 启动时间服务
    backend_log_time();
    
    println!("byeefree backend is running.");
}
