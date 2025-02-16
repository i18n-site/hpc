export default allType = (all_type)=>
  proto = []
  push = proto.push.bind proto
  for li,pos in all_type
    push 'message T'+pos+' {'
    for i,p in li
      push '  '+i+' '+'t'+p+'='+(p+1)+';'
    push '}'

  '''syntax = "proto3";\n'''+proto.join('\n')
