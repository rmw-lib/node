import {cpus} from 'os'

export default (max=cpus().length*2)=>
  n = 0
  todo = []
  f = ->
    args = [...arguments]
    func = args[0]
    if n < max
      ++n
      return func(...args[1..]).finally(
        =>
          if todo.length
            setImmediate =>
              while todo.length
                args = todo.shift()
                try
                  await args[0](...args[1..])
                catch err
                  console.error(err)
              --n
      )
    todo.push args
  Object.defineProperty(
    f
    'pool'
    writeable:false
    get:=>
      loop
        await Promise.all todo
        if not todo.length
          return
  )
  f
