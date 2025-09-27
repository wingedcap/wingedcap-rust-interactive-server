use wingedcap::{server::process_get_key, GetKeyInput, GetKeyOutput};

use crate::storage::utils::get_stored_keys_util::get_stored_keys;

pub async fn get_key(payload: GetKeyInput) -> Result<GetKeyOutput, String> {
    let stored_keys = get_stored_keys()?;

    let matching_key = stored_keys
        .iter()
        .find(|key_record| key_record.receiver == payload.id);

    match matching_key {
        Some(key_record) => {
            let get_output = process_get_key(key_record)?;

            Ok(get_output)
        }

        None => Err(format!("get_key: Key not found")),
    }
}
