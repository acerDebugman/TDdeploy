#include <stdio.h>
#include <uv.h>

void on_timer(uv_timer_t *handle) {
    static int count = 0;
    printf("Tick %d\n", ++count);
    if (count >= 5) {
        uv_close((uv_handle_t*)handle, NULL);  // 关闭 handle
    }
}

int main() {
    uv_loop_t *loop = uv_default_loop();
    uv_timer_t timer;
    
    uv_timer_init(loop, &timer);
    // 启动定时器：首次延迟 1000ms，之后每 1000ms 触发
    uv_timer_start(&timer, on_timer, 1000, 1000);
    
    printf("Starting event loop...\n");
    uv_run(loop, UV_RUN_DEFAULT);  // 阻塞运行直到所有 handle 关闭
    
    uv_loop_close(loop);
    return 0;
}
