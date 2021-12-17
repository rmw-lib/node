// Generated by CoffeeScript 2.6.1
//!/usr/bin/env coffee
var c_style_comment, comment, re_c_style_comment, translate_comment;

import {
  readFileSync,
  writeFileSync
} from 'fs';

import axios from '@rmw/axios';

import cmark from 'cmark-gfm';

import TurndownService from 'turndown';

re_c_style_comment = /\/\*[\s\S]*?\*\/|([^\\:]|^)\/\/.*$/gm;

c_style_comment = async(txt, option) => {
  var i, li;
  li = [];
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
  li = (function() {
    var results;
    results = [];
    for (i of li) {
      results.push({
        'Text': i
      });
    }
    return results;
  })();
  option.textType = "plain";
  li = (await translate(li, option));
  return txt.replace(re_c_style_comment, (match, mlc, slc) => {
    if (match.startsWith('//')) {
      return "//" + li.shift();
    }
    if (match.startsWith('/*')) {
      return "/*" + li.shift() + "*/";
    }
    return match;
  });
};

comment = {
  rust: c_style_comment
};

translate_comment = async(markdown, option) => {
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
        out.push((await comment[code](code_li.join("\n"), option)));
        out.push(line);
        code = false;
      } else {
        code_li.push(line);
      }
    }
  }
  return out.join("\n");
};

export var translate = async(input, option) => {
  var data, headers, key, li, region, text, translations, url, x, y;
  ({key, region} = option);
  headers = {
    'Ocp-Apim-Subscription-Key': key,
    'Ocp-Apim-Subscription-Region': region,
    'Content-Type': 'application/json'
  };
  url = `https://api.cognitive.microsofttranslator.com/translate?api-version=3.0&to=${option.to}&from=${option.from}&textType=${option.textType || 'html'}`;
  ({data} = (await axios.post(url, JSON.stringify(input), {headers})));
  li = [];
  for (x of data) {
    ({translations} = x);
    for (y of translations) {
      ({text} = y);
      li.push(text);
    }
  }
  return li;
};

export default async(md, option) => {
  var end, html, pre, turndownService, txt;
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
  html = ((await translate([
    {
      'Text': html
    }
  ], option))).join('');
  txt = turndownService.turndown(html);
  txt = (await translate_comment(txt, option));
  return pre + txt;
};
