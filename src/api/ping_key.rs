use wingedcap::{server::process_ping_key, PingKeyInput, PingKeyOutput};

use crate::storage::{get_stored_keys, store_key};

pub async fn ping_key(payload: PingKeyInput) -> Result<PingKeyOutput, String> {
    let stored_keys = get_stored_keys()?;

    let matching_key = stored_keys
        .iter()
        .find(|key_record| key_record.sender == payload.id);

    match matching_key {
        Some(key_record) => {
            let mut key_record = key_record.clone();

            let ping_output = process_ping_key(&mut key_record)?;

            store_key(key_record)?;

            Ok(ping_output)
        }

        None => {
            let error = format!("ping_key: key not found");
            tracing::error!(error);
            Err(error)
        }
    }
}
