package com.bubble.gonyaa.model.dto;

import com.alibaba.fastjson.annotation.JSONField;
import lombok.Data;

@Data
public class CrawlerResponseItem {
    @JSONField(name = "download_cnt")
    private String downloadCnt;
    private String downloaded;
    private String id;
    @JSONField(name = "magnet_url")
    private String magnetUrl;
    private String size;
    private String title;
    @JSONField(name = "upload_cnt")
    private String uploadCnt;
    @JSONField(name = "upload_time")
    private String uploadTime;
}
