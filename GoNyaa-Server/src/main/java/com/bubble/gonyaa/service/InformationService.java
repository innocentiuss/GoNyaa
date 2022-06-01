package com.bubble.gonyaa.service;

import cn.hutool.core.codec.Base64;
import cn.hutool.http.HttpRequest;
import com.alibaba.fastjson.JSON;
import com.alibaba.fastjson.JSONArray;
import com.alibaba.fastjson.JSONObject;
import com.bubble.gonyaa.utils.CommonUtils;
import com.bubble.gonyaa.model.VideoInfoVo;
import com.bubble.gonyaa.model.VideoInformation;
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
    private CacheService cacheService;
    @Autowired
    private MemoryService memoryService;

    public List<VideoInformation> getInfos(String page, String sort) {
        sort = CommonUtils.sortChange(sort);
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
            videoInformation.setType(CommonUtils.getTypeFromId(videoInformation.getFanHao()));
            videoInformation.setViewLink(CommonUtils.getPreviewLink(videoInformation.getFanHao(), videoInformation.getType()));
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
            result.add(vo);
        }
        return result;
    }

    public List<VideoInfoVo> access(String page, String sort) {
        if (cacheService.contains(page + sort))
            return cacheService.get(page + sort);

        List<VideoInfoVo> voList = transToInfo(getInfos(page, sort));
        cacheService.put(page + sort, voList);
        return voList;
    }
}
