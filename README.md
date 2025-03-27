# byeefree : 适用于RK3588S2计算板及地面站的任务软件集合
- 支持一键安装及运行, 无需互联网连接.
提供`byeefree`命令, 可以一键启动无人机任务.

## 虚拟网卡
将esp32点对点透传的数据封装为udp/tcp并通过linux TAP虚拟网卡模拟为网卡, 然后就可以使用:
```sh
ssh qsbye@[ip地址]
```
来连接开发板了.

## 开发说明
使用nightly通道的1.85以上版本rustc编译.

### 项目目录


### 代码目录说明


### 例程说明
**文件夹: examples/**
```yaml
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
