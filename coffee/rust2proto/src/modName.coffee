> ./UpCamel.js

< (li)=>
  r = []
  for i from li
    if i.endsWith '_'
      i = i.slice(0,-1)
    r.push UpCamel i
  r.join('')


