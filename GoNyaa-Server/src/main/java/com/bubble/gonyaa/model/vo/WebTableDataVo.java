package com.bubble.gonyaa.model.vo;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.List;

@Data
@NoArgsConstructor
@AllArgsConstructor
public class WebTableDataVo {
    private List<VideoInfoItemVo> voList;
    private int page;
}
