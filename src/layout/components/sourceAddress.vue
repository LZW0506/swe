<template>
  <a-row :gutter="16">
    <a-col :span="18">
      <a-input :value="value.url" type="text" @change="onUrlChange"/>
    </a-col>
    <a-col :span="6">
      <a-input
          :value="value.port"
          type="number"
          @change="onPortChange"
      ></a-input>
    </a-col>
  </a-row>

</template>

<script lang="ts">
import type {PropType} from 'vue';
import {defineComponent} from 'vue';
import {Form} from 'ant-design-vue';


interface PriceValue {
  url: string;
  port: number | string;
}

export default defineComponent({
  props: {
    value: {type: Object as PropType<PriceValue>, required: true},
  },
  emits: ['update:value'],
  setup(props, {emit}) {
    const formItemContext = Form.useInjectFormItemContext();
    const triggerChange = (changedValue: { url?: string; port?: number | string }) => {
      emit('update:value', {...props.value, ...changedValue});
      formItemContext.onFieldChange();
    };
    const onPortChange = (e: InputEvent) => {
      triggerChange({port: (e.target as any).value});
    };
    const onUrlChange = (e: InputEvent) => {
      triggerChange({url: (e.target as any).value});
    };

    return {
      onPortChange,
      onUrlChange,
    };
  },
});
</script>