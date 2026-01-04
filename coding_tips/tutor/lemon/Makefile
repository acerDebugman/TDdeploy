# Lemon解析器生成器路径（如果已安装，直接用lemon）
#LEMON = lemon
# 或者使用本地编译的lemon可执行文件：
LEMON = ./lemon

TARGET = calc
OBJS = calc.o tokenizer.o

all: $(TARGET)

# 使用Lemon生成解析器代码
calc.c calc.h: calc.y
	$(LEMON) -m calc.y

# 编译生成的解析器
calc.o: calc.c calc.h
	gcc -c -o calc.o calc.c

# 编译主程序
tokenizer.o: tokenizer.c calc.h
	gcc -c -o tokenizer.o tokenizer.c

# 链接可执行文件
$(TARGET): $(OBJS)
	gcc -o $(TARGET) $(OBJS) -lm

clean:
	rm -f *.o calc.c calc.h $(TARGET) gram.out

# 测试运行
test: $(TARGET)
	@echo "Testing calculator..."
	./calc test_input.txt
