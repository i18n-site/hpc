#!/usr/bin/env bun

import postcss from "postcss";
import autoprefixer from "autoprefixer";
import postcssStyl from "postcss-styl";

const stylus = (code, from) => {
  const css = postcss([autoprefixer]).process(code, {
    syntax: postcssStyl,
    from
  }).root.toString();
  return css;
}
export default stylus
/*
const stylusCode = `
a
  transform scale(0.5)
  xxx x
  xbbb:w
  appearance none
  &:hover
    color #fe4334
`;


*/
