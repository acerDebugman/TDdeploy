



```
root@ha ~/zgc/dev/taosx (test/6859008002-main)$ cat ralph-loop.sh 
#!/bin/bash
# ralph-loop.sh - 手动实现 Ralph Loop 逻辑

TASK_FILE="$1"
MAX_ITERATIONS="${2:-20}"
COMPLETION_PROMISE="${3:-DONE}"
ITERATION=0

if [ ! -f "$TASK_FILE" ]; then
    echo "Usage: $0 <task-file.md> [max-iterations] [completion-promise]"
    exit 1
fi

echo "🚀 启动 Ralph Loop (Bash 版)"
echo "任务文件: $TASK_FILE"
echo "最大迭代: $MAX_ITERATIONS"
echo "完成标记: $COMPLETION_PROMISE"
echo ""

while [ $ITERATION -lt $MAX_ITERATIONS ]; do
    ITERATION=$((ITERATION + 1))
    echo "=========================================="
    echo "🔄 第 $ITERATION 轮迭代"
    echo "=========================================="
    
    # 读取任务描述（每次重新加载，模拟 fresh context）
    TASK_CONTENT=$(cat "$TASK_FILE")
    
    # 调用 Claude（使用 -p 参数非交互式执行）
    # 注意：这里使用 claude -p 在 Claude Code 中可能受限，需要根据实际情况调整
    RESPONSE=$(claude -p "$TASK_CONTENT
    
当前进度：第 $ITERATION 轮。
请执行下一步任务，如果全部完成请输出 <$COMPLETION_PROMISE>。")
    
    echo "$RESPONSE"
    
    # 检测完成标记
    if echo "$RESPONSE" | grep -q "<$COMPLETION_PROMISE>"; then
        echo ""
        echo "✅ 检测到完成标记，循环结束"
        echo "总迭代次数: $ITERATION"
        exit 0
    fi
    
    # 可选：保存每轮输出到日志
    echo "$RESPONSE" > ".ralph-round-$ITERATION.log"
    
    //echo ""
    //read -p "按 Enter 继续下一轮，或 Ctrl+C 停止..."
    echo "⏳ 等待 3 秒继续..."
    sleep 3
done

echo "⚠ 达到最大迭代次数 $MAX_ITERATIONS"
exit 1
```

执行脚本：

```
./ralph-loop.sh .claude/ralph-tasks/task-mysql-rust.md
```

.claude/ralph-tasks/task-mysql-rust.md 内容：

```
# Ralph Task: MySQL 数据源 Rust 集成测试

## 目标
将 `tests/e2e/test_function/mysql_test.py` 的逻辑迁移到 Rust，实现 `tests/integration/datasources/mysql.rs`。

## 参考学习材料
1. **风格参考**：`tests/integration/datasources/mqtt.rs` - 重点学习：
   - 如何初始化 TaosConn（直接使用原生连接，端口 6030）
   - 测试模块的组织结构（`mod tests` 或独立文件）
   - 断言风格（assert_eq! 或自定义宏）
   - 异步测试的处理方式（#[tokio::test] 或同步）

2. **业务逻辑来源**：`tests/e2e/test_function/mysql_test.py` - 提取：
   - 测试数据准备逻辑: 数据来自 192.168.1.45:3306 的 mysql 服务，不允许修改数据源数据
   - 查询验证逻辑（SELECT 结果比对）
   - 清理逻辑（DROP TABLE）

## 技术约束
- **连接方式**：使用 `TaosConn` 直接连接 `localhost:6030`，不使用 taos-adapter
- **Explorer 端口**：6060（用于验证数据源配置，如有）
- **文件位置**：`tests/integration/datasources/mysql.rs`
- **编译要求**：必须通过 `cargo check` 和 `cargo test --test integration`

## 完成标准（Completion Criteria）
1. [ ] 文件 `tests/integration/datasources/mysql.rs` 存在且可编译
2. [ ] 包含至少 3 个核心测试用例（对应 Python 文件的主要场景）
3. [ ] `cargo test mysql --test integration` 全部通过
4. [ ] 代码风格与 `tests/integration/datasources/mqtt.rs` 一致
5. [ ] 输出标记 `&lt;promise&gt;DONE&lt;/promise&gt;` 表示完成

## 当前状态
- [x] 任务已启动
- [ ] 已分析参考文件
- [ ] 已实现基础结构
- [ ] 已实现核心测试逻辑
- [ ] 已通过编译检查
- [ ] 已通过运行测试
```

