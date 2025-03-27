//! 命令

// 1. 命令定义
pub mod defines;
pub use defines::{
    ByeefreeCommand, Commands, AccountCommands,
    UbuntuCommands, UtilCommands, ServiceCommands, 
};

// 2. ubuntu命令处理
pub mod ubuntu;

// 3. 命令解析
pub mod parser;
pub use parser::command_parser;
