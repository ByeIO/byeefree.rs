#![allow(unused_imports)]

//! 使用tappers库创建TAP网络接口

use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tappers::{AddAddressV4, AddAddressV6, AddressInfo, DeviceState, Interface, Tap};
use std::env::consts::OS;
use std::process::Stdio;
use std::process::Command;

fn main() -> io::Result<()> {
    // 获取系统名称
    let os_name = std::env::consts::OS;
    
    let tap_interface_name = match os_name{
        "macos" => {"feth233"},
        "linux" => {"tap233"},
        _ => {""},
    };
    
    // 选择要打开的现有（或新建）TAP接口名称
    let tap_name = Interface::new(tap_interface_name)?;
    // let tap_name = Interface::new(format!("{prefix}233"))?;
    println!("创建tap_interface成功:{}", tap_interface_name);

    // 使用ifconfig判断是否有tap_interface_name `ifconfig | grep ${tap_interface_name}`
    let ifconfig_output = std::process::Command::new("ifconfig")
        .output()?;
    let tap_interface_name_exists = String::from_utf8_lossy(&ifconfig_output.stdout).contains(tap_interface_name);
    
    // 打开名为"tap233"或"utun233"的TAP设备（如果不存在则创建）
    let mut tap = if tap_interface_name_exists {
            println!("tap设备已存在");
            Tap::new()?
        }else{
            Tap::new_named(tap_name)?
    };
    println!("创建tap设备成功");

    // 为TAP设备添加新的IPv4地址及关联信息
    let new_addr = Ipv4Addr::new(10, 168, 233, 1);
    let mut addr_req = AddAddressV4::new(new_addr);
    addr_req.set_netmask(24);
    addr_req.set_broadcast(Ipv4Addr::new(10, 168, 233, 255));
    tap.add_addr(addr_req)?;
    println!("为TAP设备添加新的IPv4地址及关联信息成功");

    // // 获取TAP设备绑定的所有IPv4/IPv6地址信息
    // let addrs = tap.addrs()?;
    // println!("获取TAP设备绑定的所有IPv4/IPv6地址信息成功");
    // for addr_info in addrs {
    //     println!("IP地址: {}", addr_info.address());
    //     if let Some(netmask) = addr_info.netmask() {
    //         println!("子网掩码: {}", netmask);
    //     }
    //     if let Some(broadcast) = addr_info.broadcast() {
    //         println!("广播地址: {}", broadcast);
    //     }
    // }

    // 从TAP设备移除指定的IP地址
    // tap.remove_addr(IpAddr::V4(new_addr))?;

    // 设置TAP设备为非阻塞模式（读写操作立即返回）
    tap.set_nonblocking(true)?;

    // 启动网络设备以允许数据包交换
    tap.set_state(DeviceState::Up)?;

    let mut buf = [0; 65536];

    // 从接口接收数据包
    let amount = tap.recv(&mut buf)?;
    println!("接收到 {} 字节的数据", amount);

    // 通过接口发送数据包
    let amount = tap.send(&buf[..amount])?;
    println!("发送了 {} 字节的数据", amount);

    // 关闭网络设备停止数据包交换
    tap.set_state(DeviceState::Down)?;

    // 当tap变量离开作用域时，系统会自动删除TUN设备
    Ok(())
}
