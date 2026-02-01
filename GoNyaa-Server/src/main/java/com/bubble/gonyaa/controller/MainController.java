package com.bubble.gonyaa.controller;


import cn.hutool.http.HttpStatus;
import com.alibaba.fastjson.JSON;
import com.bubble.gonyaa.model.vo.WebTableDataVo;
import com.bubble.gonyaa.model.vo.VideoInfoItemVo;
import com.bubble.gonyaa.model.vo.WebResponse;
import com.bubble.gonyaa.service.*;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
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

    /**
     * 导出已阅列表为文件下载
     */
    @GetMapping("/exportViewed")
    public ResponseEntity<byte[]> exportViewed() {
        List<String> viewedList = memoryService.getViewedList();
        // 使用分号分隔，与现有存储格式保持一致
        String content = String.join(";", viewedList);
        if (!content.isEmpty()) {
            content += ";"; // 末尾加上分号，保持格式一致
        }

        byte[] bytes = content.getBytes(StandardCharsets.UTF_8);

        HttpHeaders headers = new HttpHeaders();
        headers.setContentType(MediaType.TEXT_PLAIN);
        headers.setContentDispositionFormData("attachment", "viewed_export_" + System.currentTimeMillis() + ".txt");
        headers.setContentLength(bytes.length);

        return ResponseEntity.ok()
                .headers(headers)
                .body(bytes);
    }

    /**
     * 导入已阅列表文件
     * @param file 上传的文件
     * @param mode 导入模式：append(追加) 或 overwrite(覆盖)
     */
    @PostMapping("/importViewed")
    public String importViewed(@RequestParam("file") MultipartFile file,
                               @RequestParam(defaultValue = "append") String mode) {
        try {
            if (file.isEmpty()) {
                return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_BAD_REQUEST, "文件为空"));
            }

            String content = new String(file.getBytes(), StandardCharsets.UTF_8);
            // 按分号或换行符分割
            List<String> banGoList = Arrays.asList(content.split("[;\\n\\r]+"));

            boolean append = !"overwrite".equalsIgnoreCase(mode);
            int importedCount = memoryService.importViewedList(banGoList, append);

            // 立即保存到文件
            memoryService.save();

            String msg = String.format("导入成功，新增 %d 条记录（模式：%s）", importedCount, append ? "追加" : "覆盖");
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_OK, msg));
        } catch (IOException e) {
            log.error("导入已阅列表失败", e);
            return JSON.toJSONString(new WebResponse<>(HttpStatus.HTTP_INTERNAL_ERROR, "导入失败：" + e.getMessage()));
        }
    }
}
