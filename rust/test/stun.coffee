#!/usr/bin/env coffee

import rust from '..'


buf = await rust.stun_external_addr("stun.ooma.com:3478",3333)
console.log buf
console.log new Uint8Array(buf[..3]), new DataView(buf).getUint16(4,true)
