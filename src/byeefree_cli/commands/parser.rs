#![allow(unused)]

//! 解析命令

// 命令行解析
use clap::{Parser, Subcommand, Args};

// 内部库
use super::{
    ByeefreeCommand, Commands, AccountCommands,
    UbuntuCommands, UtilCommands, ServiceCommands, 
};
use super::ubuntu::ubuntu_command;

/// 命令解析器入口
pub fn command_parser(){
    // 解析命令
    let args = super::ByeefreeCommand::parse();
    
    match args.command {
        
        // 处理ubuntu子命令
        Some(Commands::Ubuntu { commands, command }) => {
            // 处理直接使用 -c 参数的情况
            if let Some(cmd) = commands {
                println!("执行命令: {}", cmd);
                ubuntu_command(cmd);
            }

            // 处理 run 子命令的情况
            if let Some(subcmd) = command {
                match subcmd {
                    UbuntuCommands::Run { commands: run_cmd } => {
                        if let Some(cmd) = run_cmd {
                            println!("通过run执行命令: {}", cmd);
                            ubuntu_command(cmd);
                        }
                    }
                }// end match
            }// end if
        },
        // 其他子命令的处理...
        _ => {}
    }// end match
}// end fn
