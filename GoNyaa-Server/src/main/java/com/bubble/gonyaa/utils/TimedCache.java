package com.bubble.gonyaa.utils;

import java.util.HashMap;
import java.util.Map;

public class TimedCache<V> {

    private final Map<String, CachedContent<V>> map = new HashMap<>();

    public synchronized V get(String key) {
        CachedContent<V> target;
        if ((target = map.get(key)) == null)
            return null;

        // valid
        if (System.currentTimeMillis() / 1000L - target.addedTimestamp < 600)
            return target.obj;

        // out of date
        map.remove(key);
        return null;
    }

    public synchronized void set(String key, V value) {
        map.put(key, new CachedContent<>(value, System.currentTimeMillis() / 1000L));
    }

    public synchronized void clear() {
        map.clear();
    }

    static private class CachedContent<T> {
        T obj;
        long addedTimestamp;

        public CachedContent(T obj, long addedTimestamp) {
            this.obj = obj;
            this.addedTimestamp = addedTimestamp;
        }
    }
}
