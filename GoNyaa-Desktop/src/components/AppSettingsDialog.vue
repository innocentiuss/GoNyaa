<template>
  <el-dialog
    :model-value="modelValue"
    title="应用设置"
    width="760px"
    top="5vh"
    :show-close="!required && !saving"
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    @close="emit('update:modelValue', false)"
  >
    <div class="settings-body">
      <el-alert v-if="required" type="warning" :closable="false" title="请先填写爬虫地址，再使用列表和搜索。" />
      <el-tabs v-if="preferences" v-model="activeTab">
        <el-tab-pane label="列表显示" name="display">
          <el-form label-position="top" :disabled="saving">
            <el-form-item label="完成数高亮">
              <el-switch v-model="preferences.completionHighlight.enabled" />
              <span class="switch-tip">用低饱和淡绿色标识达到条件的整行</span>
            </el-form-item>
            <el-form-item label="完成数阈值">
              <el-input-number
                v-model="preferences.completionHighlight.threshold"
                :min="0"
                :max="4294967295"
                :precision="0"
                :step="100"
                controls-position="right"
              />
            </el-form-item>
            <p class="field-tip">完成数严格大于此数值时高亮，等于时不高亮。首页和搜索结果均生效。</p>
          </el-form>
        </el-tab-pane>
        <el-tab-pane label="浏览器" name="browser">
          <el-form label-position="top" :disabled="saving">
            <el-form-item label="打开预览链接的浏览器">
              <el-select v-model="preferences.browser.kind" style="width: 100%" @change="handleBrowserChange">
                <el-option label="系统默认浏览器" value="default" />
                <el-option label="Google Chrome" value="chrome" />
                <el-option label="Microsoft Edge" value="edge" />
                <el-option label="Mozilla Firefox" value="firefox" />
              </el-select>
            </el-form-item>
            <el-form-item label="无痕模式">
              <el-switch v-model="preferences.browser.privateMode" :disabled="preferences.browser.kind === 'default'" />
              <span class="switch-tip">{{ preferences.browser.kind === 'default' ? '选择具体浏览器后可开启' : '使用无痕 / InPrivate 窗口打开预览链接' }}</span>
            </el-form-item>
            <el-form-item v-if="preferences.browser.kind !== 'default'" label="浏览器程序路径（可选）">
              <div class="path-row">
                <el-input v-model="preferences.browser.executablePath" clearable placeholder="留空则自动查找所选浏览器；也可指定便携版路径" />
                <el-button :loading="picking" @click="pickExecutable">选择文件</el-button>
              </div>
              <p class="field-tip">请选择与上方类型一致的浏览器程序。单条预览和批量打开均使用此设置。</p>
            </el-form-item>
          </el-form>
        </el-tab-pane>
        <el-tab-pane label="第三方预览链接" name="links">
          <p class="field-tip">用 <code>{id}</code> 代入番号，例如 ABC-123。支持 http / https 地址，番号会自动进行 URL 编码。</p>
          <el-form label-position="top" :disabled="saving">
            <section v-for="(provider, index) in providers" :key="index" class="provider-section">
              <el-form-item :label="`链接 ${index + 1} 名称`">
                <el-input v-model="provider.name" maxlength="24" show-word-limit />
              </el-form-item>
              <el-form-item label="URL 模板">
                <el-input v-model="provider.urlTemplate" placeholder="https://example.com/search?q={id}" />
              </el-form-item>
              <p class="field-tip preview">示例：<code>{{ provider.urlTemplate.split('{id}').join('ABC-123') }}</code></p>
            </section>
            <el-button :loading="resetting" @click="resetLinks">恢复默认链接</el-button>
          </el-form>
        </el-tab-pane>
        <el-tab-pane label="爬虫服务" name="crawler">
          <el-form label-position="top" :disabled="saving">
            <el-form-item label="爬虫地址">
              <el-input v-model="localUrl" placeholder="http://127.0.0.1:5001" />
            </el-form-item>
            <p class="field-tip">例如 http://127.0.0.1:5001 或 https://crawler.example.com</p>
          </el-form>
        </el-tab-pane>
      </el-tabs>
      <el-empty v-else description="正在加载设置…" />
    </div>
    <template #footer>
      <el-button v-if="!required" :disabled="saving" @click="emit('update:modelValue', false)">取消</el-button>
      <el-button type="primary" :loading="saving" :disabled="!preferences || picking || resetting" @click="handleSave">保存</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { desktopApi } from '@/api/desktop'
