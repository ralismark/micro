const pattern = /^(X-)?(Clacks-Overhead)$/i;

let meta = Array.from(document.querySelectorAll("meta"));
let clacks = meta.filter((tag) => pattern.test(tag.getAttribute("http-equiv")))
	.map((tag) => tag.getAttribute("content"))

browser.runtime.sendMessage({ clacks: clacks })
