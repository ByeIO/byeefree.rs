bun install -g yo generator-code
export PATH="/Users/workspace/.bun/bin:$PATH"
bun --bunx yo code
yo code
bun install -g @vscode/vsce
bun install -g vsce

# 打包成 VSIX
bun --bunx vsce package
vsce package

# 安装依赖
npm install --save express cors 
npm install --save @kurkle/color
npm install --save copy-webpack-plugin

# 更新nodejs
# Download and install nvm:
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.2/install.sh | bash
# in lieu of restarting the shell
\. "$HOME/.nvm/nvm.sh"
# Download and install Node.js:
nvm install 22
# Verify the Node.js version:
node -v # Should print "v22.14.0".
nvm current # Should print "v22.14.0".
# Verify npm version:
npm -v # Should print "10.9.2".

# 编译
python -c "import toml, json; data = toml.load(open('package.toml')); json.dump(data, open('package.json', 'w'), indent=2)"
# npm run compile
# npm run test
vsce package
