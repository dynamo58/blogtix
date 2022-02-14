const $ = (r) => document.getElementById(r);

(function run() {
	const OPTION_SELECTOR = $("opt");

	if (OPTION_SELECTOR === null) return;

	const options = Array.from(OPTION_SELECTOR.options).map(opt => opt.value);

	OPTION_SELECTOR.addEventListener("change", e => {
		options.forEach(opt => {
			if (opt == OPTION_SELECTOR.value)
				[...document.getElementsByClassName(opt)].forEach(el => el.style.display = "block");
			else
				[...document.getElementsByClassName(opt)].forEach(el => el.style.display = "none");
		});
	});

	OPTION_SELECTOR.dispatchEvent(new Event("change"));
})();