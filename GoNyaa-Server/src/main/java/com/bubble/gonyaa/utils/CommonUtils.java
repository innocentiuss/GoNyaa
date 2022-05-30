package com.bubble.gonyaa.utils;

import cn.hutool.core.io.file.FileReader;
import com.bubble.gonyaa.enums.Type;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class CommonUtils {

    static Set<String> mgsSet = new HashSet<>();
    static {
        String mgsListString = new FileReader("MGSList.txt").readString();
        String[] strings = mgsListString.split("\n");
        mgsSet.addAll(Arrays.asList(strings));
    }
    public static Type getTypeFromId(String fanHao) {
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

}
