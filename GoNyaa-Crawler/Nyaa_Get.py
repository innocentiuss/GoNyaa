# coding:utf-8

from flask import Flask, jsonify, request
import requests
import base64
from bs4 import BeautifulSoup
from bs4 import Tag
import re

app = Flask(__name__)
# 解决乱码
app.config['JSON_AS_ASCII'] = False


@app.route('/app')
def server():
    page = request.args.get('page')
    sort = request.args.get('sort')
    full_text = requests.get("https://sukebei.nyaa.si/?s=" + sort + "&o=desc&c=2_2&p=" + page).text
    table_list_bs = BeautifulSoup(full_text, 'lxml').html.body.find_all('tbody')[0].contents
    table_list_bs = [i for i in table_list_bs if isinstance(i, Tag)]
    li = main_process(table_list_bs)
    li = [i.get_dict() for i in li]
    return jsonify(li)


@app.route('/search')
def search_api():
    page = request.args.get('page')
    sort = request.args.get('sort')
    keyword = request.args.get('keyword')
    full_text = requests.get("https://sukebei.nyaa.si/?s=" + sort + "&o=desc&c=2_2&p=" + page + "&q=" + keyword).text
    table_list_bs = BeautifulSoup(full_text, 'lxml').html.body.find_all('tbody')[0].contents
    table_list_bs = [i for i in table_list_bs if isinstance(i, Tag)]
    li = main_process(table_list_bs)
    li = [i.get_dict() for i in li]
    return jsonify(li)


class Result:
    def __init__(self, id, title, size, magnet_url, upload_time, upload_cnt, download_cnt, downloaded):
        self.id = id
        self.title = title
        self.size = size
        self.magnet_url = magnet_url
        self.upload_time = upload_time
        self.upload_cnt = upload_cnt
        self.download_cnt = download_cnt
        self.downloaded = downloaded

    def get_dict(self):
        return self.__dict__


def main_process(table_list_bs):
    res_list = []
    for tr in table_list_bs:
        # pre process
        temp = tr.find_all(title=True, class_=False)
        full_title = temp[1].string

        # get id
        id = get_id_from_title(full_title)
        if id is None:
            continue

        # get title
        title = get_title_from_full_title(full_title, id)
        # get_other_inf(tr)
        size, date, up_cnt, down_cnt, finish_cnt, magnet = get_other_inf(tr)
        res = Result(id, title, size, magnet, date, up_cnt, down_cnt, finish_cnt)
        res_list.append(res)
    return res_list


def get_id_from_title(full_title):
    matched = re.findall('fc2-ppv-.......|[a-z]?[0-9]*[a-z]*-[0-9]*', full_title, flags=re.I)
    if len(matched) > 0 and len(matched[0]) > 6:
        return matched[0]
    return None


def get_title_from_full_title(full_title, fan_hao):
    # 将标题先utf8转成byte流, 再base64
    raw = full_title.split(fan_hao)[1]
    return base64.b64encode(raw.encode('utf-8')).decode('utf-8')  # 返回str形式


# 分别为大小、时间、上传数、下载数、完成数、磁力链接
def get_other_inf(tr):
    td_list = tr.find_all(class_='text-center')
    mag_link = td_list[0].find_all('a')[-1].get('href')
    return td_list[1].string, td_list[2].string, td_list[3].string, td_list[4].string, td_list[5].string, mag_link


if __name__ == '__main__':
    app.run('0.0.0.0', port=5001)
