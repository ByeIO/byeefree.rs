import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          isCustomElement: (tag) => tag.startsWith("mdui-"),
        },
      },
    }),
    // 复制static文件夹到dist打包目录中
    viteStaticCopy({
      targets: [
        {
          // 依赖包
          src: "./static/*",
          dest: "static",
        },
        {
          // 模型
          src: "./models/*",
          dest: "models",
        },
        // {
        //   // 数据集
        //   src: "./dataset/*",
        //   dest: "dataset",
        // },
        {
          // 人工智能聊天机器人
          src: "./src/deepseek-r1-webgpu/dist/assets/*",
          dest: "assets",
        },
        {
          // 人工智能聊天机器人
          src: "./src/deepseek-r1-webgpu/dist/*",
          dest: "static/chatbot",
        },
        {
          // Jupyter服务端
          src: "./static/jupyterlite/*",
          dest: "static/jupyterlite",
        },
      ],
    }),
  ],
});
