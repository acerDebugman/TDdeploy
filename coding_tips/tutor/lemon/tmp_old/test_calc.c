// tokenizer.c
#include "calc.h"  // 包含 token ID

extern void *ParseAlloc(void *(*mallocProc)(size_t));
extern void ParseFree(void *pParser, void (*freeProc)(void *));
extern void Parse(void *pParser, int tokenId, Token token, void *state);

typedef struct {
    int value;
    int line;
} Token;

int main() {
    void *pParser = ParseAlloc(malloc);
    Token token;
    int tokenId;
    
    // 循环读取 token
    while ((tokenId = getNextToken(&token)) != 0) {
        Parse(pParser, tokenId, token, NULL);
    }
    
    // 发送结束标记
    Parse(pParser, 0, token, NULL);
    ParseFree(pParser, free);
    return 0;
}
