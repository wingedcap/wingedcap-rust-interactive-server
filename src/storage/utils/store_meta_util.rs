use wingedcap::client::ServerMeta;

use cross_storage::storage_set_object;

use super::super::constants::SERVER_META_STORAGE_NAME;

pub fn store_meta(meta: ServerMeta) -> Result<(), String> {
    storage_set_object(SERVER_META_STORAGE_NAME, meta)
}
