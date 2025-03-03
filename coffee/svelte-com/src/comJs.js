import { basename, dirname, join, relative } from "path"
import css2mod from "@3-/css2js/css2mod.js"
import extract from "@3-/extract"
import write from "@3-/write"
import IS_PROD from "./IS_PROD.js"
import camel from "@3-/camel/lowCamel.js"
import minifyCss from "./minifyCss.js"

export default {
	name: "svelte-post-process",
	enforce: "post",
	generateBundle: async ({ dir }, bundle) => {
		const dir_css = join(dirname(dir), "css")
		for (const [fileName, chunk] of Object.entries(bundle)) {
			if (!fileName.endsWith(".js")) continue
			let css
			const is_svelte =
				chunk.type === "chunk" &&
				chunk.facadeModuleId?.endsWith(".svelte") &&
				chunk.isEntry
			if (is_svelte) {
				chunk.code = chunk.code
					.replace(
						"append_styles($$anchor, $$css);",
						"// append_styles($$anchor, $$css);",
					)
					.replace(/\s*\w+ as append_styles,/, "")
					.replaceAll(' class="_-_"', "")
					.replaceAll('class="_-_ ', 'class="')
					.replaceAll(" _-_ ", " ")
					.replaceAll(' _-_"', '"')

				css = extract("const $$css = {", "function ", chunk.code)

				const no_ext_name = fileName.slice(0, -3)

				let define

				if (css) {
					const name = "i-" + camel(basename(fileName.slice(0, -3)))
					css = css
						.slice(css.indexOf('code: "'), css.lastIndexOf("};"))
						.slice(6)
						.trim()
						.slice(1, -1)
						.replaceAll("\\n", "\n")
						.replaceAll("\\t", "\t")

					let r = minifyCss(css, fileName, false)
					if (r === undefined) {
						throw new Error("invalid css :\n" + css)
					}
					css = r.code.toString()

					css = css
						.trim()
						.replaceAll("._-_", "")
						.replaceAll("_-_-", "")
						.replaceAll(":where()", "")
						.replace(/^([^\s].*)\{$/gm, (m) => {
							if (m.charAt(0) == "@") {
								return m
							}
							return m
								.split(",")
								.map((i) => name + ">" + i)
								.join(",")
						})
						.replaceAll(name + ">_ ", name + " ")

					write(join(dir_css, no_ext_name + ".css"), css)

					r = minifyCss(css, fileName, true)
					if (r) {
						const { code } = r
						css = code.toString()
					}
					write(join(dir_css, fileName), css2mod(css))
					define = "S"
				} else {
					define = "C"
				}
				chunk.code =
					`import {${define}} from '-/dom/_.js'\n` +
					chunk.code.replace('customElements.define("i-', define + '("')
			}

			if (fileName.startsWith("custom-element")) {
				chunk.code =
					`import {DOC,Tag} from '-/dom/_.js'\n` +
					chunk.code
						.split("\n")
						.filter((i) => !i.trimStart().startsWith("append_styles as "))
						.join("\n")
						.replaceAll("document", "DOC")
						.replaceAll("DOC.createElement", "Tag")
			}

			let outname
		}
	},
}
