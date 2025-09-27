use wingedcap::server::KeyRecord;

use cross_storage::storage_set_object;

use super::super::constants::KEY_STORAGE_NAME_PREFIX;

pub fn store_key(key_record: KeyRecord) -> Result<(), String> {
    let storage_id = key_record.service.clone();

    let storage_id = format!("{}_{}", KEY_STORAGE_NAME_PREFIX, storage_id);

    storage_set_object(&storage_id, key_record)
}
