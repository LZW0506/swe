<script lang="ts" setup>
import {createFromIconfontCN} from "@ant-design/icons-vue";

const IconFont = createFromIconfontCN({
  scriptUrl: import.meta.env.VITE_ICON_FONT,
});
defineProps({
  type: {
    type: String,
    required: true
  },
  style: {}

})
</script>

<template>
  <icon-font :type="type"></icon-font>
</template>

<style lang="less" scoped>

</style>
