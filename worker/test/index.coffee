#!/usr/bin/env coffee

import worker from '@rmw/worker'
import sleep from 'await-sleep'

func = worker (name)=>
  fs = require 'fs'
  return name

console.log await func('hi')

