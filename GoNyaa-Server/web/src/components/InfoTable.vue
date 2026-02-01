<template>
  <el-container>
    <el-header style="height: auto; padding: 10px;">
      <!-- 操作栏 -->
      <el-card shadow="hover" :body-style="{ padding: '10px' }" style="margin-bottom: 10px">
        <div class="toolbar-row">
          <el-button :icon="HomeFilled" circle @click="refreshPage" title="刷新"/>

          <el-divider direction="vertical"/>

          <el-radio-group v-model="sortBy" @change="handleSortChange">
            <el-radio-button label="uploading">
              <el-icon><SortUp/></el-icon> 上传数
            </el-radio-button>
            <el-radio-button label="downloading">
              <el-icon><SortDown/></el-icon> 下载数
            </el-radio-button>
          </el-radio-group>

          <el-divider direction="vertical"/>

          <el-button @click="toSearchPage">
            <el-icon><Search/></el-icon> 搜索页
          </el-button>

          <el-switch
              v-model="autoSet"
              style="--el-switch-on-color: #13ce66"
              inline-prompt
              active-text="自动已阅"
              inactive-text="自动已阅"
          />

          <div class="spacer"></div>

          <el-button-group>
            <el-button type="info" plain @click="openDialog" :loading="tableLoading" :icon="Setting" title="MGStage番号配置"/>
            <el-button type="success" plain @click="handleSave" :loading="tableLoading" :icon="UploadFilled" title="保存已阅状态"/>
            <el-button type="warning" plain @click="handleClearCache" :loading="tableLoading" :icon="Refresh" title="清除缓存"/>
            <el-button type="primary" plain @click="handleExport" :loading="tableLoading" :icon="Download" title="导出已阅列表"/>
            <el-button type="danger" plain @click="openImportDialog" :loading="tableLoading" :icon="FolderAdd" title="导入已阅列表"/>
          </el-button-group>
        </div>
      </el-card>

      <!-- 筛选与分页栏 -->
      <el-card shadow="never" :body-style="{ padding: '10px' }">
        <div class="toolbar-row">
          <div class="filter-group">
            <span class="filter-label">已阅</span>
            <el-radio-group v-model="filters.viewed" size="small">
              <el-radio-button label="all">全部</el-radio-button>
              <el-radio-button label="unviewed">未阅</el-radio-button>
              <el-radio-button label="viewed">已阅</el-radio-button>
            </el-radio-group>
          </div>

          <el-divider direction="vertical"/>

          <div class="filter-group">
            <span class="filter-label">FC2</span>
            <el-radio-group v-model="filters.fc2" size="small">
              <el-radio-button label="all">全部</el-radio-button>
              <el-radio-button label="only">仅FC2</el-radio-button>
              <el-radio-button label="exclude">排除</el-radio-button>
            </el-radio-group>
          </div>

          <el-tag v-if="hasFilter" size="small" type="info" effect="plain" style="margin-left: 5px">
            {{ filteredData.length }} / {{ pageData.arr.length }}
          </el-tag>
          <el-button
              v-if="hasFilter"
              type="primary"
              link
              size="small"
              @click="resetFilters"
          >
            重置
          </el-button>

          <div class="spacer"></div>

          <el-dropdown @command="handleBatchView" size="small" split-button type="danger" :loading="batchViewing">
            <el-icon><View/></el-icon> 批量检阅
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="official">官方</el-dropdown-item>
                <el-dropdown-item command="third1">第三方1</el-dropdown-item>
                <el-dropdown-item command="third2">第三方2</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>

          <el-divider direction="vertical"/>

          <el-tag type="info" effect="plain">第 {{ currentPage }} 页</el-tag>
          <el-button-group>
            <el-button type="primary" :loading="tableLoading" @click="goToFirstPage" :disabled="currentPage === 1">
              <el-icon><ArrowLeftBold/></el-icon>
            </el-button>
            <el-button type="primary" :loading="tableLoading" @click="handlePageChange(-1)" :disabled="currentPage === 1">
              <el-icon><ArrowLeft/></el-icon>
            </el-button>
            <el-button type="primary" :loading="tableLoading" @click="handlePageChange(1)">
              <el-icon><ArrowRight/></el-icon>
            </el-button>
          </el-button-group>
        </div>
      </el-card>
    </el-header>
    <el-main>
      <div v-loading="tableLoading">
        <el-row justify="end">
          <el-col :span="3">

          </el-col>
        </el-row>
        <el-backtop :right="100" :bottom="100"/>
        <el-table :data="filteredData" stripe style="width: 100%"
                  table-layout="auto">
          <el-table-column label="已阅" width="65">
            <template #default="scope">
              <el-switch v-model="scope.row.viewed"
                         :active-icon="Check"
                         inline-prompt
                         @change="handleViewed(scope.row, true)"
                         :loading="viewChanging"
                         :inactive-icon="Close"/>
            </template>
          </el-table-column>
          <el-table-column prop="fanHao" label="番号" width="90"/>
          <el-table-column prop="size" label="大小" width="79"/>
          <el-table-column prop="date" label="上传日期" width="100"/>
          <el-table-column label="检阅" width="175">
            <template #default="scope">
              <el-link
                  type="primary"
                  :href="scope.row.viewLink"
                  target="_blank"
                  @click.middle="handleViewed(scope.row, false)"
                  @click.left="handleViewed(scope.row, false)">官方
              </el-link>
              |
              <el-link type="primary" :href="scope.row.viewLink2" target="_blank"
                       @click.middle="handleViewed(scope.row, false)"
                       @click.left="handleViewed(scope.row, false)">第三方1
              </el-link>
              |
              <el-link type="primary" :href="scope.row.viewLink3" target="_blank"
                       @click.middle="handleViewed(scope.row, false)"
                       @click.left="handleViewed(scope.row, false)">第三方2
              </el-link>
            </template>
          </el-table-column>
          <el-table-column prop="title" label="标题"></el-table-column>
          <el-table-column prop="upCnt" label="上传数" width="70"></el-table-column>
          <el-table-column prop="downCnt" label="下载数" width="70"></el-table-column>
          <el-table-column prop="finCnt" label="完成数" width="70"></el-table-column>
          <el-table-column label="磁力" width="100">
            <template #default="scope">
              <el-button type="primary" @click="magnetDownload(scope.row.magnetLink)" circle>
                <el-icon>
                  <Magnet/>
                </el-icon>
              </el-button>
              <el-button type="primary" @click="copy2Clipboard(scope.row.magnetLink)" circle>
                <el-icon><CopyDocument /></el-icon>
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
      <el-dialog
          v-model="dialogVisible"
          title="MGStage番号前缀列表"
          width="50%"
      >
        <p>不在这个列表里面的官方渠道检阅会去fanza而不是mgstage</p>
        <el-tag
            v-for="tag in mgsList.arr"
            :key="tag"
            class="mx-1"
            closable
            :disable-transitions="false"
            @close="handleClose(tag)"
        >
          {{ tag }}
        </el-tag>
        <el-input
            v-if="inputVisible"
            ref="InputRef"
            v-model="inputValue"
            class="ml-1 w-20"
            size="small"
            @keyup.enter="handleInputConfirm"
            @blur="handleInputConfirm"
        />
        <el-button v-else class="button-new-tag ml-1" size="small" @click="showInput">
          + New Tag
        </el-button>
        <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialogVisible = false" :loading="dialogSaveLoading">取消</el-button>
        <el-button type="primary" @click="saveMgsList" :loading="dialogSaveLoading">
          保存
        </el-button>
      </span>
        </template>
      </el-dialog>

      <!-- 导入已阅列表对话框 -->
      <el-dialog
          v-model="importDialogVisible"
          title="导入已阅列表"
          width="400px"
      >
        <el-form label-position="top">
          <el-form-item label="导入模式">
            <el-radio-group v-model="importMode">
              <el-radio label="append">追加模式（保留现有数据）</el-radio>
              <el-radio label="overwrite">覆盖模式（清空现有数据）</el-radio>
            </el-radio-group>
          </el-form-item>
          <el-form-item label="选择文件">
            <el-upload
                ref="uploadRef"
                action="#"
                :auto-upload="false"
                :on-change="handleFileChange"
                :limit="1"
                accept=".txt"
            >
              <el-button type="primary">选择文件</el-button>
              <template #tip>
                <div class="el-upload__tip">
                  请选择之前导出的 .txt 文件，支持分号或换行分隔的番号列表
                </div>
              </template>
            </el-upload>
          </el-form-item>
        </el-form>
        <template #footer>
          <span class="dialog-footer">
            <el-button @click="importDialogVisible = false">取消</el-button>
            <el-button type="primary" @click="handleImport" :loading="importLoading" :disabled="!selectedFile">
              导入
            </el-button>
          </span>
        </template>
      </el-dialog>
    </el-main>
  </el-container>
