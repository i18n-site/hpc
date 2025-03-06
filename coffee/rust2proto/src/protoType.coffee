> ./rustTypeToProtobufType.js

export default (jsTypeId)=>
  (protoPush)=>
    (struct_name, name_type_li)=>
      if not name_type_li.length
        return
      type_li = []
      protoPush """message #{struct_name} {"""
      for [name, type] from name_type_li
        if not name
          name = 'v'
        type = rustTypeToProtobufType type
        type_li.push type
        protoPush '  ' + type + ' ' + name + ' = ' + type_li.length + ';'
      protoPush '}\n'

      return jsTypeId type_li
