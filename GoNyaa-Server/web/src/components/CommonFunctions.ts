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

