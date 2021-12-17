#!/usr/bin/env coffee

import {join} from 'path'
import {homedir} from 'os'
import { open } from 'lmdb'
import blake3 from 'blake3'

export default class Cache
  constructor:(
    @translate
    db=join(homedir(),".cache/markdown-translate/deepl")
  )->
    @db = open {
      path : db
    }


  txt:(text, target_lang="EN-US")->
    {db} = @
    hash = blake3.hash text
    r = await db.get(hash)
    if not r
      r = @translate.txt(text,target_lang)
      await db.set hash, r
    r
