#!/usr/bin/env coffee

import axios from '@rmw/axios'

export default class Translate
  constructor:(
    option={}
    @url="https://api-free.deepl.com/v2/translate"
  )->

    @option = {
      source_lang:"ZH"
      ...option
    }

  post:(option)->
    o = new URLSearchParams()
    for k,v of option
      o.append(k,v)

    axios.post @url, o, {
      'Content-Type': 'application/x-www-form-urlencoded'
    }

  txt:(text, target_lang="EN-US", option={})->
    option = {
      text
      target_lang
      ...@option
      ...option
    }

  xml:(text, target_lang="EN-US", option={})->
    @post {
      text
      target_lang
      tag_handling:"xml"
      ignore_tags:"code,pre,img,iframe,video,script,var"
      non_splitting_tags:"code,a,b,strong,em,del,i,span"
      ...@option
      ...option
    }
