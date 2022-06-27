package com.bubble.gonyaa.utils;

import cn.hutool.core.io.file.FileReader;
import com.bubble.gonyaa.enums.Type;
import org.springframework.beans.factory.InitializingBean;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

@Service
public class CommonUtils implements InitializingBean {

    Set<String> mgsSet = new HashSet<>();

    @Value("${mgslist.txt.path}")
    private String mgsTxtPath;

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
            return "https://adult.contents.fc2.com/article/"+fc2Id+"/";
        }
        else if (type == Type.Normal) {
            return "https://www.dmm.co.jp/search/=/searchstr=" + fanHao + "/";
        }
        return "https://www.mgstage.com/search/cSearch.php?search_word=" + fanHao + "/";
    }

    public static String sortChange(String origin) {
        if (origin.equals("downloading")) return "leechers";
        if (origin.equals("uploading")) return "seeders";
        return "downloads";
    }

    @Override
    public void afterPropertiesSet() throws Exception {
        String mgsListString = new FileReader(mgsTxtPath).readString();
        String[] strings = mgsListString.split("\n");
        mgsSet.addAll(Arrays.asList(strings));
    }
}
