#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include "xstr.h"

#define THREAD_NUM 10
void* thread_function(void* t_arg);
int test1(char* args);
char* xstrcat(char* str1, char* str2);
char* xitoa(int num);

struct ThreadArgs {
    char* name;
    int idx;
};

int main(int argc, char* argv[]) {
    test1("t_zgc");
}

int test1(char* args) {
    struct ThreadArgs* t_args = (struct ThreadArgs*)malloc(sizeof(struct ThreadArgs) * THREAD_NUM);
    pthread_t thread_id[THREAD_NUM];
    for (int i=0; i < THREAD_NUM; i++) {
        t_args[i].name = xstrcat(xstrcat(args, "_"), xitoa(i));
        t_args[i].idx = i;
        int ret = pthread_create(&thread_id[i], NULL, thread_function, &t_args[i]);
        if (ret != 0) {
            printf("pthread_create failed, ret = %d\n", ret);
            exit(EXIT_FAILURE);
        }
    }
    for(int i = 0; i < THREAD_NUM; i++)
        pthread_join(thread_id[i], NULL);
    return 0;
}

void* thread_function(void* t_arg) {
    struct ThreadArgs* arg = (struct ThreadArgs*)t_arg;
    printf("Thread is running, arg = %s, thread num: %d\n", arg->name, arg->idx);
    return 0;
}
