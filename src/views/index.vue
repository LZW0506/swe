<template>
  <a-spin :spinning="queryLoading">
    <div class="show-content">
      <div class="search-input">
        <a-form :model="searchQuery" layout="inline">
          <a-form-item label="表名" name="name">
            <a-input v-model:value="searchQuery.name"></a-input>
          </a-form-item>
          <a-form-item>
            <a-space>
              <a-button type="primary" @click="getList">搜索</a-button>
              <a-button>重置</a-button>
            </a-space>

          </a-form-item>
        </a-form>
      </div>
      <div class="top-search">
        <div>
          <a-button :disabled="state.selectedRowKeys.length === 0" :icon="h(DownloadOutlined )" @click="downWord" >
            导出word
          </a-button>


        </div>
        <div class="right-button">
          <a-tooltip title="刷新">
            <a-button :icon="h(ReloadOutlined)" shape="circle" type="dashed" @click="getList"/>
          </a-tooltip>
        </div>
      </div>
      <a-table
          :columns="columns"
          :data-source="tableList"
          :row-selection="{ selectedRowKeys: state.selectedRowKeys, onChange: onSelectChange }"
          :scroll="{ x: 1300, y: 500 }"
          bordered
          rowKey="Name">
      </a-table>
    </div>
  </a-spin>

</template>
<script lang="ts" setup>
import {useSourceStore} from "../stores/modules/source.ts";
import {h, reactive, ref, watch} from "vue";
import {FormState, tableInfoType} from "../types/source.ts";
import {sourceDb} from "../utils/localforage.ts";
import {invoke} from "@tauri-apps/api/tauri";
import {message} from "ant-design-vue";
import {DownloadOutlined, ReloadOutlined} from '@ant-design/icons-vue';

type Key = string | number;
const sourceStore = useSourceStore()
interface searchType {
  name?: string
}

const searchQuery = reactive<searchType>({
  name: ''
})
const columns = [
  {
    title: '表名',
    dataIndex: 'Name',
    key: 'name'
  },
  {
    title: '引擎',
    dataIndex: 'Engine',
    key: 'engine'
  },
  {
    title: '版本',
    dataIndex: 'Version',
    key: 'version'
  },
  {
    title: '格式',
    dataIndex: 'Row_format',
    key: 'row_format'
  }, {
    title: '行数',
    dataIndex: 'Rows',
    key: 'rows'
  },
  {
    title: '创建时间',
    dataIndex: 'Create_time',
    key: 'create_time',
  },
  {
    title: '排序',
    dataIndex: 'Collation',
    key: 'collation'

  }, {
    title: '注释',
    dataIndex: 'Comment',
    fixed: 'right',
    key: 'comment'
  },
];
const key = ref<string | null | number>(null)
const sourceData = ref<FormState>()
const tableList = ref<tableInfoType[]>([])
const queryLoading = ref<boolean>(false)
watch(() => sourceStore.selectKey, (v) => {
  if (v) {
    key.value = v
    getList()
  }
})
const state = reactive<{
  selectedRowKeys: Key[];
  loading: boolean;
}>({
  selectedRowKeys: [], // Check here to configure the default column
  loading: false,
});
const getList = () => {
  queryLoading.value = true
  sourceDb.getItem(<string>key.value).then((res) => {
    sourceData.value = JSON.parse(<string>res)
    console.log({...sourceData.value, ...searchQuery})
    invoke('query_table_info', {...sourceData.value, ...searchQuery}).then((res) => {
      console.log(res)
      tableList.value = <tableInfoType[]>res
      queryLoading.value = false
    }).catch((e) => {
      message.error(e)
      queryLoading.value = false
    })
  })
}
const onSelectChange = (selectedRowKeys: Key[]) => {
  state.selectedRowKeys = selectedRowKeys;
};
enum downCode {
  success = "Success",
  cancel = "Cancel"
}
const downWord = () => {
  queryLoading.value = true
  invoke('down_word', {...sourceData.value, names: state.selectedRowKeys,...searchQuery}).then((res) => {
    if(res === downCode.success){
      message.success("导出成功！")
    }
    queryLoading.value = false
  }).catch((e) => {
    message.error(e)
    queryLoading.value = false
  })
}
</script>
<style lang="less" scoped>
.top-search {
  display: flex;
  justify-content: space-between;
  margin-bottom: 20px;

  .right-button {
    display: flex;
  }
}

.show-content {
  padding: 10px;
  height: 100%;
  background: #ffffff;
  width: 100%;
}

.search-input {
  margin-bottom: 20px;
}
</style>
