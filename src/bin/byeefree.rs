#![allow(unused)]

//! byeefree主程序入口

// 内部库
/// 常量
use byeefree_rs::byeefree_cli::utils::consts::{VERSION, BUILD_TIMESTAMP, GIT_HASH};
/// 命令定义
use byeefree_rs::byeefree_cli::commands::command_parser;

// 日志库
use log::{debug, error, log_enabled, info, Level};

// log库和tracing库共存工具
use tracing_log::LogTracer;

// 标准库
use std::env;

// 日志库前端
use env_logger::{Builder, Target};

fn main() {
    // NOTE: wasmtime库初始化tracing, 不能重复初始化.
    
    // 初始化LogTracer以转发log记录
    // LogTracer::init().expect("Failed to set logger");
    
    // 初始化日志库前端
    // let mut builder = Builder::from_default_env();
    // builder.target(Target::Stdout);
    // builder.init();
    
    // 欢迎信息
    println!("欢迎使用ByeIO的byeefree命令行工具!");
    println!("软件版本: {}-{}", VERSION, GIT_HASH);
    println!("构建时间: {}", BUILD_TIMESTAMP);
    
    // 打印欢迎信息
    print_welcome_info();
    
    // 解析命令
    command_parser();
    
    // 写入日志
    log::info!("初始化完成, 后台自动执行任务程序.");
}

fn print_welcome_info(){
    println!("<<Byeefree--空中侦察四旋翼无人机指挥系统>>");
    println!("检查程序完整性...");
    println!("启动主程序...");
    println!("加载后台服务...");
    println!("浏览器访问[https://127.0.0.1:8888]打开系统界面 或者 VSCodium插件通过[wss://127.0.0.1:8888]访问系统.");
}
