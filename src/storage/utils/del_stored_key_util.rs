use cross_storage::storage_del;

use super::super::constants::KEY_STORAGE_NAME_PREFIX;

pub fn del_stored_key(service_id: String) -> Result<(), String> {
    let storage_id = format!("{}_{}", KEY_STORAGE_NAME_PREFIX, service_id);

    storage_del(&storage_id)
}
