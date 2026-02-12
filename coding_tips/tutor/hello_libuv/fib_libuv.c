#include <stdio.h>
#include <uv.h>

void fib_work(uv_work_t *req) {
    int n = *(int*)req->data;
    // 在独立线程中运行，不会阻塞事件循环
    long long a = 0, b = 1;
    for (int i = 0; i < n; i++) {
        long long c = a + b;
        a = b; b = c;
    }
    printf("Fib(%d) = %lld in thread\n", n, a);
}

void after_fib(uv_work_t *req, int status) {
    printf("Done in main thread\n");
    free(req->data);
    free(req);
}

int main() {
    uv_loop_t *loop = uv_default_loop();
    
    uv_work_t *req = malloc(sizeof(uv_work_t));
    int *n = malloc(sizeof(int));
    *n = 40;
    
    req->data = n;
    uv_queue_work(loop, req, fib_work, after_fib);
    
    uv_run(loop, UV_RUN_DEFAULT);
    return 0;
}
