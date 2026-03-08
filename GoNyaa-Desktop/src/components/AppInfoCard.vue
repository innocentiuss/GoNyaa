<template>
  <el-card shadow="never" class="status-card">
    <div class="info-card-header">
      <div>
        <h2>App Settings</h2>
        <p>Desktop storage paths and remote crawler configuration.</p>
      </div>
      <div class="info-card-actions">
        <el-button :icon="Setting" @click="$emit('open-crawler-settings')">Crawler Settings</el-button>
        <el-button :icon="FolderOpened" @click="$emit('open-data-dir')">Open Data Directory</el-button>
      </div>
    </div>

    <el-descriptions :column="2" border>
      <el-descriptions-item label="Crawler URL">
        {{ info?.crawlerConfigured ? info.crawlerBaseUrl : 'Not configured' }}
      </el-descriptions-item>
      <el-descriptions-item label="Crawler Status">
        {{ info?.crawlerConfigured ? 'Configured' : 'Configuration required' }}
      </el-descriptions-item>
      <el-descriptions-item label="Data Directory">{{ info?.dataDir ?? 'Loading' }}</el-descriptions-item>
      <el-descriptions-item label="memory.txt">{{ info?.memoryPath ?? 'Loading' }}</el-descriptions-item>
      <el-descriptions-item label="MGSList.txt">{{ info?.mgsListPath ?? 'Loading' }}</el-descriptions-item>
    </el-descriptions>
  </el-card>
</template>

<script setup lang="ts">
import { FolderOpened, Setting } from '@element-plus/icons-vue'
import type { AppInfo } from '@/types/app'

defineProps<{
  info?: AppInfo
}>()

defineEmits<{
  (event: 'open-data-dir'): void
  (event: 'open-crawler-settings'): void
}>()
</script>

<style scoped>
.info-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
  margin-bottom: 12px;
}

.info-card-header h2 {
  margin: 0 0 4px;
  font-size: 18px;
}

.info-card-header p {
  margin: 0;
  color: #5b6472;
}

.info-card-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
  justify-content: flex-end;
}
</style>
