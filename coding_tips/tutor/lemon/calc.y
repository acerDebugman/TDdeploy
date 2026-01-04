%include {
#include <stdio.h>
#include <stdlib.h>

/* 前向声明 ParseState 结构体 */
typedef struct ParseState ParseState;
}

/* 终结符的数据类型 */
%token_type {Token}
/* 定义终结符和非终结符 */
%token INTEGER FLOAT PLUS MINUS TIMES DIVIDE LPAREN RPAREN NEWLINE ERROR.

/* 关键：声明为 void* 类型，而非 double */
%type value {void*}


/* 额外的参数声明：ParseState 用于传递上下文信息 */
%extra_argument {ParseState *state}

/* 优先级 */
%left PLUS MINUS.
%left TIMES DIVIDE.
%right UMINUS.

/* 析构函数：自动释放内存 */
%destructor value { free($$); }

/* 语法规则 */
input ::= value(A) NEWLINE.  {
    /* 取出 double 值并释放内存 */
    double result = *(double*)A;
    printf("Result: %.2f\n", result);
    
    /* 使用 ParseState 存储结果和更新行号 */
    state->result = result;
    state->line++;  /* 每处理一行，行号增加 */
    
    free(A);
}

/* 处理文件末尾没有换行符的情况 */
input ::= value(A).  {
    /* 取出 double 值并释放内存 */
    double result = *(double*)A;
    printf("Result: %.2f\n", result);
    
    /* 使用 ParseState 存储结果和更新行号 */
    state->result = result;
    state->line++;  /* 每处理一行，行号增加 */
    
    free(A);
}

/* 整数：分配内存存储 */
value(A) ::= INTEGER(B).  {
    double *val = malloc(sizeof(double));
    *val = (double)B.value;
    A = (void*)val;
}

/* 浮点数：分配内存存储 */
value(A) ::= FLOAT(B).  {
    double *val = malloc(sizeof(double));
    *val = B.value;
    A = (void*)val;
}

/* 加法：计算并释放子节点内存 */
value(A) ::= value(B) PLUS value(C).  {
    double *val = malloc(sizeof(double));
    *val = *(double*)B + *(double*)C;
    A = (void*)val;
    free(B); free(C);
}

/* 减法 */
value(A) ::= value(B) MINUS value(C).  {
    double *val = malloc(sizeof(double));
    *val = *(double*)B - *(double*)C;
    A = (void*)val;
    free(B); free(C);
}

/* 乘法 */
value(A) ::= value(B) TIMES value(C).  {
    double *val = malloc(sizeof(double));
    *val = *(double*)B * *(double*)C;
    A = (void*)val;
    free(B); free(C);
}

/* 除法（含零检查） */
value(A) ::= value(B) DIVIDE value(C).  {
    if (*(double*)C != 0.0) {
        double *val = malloc(sizeof(double));
        *val = *(double*)B / *(double*)C;
        A = (void*)val;
    } else {
        fprintf(stderr, "Error: Division by zero\n");
        exit(1);
    }
    free(B); free(C);
}

/* 括号：直接传递指针 */
value(A) ::= LPAREN value(B) RPAREN.  {
    A = B;
}

/* 一元负号：分配新内存 */
value(A) ::= MINUS value(B). [UMINUS]  {
    double *val = malloc(sizeof(double));
    *val = -*(double*)B;
    A = (void*)val;
    free(B);
}