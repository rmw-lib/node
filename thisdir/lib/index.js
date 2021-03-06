// Generated by CoffeeScript 2.6.1
//!/usr/bin/env coffee
var PREFIX_LEN;

import {
  dirname,
  sep
} from 'path';

if (sep === '\\') {
  PREFIX_LEN = 8;
} else {
  PREFIX_LEN = 7;
}

// windows is "file:///c:/dev/node/walk/lib/test.js"
export var thisfile = ({url}) => {
  return decodeURI(url.slice(PREFIX_LEN));
};

export var thisdir = (meta) => {
  return dirname(thisfile(meta));
};

export default thisdir;
