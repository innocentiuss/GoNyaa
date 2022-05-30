package com.example.geturl.controller;



import com.example.geturl.model.VideoInfoVo;
import com.example.geturl.service.InformationService;
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
    public String getUrl(Model model, HttpServletRequest request, HttpServletResponse response,@RequestParam(defaultValue = "1", required = false) String page) {
        List<VideoInfoVo> voList = informationService.access(page);
        int curPage = Integer.parseInt(page);
        model.addAttribute("curPage", curPage);
        model.addAttribute("nextPage", curPage + 1);
        model.addAttribute("prevPage", Math.max(curPage - 1, 1));
        model.addAttribute("goodsList", voList);
        WebContext webContext = new WebContext(request, response, request.getServletContext(), request.getLocale(), model.asMap());
        return thymeleafViewResolver.getTemplateEngine().process("ViewList", webContext);
    }

}
