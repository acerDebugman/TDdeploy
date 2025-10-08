#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* xstrcat(char* str1, char* str2) {
    size_t len1 = strlen(str1);
    size_t len2 = strlen(str2);
    char* str = (char*)malloc(len1 + len2 + 1);
    if (str == NULL) {
        perror("Memory allocation failed");
        return NULL;
    }
    strcpy(str, str1);
    strcat(str, str2);
    return str;
}

char* xitoa(int num) {
    size_t len = snprintf(NULL, 0, "%d", num) + 1;
    char* str = (char*)malloc(len);
    if (str == NULL) {
        perror("Memory allocation failed");
        return NULL;
    }
    snprintf(str, len, "%d", num);
    return str;
}
