export type SortKey = 'uploading' | 'downloading' | 'finished'
export type ImportMode = 'append' | 'overwrite'
export type LinkField = 'viewLink' | 'viewLink2' | 'viewLink3'

export interface VideoInfo {
  fanHao: string
  viewLink: string
  size: string
  title: string
  date: string
  upCnt: string
  downCnt: string
  finCnt: string
  magnetLink: string
  isViewed: string
  viewed: boolean
  viewLink2: string
  viewLink3: string
}

export interface TableData {
  page: number
  voList: VideoInfo[]
}

export interface WebResponse<T> {
  code: number
  msg: T
}

export interface AppInfo {
  dataDir: string
  memoryPath: string
  mgsListPath: string
  crawlerBaseUrl: string
  crawlerConfigured: boolean
}
