#![allow(unused)]

//! 颜色格式化输出日志信息

// 日志库
use log::{debug, error, log_enabled, info, Level};

// 标准库
use std::env;

// 日志库前端
use env_logger::{Builder, Target};

fn main() {
    // 配置日志前端
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    
    // 设置输出到stdout而不是stderr
    builder.init();
    
    // 初始化日志前端
    // env_logger::init();
    
    // 打印日志
    log::debug!("this is a debug {}", "message");
    log::error!("this is printed by default");
    
    if log_enabled!(Level::Info) {
        let x = 3 * 4;
        info!("the answer was: {}", x);
    }
}
