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
      ],
    }),
  ],
});
