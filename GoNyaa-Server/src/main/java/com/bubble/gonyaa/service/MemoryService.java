package com.bubble.gonyaa.service;

import com.bubble.gonyaa.repository.PersistenceService;
import lombok.extern.slf4j.Slf4j;


import org.springframework.beans.factory.InitializingBean;

import org.springframework.stereotype.Service;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

@Service
// 读写缓存
@Slf4j
public class MemoryService implements InitializingBean {


    private Set<String> viewedSet;

    private final PersistenceService persistenceService;

    public MemoryService(PersistenceService persistenceService) {
        this.persistenceService = persistenceService;
    }

    private volatile boolean isModified;


    public synchronized void save() {
        if (!isModified) {
            log.info("no change, skip save");
            return;
        }

        StringBuilder sb = new StringBuilder();
        for (String s : viewedSet) {
            sb.append(s).append(";");
        }
        persistenceService.saveMemory(sb.toString());
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
    // 加载数据到内存
    public void afterPropertiesSet() {
        viewedSet = new HashSet<>();
        String memory = persistenceService.loadMemory();
        viewedSet.addAll(Arrays.asList(memory.split(";")));
        isModified = false;
    }
}
