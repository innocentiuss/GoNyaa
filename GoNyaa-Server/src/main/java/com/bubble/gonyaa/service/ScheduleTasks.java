package com.bubble.gonyaa.service;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Service;

@Service
public class ScheduleTasks {

    @Autowired
    private MemoryService memoryService;
    @Scheduled(cron = "0 * * * * ?")
    public void autoSave() {
        memoryService.save();
    }
}
