#!/usr/bin/env coffee

> ./rustTypeToProtobufType.js

export default (
  func_name
  inputs
  jsPush
  protoType
)=>
  name_type = []
  for [name,args] from inputs
    {
      borrowed_ref
    } = args
    if borrowed_ref
      {
        primitive
      } = borrowed_ref.type
      name_type.push [
        name
        primitive
      ]
      continue
    throw new Error('genArgs TODO', JSON.stringify(args, null, 2))

  if name_type.length
    proto_name = func_name+'Args'
    return [
      protoType(
        proto_name
        name_type
      )
      proto_name
    ]
  return
