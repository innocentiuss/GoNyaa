package com.bubble.gonyaa.controller;

import cn.hutool.http.HttpStatus;
import com.alibaba.fastjson.JSON;
import com.bubble.gonyaa.model.vo.WebResponse;
import com.bubble.gonyaa.repository.MongoService;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;

@Controller
@ConditionalOnProperty(name = "data.save.mode", havingValue = "mongodb")
public class MigrationController {

    private final MongoService mongoService;

    public MigrationController(MongoService mongoService) {
        this.mongoService = mongoService;
    }

    @GetMapping("/migration/sync/memory/mongo")
    public String syncMemory2Mongo() {
        try {
            mongoService.syncFile2Mongo();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "sync ok"));
        } catch (Exception e) {
            e.printStackTrace();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, e.getMessage()));
        }
    }

    @GetMapping("/migration/sync/list/mongo")
    public String syncList2Mongo() {
        try {
            mongoService.syncList2Mongo();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "sync ok"));
        } catch (Exception e) {
            e.printStackTrace();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, e.getMessage()));
        }
    }
}
