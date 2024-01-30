<template>
  <a-modal v-model:open="newOpen" :confirm-loading="confirmLoading" :keyboard="false" :maskClosable="false"
           title="添加数据源"
           @cancel="resetForm" @ok="handleOk">
    <a-form
        ref="formRef"
        :labelCol="{span: 4}"
        :model="formState"
        :rules="rules"
        labelAlign="right"
    >
      <a-form-item
          label="数据类型"
          name="sourceType"
      >
        <a-select v-model:value="formState.sourceType" @change="sourceTypeChange">
          <a-select-option v-for="item in sourceList" :key="item.key" :value="item.key">
            <a-space>
              <icon-font :type="item.icon"/>
              {{ item.key }}
            </a-space>
          </a-select-option>
        </a-select>
      </a-form-item>
      <a-form-item
          label="地址"
          name="address"
      >
        <source-address v-model:value="formState.address"></source-address>
      </a-form-item>
      <a-form-item
          label="账号"
          name="username"
      >
        <a-input v-model:value="formState.username"></a-input>
      </a-form-item>
      <a-form-item
          label="密码"
          name="password"
      >
        <a-input v-model:value="formState.password"></a-input>
      </a-form-item>
      <a-form-item
          label="数据库"
          name="database"
      >
        <a-input v-model:value="formState.database"></a-input>
      </a-form-item>
    </a-form>
  </a-modal>
</template>
<script lang="ts" setup>
import {computed, reactive, ref} from 'vue';
import {createFromIconfontCN} from '@ant-design/icons-vue';
import type {Rule} from 'ant-design-vue/es/form';
import SourceAddress from "./sourceAddress.vue";
import {invoke} from "@tauri-apps/api/tauri";
import {message} from "ant-design-vue";

const confirmLoading = ref<boolean>(false);

const formRef = ref()
const checkUrl = (_: any, value: { url: string }) => {
  if (value.url) {
    return Promise.resolve();
  }
  return Promise.reject(new Error('请输入地址'));
};
const rules: Record<string, Rule[]> = {
  sourceType: [{required: true, message: '请选择数据源', trigger: 'change'},],
  username: [{required: true, message: '请输入账号', trigger: 'change'},],
  password: [{required: true, message: '请输入密码', trigger: 'change'},],
  address: [{required: true, validator: checkUrl, trigger: 'change'},],
  database: [{required: true, message: '请输入数据库', trigger: 'change'},],
};
const sourceTypeChange = (v: string) => {
  formState.address.port = sourceList.value.find(i => i.key == v)!.port
}

const IconFont = createFromIconfontCN({
  scriptUrl: import.meta.env.VITE_ICON_FONT,
});
const newOpen = computed({
  get() {
    return props.modelValue;
  },
  set(value) {
    emit("update:modelValue", value);
  },
});
const emit = defineEmits(['update:modelValue'])
const props = defineProps({
  modelValue: {
    type: Boolean,
    required: true
  }
})

const sourceList = ref([{
  icon: 'swemysql',
  key: 'mysql',
  label: 'mysql',
  port: '3306'
}])

interface FormState {
  sourceType: string,
  username: string,
  password: string,
  address: {
    url: string,
    port: number | String
  },
  database: string
}


const formState = reactive<FormState>({
  sourceType: '',
  username: '',
  password: '',
  address: {
    url: '',
    port: 0
  },
  database: ''
});
const handleOk = () => {
  formRef.value
      .validate()
      .then(async () => {
        confirmLoading.value = true;

        invoke("test_data_source", formState).then(() => {
          message.success('连接成功！')
          confirmLoading.value = false;
        }).catch(e => {
          message.error(e)
          confirmLoading.value = false;
        })
      })
};
const resetForm = () => {
  cancel()
  formRef.value.resetFields()
}
const cancel = () => {
  newOpen.value = false
  confirmLoading.value = false;
}
</script>
<style lang="less" scoped>

</style>