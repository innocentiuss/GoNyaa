<template>
  <el-table :data="rows" stripe table-layout="auto" style="width: 100%">
    <el-table-column label="已阅" width="76">
      <template #default="{ row }">
        <el-switch
          :model-value="row.viewed"
          inline-prompt
          :active-icon="Check"
          :inactive-icon="Close"
          @change="emitToggle(row, true)"
        />
      </template>
    </el-table-column>
    <el-table-column prop="fanHao" label="番号" width="110" />
    <el-table-column prop="size" label="大小" width="88" />
    <el-table-column prop="date" label="上传日期" width="120" />
    <el-table-column label="预览" width="220">
      <template #default="{ row }">
        <el-space spacer="|">
          <el-link type="primary" @click.prevent="emitOpenLink(row, 'viewLink')">官方</el-link>
          <el-link type="primary" @click.prevent="emitOpenLink(row, 'viewLink2')">JavStore</el-link>
          <el-link type="primary" @click.prevent="emitOpenLink(row, 'viewLink3')">JavArchive</el-link>
        </el-space>
      </template>
    </el-table-column>
    <el-table-column prop="title" label="标题" min-width="420" />
    <el-table-column prop="upCnt" label="上传" width="78" />
    <el-table-column prop="downCnt" label="下载" width="78" />
    <el-table-column prop="finCnt" label="完成" width="78" />
    <el-table-column label="磁力" width="110">
      <template #default="{ row }">
        <el-space>
          <el-button circle type="primary" @click="$emit('open-magnet', row.magnetLink)">
            <el-icon><Magnet /></el-icon>
          </el-button>
          <el-button circle @click="$emit('copy-magnet', row.magnetLink)">
            <el-icon><CopyDocument /></el-icon>
          </el-button>
        </el-space>
      </template>
    </el-table-column>
  </el-table>
</template>

<script setup lang="ts">
import { Check, Close, CopyDocument, Magnet } from '@element-plus/icons-vue'
import type { LinkField, VideoInfo } from '@/types/app'

defineProps<{
  rows: VideoInfo[]
}>()

const emit = defineEmits<{
  (event: 'toggle-viewed', row: VideoInfo, fromSwitch: boolean): void
  (event: 'open-link', row: VideoInfo, field: LinkField): void
  (event: 'open-magnet', url: string): void
  (event: 'copy-magnet', url: string): void
}>()

function emitToggle(row: VideoInfo, fromSwitch: boolean) {
  row.viewed = !row.viewed
  emit('toggle-viewed', row, fromSwitch)
}

function emitOpenLink(row: VideoInfo, field: LinkField) {
  emit('open-link', row, field)
}
</script>
