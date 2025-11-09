

## 记录

开发分支：

```
feat/TD-37693-main
```

开发步骤：

1. 先将 分支：**feat/xnode-syntax**  的内容同步过来。
2. 基于此继续开发




其他：

1. 尝试添加新的表
2. 


## 逻辑

tmsg.h 里的 ENodeType 应该是 sql 的 statement: 

```

  QUERY_NODE_CREATE_XNODE_STMT,        // XNode
  QUERY_NODE_DROP_XNODE_STMT,          // XNode
  QUERY_NODE_UPDATE_XNODE_STMT,        // XNode for taosx
  QUERY_NODE_XNODE_TASK_OPTIONS,       // XNode task options
  QUERY_NODE_XNODE_TASK_SOURCE_OPT,    // XNode task source
  QUERY_NODE_XNODE_TASK_SINK_OPT,      // XNode task sink
  QUERY_NODE_CREATE_XNODE_AGENT_STMT,  // XNode agent
  QUERY_NODE_CREATE_XNODE_TASK_STMT,   // XNode task
  QUERY_NODE_DROP_XNODE_TASK_STMT,     // XNode task
  QUERY_NODE_DROP_XNODE_AGENT_STMT,    // XNode agent
  QUERY_NODE_ALTER_XNODE_TASK_STMT,    // XNode task
  QUERY_NODE_CREATE_XNODE_JOB_STMT,    // XNode task job
  QUERY_NODE_DROP_XNODE_JOB_STMT,      // XNode task job
```


tmsgdef.h  里定义 命令 吗？

```

// xnode msg overload
  TD_DEF_MSG_TYPE(TDMT_MND_CREATE_XNODE, "create-xnode", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_UPDATE_XNODE, "update-xnode", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_DROP_XNODE, "drop-xnode", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_CREATE_XNODE_TASK, "create-xnode-task", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_UPDATE_XNODE_TASK, "update-xnode-task", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_DROP_XNODE_TASK, "drop-xnode-task", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_CREATE_XNODE_AGENT, "create-xnode-agent", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_DROP_XNODE_AGENT, "drop-xnode-agent", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_CREATE_XNODE_JOB, "create-xnode-job", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_UPDATE_XNODE_JOB, "update-xnode-job", NULL, NULL)
  TD_DEF_MSG_TYPE(TDMT_MND_DROP_XNODE_JOB, "drop-xnode-job", NULL, NULL)

```


20251105:

做到 添加 xnode.h




## 测试
