#!/usr/bin/env coffee

_rustTypeToProtobufType = (type, prefix) =>
  if type.startsWith('Box<')
    return _rustTypeToProtobufType type.slice(4,-1), prefix
  return (prefix or '') + (
    switch type
      when 'bool' then 'bool'
      when 'i8', 'i16' then 'sint32'
      when 'i32' then 'sint32'
      when 'i64' then 'sint64'
      when 'u8', 'u16' then 'uint32'
      when 'u32' then 'uint32'
      when 'u64' then 'uint64'
      when 'f32' then 'float'
      when 'f64' then 'double'
      when 'str','String' then 'string'
      when '[u8]', 'Vec<u8>' then 'bytes'
      else
        throw new Error "UnknownType: #{type}"
  )

export default rustTypeToProtobufType = (type) =>
  prefix = ''
  if type.startsWith('Option<')
    type = type.slice(7,-1)
    prefix += 'optional '

  if type.startsWith('Vec<')
    _type = type.slice(4,-1)
  else if type.startsWith '['
    if type.startsWith '[u8;' or type == '[u8]'
      type = '[u8]'
    else
      _type = type.slice(1,type.indexOf(';'))

  if _type and _type != 'u8'
    type = _type
    prefix += 'repeated '

  return _rustTypeToProtobufType type, prefix

