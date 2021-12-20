<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# @rmw/csv

async read csv line by line

##  安装

```
yarn add @rmw/csv
```

或者

```
npm install @rmw/csv
```

## 使用

```
#!/usr/bin/env coffee

import CsvIter from '@rmw/csv'
import {join,dirname} from 'path'

{pathname} = new URL(import.meta.url)
csv = join(dirname(pathname),"test.csv")

for await line from CsvIter(csv)
  console.log line
```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)