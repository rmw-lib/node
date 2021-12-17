import {cpus} from 'os'

export default (max=cpus().length*2)=>
  n = 0
  todo = []
  (func)=>
    if n < max
      ++n
      return func().finally(
        =>
          if todo.length
            setImmediate =>
              while todo.length
                [func,resolve,reject] = todo.shift()
                try
                  resolve await func()
                catch err
                  reject(err)
              --n
      )

    new Promise (resolve, reject)=>
      todo.push [func, resolve, reject]
      return


