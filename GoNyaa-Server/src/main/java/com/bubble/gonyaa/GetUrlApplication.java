package com.bubble.gonyaa;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.scheduling.annotation.EnableScheduling;

@SpringBootApplication
@EnableScheduling
public class GetUrlApplication {

    public static void main(String[] args) {
        SpringApplication.run(GetUrlApplication.class, args);
    }

}
