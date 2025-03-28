bun run
bunx --bun vite
bunx --bun vite build
/opt/anaconda3/bin/python ./tests/test1.py
python3 -m http.server 8008
python3 ./tests/test9.py
cargo tauri dev
cargo tauri build
tauri tauri bundle

cd src/deepseek-r1-webgpu && bunx --bun vite build && cd ../..
bunx --bun vite dev --host
# 允许跨域请求
simple-http-server -i -p 8008 --cors

# 打包为pack文件
./tools/web-static-pack-packer directory-single ./dist ./release/apple_v0p0p3.pack
