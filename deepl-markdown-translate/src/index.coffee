#!/usr/bin/env coffee

import {readFileSync, writeFileSync} from 'fs'
import axios from '@rmw/axios'
import cmark from 'cmark-gfm'
import TurndownService from 'turndown'

re_c_style_comment = /\/\*[\s\S]*?\*\/|([^\\:]|^)\/\/.*$/gm

c_style_comment = (txt, option)=>
  li = []

  txt.replace(
    re_c_style_comment
    (match)=>
      if match.startsWith '//'
        li.push match[2..]
      if match.startsWith '/*'
        li.push match[2...-2]
      return ''
  )

  if not li.length
    return txt

  li = ({ 'Text':i } for i from li)

  option.textType = "plain"
  li = await translate(
    li
    option
  )

  txt.replace(
    re_c_style_comment
    (match, mlc, slc)=>
      if match.startsWith '//'
        return "//"+li.shift()
      if match.startsWith '/*'
        return "/*"+li.shift()+"*/"
      match
  )

comment = {
  rust:c_style_comment
}


translate_comment = (markdown, option)=>
  li = markdown.split("\n")

  out = []
  code = false
  code_li = []

  for line in li
    if code == false
      out.push line
      pos = line.indexOf("```")
      if pos+1
        _code = line[pos+3..].trim()
        if _code of comment
          code = _code
    else
      if line.indexOf("```") + 1
        out.push await comment[code] code_li.join("\n"),option
        out.push line
        code = false
      else
        code_li.push(line)
  return out.join("\n")



export translate = (input, option)=>
  {key,region} = option

  url = "https://api-free.deepl.com/v2/translate"
  {data} = (await axios.post(
    url
    JSON.stringify(input)
    {
      headers
    }
  ))

  li = []
  for {translations} from data
    for {text} from translations
      li.push text
  li

export default (md, option)=>
  pre = ""
  if md.startsWith("---\n")
    end = md.indexOf("\n---\n",4)
    if end > 0
      end += 5
      pre = md[..end]
      md = md[end+1..]

  html = await cmark.renderHtml md,{
    hardbreaks:true
    extensions:
      strikethrough: true
  }


  html = (await translate(
    [
      {
        'Text':html
      }
    ]
    option
  )).join ''


  turndownService = new TurndownService {
    headingStyle:"atx"
    hr: '---'
    codeBlockStyle: "fenced"
  }
  turndownService.addRule 'listItem',
    filter: 'li'
    replacement: (content, node, options) ->
      content = content.replace(/^\n+/, '').replace(/\n+$/, '\n').replace(/\n/gm, '\n  ')
      # indent
      prefix = options.bulletListMarker + ' '
      parent = node.parentNode
      if parent.nodeName == 'OL'
        start = parent.getAttribute('start')
        index = Array::indexOf.call(parent.children, node)
        prefix = (if start then Number(start) + index else index + 1) + '. '
      prefix + content + (if node.nextSibling and !/\n$/.test(content) then '\n' else '')

  txt = turndownService.turndown html

  txt = await translate_comment(txt, option)
  pre+txt



