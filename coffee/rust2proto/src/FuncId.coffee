#!/usr/bin/env coffee

> path > join
  @3-/write
  @3-/read
  @3-/snake > SNAKE
  fs > existsSync
  ./crateFuncName.js

< (dir_proto, dir_js)=>
  id = 0
  map = new Map

  out_proto = join dir_proto, '_.proto'
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

            if func_id > id
              id = func_id

  decode_import = new Set
  encode_import = new Set

  [
    # fundId
    (mod)=>
      (
        func, args, args_id, rt_type_li, rt_id
      )=>
        name = crateFuncName(mod,func)
        fid = map.get name
        if not fid
          fid = ++id
        map.set(
          name
          [
            fid
            args
            args_id
            rt_type_li
            rt_id
          ]
        )

        # if ( rt_id != undefined) and ( rt_id !== 0 )
        if rt_id
          decode_import.add rt_id

        # T0 encode already import
        if args_id
          encode_import.add args_id

        return fid

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

      encode_import = Array.from(
        encode_import
      ).map (i)=>
        'T'+i+'Encode,'

      decode_import = Array.from(
        decode_import
      ).map (i)=>
        'T'+i+'Decode,'

      js = [
        """
import {
  #{encode_import.join('')}
  #{decode_import.join('')}
  T0Encode, //CallLiEncode,
  T0Decode // BinLiDecode
} from './_.pb.js'

import hpc from '-/lib/hpc.js'

const [
  _set,
  _noArgs,
  _req
] = hpc(T0Encode, T0Decode), NULL = ()=>{}

export const set = _set;\n"""
      ]
      for [
        func_name
        fid
      ] from map.entries()

        has_func = Array.isArray fid
        if has_func
          [
            fid, args, args_id, rt_type_li, rt_id
          ] = fid
        else
          args_id = undefined

        line = "#{func_name} = #{fid};"

        if not has_func
          line = '// ' + line

        proto.push '  '+line

        if not has_func
          continue

        js_func = "export const #{func_name[0].toLowerCase()+func_name.slice(1)} = "
        has_args = args_id != undefined
        if has_args
          js_func += "(#{args.map(([name, type])=> "#{name} /* #{type} */").join(',')})=>_req("+fid
        else
          js_func += '_noArgs('+fid

        js_func += ','
        has_rt = rt_id != undefined
        if has_rt
          js_func += "T#{rt_id}Decode"
        else
          js_func += 'NULL'

        if has_args
          encode_args = args.map(([i])=>i).join(',')
          if args.length > 1
            encode_args = '['+encode_args+']'
          js_func += ",T#{args_id}Encode(#{encode_args})"

        js_func += ')'

        if has_rt
          js_func += ' /* '+rt_type_li.map(([name,type])=>name+':'+type).join(',')+' */'

        js.push js_func

      proto.push "}\n"
      write(
        out_proto
        proto.join('\n')
      )
      write(
        join dir_js, 'api.js'
        js.join('\n')
      )
      return
  ]

