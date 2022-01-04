<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# @rmw/thisdir

promise thisdir

##  安装

```
yarn add @rmw/thisdir
```

或者

```
npm install @rmw/thisdir
```

## 使用

```
#!/usr/bin/env coffee

import {thisfile,thisdir} from '@rmw/thisdir'

do =>
  console.log thisfile(import.meta)
  console.log thisdir(import.meta)
```

## 关于

本项目隶属于**人民网络([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)