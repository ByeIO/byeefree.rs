# byeefree : 适用于RK3588S2计算板及地面站的任务软件集合
- 支持一键安装及运行, 无需互联网连接.
提供`byeefree`命令, 可以一键启动无人机任务.

## 使用说明
### 编译&安装
```sh
# 安装编译工具
# sudo apt install -y rustup
rustup toolchain add nightly
cargo install cargo-zigbuild
rustup target add aarch64-unknown-linux-gnu
rustup target add aarch64-apple-darwin
# 第一步: 编译主程序
cargo-zigbuild build --release --bin byeefree --target aarch64-unknown-linux-gnu
cargo-zigbuild build --release --bin byeefree --target aarch64-apple-darwin
# 第二步: 编译安装器
cargo-zigbuild build --release --bin installer --target aarch64-unknown-linux-gnu
# 第三步: 上传文件
rsync -avz --partial --progress /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/aarch64-unknown-linux-gnu/release/installer qsbye@192.168.30.33:/home/qsbye
chmod +x installer
sudo ./installer
# 第四步: 测试
byeefree -h
```

### byeefree命令行
```sh
byeefree -h
byeefree -v
# 机器人框架
byeefree rosette -h
# 可视化界面
byeefree rosette webviz
# mosh远程连接工具
byeefree mosh
# rsync文件同步工具
byeefree rsync
# 安装/卸载服务
byeefree util service install
byeefree util service uninstall
byeefree util service status
byeefree util service restart
byeefree util service run
# 使用ubuntu运行某些难以编译为wasm的程序
byeefree ubuntu -c "uname -r"
# 登录账户(使用sm4-key)
byeefree account login "qsbye"
# 注册帐户: 自动生成私钥并保存到/tmp/_ByeIO_目录
byeefree account register "qsbye"
# 查看日志
byeefree log
# 查看系统cpu及内存占用率
byeefree util sysinfo
# 打印usb设备(类似adb device)
byeefree util device
# 设置本机的角色
byeefree util role air # 空中 
byeefree util role ground # 地面站 
```

### 虚拟网卡
将esp32点对点透传的数据封装为udp/tcp并通过linux TAP虚拟网卡模拟为网卡, 然后就可以使用:`byeefree mosh qsbye@[ip地址]`来连接机载计算机了.

## 开发说明
推荐使用nightly通道的1.85版本rustc编译.过高版本或过低版本都不能保证兼容性.

### 项目目录
```sh
- .cargo文件夹 : 编译器配置
- assets文件夹 : 资源文件
- docs文件夹 : 文档
- examples文件夹 : 例程
- files文件夹 : 资源文件
- result文件夹 : 代码运行输出
- src文件夹 : 主要代码
- static文件夹 : 魔改的第三方库
- target文件夹 : 编译结果
- vendor文件夹 : 所有的第三方库
- `问题记录`文件夹 : 问题问答
* build.rs : 编译前处理
* build.sh : 编译命令
* Cargo.toml : 项目配置
* clippy.toml : 代码风格设定
* LICENSE : 代码开源许可
* README.md : 说明书
* rust-toolchain.toml : 编译器配置
```

### 代码目录说明
```sh
- bin
    * byeefree.rs : byeefree-cli命令行工具
    * installer.rs : 服务安装程序
    * node_a.rs : A节点专用代码
    * node_b.rs : B节点专用代码
- byeefree_cli : 
```

### 例程说明
**文件夹: examples/**
```sh
- clap_byeefree.rs : 命令行程序
- create_user_group.rs : 创建系统用户和用户组
- database_login.rs : 数据库+登录用户账户
- database_sql.rs : 数据库基本操作
- embed_file.rs : 内嵌二进制文件
- log_file_service.rs : 写入当前时间到日志
- nusb_attach.rs : 使用nusb连接usb设备
- nusb_control.rs : 使用nusb控制usb设备端点
- read_write_usb_cdc.rs : 使用nusb读写usb-cdc-acm虚拟串口
- service_installer.rs : 自动安装系统服务
- sm4_encrypt.rs : SM4加密解密
- sm9_encrypy.rs : SM9加密解密
- sysinfo_cpu_mem.rs : 查看系统CPU占用率及内存使用情况
- tap_interface.rs : 使用TAP虚拟网卡接口
- tap_tcp_udp_bridge.rs : 虚拟网桥桥接tcp, udp
- tap_virtual_net.rs : TAP虚拟网桥
- tempfile.rs : 创建临时文件
- udp_base64.rs : udp包使用base64编码
- udp_tcp_multi_package.rs : 分包传输超过MTU的tcp和udp包
- usb_descriptor.rs : 使用nusb获取usb设备描述符
- wasmtime_ubuntu.rs : 使用wasmtime运行ubuntu系统
- wasmtime_yosys.rs : 使用wasmtime运行yosys工具
```
