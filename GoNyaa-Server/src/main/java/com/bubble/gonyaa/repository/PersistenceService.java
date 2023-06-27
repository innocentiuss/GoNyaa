package com.bubble.gonyaa.repository;

import org.springframework.beans.factory.InitializingBean;

import java.io.IOException;

public interface PersistenceService{

    String loadMemory();
    String loadList();
    void saveMemory(String content);
    void saveList(String content);
}
