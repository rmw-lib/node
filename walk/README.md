<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# @rmw/walk

walk directory recursive

##  安装

```
yarn add @rmw/walk
```

或者

```
npm install @rmw/walk
```

## 使用

```
#!/usr/bin/env coffee

import walk, {walkRel} from '@rmw/walk'
import {dirname} from 'path'

{pathname} = new URL(import.meta.url)

dir = dirname pathname

console.log dir

console.log '> full path'
for await i from walk(dir)
  console.log i

console.log '\n> relative path'
for await i from walkRel(dir)
  console.log i

```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
