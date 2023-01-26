package com.bubble.gonyaa.utils;

import cn.hutool.core.io.IoUtil;

import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

public class FileProcessor {

    // 读取配置文件, 如果不存在则创建模板
    public static String readTxt2String(String fileName, String templateName) {
        Path filePath = Paths.get(fileName);
        byte[] fileBytes;
        try {
            // 文件不存在时会先创建一个模板
            if (!Files.exists(filePath)) {
                byte[] templateBytes = IoUtil.readBytes(FileProcessor.class.getClassLoader().getResourceAsStream(templateName));
                String templateContent = new String(templateBytes, StandardCharsets.UTF_8);
                Files.write(filePath, templateContent.getBytes(StandardCharsets.UTF_8));
            }
            fileBytes = Files.readAllBytes(filePath);
        } catch (IOException e) {
            e.printStackTrace();
            throw new RuntimeException(e.getMessage());
        }
        return new String(fileBytes, StandardCharsets.UTF_8);
    }

    public static void appendString2File(String toWrite, String fileName) throws IOException {
        FileOutputStream fileOutputStream = new FileOutputStream(fileName, true);
        OutputStreamWriter outputStreamWriter = new OutputStreamWriter(fileOutputStream, StandardCharsets.UTF_8);
        outputStreamWriter.write(toWrite);
        outputStreamWriter.close();
    }

    public static void writeString2File(String toWrite, String fileName) throws IOException {
        FileOutputStream fileOutputStream = new FileOutputStream(fileName);
        OutputStreamWriter outputStreamWriter = new OutputStreamWriter(fileOutputStream, StandardCharsets.UTF_8);
        outputStreamWriter.write(toWrite);
        outputStreamWriter.close();
    }
}
