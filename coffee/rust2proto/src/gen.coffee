#!/usr/bin/env coffee

> zx/globals:
  ./FuncId.js
  ./mod.js
  ./crateProto.js:@ > ALL_TYPE jsTypeId
  path > join
  @3-/write
  fs > readdirSync unlinkSync existsSync
  ./allType.js
  ./rsSave.js

await do =>
  root = process.cwd()

  {
    stdout
  } = await $'cargo metadata --no-deps --format-version=1'

  $.verbose = true

  {
    target_directory: dir_target
  } = JSON.parse(stdout)

  [
    crate_map
    crate2proto
  ] = await mod(dir_target)


  dir_gen = join root, 'gen'
  dir_proto = join dir_gen, 'proto'
  dir_js = join dir_gen, 'js'

  if existsSync dir_proto
    for i from readdirSync dir_proto
      if i.endsWith('.proto') and not i.startsWith('_')
        name = i.slice(0,-6)
        if not crate_map.has name
          unlinkSync join(dir_proto, i)

  [
    modFuncId
    saveFuncId
  ] = FuncId dir_proto, dir_js


  # T0 CallLi and BinLi
  jsTypeId  [
    'repeated uint32' #func_li
    'repeated bytes' #args_li
  ]

  # T1 Captcha
  jsTypeId  [
    'bytes'
    'bytes'
    'bytes'
  ]

  # T2 Err Code Li
  jsTypeId [
    'repeated uint32'
  ]

  proto2str = new Map
  rs_run = []
  for [crate, proto_name] from crate2proto.entries()
    [
      js
      proto
      _run
    ] = await crateProto(
      modFuncId
      crate
      dir_target
      proto_name
      crate_map.get(crate)
    )
    rs_run.push _run

    for [name, code_li] from js
      write(
        join dir_js, proto_name, name+'.js'
        code_li.join('\n')
      )

    p = proto2str.get proto_name
    if p
      p += '\n'+proto
    else
      p = proto
    proto2str.set proto_name, p

  for [proto, code] from proto2str.entries()
    write(
      join dir_proto, proto+'.proto'
      code
    )


  write(
    join dir_js, '_.proto'
    allType ALL_TYPE
  )

  await saveFuncId()
  cd dir_js
  await import('@3-/protoscript/cli/index.js')
  rsSave(
    dir_gen
    rs_run.join('\n')
  )
  return

process.exit()
