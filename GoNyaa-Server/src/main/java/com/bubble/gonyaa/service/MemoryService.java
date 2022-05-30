package com.bubble.gonyaa.service;

import cn.hutool.core.io.file.FileReader;
import cn.hutool.core.io.file.FileWriter;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

@Service
// 读写缓存, 定时持久化
@Slf4j
public class MemoryService {

    private final Set<String> viewedSet;

    public MemoryService() {
        viewedSet = new HashSet<>();
        String memory = new FileReader("memory.txt").readString();
        viewedSet.addAll(Arrays.asList(memory.split(";")));
    }

    public void save() {
        StringBuilder sb = new StringBuilder();
        for (String s : viewedSet) {
            sb.append(s).append(";");
        }
        FileWriter writer = new FileWriter("memory.txt");
        writer.write(sb.toString());
        log.info("Save memory cache ok");
    }


    public boolean isViewed(String banGo) {
        return viewedSet.contains(banGo);
    }

    public void setViewed(String banGo) {
        viewedSet.add(banGo);
    }

    public void unsetViewed(String banGo) {
        viewedSet.remove(banGo);
    }
}
