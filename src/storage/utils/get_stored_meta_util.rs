use wingedcap::client::ServerMeta;

use cross_storage::storage_get;

use crate::storage::constants::SERVER_META_STORAGE_NAME;

pub fn get_stored_meta() -> ServerMeta {
    match storage_get(SERVER_META_STORAGE_NAME) {
        Ok(meta) => match serde_json::from_str(&meta) {
            Ok(meta) => meta,
            Err(_) => ServerMeta::default(),
        },
        Err(_) => ServerMeta::default(),
    }
}
