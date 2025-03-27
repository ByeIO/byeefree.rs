#![allow(unused)]

//! ubuntu命令

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
use wasmtime_cli::cli::WasmtimeCli;

// 嵌入文件
use embed_file::embed_bytes;

// 临时文件
use tempfile::tempdir;

pub fn ubuntu_command()->anyhow::Result<(), anyhow::Error>{
    // 修改后的代码段
    use stdio_override::{StdoutOverride, StderrOverride};
    
    // 嵌入二进制文件到编译产物中 
    let mut ubuntu_wasm_bytes = embed_bytes!("../../../assets/ubuntu2204.wasm");
    
    // 构造临时文件
    let temp_dir = tempdir().expect("创建临时目录失败");
    let ubuntu_wasm_file_path = temp_dir.path().join("ubuntu2204.wasm");
    
    // 将嵌入的字节写入临时文件
    let mut ubuntu_wasm_file = std::fs::File::create(&ubuntu_wasm_file_path).expect("创建临时文件失败");
    ubuntu_wasm_file.write_all(&ubuntu_wasm_bytes).expect("写入临时文件失败");
    
    // 获取当前目录
    let current_dir = std::env::current_dir()?.display().to_string();

    // 等效于`wasmtime run --dir $PWD::/home ubuntu2204.wasm bash -c 'ls && echo "hello from ubuntu" && uname -a'`
    let run_result = WasmtimeCli::run(&format!(
        r#"run --dir {}::/home {} bash -c 'ls && echo "hello from ubuntu" && uname -a'"#,
        current_dir,
        ubuntu_wasm_file_path.display()
    ));
    
    run_result?;
    
    anyhow::Ok(())
}
