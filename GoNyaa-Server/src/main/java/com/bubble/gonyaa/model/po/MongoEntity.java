package com.bubble.gonyaa.model.po;

import lombok.Getter;
import lombok.Setter;
import org.springframework.data.mongodb.core.mapping.Document;

@Document
@Getter
@Setter
public class MongoEntity {
    private String id;
    private String data;
}
