#!/usr/bin/env coffee

import {join} from 'path'
import {homedir} from 'os'
import { open } from 'lmdb'
import blake3 from 'blake3'
import {encode, decode} from '@rmw/utf8'

split_p = (xml)=>
  p = 0
  li = []
  loop
    np = xml.indexOf('</p>',p)
    if np < 0
      if p<xml.length
        li.push xml[p..]
      break
    np+=4
    li.push xml[p...np]
    p = np
  li


class RmwUtf8Db
  constructor:(@db)->

  get:(key)->
    r = @db.get(key)
    if r
      return decode(r)

  set:(key,val)->
    e = encode(val)
    #console.log "compress ratio", (100*e.length/Buffer.from(val,'utf8').length) + "%"
    @db.put(key,e)

export default class Cache
  constructor:(
    @translate
    db=join(homedir(),".cache/markdown-translate/deepl")
  )->
    @db = open {
      path : db
      maxDbs: 256
    }
    @lang_db = new Map()

  _db : (lang)=>
    db = @lang_db.get(lang)
    if not db
      db = new RmwUtf8Db @db.openDB(
        lang
        keyEncoding:'binary'
        encoding:'binary'
      )
      @lang_db.set(lang,db)
    db


  xml:(text, target_lang="EN-US")->
    db = @_db target_lang

    cache = new Map()
    not_exist = []

    li = split_p(text)
    for i from li
      hash = blake3.hash i
      r = db.get(hash)
      if r
        cache.set(i,r)
      else
        not_exist.push(i)

    if not_exist.length
      for i,pos in split_p await @translate.xml(not_exist.join(''), target_lang)
        t = not_exist[pos]
        cache.set(t, i)
        hash = blake3.hash t
        await db.set hash, i

    xml = []
    for i from li
      xml.push cache.get(i)

    return xml.join('')



  txt:(text, target_lang="EN-US")->
    db = @_db target_lang
    hash = blake3.hash text
    r = db.get(hash)
    if not r
      r = await @translate.txt(text,target_lang)
      await db.set hash, r
    r
