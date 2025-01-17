#!/usr/bin/env coffee

export default (struct)=>
  # console.log JSON.stringify(struct, null, 2)
  name = struct.resolved_path.name
  if struct.resolved_path.args?.angle_bracketed?.args?
    args = struct.resolved_path.args.angle_bracketed.args.map((arg) ->
      if arg.type?
        if arg.type.slice?
          if arg.type.slice.primitive?
            return "[" + arg.type.slice.primitive + "]"
          else
            return "[" + struct2str({resolved_path: {name: arg.type.slice}}) + "]"
        else if arg.type.resolved_path?
            return struct2str({resolved_path: arg.type.resolved_path})
        else if arg.type.generic?
            return arg.type.generic
        else
          return rust_type_to_readable(arg.type) # 如果还有其他情况，继续完善这里的逻辑
      else
        return "" # 理论上不应该出现这种情况，除非 JSON 数据有问题
    ).join(", ")
    return "#{name}<#{args}>"
  return name

