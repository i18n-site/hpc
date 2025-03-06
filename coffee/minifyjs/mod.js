#!/usr/bin/env bun

import putout from "putout"
import { mkdir, readFile, writeFile } from "node:fs/promises"
import { minify } from "@swc/core"
import func2arrow from "./func2arrow.js"
import { writeFileSync } from "node:fs"

const main = async (code, filename) => {
	code = putout(code, {
		plugins: ["esm"],
		rules: {
			"esm/remove-empty-import": [
				"off",
				{
					ignore: [],
				},
			],
		},
		printer: [
			"putout",
			{
				format: {
					quote: '"',
				},
				semantics: {
					encodeSingleQuote: false,
					encodeDoubleQuote: true,
				},
			},
		],
	}).code

	code = await func2arrow(code)
	const r = await minify(code, {
			compress: {
				toplevel: true, // 启用顶层优化
				unused: true, // 删除未导出变量
				defaults: true, // 启用所有压缩选项
				drop_console: true,
				hoist_funs: true,
				hoist_vars: true,
				booleans_as_integers: true,
				reduce_funcs: true,
				unsafe: true,
			},
			mangle: {
				toplevel: true, // 缩短顶层变量名
			},
			sourceMap: true,
			ecma: 2022,
			module: true,
		}),
		map = JSON.parse(r.map)

	map.sources[0] = filename

	r.map = JSON.stringify(map)
	return r
}

export default main
