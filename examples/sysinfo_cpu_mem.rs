//! 显示系统信息mem&cpu

extern crate systemstat;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

fn main() {
    let sys = System::new();

    // 获取并显示挂载点信息
    match sys.mounts() {
        Ok(mounts) => {
            println!("\n挂载点:");
            for mount in mounts.iter() {
                println!("{} ---{}---> {} (可用 {} of {})",
                         mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
            }
        }
        Err(x) => println!("\n挂载点: 错误: {}", x)
    }

    // 获取并显示根目录挂载点信息
    match sys.mount_at("/") {
        Ok(mount) => {
            println!("\n根目录挂载点:");
            println!("{} ---{}---> {} (可用 {} of {})",
                     mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
        }
        Err(x) => println!("\n根目录挂载点: 错误: {}", x)
    }

    // 获取并显示块设备统计信息
    match sys.block_device_statistics() {
        Ok(stats) => {
            for blkstats in stats.values() {
                println!("{}: {:?}", blkstats.name, blkstats);
            }
        }
        Err(x) => println!("\n块设备统计信息: 错误: {}", x)
    }

    // 获取并显示网络接口信息
    match sys.networks() {
        Ok(netifs) => {
            println!("\n网络接口:");
            for netif in netifs.values() {
                println!("{} ({:?})", netif.name, netif.addrs);
            }
        }
        Err(x) => println!("\n网络接口: 错误: {}", x)
    }

    // 获取并显示网络接口统计信息
    match sys.networks() {
        Ok(netifs) => {
            println!("\n网络接口统计信息:");
            for netif in netifs.values() {
                println!("{} 统计信息: ({:?})", netif.name, sys.network_stats(&netif.name));
            }
        }
        Err(x) => println!("\n网络接口统计信息: 错误: {}", x)
    }

    // 获取并显示电池信息
    match sys.battery_life() {
        Ok(battery) =>
            print!("\n电池: {}%, {}小时{}分钟剩余",
                   battery.remaining_capacity*100.0,
                   battery.remaining_time.as_secs() / 3600,
                   battery.remaining_time.as_secs() % 60),
        Err(x) => print!("\n电池: 错误: {}", x)
    }

    // 获取并显示电源状态
    match sys.on_ac_power() {
        Ok(power) => println!(", 电源: {}", power),
        Err(x) => println!(", 电源: 错误: {}", x)
    }

    // 获取并显示内存信息
    match sys.memory() {
        Ok(mem) => println!("\n内存: {} 已使用 / {} ({} 字节) 总量 ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => println!("\n内存: 错误: {}", x)
    }

    // 获取并显示交换分区信息
    match sys.swap() {
        Ok(swap) => println!("\n交换分区: {} 已使用 / {} ({} 字节) 总量 ({:?})", saturating_sub_bytes(swap.total, swap.free), swap.total, swap.total.as_u64(), swap.platform_swap),
        Err(x) => println!("\n交换分区: 错误: {}", x)
    }

    // 获取并显示系统负载
    match sys.load_average() {
        Ok(loadavg) => println!("\n系统负载: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
        Err(x) => println!("\n系统负载: 错误: {}", x)
    }

    // 获取并显示系统运行时间
    match sys.uptime() {
        Ok(uptime) => println!("\n运行时间: {:?}", uptime),
        Err(x) => println!("\n运行时间: 错误: {}", x)
    }

    // 获取并显示系统启动时间
    match sys.boot_time() {
        Ok(boot_time) => println!("\n启动时间: {}", boot_time),
        Err(x) => println!("\n启动时间: 错误: {}", x)
    }

    // 获取并显示CPU负载
    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            println!("\n测量CPU负载...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            println!("CPU负载: {}% 用户, {}% 优先级, {}% 系统, {}% 中断, {}% 空闲 ",
                cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
        },
        Err(x) => println!("\nCPU负载: 错误: {}", x)
    }

    // 获取并显示CPU温度
    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU温度: {}", cpu_temp),
        Err(x) => println!("\nCPU温度: {}", x)
    }

    // 获取并显示系统套接字统计信息
    match sys.socket_stats() {
        Ok(stats) => println!("\n系统套接字统计信息: {:?}", stats),
        Err(x) => println!("\n系统套接字统计信息: 错误: {}", x)
    }
}
