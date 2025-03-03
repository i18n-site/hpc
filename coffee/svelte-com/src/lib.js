import { readdirSync } from "fs"
import { basename, dirname, join } from "path"
import { walkRel } from "@3-/walk"
import { svelte } from "@sveltejs/vite-plugin-svelte"
import coffeePreprocessor from "./coffee.js"
import comJs from "./comJs.js"
import pug from "./pug.js"
import stylus from "./stylus.js"
import svelteCustomElement from "./svelteCustomElement.js"
import camel from '@3-/camel/lowCamel.js'

const IGNORE_WARN = new Set([
  'a11y-click-events-have-key-events',
  'a11y_consider_explicit_label',
  'a11y-missing-content'
])

export default async (src) => {
	const entry = {},
		root = dirname(src)

	for await (const rel of walkRel(src)) {
		if (rel.endsWith(".svelte")) {
			const fname = basename(rel)
			entry[rel.slice(0, -fname.length) + camel(fname.slice(0, -7))] =
				"./" + rel
		}
	}

	const compilerOptions = {
		customElement: true,
	}

  // 禁用哈希后缀
  compilerOptions.cssHash = ({ hash, css, name, filename }) => {
    return '_-_'
  }

	return {
		plugins: [
			comJs,
			svelteCustomElement,
			svelte({
        onwarn: ({code, message})=>{
          if(code == 'a11y-missing-attribute'){
            return !message.includes('<a>')
          }
          return !IGNORE_WARN.has(code)
        },
				preprocess: [
          coffeePreprocessor, 
          pug(src), 
          stylus
        ],
				compilerOptions,
			}),
		],
		build: {
			outDir: join(root, "dist", src.slice(root.length + 1)),
			cssMinify: false,
			emptyOutDir: true,
			minify: false,
			rollupOptions: {
				output: {
					entryFileNames: `[name].js`,
					chunkFileNames: `[name].js`,
					assetFileNames: `[name].[ext]`,
				},
			},
			lib: {
				entry,
				fileName: (_format, entryName) => `${entryName}.js`,
				formats: ["es"],
			},
		},
	}
}
