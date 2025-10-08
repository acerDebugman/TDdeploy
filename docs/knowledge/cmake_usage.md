## CMake 工具概述
1. 这是一个跨平台的构建工具，用于管理 C/C++ 项目的构建过程。
2. 这是一个2阶段的构建工具，配置和生成两个阶段：
    1. 配置阶段：读取 CMakeLists.txt 文件，根据用户指令和环境配置参数，生成 Makefile 或者 其他构建系统的配置文件。这一步主要是解析 CMakeLists.txt 文件里的语法。常用命令比如：
    ```
    cmake -S . -B build
    cmake -B build -D CMAKE_BUILD_TYPE=Release
    cmake -B debug -D CMAKE_BUILD_TYPE=Debug
    ```
    2. 生成阶段：根据配置阶段生成的文件，调用相应的构建系统（如 Make、Visual Studio 等）进行编译和链接。比如如果是底层用 make 构建，就会调用 make 命令；如果是 Visual Studio，就会调用 MSBuild。一般情况下用 make 构建。如果使用 make 作为基础构建组件，可进入 build 目录下执行 make 命令,因为 make 需要的 Makefile 已经被 Cmake 生成了。


## CMake 常用命令

aux_source_directory:
```
aux_source_directory(<dir> <variable>)
eg:
aux_source_directory(src MONITOR_SRC)
```
递归扫描当前目录下的所有源文件，将它们的文件名列表存储在指定的变量中。
递归扫描当前 CMakeLists.txt 所在目录下的 src/ 子目录，把所有 .c/.cpp 文件路径塞进变量 MONITOR_SRC。

创建静态库：
```
add_library(<name> STATIC <src1> <src2> ...)

eg:
add_library(monitor STATIC ${MONITOR_SRC})
```
用刚才收集的源文件生成一个 静态库（.a 文件），库名叫 monitor。
后续其他目标只要 target_link_libraries(xxx monitor) 就能链接进来。


target_include_directories:
```
target_include_directories(<target>
  <INTERFACE|PUBLIC|PRIVATE> [items1...]
  [<INTERFACE|PUBLIC|PRIVATE> [items2...] ...])
```
为指定的目标添加头文件搜索路径。
<target> 是目标名称，<INTERFACE|PUBLIC|PRIVATE> 是指定路径的范围，items1... 是要添加的路径列表。

eg:
```
target_include_directories(
  monitor
  PUBLIC "${TD_SOURCE_DIR}/include/libs/monitor"
  PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/inc"
)
```


target_link_libraries:
```
target_link_libraries(<target> <item1> <item2> ...)
```
为指定的目标添加链接库。
<target> 是目标名称，<item1> <item2> ... 是要链接的库列表。

eg:
```
target_link_libraries(monitor os util common qcom transport monitorfw)
```
把 os util common ... monitorfw 这几个库（或目标）链接到 monitor。
如果那些目标也是当前项目里 add_library 生成的，CMake 会自动处理顺序和路径；如果是外部库，需要事先 find_package 或 add_subdirectory 好。

target_link_libraries 依赖使用的库，其实是在 add_subdirectory 里的 CMakeLists.txt 使用 add_library 生成的,如果是系统自带的库，比如 pthread, 就不需要 find_package 或 add_subdirectory 了, 直接在 target_link_libraries 里写 pthread 就可以了。
这里尤其注意 target_link_libraries 的依赖库的位置。
