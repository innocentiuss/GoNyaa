package com.bubble.gonyaa.controller;


import cn.hutool.http.HttpStatus;
import com.alibaba.fastjson.JSON;
import com.bubble.gonyaa.model.TableDataVo;
import com.bubble.gonyaa.model.VideoInfoVo;
import com.bubble.gonyaa.model.WebResponse;
import com.bubble.gonyaa.service.BanGoService;
import com.bubble.gonyaa.service.CacheService;
import com.bubble.gonyaa.service.InformationService;
import com.bubble.gonyaa.service.MemoryService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.*;
import org.thymeleaf.context.WebContext;
import org.thymeleaf.spring5.view.ThymeleafViewResolver;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.List;

@RestController
public class MainController {

    @Autowired
    private ThymeleafViewResolver thymeleafViewResolver;
    @Autowired
    private InformationService informationService;
    @Autowired
    private MemoryService memoryService;
    @Autowired
    private CacheService cacheService;
    @Autowired
    private BanGoService banGoService;

    @GetMapping("/list")
    @Deprecated
    public String getUrl(Model model, HttpServletRequest request, HttpServletResponse response,
                         @RequestParam(defaultValue = "1", required = false) String page,
                         @RequestParam(defaultValue = "uploading", required = false) String sort) {
        List<VideoInfoVo> voList = informationService.access(page, sort);
        int curPage = Integer.parseInt(page);
        model.addAttribute("curPage", curPage);
        model.addAttribute("sort", sort);
        model.addAttribute("nextPage", curPage + 1);
        model.addAttribute("prevPage", Math.max(curPage - 1, 1));
        model.addAttribute("goodsList", voList);
        WebContext webContext = new WebContext(request, response, request.getServletContext(), request.getLocale(), model.asMap());
        return thymeleafViewResolver.getTemplateEngine().process("ViewList", webContext);
    }

    @GetMapping("/save")
    @Deprecated
    public String saveMemory() throws IOException {
        memoryService.save();
        return "save ok";
    }

    @GetMapping("/refresh")
    @Deprecated
    public String refreshCache() {
        cacheService.refreshCache();
        return "refresh ok";
    }

    @GetMapping("/change")
    @Deprecated
    public String setBanGoViewed(@RequestParam String banGo) {
        if (memoryService.isViewed(banGo)) memoryService.unsetViewed(banGo);
        else memoryService.setViewed(banGo);
        return "change ok";
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
