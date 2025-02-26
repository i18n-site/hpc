#!/usr/bin/env coffee

export default (
  func_name
  inputs
  jsPush
  protoType
  paths
  proto_name
  crate
  from_req
)=>
  args_pos = -1
  name_type = []

  for [name,args] from inputs
    ++args_pos

    {
      resolved_path
      primitive
      borrowed_ref
    } = args

    if borrowed_ref
      {
        primitive
        resolved_path
        slice
      } = borrowed_ref.type
      if slice and not primitive
        primitive = '[' + slice.primitive + ']'

    if resolved_path
      {path} = paths[resolved_path.id]
      if path[0] == crate
        path = [proto_name].concat path.slice(1)

      from_req[args_pos] = [
        name
        path.join('::')
      ]
      continue


    name_type.push [
      name
      primitive or slice
    ]

  if name_type.length
    proto_name = func_name+'Args'

    return [
      protoType(
        proto_name
        name_type
      )
      proto_name
      name_type
    ]
  return
