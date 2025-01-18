#!/usr/bin/env coffee

> zx/globals:
  ./mod.js

{
  stdout
} = await $'cargo metadata --no-deps --format-version=1'

$.verbose = true

{
  target_directory: dir_target
} = JSON.parse(stdout)

[
  crate_map
  mod2crate
] = await mod(dir_target)

console.log crate_map
console.log mod2crate

# ./lib.js:rust2proto
# await rust2proto process.cwd()
process.exit()
