package com.bubble.gonyaa.model.vo;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.List;

@Data
@NoArgsConstructor
@AllArgsConstructor
public class TableDataVo {
    private List<VideoInfoVo> voList;
    private int page;
}
