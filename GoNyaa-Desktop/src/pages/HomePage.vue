<template>
  <div class="page-shell">
    <div class="page-header" style="align-items: center;">
      <h1 style="margin: 0;">GoNyaa Desktop</h1>
      <el-space wrap>
        <el-button :icon="Search" type="primary" @click="router.push('/search')">Search</el-button>
        <el-button :icon="Refresh" @click="loadList(currentPage)">Refresh</el-button>
        <el-button :icon="Setting" @click="openCrawlerDialog(false)">设置</el-button>
        <el-button :icon="FolderOpened" @click="handleOpenDataDir">Data Dir</el-button>
      </el-space>
    </div>

    <el-card shadow="hover">
      <div class="toolbar-row">
        <el-radio-group v-model="sortBy" @change="handleSortChange">
          <el-radio-button label="uploading">Uploads</el-radio-button>
          <el-radio-button label="downloading">Downloads</el-radio-button>
          <el-radio-button label="finished">Finished</el-radio-button>
        </el-radio-group>

        <el-switch
          v-model="autoViewed"
          inline-prompt
          active-text="Auto viewed"
          inactive-text="Auto viewed"
        />

        <div class="toolbar-spacer"></div>

        <el-button-group>
          <el-button :icon="Setting" @click="openMgsDialog">MGStage Prefixes</el-button>
          <el-button :icon="UploadFilled" @click="handleSaveMemory">Save Viewed</el-button>
          <el-button :icon="Delete" @click="handleClearCache">Clear Cache</el-button>
          <el-button :icon="Download" @click="handleExportViewed">Export Viewed</el-button>
          <el-button :icon="Upload" @click="importDialogVisible = true">Import Viewed</el-button>
        </el-button-group>
      </div>
    </el-card>

    <el-card shadow="never">
      <div class="toolbar-row">
        <div class="filter-block">
          <span>Viewed</span>
          <el-radio-group v-model="filters.viewed" size="small">
            <el-radio-button label="all">All</el-radio-button>
            <el-radio-button label="unviewed">Unviewed</el-radio-button>
            <el-radio-button label="viewed">Viewed</el-radio-button>
          </el-radio-group>
        </div>

        <div class="filter-block">
          <span>FC2</span>
          <el-radio-group v-model="filters.fc2" size="small">
            <el-radio-button label="all">All</el-radio-button>
            <el-radio-button label="only">Only FC2</el-radio-button>
            <el-radio-button label="exclude">Exclude FC2</el-radio-button>
          </el-radio-group>
        </div>

        <el-tag v-if="hasFilter" type="info" effect="plain">{{ filteredRows.length }} / {{ rows.length }}</el-tag>
        <el-button v-if="hasFilter" link type="primary" @click="resetFilters">Reset</el-button>

        <div class="toolbar-spacer"></div>

        <el-dropdown split-button type="danger" @command="handleBatchOpenCommand">
          Batch Open Unviewed
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="viewLink">Official</el-dropdown-item>
              <el-dropdown-item command="viewLink2">{{ appInfo?.preferences.thirdPartyLinks.primary.name ?? 'JavStore' }}</el-dropdown-item>
              <el-dropdown-item command="viewLink3">{{ appInfo?.preferences.thirdPartyLinks.secondary.name ?? 'JavArchive' }}</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>

        <el-tag type="info" effect="plain">Page {{ currentPage }}</el-tag>
        <el-button-group>
          <el-button :disabled="currentPage === 1" @click="loadList(1)">First</el-button>
          <el-button :disabled="currentPage === 1" @click="loadList(currentPage - 1)">Prev</el-button>
          <el-button @click="loadList(currentPage + 1)">Next</el-button>
        </el-button-group>
      </div>
    </el-card>

    <el-card shadow="never" class="table-card" v-loading="tableLoading">
      <VideoTable
        :rows="filteredRows"
        :third-party-links="appInfo?.preferences.thirdPartyLinks"
        :completion-highlight="appInfo?.preferences.completionHighlight"
        @toggle-viewed="handleViewed"
        @open-link="handleOpenLink"
        @open-magnet="handleOpenMagnet"
        @copy-magnet="handleCopyMagnet"
      />
    </el-card>

    <AppSettingsDialog
      v-model="crawlerDialogVisible"
      :initial-url="crawlerBaseUrlDraft"
      :initial-preferences="appInfo?.preferences"
      :required="crawlerDialogRequired"
      :saving="crawlerSaving"
      @save="handleSaveSettings"
    />

    <el-dialog v-model="mgsDialogVisible" title="MGStage Prefixes" width="560px">
      <el-alert type="info" :closable="false" title="Use one prefix per line. Saving writes back to MGSList.txt." />
      <el-input v-model="mgsText" type="textarea" :rows="12" style="margin-top: 16px" />
      <template #footer>
        <el-button @click="mgsDialogVisible = false">Cancel</el-button>
        <el-button type="primary" :loading="dialogSaving" @click="handleSaveMgsList">Save</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="importDialogVisible" title="Import Viewed List" width="420px">
      <el-radio-group v-model="importMode">
        <el-radio label="append">Append</el-radio>
        <el-radio label="overwrite">Overwrite</el-radio>
      </el-radio-group>
      <p class="dialog-tip">After confirmation, the desktop app opens a file dialog and accepts semicolon or newline separated text files.</p>
      <template #footer>
        <el-button @click="importDialogVisible = false">Cancel</el-button>
        <el-button type="primary" :loading="importLoading" @click="handleImportViewed">Choose File</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import {
  Delete,
  Download,
  FolderOpened,
  Refresh,
  Search,
  Setting,
  Upload,
  UploadFilled,
} from '@element-plus/icons-vue'
import { useRouter } from 'vue-router'
import { desktopApi } from '@/api/desktop'
import AppSettingsDialog from '@/components/AppSettingsDialog.vue'
import VideoTable from '@/components/VideoTable.vue'
import { useNotifications } from '@/composables/useNotifications'
import { useVideoActions } from '@/composables/useVideoActions'
import type { AppInfo, ImportMode, LinkField, SettingsDraft, SortKey, VideoInfo } from '@/types/app'

