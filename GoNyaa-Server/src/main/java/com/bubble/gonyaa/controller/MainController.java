package com.bubble.gonyaa.controller;



import com.bubble.gonyaa.model.VideoInfoVo;
import com.bubble.gonyaa.service.InformationService;
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

}
