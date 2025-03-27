#![allow(unused)]

//! byeefree主程序入口

// 内部库
/// 常量
use byeefree_rs::byeefree_cli::utils::consts::{VERSION, BUILD_TIMESTAMP, GIT_HASH};
/// 命令定义
use byeefree_rs::byeefree_cli::commands::command_parser;

fn main() {
    // 欢迎信息
    println!("欢迎使用ByeIO的byeefree命令行工具!");
    println!("软件版本: {}-{}", VERSION, GIT_HASH);
    println!("构建时间: {}", BUILD_TIMESTAMP);
    // 解析命令
    command_parser();
}
