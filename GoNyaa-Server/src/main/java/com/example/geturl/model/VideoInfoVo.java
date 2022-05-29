package com.example.geturl.model;

import com.example.geturl.enums.Type;
import lombok.Data;

@Data
public class VideoInfoVo {
    private String fanHao;
    private String viewLink;
    private String size;
    private String title;
    private String date;
    private String upCnt;
    private String downCnt;
    private String finCnt;
    private String magnetLink;
}
