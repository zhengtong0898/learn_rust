### 如何在release模式下相对有效的展示错误堆栈
思路:  
1. 给每一个模块文件分配一组错误编号
2. 使用map_err重写每一个Result, 并向上传播Result
3. 在停止向上传播Result的地方将错误信息打印出来

```
use anyhow::{anyhow, Result};

fn third() -> Result<()> {
    return Err(anyhow!("\nE000000: 发现 <第三层函数> 错误"));
}

fn second() -> Result<()> {
    third().map_err(|e| anyhow!("\nE000001: 发现 <第二层函数> 错误: {e:?}"))
}

fn first() -> Result<()> {
    second().map_err(|e| anyhow!("\nE000002: 发现 <第一层函数> 错误: {e:?}"))
}

fn main() {
    first().map_err(|e| println!("{e:?}")).unwrap();
}
```

输出结果

```

E000002: 发现 <第一层函数> 错误:
E000001: 发现 <第二层函数> 错误:
E000000: 发现 <第三层函数> 错误
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ()', src\main.rs:16:44
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

搭配 [02_如何设置env_logger的日志格式.md](./02_如何设置env_logger的日志格式.md) 日志输出效果如下   

```
[2024-06-18 13:41:38.692553 INFO untitled\src\main.rs:L74] hello world!
[2024-06-18 13:41:38.693102 ERROR untitled\src\main.rs:L75]
E000002: 发现 <第一层函数> 错误:
E000001: 发现 <第二层函数> 错误:
E000000: 发现 <第三层函数> 错误
[2024-06-18 13:41:38.692553 INFO untitled\src\main.rs:L76] hello world!
```

这种方式可以获得一下收益:
1. 无需开启release模式下的debug参数.
2. 可以自定义输出 <关键数据> 错误信息.
3. 可以通过 <错误代码> 找到具体报错在哪一行.
4. 拥有完整的调用链路信息.
