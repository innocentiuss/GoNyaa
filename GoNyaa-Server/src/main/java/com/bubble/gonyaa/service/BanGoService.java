package com.bubble.gonyaa.service;

import com.bubble.gonyaa.enums.Type;
import com.bubble.gonyaa.repository.PersistenceService;
import org.springframework.beans.factory.InitializingBean;
import org.springframework.stereotype.Service;

import java.util.*;

@Service
public class BanGoService implements InitializingBean {

    Set<String> mgsSet = new HashSet<>();

    private final PersistenceService persistenceService;

    public BanGoService(PersistenceService persistenceService) {
        this.persistenceService = persistenceService;
    }

    public Type getTypeFromId(String fanHao) {
        if (fanHao.startsWith("FC2-PPV")) return Type.FC2;
        for (String s : mgsSet) {
            if (fanHao.startsWith(s)) return Type.Prestige;
        }
        return Type.Normal;
    }


    public static String getPreviewLink(String fanHao, Type type) {
        if (type == Type.FC2) {
            String fc2Id = fanHao.substring(8);
            return "https://adult.contents.fc2.com/article/" + fc2Id + "/";
        } else if (type == Type.Normal) {
            return "https://www.dmm.co.jp/search/=/searchstr=" + fanHao + "/";
        }
        return "https://www.mgstage.com/search/cSearch.php?search_word=" + fanHao + "/";
    }

    public static String getJavBeeLink(String fanHao) {
        return "http://javbee.org/search?keyword=" + fanHao;
    }

    public static String getJavStoreLink(String fanhao) {
        return "http://javstore.net/search/" + fanhao + ".html";
    }

    public static String sortChange(String origin) {
        if (origin.equals("downloading")) return "leechers";
        if (origin.equals("uploading")) return "seeders";
        return "downloads";
    }

    @Override
    public void afterPropertiesSet() {
        mgsSet.clear();
        String mgsListString = persistenceService.loadList();
        String[] strings = mgsListString.split("\n");
        mgsSet.addAll(Arrays.asList(strings));
    }

    public List<String> getMGSList() {
        return new ArrayList<>(mgsSet);
    }

    public synchronized void saveList(List<String> mgsList) {
        StringBuilder toWrite = new StringBuilder();
        for (String s : mgsList) {
            toWrite.append(s).append("\n");
        }
        persistenceService.saveList(toWrite.toString());
        afterPropertiesSet();
    }
}
