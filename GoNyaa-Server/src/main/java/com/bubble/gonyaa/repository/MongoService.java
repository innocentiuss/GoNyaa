package com.bubble.gonyaa.repository;

import com.bubble.gonyaa.model.po.MongoEntity;
import org.apache.logging.log4j.util.Strings;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.data.mongodb.core.MongoTemplate;
import org.springframework.stereotype.Service;

import java.util.Objects;

@Service
@ConditionalOnProperty(name = "data.save.mode", havingValue = "mongodb")
public class MongoService implements PersistenceService {

    private final MongoTemplate mongoTemplate;

    public static final String DEFAULT_COLLECTION_NAME = "mongo_gonyaa";
    public static final String MONGO_MEMORY_ID = "mongo_memory";
    public static final String MONGO_BANGO_INFO_ID = "mongo_bango_info";

    public MongoService(MongoTemplate mongoTemplate) {
        this.mongoTemplate = mongoTemplate;
    }

    public void save(MongoEntity mongoEntity, String collectionName) {
        mongoTemplate.save(mongoEntity, collectionName);
    }

    public String get(String id, String collectionName) {
        MongoEntity res = mongoTemplate.findById(id, MongoEntity.class, collectionName);
        if (Objects.isNull(res)) return null;
        return res.getData();
    }


    @Override
    public String loadMemory() {
        String content = get(MONGO_MEMORY_ID, DEFAULT_COLLECTION_NAME);
        if (Strings.isBlank(content)) {
            content = "";
        }
        return content;
    }

    @Override
    public String loadList() {
        String content = get(MONGO_BANGO_INFO_ID, DEFAULT_COLLECTION_NAME);
        if (Strings.isBlank(content)) {
            content = "";
        }
        return content;
    }

    @Override
    public void saveMemory(String content) {
        MongoEntity mongoEntity = new MongoEntity();
        mongoEntity.setId(MONGO_MEMORY_ID);
        mongoEntity.setData(content);
        save(mongoEntity, DEFAULT_COLLECTION_NAME);
    }

    @Override
    public void saveList(String content) {
        MongoEntity mongoEntity = new MongoEntity();
        mongoEntity.setId(MONGO_BANGO_INFO_ID);
        mongoEntity.setData(content);
        save(mongoEntity, DEFAULT_COLLECTION_NAME);
    }
}
