use futures::StreamExt;
use std::sync::Arc;
use tokio_tungstenite_wasm::{connect, Message};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures;

use wingedcap::server::Handler;

use crate::utils::{get_ws_scheme, handle_tunnel_connection};

pub async fn tunnel_serve(
    handlers: Vec<Handler>,
    sk: String,
    tunnel_host: String,
    server_id: String,
) -> Result<(), String> {
    let handle_tunnel = {
        let tunnel_host = tunnel_host.clone();
        let handlers = Arc::new(handlers);
        let sk = sk.clone();

        move |tunnel_id: String| {
            let handlers = Arc::clone(&handlers);
            let sk = sk.clone();
            let tunnel_host = tunnel_host.clone();

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                let _ = handle_tunnel_connection(tunnel_id, handlers, sk, tunnel_host).await;
            });

            #[cfg(not(target_arch = "wasm32"))]
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let _ = rt.block_on(handle_tunnel_connection(
                    tunnel_id,
                    handlers,
                    sk,
                    tunnel_host,
                ));
            });
        }
    };

    let scheme = get_ws_scheme()?;

    let url = format!("{scheme}://{tunnel_host}/listen/{server_id}");

    let ws_stream = connect(&url)
        .await
        .expect("Failed to connect to tunnel host");

    let (_ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Text(text) = message {
            handle_tunnel.clone()(text.to_string());
        }
    }

    Ok(())
}
