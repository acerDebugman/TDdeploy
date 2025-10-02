#include <stdio.h>
 
/* 
 * 预定义宏演示程序
 * 展示ANSI C标准中常用的预定义宏及其用途
 */
int main() {
    // 打印当前源文件名（字符串常量）
    printf("当前文件: %s\n", __FILE__);
    
    // 打印编译日期（"MMM DD YYYY"格式）
    printf("编译日期: %s\n", __DATE__);
    
    // 打印编译时间（"HH:MM:SS"格式）
    printf("编译时间: %s\n", __TIME__);
    
    // 打印当前行号（十进制整数）
    printf("当前行号: %d\n", __LINE__);
    
    // 检查是否符合ANSI/ISO标准（1表示符合）
    printf("ANSI标准: %d\n", __STDC__);
    
    // 实用示例：调试信息输出
    printf("\n[调试信息] %s (第%d行) 编译于 %s %s\n", 
           __FILE__, __LINE__, __DATE__, __TIME__);
    
    return 0;
}
