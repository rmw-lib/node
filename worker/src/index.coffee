#!/usr/bin/env coffee

import {Worker} from 'worker_threads'
import {parentPort} from 'worker_threads'


export default worker = (func)=>
  func = """const {workerData,parentPort}=require('worker_threads');
(async ()=>{
try{
parentPort.postMessage([0,await (#{func.toString()}).apply(null,workerData)])
}catch(err){
parentPort.postMessage([1,err])
}
})()"""
  (args...)->
    w = new Worker(
      func
      eval: true
      workerData:args
    )
    new Promise (resolve, reject)=>
      w.addListener 'message', ([err,msg])=>
        if err
          next = reject
        else
          next = resolve
        next msg
        return

