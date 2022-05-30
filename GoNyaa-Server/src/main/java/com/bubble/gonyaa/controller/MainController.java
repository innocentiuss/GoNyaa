package com.bubble.gonyaa.controller;



import com.bubble.gonyaa.model.VideoInfoVo;
import com.bubble.gonyaa.service.CacheService;
import com.bubble.gonyaa.service.InformationService;
import com.bubble.gonyaa.service.MemoryService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.thymeleaf.context.WebContext;
import org.thymeleaf.spring5.view.ThymeleafViewResolver;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
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

    @GetMapping("/list")
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
    public String saveMemory() {
        memoryService.save();
        return "save ok";
    }

    @GetMapping("/refresh")
    public String refreshCache() {
        cacheService.refreshCache();
        return "refresh ok";
    }

    @GetMapping("/change")
    public String setBanGoViewed(@RequestParam String banGo) {
        if (memoryService.isViewed(banGo)) memoryService.unsetViewed(banGo);
        else memoryService.setViewed(banGo);
        return "change ok";
    }

}
