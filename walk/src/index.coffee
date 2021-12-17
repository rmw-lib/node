import {opendir, readlink, stat} from 'fs/promises'
import {dirname, join, normalize} from "path"



export default walk = (dir) ->
  for await d from await opendir(dir)
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

export walkRel = (dir) ->
  len = dir.length + 1
  for await d from walk(dir, follow_symbol=true)
    yield d[len..]
