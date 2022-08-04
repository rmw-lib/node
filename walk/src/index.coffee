import {opendir, readlink, stat} from 'fs/promises'
import {dirname, join, normalize} from "path"

walk = (dir, ignore) ->
  for await d from await opendir(dir)
    if ignore?(d)
      continue

    entry = join(dir, d.name)
    if d.isDirectory()
      yield from walk(entry)
    else if (d.isFile())
      yield entry
    else if d.isSymbolicLink()
      p = await readlink(entry)
      if not p.startsWith '/'
        p = normalize join dir, p
      try
        s = await stat(p)
      catch err
        continue
      if s.isDirectory()
        len = p.length
        for await i from walk(p)
          yield join entry, i[len..]
      else if s.isFile()
        yield entry

if process.platform == 'win32'
  _walk = walk
  walk = (...args)->
    for await i from _walk.apply(this,arguments)
      yield i.replaceAll '\\','/'
    return

export default walk

export walkRel = (dir, ignore) ->
  len = dir.length + 1
  for await d from walk(dir, ignore)
    yield d[len..]
