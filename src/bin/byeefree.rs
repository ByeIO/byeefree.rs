#![allow(unused)]

//! byeefree主程序入口

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 内部库
/// 版本号
use byeefree_rs::byeefree_cli::utils::consts::VERSION;
/// 命令定义
use byeefree_rs::byeefree_cli::commands::command_parser;

fn main() {
    command_parser();
    // 打印版本
    println!("version:{}", VERSION);
}
