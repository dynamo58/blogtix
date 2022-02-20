document.getElementById("submitBtn").onclick = async () => {
	const responseField = document.getElementById("responseText");
	const loadingImg = document.getElementById("loadingIcon");

	loadingImg.style.display = "block";

	const nameRef         = window.location.pathname.split("/").pop();
	const newName         = document.getElementById("articleName").value;
	const newDescription  = document.getElementById("articleDescription").value;
	const newContent      = document.getElementById("articleContent").value;
	const author          = document.getElementById("authorName").value;
	const _password       = document.getElementById("_authorPassword").value;

	const location = window.location.pathname.split("/").slice(-2, -1)[0];
	let _method;
	switch(location) {
		case "edit":
			_method = "PUT";
			break;
		case "new":
			_method = "POST";
			break;
	}

	await fetch("/api/article", {
		method: _method,
		headers: {
			'Accept': '*/*',
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			"name_ref": nameRef,
			"name": newName,
			"content": newContent,
			"description": newDescription,
			"author_name": author,
			"password": _password
		})
	})
		.then(res => res.json())
		.then(data => {
			loadingImg.style.display = "none";
			responseField.innerText = data.message;
		});
}
