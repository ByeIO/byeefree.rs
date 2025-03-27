#![allow(unused)]

//! 使用wasmtime运行时调用ubuntu2204.wasm获取`uname -a`指令结果

// 标准库
use std::sync::{Arc, Mutex};

// 错误处理
use anyhow::{Context, Result, Ok};

// wasmtime运行时
use wasmtime::*;
use wasmtime_wasi::preview1;
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder, DynOutputStream};
use wasi_common::sync::WasiCtxBuilder as CommonWasiCtxBuilder;
// use wasmtime_wasi::WasiCtxBuilder;

// 字节处理
use bytes::Bytes;

// 嵌入文件
use embed_file::embed_bytes;

// 复杂命令处理
use shell_words;

/// WASM命令执行上下文
pub struct WasmtimeCli {
    /// 存储编译后的WASM二进制模块
    wasm_module: Vec<u8>,
}

impl WasmtimeCli {
    pub fn init(wasm_binary: &[u8]) -> Self {
        Self {
            wasm_module: wasm_binary.to_vec(),
        }
    }

    pub fn run(&self, command: &str) -> Result<String> {
        let engine = Engine::default();
        let module = Module::from_binary(&engine, &self.wasm_module)
            .context("加载WASM模块失败")?;

        // 构建WASI上下文
        let mut wasi_builder = WasiCtxBuilder::new();
        wasi_builder
            .args(&shell_words::split(command).context("命令解析失败")?)
            // 继承宿主标准输入输出
            .inherit_stdio()
            // 继承宿主的网络到内部网络
            .inherit_network()
            // 使用wasi-p1标准
            .build_p1();

        let wasi_ctx = wasi_builder.build();

        // 链接wasm引擎
        let mut store = Store::new(&engine, wasi_ctx);
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |ctx| ctx)
            .context("链接WASI失败")?;

        // 实例化模块
        let instance = linker.instantiate(&mut store, &module)
            .context("实例化模块失败")?;

        // 获取入口函数
        let start_func = instance.get_typed_func::<(), ()>(&mut store, "_start")
            .context("获取入口函数失败")?;

        start_func.call(&mut store, ()).context("执行失败")?;

        // 已经继承内部stdio了, 所以直接获取系统stdout即可
        
        
        
    }
}


fn main() -> Result<()> {
    // 内嵌WASM二进制文件（编译时打包进可执行文件）
    let wasm_bytes = embed_bytes!("../assets/ubuntu2204.wasm");
    
    // 初始化执行上下文
    let wasm_cli = WasmtimeCli::init(wasm_bytes.as_ref());
    
    // 执行命令（等效于命令行：wasmtime ubuntu2204.wasm uname -a）
    let output = wasm_cli.run("uname -a")?;
    
    // 打印结果
    println!("{}", output.trim_end());

    Ok(())
}
