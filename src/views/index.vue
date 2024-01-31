<template>
  <a-spin :spinning="queryLoading">
    <div class="show-content">
      <div></div>
      <a-table :columns="columns" :data-source="tableList" :scroll="{ x: 1300, y: 500 }" bordered></a-table>
    </div>
  </a-spin>

</template>
<script lang="ts" setup>
import {useSourceStore} from "../stores/modules/source.ts";
import {ref, watch} from "vue";
import {FormState, tableInfoType} from "../types/source.ts";
import {sourceDb} from "../utils/localforage.ts";
import {invoke} from "@tauri-apps/api/tauri";
import {message} from "ant-design-vue";

const sourceStore = useSourceStore()

const columns = [
  {
    title: '表名',
    dataIndex: 'Name',
  },
  {
    title: '引擎',
    dataIndex: 'Engine',
  },
  {
    title: '版本',
    dataIndex: 'Version',
  },
  {
    title: '格式',
    dataIndex: 'Row_format',
  }, {
    title: '行数',
    dataIndex: 'Rows',
  },
  // {
  //   title: '创建时间',
  //   key: 'create_time',
  // },
  {
    title: '排序',
    dataIndex: 'Collation',
  }, {
    title: '注释',
    dataIndex: 'Comment',
    fixed: 'right',
  },
];
const sourceData = ref<FormState>()
const tableList = ref<tableInfoType[]>([])
const queryLoading = ref<boolean>(false)
watch(() => sourceStore.selectKey, (v) => {
  if (v) {
    queryLoading.value = true
    sourceDb.getItem(<string>v).then((res) => {
      sourceData.value = JSON.parse(<string>res)
      invoke('query_table_info', <undefined>sourceData.value).then((res) => {
        console.log(res)
        tableList.value = <tableInfoType[]>res
        queryLoading.value = false
      }).catch((e) => {
        message.error(e)
        queryLoading.value = false
      })
    })
  }
})
</script>
<style lang="less" scoped>
.show-content {
  padding: 10px;
  height: 100%;
  background: #ffffff;
  width: 100%;
}
</style>