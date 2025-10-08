#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

#define THREAD_NUM 3

int             turn = 0;               // 当前该谁打印
pthread_mutex_t mtx  = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t  cond = PTHREAD_COND_INITIALIZER;

void* thread_func(void* arg) {
    int id = *(int*)arg;                // 线程序号 0,1,2
    int print_val = id + 1;             // 要打印的值 1,2,3

    while (1) {
        pthread_mutex_lock(&mtx);
        while (turn != id)               // 不是自己轮次就等
            pthread_cond_wait(&cond, &mtx);

        printf("%d\n", print_val);      // 打印
        turn = (turn + 1) % THREAD_NUM; // 把令牌给下一位
        pthread_cond_broadcast(&cond);  // 唤醒其余线程
        pthread_mutex_unlock(&mtx);
    }
    return NULL;
}

int main() {
    pthread_t tid[THREAD_NUM];
    int       id[THREAD_NUM];

    for (int i = 0; i < THREAD_NUM; ++i) {
        id[i] = i;
        pthread_create(&tid[i], NULL, thread_func, &id[i]);
    }
    for (int i = 0; i < THREAD_NUM; ++i)
        pthread_join(tid[i], NULL);

    return 0;
}

