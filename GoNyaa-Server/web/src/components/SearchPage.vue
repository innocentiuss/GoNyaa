<template>
<div class="common-layout">
    <el-container>
      <el-header>
        <el-row>
          <el-col :span="8">
            <el-radio-group v-model="sort" @input="changeSortType">
              <el-radio-button :disabled="loading" label="uploading"  :loading="loading">上传量顺序</el-radio-button>
              <el-radio-button :disabled="loading" label="downloading"  :loading="loading" >下载量顺序</el-radio-button>
              <el-radio-button :disabled="loading" label="finished"  :loading="loading" >完成量顺序</el-radio-button>
            </el-radio-group>
          </el-col>
          <el-col :span="8">
            <el-input v-model="keyword" prefix-icon="search" style="width: 80%" @keyup.enter="keywordSearch" clearable placeholder="输入搜索关键字" :disable="loading">
            </el-input>
            <el-button  icon="search" @click="keywordSearch"></el-button>
          </el-col>
          <el-col :span="8">
            <el-button :loading="loading" style="margin-right: 10px;" @click="backToInfoTable">回到采集页</el-button>
            <el-tag style="margin-right: 10px;">第{{ currentPage }}页</el-tag>
            <el-button-group style="">
              <el-button type="primary" :loading="loading" @click="handlePageChange(-1)">
                <el-icon class="el-icon--left">
                  <ArrowLeft/>
                </el-icon>
                上一页
              </el-button>
              <el-button type="primary" :loading="loading" @click="handlePageChange(1)">
                下一页
                <el-icon class="el-icon--right">
                  <ArrowRight/>
                </el-icon>
              </el-button>
            </el-button-group>
          </el-col>
        </el-row>
      </el-header>
      <el-main>
        <el-backtop :right="100" :bottom="100"/>
        <div v-loading="loading">
          <el-table :data="tableData" style="width: 100%">
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
      </el-main>
    </el-container>
  </div>
</template>

<script lang="ts" setup>
import router from '@/router';
import { onMounted, ref, nextTick } from 'vue'
import { searchData, magnetDownload, copy2Clipboard, changeViewed } from './CommonFunctions';
import { ElNotification } from 'element-plus';
import {Check, Close} from '@element-plus/icons-vue'
const keyword = ref('')
const currentPage = ref(1)
const sort = ref('uploading')
const loading = ref(false)
function backToInfoTable() {
  router.push('/')
}
async function changeSortType() {
  if (loading.value) return;
  if (!keyword.value) return;
  loading.value=true
  setTimeout(() => {
    search();
  }, 200); 
}
async function keywordSearch() {
  if (loading.value) {
    return
  }
  if (!keyword.value) {
    ElNotification({
    title: '搜索',
    message: '请先输入搜索关键字',
    type: 'warning'})
    return
  }
  currentPage.value = 1
  await search()
}
const autoSet = ref(true)
const viewChanging = ref(false)
function handleViewed(index: number, fromSwitch: boolean) {
      viewChanging.value = true
      try {
        // 已经阅过的可以不改了
        if (!fromSwitch && tableData.value[index].viewed == true) return;
        if (!fromSwitch && autoSet.value == false) return;

        // 发送请求修改服务端数据
        changeViewed(tableData.value[index].fanHao).then(res => {
          if (res.code == 200) {
            // 如果成功则修改前端开关 因为从按钮点的话 已经自动变值了 变所有值的时候需要跳过它
            if (fromSwitch) changeViewedLocal(tableData.value[index].fanHao, index)
            else changeViewedLocal(tableData.value[index].fanHao, -1)
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
async function search() {
  loading.value = true
  try {
    const res = await searchData(sort.value, currentPage.value, keyword.value)
    if (res.code == 200) {
      tableData.value = res.msg.voList
    }
    else {
      ElNotification({
    title: 'Warning',
    message: '获取数据失败',
    type: 'warning',
  })
    }
  } catch(e) {
    console.log(e)
  } finally {
    loading.value = false
  }
}

async function handlePageChange(delta: number) {
  if (!keyword.value) {
    ElNotification({
    title: '翻页器',
    message: '请先输入搜索关键字',
    type: 'warning'})
    return
  }
  loading.value = true
  const targetPage = currentPage.value + delta < 1 ? 1 : currentPage.value + delta;
  currentPage.value = targetPage
  await search()
  loading.value = false

}
function changeViewedLocal(banGo: string, passIndex: number) {
      const deepCopy = JSON.parse(JSON.stringify(tableData.value))
      for (let i = 0; i < deepCopy.length; i++) {
        if (i == passIndex) continue
        if (deepCopy[i].fanHao == banGo) deepCopy[i].viewed = !deepCopy[i].viewed
      }
      tableData.value = deepCopy
    }
const tableData: any = ref([])

</script>

<style scoped>

</style>