#!/usr/bin/env coffee

> zx/globals:
  path > dirname join
  fs > readdirSync rmSync existsSync
  @3-/read
  @3-/utf8/utf8e.js
  @3-/camel
  ./jsType.js
  ./genRt.js
  ./genArgs.js
  ./genRs.js
  ./protoType.js
  ./UpCamel.js
  ./modName.js

rm = (dir)=>
  rmSync dir, recursive: true, force: true
  return

[
  jsTypeId
  ALL_TYPE
] = jsType()

export ALL_TYPE = ALL_TYPE
export jsTypeId = jsTypeId
_protoType = protoType jsTypeId


genFunc = (
  item, crate_func, funcMod, jsPush, typeId, paths, proto_name, crate, proto_exist
  _genRt
  index
  protoPush
  rsRunPush
  func_li
  funcId
)=>
  func = item.inner?.function
  if not ( func and item.visibility == 'public' )
    return

  {docs, name: func_name} = item
  mod_li = funcMod(item.id)
  mod_name = modName mod_li

  if crate_func != 0
    func_name = crate_func.get func_name
    if not func_name
      return

  {sig} = func
  {inputs} = sig

  up_func_name = mod_name + UpCamel(
    camel func_name
  )

  if docs
    docs = new Set docs.split('\n')
    has_captcha = docs.has '@captcha'

  from_req = []
  r = genArgs(
    up_func_name
    inputs
    jsPush
    typeId
    paths
    proto_name
    crate
    from_req
  )

  func_sign = ' * - '
  if has_captcha
    func_sign += '@captcha '
  if mod_li.length
    func_sign += mod_li.join('::') + '::'

  func_sign += "#{func_name}("

  name_type = args_id = rt_type_li = rt_id = undefined

  if r
    [
      args_id
      name
      name_type
    ] = r
    func_sign += name

  func_sign += ')'

  {
    resolved_path
  } = sig.output

  {
    name
    path
  } = resolved_path

  if path.endsWith('::Result')
    name = 'Result'

  if path.endsWith('::Void')
    name = 'Void'

  switch name
    when 'Void'
      break
    when 'Result'
      {
        type
      } = resolved_path.args.angle_bracketed.args[0]

      id = type.resolved_path?.id
      if id
        if not proto_exist.has id
          proto_exist.add id
          r = _genRt(
            index
            type
            jsPush
            protoPush
            jsTypeId
            typeId
            mod_name
          )
          if r
            [
              rt_id
              rt_name
              rt_type_li
            ] = r
            func_sign += " → #{rt_name}"
    else
      throw new Error 'TODO: ' + JSON.stringify resolved_path,null,2


  genRs(
    rsRunPush
    up_func_name
    name_type
    args_id, rt_type_li
    rt_id, rt_name, func_name, proto_name
    mod_li.map((i)=>i+'::').join('')
    from_req
    has_captcha
  )

  func_li.push func_sign
  funcId(
    up_func_name
    name_type
    args_id
    rt_type_li
    rt_id
  )
  return

export default (
  modFuncId
  crate
  dir_target
  proto_name
  crate_func
)=>
  _genRt = genRt()
  camel_crate = camel crate
  funcId = modFuncId UpCamel(camel_crate)
  {
    exitCode
    stderr
  } = await $"cargo rustdoc --output-format json -Z unstable-options -p #{crate}"
  if exitCode != 0
    throw new Error stderr

  json = JSON.parse(
    read join(
      dir_target, 'doc', crate+'.json'
    )
  )

  {
    index
    paths
  } = json

  rs_run = []
  rsRunPush = rs_run.push.bind rs_run

  js = []
  jsPush = js.push.bind js

  proto = []
  protoPush = proto.push.bind proto

  typeId = _protoType(protoPush)

  proto_exist = new Set
  func_li = []

  id2mod = new Map
  inner2mod_id = new Map

  for item from Object.values index
    module = item.inner?.module
    if module
      id2mod.set item.id,item.name
      for i from module.items
        inner2mod_id.set i,item.id

  _funcMod = (id, li)=>
    mid = inner2mod_id.get id
    if not mid
      return li
    li.push mid
    return _funcMod(mid,li)

  funcMod = (id)=>
    r = []
    li = _funcMod(id,[])
    li.pop()
    li.reverse()
    for i in li
      r.push id2mod.get(i)
    r

  for item from Object.values index
    genFunc(
      item, crate_func, funcMod, jsPush, typeId, paths, proto_name, crate, proto_exist
      _genRt
      index
      protoPush
      rsRunPush
      func_li
      funcId
    )

  [
    js
    """
/**
 * ## #{crate}
#{func_li.join('\n')}
 */

syntax = "proto3";
package #{camel_crate};
\n"""+proto.join('\n')
    rs_run.join('')
  ]

