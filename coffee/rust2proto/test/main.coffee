#!/usr/bin/env coffee

> assert/strict > equal
  ../src/rustTypeToProtobufType

for [rust_type, proto_type] from [
  [
    'Vec<Box<u8>>'
    'repeated uint32'
  ]
  [
    'Vec<u8>'
    'bytes'
  ]
  [
    'Option<Vec<u8>>'
    'optional bytes'
  ]
  [
    'Option<Box<[u8]>>'
    'optional bytes'
  ]
  [
    '[u8;3]'
    'bytes'
  ]
  [
    'Option<[u8;3]>'
    'optional bytes'
  ]
  [
    'Option<[u16;3]>'
    'optional repeated uint32'
  ]
]
  gen_type = rustTypeToProtobufType(rust_type)
  console.log rust_type, '→',gen_type
  equal gen_type, proto_type
