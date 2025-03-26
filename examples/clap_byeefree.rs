//! 使用clap构建byeefree命令
use clap::Parser;

const VERSION : &str = "0.0.1";

/// Byeefree命令的参数
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ByeefreeCommand {
    // 默认自动生成version命令打印软件版本
    // 默认自动生成help命令打印帮助信息
    
    
}

fn main() {
    // 解析命令
    let args = Args::parse();

    println!("version:{}", VERSION);
}
