#!/usr/bin/env coffee
import translate from '@rmw/deepl-markdown-translate'

txt = """
---

next: /日志/2020-11-29

---

[[toc]]

上面的是博客配置，会不翻译

可以翻译[带有连接的文本](https://rmw.link)

程序人生 :

> 路漫漫其修远兮，
> 吾将上下而求索。

internal/main/run_main_module

\```rust
// 可以翻译rust的代码注释
/*
可以翻译
多行注释
*/
let s  = "字符串中的文本不会翻译 https://阿里巴巴.com";
\```

对 [libmdbx](https://github.com/rmw-lib/mdbx) 的封装
"""



console.log await translate txt, "EN-US"
