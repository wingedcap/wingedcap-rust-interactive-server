pub fn get_ws_scheme() -> Result<String, String> {
    #[cfg(not(target_family = "wasm"))]
    let scheme = "ws";

    #[cfg(target_family = "wasm")]
    let scheme = {
        let window = web_sys::window().ok_or("Failed to get window location href")?;
        let location = window.location();
        let href = location.href().map_err(|_| "Failed to get href")?;
        let href_scheme = href.split("://").nth(0).ok_or("Failed to get scheme")?;
        let is_secure = href_scheme == "https";
        if is_secure {
            "wss"
        } else {
            "ws"
        }
    };

    Ok(scheme.to_string())
}
