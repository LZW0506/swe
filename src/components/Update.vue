<template>
  <div>
    <a-modal v-model:open="newModelValue" title="更新" @ok="update"  :confirm-loading="confirmLoading" :closable="false" :keyboard="false" :maskClosable="false">
      <p>{{content}}</p>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { relaunch } from '@tauri-apps/api/process';
import {computed, ref, watch} from "vue";
const emit = defineEmits(["update:modelValue"]);
const confirmLoading = ref<boolean>(false);
const props = defineProps({
  modelValue: {
    type: Boolean,
    default: false,
  }
});
const content = ref('检测到新版本，是否立即更新?')
const newModelValue = computed({
  get: () => props.modelValue,
  set: (value) => {
    emit("update:modelValue", value);
  },
})
watch(() => props.modelValue, (value) => {
  if (value) {
    confirmLoading.value = false
    content.value = '检测到新版本，是否立即更新?'
  }
})
const update = async () => {
  try {
    confirmLoading.value = true
    content.value = '正在检测更新..'
    const { shouldUpdate } = await checkUpdate();
    if (shouldUpdate) {
      // 显示正在更新的提示或加载页面
      content.value = '正在更新中...'
      // 安装更新
      await installUpdate();
      content.value = '更新完成，正在重启应用...'
      // 重新启动应用
      await relaunch();
      confirmLoading.value = false
    }
  } catch (error) {
    content.value = '更新失败：'+error
    confirmLoading.value = false
  }
};
</script>
