#!/usr/bin/env coffee

> @3-/snake:@ > SNAKE
  ./struct2str.js
  ./UpCamel.js

# 提取返回值
export default =>
  has_str = 0
  (
    json, type, jsPush, protoPush
    jsTypeId
    protoType
    mod_name
  )=>
    {tuple, resolved_path, primitive} = type

    if tuple
      if tuple.length == 0
        return
      else
        console.log "TODO tuple",tuple
    else if primitive
      type_li = [
        [
          ''
          primitive
        ]
      ]
      struct_name = UpCamel primitive
      rt_id = protoType struct_name, type_li
    else
      struct_name = resolved_path.path
      type_meta = json[type.resolved_path.id]

      if type_meta
        {struct, enum:_enum} = type_meta.inner
        + rt_id
        # 枚举
        if _enum
          up_name = SNAKE struct_name
          protoPush "enum #{struct_name} {"
          t = []
          for i,pos in _enum.variants
            n = snake(json[i].name).toUpperCase()
            t.push 'export const ' + n + ' = ' + pos
            protoPush '  ' + n + ' = ' + pos + ';'

          jsPush [up_name, t]
          protoPush '}\n'
          rt_id = jsTypeId ['int32']
          type_li = [
            [ up_name, 'enum']
          ]
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
              type_li.push [
                name
                struct2str(struct_field)
              ]
          rt_id = protoType mod_name+struct_name, type_li
      else if struct_name == 'String'
        if not has_str
          has_str = 1
          protoPush """
  message String {
    string v = 1;
  }
  """

        rt_id = jsTypeId ['string']
        type_li = [
          [
            ''
            'string'
          ]
        ]
      else
        throw new Error "UNKOWN struct",struct_name

    if rt_id
      return [
        rt_id
        struct_name
        type_li
      ]
    return

