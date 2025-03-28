#![allow(unused)]

//! byeefree安装器

// 标准库
use std::{fs, path::Path};
use std::error::Error;
use std::process::{Command, Stdio};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::env::consts::OS;
use std::borrow::Cow;

// 嵌入文件
use embed_file::embed_bytes;

// 错误处理
use anyhow;

/// 嵌入二进制文件到编译产物中 
const SERVICE_FILE_BYTES : Cow<'_, [u8]> = embed_bytes!("../../assets/byeefree_rs.service");
const PLIST_FILE_BYTES : Cow<'_, [u8]> = embed_bytes!("../../assets/byeefree_rs.plist");
const MACOS_BIN_FILE_BYTES : Cow<'_, [u8]> = embed_bytes!("../../target/aarch64-apple-darwin/release/byeefree");
const LINUX_BIN_FILE_BYTES : Cow<'_, [u8]> = embed_bytes!("../../target/aarch64-unknown-linux-gnu/release/byeefree");

fn main() -> anyhow::Result<(), > {
    /// 获取操作系统
    let os_name = std::env::consts::OS;
    println!("当前操作系统为:{}", os_name);
    
    // 判断系统并进行分别处理
    match os_name {
        "macos" => {
            macos_installer();
        },
        "linux" => {
            linux_installer();
        },
        "windows" => {
            windows_installer();
        },
        _ => {
            println!("暂不支持, 安装程序即将退出");
            return anyhow::Ok(());
        }
    }// end match
    
    return anyhow::Ok(());
}

/* start 分别处理各个系统 */

/// 处理macos系统安装情况
fn macos_installer() -> anyhow::Result<(), > {
    // 创建用户组和用户
    let _ = macos_create_user_group()?;
    
    // 嵌入二进制文件到编译产物中
    let plist_file : &[u8] = &PLIST_FILE_BYTES;
    let exec_file : &[u8] = &MACOS_BIN_FILE_BYTES;
    
    // 解压可执行文件并配置系统后台服务
    println!("复制文件中...");
    // 创建目标目录（如果不存在）
    let output_dir_1 = Path::new("/tmp/_Byeefree_");
    let output_dir_3 = Path::new("/Library/LaunchDaemons");
    
    // 递归创建目录
    let _ = fs::create_dir_all(output_dir_1)?;
    
    // 构建输出文件路径
    let output_plist_path = output_dir_3.join("byeefree_rs.plist");
    let output_exec_path = output_dir_1.join("byeefree");
    
    // 将嵌入的二进制数据写入文件
    let _ = fs::create_dir_all(output_dir_3)?;
    let _ = fs::write(&output_plist_path, plist_file)?;
    println!("文件已保存至：{}", output_plist_path.display());
    let _ = fs::write(&output_exec_path, exec_file)?;
    println!("文件已保存至：{}", output_exec_path.display());
    
    // 配置可执行文件权限及所有权
    let _ = fs::set_permissions(output_exec_path.clone(), fs::Permissions::from_mode(0o755))?;
    let chown_status = Command::new("chown")
        .arg("_Byeefree_:_ByeIO_")
        .arg(&output_exec_path)
        .status()?;
    if !chown_status.success() {
        return anyhow::Ok(());
    }else{
        println!("文件添加可执行权限成功.");
    }
    
    // 设置服务文件权限
    let _ = fs::set_permissions(output_plist_path.clone(), fs::Permissions::from_mode(0o644))?;
    println!("服务配置文件添加权限成功.");
    
    // 启动服务并设置开机自启
    let _ = Command::new("launchctl")
        .args(&["load", "-w", output_plist_path.clone().to_str().unwrap()])
        .status()?;
    println!("macOS服务设置开机自启成功.");
    
    // 配置命令别名
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo 'alias {}={}' >> ~/.bash_profile",
            "byeefree_cli", output_exec_path.display()
        ))
        .status()?;
    let _ = Command::new("sh")
        .arg("-c")
        .arg("source ~/.bashrc")
        .status()?;
    
    // println!("安装服务完成, 输入`cat /tmp/byeefree_log_time`查看效果");
    println!("安装服务完成, 输入`byeefree`查看效果");
    
    return anyhow::Ok(());
}

