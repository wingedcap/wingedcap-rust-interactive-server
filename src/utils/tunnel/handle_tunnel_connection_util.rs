use std::sync::Arc;

use tokio::sync::Mutex;

use tokio_tungstenite_wasm::connect;

use wingedcap::{
    client::{ws_receive, ws_send},
    server::{serve, Handler},
    ReceiveFuture, SecureChannel, SendFuture,
};

use crate::utils::get_ws_scheme;

pub async fn handle_tunnel_connection(
    tunnel_id: String,
    handlers: Arc<Vec<Handler>>,
    sk: String,
    tunnel_host: String,
) -> Result<(), String> {
    let scheme = get_ws_scheme()?;

    let url = format!("{scheme}://{tunnel_host}/join/{tunnel_id}");

    let socket = connect(&url).await.expect("Failed to connect to tunnel");

    let shared_socket = Arc::new(Mutex::new(socket));

    let sender_socket = shared_socket.clone();

    let raw_send = move |message: String| {
        let socket = sender_socket.clone();

        let fut = async move {
            let mut socket = socket.lock().await;
            ws_send(&mut socket, &message).await
        };

        Box::pin(fut) as SendFuture
    };

    let receiver_socket = shared_socket.clone();

    let raw_receive = move || {
        let timeout_ms = 10000;

        let socket = receiver_socket.clone();

        let fut = async move {
            let mut socket = socket.lock().await;
            ws_receive(&mut socket, timeout_ms).await
        };

        Box::pin(fut) as ReceiveFuture
    };

    let channel = SecureChannel::new(raw_send, raw_receive, Some(sk), None).await?;

    serve(handlers, channel).await
}
