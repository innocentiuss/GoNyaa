package com.bubble.gonyaa.service;

import com.bubble.gonyaa.repository.PersistenceService;
import lombok.extern.slf4j.Slf4j;


import org.springframework.beans.factory.InitializingBean;

import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
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
//            log.info("no change, skip save");
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

    /**
     * 获取已阅列表（用于导出）
     */
    public synchronized List<String> getViewedList() {
        return new ArrayList<>(viewedSet);
    }

    /**
     * 导入已阅列表
     * @param banGoList 要导入的番号列表
     * @param append true为追加模式，false为覆盖模式
     * @return 导入成功的数量
     */
    public synchronized int importViewedList(List<String> banGoList, boolean append) {
        if (!append) {
            // 覆盖模式：清空现有数据
            viewedSet.clear();
        }
        int beforeSize = viewedSet.size();
        for (String banGo : banGoList) {
            if (banGo != null && !banGo.trim().isEmpty()) {
                viewedSet.add(banGo.trim());
            }
        }
        int addedCount = viewedSet.size() - beforeSize;
        if (addedCount > 0 || !append) {
            isModified = true;
        }
        log.info("Imported {} items into viewedSet (mode: {}), total: {}", addedCount, append ? "append" : "overwrite", viewedSet.size());
        return addedCount;
    }
}
