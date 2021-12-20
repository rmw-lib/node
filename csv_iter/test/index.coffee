#!/usr/bin/env coffee

import CsvIter from '@rmw/csv_iter'
import {join,dirname} from 'path'

{pathname} = new URL(import.meta.url)
csv = join(dirname(pathname),"test.csv")

for await line from CsvIter(csv)
  console.log line



