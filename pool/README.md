<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# @rmw/pool

promise pool

##  安装

```
yarn add @rmw/pool
```

或者

```
npm install @rmw/pool
```

## 使用

```
#!/usr/bin/env coffee

import Pool from '@rmw/pool'
import sleep from 'await-sleep'

pool = Pool 5

job = (n)=>
  console.log n
  await sleep 500*n
  console.log 'done\t',n

n = 0
while ++n<10
 pool job,n

await pool.done
```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)