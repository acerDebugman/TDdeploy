



一次请求链路里有两次 transaction 的现象是：

1. 第二次的 transaction 请求会覆盖第一次的 transaction 请求的结果，我记得是这样的; 实际肯定不会发生阻塞，因为start xnode task 里已经这么用了！









show transaction 信息：

![image-20260123135535797](mnode里的transaction.assets/image-20260123135535797.png)
