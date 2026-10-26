import type { ComputedRef, Ref } from 'vue'
import { desktopApi } from '@/api/desktop'
import { useNotifications } from '@/composables/useNotifications'
import type { LinkField, VideoInfo } from '@/types/app'

interface UseVideoActionsOptions {
  rows: Ref<VideoInfo[]>
  autoViewed: Ref<boolean>
}

export function useVideoActions({ rows, autoViewed }: UseVideoActionsOptions) {
  const { notifyError, notifySuccess } = useNotifications()
  const markingViewed = new Set<string>()

  function syncViewed(banGo: string, viewed: boolean) {
    rows.value = rows.value.map((row) =>
      row.fanHao === banGo
        ? {
            ...row,
            viewed,
            isViewed: viewed ? 'yes' : 'no',
          }
        : row,
    )
  }

  async function handleViewed(row: VideoInfo, fromSwitch: boolean) {
    if (!fromSwitch && row.viewed) {
      return
    }
    if (!fromSwitch && !autoViewed.value) {
      return
    }

    try {
      await desktopApi.toggleViewed(row.fanHao)
      syncViewed(row.fanHao, fromSwitch ? row.viewed : true)
    } catch (error) {
      if (fromSwitch) {
        syncViewed(row.fanHao, !row.viewed)
      }
      notifyError('Failed to update viewed state', error)
    }
  }

  async function markViewedAfterOpen(row: VideoInfo) {
    const currentViewed = rows.value.find((item) => item.fanHao === row.fanHao)?.viewed ?? row.viewed
    if (!autoViewed.value || currentViewed || markingViewed.has(row.fanHao)) {
      return
    }

    markingViewed.add(row.fanHao)
    try {
      await desktopApi.toggleViewed(row.fanHao)
      syncViewed(row.fanHao, true)
    } catch (error) {
      notifyError('Failed to mark item as viewed', error)
    } finally {
      markingViewed.delete(row.fanHao)
    }
  }

  async function handleOpenLink(row: VideoInfo, field: LinkField, keepForeground = false) {
    try {
      await desktopApi.openExternal(row[field], keepForeground)
      await markViewedAfterOpen(row)
    } catch (error) {
      notifyError('Failed to open external link', error)
    }
  }

  async function handleOpenMagnet(url: string) {
    try {
      await desktopApi.openMagnet(url)
    } catch (error) {
      notifyError('Failed to open magnet link', error)
    }
  }

  async function handleCopyMagnet(url: string) {
    try {
      await navigator.clipboard.writeText(url)
      notifySuccess('Magnet link copied')
    } catch (error) {
      notifyError('Failed to copy magnet link', error)
    }
  }

  async function handleBatchOpen(targetRows: ComputedRef<VideoInfo[]>, field: LinkField) {
    const targets = targetRows.value.filter((row) => !row.viewed).slice(0, 10)
    if (!targets.length) {
      notifySuccess('No unviewed items in the current list')
      return
    }

    for (const row of targets) {
      try {
        await desktopApi.openExternal(row[field])
        if (!row.viewed) {
          await desktopApi.toggleViewed(row.fanHao)
          syncViewed(row.fanHao, true)
        }
      } catch (error) {
        notifyError(`Failed to batch-open ${row.fanHao}`, error)
      }
    }

    notifySuccess('Batch open completed', `Processed ${targets.length} items this time`)
  }

  return {
    syncViewed,
    handleViewed,
    handleOpenLink,
    handleOpenMagnet,
    handleCopyMagnet,
    handleBatchOpen,
  }
}
