#![allow(unused)]

//! 服务管理

// 标准库
use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::env::consts::OS;
use std::path::Path;
use std::process::{Command, Stdio};

// 错误处理
use anyhow;

// 公共函数实现
fn common_install(os_name: &str, service_path: &str, exec_path: &str) -> anyhow::Result<()> {
    match os_name {
        "macos" => {
            // macOS 服务安装
            let _ = Command::new("launchctl")
                .args(&["load", "-w", service_path])
                .status()?;
            println!("服务安装成功，执行文件路径：{}", exec_path);
        }
        "linux" => {
            // Linux 服务安装
            let _ = Command::new("systemctl")
                .args(&["daemon-reload"])
                .status()?;
            let _ = Command::new("systemctl")
                .args(&["enable", service_path])
                .status()?;
            println!("服务已设置开机启动，执行文件路径：{}", exec_path);
        }
        _ => {}
    }// end match
    
    anyhow::Ok(())
}

// 1. 安装服务
pub fn util_service_install_cmd() -> anyhow::Result<()> {
    println!("安装后台服务");
    /// 获取操作系统
    let os_name = std::env::consts::OS;
    println!("当前操作系统为:{}", os_name);
    
    // 判断系统并进行分别处理
    match os_name {
        "macos" => {
            let plist_path = "/Library/LaunchDaemons/byeefree_rs.plist";
            let exec_path = "/usr/local/bin/byeefree";
            
            // 设置文件权限
            fs::set_permissions(plist_path, fs::Permissions::from_mode(0o644))?;
            fs::set_permissions(exec_path, fs::Permissions::from_mode(0o755))?;
            
            common_install(os_name, plist_path, exec_path)?;
            Command::new("launchctl").args(&["start", "byeefree_rs"]).status()?;
        }
        "linux" => {
            let service_path = "/etc/systemd/system/byeefree_rs.service";
            let exec_path = "/usr/local/bin/byeefree";
            
            // 设置文件权限
            fs::set_permissions(service_path, fs::Permissions::from_mode(0o644))?;
            fs::set_permissions(exec_path, fs::Permissions::from_mode(0o755))?;
            
            common_install(os_name, service_path, exec_path)?;
            Command::new("systemctl").args(&["start", "byeefree_rs.service"]).status()?;
        }
        "windows" => println!("暂不支持"),
        _ => println!("暂不支持"),
    }// end match
    
    // 返回
    anyhow::Ok(())
    
}

// 2. 卸载服务
pub fn util_service_uninstall_cmd() -> anyhow::Result<()> {
    println!("卸载后台服务");
    
    /// 获取操作系统
    let os_name = std::env::consts::OS;
    println!("当前操作系统为:{}", os_name);
    
    // 判断系统并进行分别处理
    match os_name {
        "macos" => {
            let plist_path = "/Library/LaunchDaemons/byeefree_rs.plist";
            Command::new("launchctl").args(&["unload", "-w", plist_path]).status()?;
            fs::remove_file(plist_path)?;
            fs::remove_file("/usr/local/bin/byeefree")?;
            println!("服务卸载完成");
        }
        "linux" => {
            let service_path = "/etc/systemd/system/byeefree_rs.service";
            Command::new("systemctl").args(&["stop", "byeefree_rs.service"]).status()?;
            Command::new("systemctl").args(&["disable", "byeefree_rs.service"]).status()?;
            fs::remove_file(service_path)?;
            fs::remove_file("/usr/local/bin/byeefree")?;
            println!("服务卸载完成");
        }
        "windows" => println!("暂不支持"),
        _ => println!("暂不支持"),
    }// end match
    
    // 返回
    anyhow::Ok(())
}

// 3. 查看服务状态
pub fn util_service_status_cmd() -> anyhow::Result<()> {
    println!("查看服务状态");
    
    /// 获取操作系统
    let os_name = std::env::consts::OS;
    println!("当前操作系统为:{}", os_name);
    
    // 判断系统并进行分别处理
    match os_name {
        "macos" => {
            let output = Command::new("launchctl")
                .args(&["list", "byeefree_rs"])
                .output()?;
            println!("服务状态：\n{}", String::from_utf8_lossy(&output.stdout));
        }
        "linux" => {
            let output = Command::new("systemctl")
                .args(&["status", "byeefree_rs.service"])
                .output()?;
            println!("服务状态：\n{}", String::from_utf8_lossy(&output.stdout));
        }
        "windows" => println!("暂不支持"),
        _ => println!("暂不支持"),
    }// end match
    
    // 返回
    anyhow::Ok(())
}

// 4. 重启服务
pub fn util_service_restart_cmd() -> anyhow::Result<()> {
    println!("重启服务");
    
    /// 获取操作系统
    let os_name = std::env::consts::OS;
    println!("当前操作系统为:{}", os_name);
    
    // 判断系统并进行分别处理
    match os_name {
        "macos" => {
            Command::new("launchctl").args(&["stop", "byeefree_rs"]).status()?;
            Command::new("launchctl").args(&["start", "byeefree_rs"]).status()?;
            println!("服务重启成功");
        }
        "linux" => {
            Command::new("systemctl").args(&["restart", "byeefree_rs.service"]).status()?;
            println!("服务重启成功");
        }
        "windows" => println!("暂不支持"),
        _ => println!("暂不支持"),
    }// end match
    
    // 返回
    anyhow::Ok(())
}
