#!/usr/bin/env coffee

> path > join
  @3-/write
  @3-/read
  fs > existsSync

< (dir)=>
  id = 0
  map = new Map
  exist = new Set

  out_proto = join dir, '_.proto'
  if existsSync out_proto
    for i from read(out_proto).split '\n'
      i = i.trim()
      if not i
        continue

      if i.endsWith ';'

        if i.startsWith '//'
          i = i.slice(2).trim()

        p = i.indexOf(' = ')

        if p > 0
          func_id = Number.parseInt i.slice(p+3,-1)
          if func_id
            name = i.slice(0,p)
            map.set name, func_id
            exist.add name

            if func_id > id
              id = func_id

  [
    # fundId
    (mod)=>
      (func)=>
        name = mod
        if mod != func
          name += func
        fid = map.get name
        if fid
          exist.delete name
          return fid

        map.set name, ++id

        return id

    # saveFundId
    =>
      proto = [
        '''
/**
 * rust crate function enum id
 */

syntax = "proto3";

enum Func {
  None = 0;\n'''
      ]

      for [name, func_id] from map.entries()
        line = "#{name} = #{func_id};"
        if exist.has name
          line = '// ' + line
        proto.push '  '+line

      proto.push "}\n"

      write(
        out_proto
        proto.join('\n')
      )
      return
  ]

