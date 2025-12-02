cmake_minimum_required(VERSION 3.15)
project(libuv_client C)

set(CMAKE_C_STANDARD 11)

# 查找 libuv
find_path(UV_INCLUDE_DIR uv.h
    PATHS /usr/include /usr/local/include /opt/libuv/include
)
find_library(UV_LIB uv
    PATHS /usr/lib /usr/local/lib /opt/libuv/lib
)

if(NOT UV_INCLUDE_DIR OR NOT UV_LIB)
    message(FATAL_ERROR "libuv not found! Please install libuv-dev or set UV_INCLUDE_DIR/UV_LIB")
endif()

add_executable(client client.c)
target_include_directories(client PRIVATE ${UV_INCLUDE_DIR})
target_link_libraries(client PRIVATE ${UV_LIB})

if(WIN32)
    target_link_libraries(client PRIVATE ws2_32)
endif()

# 编译命令示例：
# mkdir build && cd build
# cmake .. && cmake --build .
