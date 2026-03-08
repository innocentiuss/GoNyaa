<template>
  <el-dialog
    :model-value="modelValue"
    title="Crawler Settings"
    width="520px"
    :show-close="!required"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    @close="emit('update:modelValue', false)"
  >
    <el-alert
      :type="required ? 'warning' : 'info'"
      :closable="false"
      :title="required ? 'Crawler address is required before using the app.' : 'Saving updates the remote crawler target immediately.'"
    />

    <el-input
      v-model="localUrl"
      style="margin-top: 16px"
      placeholder="http://127.0.0.1:5000"
      @keyup.enter="handleSave"
    />

    <p class="dialog-tip">
      Examples: <code>http://127.0.0.1:5000</code> or <code>https://crawler.example.com</code>
    </p>

    <template #footer>
      <el-button v-if="!required" @click="emit('update:modelValue', false)">Cancel</el-button>
      <el-button type="primary" :loading="saving" @click="handleSave">Save</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  modelValue: boolean
  initialUrl: string
  required: boolean
  saving: boolean
}>()

const emit = defineEmits<{
  (event: 'update:modelValue', value: boolean): void
  (event: 'save', value: string): void
}>()

const localUrl = ref('')

watch(
  () => [props.modelValue, props.initialUrl] as const,
  ([visible, initialUrl]) => {
    if (visible) {
      localUrl.value = initialUrl
    }
  },
  { immediate: true },
)

function handleSave() {
  emit('save', localUrl.value)
}
</script>

<style scoped>
.dialog-tip {
  margin: 12px 0 0;
  color: #5b6472;
}
</style>
