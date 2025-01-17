#!/usr/bin/env coffee

> @3-/snake
  ./struct2str.js
  ./rustTypeToProtobufType.js
  ./protoType.js

# 提取返回值
export default (
  json, type, jsPush, protoPush
  jsTypeId
  protoType
)=>
  {tuple, resolved_path} = type

  if tuple
    if tuple.length == 0
      return
    else
      console.log "TODO tuple",tuple
  else
    struct_name = resolved_path.name

    {struct, enum:_enum} = json[
      type.resolved_path.id
    ].inner

    + rt_id
    # 枚举
    if _enum
      protoPush "enum #{struct_name} {"
      t = []
      for i,pos in _enum.variants
        n = snake(json[i].name).toUpperCase()
        jsPush 'export const ' + n + ' = ' + pos
        protoPush '  ' + n + ' = ' + pos + ';'
      protoPush '}\n'
      rt_id = jsTypeId ['int32']
    # 结构体
    else
      type_li = []
      typeLiPush = type_li.push.bind type_li
      for i from struct.kind.plain.fields
        {
          name
          visibility
          inner:{struct_field}
        } = json[i]
        if visibility == 'public'
          type_li.push [name,struct2str(struct_field)]
      rt_id = protoType struct_name, type_li
  if rt_id
    return [
      rt_id
      struct_name
    ]
  return
