#!/usr/bin/env coffee

> zx/globals:
  path > dirname join
  fs > readdirSync rmSync existsSync
  js-yaml > load
  @3-/read
  @3-/utf8/utf8e.js
  @3-/write
  @3-/camel
  ./jsType.js
  ./genRt.js
  ./genArgs.js
  ./protoType.js
  ./FuncId.js
  ./allType.js


UpCamel = (name)=>
  name.charAt(0).toUpperCase()+name.slice(1)

rm = (dir)=>
  rmSync dir, recursive: true, force: true
  return

export default (root)=>
  DIR_MOD = join root, 'mod'
  DIR_GEN = join root, 'gen'
  DIR_JS = join DIR_GEN, 'js'
  DIR_PROTO = join DIR_GEN, 'proto'

  exist_proto = new Set
  if existsSync DIR_PROTO
    for i from readdirSync(DIR_PROTO)
      if i.endsWith '.proto'
        i = i.slice(0,-6)
        if i != '_'
          exist_proto.add i

  [
    modFuncId
    saveFundId
  ] = FuncId DIR_PROTO

  cd DIR_MOD

  {
    stdout
  } = await $'cargo metadata --no-deps --format-version=1'
  {
    target_directory
  } = JSON.parse(stdout)

  TARGET_DOC = join(target_directory, 'doc')

  [
    jsTypeId
    all_type
  ] = jsType()

  call_id = jsTypeId  [
    'uint32' #func
    'bytes' #args
  ]
  call_li_id = jsTypeId  [
    'repeated uint32' #func_li
    'repeated bytes' #args_li
  ]
  bin_id = jsTypeId  [
    'bytes' #bin
  ]
  bin_li_id = jsTypeId  [
    'repeated bytes' #bin_li
  ]


  _protoType = protoType jsTypeId

  gen = (crate)=>
    camel_crate = camel crate
    outjs = join DIR_JS, camel_crate
    funcId = modFuncId UpCamel(camel_crate)
    rm outjs

    cd join(DIR_MOD, crate)
    {
      exitCode
      stderr
    } = await $'cargo rustdoc --output-format json -Z unstable-options'
    if exitCode != 0
      throw new Error stderr

    json = JSON.parse(read join(TARGET_DOC, crate+'.json')).index
    proto_exist = new Set
    proto = []
    protoPush = proto.push.bind proto
    typeId = _protoType(protoPush)
    func_li = []

    for item from Object.values json
      # 仅处理当前 crate
      if item.crate_id != 0
        continue

      func = item.inner?.function
      if func and item.visibility == 'public'

        {name: func_name} = item

        {sig} = func
        {inputs} = sig

        camel_func_name = camel func_name
        up_func_name = UpCamel(camel_func_name)

        func_id = funcId up_func_name
        console.log camel_crate + up_func_name

        r = genArgs(
          up_func_name
          inputs
          jsPush
          typeId
        )

        func_sign = " * *. #{func_name}("

        if r
          [
            args_id
            name
          ] = r
          func_sign += name
        func_sign += ')'

        {
          resolved_path
        } = sig.output
        {
          type
        } = resolved_path.args.angle_bracketed.args[0]

        id = type.resolved_path?.id
        if id
          if proto_exist.has id
            continue
          proto_exist.add id

        js = []
        jsPush = js.push.bind js

        r = genRt(
          json
          type
          jsPush
          protoPush
          jsTypeId
          typeId
        )

        if r
          [
            rt_id
            name
          ] = r
          func_sign += " → #{name}"

        func_li.push func_sign

        write(
          join outjs, "#{camel_func_name}.js"
          js.join('\n')
        )

    write(
      join DIR_PROTO, camel_crate + '.proto'
      """
/**
 * ## function
#{func_li.join('\n')}
 */

syntax = "proto3";
package #{camel_crate};

#{proto.join('\n')}"""
    )
    return


  for crate from load read join DIR_MOD,'_.yml'
    exist_proto.delete crate
    await gen crate

  for i from exist_proto
    rmSync join DIR_PROTO, i + '.proto'

  write(
    join DIR_JS, '_.proto'
    allType all_type
  )
  cd DIR_JS
  saveFundId()
  await import('@3-/protoscript/cli/index.js')
  return
