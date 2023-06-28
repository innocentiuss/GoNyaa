package com.bubble.gonyaa.config;

import com.mongodb.MongoSocketOpenException;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.core.env.Environment;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;

@ControllerAdvice
@Slf4j
public class GlobalExceptionHandler {

    @Autowired
    private Environment environment;

    @ExceptionHandler(MongoSocketOpenException.class)
    public void handleNotFoundException(MongoSocketOpenException ex) {
        // 如果没配置mongo的连接, 此时自动装配会去配置导致报错, 这里处理一下
        String saveMode = environment.getProperty("data.save.mode");
        if (saveMode != null && saveMode.equals("mongodb")) {
            log.info("连接MongoDB失败，请检查对应的配置是否正确");
            ex.printStackTrace();
            return;
        }
        log.info("检测到未配置MongoDB");
    }
}
