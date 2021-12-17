#!/usr/bin/env coffee

import axios from '@rmw/axios'

export default class Translate
  constructor:(
    option={}
    @url="https://api-free.deepl.com/v2/translate"
  )->
    @option = {
      ...option
    }

  post:(option)->
    if not option.text
      return ''
    o = new URLSearchParams()
    for k,v of option
      o.append(k,v)

    #console.log ">> translate", option.text
    {data} = await axios.post @url, o, {
      'Content-Type': 'application/x-www-form-urlencoded'
    }
    {translations} = data
    if translations
      return translations[0].text
    throw Error(JSON.stringify r)

  txt:(text, target_lang="EN-US", option={})->
    @post {
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
