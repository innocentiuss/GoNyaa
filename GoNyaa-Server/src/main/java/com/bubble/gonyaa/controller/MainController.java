package com.bubble.gonyaa.controller;


import cn.hutool.http.HttpStatus;
import com.alibaba.fastjson.JSON;
import com.bubble.gonyaa.model.vo.TableDataVo;
import com.bubble.gonyaa.model.vo.VideoInfoVo;
import com.bubble.gonyaa.model.vo.WebResponse;
import com.bubble.gonyaa.service.*;
import org.springframework.web.bind.annotation.*;


import java.util.List;

@RestController
public class MainController {

    private final InformationService informationService;
    private final MemoryService memoryService;
    private final BanGoService banGoService;

    public MainController(InformationService informationService, MemoryService memoryService, BanGoService banGoService) {
        this.informationService = informationService;
        this.memoryService = memoryService;
        this.banGoService = banGoService;
    }

    @GetMapping("/api/list")
    public String getList(@RequestParam(defaultValue = "1", required = false) String page,
                          @RequestParam(defaultValue = "uploading", required = false) String sort) {

        try {
            List<VideoInfoVo> voList = informationService.getInfo(page, sort);
            int curPage = Integer.parseInt(page);
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, new TableDataVo(voList, curPage)));
        } catch (Exception e) {
            e.printStackTrace();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, e.getMessage()));
        }
    }

    @GetMapping("/api/change")
    public String setViewed(@RequestParam String banGo) {
        try {
            if (memoryService.isViewed(banGo)) memoryService.unsetViewed(banGo);
            else memoryService.setViewed(banGo);
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "change ok"));
        } catch (Exception e) {
            e.printStackTrace();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, "change error"));
        }
    }

    @GetMapping("/api/clear")
    public String clearCache() {
        try {
            informationService.cleanCache();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "clear ok"));
        } catch (Exception e) {
            e.printStackTrace();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, "clear error"));
        }
    }

    @GetMapping("/api/save")
    public String saveMemoryAPI() {
        try {
            memoryService.save();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "save ok"));
        } catch (Exception e) {
            e.printStackTrace();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, "save error"));
        }
    }

    @GetMapping("/api/getMGSList")
    public String getMGSList() {
        try {
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, banGoService.getMGSList()));
        } catch (Exception e) {
            e.printStackTrace();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, "get list error"));
        }
    }

    @PostMapping("/api/saveMGSList")
    public String addMGSBanGo(@RequestBody List<String> mgsList) {
        try {
            banGoService.saveList(mgsList);
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "add ok"));
        } catch (Exception e) {
            e.printStackTrace();
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, e.getMessage()));
        }
    }

}
