package com.bubble.gonyaa.controller;


import cn.hutool.http.HttpStatus;
import com.alibaba.fastjson.JSON;
import com.bubble.gonyaa.model.vo.WebTableDataVo;
import com.bubble.gonyaa.model.vo.VideoInfoItemVo;
import com.bubble.gonyaa.model.vo.WebResponse;
import com.bubble.gonyaa.service.*;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;


import java.util.List;

@RestController
@Slf4j
@RequestMapping("/api")
public class MainController {

    private final InformationService informationService;
    private final MemoryService memoryService;
    private final BanGoService banGoService;

    public MainController(InformationService informationService, MemoryService memoryService, BanGoService banGoService) {
        this.informationService = informationService;
        this.memoryService = memoryService;
        this.banGoService = banGoService;
    }

    @GetMapping("/list")
    public String getList(@RequestParam(defaultValue = "1", required = false) String page,
                          @RequestParam(defaultValue = "uploading", required = false) String sort) {

        List<VideoInfoItemVo> voList = informationService.getInfo(page, sort);
        int curPage = Integer.parseInt(page);
        return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, new WebTableDataVo(voList, curPage)));
    }

    @GetMapping("/change")
    public String setViewed(@RequestParam String banGo) {
        if (memoryService.isViewed(banGo)) memoryService.unsetViewed(banGo);
        else memoryService.setViewed(banGo);
        return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "change ok"));
    }

    @GetMapping("/clear")
    public String clearCache() {
        informationService.cleanCache();
        return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "clear ok"));
    }

    @GetMapping("/save")
    public String saveMemoryAPI() {
        memoryService.save();
        return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "save ok"));
    }

    @GetMapping("/getMGSList")
    public String getMGSList() {
        return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, banGoService.getMGSList()));
    }

    @PostMapping("/saveMGSList")
    public String addMGSBanGo(@RequestBody List<String> mgsList) {
        banGoService.saveList(mgsList);
        return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, "add ok"));
    }

    @GetMapping("/search")
    public String searchList(@RequestParam(defaultValue = "1", required = false) String page,
                             @RequestParam(defaultValue = "uploading", required = false) String sort,
                             @RequestParam String keyword) {
        List<VideoInfoItemVo> voList = informationService.searchInfo(page, sort, keyword);
        int curPage = Integer.parseInt(page);
        return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, new WebTableDataVo(voList, curPage)));
    }
}
