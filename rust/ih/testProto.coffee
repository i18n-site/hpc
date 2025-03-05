#!/usr/bin/env coffee

> ./proto/_.pb.js > CallLiEncode CallLiDecode

call_li = [
  [
    3
    5
  ]
  [
    new Uint8Array [3,2,1]
    new Uint8Array [7,9]
  ]
]

bin = CallLiEncode call_li
console.log call_li
console.log CallLiDecode bin
#
# msg = [
#   ERR_CODE_FORM
#   new Uint8Array(
#     [1, 2, 250, 1]
#     # []
#   )
# ]
# bin = ErrMsgEncode(
#   msg
# )
# console.log msg
# console.log bin
# console.log ErrMsgDecode(bin)
# console.log ErrMsgEncode(ErrMsgDecode(bin))
