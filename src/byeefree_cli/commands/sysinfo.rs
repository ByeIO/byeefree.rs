#![allow(unused)]

//! 显示系统资源情况

// 标准库
use std::thread;
use std::time::Duration;

// 显示系统资源
use systemstat::{System, Platform, saturating_sub_bytes};

pub fn util_sysinfo_cmd(){
    // 获取系统资源情况实例
    let sys = System::new();
    
    // 获取并显示内存信息
    match sys.memory() {
        Ok(mem) => println!("\n内存: {} 已使用 / {} ({} 字节) 总量 ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => println!("\n内存: 错误: {}", x)
    }
    
    // 获取并显示系统负载
    match sys.load_average() {
        Ok(loadavg) => println!("\n系统负载: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
        Err(x) => println!("\n系统负载: 错误: {}", x)
    }
    
    // 获取并显示CPU负载
    // match sys.cpu_load_aggregate() {
    //     Ok(cpu)=> {
    //         println!("\n测量CPU负载...");
    //         thread::sleep(Duration::from_secs(1));
    //         let cpu = cpu.done().unwrap();
    //         println!("CPU负载: {}% 用户, {}% 优先级, {}% 系统, {}% 中断, {}% 空闲 ",
    //             cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
    //     },
    //     Err(x) => println!("\nCPU负载: 错误: {}", x)
    // }
    
}
