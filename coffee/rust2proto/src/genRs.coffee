#!/usr/bin/env coffee
> ./crateFuncName.js
  ./UpCamel.js

returnType = (
  rsRunPush
  proto_name, rt_name
  rt_type_li
)=>
  if rt_type_li
    if rt_type_li.length == 1
      [rt_type] = rt_type_li
      if rt_type[1] == 'enum'
        pb_name = "pb::#{proto_name}::#{rt_name}"
        rsRunPush """
\n    match (r as i32).try_into() {
        Ok::<#{pb_name}, _>(r) => r.serialize_to_vec(),
        Err(err) => return Err(anyhow!(format!("#{pb_name} Invalid: {err}"))),
      }"""
        return

    rsRunPush "pb::#{proto_name}::#{rt_name} {"

    t = []

    for [name, type] from rt_type_li

      code = '\n      ' + (
        if name then "#{name}: r.#{name}" else "v: r"
      )

      if not ['i64','i32','u32','u64', 'Vec<u8>', 'String'].includes type
        code += '.into()'

      t.push code

    rsRunPush t.join(',')
    rsRunPush '\n    }.serialize_to_vec()'
  else
    rsRunPush """\n        vec![]"""
  return

export default (
  rsRunPush
  up_func_name
  args_li, args_id, rt_type_li
  rt_id, rt_name
  func_name, proto_name
  sub_mod
  from_req
)=>
  # console.log {
  #   up_func_name
  #   args_li
  #   args_id, rt_type_li, rt_id, rt_name, func_name, proto_name
  # }

  func = """\nFunc::#{crateFuncName(UpCamel(proto_name),up_func_name)} => {\n"""
  rsRunPush func

  mod_func = """#{proto_name}::#{sub_mod}#{func_name}"""

  if args_id
    args_parse = "pb::#{proto_name}::#{up_func_name}Args::deserialize_from_slice(args)"
    rsRunPush '  let args = '+args_parse+'?;\n'

  `args_li??=[]`
  args_len = Math.max(from_req.length, args_li.length)
  args = []
  n = -1
  while ++n < args_len
    name_req_get = from_req[n]
    if name_req_get
      [name, req_get] = name_req_get
      rsRunPush '  let '+name+' = '+(
        if req_get == 'req_::set_header::SetHeader' then 'req_::sync::get(req) 'else 'req_::get(req).await?'
      ) + ';\n'
      args.push '&'+name
    else
      [name, type] = args_li[n]
      code = """args.#{name}"""
      if [
        'str', 'String'
        '[u8]'
      ].includes type
        code = '&' + code
      else if [
        'u8'
        'u16'
        'i8'
        'i16'
      ].includes type
        code = code + ' as ' + type
      args.push code

  rsRunPush '  match '

  rsRunPush mod_func+'('

  rsRunPush "#{args.join(',')}"

  rsRunPush """
).await {
    Err(err)=>Err(CallErr{
      func: "#{mod_func}",
      args: dvec![#{args.join(',')}],
      err: err.to_string()
    })?,\n    Ok("""
  if rt_type_li
    rsRunPush  'r)=>'
    returnType rsRunPush, proto_name, rt_name, rt_type_li
  else
    rsRunPush  '_)=>vec![]'
  rsRunPush '\n  }\n}'

  return


