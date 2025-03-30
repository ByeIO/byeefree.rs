# byeefree-vscode-extension插件
[https://github.com/ByeIO/byeefree.rs/vscode-extension]

## 使用说明
1. 插件的vsix文件拖入VSCodium插件安装器.
2. `Ctrl + Shift + P` -> `byeefree` 或者从侧边栏图标打开界面.

## 开发说明
### 目录结构
```sh
- dist : 编译产物
- node_modules : (网络不好情况下请勿删除)依赖库
- src : 源码
- package.json : 项目配置
- tsconfig.json : 项目配置
- webpack.config.js : 打包配置
```

### 代码说明
```sh
- src
    * extension.ts : 插件入口
    - test : 测试用例
        * extension.test.ts : 单元测试
```

### 编译说明
```sh
# 转换package.toml到package.json文件
pip install toml -i https://mirrors.aliyun.com/pypi/simple/
python -c "import toml, json; data = toml.load(open('package.toml')); json.dump(data, open('package.json', 'w'), indent=2)"
# 编译
vsce package
```
