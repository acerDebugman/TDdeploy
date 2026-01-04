#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>

// 响应数据回调（用于处理服务器返回）
struct Response {
    char *data;
    size_t size;
};

size_t write_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    struct Response *resp = (struct Response *)userp;
    
    char *ptr = realloc(resp->data, resp->size + realsize + 1);
    if(!ptr) return 0;
    
    resp->data = ptr;
    memcpy(&(resp->data[resp->size]), contents, realsize);
    resp->size += realsize;
    resp->data[resp->size] = 0;
    
    return realsize;
}

int main(void) {
    CURL *curl;
    CURLcode res;
    struct Response resp = {0};
    
    curl_global_init(CURL_GLOBAL_DEFAULT);
    curl = curl_easy_init();
    
    if(curl) {
        // 1. 设置目标 URL（Rust 服务器地址）
        curl_easy_setopt(curl, CURLOPT_URL, "http://127.0.0.1:3000/api/data");
        
        // 2. 设置请求头（JSON 格式）
        struct curl_slist *headers = NULL;
        headers = curl_slist_append(headers, "Content-Type: application/json");
        headers = curl_slist_append(headers, "User-Agent: C99-Client/1.0");
        curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
        
        // 3. 设置 POST 数据和内容
        const char *json_data = "{\"message\": \"Hello from C99\", \"number\": 42}";
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, json_data);
        curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)strlen(json_data));
        
        // 4. 设置响应回调
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&resp);
        
        // 5. 设置超时（可选）
        curl_easy_setopt(curl, CURLOPT_TIMEOUT, 5L);
        
        // 6. 执行请求
        res = curl_easy_perform(curl);
        
        // 7. 错误处理
        if(res != CURLE_OK) {
            fprintf(stderr, "curl_easy_perform() failed: %s\n",
                    curl_easy_strerror(res));
        } else {
            printf("Server response: %s\n", resp.data);
            long http_code;
            curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
            printf("HTTP Status: %ld\n", http_code);
        }
        
        // 8. 清理
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        free(resp.data);
    }
    
    curl_global_cleanup();
    return 0;
}
