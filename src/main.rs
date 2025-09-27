use cross_storage::{storage_del, storage_get, storage_set};
use wingedcap::{
    handlers_vec, rand_hex_str, server::sk_to_pk, GET_KEY_ENDPOINT, PING_KEY_ENDPOINT,
    SET_KEY_ENDPOINT,
};

use dioxus::LaunchBuilder;

use crate::{
    api::{get_key, ping_key, set_key},
    app::AppProps,
    constants::TUNNEL_HOST,
    storage::get_stored_keys,
    utils::tunnel_serve,
};

mod constants;

mod types;

mod utils;

mod storage;

mod api;

mod ui;

mod components;

mod app;

fn main() {
    let handlers = handlers_vec![
        (SET_KEY_ENDPOINT, set_key),
        (PING_KEY_ENDPOINT, ping_key),
        (GET_KEY_ENDPOINT, get_key)
    ];

    let (server_id, server_sk) = if let (Ok(server_id), Ok(server_sk)) =
        (storage_get("server_id"), storage_get("server_sk"))
    {
        (server_id, server_sk)
    } else {
        let server_id = uuid::Uuid::new_v4().to_string();
        let server_sk = rand_hex_str();

        let _ = storage_set("server_id", &server_id);
        let _ = storage_set("server_sk", &server_sk);

        if let Ok(stored_keys) = get_stored_keys() {
            for key_record in stored_keys {
                let storage_id = key_record.service.clone();

                let _ = storage_del(&storage_id);
            }
        }

        (server_id, server_sk)
    };

    let server_pk = sk_to_pk(&server_sk).unwrap();

    {
        let server_id = server_id.clone();

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            let _ = tunnel_serve(handlers, server_sk, TUNNEL_HOST.to_string(), server_id).await;
        });

        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let _ = rt.block_on(tunnel_serve(
                handlers,
                server_sk,
                TUNNEL_HOST.to_string(),
                server_id,
            ));
        });
    }

    LaunchBuilder::new()
        .with_context(AppProps {
            tunnel_host: TUNNEL_HOST.to_string(),
            server_id,
            server_pk,
        })
        .launch(app::App);
}
