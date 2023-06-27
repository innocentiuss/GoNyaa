package com.bubble.gonyaa.model.vo;

import lombok.AllArgsConstructor;
import lombok.Data;

@AllArgsConstructor
@Data
public class WebResponse<T> {
    private int code;
    private T msg;
}
