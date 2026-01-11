

lemmon 语法和定义：



```
%name Parse
%token_prefix TK_
%token_type  {char*}
%default_type {char*}
%include {
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
}

/* ⚠️ 核心：所有 char* 符号在栈弹出时自动 free */
%destructor { free($$); }

%token NUM.
%token PLUS.

/* 最终表达式：手动 free 结果（A 的所有权已转移给我们） */
program ::= expr(A).   { printf("result = %s\n", A); free(A); }

/* 数字规则：strdup(B) 复制一份，B 会被 Lemon 自动 free */
expr(A) ::= NUM(B).    { A = strdup(B); }

/* 加法规则：**B 和 C 在这里之后会被自动 free** */
expr(A) ::= expr(B) PLUS expr(C). {
    size_t len = strlen(B) + strlen(C) + 4; /* +4 给 '+'、括号、\0 */
    A = malloc(len);
    if (A) snprintf(A, len, "(%s+%s)", B, C);
    /* B 和 C 不需要手动 free，Lemon 在动作结束后自动清理 */
}
```

这里： expr(B) PLUS expr(C) 更像是是函数签名，代码块里的代码：{...}，就是 expr(B) 中的 B 弹出来，C 弹出来后，传入这个函数执行，执行代码是 {....} 里；

执行完得到的 A 入栈，然后销毁 B 和 C， 销毁函数是： %desctructor 里的 free($$);



lexer (strdup) → token (NUM/PLUS) → Parse() → 栈
                                       ↓
    归约: expr ::= NUM
        - 动作: A = strdup(B)      // B 来自 token，B 之后被 free
                - 清理: token 弹出 → free(B)
                                       ↓
        归约: expr ::= expr PLUS expr
                - 动作: A = malloc(...)    // 使用 B 和 C 创建新串
                        - 清理: 右侧 expr 弹出 → free(B), free(C)
                                       ↓
            归约: program ::= expr
                        - 动作: printf(...) → free(A)  // 最终所有权交给我们





代码：

```
#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include "Parse.h"

/* 词法扫描：每次返回一个 token，value 必须动态分配 */
static int scan(const char *p, char **value)
{
    static const char *cursor;
    if (!p) cursor = p;
    if (*cursor == 0) return 0;  /* EOF */

    while (isspace(*cursor)) ++cursor;

    /* 数字：strdup 分配内存 */
    if (isdigit((unsigned char)*cursor)) {
        char buf[32];
        int i = 0;
        while (isdigit((unsigned char)*cursor) && i < 31)
            buf[i++] = *cursor++;
        buf[i] = '\0';
        *value = strdup(buf);  /* 必须动态分配！ */
        return TK_NUM;
    }

    /* 加号：同样 strdup，统一由 Lemon 管理释放 */
    if (*cursor == '+') {
        ++cursor;
        *value = strdup("+");
        return TK_PLUS;
    }

    return -1;  /* 非法字符 */
}

int main(void)
{
    const char input[] = "3+4";
    void *parser = ParseAlloc(malloc);

    int tok;
    char *val;
    const char *p = input;

    /* 逐个 token 喂给解析器，交出内存管理权 */
    while ((tok = scan(p, &val)) != 0) {
        p = NULL;
        Parse(parser, tok, val);  /* val 的内存现在由 Lemon 负责 */
    }
    Parse(parser, 0, NULL);   /* EOF */

    ParseFree(parser, free);
    return 0;
}
```



```
$ lemon calc_str.y              # 生成 Parse.c / Parse.h
$ gcc -g lemon_str_ex.c Parse.c -o calc_str
$ valgrind --leak-check=full ./calc_str
```



前面程序的这几句代码做什么：

    Parse(parser, 0, NULL);   /* EOF */
    ParseFree(parser, free);

### 1. `Parse(parser, 0, NULL);   /* EOF */`

**作用**：向解析器发送 **结束符（EOF）信号**，告诉它“所有 token 已经喂完，可以收尾了”。

**为什么需要**：

- Lemon 是 **LALR(1)** 解析器，必须用 EOF 才能触发 **起始规则**（`program ::= expr`）的归约。
- 如果不调用这句，`program` 规则永远不会执行，你的最终结果（`printf("result = ...")`）也不会打印。

**举例说明**：

假设输入 `3+4`，调用的时序是：

表格

复制

| 调用                           | 解析栈状态     | 是否归约 `program`            |
| :----------------------------- | :------------- | :---------------------------- |
| `Parse(p, TK_NUM, "3")`        | `NUM`          | ❌ 否                          |
| `Parse(p, TK_PLUS, "+")`       | `NUM PLUS`     | ❌ 否                          |
| `Parse(p, TK_NUM, "4")`        | `NUM PLUS NUM` | ❌ 否                          |
| `Parse(p, 0, NULL)` **← 关键** | `expr`         | ✅ **触发** `program ::= expr` |

**底层逻辑**：

- `major = 0` 是 Lemon 内部定义的 **EOF 标记**（你不可能在语法里定义 token 0）
- 解析器收到 EOF 后，会执行所有可能的归约，直到栈里只剩起始符号

------

### 2. `ParseFree(parser, free);`

**作用**：**释放解析器实例占用的所有内存**，防止内存泄漏。

**为什么需要**：

- `ParseAlloc(malloc)` 时，Lemon 分配了内部状态机、栈、缓冲区等结构
- 这些内存不会自动释放，必须手动调用 `ParseFree`
- 第二个参数 `free` 告诉 Lemon 用哪个函数释放内存（必须与 `ParseAlloc` 的 `malloc` 配对）





## 总结：

1. LALR(1) 应该是在 LL(1) 基础上，把元素按后序遍历入栈后，再进行弹栈和操作符进行规约操作！

或者是在 LL(1) 后得到的 AST 进行 后序遍历，然后进行规约。

2. 
