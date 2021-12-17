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
