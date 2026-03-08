<template>
  <div class="page-shell">
    <div class="page-header" style="align-items: center;">
      <h1 style="margin: 0;">Search</h1>
      <el-space wrap>
        <el-button :icon="House" @click="router.push('/')">Back Home</el-button>
        <el-button :icon="Setting" @click="openCrawlerDialog(false)">Crawler</el-button>
      </el-space>
    </div>

    <el-card shadow="hover">
      <div class="toolbar-row">
        <el-radio-group v-model="sortBy">
          <el-radio-button label="uploading">Uploads</el-radio-button>
          <el-radio-button label="downloading">Downloads</el-radio-button>
          <el-radio-button label="finished">Finished</el-radio-button>
        </el-radio-group>

        <el-input
          v-model="keyword"
          clearable
          placeholder="Type a keyword and press Enter"
          style="width: 300px;"
          @keyup.enter="handleSearch"
        />

        <el-switch
          v-model="autoViewed"
          inline-prompt
          active-text="Auto viewed"
          inactive-text="Auto viewed"
        />

        <el-button type="primary" :icon="Search" @click="handleSearch">Search</el-button>

        <div class="toolbar-spacer"></div>

        <el-tag type="info" effect="plain">Page {{ currentPage }}</el-tag>
        <el-button-group>
          <el-button :disabled="currentPage === 1" @click="handlePageChange(-1)">Prev</el-button>
          <el-button @click="handlePageChange(1)">Next</el-button>
        </el-button-group>
      </div>
    </el-card>

    <el-card shadow="never" v-loading="loading">
      <VideoTable
        :rows="rows"
        @toggle-viewed="handleViewed"
        @open-link="handleOpenLink"
        @open-magnet="handleOpenMagnet"
        @copy-magnet="handleCopyMagnet"
      />
    </el-card>

    <CrawlerSettingsDialog
      v-model="crawlerDialogVisible"
      :initial-url="crawlerBaseUrlDraft"
      :required="crawlerDialogRequired"
      :saving="crawlerSaving"
      @save="handleSaveCrawlerBaseUrl"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { House, Search, Setting } from '@element-plus/icons-vue'
import { useRouter } from 'vue-router'
import { desktopApi } from '@/api/desktop'
import CrawlerSettingsDialog from '@/components/CrawlerSettingsDialog.vue'
import VideoTable from '@/components/VideoTable.vue'
import { useNotifications } from '@/composables/useNotifications'
import { useVideoActions } from '@/composables/useVideoActions'
import type { AppInfo, SortKey, VideoInfo } from '@/types/app'

const router = useRouter()
const { notifyError, notifySuccess, notifyWarning } = useNotifications()
const appInfo = ref<AppInfo>()
const keyword = ref('')
const sortBy = ref<SortKey>('uploading')
const currentPage = ref(1)
const loading = ref(false)
const autoViewed = ref(true)
const rows = ref<VideoInfo[]>([])
const crawlerDialogVisible = ref(false)
const crawlerDialogRequired = ref(false)
const crawlerBaseUrlDraft = ref('')
const crawlerSaving = ref(false)

const {
  handleViewed,
  handleOpenLink,
  handleOpenMagnet,
  handleCopyMagnet,
} = useVideoActions({ rows, autoViewed })

async function loadAppInfo() {
  try {
    const response = await desktopApi.getAppInfo()
    appInfo.value = response.msg
    crawlerBaseUrlDraft.value = response.msg.crawlerBaseUrl
  } catch (error) {
    notifyError('Failed to load app info', error)
  }
}

function openCrawlerDialog(required: boolean) {
  crawlerDialogRequired.value = required
  crawlerBaseUrlDraft.value = appInfo.value?.crawlerBaseUrl ?? ''
  crawlerDialogVisible.value = true
}

function ensureCrawlerConfigured() {
  if (appInfo.value?.crawlerConfigured) {
    return true
  }

  openCrawlerDialog(true)
  notifyWarning('Crawler address required', 'Please configure the remote crawler address first.')
  return false
}

async function handleSaveCrawlerBaseUrl(url: string) {
  crawlerSaving.value = true
  try {
    const response = await desktopApi.saveCrawlerBaseUrl(url)
    appInfo.value = response.msg
    crawlerBaseUrlDraft.value = response.msg.crawlerBaseUrl
    crawlerDialogVisible.value = false
    crawlerDialogRequired.value = false
    notifySuccess('Crawler address saved', response.msg.crawlerBaseUrl)
  } catch (error) {
    notifyError('Failed to save crawler address', error)
  } finally {
    crawlerSaving.value = false
  }
}

async function runSearch() {
  if (!keyword.value.trim()) {
    notifyWarning('Missing keyword', 'Please enter a search keyword first.')
    return
  }

  if (!ensureCrawlerConfigured()) {
    return
  }

  loading.value = true
  try {
    const response = await desktopApi.searchList(currentPage.value, sortBy.value, keyword.value.trim())
    rows.value = response.msg.voList
    currentPage.value = response.msg.page
  } catch (error) {
    notifyError('Search failed', error)
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  currentPage.value = 1
  runSearch()
}

async function handlePageChange(delta: number) {
  if (!keyword.value.trim()) {
    notifyWarning('Missing keyword', 'Please enter a search keyword before paging.')
    return
  }

  if (!ensureCrawlerConfigured()) {
    return
  }

  currentPage.value = Math.max(1, currentPage.value + delta)
  await runSearch()
}

onMounted(async () => {
  await loadAppInfo()
})
</script>

