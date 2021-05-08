function loadClacks(strings, tabId)
{
	console.log("loadClack", tabId, strings);
	if(strings.length > 0) {
		let title = {
			tabId: tabId,
			title: strings.join("\n")
		};
		browser.pageAction.setTitle(title);
		browser.pageAction.show(tabId);
	}
}

browser.webRequest.onCompleted.addListener((details) => {
	if(details.frameId > 0 // ignore subframes
	   || details.tabId < 0 // must be in a tab
	   || details.documentUrl !== undefined // must be top-level document
	) {
		return;
	}

	const pattern = /^(X-)?(Clacks-Overhead)$/i;
	const clacks_entries = details.responseHeaders
		.filter((header) => pattern.test(header.name))
		.map((header) => header.value);

	loadClacks(clacks_entries, details.tabId);
},
	// Note: <all_urls> matches HTTP and HTTPS only
	{ "urls": [ "http://*/*", "https://*/*" ] },
	[ "responseHeaders" ]);

browser.runtime.onMessage.addListener((message, sender, sendResponse) => {
	loadClacks(message.clacks, sender.tab.id);
});

browser.webNavigation.onBeforeNavigate.addListener((details) => {
	if(details.frameId !== 0) {
		return;
	}

	browser.pageAction.hide(details.tabId);
});

browser.pageAction.onClicked.addListener((tab) => {
	sequence = [
		'icons/icon-g.svg',
		'icons/icon-n.svg',
		'icons/icon-u.svg',
	];

	function show(idx)
	{
		if(idx === sequence.length) {
			browser.pageAction.setIcon({ path: null, tabId: tab.id });
		} else {
			browser.pageAction.setIcon({ path: sequence[idx], tabId: tab.id });
			setTimeout(() => show(idx + 1), 300);
		}
	}

	show(0);
});
