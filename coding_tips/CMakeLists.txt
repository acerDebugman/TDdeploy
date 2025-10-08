cmake_minimum_required(VERSION 3.10.0)
project(coding_tips VERSION 0.1.0 LANGUAGES C)

if(NOT CMAKE_BUILD_TYPE)
    #set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type" FORCE)
endif()

set(GC_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})

message(STATUS "CMAKE_BUILD_TYPE: ${CMAKE_BUILD_TYPE}")
message(STATUS "GC_SOURCE_DIR: ${GC_SOURCE_DIR}")

SET(EXECUTABLE_OUTPUT_PATH ${CMAKE_BINARY_DIR}/bin)

# 增加子目录
add_subdirectory(thread_in_turn)
add_subdirectory(util)
add_subdirectory(libs)

# 执行文件 可以在这里配置，也可以在子目录的 CMakeLists.txt 中配置
#add_executable(try_thread try_thread.c)
#add_executable(thread_in_turn thread_in_turn.c)

# add_executable(try_thread test/try_thread.c)
