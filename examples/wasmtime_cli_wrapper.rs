#![allow(unused)]

//! 封装wasmtime_cli命令

// 错误处理
use anyhow::Result;
// 命令行参数解析
use clap::{Parser, Subcommand};
// 处理复杂命令
use shell_words;
// wasmtime-cli的接口(魔改wasmtime_cli库)
use wasmtime_cli::cli::Wasmtime;

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
    WasmtimeCli::run("-V")?;
    anyhow::Ok(())
}
