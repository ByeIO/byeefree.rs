//! 使用nusb打开VID（`303a`）和PID（`4001`）的USB设备
use std::{thread::sleep, time::Duration};

// USB操作
use nusb::MaybeFuture;

fn main() {
    env_logger::init();
    let di = nusb::list_devices()
        .wait()
        .unwrap()
        .find(|d| d.vendor_id() == 0x303a && d.product_id() == 0x1001)
        .expect("device should be connected");
    println!("usb设备已找到");
    let device = di.open().wait().unwrap();
    device.detach_kernel_driver(0).unwrap();
    sleep(Duration::from_secs(10));
    device.attach_kernel_driver(0).unwrap();
}
