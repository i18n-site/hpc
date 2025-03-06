#!/usr/bin/env coffee

> zx/globals:
  path > dirname basename join
  @3-/write

ROOT = dirname import.meta.dirname
cd dirname ROOT

{
  stdout
} = await $"fd --type f --hidden --no-ignore-vcs '.git/config' -p -E 'node_modules' | xargs -I {} grep -m 1 'url =' {}"

li = []
stdout.trim().split("\n").forEach (i)=>
  i = i.replaceAll(':443','')
  p = i.indexOf('github.com')
  if p < 0
    return
  i = i.slice(p).slice(0,-4)
  li.push "[#{basename(i)}](https://#{i})"
  return

write(join(ROOT,'git.md'),li.join(' '))

cd ROOT
await $"bun x mdt ."
