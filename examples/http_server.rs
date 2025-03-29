#![allow(unused)]

//! 使用simple-http-server lib启动网页服务器

// 多线程
use tokio::time::{sleep, Duration};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// 标准库
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::fs;
use std::process::{Command, Stdio};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::env::consts::OS;
use std::borrow::Cow;

// 嵌入文件
use embed_file::embed_bytes;

// 临时文件
use tempfile::{ tempdir, TempDir };

// 错误处理
use anyhow::{Context, Result, Ok, Error};

// 网页服务器
use simple_http_server::cli::SimpleHttpServerCli;

/// 嵌入二进制文件到编译产物中 
const TAR_FILE_BYTES : Cow<'_, [u8]> = embed_bytes!("../assets/frontend.tar");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 解包网页资源文件
    let tar_files_path = unpack_tar(&TAR_FILE_BYTES)?;
    
    // 构造命令
    let cmd = format!(
        r#"simple-http-server {} -p 8888 --cors --try-file {}/static/wenxiaobai.html"#,
        tar_files_path.path().display(),
        tar_files_path.path().display(),
    );
    println!("命令为:{}", cmd);
    
    // 运行网页服务器
    SimpleHttpServerCli::run(cmd).await;
    
    // 返回值
    anyhow::Ok(())
}

// 解包tar文件的字节数组到临时文件夹并返回临时文件夹的路径
pub fn unpack_tar(tar_bytes: &[u8]) -> Result<tempfile::TempDir> {
    // tar解包文件
    use tar::Archive;
    
    // 错误处理
    use anyhow::{Context, Result, Ok, Error};
    
    // 标准库
    use std::io::Cursor;
    
    // 创建临时目录
    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;
    
    // 创建内存中的tar文件读取器
    let tar_cursor = Cursor::new(tar_bytes);
    
    // 直接从内存解析tar文件
    let mut archive = Archive::new(tar_cursor);
    
    // 解包到临时目录
    archive.unpack(temp_dir.path())
        .context("Failed to unpack tar archive")?;

    Ok(temp_dir)
}

// 将[u8]字节数组构造为临时文件夹中的文件并返回临时文件的路径, 返回包含临时目录的结构体以延长生命周期
pub fn bytes_to_file(bytes: &[u8], file_name: &str) -> anyhow::Result<(TempDir, PathBuf)> {
    // 标准库
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    
    // 临时文件
    use tempfile::TempDir;
    
    // 错误处理
    use anyhow::Context;
    
    let temp_dir = tempfile::tempdir().context("创建临时目录失败")?;
    let file_path = temp_dir.path().join(file_name);
    
    // 创建并写入临时文件
    let mut temp_file = File::create(&file_path).context("创建临时文件失败")?;
    temp_file.write_all(bytes).context("写入临时文件失败")?;
    
    Ok((temp_dir, file_path))
}
