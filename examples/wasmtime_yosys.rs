#![allow(unused)]

//! 使用wasmtime加载yosys

// 标准库
use std::path::Path;
use std::error::Error;
use std::io::Write;
use std::fs::File;

// 错误处理
use anyhow::{Ok, Result};

// wasmtime运行时
use wasmtime_wasi::preview1;
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};
use wasmtime::*;

// 嵌入文件
use embed_file::embed_bytes;

// 临时文件
use tempfile::tempdir;

struct MyState {
    wasi: preview1::WasiP1Ctx,
}

impl MyState {
    fn new() -> Result<MyState> {

        let wasi = WasiCtxBuilder::new()
            .args(&[
                "yosys",
                // "-s",
                // "/ws/half_adder.ys"
                "-h",
            ])

            // wasi 内访问 /share 开头的都会在 runtime 中以 ./result/share 替换
            .preopened_dir(
                Path::new("./result/wasmtime_yosys/device"), 
                "/share",
                DirPerms::all(),
                FilePerms::all()
            )?

            // wasi 内访问 /ws 开头的都会在 runtime 中以 ./test 替换
            .preopened_dir(
                Path::new("./result/wasmtime_yosys/test"), 
                "/ws",
                DirPerms::all(),
                FilePerms::all()
            )?
            // 继承宿主的stdio到内部stdio
            .inherit_stdio()
            // 继承宿主的网络到内部网络
            .inherit_network()
            .build_p1();

        Ok(MyState {
            wasi
        })
    }
}

fn main() -> Result<()> {
    // 嵌入二进制文件到编译产物中 
    let mut yosys_wasm_bytes = embed_bytes!("../assets/yosys.core.wasm");
    
    // 构造临时文件
    let temp_dir = tempdir().expect("创建临时目录失败");
    let yosys_wasm_file_path = temp_dir.path().join("yosys.core.wasm");
    
    // 将嵌入的字节写入临时文件
    let mut yosys_wasm_file = std::fs::File::create(&yosys_wasm_file_path).expect("创建临时文件失败");
    yosys_wasm_file.write_all(&yosys_wasm_bytes).expect("写入临时文件失败");
    
    // 使用嵌入的文件构造wasm运行时
    let engine = Engine::default();
    let module = Module::from_file(&engine, yosys_wasm_file_path)?;

    // 配置 WASI 上下文    
    let my_state = MyState::new()?;
    let mut store = Store::new(&engine, my_state);
    let mut linker = Linker::<MyState>::new(&engine);

    preview1::add_to_linker_sync(&mut linker, |cx| &mut cx.wasi)?;

    // 实例化模块
    let instance = linker.instantiate(&mut store, &module)?;
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    
    match start.call(&mut store, ()) {
        std::result::Result::Ok(_) => {
            println!("执行成功");
            Ok(())
        }
        std::result::Result::Err(_) => {
            Ok(())
        }
    }
    
}
