//! 命令

// 1. 命令定义
pub mod defines;
pub use defines::{
    ByeefreeCommand, Commands, AccountCommands,
    UbuntuCommands, UtilCommands, ServiceCommands, 
    command_parser,
};

// 2. ubuntu命令处理
pub mod ubuntu;
