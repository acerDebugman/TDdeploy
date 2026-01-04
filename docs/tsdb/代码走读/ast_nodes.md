
一个实际的 where 语句的 LogicCondition 的表达式例子：

```
taos> rebalance xnode job where age > 10 or age > 1 or name=1 or zgc=1 or abc=10;
xxxzgc *** where nodetype: 4, ast: {
        "NodeType":     "4",
        "Name": "LogicCondition",
        "LogicCondition":       {
                "DataType":     {
                        "Type": "0",
                        "Precision":    "0",
                        "Scale":        "0",
                        "Bytes":        "0"
                },
                "AliasName":    "2515744548289141832",
                "UserAlias":    "age > 10 or age > 1 or name=1 or zgc=1 or abc=10",
                "RelatedTo":    "0",
                "BindExprID":   "0",
                "CondType":     "2",
                "Parameters":   [{
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "12521370265370106441",
                                        "UserAlias":    "age > 10",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "40",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "age",
                                                        "UserAlias":    "age",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "age",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "4555470977590941194",
                                                        "UserAlias":    "10",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "2",
                                                        "Literal":      "10",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }, {
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "1609527377948162648",
                                        "UserAlias":    "age > 1",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "40",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "age",
                                                        "UserAlias":    "age",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "age",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "5001870860487857737",
                                                        "UserAlias":    "1",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "1",
                                                        "Literal":      "1",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }, {
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "1503636689330325722",
                                        "UserAlias":    "name=1",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "44",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "name",
                                                        "UserAlias":    "name",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "name",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "5001870860487857737",
                                                        "UserAlias":    "1",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "1",
                                                        "Literal":      "1",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }, {
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "12438137017733867032",
                                        "UserAlias":    "zgc=1",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "44",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "zgc",
                                                        "UserAlias":    "zgc",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "zgc",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "5001870860487857737",
                                                        "UserAlias":    "1",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "1",
                                                        "Literal":      "1",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }, {
                                "NodeType":     "3",
                                "Name": "Operator",
                                "Operator":     {
                                        "DataType":     {
                                                "Type": "0",
                                                "Precision":    "0",
                                                "Scale":        "0",
                                                "Bytes":        "0"
                                        },
                                        "AliasName":    "9372953878580161909",
                                        "UserAlias":    "abc=10",
                                        "RelatedTo":    "0",
                                        "BindExprID":   "0",
                                        "OpType":       "44",
                                        "Left": {
                                                "NodeType":     "1",
                                                "Name": "Column",
                                                "Column":       {
                                                        "DataType":     {
                                                                "Type": "0",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "0"
                                                        },
                                                        "AliasName":    "abc",
                                                        "UserAlias":    "abc",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "TableId":      "0",
                                                        "TableType":    "0",
                                                        "ColId":        "0",
                                                        "ProjId":       "0",
                                                        "ColType":      "0",
                                                        "DbName":       "",
                                                        "TableName":    "",
                                                        "TableAlias":   "",
                                                        "ColName":      "abc",
                                                        "DataBlockId":  "0",
                                                        "SlotId":       "0",
                                                        "TableHasPk":   false,
                                                        "IsPk": false,
                                                        "NumOfPKs":     "0",
                                                        "HasDep":       false,
                                                        "HasRef":       false,
                                                        "RefDb":        "",
                                                        "RefTable":     "",
                                                        "RefCol":       "",
                                                        "IsPrimTs":     false
                                                }
                                        },
                                        "Right":        {
                                                "NodeType":     "2",
                                                "Name": "Value",
                                                "Value":        {
                                                        "DataType":     {
                                                                "Type": "14",
                                                                "Precision":    "0",
                                                                "Scale":        "0",
                                                                "Bytes":        "8"
                                                        },
                                                        "AliasName":    "4555470977590941194",
                                                        "UserAlias":    "10",
                                                        "RelatedTo":    "0",
                                                        "BindExprID":   "0",
                                                        "LiteralSize":  "2",
                                                        "Literal":      "10",
                                                        "Flag": false,
                                                        "Translate":    false,
                                                        "NotReserved":  false,
                                                        "IsNull":       false,
                                                        "Unit": "0"
                                                }
                                        }
                                }
                        }]
        }
}

```
