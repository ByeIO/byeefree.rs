#![allow(unused)]

//! byeefree主程序入口

// 内部库
/// 常量
use byeefree_rs::byeefree_cli::utils::consts::{VERSION, BUILD_TIMESTAMP, GIT_HASH};
/// 命令定义
use byeefree_rs::byeefree_cli::commands::command_parser;

// 日志库
use log::{debug, error, log_enabled, info, Level};

// 标准库
use std::env;

// 日志库前端
use env_logger::{Builder, Target};

fn main() {
    // 初始化日志库前端
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
    
    // 欢迎信息
    println!("欢迎使用ByeIO的byeefree命令行工具!");
    println!("软件版本: {}-{}", VERSION, GIT_HASH);
    println!("构建时间: {}", BUILD_TIMESTAMP);
    // 解析命令
    command_parser();
    
    log::info!("初始化完成, 后台自动执行任务程序.");
}
