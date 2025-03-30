// 导入 vscode 模块，用于与 VSCode 编辑器进行交互
import * as vscode from 'vscode';
// 导入 express 模块，用于创建 Web 服务器
import express from 'express';
// 导入 cors 模块，用于处理跨域请求
import cors from 'cors';
// 导入 path 模块，用于处理文件路径
import * as path from 'path';
// 颜色处理
import("@kurkle/color")

// 激活扩展时调用的函数
export function activate(context: vscode.ExtensionContext) {
    // 创建 Express 应用实例
    const app = express();
    // 定义服务器监听的端口
    const port = 8888;

    // 启用 CORS 跨域支持
    app.use(cors());

    // 获取插件安装目录的绝对路径
    const extensionPath = context.extensionPath;

    // 设置静态文件目录（指向 byeefree-frontend 目录）
    const frontendPath = path.join(extensionPath, 'static', 'byeefree-frontend');
    app.use(express.static(frontendPath));

    // 处理 404 请求，返回指定的错误页面
    app.use((req: express.Request, res: express.Response) => {
        res.status(404).sendFile(
            path.join(frontendPath, 'static', 'wenxiaobai.html')
        );
    });

    // 启动服务器
    const server = app.listen(port, () => {
        console.log(`[byeefree] 服务已启动在端口 ${port}`);
        vscode.window.showInformationMessage(`byeefree服务已启动在端口 ${port}`);

        // 在 VSCode 页面中显示网页主页
        const panel = vscode.window.createWebviewPanel(
            'byeefreeHomepage', // 面板的标识符
            'Byeefree 主页', // 面板的标题
            vscode.ViewColumn.One, // 面板显示的位置
            {
                enableScripts: true, // 启用脚本
                localResourceRoots: [vscode.Uri.file(frontendPath)] // 允许加载的本地资源路径
            }
        );

        // 设置 Webview 的 HTML 内容
        panel.webview.html = `
            <!DOCTYPE html>
            <html lang="zh-CN">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Byeefree 主页</title>
            </head>
            <body>
                <iframe src="http://localhost:${port}" width="100%" height="100%" frameborder="0"></iframe>
            </body>
            </html>
        `;
    });

    // 注册销毁时的清理操作
    context.subscriptions.push({
        dispose: () => {
            server.close();
            console.log('[byeefree] 服务已停止');
            vscode.window.showInformationMessage('byeefree服务已停止');
        }
    });
}

// 停用扩展时调用的函数
export function deactivate() {}
