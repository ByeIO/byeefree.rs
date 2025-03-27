#![allow(unused)]

//! 使用clap构建byeefree命令

// 命令行解析
use clap::{Parser, Subcommand, Args};
// 环境日志
use env_logger::{Builder, Target};
// 日志
use log::{info, error, LevelFilter};

// 标准库
use std::time::Instant;

// 版本号
const VERSION : &str = "0.0.1";

/// 命令解析器入口
pub fn command_parser(){
    // 解析命令
    let args = ByeefreeCommand::parse();
}

/// Byeefree命令的参数
#[derive(Parser, Debug)]
#[command(name = "byeefree")]
#[command(bin_name = "byeefree")]
#[command(version, about, long_about = None)]
pub struct ByeefreeCommand {
    // 使用Subcommand来定义子命令
    #[command(subcommand)]
    command: Commands,
}

/// 定义子命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 账户
    Account {
        #[command(subcommand)]
        action: AccountCommands,
    },
    
    /// 查看日志
    Log,
    
    /// 实用工具
    Util {
        #[command(subcommand)]
        util: UtilCommands,
    },
    
    /// 机器人框架
    Rosette,
    
    /// mosh远程连接
    Mosh,
    
    /// rsync远程文件同步
    Rsync,
}

/// 账户子命令
#[derive(Subcommand, Debug)]
pub enum AccountCommands {
    /// 登录账户
    Login {
        /// 账户名称
        account_name: String,
    },
    /// 注册账户
    Register {
        /// 账户名称
        account_name: String,
    },
}

/// ubuntu虚拟机命令
#[derive(Parser, Debug)]
pub struct UbuntuCommands {
    /// shell命令
    #[arg(short = 'c', last = true)]
    commands: String,
}

/// 实用工具命令
#[derive(Subcommand, Debug)]
pub enum UtilCommands {
    /// 后台服务管理
    Service{
        #[command(subcommand)]
        action: ServiceCommands,
    },
    /// 系统使用情况
    Sysinfo,
    /// 设备管理
    Device,
    /// 角色设置
    Role {
        /// 角色类型
        role_type: String,
    },
}

/// 服务管理命令
#[derive(Subcommand, Debug)]
pub enum ServiceCommands {
    /// 安装服务
    Install,
    /// 卸载服务
    Uninstall,
    /// 查看服务状态
    Status,
    /// 重启服务
    Restart,
}
