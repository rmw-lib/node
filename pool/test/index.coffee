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

