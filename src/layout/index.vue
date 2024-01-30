<template>
  <a-layout has-sider>
    <a-layout-sider class="sider" theme="light">
      <div class="add-source">
        <a-button type="primary" @click="addSource">添加数据源</a-button>
      </div>
      <a-menu v-if="items!.length>0" :items="items" mode="inline">
        
      </a-menu>
      <a-empty v-else :image="simpleImage"/>
    </a-layout-sider>
    <a-layout-content>
      <router-view></router-view>
    </a-layout-content>
  </a-layout>
  <add-source-dialog v-model="openAdd" @get-list="getList"></add-source-dialog>
</template>
<script lang="ts" setup>
import {Empty, MenuProps} from "ant-design-vue";
import {h, onMounted, ref} from "vue";
import AddSourceDialog from "./components/addSourceDialog.vue";
import {sourceDb} from "../utils/localforage.ts";
import {FormState} from "./types";
import SvgIcon from "../components/SvgIcon.vue";

const simpleImage = Empty.PRESENTED_IMAGE_SIMPLE;


const openAdd = ref<boolean>(false)
const items = ref<MenuProps['items']>([])
onMounted(() => {
  getList()
})
const getList = () => {
  items.value = []
  sourceDb.iterate(function (value: string, key: string) {
    let objectValue: FormState = JSON.parse(value)
    items.value?.push({
      key,
      icon: () => h(SvgIcon, {type: objectValue.address.icon}),
      label: objectValue.name
    })
  })
}
// 添加数据源
const addSource = () => {
  openAdd.value = true
}
</script>
<style lang="less" scoped>
.sider {
  height: 100vh;
  overflow: hidden;

  .add-source {
    padding: 20px 0;
    display: flex;
    justify-content: center;
  }
}
</style>