const router = useRouter()
const { notifyError, notifySuccess, notifyWarning } = useNotifications()

const appInfo = ref<AppInfo>()
const currentPage = ref(1)
const sortBy = ref<SortKey>('uploading')
const rows = ref<VideoInfo[]>([])
const tableLoading = ref(false)
const autoViewed = ref(true)
const importMode = ref<ImportMode>('append')
const importLoading = ref(false)
const importDialogVisible = ref(false)
const mgsDialogVisible = ref(false)
const mgsText = ref('')
const dialogSaving = ref(false)
const crawlerDialogVisible = ref(false)
const crawlerDialogRequired = ref(false)
const crawlerBaseUrlDraft = ref('')
const crawlerSaving = ref(false)

const filters = reactive({
  viewed: 'all',
  fc2: 'all',
})

const filteredRows = computed(() =>
  rows.value.filter((row) => {
    if (filters.viewed === 'unviewed' && row.viewed) return false
    if (filters.viewed === 'viewed' && !row.viewed) return false

    const isFc2 = row.fanHao.toUpperCase().startsWith('FC2')
    if (filters.fc2 === 'only' && !isFc2) return false
    if (filters.fc2 === 'exclude' && isFc2) return false

    return true
  }),
)

const hasFilter = computed(() => filters.viewed !== 'all' || filters.fc2 !== 'all')

const {
  handleViewed,
  handleOpenLink,
  handleOpenMagnet,
  handleCopyMagnet,
  handleBatchOpen,
} = useVideoActions({ rows, autoViewed })

async function loadAppInfo() {
  try {
    const response = await desktopApi.getAppInfo()
    appInfo.value = response.msg
    crawlerBaseUrlDraft.value = response.msg.crawlerBaseUrl

    return response.msg
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

async function loadList(page: number) {
  if (!ensureCrawlerConfigured()) {
    return
  }

  tableLoading.value = true
  try {
    const response = await desktopApi.fetchList(page, sortBy.value)
    currentPage.value = response.msg.page
    rows.value = response.msg.voList
  } catch (error) {
    notifyError('Failed to load video list', error)
  } finally {
    tableLoading.value = false
  }
}

function resetFilters() {
  filters.viewed = 'all'
  filters.fc2 = 'all'
}

function handleSortChange() {
  loadList(1)
}

async function handleSaveSettings(draft: SettingsDraft) {
  crawlerSaving.value = true
  try {
    const response = await desktopApi.saveSettings(draft)
    appInfo.value = response.msg
    crawlerBaseUrlDraft.value = response.msg.crawlerBaseUrl
    crawlerDialogVisible.value = false
    crawlerDialogRequired.value = false
    notifySuccess('设置已保存')
    await loadList(currentPage.value)
  } catch (error) {
    notifyError('保存设置失败', error)
  } finally {
    crawlerSaving.value = false
  }
}

async function handleSaveMemory() {
  try {
    const response = await desktopApi.saveMemory()
    notifySuccess('Viewed list saved', response.msg)
  } catch (error) {
    notifyError('Failed to save viewed list', error)
  }
}

async function handleClearCache() {
  try {
    const response = await desktopApi.clearCache()
    notifySuccess('Cache cleared', response.msg)
    await loadList(currentPage.value)
  } catch (error) {
    notifyError('Failed to clear cache', error)
  }
}

async function openMgsDialog() {
  try {
    const response = await desktopApi.getMgsList()
    mgsText.value = response.msg.join('\n')
    mgsDialogVisible.value = true
  } catch (error) {
    notifyError('Failed to load MGStage prefixes', error)
  }
}

async function handleSaveMgsList() {
  dialogSaving.value = true
  try {
    const list = mgsText.value
      .split(/\r?\n/)
      .map((item) => item.trim().toUpperCase())
      .filter(Boolean)
    const response = await desktopApi.saveMgsList(list)
    notifySuccess('MGStage prefixes saved', response.msg)
    mgsDialogVisible.value = false
  } catch (error) {
    notifyError('Failed to save MGStage prefixes', error)
  } finally {
    dialogSaving.value = false
  }
}

async function handleExportViewed() {
  try {
    const response = await desktopApi.exportViewed()
    notifySuccess('Export completed', response.msg)
  } catch (error) {
    notifyError('Failed to export viewed list', error)
  }
}

async function handleImportViewed() {
  importLoading.value = true
  try {
    const response = await desktopApi.importViewed(importMode.value)
    notifySuccess('Import completed', response.msg)
    importDialogVisible.value = false
    await loadList(currentPage.value)
  } catch (error) {
    notifyError('Failed to import viewed list', error)
  } finally {
    importLoading.value = false
  }
}

function handleBatchOpenCommand(command: string) {
  if (!ensureCrawlerConfigured()) {
    return
  }

  handleBatchOpen(filteredRows, command as LinkField)
}

async function handleOpenDataDir() {
  try {
    await desktopApi.openDataDir()
  } catch (error) {
    notifyError('Failed to open data directory', error)
  }
}

onMounted(async () => {
  const info = await loadAppInfo()
  if (info?.crawlerConfigured) {
    await loadList(1)
  }
})
</script>

