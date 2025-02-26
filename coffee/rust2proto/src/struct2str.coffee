#!/usr/bin/env coffee

export default (struct)=>
  {primitive} = struct
  if primitive
    return primitive
  # console.log JSON.stringify(struct, null, 2)
  name = struct.resolved_path.path
  args = struct.resolved_path.args?.angle_bracketed?.args
  if args
    args = args.map((arg) =>
      {type} = arg
      if type
        if type.slice?
          if type.slice.primitive?
            return "[" + type.slice.primitive + "]"
          else
            return "[" + struct2str({resolved_path: {name: type.slice}}) + "]"
        else if type.resolved_path?
            return struct2str({resolved_path: type.resolved_path})
        else if type.generic?
            return type.generic
        else
          {primitive} = type
          if primitive
            return primitive
        # else
        #   return rust_type_to_readable(arg.type) # 如果还有其他情况，继续完善这里的逻辑
      else
        return "" # 理论上不应该出现这种情况，除非 JSON 数据有问题
    ).join(", ")
    return "#{name}<#{args}>"
  return name

