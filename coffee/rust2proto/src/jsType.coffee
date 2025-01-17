#!/usr/bin/env coffee

export default =>
  exist = new Map
  all_type = []
  [
    (
      type_li
    )=>
      type_json = JSON.stringify(type_li)
      id = exist.get(type_json)
      if not id
        id = all_type.length
        exist.set type_json, id
        all_type.push type_li
      return id
    all_type
  ]
