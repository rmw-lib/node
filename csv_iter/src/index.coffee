import {pipeline} from 'stream/promises'
import {createReadStream} from 'fs'
import {parse} from '@fast-csv/parse'

export stream = (stream, option)->
  csv = parse({
    headers: true
    ...option
  })
  p = pipeline(
    stream
    csv
  )
  yield from csv
  await p

export default (file, option)->
  stream(
    createReadStream(file)
    option
  )
