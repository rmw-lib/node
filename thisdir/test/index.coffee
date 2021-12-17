#!/usr/bin/env coffee

import {thisfile,thisdir} from '@rmw/thisdir'

do =>
  console.log thisfile(import.meta)
  console.log thisdir(import.meta)

