<script setup>
import { ref } from "vue";

defineProps({
  msg: String,
});

const count = ref(0);

// 数据示例
const tableData = [
  { wavelength: "320.4690", strength: "5.9652e+03" },
  { wavelength: "323.1660", strength: "5.8518e+03" },
  { wavelength: "325.8610", strength: "6031" },
  { wavelength: "328.5530", strength: "5.8108e+03" },
  { wavelength: "331.2430", strength: "5.8509e+03" },
  { wavelength: "333.9300", strength: "5.7978e+03" },
  { wavelength: "336.6160", strength: "5.9301e+03" },
];
</script>

<template>
  <div class="mdui-container">
    <h1>{{ msg }}</h1>

    <div class="mdui-row">
      <!-- 左侧区域: 图表 -->
      <div class="mdui-col mdui-col-xs-12 mdui-col-md-6">
        <div class="mdui-card">
          <div class="mdui-card-content">
            <h3>声振信号</h3>
            <canvas
              id="sound-graph"
              style="width: 100%; height: 300px"
            ></canvas>
          </div>
        </div>
      </div>

      <!-- 右侧区域: 表格 -->
      <div class="mdui-col mdui-col-xs-12 mdui-col-md-6">
        <div class="mdui-card">
          <div class="mdui-card-content">
            <h3>数据表</h3>
            <table class="mdui-table">
              <thead>
                <tr>
                  <th>Wavelength (nm)</th>
                  <th>Strength</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(item, index) in tableData" :key="index">
                  <td>{{ item.wavelength }}</td>
                  <td>{{ item.strength }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部区域: 按钮和其他信息 -->
    <div class="mdui-row">
      <div class="mdui-col mdui-col-xs-12">
        <div class="mdui-card">
          <div class="mdui-card-content">
            <button
              class="mdui-btn mdui-btn-raised mdui-color-theme"
              @click="count++"
            >
              当前计数: {{ count }}
            </button>
            <p>编辑 <code>components/HelloWorld.vue</code> 以测试 HMR</p>
          </div>
        </div>
      </div>
    </div>

    <p>
      查看
      <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
        >create-vue</a
      >, 官方的 Vue + Vite 启动器。
    </p>
    <p>
      了解更多关于 IDE 对 Vue 的支持，查看
      <a
        href="https://vuejs.org/guide/scaling-up/tooling.html#ide-support"
        target="_blank"
        >Vue 文档中的扩展指南</a
      >。
    </p>
    <p class="read-the-docs">点击 Vite 和 Vue 的 logo 了解更多</p>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}

.mdui-row {
  margin-top: 20px;
}

.mdui-col {
  padding: 10px;
}
</style>
