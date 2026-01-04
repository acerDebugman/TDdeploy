/* Token type definitions for the calculator parser */

#define INTEGER                          1
#define FLOAT                            2
#define PLUS                             3
#define MINUS                            4
#define TIMES                            5
#define DIVIDE                           6
#define LPAREN                           7
#define RPAREN                           8
#define NEWLINE                          9
#define ERROR                           10
#define UMINUS                          11

/* Token数据结构：存储token的值和行号 */
typedef struct {
    double value;
    int line;
} Token;

/* ParseState结构体：存储解析状态信息 */
struct ParseState {
    int line;      /* 当前行号 */
    double result; /* 最后一次计算的结果 */
};