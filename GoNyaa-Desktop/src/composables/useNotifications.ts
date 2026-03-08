import { ElNotification } from 'element-plus'

export function useNotifications() {
  function notifySuccess(title: string, message = '') {
    ElNotification({
      title,
      message,
      type: 'success',
    })
  }

  function notifyWarning(title: string, message = '') {
    ElNotification({
      title,
      message,
      type: 'warning',
    })
  }

  function notifyError(title: string, error: unknown) {
    ElNotification({
      title,
      message: error instanceof Error ? error.message : String(error),
      type: 'error',
    })
  }

  return {
    notifySuccess,
    notifyWarning,
    notifyError,
  }
}