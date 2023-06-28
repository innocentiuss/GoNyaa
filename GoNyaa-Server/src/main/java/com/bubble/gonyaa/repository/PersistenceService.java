package com.bubble.gonyaa.repository;



public interface PersistenceService{

    String loadMemory();
    String loadList();
    void saveMemory(String content);
    void saveList(String content);
}
