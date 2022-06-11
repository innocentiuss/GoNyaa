package com.bubble.gonyaa.service;

import cn.hutool.core.io.file.FileReader;
import cn.hutool.core.io.file.FileWriter;
import cn.hutool.setting.dialect.Props;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

@Service
// 读写缓存
@Slf4j
public class MemoryService {

    private final Set<String> viewedSet;

    private final String memoryPath = new Props("application.properties").getStr("memory.txt.path");

    private volatile boolean isModified;

    public MemoryService() {
        viewedSet = new HashSet<>();
        String memory = new FileReader(memoryPath).readString();
        viewedSet.addAll(Arrays.asList(memory.split(";")));
        isModified = false;
    }

    public synchronized void save() {
        if (!isModified) {
            log.info("no change, skip save");
            return;
        }
        StringBuilder sb = new StringBuilder();
        for (String s : viewedSet) {
            sb.append(s).append(";");
        }
        FileWriter writer = new FileWriter(memoryPath);
        writer.write(sb.toString());
        isModified = false;
        log.info("Save memory cache ok");
    }


    public synchronized boolean isViewed(String banGo) {
        return viewedSet.contains(banGo);
    }

    public synchronized void setViewed(String banGo) {
        viewedSet.add(banGo);
        isModified = true;
    }

    public synchronized void unsetViewed(String banGo) {
        viewedSet.remove(banGo);
        isModified = true;
    }
}