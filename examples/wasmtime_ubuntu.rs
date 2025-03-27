#![allow(unused)]

//! 封装wasmtime_cli命令

// 标准库
use std::path::Path;
use std::error::Error;
use std::io::Write;
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};

// POSIX接口
use libc::{self, c_int, dup, dup2, STDERR_FILENO, STDOUT_FILENO};

// 错误处理
use anyhow::Result;

// 命令行参数解析
use clap::{Parser, Subcommand};

// 处理复杂命令
use shell_words;

// wasmtime-cli的接口(魔改wasmtime_cli库)
use wasmtime_cli::cli::Wasmtime;

// 嵌入文件
use embed_file::embed_bytes;

// 临时文件
use tempfile::tempdir;

/* start 封装wasmtime-cli */
/// 可以使用例如`WasmtimeCli::run("run test.wasm")?;`达到命令行的`wasmtime run test.wasm`同样效果.
/// 类似于std::process::Command执行shell命令的效果, 但是无需安装wasmtime.
pub struct WasmtimeCli;
impl WasmtimeCli {
    pub fn run(command_line: &str) -> Result<()> {
        let args = shell_words::split(command_line)?;
        let full_args = std::iter::once("wasmtime".to_string()).chain(args);
        let wasmtime = Wasmtime::try_parse_from(full_args)?;
        wasmtime.execute()
    }
}
/* end 封装wasmtime-cli */

fn main()->anyhow::Result<(), anyhow::Error>{
    // 修改后的代码段
    use stdio_override::{StdoutOverride, StderrOverride};
    
    // 嵌入二进制文件到编译产物中 
    let mut ubuntu_wasm_bytes = embed_bytes!("../assets/ubuntu2204.wasm");
    
    // 构造临时文件
    let temp_dir = tempdir().expect("创建临时目录失败");
    let ubuntu_wasm_file_path = temp_dir.path().join("ubuntu2204.wasm");
    
    // 将嵌入的字节写入临时文件
    let mut ubuntu_wasm_file = std::fs::File::create(&ubuntu_wasm_file_path).expect("创建临时文件失败");
    ubuntu_wasm_file.write_all(&ubuntu_wasm_bytes).expect("写入临时文件失败");
    
    // 获取当前目录
    let current_dir = std::env::current_dir()?.display().to_string();

    // 创建临时文件捕获输出
    let output_tempdir = tempdir()?;
    let output_path = output_tempdir.path().join("wasm_output.txt");
    
    // // 重定向标准输出和错误到文件 
    // let guard_stdout = StdoutOverride::from_file(&output_path)?;
    // let guard_stderr = StderrOverride::from_file(&output_path)?;

    // 等效于`wasmtime run --dir $PWD::/home ubuntu2204.wasm bash -c 'ls && echo "hello from ubuntu" && uname -a'`
    let run_result = WasmtimeCli::run(&format!(
        r#"run --dir {}::/home {} bash -c 'ls && echo "hello from ubuntu" && uname -a'"#,
        current_dir,
        ubuntu_wasm_file_path.display()
    ));
    
    // // 显式释放守卫以恢复原始输出（作用域结束时也会自动释放）
    // drop(guard_stdout);
    // drop(guard_stderr);
    
    // 读取并检查输出
    let output = std::fs::read_to_string(&output_path)?;
    println!("#######\n{}######\n", output);
    if output.contains("Linux localhost 6.1.0") {
        println!("执行成功，已找到stdout字符串，特征识别成功");
    }
    
    run_result?;
    
    anyhow::Ok(())
}
