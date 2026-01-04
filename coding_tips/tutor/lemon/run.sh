#!/bin/bash

# 在项目根目录下载 lemon.c（仅 24KB）
#curl -O https://sqlite.org/src/raw/tool/lemon.c

# 编译成工具
#gcc -o lemon lemon.c

# 生成 gram.c, gram.h, gram.out 三个文件
#lemon calc.y

# 仅生成解析器代码（无头文件）
#lemon -m calc.y

make
./calc test_input.txt


