import extractReplaceInclude from "@3-/extract/extractReplaceInclude.js"
import pug2htm from "./pug2htm.js"
import pugHack from "./pugHack.js"

export default {
	markup: (o) => {
		const r = {
			code: extractReplaceInclude(
				'<template lang="pug">',
				"</template>",
				(pug) => {
					const { code } = pug2htm(pugHack(pug.slice(21, -11)), o.filename)
					return "\n" + code
				},
				o.content,
			),
		}
		return r
	},
}
