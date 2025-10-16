
## 处理逻辑
1. row_block 处理逻辑没问题，因为 flat_write_with_raw_block 就是按 子表写入的，并且出错时也只 archive 出错的 record batch, 而不是整个超级表。错误粒度只到 子表。
2. flat_write_with_sql 才会出现 archive 超级表错误，但是按所有涉及 子表 写入的问题，这样子表多，写入频次就多
3. 

