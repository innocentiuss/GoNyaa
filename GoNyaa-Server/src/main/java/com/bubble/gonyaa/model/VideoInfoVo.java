package com.bubble.gonyaa.model;

import lombok.Getter;
import lombok.Setter;

@Getter
@Setter
public class VideoInfoVo extends VideoExtraInfo{
    private String fanHao;
    private String viewLink;
    private String size;
    private String title;
    private String date;
    private String upCnt;
    private String downCnt;
    private String finCnt;
    private String magnetLink;
    private String isViewed;
}
