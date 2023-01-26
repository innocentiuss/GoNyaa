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

export interface WebResponse<T> {
    code: number
    msg: T
}

export interface TableData {
    page: number
    voList: Array<VideoInfo>
}