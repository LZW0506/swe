<template>
  <a-layout has-sider>
    <a-layout-sider class="sider">
      <div class="add-source">
        <a-button type="primary" @click="addSource">添加数据源</a-button>
      </div>
      <a-menu  theme="dark" mode="inline" :items="items"></a-menu>
    </a-layout-sider>
    <a-layout-content >
      <router-view></router-view>
    </a-layout-content>
  </a-layout>
  <add-source-dialog v-model="openAdd"></add-source-dialog>
</template>
<script lang="ts" setup>
import {MenuProps} from "ant-design-vue";
import {ref,h} from "vue";
import { MailOutlined } from '@ant-design/icons-vue';
import AddSourceDialog from "./components/addSourceDialog.vue";

const openAdd = ref<boolean>(false)
const items = ref<MenuProps['items']>([
  {
    key: 'mail',
    icon: () => h(MailOutlined),
    label: 'Navigation One',
    title: 'Navigation One',
  }])
// 添加数据源
const addSource = () => {
  openAdd.value =  true
}
</script>
<style scoped lang="less">
.sider{
  height: 100vh;
  overflow: hidden;
  .add-source{
    padding: 20px 0;
    display: flex;
    justify-content: center;
  }
}
</style>
