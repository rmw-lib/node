#!/usr/bin/env coffee

import rust from ".."

do =>
  port = 23213
  rust.upnp_daemon(
    "rust upnp test"
    port
    1
    (ext_port, ip, gateway)=>
      console.log(
        "resolve"
        new Uint8Array(gateway[..3])
        new DataView(gateway).getUint16(4,true)
        ext_port
        new Uint8Array(ip)
        port
      )
    (err)=>
      console.log "reject",err
  )
  await new Promise(
    (resolve)=>
      console.log 'sleep'
      setTimeout(resolve,300000)
  )
