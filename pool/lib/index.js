// Generated by CoffeeScript 2.6.1
import {
  cpus
} from 'os';

export default (max = cpus().length * 2) => {
  var _done, _init_all, all, f, n, todo;
  n = 0;
  todo = [];
  all = _done = void 0;
  _init_all = () => {
    return all = new Promise((resolve) => {
      return _done = () => {
        _init_all();
        resolve();
      };
    });
  };
  _init_all();
  f = function() {
    var args, func, p;
    args = [...arguments];
    func = args[0];
    p = new Promise((resolve) => {
      return todo.push([resolve, args]);
    });
    if (n < max) {
      ++n;
      setImmediate(async() => {
        var err, resolve;
        while (todo.length) {
          [resolve, args] = todo.shift();
          try {
            await args[0](...args.slice(1));
          } catch (error) {
            err = error;
            console.error(err);
          }
          resolve();
        }
        if (0 === --n) {
          return _done();
        }
      });
      return;
    }
    return p;
  };
  Object.defineProperty(f, 'done', {
    writeable: false,
    get: () => {
      if (n === 0) {
        return;
      }
      return all;
    }
  });
  return f;
};
