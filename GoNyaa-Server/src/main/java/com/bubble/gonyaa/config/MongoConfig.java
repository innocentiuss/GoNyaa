package com.bubble.gonyaa.config;

import com.mongodb.ConnectionString;
import com.mongodb.MongoClientSettings;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.context.annotation.Configuration;
import org.springframework.data.mongodb.config.AbstractMongoClientConfiguration;

@Configuration
@ConditionalOnProperty(name = "data.save.mode", havingValue = "mongodb")
public class MongoConfig extends AbstractMongoClientConfiguration {

    @Value("${mongodb.url}")
    private String MONGO_URL;
    @Value("${mongodb.db}")
    private String MONGO_DB;
    @Override
    protected String getDatabaseName() {
        return MONGO_DB;
    }

    @Override
    protected void configureClientSettings(MongoClientSettings.Builder builder) {
        // 配置MongoClientSettings，例如设置连接URL、凭据等
        builder.applyConnectionString(new ConnectionString(MONGO_URL));
    }
}
