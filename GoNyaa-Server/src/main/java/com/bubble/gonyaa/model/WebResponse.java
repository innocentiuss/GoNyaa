package com.bubble.gonyaa.model;

import lombok.AllArgsConstructor;
import lombok.Data;

@AllArgsConstructor
@Data
public class WebResponse<T> {
    private int code;
    private T msg;
}
