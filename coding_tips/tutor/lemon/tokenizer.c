#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include "calc.h"  /* Lemon生成的头文件 */

/* 全局变量：用于词法分析 */
static char *input = NULL;
static size_t input_len = 0;
static size_t pos = 0;
static int current_line = 1;

/* 解析器状态结构体 */
typedef struct {
    int line;
    double result;
} ParseState;

/* 跳过空白字符（空格、制表符） */
static void skip_whitespace(void) {
    while (pos < input_len && (input[pos] == ' ' || input[pos] == '\t')) {
        pos++;
    }
}

/* 读取数字token（整数或浮点数） */
static int read_number(Token *token) {
    char buffer[64];
    size_t i = 0;
    int has_dot = 0;
    
    /* 处理负号（作为一元运算符的情况已在语法中处理） */
    if (pos < input_len && input[pos] == '-' && 
        pos + 1 < input_len && isdigit(input[pos + 1])) {
        buffer[i++] = input[pos++];
    }
    
    /* 读取数字部分 */
    while (pos < input_len && i < sizeof(buffer) - 1) {
        if (input[pos] == '.') {
            if (has_dot) break;  /* 已经有一个小数点 */
            has_dot = 1;
        } else if (!isdigit(input[pos])) {
            break;
        }
        buffer[i++] = input[pos++];
    }
    
    buffer[i] = '\0';
    
    if (has_dot) {
        token->value = atof(buffer);
        return FLOAT;
    } else {
        token->value = atoi(buffer);
        return INTEGER;
    }
}

/* 获取下一个token */
static int get_next_token(Token *token) {
    skip_whitespace();
    
    if (pos >= input_len) {
        return 0;  /* 文件结束 */
    }
    
    char ch = input[pos];
    
    switch (ch) {
        case '\n':
            pos++;
            current_line++;
            return NEWLINE;
            
        case '+':
            pos++;
            return PLUS;
            
        case '-':
            pos++;
            return MINUS;
            
        case '*':
            pos++;
            return TIMES;
            
        case '/':
            pos++;
            return DIVIDE;
            
        case '(':
            pos++;
            return LPAREN;
            
        case ')':
            pos++;
            return RPAREN;
            
        default:
            if (isdigit(ch) || (ch == '-' && pos + 1 < input_len && isdigit(input[pos + 1]))) {
                return read_number(token);
            } else {
                fprintf(stderr, "Error: Invalid character '%c' at line %d\n", ch, current_line);
                pos++;  /* 跳过错误字符继续 */
                return ERROR;
            }
    }
}

int main(int argc, char *argv[]) {
    /* 使用Lemon生成的解析器函数 */
    extern void *ParseAlloc(void *(*mallocProc)(size_t));
    extern void ParseFree(void *pParser, void (*freeProc)(void *));
    extern void Parse(void *pParser, int tokenId, Token token, ParseState *state);
    
    /* 启用调试跟踪（可选） */
    // extern void ParseTrace(FILE *stream, char *zPrefix);
    // ParseTrace(stderr, "PARSER>>> ");
    
    /* 读取输入（从stdin或文件） */
    FILE *fp = stdin;
    if (argc > 1) {
        fp = fopen(argv[1], "r");
        if (!fp) {
            perror("Error opening file");
            return 1;
        }
    }
    
    /* 读取所有输入到内存 */
    size_t capacity = 1024;
    input = malloc(capacity);
    if (!input) {
        perror("malloc");
        return 1;
    }
    
    size_t n;
    while ((n = fread(input + input_len, 1, capacity - input_len, fp)) > 0) {
        input_len += n;
        if (input_len >= capacity - 1) {
            capacity *= 2;
            char *new_input = realloc(input, capacity);
            if (!new_input) {
                perror("realloc");
                free(input);
                return 1;
            }
            input = new_input;
        }
    }
    input[input_len] = '\0';
    
    if (fp != stdin) fclose(fp);
    
    /* 初始化解析器 */
    ParseState state = { .line = 1, .result = 0.0 };
    void *parser = ParseAlloc(malloc);
    if (!parser) {
        fprintf(stderr, "ParseAlloc failed\n");
        return 1;
    }
    
    /* 词法分析与解析循环 */
    Token token;
    int tokenId;
    pos = 0;
    current_line = 1;
    
    int has_tokens = 0;  /* 跟踪是否有有效的token */
    while ((tokenId = get_next_token(&token)) != 0) {
        token.line = current_line;
        
        /* 跳过错误token */
        if (tokenId == ERROR) {
            continue;
        }
        
        has_tokens = 1;  /* 标记有有效token */
        
        /* 将token发送给解析器 */
        Parse(parser, tokenId, token, &state);
        
        /* 如果遇到换行符，重置解析器以处理下一个表达式 */
        if (tokenId == NEWLINE) {

            /* 发送结束标记给当前解析器 */
            Token end_token = {0};
            Parse(parser, 0, end_token, &state);

            
            /* 销毁当前解析器并创建新的解析器 */
            ParseFree(parser, free);
            parser = ParseAlloc(malloc);
            if (!parser) {
                fprintf(stderr, "ParseAlloc failed\n");
                return 1;
            }
            has_tokens = 0;  /* 重置token跟踪 */
        }
    }
    
    /* 处理最后一个表达式（如果文件末尾没有换行符但有有效token） */
    if (has_tokens) {
        Token end_token = {0};
        Parse(parser, 0, end_token, &state);
    }

    /* 显示统计信息 */
    printf("\n=== 解析统计 ===\n");
    printf("总行数: %d\n", state.line - 1);
    printf("最后结果: %.2f\n", state.result);
    
    /* 清理 */
    ParseFree(parser, free);
    free(input);
    
    return 0;
}