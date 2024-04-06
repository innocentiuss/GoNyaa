package com.bubble.gonyaa.config;

import com.alibaba.fastjson.JSON;
import com.bubble.gonyaa.model.vo.WebResponse;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

@RestControllerAdvice
@Slf4j
public class GlobalExceptionHandler {

    @ExceptionHandler(Exception.class)
    public String exceptionHandler(Exception e) {
        log.error("api execute exception", e);
        return JSON.toJSONString(new WebResponse<>(500, e.getMessage()));
    }
}
