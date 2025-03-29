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
use super::sysinfo::util_sysinfo_cmd;

/// 命令解析器入口
pub async fn command_parser(){
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
        
        // 处理账户命令
        Some(Commands::Account { command } ) => {
            // 处理子命令
            if let Some(cmd) = command {
                match cmd {
                    AccountCommands::Login { name } => {
                        if let Some(n) = name {
                            println!("登录{}账户成功!", n);
                        }// end if let
                    },
                    AccountCommands::Register { name } => {
                        if let Some(n) = name {
                            println!("注册{}账户成功!", n);
                        } // end if let
                    },
                } // end match
            } // end if let
        },
        
        // 处理实用工具命令
        Some(Commands::Util { command } ) => {
            // 处理子命令
            if let Some(cmd) = command {
                match cmd {
                    // 系统资源情况
                    UtilCommands::Sysinfo => {
                        util_sysinfo_cmd();
                    },
                    // 设备管理
                    UtilCommands::Device => {
                        println!("设备管理");
                    },
                    // 角色设置
                    UtilCommands::Role { role } => {
                        println!("设置{}角色成功", role);
                    }
                    // 服务管理
                    UtilCommands::Service { action } => {
                        if let Some(act) = action {
                            match act {
                                // 安装后台服务
                                ServiceCommands::Install => {
                                    use super::service::util_service_install_cmd;
                                    let _ = util_service_install_cmd();
                                },
                                // 卸载后台服务
                                ServiceCommands::Uninstall => {
                                    use super::service::util_service_uninstall_cmd;
                                    let _ = util_service_uninstall_cmd();
                                }, 
                                // 查看服务状态
                                ServiceCommands::Status => {
                                    use super::service::util_service_status_cmd;
                                    let _ = util_service_status_cmd();
                                },
                                // 重启服务
                                ServiceCommands::Restart => {
                                    use super::service::util_service_restart_cmd;
                                    let _ = util_service_restart_cmd();
                                },
                                // 运行服务(与start有区别)
                                ServiceCommands::Run => {
                                    println!("运行服务...");
                                    use crate::backend::run_backend;
                                    run_backend().await;
                                },
                                
                            } // end match
                        }// end if let
                    },
                } // end match
            }// end if let
        },
        
        // 处理查看日志命令
        Some(Commands::Log) => {
            println!("查看日志!");
        },
        
        // 处理机器人框架命令
        Some(Commands::Rosette) => {
            println!("机器人框架命令");
        },
        
        // 处理mosh远程连接命令
        Some(Commands::Mosh) => {
            println!("远程连接!");
        },
        
        //  处理rsync同步文件命令
        Some(Commands::Rsync) => {
            println!("文件同步!");
        },
        
        // 其他子命令的处理...
        _ => {
            println!("其他命令正在完善中...")
        },
    }// end match
}// end fn
