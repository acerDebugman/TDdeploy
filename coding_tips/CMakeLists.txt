cmake_minimum_required(VERSION 3.10.0)
project(thread_in_turn VERSION 0.1.0 LANGUAGES C)

if(NOT CMAKE_BUILD_TYPE)
    #set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type" FORCE)
endif()

add_executable(try_thread thread_in_turn/try_thread.c)
add_executable(thread_in_turn thread_in_turn/thread_in_turn.c)

# add_executable(try_thread test/try_thread.c)