import { useNotifications } from '@/composables/useNotifications'
import type { AppPreferences, SettingsDraft } from '@/types/app'

const props = defineProps<{
  modelValue: boolean
  initialUrl: string
  initialPreferences?: AppPreferences
  required: boolean
  saving: boolean
}>()
const emit = defineEmits<{
  (event: 'update:modelValue', value: boolean): void
  (event: 'save', value: SettingsDraft): void
}>()
const { notifyError, notifyWarning } = useNotifications()
const localUrl = ref('')
const preferences = ref<AppPreferences>()
const activeTab = ref('browser')
const picking = ref(false)
const resetting = ref(false)
const providers = computed(() => preferences.value
  ? [preferences.value.thirdPartyLinks.primary, preferences.value.thirdPartyLinks.secondary]
  : [])

watch(() => props.modelValue, async (visible) => {
  if (!visible) return
  localUrl.value = props.initialUrl
  activeTab.value = props.required ? 'crawler' : 'browser'
  preferences.value = undefined
  try {
    const initial = props.initialPreferences ?? (await desktopApi.getDefaultPreferences()).msg
    preferences.value = JSON.parse(JSON.stringify(initial)) as AppPreferences
  } catch (error) {
    notifyError('加载设置失败', error)
  }
})

function handleBrowserChange() {
  if (!preferences.value) return
  preferences.value.browser.executablePath = ''
  if (preferences.value.browser.kind === 'default') preferences.value.browser.privateMode = false
}

async function pickExecutable() {
  picking.value = true
  try {
    const response = await desktopApi.pickBrowserExecutable()
    if (response.msg && preferences.value) preferences.value.browser.executablePath = response.msg
  } catch (error) {
    notifyError('选择浏览器失败', error)
  } finally {
    picking.value = false
  }
}

async function resetLinks() {
  resetting.value = true
  try {
    const response = await desktopApi.getDefaultPreferences()
    if (preferences.value) preferences.value.thirdPartyLinks = response.msg.thirdPartyLinks
  } catch (error) {
    notifyError('恢复默认链接失败', error)
  } finally {
    resetting.value = false
  }
}

function handleSave() {
  if (!preferences.value || props.saving) return
  const threshold = preferences.value.completionHighlight.threshold
  if (!Number.isInteger(threshold) || threshold < 0 || threshold > 4294967295) {
    activeTab.value = 'display'
    notifyWarning('完成数阈值需为 0–4294967295 的整数')
    return
  }
  if (!localUrl.value.trim()) {
    activeTab.value = 'crawler'
    notifyWarning('请填写爬虫地址')
    return
  }
  emit('save', { crawlerBaseUrl: localUrl.value, preferences: preferences.value })
}
</script>

<style scoped>
.settings-body { max-height: 64vh; overflow-y: auto; padding: 0 4px; }
.path-row { display: flex; gap: 8px; width: 100%; }
.switch-tip { margin-left: 12px; color: #5b6472; }
.field-tip { margin: 8px 0 16px; color: #5b6472; font-size: 13px; }
.provider-section { margin: 16px 0; padding: 16px; border: 1px solid #dce3eb; border-radius: 8px; }
.provider-section .el-form-item:last-of-type { margin-bottom: 8px; }
.preview { margin-bottom: 0; overflow-wrap: anywhere; }
</style>
