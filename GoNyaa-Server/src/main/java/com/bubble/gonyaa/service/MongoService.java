package com.bubble.gonyaa.service;

import com.bubble.gonyaa.model.po.MongoEntity;
import org.springframework.data.mongodb.core.MongoTemplate;
import org.springframework.stereotype.Service;
import org.springframework.util.Assert;

@Service
public class MongoService implements PersistenceService {

    private final MongoTemplate mongoTemplate;

    private static final String DEFAULT_COLLECTION_NAME = "mongo_gonyaa";
    private static final String MONGO_MEMORY_ID = "mongo_memory";
    private static final String MONGO_BANGO_INFO_ID = "mongo_bango_info";

    public MongoService(MongoTemplate mongoTemplate) {
        this.mongoTemplate = mongoTemplate;
    }

    public void save(MongoEntity mongoEntity, String collectionName) {
        mongoTemplate.save(mongoEntity, collectionName);
    }

    public String get(String id, String collectionName) {
        MongoEntity res = mongoTemplate.findById(id, MongoEntity.class, collectionName);
        Assert.notNull(res, "can not get object");
        return res.getData();
    }


    @Override
    public String loadMemory() {
        return null;
    }

    @Override
    public String loadList() {
        return null;
    }

    @Override
    public void saveMemory() {

    }

    @Override
    public void saveList() {

    }

    @Override
    public void afterPropertiesSet() throws Exception {

    }
}
