#!/usr/bin/env node

import putout from "putout"
import {readFileSync} from 'node:fs'  

console.log(putout(
`
import {x} from 'a'; 
import {b} from 'a'

const a = "n'"
`,
  {
		plugins: [
      "esm"
    ],
    printer: [
        'putout', {
          format: {
              quote: '"',
          },
          semantics: {
              encodeSingleQuote: false,
              encodeDoubleQuote: true,
          }
        }
    ]
  }
).code)

