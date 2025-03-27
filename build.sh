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
