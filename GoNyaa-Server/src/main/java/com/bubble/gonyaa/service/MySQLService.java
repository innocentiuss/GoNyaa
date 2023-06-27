package com.bubble.gonyaa.service;

import org.springframework.stereotype.Service;

@Service
// todo: 支持MySQL同步数据
public class MySQLService implements PersistenceService{
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
