cmake_minimum_required(VERSION 3.10.0)
project(lampnet VERSION 0.1.0 LANGUAGES C)

add_executable(lampnet main.c)

add_executable(try_thread test/try_thread.c)