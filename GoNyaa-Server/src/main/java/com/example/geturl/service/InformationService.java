package com.example.geturl.service;

import cn.hutool.http.HttpRequest;
import com.alibaba.fastjson.JSON;
import com.alibaba.fastjson.JSONArray;
import com.alibaba.fastjson.JSONObject;
import com.example.geturl.model.VideoInfoVo;
import com.example.geturl.model.VideoInformation;
import com.example.geturl.utils.CommonUtils;
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

    public List<VideoInformation> getInfos(String page) {
        String body = HttpRequest.get("http://" + pythonFlaskIp + ":" + pythonFlaskPort + "/app?page=" + page).execute().body();
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
            videoInformation.setTitle(jsonObject.getString("title"));
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
            result.add(vo);
        }
        return result;
    }

    public List<VideoInfoVo> access(String page) {
        if (cacheService.contains(page))
            return cacheService.get(page);

        List<VideoInfoVo> voList = transToInfo(getInfos(page));
        cacheService.put(page, voList);
        return voList;
    }
}