</template>

<script lang="ts">
import {defineComponent, nextTick, reactive, ref, computed} from 'vue'
import {changeViewed, clearCache, getData, getMGSList, saveMemory, saveMGSList, exportViewedFile, importViewedFile} from "@/components/CommonFunctions";
import {VideoInfo} from "@/components/CommonTypes";
import {Check, Close, SortUp, SortDown, Search, Setting, UploadFilled, Refresh, HomeFilled, ArrowLeftBold, View, Download, FolderAdd} from '@element-plus/icons-vue'
import {ElInput, ElNotification} from 'element-plus'
import router from '@/router';

export default defineComponent({
  setup() {
    const currentPage = ref(1)
    const sortBy = ref('uploading')
    const tableLoading = ref(true)
    const pageData = reactive<{ arr: Array<VideoInfo> }>({arr: []})
    const input = ref('')
    const autoSet = ref(true)
    const viewChanging = ref(false)
    const batchViewing = ref(false)
    const mgsList = reactive<{ arr: string[] }>({arr: []})
    const dialogVisible = ref(false)
    const dialogSaveLoading = ref(false)

    const handleSortChange = () => {
      initData()
    }

    function magnetDownload(url: string) {
      // window.open(url)
      // window.open(url, '_blank')
      window.location.assign(url)
    }

    function getMgsList() {
      getMGSList().then(res => {
        if (res.code == 200) {
          mgsList.arr = res.msg
          console.log(res.msg)
        } else {
          ElNotification({
            title: '服务端错误',
            message: '获取MGStage番号列表失败了',
            type: 'error',
          })
        }
      })
    }

    function openDialog() {
      getMgsList()
      dialogVisible.value = true;
    }

    function saveMgsList() {
      dialogSaveLoading.value = true
      saveMGSList(mgsList.arr).then(res => {
        if (res.code == 200) {
          ElNotification({
            title: '列表保存成功',
            message: '',
            type: 'success',
          })
        }else {
          ElNotification({
            title: '列表保存失败',
            message: '',
            type: 'error',
          })
        }
      })
      dialogSaveLoading.value = false
      dialogVisible.value = false
    }

    function initData() {
      tableLoading.value = true
      getData(sortBy.value, currentPage.value).then(res => {
        if (res.code == 200) {
          pageData.arr = res.msg.voList
          tableLoading.value = false
        } else {
          ElNotification({
            title: '获取数据失败',
            message: '',
            type: 'warning',
          })
        }
      })
    }

    function changeViewedLocal(banGo: string, viewedValue: boolean) {
      // 查找所有相同番号的项并同步状态（包括筛选后的数据）
      pageData.arr.forEach(item => {
        if (item.fanHao === banGo) {
          item.viewed = viewedValue
        }
      })
    }

    function handleViewed(row: VideoInfo, fromSwitch: boolean) {
      viewChanging.value = true
      try {
        // 已经阅过的可以不改了
        if (!fromSwitch && row.viewed == true) return;
        if (!fromSwitch && autoSet.value == false) return;

        // 发送请求修改服务端数据
        changeViewed(row.fanHao).then(res => {
          if (res.code == 200) {
            // 同步所有相同番号项的状态
            // 开关点击：Vue已自动改变row.viewed，使用当前值
            // 链接点击：强制设为true（因为row.viewed还是false）
            const newViewedValue = fromSwitch ? row.viewed : true
            changeViewedLocal(row.fanHao, newViewedValue)
          } else {
            // 失败则提示
            ElNotification({
              title: '修改已阅失败',
              message: '',
              type: 'error',
            })
          }
        })
      } finally {
        viewChanging.value = false
      }
    }

    function handleClearCache() {
      tableLoading.value = true
      clearCache().then(res => {
        if (res.code == 200) {
          initData()
          ElNotification({
            title: '清除成功',
            message: '',
            type: 'success',
          })
        } else {
          ElNotification({
            title: '清除失败',
            message: '',
            type: 'warning',
          })
        }
      })
      tableLoading.value = false
    }

    function handleSave() {
      tableLoading.value = true
      saveMemory().then(res => {
        if (res.code == 200) {
          ElNotification({
            title: '保存成功',
            message: '',
            type: 'success',
          })
        } else {
          ElNotification({
            title: '保存失败',
            message: '',
            type: 'warning',
          })
        }
      })
      tableLoading.value = false
    }

    function handlePageChange(target: number) {
      tableLoading.value = true
      const targetPage = currentPage.value + target < 1 ? 1 : currentPage.value + target;
      getData(sortBy.value, targetPage).then(res => {
        if (res.code == 200) {
          pageData.arr = res.msg.voList
        } else {
          ElNotification({
            title: '翻页失败',
            message: '',
            type: 'warning',
          })
        }
        tableLoading.value = false
        currentPage.value = targetPage
      })
    }

    function goToFirstPage() {
      if (currentPage.value === 1) return
      tableLoading.value = true
      getData(sortBy.value, 1).then(res => {
        if (res.code == 200) {
          pageData.arr = res.msg.voList
          currentPage.value = 1
        } else {
          ElNotification({
            title: '返回首页失败',
            message: '',
            type: 'warning',
          })
        }
        tableLoading.value = false
      })
    }

    async function handleBatchView(command: string) {
      const type = command as 'official' | 'third1' | 'third2'

      // 获取当前筛选结果中未阅的项
      const unviewed = filteredData.value.filter(row => !row.viewed)

      if (unviewed.length === 0) {
        ElNotification({
          title: '提示',
          message: '当前列表没有未阅项',
          type: 'info'
        })
        return
      }

      // 限制数量，避免打开太多
      const limit = 10
      const toView = unviewed.slice(0, limit)

      if (unviewed.length > limit) {
        ElNotification({
          title: '提示',
          message: `当前有${unviewed.length}个未阅项，本次只处理前${limit}个`,
          type: 'warning'
        })
      }

      // 确定链接字段
      const linkField: keyof VideoInfo = {
        official: 'viewLink',
        third1: 'viewLink2',
        third2: 'viewLink3'
      }[type] as keyof VideoInfo

      batchViewing.value = true
      let opened = 0
      let failed = 0

      for (const row of toView) {
        try {
          // 后台打开标签页
          const newWindow = window.open(row[linkField] as string, '_blank')
          if (newWindow) {
            opened++
            // 只有成功打开才标记已阅
            if (!row.viewed) {
              const res = await changeViewed(row.fanHao)
              if (res.code === 200) {
                row.viewed = true
              }
            }
          } else {
            // 被浏览器拦截，跳过已阅标记
            failed++
          }
        } catch (e) {
          console.error('批量检阅失败:', e)
          failed++
        }

        // 小延迟，避免浏览器拦截
        await new Promise(r => setTimeout(r, 200))
      }

      batchViewing.value = false

      if (failed > 0) {
        ElNotification({
          title: '批量检阅完成',
          message: `成功打开${opened}个页面并标记已阅，${failed}个被浏览器拦截`,
          type: 'warning'
        })
      } else {
        ElNotification({
          title: '批量检阅完成',
          message: `已打开${opened}个页面并标记已阅`,
          type: 'success'
        })
      }
    }

    function copy2Clipboard(content: string) {
      navigator.clipboard.writeText(content)
    }

    const inputValue = ref('')
    const inputVisible = ref(false)
    const InputRef = ref<InstanceType<typeof ElInput>>()

    // 导入对话框相关
    const importDialogVisible = ref(false)
    const importMode = ref<'append' | 'overwrite'>('append')
    const importLoading = ref(false)
    const selectedFile = ref<File | null>(null)
    const uploadRef = ref<any>(null)

    const handleClose = (tag: string) => {
      const copy = JSON.parse(JSON.stringify(mgsList.arr))
      copy.splice(copy.indexOf(tag), 1)
      mgsList.arr = copy
    }

    const showInput = () => {
      inputVisible.value = true
      nextTick(() => {
        InputRef.value!.input!.focus()
      })
    }

    const handleInputConfirm = () => {
      if (inputValue.value) {
        const copy = JSON.parse(JSON.stringify(mgsList.arr))
        copy.push(inputValue.value)
        mgsList.arr = copy
      }
      inputVisible.value = false
      inputValue.value = ''
    }

    function toSearchPage() {
      router.push('/search')
    }

    // 导出已阅列表
    function handleExport() {
      exportViewedFile()
      ElNotification({
        title: '导出成功',
        message: '已阅列表已开始下载',
        type: 'success',
      })
    }

    // 打开导入对话框
    function openImportDialog() {
      importDialogVisible.value = true
      selectedFile.value = null
      importMode.value = 'append'
    }

    // 文件选择变化
    function handleFileChange(uploadFile: any) {
      selectedFile.value = uploadFile.raw
    }

    // 执行导入
    async function handleImport() {
      if (!selectedFile.value) {
        ElNotification({
          title: '请选择文件',
          message: '',
          type: 'warning',
        })
        return
      }

      importLoading.value = true
      try {
        const res = await importViewedFile(selectedFile.value, importMode.value)
        if (res.code === 200) {
          ElNotification({
            title: '导入成功',
            message: res.msg,
            type: 'success',
          })
          importDialogVisible.value = false
          // 刷新页面数据以反映新的已阅状态
          initData()
        } else {
          ElNotification({
            title: '导入失败',
            message: res.msg,
            type: 'error',
          })
        }
      } catch (e) {
        ElNotification({
          title: '导入失败',
          message: '请检查文件格式是否正确',
          type: 'error',
        })
      } finally {
        importLoading.value = false
        // 清空文件列表
        if (uploadRef.value) {
          uploadRef.value.clearFiles()
        }
        selectedFile.value = null
      }
    }

    function refreshPage() {
      initData()
      ElNotification({
        title: '刷新成功',
        message: '数据已重新加载',
        type: 'success',
        duration: 1500
      })
    }

    // 筛选状态 - 两个独立维度
    const filters = reactive({
      viewed: 'all',   // 'all' | 'unviewed' | 'viewed'
      fc2: 'all'       // 'all' | 'only' | 'exclude'
    })

    // 是否有筛选激活
    const hasFilter = computed(() => {
      return filters.viewed !== 'all' || filters.fc2 !== 'all'
    })

    // 重置筛选
    const resetFilters = () => {
      filters.viewed = 'all'
      filters.fc2 = 'all'
    }

    // 组合筛选逻辑
    const filteredData = computed(() => {
      return pageData.arr.filter(row => {
        // 维度1：已阅状态筛选
        if (filters.viewed === 'unviewed' && row.viewed) return false
        if (filters.viewed === 'viewed' && !row.viewed) return false

        // 维度2：FC2筛选
        const isFC2 = row.fanHao.toUpperCase().startsWith('FC2')
        if (filters.fc2 === 'only' && !isFC2) return false
        if (filters.fc2 === 'exclude' && isFC2) return false

        return true
      })
    })

    initData()

    return {
      handleViewed, magnetDownload, handleClose, inputValue, inputVisible,
      pageData, tableLoading, showInput, handleInputConfirm,
      Check, Close, SortUp, SortDown, Search, Setting, UploadFilled, Refresh, HomeFilled, ArrowLeftBold, View, Download, FolderAdd,
      autoSet, saveMgsList, openDialog,
      currentPage, dialogVisible,copy2Clipboard,
      handleSortChange, handleClearCache, handleSave, handlePageChange, refreshPage, goToFirstPage, handleBatchView,
      input, viewChanging, mgsList, dialogSaveLoading,toSearchPage, batchViewing,
      filters, hasFilter, resetFilters, filteredData, sortBy,
      // 导入导出
      importDialogVisible, importMode, importLoading, selectedFile, uploadRef,
      handleExport, openImportDialog, handleFileChange, handleImport
    }
  }
});
</script>

<style scoped>
.toolbar-row {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.spacer {
  flex: 1;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-label {
  font-size: 12px;
  color: #606266;
  white-space: nowrap;
}
</style>