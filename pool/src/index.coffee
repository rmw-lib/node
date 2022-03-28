import {cpus} from 'os'

export default (max=cpus().length*2)=>
  n = 0
  todo = []

  all = _done = undefined
  _init_all = =>
    all = new Promise (resolve)=>
      _done = =>
        _init_all()
        resolve()
        return

  _init_all()

  f = ->
    args = [...arguments]
    func = args[0]
    p = new Promise (resolve)=>
      todo.push [resolve,args]
    if n < max
      ++n
      setImmediate =>
        while todo.length
          [resolve,args] = todo.shift()
          try
            await args[0](...args[1..])
          catch err
            console.error(err)
          resolve()
        if 0 == --n
          _done()
      return
    p

  Object.defineProperty(
    f
    'done'
    writeable:false
    get:=>
      if n == 0
        return
      all
  )
  f
