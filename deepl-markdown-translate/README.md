<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# @rmw/deepl-markdown-translate

## 说明

市面上的 markdown 翻译工具都有问题，不好用。

比如用 [menthays/markdown-translator](https://github.com/menthays/markdown-translator) 翻译 markdown 文本，会把

`对 [libmdbx](https://github.com/erthink/libmdbx) 的 rust 封装`

翻译成

`Right [libmdbx](https://github.com/erthink/libmdbx) The rust package of`。

原因是它把文本拆分为 ```{ text: '对 ' },{ text: 'libmdbx' },{ text: ' 的 rust 封装' }```分开去翻译。

而用另外一些，比如[bilingual](https://github.com/zjp-CN/bilingual/issues/22) ，又不能保持链接的样式。

我写的 `@rmw/deepl-markdown-translate` 解决了一系列问题，并且支持

* 翻译 `rust` 代码的注释文本
* 不翻译 [vuepress](https://v2.vuepress.vuejs.org/zh/reference/default-theme/frontmatter.html#prev) 中的配置字段
* 逐行缓存，节约翻译成本

##  安装

`yarn add @rmw/deepl-markdown-translate` 或者 `npm install @rmw/deepl-markdown-translate`

## 使用

翻译需要 [deepl 的`api key`](https://www.deepl.com/pro-api)，请先去申请。( 开通需要美国信用卡，有需求可以邮件 `i@rmw.link` 帮忙代办 )。

`$HOME/.config/deepl.js` 配置如下

```js
module.exports = {
  auth_key: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx:xx",
  source_lang:"ZH"
};
```

用法演示如下 :

```
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
let x = 1; // 可以翻译rust的代码注释
/*
可以翻译
多行注释
*/
let s  = "字符串中的文本不会翻译 https://阿里巴巴.com";
\```

对 [libmdbx](https://github.com/rmw-lib/mdbx) 的封装
"""



console.log await translate txt, "EN-US"
```

运行后输出如下

![](https://raw.githubusercontent.com/gcxfd/img/gh-pages/CytFEw.png)

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)