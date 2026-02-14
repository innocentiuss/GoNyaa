import { invoke } from '@tauri-apps/api/core';
import { WebResponse } from "@/components/CommonTypes";

export async function getData(sort: string, page: number): Promise<WebResponse> {
    try {
        return await invoke('get_list', { page: String(page), sort });
    } catch (e) {
        console.error(e);
        return { code: 500, msg: String(e) };
    }
}

export async function searchData(sort: string, page: number, keyword: string): Promise<WebResponse> {
    try {
        return await invoke('search_list', { keyword, page: String(page), sort });
    } catch (e) {
        console.error(e);
        return { code: 500, msg: String(e) };
    }
}

export function magnetDownload(url: string) {
    invoke('open_url', { url });
}

export function openUrl(url: string) {
    invoke('open_url', { url });
}

export function copy2Clipboard(content: string) {
    navigator.clipboard.writeText(content);
}

export async function changeViewed(banGo: string): Promise<WebResponse> {
    try {
        return await invoke('change_viewed', { banGo });
    } catch (e) {
        console.error(e);
        return { code: 500, msg: String(e) };
    }
}

export async function clearCache(): Promise<WebResponse> {
    try {
        return await invoke('clear_cache');
    } catch (e) { // catch invoke error
        return { code: 500, msg: String(e) };
    }
}

export async function saveMemory(): Promise<WebResponse> {
    try {
        return await invoke('save_memory');
    } catch (e) {
        return { code: 500, msg: String(e) };
    }
}

export async function saveMGSList(mgsList: string[]): Promise<WebResponse> {
    try {
        return await invoke('save_mgs_list', { mgsList });
    } catch (e) {
        return { code: 500, msg: String(e) };
    }
}

export async function getMGSList(): Promise<WebResponse> {
    try {
        return await invoke('get_mgs_list');
    } catch (e) {
        return { code: 500, msg: String(e) };
    }
}

/**
 * 导出已阅列表为文件
 */
export async function exportViewedFile() {
    try {
        const res = await invoke<WebResponse<string>>('export_viewed');
        if (res.code === 200) {
            const content = res.msg;
            const blob = new Blob([content as string], { type: 'text/plain;charset=utf-8' });
            const url = URL.createObjectURL(blob);
            const link = document.createElement('a');
            link.href = url;
            link.download = `viewed_export_${new Date().getTime()}.txt`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(url);
        } else {
            console.error(res.msg);
        }
    } catch (e) {
        console.error(e);
    }
}

/**
 * 导入已阅列表文件
 * @param file 要上传的文件
 * @param mode 导入模式：append(追加) 或 overwrite(覆盖)
 */
export async function importViewedFile(file: File, mode: 'append' | 'overwrite' = 'append'): Promise<WebResponse<string>> {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = async (e) => {
            const content = e.target?.result as string;
            if (!content) {
                resolve({ code: 500, msg: "File empty" });
                return;
            }
            try {
                const res = await invoke<WebResponse<string>>('import_viewed', { content, mode });
                resolve(res);
            } catch (err) {
                resolve({ code: 500, msg: String(err) });
            }
        };
        reader.onerror = () => resolve({ code: 500, msg: "Read file failed" });
        reader.readAsText(file);
    });
}
