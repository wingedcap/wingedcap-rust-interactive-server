use cross_storage::storage_set_object;
use wingedcap::{server::process_new_key, SetKeyInput, SetKeyOutput};

pub async fn set_key(input: SetKeyInput) -> Result<SetKeyOutput, String> {
    let SetKeyInput { timelock } = input;

    let (key_record, set_output) = process_new_key(timelock)?;

    let service_id = key_record.service.clone();

    let storage_name = format!("key_{}", service_id);

    let _ = storage_set_object(&storage_name, &key_record);

    Ok(set_output)
}