/// 处理linux系统安装情况
fn linux_installer() -> anyhow::Result<(), > {
    // 创建用户和用户组
    let _ = linux_create_user_group()?;
    
    // 嵌入二进制文件到编译产物中
    let service_file : &[u8] = &SERVICE_FILE_BYTES;
    let elf_file : &[u8] = &LINUX_BIN_FILE_BYTES;
    
    // 构建文件输出路径
    let output_dir_2 = Path::new("/etc/systemd/system");
    let output_dir_4 = Path::new("/usr/local/bin");
    let output_service_path = output_dir_2.join("byeefree_rs.service");
    let output_exec_path = output_dir_4.join("byeefree");
        
    // 将嵌入的二进制数据写入文件
    let _ = fs::create_dir_all(output_dir_2)?;
    let _ = fs::write(&output_service_path, service_file)?;
    println!("文件已保存至：{}", output_service_path.display());
    let _ = fs::write(&output_exec_path, elf_file)?;
    println!("文件已保存至：{}", output_exec_path.display());
    
    // 配置可执行文件权限及所有权
    let _ = fs::set_permissions(output_exec_path.clone(), fs::Permissions::from_mode(0o755))?;
    let chown_status = Command::new("chown")
        .arg("_Byeefree_:_ByeIO_")
        .arg(&output_exec_path)
        .status()?;
    if !chown_status.success() {
        return anyhow::Ok(());
    }else{
        println!("文件添加可执行权限成功.");
    }
    
    // 添加服务权限
    let _ = fs::set_permissions(output_service_path.clone(), fs::Permissions::from_mode(0o644))?;
    println!("服务配置文件添加权限成功.");
    
    // 启动服务并设置开机自启
    let _ = Command::new("systemctl")
        .args(&["daemon-reload"])
        .status()?;
    println!("linux服务配置重载成功.");
    let _ = Command::new("systemctl")
        .args(&["start", "byeefree_rs.service"])
        .status()?;
    println!("linux服务启动成功.");
    let _ = Command::new("systemctl")
        .args(&["enable", "byeefree_rs.service"])
        .status()?;
    println!("linux服务设置开机自启成功.");
    
    // 配置命令别名
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo 'alias {}={}' >> ~/.bashrc",
            "byeefree_cli", output_exec_path.display()
        ))
        .status()?;
    let _ = Command::new("bash")
        .arg("-c")
        .arg("source ~/.bashrc")
        .status()?;
    
    // 打印完成
    println!("安装服务完成, 输入`byeefree`查看帮助.");
    
    return anyhow::Ok(());
}

/// 处理windows系统安装情况
fn windows_installer() -> anyhow::Result<(), > {
    println!("暂不支持");
    return anyhow::Ok(());
}

/* end 分别处理各个系统 */

/* start 创建用户组和用户 */

fn linux_create_user_group() -> io::Result<()> {
    use std::io::{self, ErrorKind, Error};

    // 检查用户组是否存在
    let group_exists = Command::new("getent")
        .arg("group")
        .arg("_ByeIO_")
        .status()?
        .success();

    if !group_exists {
        // 创建用户组
        let group_output = Command::new("groupadd")
            .arg("_ByeIO_")
            .output()?;

        if !group_output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("用户组创建失败: {}", String::from_utf8_lossy(&group_output.stderr))
            ));
        }
    }

    // 检查用户是否存在
    let user_exists = Command::new("id")
        .arg("-u")
        .arg("_Byeefree_")
        .status()?
        .success();

    if !user_exists {
        // 创建用户
        let user_output = Command::new("useradd")
            .args(&["-g", "_ByeIO_", "_Byeefree_"])
            .output()?;

        if !user_output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("用户创建失败: {}", String::from_utf8_lossy(&user_output.stderr))
            ));
        }
    }

    Ok(())
}

fn macos_create_user_group() -> io::Result<()> {
    use std::io::{self, ErrorKind, Error};

    // 检查用户组是否存在
    let group_exists = Command::new("sudo")
        .args(&["dscl", ".", "-read", "/Groups/_ByeIO_"])
        .status()
        .map(|s| s.success())?;

    if !group_exists {
        // 创建用户组
        let group_create = Command::new("sudo")
            .args(&["dscl", ".", "-create", "/Groups/_ByeIO_"])
            .status()?;

        if !group_create.success() {
            return Err(Error::new(ErrorKind::Other, "macOS用户组创建失败"));
        }
    }

    // 检查用户是否存在
    let user_exists = Command::new("sudo")
        .args(&["dscl", ".", "-read", "/Users/_Byeefree_"])
        .status()
        .map(|s| s.success())?;

    if !user_exists {
        // 创建用户属性
        let user_commands = [
            ("/Users/_Byeefree_", "UserShell", "/bin/bash"),
            ("/Users/_Byeefree_", "RealName", "ByeIO Service Account"),
            ("/Users/_Byeefree_", "PrimaryGroupID", "2000"),
        ];

        for (path, key, value) in &user_commands {
            let status = Command::new("sudo")
                .args(&["dscl", ".", "-create", path, key, value])
                .status()?;
            if !status.success() {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("用户属性创建失败: {}/{}/{}", path, key, value)
                ));
            }
        }

        // 将用户加入组
        let append_status = Command::new("sudo")
            .args(&["dscl", ".", "-append", "/Groups/_ByeIO_", "GroupMembership", "_Byeefree_"])
            .status()?;
        if !append_status.success() {
            return Err(Error::new(ErrorKind::Other, "无法将用户添加到组"));
        }
    }

    Ok(())
}

/* end 创建用户组和用户 */
