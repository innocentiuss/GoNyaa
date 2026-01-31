<template>
  <el-container>
    <el-header>
      <el-menu
          :default-active="activeIndex"
          class="el-menu-demo"
          mode="horizontal"
          @select="handleMenuSelect"
      >
        <el-menu-item index="1">实时上传数排序</el-menu-item>
        <el-menu-item index="2">实时下载数排序</el-menu-item>
        <el-button style="margin-left: 10px; margin-top: 10px" @click="toSearchPage">进入搜索页</el-button>
        <el-switch v-model="autoSet"
                   style="--el-switch-on-color: #13ce66; margin-top: 10px;margin-left: auto"
                   inline-prompt
                   active-text="自动已阅"
                   inactive-text="自动已阅"
        />
        <el-button
            type="info"
            style="margin-left: 10px; margin-top: 10px"
            plain
            @click="openDialog"
            :loading="tableLoading"
        >
          <el-icon>
            <Setting/>
          </el-icon>
          设置
        </el-button>
        <div style="width: 10px"></div>
        <el-button type="success" plain style="margin-top: 10px;" @click="handleSave" :loading="tableLoading">
          <el-icon>
            <UploadFilled/>
          </el-icon>
          手动保存
        </el-button>
        <el-button type="warning" plain style="margin-top: 10px" @click="handleClearCache" :loading="tableLoading">
          <el-icon>
            <Refresh/>
          </el-icon>
          清除缓存
        </el-button>
        <div style="width: 10px"></div>
        <!-- 筛选控件 -->
        <el-divider direction="vertical" style="margin-top: 10px" />
        <el-radio-group v-model="filters.viewed" size="small" style="margin-top: 10px">
          <el-radio-button label="all">全部</el-radio-button>
          <el-radio-button label="unviewed">仅未阅</el-radio-button>
          <el-radio-button label="viewed">仅已阅</el-radio-button>
        </el-radio-group>
        <el-divider direction="vertical" style="margin-top: 10px" />
        <el-radio-group v-model="filters.fc2" size="small" style="margin-top: 10px">
          <el-radio-button label="all">全部</el-radio-button>
          <el-radio-button label="only">仅FC2</el-radio-button>
          <el-radio-button label="exclude">不看FC2</el-radio-button>
        </el-radio-group>
        <el-tag v-if="hasFilter" size="small" type="info" style="margin-top: 14px">
          {{ filteredData.length }} / {{ pageData.arr.length }}
        </el-tag>
        <el-button
            v-if="hasFilter"
            type="primary"
            link
            size="small"
            style="margin-top: 10px"
            @click="resetFilters"
        >
          重置筛选
        </el-button>

        <div style="width: 20px"></div>
        <el-tag class="ml-2" style=" margin-top:14px">第{{ currentPage }}页</el-tag>
        <div style="width: 10px"></div>
        <el-button-group style=" margin-top:10px">
          <el-button type="primary" :loading="tableLoading" @click="handlePageChange(-1)">
            <el-icon class="el-icon--left">
              <ArrowLeft/>
            </el-icon>
            上一页
          </el-button>
          <el-button type="primary" :loading="tableLoading" @click="handlePageChange(1)">
            下一页
            <el-icon class="el-icon--right">
              <ArrowRight/>
            </el-icon>
          </el-button>
        </el-button-group>
      </el-menu>
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
                         @change="handleViewed(scope.$index, true)"
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
                  @click.middle="handleViewed(scope.$index, false)"
                  @click.left="handleViewed(scope.$index, false)">官方
              </el-link>
              |
              <el-link type="primary" :href="scope.row.viewLink2" target="_blank"
                       @click.middle="handleViewed(scope.$index, false)"
                       @click.left="handleViewed(scope.$index, false)">第三方1
              </el-link>
              |
              <el-link type="primary" :href="scope.row.viewLink3" target="_blank"
                       @click.middle="handleViewed(scope.$index, false)"
                       @click.left="handleViewed(scope.$index, false)">第三方2
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
    </el-main>
  </el-container>
</template>

<script lang="ts">
import {defineComponent, nextTick, reactive, ref, computed} from 'vue'
import {changeViewed, clearCache, getData, getMGSList, saveMemory, saveMGSList} from "@/components/CommonFunctions";
import {VideoInfo} from "@/components/CommonTypes";
import {Check, Close} from '@element-plus/icons-vue'
import {ElInput, ElNotification} from 'element-plus'
import router from '@/router';

export default defineComponent({
  setup() {
    const currentPage = ref(1)
    const sortBy = ref('uploading')
    const tableLoading = ref(true)
    const pageData = reactive<{ arr: Array<VideoInfo> }>({arr: []})
    const activeIndex = ref('1')
    const input = ref('')
    const autoSet = ref(true)
    const viewChanging = ref(false)
    const mgsList = reactive<{ arr: string[] }>({arr: []})
    const dialogVisible = ref(false)
    const dialogSaveLoading = ref(false)

    const handleMenuSelect = (key: string, keyPath: string[]) => {
      if (key == '1' && sortBy.value != 'uploading') {
        sortBy.value = 'uploading'
        initData()
      } else if (key == '2' && sortBy.value != 'downloading') {
        sortBy.value = 'downloading'
        initData()
      }
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

    function changeViewedLocal(banGo: string, passIndex: number) {
      const item = pageData.arr.find((_, index) => index !== passIndex && _.fanHao === banGo)
      if (item) item.viewed = !item.viewed
    }

    function handleViewed(index: number, fromSwitch: boolean) {
      viewChanging.value = true
      try {
        // 已经阅过的可以不改了
        if (!fromSwitch && pageData.arr[index].viewed == true) return;
        if (!fromSwitch && autoSet.value == false) return;

        // 发送请求修改服务端数据
        changeViewed(pageData.arr[index].fanHao).then(res => {
          if (res.code == 200) {
            // 如果成功则修改前端开关 因为从按钮点的话 已经自动变值了 变所有值的时候需要跳过它
            if (fromSwitch) changeViewedLocal(pageData.arr[index].fanHao, index)
            else changeViewedLocal(pageData.arr[index].fanHao, -1)
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

    function copy2Clipboard(content: string) {
      navigator.clipboard.writeText(content)
    }

    const inputValue = ref('')
    const inputVisible = ref(false)
    const InputRef = ref<InstanceType<typeof ElInput>>()

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
      Check, Close, autoSet, saveMgsList, openDialog,
      activeIndex, currentPage, dialogVisible,copy2Clipboard,
      handleMenuSelect, handleClearCache, handleSave, handlePageChange,
      input, viewChanging, mgsList, dialogSaveLoading,toSearchPage,
      filters, hasFilter, resetFilters, filteredData
    }
  }
});
</script>

<style scoped>

</style>