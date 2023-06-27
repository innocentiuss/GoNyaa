package com.bubble.gonyaa.repository;

import com.bubble.gonyaa.utils.FileProcessor;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.stereotype.Service;

import java.io.IOException;

@Service
@ConditionalOnProperty(name = "data.save.mode", havingValue = "file")
public class FileMemService implements PersistenceService{

    @Value("${memory.txt.name}")
    private String MEMORY_FILE_NAME;
    @Value("${mgslist.txt.name}")
    private String MGS_TEXT_FILE_NAME;

    @Override
    public String loadMemory() {
        return FileProcessor.readTxt2String(MEMORY_FILE_NAME, "memory.txt");
    }

    @Override
    public String loadList() {
        return FileProcessor.readTxt2String(MGS_TEXT_FILE_NAME, "MGSList.txt");
    }

    @Override
    public void saveMemory(String content) {
        try {
            FileProcessor.writeString2File(content, MEMORY_FILE_NAME);
        } catch (IOException e) {
            e.printStackTrace();
            throw new RuntimeException("writing memory file error");
        }
    }

    @Override
    public void saveList(String content) {
        try {
            FileProcessor.writeString2File(content, MGS_TEXT_FILE_NAME);
        } catch (IOException e) {
            e.printStackTrace();
            throw new RuntimeException("writing mgs list file error");
        }
    }

}
