import { invoke } from '@tauri-apps/api/core'
import type { AppInfo, AppPreferences, ImportMode, SettingsDraft, SortKey, TableData, WebResponse } from '@/types/app'

async function invokeResponse<T>(command: string, args?: Record<string, unknown>) {
  return invoke<WebResponse<T>>(command, args)
}

export const desktopApi = {
  fetchList(page: number, sort: SortKey) {
    return invokeResponse<TableData>('fetch_list', { page, sort })
  },
  searchList(page: number, sort: SortKey, keyword: string) {
    return invokeResponse<TableData>('search_list', { page, sort, keyword })
  },
  toggleViewed(banGo: string) {
    return invokeResponse<string>('toggle_viewed', { banGo })
  },
  saveMemory() {
    return invokeResponse<string>('save_memory')
  },
  clearCache() {
    return invokeResponse<string>('clear_cache')
  },
  getMgsList() {
    return invokeResponse<string[]>('get_mgs_list')
  },
  saveMgsList(list: string[]) {
    return invokeResponse<string>('save_mgs_list', { list })
  },
  exportViewed() {
    return invokeResponse<string>('export_viewed')
  },
  importViewed(mode: ImportMode) {
    return invokeResponse<string>('import_viewed', { mode })
  },
  openExternal(url: string, keepForeground = false) {
    return invokeResponse<string>('open_external', { url, keepForeground })
  },
  openMagnet(url: string) {
    return invokeResponse<string>('open_magnet', { url })
  },
  openDataDir() {
    return invokeResponse<string>('open_data_dir')
  },
  getAppInfo() {
    return invokeResponse<AppInfo>('get_app_info')
  },
  saveSettings(draft: SettingsDraft) {
    return invokeResponse<AppInfo>('save_app_settings', { ...draft })
  },
  getDefaultPreferences() {
    return invokeResponse<AppPreferences>('get_default_preferences')
  },
  pickBrowserExecutable() {
    return invokeResponse<string | null>('pick_browser_executable')
  },
}
