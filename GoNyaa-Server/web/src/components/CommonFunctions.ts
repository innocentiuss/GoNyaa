import axios from "axios";
import {TableData, WebResponse} from "@/components/CommonTypes";


const host = window.location.hostname;
export const port = '8082';

export async function getData(sort: string, page: number): Promise<WebResponse<TableData>> {
    try {
        const response = await axios.get('http://' + host + ':' + port + '/api/list', {
            params: {
                page: page, sort: sort
            }
        })

        return response.data
    }catch (e) {
        console.log(e)
        throw e
    }
}

export async function searchData(sort: string, page: number, keyword: string) {
    try {
        const response = await axios.get('http://' + host + ':' + port + '/api/search', {
            params: {
                page: page,
                sort: sort,
                keyword: keyword
            }
        })
        return response.data
    } catch(e) {
        console.log(e)
        throw e
    }
}

export function magnetDownload(url: string) {
    // window.open(url)
    // window.open(url, '_blank')
    window.location.assign(url)
}
export function copy2Clipboard(content: string) {
    navigator.clipboard.writeText(content)
}

export async function changeViewed(banGo: string) {
    try {
        const response = await axios.get('http://' + host + ':' + port + '/api/change',
            {params: {banGo: banGo}})
        return response.data
    }catch (e) {
        console.log(e)
        throw e
    }
}

export async function clearCache() {
    try {
        const response = await axios.get('http://' + host + ':' + port + '/api/clear')
        return response.data
    }catch (e) {
        console.log(e)
        throw e
    }
}

export async function saveMemory() {
    try {
        const response = await axios.get('http://' + host + ':' + port + '/api/save')
        return response.data
    }catch (e) {
        console.log(e)
        throw e
    }
}

export async function saveMGSList(mgsList: string[]) {
    try {
        const response = await axios.post('http://' + host + ':' + port + '/api/saveMGSList', mgsList)
        return response.data
    }catch (e) {
        console.log(e)
        throw e
    }
}

export async function getMGSList() {
    try {
        const response = await axios.get('http://' + host + ':' + port + '/api/getMGSList')
        return response.data
    }catch (e) {
        console.log(e)
        throw e
    }
}

/**
 * 导出已阅列表为文件
 */
export function exportViewedFile() {
    // 使用a标签触发下载
    const link = document.createElement('a')
    link.href = 'http://' + host + ':' + port + '/api/exportViewed'
    link.download = ''  // 文件名由服务端设置
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
}

/**
 * 导入已阅列表文件
 * @param file 要上传的文件
 * @param mode 导入模式：append(追加) 或 overwrite(覆盖)
 */
export async function importViewedFile(file: File, mode: 'append' | 'overwrite' = 'append'): Promise<WebResponse<string>> {
    const formData = new FormData()
    formData.append('file', file)
    formData.append('mode', mode)

    try {
        const response = await axios.post('http://' + host + ':' + port + '/api/importViewed', formData, {
            headers: {
                'Content-Type': 'multipart/form-data'
            }
        })
        return response.data
    } catch (e) {
        console.log(e)
        throw e
    }
}

