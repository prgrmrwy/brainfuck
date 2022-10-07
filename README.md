# rust 学习项目

brainfuck 的 rust 实现

```bash
cg run helloworld.bf
```

借鉴项目: https://github.com/dontpanic92/rainfuck
> JIT TODO，有点难，以后再说(flag)

# Brainfuck 简介
https://juejin.cn/post/7026931753616965663


# 解释器基本定义
字符|含义|方法
---|---|---
\>|指针向右移一|forward()
<|指针向左移一|backward()
+|当前指针指向的数据带值+1|increase()
-|当前指针指向的数据带值-1|reduce()
.|将当前指针指向的数据带值的ASCII码打印|print()
,|获取键盘输入的字节流，写入当前指针指向的数据带|input()
[|循环开始，如果当前指针指向的数据带值为 0，则跳到与之匹配的 ]后一条指令|whileEntity()
]|循环结束,  如果当前指针指向的数据带值不为 0，则跳到与之匹配的 [后一条指令|whileEnd()
