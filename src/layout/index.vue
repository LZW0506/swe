<template>
  <a-layout has-sider>
    <a-layout-sider class="sider" theme="light">
      <div class="add-source">
        <a-button type="primary" @click="addSource">添加数据源</a-button>
      </div>
      <div v-if="menuItems.length>0" class="menu-box">
        <div v-for="item in menuItems" :key="item.key" :class="selectKey === item.key ? 'menu-item-active':''"
             class="menu-item">
          <div class="flex-box">
            <div class="menu-item-title" @click="menuItemClick(item)">
              <svg-icon :type="item.icon"></svg-icon>
              <div class="nav-text">{{ item.label }}</div>
            </div>
            <EditOutlined class="del-icon" @click="editItem(item)"/>
            <a-popconfirm
                cancel-text="取消"
                ok-text="确认"
                placement="right"
                title="确认删除数据源？"
                @confirm="delItem(item)"
            >
              <DeleteOutlined class="del-icon"/>
            </a-popconfirm>
          </div>
        </div>
      </div>
      <a-empty v-else :image="simpleImage"/>
    </a-layout-sider>
    <a-layout-content>
      <div style="margin-left: 20px">
        <router-view></router-view>
      </div>
    </a-layout-content>
  </a-layout>
  <add-source-dialog v-model="openAdd" :editData="editData" @get-list="getList"></add-source-dialog>
</template>
<script lang="ts" setup>
import {Empty, message} from "ant-design-vue";
import {computed, onMounted, ref} from "vue";
import AddSourceDialog from "./components/addSourceDialog.vue";
import {sourceDb} from "../utils/localforage.ts";
import {editMenuItemType, FormState, menuItemType} from "./types";
import SvgIcon from "../components/SvgIcon.vue";
import {useSourceStore} from "../stores/modules/source.ts";
import {DeleteOutlined, EditOutlined} from '@ant-design/icons-vue';

const simpleImage = Empty.PRESENTED_IMAGE_SIMPLE;

const editData = ref<editMenuItemType | null>(null)
const openAdd = ref<boolean>(false)
const menuItems = ref<menuItemType[]>([])
onMounted(() => {
  getList()
})
const getList = () => {
  menuItems.value = []
  sourceDb.iterate(function (value: string, key: string) {
    let objectValue: FormState = JSON.parse(value)
    menuItems.value?.push({
      key,
      icon: objectValue.address.icon,
      label: objectValue.name
    })
  })
}
const selectKey = computed(() => sourceStore.selectKey)
// 添加数据源
const addSource = () => {
  openAdd.value = true
}
const sourceStore = useSourceStore()
const menuItemClick = (item: menuItemType) => {
  sourceStore.setSelectKey(item!.key)
}
const delItem = (item: menuItemType) => {
  sourceDb.removeItem(<string>item.key).then(() => {
    message.success('删除成功')
    getList()
  }).catch(e => {
    message.error(e)
  })
}
const editItem = (item: menuItemType) => {
  sourceDb.getItem(<string>item.key).then((res) => {
    editData.value = JSON.parse(<string>res)
    editData.value!.uuid = <string>item.key
    openAdd.value = true
  }).catch((err) => {
    message.error(err)
  })
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

.flex-box {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.del-icon {
  font-size: 12px;
  color: rgba(0, 0, 0, 0.45);
}

.del-icon + .del-icon {
  margin-left: 10px;
}

.del-icon:hover {
  font-size: 16px;
  color: #007DFA;
}

.menu-box {
  padding: 10px;

  .menu-item-title {
    width: 100%;
    padding: 10px 0;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .nav-text {
    width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .menu-item {
    padding: 0 10px;
    border-radius: 5px;
  }

  .menu-item:hover {
    background: #EBEBEB;
    cursor: pointer;
  }

  .menu-item-active {
    background: #E5F5FF !important;
  }
}
</style>
