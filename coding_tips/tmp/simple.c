#include <stdio.h>
#include <stdlib.h>

int main() {
    char *s = {0};
    char s2 = NULL; 

    if (&s2) {
        printf("ref s2 is not null\n");
    }
    printf("ref s is %p\n", &s);
    param_check(&s);
    printf("ref s2 is %p\n", &s2);
    param_check(&s2);
        
    if (*&s == NULL) {
        printf("ref s is null\n");
    }
    printf("end\n");
}

#define OS_PARAM_CHECK(_o)             \
  do {                                 \
    if ((_o) == NULL) {                \
      terrno = TSDB_CODE_INVALID_PARA; \
      return terrno;                   \
    }                                  \
  } while (0)

void param_check(char *s) {
    if (s == NULL) {
        printf("param s is null\n");
    }
}