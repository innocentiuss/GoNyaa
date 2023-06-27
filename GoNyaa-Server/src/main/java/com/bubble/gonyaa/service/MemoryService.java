package com.bubble.gonyaa.service;

import com.bubble.gonyaa.utils.FileProcessor;
import lombok.extern.slf4j.Slf4j;


import org.springframework.beans.factory.InitializingBean;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.mongodb.core.MongoTemplate;
import org.springframework.stereotype.Service;

import java.io.IOException;
import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

@Service
// 读写缓存
@Slf4j
public class MemoryService implements InitializingBean {

    private Set<String> viewedSet;

    @Autowired
    MongoTemplate mongoTemplate;

    @Value("${memory.txt.name}")
    private String memoryFileName;

    private volatile boolean isModified;

    public MemoryService() {
    }

    public synchronized void save() throws IOException {
        if (!isModified) {
            log.info("no change, skip save");
            return;
        }

        StringBuilder sb = new StringBuilder();
        for (String s : viewedSet) {
            sb.append(s).append(";");
        }
        FileProcessor.writeString2File(sb.toString(), memoryFileName);
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

    @Override
    public void afterPropertiesSet() throws Exception {
        viewedSet = new HashSet<>();
        String memory = FileProcessor.readTxt2String(memoryFileName, "memory.txt");
        viewedSet.addAll(Arrays.asList(memory.split(";")));
        isModified = false;
    }
}
