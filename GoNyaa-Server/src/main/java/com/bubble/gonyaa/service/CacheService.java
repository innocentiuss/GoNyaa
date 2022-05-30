package com.bubble.gonyaa.service;

import cn.hutool.cache.impl.TimedCache;
import com.bubble.gonyaa.model.VideoInfoVo;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.util.List;

import java.util.concurrent.locks.ReentrantLock;

@Service
public class CacheService {
    @Value("${cache.timeout.ms}")
    private int timeout;
    @Value("${cache.auto.prune}")
    private boolean autoPrune;
    @Value("${cache.prune.interval}")
    private int pruneInterval;

    private final TimedCache<String, List<VideoInfoVo>> resultCache = new TimedCache<>(timeout);
    // lock for timed cache(dev by hashmap)
    private final ReentrantLock lock = new ReentrantLock();

    public CacheService() {
        if (autoPrune) {
            resultCache.schedulePrune(pruneInterval);
        }
    }

    public boolean contains(String key) {
        lock.lock();
        try {
            return resultCache.containsKey(key);
        } finally {
            lock.unlock();
        }
    }

    public void put(String key, List<VideoInfoVo> value) {
        lock.lock();
        try {
            resultCache.put(key, value);
        } finally {
            lock.unlock();
        }
    }

    public List<VideoInfoVo> get(String key) {
        lock.lock();
        try {
            return resultCache.get(key);
        } finally {
            lock.unlock();
        }
    }

}
