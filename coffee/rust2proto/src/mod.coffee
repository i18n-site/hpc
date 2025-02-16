#!/usr/bin/env coffee

> @3-/read
  zx/globals:
  path > join


export default (dir_target)=>
  await $'cargo rustdoc --output-format json -Z unstable-options -p mod'

  mod = JSON.parse read join(
    dir_target
    'doc/mod.json'
  )

  crate_set = new Set
  mod2crate = new Map

  modEnd = (name)=>
    for i from crate_set
      mod2crate.set i, name
    crate_set = new Set
    return

  crate_map = new Map

  for [id,val] from Object.entries mod.index
    if 'public' == val.visibility
      {
        inner: {
          module
          use
        }
      } = val
      if module and not module.is_crate
        modEnd val.name
      else if use
        {
          source
        } = use
        if use.is_glob
          crate_map.set source, 0
          crate_set.add source
        else
          {
            name
          } = use
          # use captcha_::captcha as xxx , name will be xxx
          p = source.indexOf '::'
          if p > 0
            crate = source.slice(0,p)
            crate_set.add crate
            func = source.slice(p+2)
            crate_set.add crate
            m = crate_map.get crate
            if not m
              m = new Map
              crate_map.set crate, m
            m.set func, name

  return [
    crate_map
    mod2crate
  ]
