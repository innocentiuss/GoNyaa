package com.bubble.gonyaa.service;

import cn.hutool.core.codec.Base64;
import cn.hutool.http.HttpRequest;
import com.alibaba.fastjson.JSON;
import com.alibaba.fastjson.JSONArray;
import com.alibaba.fastjson.JSONObject;
import com.bubble.gonyaa.model.vo.VideoInfoVo;
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

    @Value("${python.flask.ip}")
    private String pythonFlaskIp;
    @Value("${python.flask.port}")
    private String pythonFlaskPort;

    @Autowired
    BanGoService banGoService;

    @Autowired
    private CacheService cacheService;
    @Autowired
    private MemoryService memoryService;

    private final TimedCache<List<VideoInfoVo>> cache = new TimedCache<>();

    public List<VideoInformation> getInfos(String page, String sort) {
        sort = BanGoService.sortChange(sort);
        String body = HttpRequest.get("http://" + pythonFlaskIp + ":" + pythonFlaskPort + "/app?page=" + page + "&sort=" + sort).execute().body();

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

    public List<VideoInfoVo> transToInfo(List<VideoInformation> originList) {
        List<VideoInfoVo> result = new ArrayList<>();
        for (VideoInformation originInfo : originList) {
            VideoInfoVo vo = new VideoInfoVo();
            BeanUtils.copyProperties(originInfo, vo);
            // 检查是否已经确认
            boolean flag = memoryService.isViewed(vo.getFanHao());
            vo.setIsViewed(flag ? "√" : "×");
            vo.setViewed(flag);
            vo.setViewLink2(BanGoService.getJavStoreLink(originInfo.getFanHao()));
            vo.setViewLink3(BanGoService.getJavBeeLink(originInfo.getFanHao()));
            result.add(vo);
        }
        return result;
    }

    @Deprecated
    public List<VideoInfoVo> access(String page, String sort) {
        if (cacheService.contains(page + sort)) {
            List<VideoInfoVo> result = cacheService.get(page + sort);
            // 对于view字段要重新判断一下
            for (VideoInfoVo videoInfoVo : result) {
                videoInfoVo.setIsViewed(memoryService.isViewed(videoInfoVo.getFanHao()) ? "√" : "×");
            }
            return result;
        }
        List<VideoInfoVo> voList = transToInfo(getInfos(page, sort));
        cacheService.put(page + sort, voList);
        return voList;
    }

    public List<VideoInfoVo> getInfo(String page, String sort) {
        List<VideoInfoVo> res;
        if ((res = cache.get(page + sort)) != null) {
            for (VideoInfoVo videoInfoVo : res) {
                videoInfoVo.setViewed(memoryService.isViewed(videoInfoVo.getFanHao()));
            }
            return res;
        }

        List<VideoInfoVo> voList = transToInfo(getInfos(page, sort));
        cache.set(page + sort, voList);
        return voList;
    }

    public void cleanCache() {
        cache.clear();
    }
}
