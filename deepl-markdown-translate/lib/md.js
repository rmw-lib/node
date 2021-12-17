// Generated by CoffeeScript 2.6.1
//!/usr/bin/env coffee
var Md, c_style_comment, comment, re_c_style_comment, translate_comment, turndownService;

import {
  readFileSync,
  writeFileSync
} from 'fs';

import axios from '@rmw/axios';

import cmark from 'cmark-gfm';

import TurndownService from 'turndown';

re_c_style_comment = /\/\*[\s\S]*?\*\/|\/\/.*$/gm;

c_style_comment = async(txt, translate) => {
  var i, li;
  li = [];
  txt = txt.replaceAll("://", ":-/");
  txt.replace(re_c_style_comment, (match) => {
    if (match.startsWith('//')) {
      li.push(match.slice(2));
    }
    if (match.startsWith('/*')) {
      li.push(match.slice(2, -2));
    }
    return '';
  });
  if (!li.length) {
    return txt;
  }
  li = (await (async function() {
    var results;
    results = [];
    for (i of li) {
      results.push((await translate(i)));
    }
    return results;
  })());
  return txt.replace(re_c_style_comment, (match, mlc, slc) => {
    if (match.startsWith('//')) {
      return "//" + li.shift();
    }
    if (match.startsWith('/*')) {
      return "/*" + li.shift() + "*/";
    }
    return match;
  }).replaceAll(":-/", "://");
};

comment = {
  rust: c_style_comment
};

translate_comment = async(markdown, translate) => {
  var _code, code, code_li, j, len, li, line, out, pos;
  li = markdown.split("\n");
  out = [];
  code = false;
  code_li = [];
  for (j = 0, len = li.length; j < len; j++) {
    line = li[j];
    if (code === false) {
      out.push(line);
      pos = line.indexOf("```");
      if (pos + 1) {
        _code = line.slice(pos + 3).trim();
        if (_code in comment) {
          code = _code;
        }
      }
    } else {
      if (line.indexOf("```") + 1) {
        out.push((await comment[code](code_li.join("\n"), translate)));
        out.push(line);
        code = false;
      } else {
        code_li.push(line);
      }
    }
  }
  return out.join("\n");
};

TurndownService.prototype.escape = (txt) => {
  return txt;
};

turndownService = new TurndownService({
  headingStyle: "atx",
  hr: '---',
  codeBlockStyle: "fenced"
});

turndownService.addRule('listItem', {
  filter: 'li',
  replacement: function(content, node, options) {
    var index, parent, prefix, start;
    content = content.replace(/^\n+/, '').replace(/\n+$/, '\n').replace(/\n/gm, '\n  ');
    // indent
    prefix = options.bulletListMarker + ' ';
    parent = node.parentNode;
    if (parent.nodeName === 'OL') {
      start = parent.getAttribute('start');
      index = Array.prototype.indexOf.call(parent.children, node);
      prefix = (start ? Number(start) + index : index + 1) + '. ';
    }
    return prefix + content + (node.nextSibling && !/\n$/.test(content) ? '\n' : '');
  }
});

export default Md = class Md {
  constructor(deepl1) {
    this.deepl = deepl1;
  }

  async translate(md, target_lang = "EN-US") {
    var deepl, end, html, pre, txt;
    ({deepl} = this);
    pre = "";
    if (md.startsWith("---\n")) {
      end = md.indexOf("\n---\n", 4);
      if (end > 0) {
        end += 5;
        pre = md.slice(0, +end + 1 || 9e9);
        md = md.slice(end + 1);
      }
    }
    html = (await cmark.renderHtml(md, {
      hardbreaks: true,
      extensions: {
        strikethrough: true
      }
    }));
    html = (await deepl.xml(html, target_lang));
    txt = turndownService.turndown(html);
    txt = (await translate_comment(txt, (t) => {
      if (deepl.source_lang === "ZH") {
        if (/^[\x00-\x7F]*$/.test(t)) {
          return t;
        }
      }
      return deepl.txt(t, target_lang);
    }));
    return pre + txt;
  }

};
