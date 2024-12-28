package com.yohannis.bolio;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
class BolioController {

    @GetMapping("/hello")
    public String helloWorld() {
        return "Hello World";
    }

    @GetMapping("/convert")
    public String convert() {
        return "Hello World";
    }
}