package com.bubble.gonyaa.service;

import cn.hutool.core.codec.Base64;
import cn.hutool.http.HttpRequest;
import com.alibaba.fastjson.JSON;
import com.alibaba.fastjson.JSONArray;
import com.alibaba.fastjson.JSONObject;
import com.bubble.gonyaa.model.dto.CrawlerResponseItem;
import com.bubble.gonyaa.model.vo.VideoInfoItemVo;
import com.bubble.gonyaa.model.dto.VideoInformation;
import com.bubble.gonyaa.utils.TimedCache;
import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;

@Service
public class InformationService {

    @Value("${backend.ip}")
    private String backendIp;
    @Value("${backend.port}")
    private String backendPort;

    @Autowired
    BanGoService banGoService;
    @Autowired
    private MemoryService memoryService;

    private final TimedCache<List<VideoInfoItemVo>> cache = new TimedCache<>();
    private final TimedCache<List<VideoInfoItemVo>> searchCache = new TimedCache<>();

    public List<VideoInformation> doPullInfo(String page, String sort) {
        sort = BanGoService.sortChange(sort);
        String body = HttpRequest.get("http://" + backendIp + ":" + backendPort + "/app?page=" + page + "&sort=" + sort).execute().body();

        JSONArray jsonArray = JSON.parseArray(body);
        List<VideoInformation> list = new ArrayList<>();
        for (int i = 0; i < jsonArray.size(); i++) {
            JSONObject jsonObject = jsonArray.getJSONObject(i);
            VideoInformation videoInformation = new VideoInformation();
            videoInformation.setDate(jsonObject.getString("upload_time"));
            videoInformation.setFanHao(jsonObject.getString("id").toUpperCase());
            videoInformation.setDownCnt(jsonObject.getString("download_cnt"));
            videoInformation.setUpCnt(jsonObject.getString("upload_cnt"));
            videoInformation.setMagnetLink(jsonObject.getString("magnet_url"));
            videoInformation.setSize(jsonObject.getString("size"));
            videoInformation.setTitle(Base64.decodeStr(jsonObject.getString("title")));  // base64 decode first
            videoInformation.setFinCnt(jsonObject.getString("downloaded"));
            videoInformation.setType(banGoService.getTypeFromId(videoInformation.getFanHao()));
            videoInformation.setViewLink(BanGoService.getPreviewLink(videoInformation.getFanHao(), videoInformation.getType()));
            list.add(videoInformation);
        }
        return list;
    }

    public List<VideoInformation> doSearch(String keyword, String page, String sort) {
        sort = BanGoService.sortChange(sort);
        String body = HttpRequest.get("http://" + backendIp + ":" + backendPort + "/search?page=" + page + "&sort=" + sort + "&keyword=" + keyword).execute().body();
        List<CrawlerResponseItem> response = JSON.parseArray(body, CrawlerResponseItem.class);
        List<VideoInformation> informationList = new ArrayList<>();
        for (CrawlerResponseItem crawlerResponseItem : response) {
            informationList.add(extractInformation(crawlerResponseItem));
        }
        return informationList;
    }

    private VideoInformation extractInformation(CrawlerResponseItem response) {
        VideoInformation information = new VideoInformation();
        information.setDate(response.getUploadTime());
        information.setFanHao(response.getId().toUpperCase());
        information.setDownCnt(response.getDownloadCnt());
        information.setUpCnt(response.getUploadCnt());
        information.setMagnetLink(response.getMagnetUrl());
        information.setSize(response.getSize());
        information.setTitle(Base64.decodeStr(response.getTitle()));
        information.setFinCnt(response.getDownloaded());
        information.setType(banGoService.getTypeFromId(response.getId().toUpperCase()));
        information.setViewLink(BanGoService.getPreviewLink(information.getFanHao(), information.getType()));
        return information;
    }

    public List<VideoInfoItemVo> transToInfo(List<VideoInformation> originList) {
        List<VideoInfoItemVo> result = new ArrayList<>();
        for (VideoInformation originInfo : originList) {
            VideoInfoItemVo vo = new VideoInfoItemVo();
            BeanUtils.copyProperties(originInfo, vo);
            // 检查是否已经确认
            boolean flag = memoryService.isViewed(vo.getFanHao());
            vo.setIsViewed(flag ? "√" : "×");
            vo.setViewed(flag);
            vo.setViewLink2(BanGoService.getJavStoreLink(originInfo.getFanHao()));
            vo.setViewLink3(BanGoService.getJavArchiveLink(originInfo.getFanHao()));
            result.add(vo);
        }
        return result;
    }


    public List<VideoInfoItemVo> getInfo(String page, String sort) {
        List<VideoInfoItemVo> res;
        if ((res = cache.get(page + sort)) != null) {
            for (VideoInfoItemVo videoInfoItemVo : res) {
                videoInfoItemVo.setViewed(memoryService.isViewed(videoInfoItemVo.getFanHao()));
            }
            return res;
        }

        List<VideoInfoItemVo> voList = transToInfo(doPullInfo(page, sort));
        cache.set(page + sort, voList);
        return voList;
    }

    public List<VideoInfoItemVo> searchInfo(String page, String sort, String keyword) {
        List<VideoInfoItemVo> res;
        String key = page + sort + keyword;
        if ((res = searchCache.get(key)) != null) {
            for (VideoInfoItemVo videoInfoItemVo : res) {
                videoInfoItemVo.setViewed(memoryService.isViewed(videoInfoItemVo.getFanHao()));
            }
        }
        List<VideoInfoItemVo> voList = transToInfo(doSearch(keyword, page, sort));
        cache.set(key, voList);
        return voList;
    }

    public void cleanCache() {
        cache.clear();
    }
}
