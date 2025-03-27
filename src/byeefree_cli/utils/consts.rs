#![allow(dead_code)]

// 版本号
pub const VERSION : &str = "0.0.1";

// 编译日期(需要build.rs编译期注入)
pub const BUILD_TIMESTAMP : &str =  env!("BUILD_TIMESTAMP");

// git哈希值
pub const GIT_HASH : &str = env!("GIT_HASH");
