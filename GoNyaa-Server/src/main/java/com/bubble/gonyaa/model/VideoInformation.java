package com.bubble.gonyaa.model;

import com.bubble.gonyaa.enums.Type;
import lombok.Data;

@Data
public class VideoInformation {
    private String fanHao;
    private String viewLink;
    private Type type;
    private String size;
    private String date;
    private String title;
    private String upCnt;
    private String downCnt;
    private String finCnt;
    private String magnetLink;
}
