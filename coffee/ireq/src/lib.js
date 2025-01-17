const req = async (url, errCatch, context_type, body, decode) => {
	const r = await fetch(url, {
			method: "PUT",
			headers: {
				"content-type": context_type,
			},
			body,
		}),
		{ status } = r

	if (status == 200) {
		if (decode) {
			return decode(await r.arrayBuffer())
		}
	}

	return errCatch(status, r)
}

export default async (errCatch, CallEncode, CallLiEncode, BinLiDecode) => {
	let _url, batch_li, decode_li

	const _req = (...args) => req(_url, errCatch, ...args)

	return [
		// setUrl
		(url) => {
			_url = url
		},
		// req
		(func_id, args_bin, decode) => {
			if (batch_li) {
				batch_li_li.push([func_id, args_bin])
				decode_li.push(decode)
				return
			}
			return _req("1", CallEncode([func_id, args_bin]), decode)
		},
		// batch_li
		(call) => {
			batch_li = []
			decode_li = []
			try {
				call()
				return _req("b", CallLiEncode(batch_li), (b) =>
					BinLiDecode(b).map((bin, pos) => {
						const d = decode_li[pos]
						if (d) {
							return d(bin)
						}
					}),
				)
			} finally {
				batch_li = decode_li = undefined
			}
		},
	]
}
