use wingedcap::server::KeyRecord;

use cross_storage::storage_get_matches;

use super::super::constants::KEY_STORAGE_NAME_PREFIX;

pub fn get_stored_keys() -> Result<Vec<KeyRecord>, String> {
    let matches: Vec<(String, KeyRecord)> = storage_get_matches(KEY_STORAGE_NAME_PREFIX)?;

    Ok(matches.iter().map(|(_, value)| value.clone()).collect())
}
