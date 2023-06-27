package com.bubble.gonyaa.service;

import org.springframework.beans.factory.InitializingBean;

public interface PersistenceService extends InitializingBean {

    String loadMemory();
    String loadList();
    void saveMemory();
    void saveList();
}
