@echo off
rem 设置当前控制台为UTF-8编码
chcp 65001 >> nul

javac -d build -h build app/src/main/java/com/angcyo/rust_android_demo/Rust.java