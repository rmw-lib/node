#!/usr/bin/env coffee

import Pool from '@rmw/pool'
import sleep from 'await-sleep'

job = (n)=>
  =>
    console.log n
    await sleep 500*n
    console.log '\t',n

pool = Pool 5

await pool.done
n = 0
while ++n<10
  pool job(n)

