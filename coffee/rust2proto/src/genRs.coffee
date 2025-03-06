#!/usr/bin/env coffee

> ./crateFuncName.js
  ./UpCamel.js

returnType = (
  rsRunPush
  proto_name
  rt_name
  rt_type_li
)=>
  mod_func_name = "#{proto_name}::#{rt_name}"
  if rt_type_li
    if rt_type_li.length == 1
      [rt_type] = rt_type_li
      if rt_type[1] == 'enum'
        pb_name = "pb::#{mod_func_name}"
        rsRunPush """
\n  match (r as i32).try_into() {
    Ok::<#{pb_name}, _>(r) => (State::OK, r.serialize_to_vec()),
    Err(err) => call_err("#{mod_func_name}", anyhow!("enum invaild {err}"), captcha, ||s_::EMPTY).await?
  }"""
        return

    rsRunPush "(State::OK, pb::#{mod_func_name} {"

    t = []

    for [name, type] from rt_type_li

      code = '\n    ' + (
        if name then "#{name}: r.#{name}" else "v: r"
      )

      if not ['i64','i32','u32','u64', 'Vec<u8>', 'String'].includes type
        code += '.into()'

      t.push code

    rsRunPush t.join(',')
    rsRunPush '\n  }.serialize_to_vec()'
  else
    rsRunPush """\n  vec![]"""
  rsRunPush ')'
  return

export default (
  rsRunPush
  up_func_name
  args_li, args_id, rt_type_li
  rt_id, rt_name
  func_name, proto_name
  sub_mod
  from_req
  has_captcha
)=>
  # console.log {
  #   up_func_name
  #   args_li
  #   args_id, rt_type_li, rt_id, rt_name, func_name, proto_name
  # }

  func = """\nFunc::#{crateFuncName(UpCamel(proto_name),up_func_name)} => {\n"""
  rsRunPush func

  mod_func = """#{proto_name}::#{sub_mod}#{func_name}"""

  if has_captcha
    rsRunPush "  ctx_::captcha(ctx,captcha).await?;\n"

  if args_id
    rsRunPush """  let args: pb::#{proto_name}::#{up_func_name}Args = args_decode(args,"#{proto_name}::#{up_func_name}")?;\n"""

  `args_li??=[]`
  args_len = Math.max(from_req.length, args_li.length)
  args = []
  n = -1
  while ++n < args_len
    name_req_get = from_req[n]
    if name_req_get
      [name, req_get] = name_req_get
      rsRunPush '  let '+name+' = '+(
        if req_get == 'cookie_b::Browser' then 'ctx.req.extensions.get().unwrap()' else \
        if req_get == 'http::header::map::HeaderMap' then '&ctx.req.headers' else \
        if req_get == 'ctx_::set_header::SetHeader' then 'ctx_::sync::get(ctx) 'else 'ctx_::get(ctx).await?'
      ) + ';\n'
      args.push '&'+name
    else
      [name, type] = args_li[n]
      code = """args.#{name}"""
      if [
        'str', 'String'
      ].includes(type) or type.startsWith('[')
        code = '&' + code
      else if [
        'u8'
        'u16'
        'i8'
        'i16'
      ].includes type
        code = code + ' as ' + type
      args.push code

  rsRunPush '  match '+mod_func+'('

  rsRunPush "#{args.join(',')}"

  rsRunPush """
).await {
    Err(err)=>call_err("#{mod_func}", err, captcha, || """

  if args_len
    rsRunPush """dvec![#{args.join(',')}].join(",")"""
  else
    rsRunPush "s_::EMPTY"

  rsRunPush """).await?,\n    Ok("""
  if rt_type_li
    rsRunPush  'r)=>'
    returnType rsRunPush, proto_name, rt_name, rt_type_li
  else
    rsRunPush  '_)=>(State::OK, vec![])'
  rsRunPush '\n  }\n}\n'

  return


