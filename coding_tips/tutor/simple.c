#include <stdio.h>
#include <stdlib.h>

int main() {
    char *s = NULL;
    char s2 = NULL; 

    if (&s2) {
        printf("ref s2 is not null\n");
    }
    if (*&s == NULL) {
        printf("ref s is null\n");
    }
    printf("end\n");
}

