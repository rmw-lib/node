<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# @rmw/worker

promise worker

##  安装

```
yarn add @rmw/worker
```

或者

```
npm install @rmw/worker
```

## 使用

```
#!/usr/bin/env coffee

import worker from '@rmw/worker'
import sleep from 'await-sleep'

func = worker (name)=>
  fs = require 'fs'
  return name

console.log await func('hi')


```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
