use futures_lite::future::block_on;
use nusb::{
    transfer::{ControlIn, ControlOut, ControlType, Recipient},
    MaybeFuture,
};

fn main() {
    // 初始化日志记录器
    env_logger::init();
    
    // 查找并获取指定VID/PID的USB设备
    let di = nusb::list_devices()
        .wait()  // 等待异步操作完成
        .unwrap()
        .find(|d| d.vendor_id() == 0x303a && d.product_id() == 0x1001)
        .expect("设备未连接，请检查USB连接");

    // 打印设备详细信息
    println!("设备信息: {di:?}");  

    // 打开USB设备连接
    let device = di.open().wait().unwrap();

    // Linux和macOS系统可以不声明接口直接进行控制传输
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        // 发送控制输出请求（Host -> Device）
        let result = block_on(device.control_out(ControlOut {
            // 使用厂商特定类型
            control_type: ControlType::Vendor, 
            // 目标为设备本身
            recipient: Recipient::Device,       
            // 请求码
            request: 0x81,                       
            // 参数值
            value: 0x9999,                       
            // 索引值
            index: 0x9999,                       
            // 要发送的数据
            data: &[1, 2, 3, 4],                
        }));
        println!("控制输出结果: {result:?}");

        // 发送控制输入请求（Device -> Host）
        let result = block_on(device.control_in(ControlIn {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: 0x81,
            value: 0x9999,
            index: 0x9999,
            length: 256,                         // 请求返回数据的长度
        }));
        println!("控制输入结果: {result:?}");
    }

    // Windows系统必须声明接口才能进行控制传输
    // 声明使用接口0（通常为第一个接口）
    let interface = device.claim_interface(0).wait().unwrap();

    // 通过接口发送控制输出请求
    let result = block_on(interface.control_out(ControlOut {
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request: 0x81,
        value: 0x9999,
        index: 0x9999,
        data: &[1, 2, 3, 4],
    }));
    println!("接口控制输出结果: {result:?}");

    // 通过接口发送控制输入请求
    let result = block_on(interface.control_in(ControlIn {
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request: 0x81,
        value: 0x9999,
        index: 0x9999,
        length: 256,
    }));
    println!("接口控制输入结果: {result:?}");
}
