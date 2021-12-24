#!/usr/bin/env coffee

import {dirname, sep} from 'path'

if sep == '\\'
  PREFIX_LEN = 8
else
  PREFIX_LEN = 7

# windows is "file:///c:/dev/node/walk/lib/test.js"

export thisfile = ({url})=>
  decodeURI url.slice(PREFIX_LEN)

export thisdir = (meta)=>
  dirname thisfile meta

export default thisdir
