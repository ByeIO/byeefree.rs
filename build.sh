cargo run --example embed_file
cargo run --example usbd_serial
cargo run --example usb_descriptor
sudo cargo run --example create_user_group
RUST_BACKTRACE=1 cargo run --example log_file_service && cat /tmp/log_file_service
cargo build --example service_installer
sudo cargo run --example service_installer
rustup target list | grep musl
rustup target list | grep aarch64
# RUSTFLAGS='-C target-feature=+crt-static' cargo build --example service_installer --target aarch64-unknown-linux-musl
# cargo build --example service_installer --target aarch64-unknown-none
# cargo build --example service_installer --target aarch64-linux-android
cargo zigbuild --release --example service_installer --target aarch64-unknown-linux-gnu
cargo zigbuild --release --example service_installer
cargo zigbuild --release --example log_file_service --target aarch64-unknown-linux-gnu
cargo-zigbuild run --example udp_base64
cargo-zigbuild run --release --example udp_tcp_multi_package > ./result/udp_tcp_multi_package.log
sudo cargo-zigbuild run --release --example tap_interface
cargo-zigbuild run --release --example tap_interface
cargo-zigbuild run --release --example nusb_attach
cargo-zigbuild run --release --example nusb_control
cargo-zigbuild run --release --example read_write_usb_cdc
cargo-zigbuild run --example wasmtime_ubuntu
cargo-zigbuild run --release --example clap_byeefree -- -v
wasmtime assets/ubuntu2204.wasm uname -a
wasmtime assets/ubuntu2204.wasm bash
cargo-zigbuild run --example sm4_encrypt
cargo-zigbuild run --release --example tempfile
cargo-zigbuild run --release --example sm9_encrypt > result/sm9_encrypt.log
rustup component remove --toolchain stable rust-std-wasm-wasi
rustup component remove --toolchain nightly rust-std-wasm32-wasi
rustup update stable
rustup update nightly
cargo-zigbuild run --release --example database_sql
cargo-zigbuild run --release --example database_login
cargo-zigbuild run --release --example wasmtime_yosys
cargo-zigbuild run --release --example wasmtime_yosys > result/wasmtime_yosys.log
cargo-zigbuild run --release --example wasmtime_cli_wrapper
cargo-zigbuild run --release --example wasmtime_ubuntu
cargo-zigbuild run --release --example sysinfo_cpu_mem
cargo-zigbuild run --release --example clap_example_derive
cargo-zigbuild run --release --example clap_byeefree
# 正式文件
cargo-zigbuild run --release --bin byeefree
cargo-zigbuild run --bin byeefree
cargo-zigbuild run --release --bin byeefree ubuntu -c "uname -a"
cargo-zigbuild run --bin byeefree ubuntu -c "touch /home/hello_ubuntu"
cargo-zigbuild run --bin byeefree ubuntu -c "uname -a"
RUST_BACKTRACE=1 cargo test
git rev-parse --short HEAD
cargo-zigbuild build --release --bin byeefree --target aarch64-unknown-linux-gnu.2.17
# cp /Users/workspace/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/aarch64-unknown-linux-gnu/lib/liblibgcc_s.so
cargo-zigbuild build --release --bin byeefree --target aarch64-unknown-linux-musl
ssh qsbye@192.168.30.171
rsync -avz --partial --progress /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree qsbye@192.168.30.171:/home/qsbye/byeefree
cargo build --release --bin byeefree --target aarch64-unknown-linux-gnu
cargo run --release --bin byeefree --target aarch64-unknown-linux-gnu ubuntu -c "uname -a"
rsync -avz --partial --progress qsbye@192.168.30.171:/home/qsbye/byeefree/exp226-rust-byeefree/target/aarch64-unknown-linux-gnu/release/byeefree /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/byeefree.linux.aarch64
rsync -avz --partial --progress /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/byeefree.linux.aarch64 qsbye@192.168.30.33:/home/qsbye
# 继续编译
cargo-zigbuild run --bin byeefree
cargo-zigbuild run --release --example redb_kv
cargo-zigbuild run --release --example sqlite3_sql
RUST_LOG=TRACE cargo-zigbuild run --bin byeefree account login "qsbye"
RUST_LOG=TRACE cargo-zigbuild run --release --example print_log
RUST_LOG=TRACE cargo-zigbuild run --bin byeefree util sysinfo
cargo-zigbuild run --bin installer
cargo-zigbuild build --release --bin byeefree --target aarch64-unknown-linux-gnu
cargo-zigbuild build --release --bin installer --target aarch64-unknown-linux-gnu
git add ./target/aarch64-apple-darwin/release/byeefree -f
git add ./target/aarch64-unknown-linux-gnu/release/byeefree -f
git add ./target/aarch64-apple-darwin/release/installer -f
git add ./target/aarch64-unknown-linux-gnu/release/installer -f
cargo-zigbuild build --release --bin installer
cargo-zigbuild run --release --bin installer
cargo-zigbuild build --release --bin installer --target aarch64-unknown-linux-gnu
rsync -avz --partial --progress /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/aarch64-unknown-linux-gnu/release/installer qsbye@192.168.30.33:/home/qsbye
chmod +x installer
sudo ./installer
# 手动上传
rsync -avz --partial --progress /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/aarch64-unknown-linux-gnu/release/byeefree qsbye@192.168.30.33:/usr/local/bin
chmod +x /usr/local/bin/byeefree
# 打包网页前端
bun run build
./tools/web-static-pack-packer directory-single ./website-frontend/dist ./assets/frontend.pack
# tar打包文件
tar -cvf ./assets/frontend.tar -C ./website-frontend/dist .
git rm --cache assets/webviz_wasi.wasm
cargo-zigbuild run --release --example tar_unpack
cargo-zigbuild run --release --example tokio_multithread
cargo-zigbuild run --release --example http_server_wasm
cargo-zigbuild run --release --example http_server
simple-http-server ./website-frontend/dist -p 8888 --cors --try-file ./website-frontend/dist/static/wenxiaobai.html
wasmtime run -S inherit-network=y --dir ./website-frontend/dist::/var/www/html ./assets/simple_http_server_wasi.wasm -- -p 8888 --cors 
# [http://127.0.0.1:8888/index.html]
cargo-zigbuild build --bin byeefree
cargo-zigbuild run --release --bin byeefree util service run
cargo-zigbuild build --release --bin byeefree --target aarch64-unknown-linux-gnu
cargo-zigbuild build --release --bin installer --target aarch64-unknown-linux-gnu
# 同步代码到编译机
rsync -avz --partial --progress /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree qsbye@192.168.30.171:/home/qsbye/byeefree
cargo build --release --bin byeefree --target aarch64-unknown-linux-gnu
cargo build --release --bin installer --target aarch64-unknown-linux-gnu
cargo run --release --bin installer --target aarch64-unknown-linux-gnu
cargo run --release --target aarch64-unknown-linux-gnu --bin byeefree ubuntu -c "uname -a"
# 上机测试
rsync -avz --partial --progress qsbye@192.168.30.171:/home/qsbye/byeefree/exp226-rust-byeefree/target/aarch64-unknown-linux-gnu/release/installer /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/installer.linux.aarch64
rsync -avz --partial --progress /Users/workspace/Desktop/projects/ByeIO/software/exp226-rust-byeefree/target/installer.linux.aarch64 qsbye@192.168.30.33:/home/qsbye
# 直接复制到测试机
rsync -avz --partial --progress /home/qsbye/byeefree/exp226-rust-byeefree/target/aarch64-unknown-linux-gnu/release/installer qsbye@192.168.30.33:/home/qsbye
# rm /home/qsbye/installer /home/qsbye/installer.linux.aarch64
chmod +x /home/qsbye/installer.linux.aarch64
chmod +x /home/qsbye/installer
sudo systemctl stop byeefree_rs
sudo /home/qsbye/installer.linux.aarch64
sudo /home/qsbye/installer
# [http://192.168.30.33:8888/index.html]
# 开发机测试
cargo-zigbuild run --release --bin byeefree -- -h
