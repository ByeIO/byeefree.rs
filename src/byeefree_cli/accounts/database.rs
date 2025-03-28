#![allow(unused)]

//! 处理数据库交互

// 嵌入文件夹
use rust_embed::Embed;

/// 嵌入数据库文件
#[derive(Embed)]
#[folder = "./assets/database_login.db"]
#[prefix = "db/"]
struct AccountDatabase;
