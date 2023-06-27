package com.bubble.gonyaa.service;

import com.bubble.gonyaa.model.po.MongoEntity;
import com.bubble.gonyaa.repository.MongoService;
import com.bubble.gonyaa.repository.PersistenceService;
import com.bubble.gonyaa.utils.FileProcessor;
import lombok.extern.slf4j.Slf4j;


import org.springframework.beans.factory.InitializingBean;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.mongodb.core.MongoTemplate;
import org.springframework.stereotype.Service;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

@Service
// 读写缓存
@Slf4j
public class MemoryService implements InitializingBean {
    @Value("${memory.txt.name}")
    private String MEMORY_FILE_NAME;
    @Value("${mgslist.txt.name}")
    private String MGS_TEXT_FILE_NAME;

    private Set<String> viewedSet;

    private final PersistenceService persistenceService;

    @Autowired
    private MongoTemplate mongoTemplate;

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

    public synchronized void syncFile2Mongo() {
        String content = FileProcessor.readTxt2String(MEMORY_FILE_NAME, "memory.txt");
        MongoEntity mongoEntity = new MongoEntity();
        mongoEntity.setId(MongoService.MONGO_MEMORY_ID);
        mongoEntity.setData(content);
        mongoTemplate.save(mongoEntity, MongoService.DEFAULT_COLLECTION_NAME);
    }

    public synchronized void syncList2Mongo() {
        String content = FileProcessor.readTxt2String(MGS_TEXT_FILE_NAME, "MGSList.txt");
        MongoEntity mongoEntity = new MongoEntity();
        mongoEntity.setId(MongoService.MONGO_BANGO_INFO_ID);
        mongoEntity.setData(content);
        mongoTemplate.save(mongoEntity, MongoService.DEFAULT_COLLECTION_NAME);
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
