function setTokenCookie() {
    let token = document.getElementById('set-token').value;
    let tokenCookie = 'token=' + token + "; SameSite=None; secure";
    document.cookie = tokenCookie;
    setFormTokens(token);
    document.getElementById("cookie-block").hidden = true;
}

function clearTokenCookie() {
    document.cookie = "token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; SameSite=None; secure";
    document.getElementById("cookie-block").hidden = false;
}

function setFormTokens(token) {
    let tokenFields = document.querySelectorAll(".token");
    for (t of tokenFields) {
        t.value = token;
    }
}

// if cookie is set, use it to pass the token
let c_pairs = document.cookie.split(";");
let cookie_set = false;
for (c of c_pairs) {
    if (c.trim().split("=")[0].startsWith("token")){
        // stick token into all the form input fields
        let token = c.split("=")[1];
        setFormTokens(token);
        cookie_set = true;
    }
}

if (!cookie_set) {
    document.getElementById("cookie-block").hidden = false;
}
