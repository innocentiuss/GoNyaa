package com.bubble.gonyaa;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.autoconfigure.mongo.MongoAutoConfiguration;
import org.springframework.scheduling.annotation.EnableScheduling;

import java.util.Collections;

@SpringBootApplication(exclude = {MongoAutoConfiguration.class})
@EnableScheduling
public class GetUrlApplication {

    public static void main(String[] args) {
        SpringApplication application = new SpringApplication(GetUrlApplication.class);
        application.setDefaultProperties(Collections.singletonMap("spring.config.name", "config,application-default"));
        application.run(args);
    }

}
