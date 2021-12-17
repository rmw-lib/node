#!/usr/bin/env coffee

import Translate from '@rmw/deepl-markdown-translate/deepl'
import Cache from '@rmw/deepl-markdown-translate/cache'
import Md from '@rmw/deepl-markdown-translate/md'
import {readFileSync} from 'fs'
import {homedir} from 'os'
import cmark from 'cmark-gfm'
import {join} from 'path'

translate = new Translate(
  (await import(join(homedir(),".config/deepl.js"))).default
)

md = new Md(new Cache(translate))

export default (txt, target_lang="EN-US")=>
  md.translate txt, target_lang
