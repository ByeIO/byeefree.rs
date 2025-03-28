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

/// 最顶层byeefree命令
#[derive(Parser, Debug)]
#[command(name = "byeefree")]
#[command(bin_name = "byeefree")]
#[command(version, about, long_about = None)]
pub struct ByeefreeCommand {
    // 使用Subcommand来定义子命令
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// 定义子命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 账户
    Account {
        #[command(subcommand)]
        command: Option<AccountCommands>,
    },
    
    /// 查看日志
    Log,
    
    /// 实用工具
    Util {
        #[command(subcommand)]
        command: Option<UtilCommands>,
    },
    
    /// 机器人框架
    Rosette,
    
    /// mosh远程连接
    Mosh,
    
    /// rsync远程文件同步
    Rsync,
    
    /// ubuntu容器(wasm)
    Ubuntu{
        /// shell命令（直接使用时）
        #[arg(short = 'c')]
        commands: Option<String>,
        
        /// 子命令
        #[command(subcommand)]
        command: Option<UbuntuCommands>,
    },
}

/// 账户子命令
#[derive(Subcommand, Debug)]
pub enum AccountCommands {
    /// 登录账户
    Login {
        /// 账户名称
        name: Option<String>,
    },
    /// 注册账户
    Register {
        /// 账户名称
        name: Option<String>,
    },
}

/// ubuntu虚拟机命令(ubuntu run -c "命令"或者ubuntu -c "命令")
#[derive(Subcommand, Debug)]
pub enum UbuntuCommands {
    /// 运行命令（显式使用run子命令）
    Run {
        /// shell命令
        #[arg(short = 'c', last = true)]
        commands: Option<String>,
    },
}

/// 实用工具命令
#[derive(Subcommand, Debug)]
pub enum UtilCommands {
    /// 后台服务管理
    Service{
        #[command(subcommand)]
        action: Option<ServiceCommands>,
    },
    /// 系统使用情况
    Sysinfo,
    /// 设备管理
    Device,
    /// 角色设置
    Role {
        /// 角色类型
        role: String,
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
