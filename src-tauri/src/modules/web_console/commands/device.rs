use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        // device
        "get_device_profiles" => {
            to_value(crate::commands::device::get_device_profiles(arg(args, "accountId")?).await)
        }
        "bind_device_profile" => to_value(
            crate::commands::device::bind_device_profile(
                arg(args, "accountId")?,
                arg(args, "mode")?,
            )
            .await,
        ),
        "bind_device_profile_with_profile" => to_value(
            crate::commands::device::bind_device_profile_with_profile(
                arg(args, "accountId")?,
                arg(args, "profile")?,
            )
            .await,
        ),
        "list_device_versions" => {
            to_value(crate::commands::device::list_device_versions(arg(args, "accountId")?).await)
        }
        "restore_device_version" => to_value(
            crate::commands::device::restore_device_version(
                arg(args, "accountId")?,
                arg(args, "versionId")?,
            )
            .await,
        ),
        "delete_device_version" => to_value(
            crate::commands::device::delete_device_version(
                arg(args, "accountId")?,
                arg(args, "versionId")?,
            )
            .await,
        ),
        "restore_original_device" => {
            to_value(crate::commands::device::restore_original_device().await)
        }
        "open_device_folder" => to_value(crate::commands::device::open_device_folder().await),
        "preview_generate_profile" => {
            to_value(crate::commands::device::preview_generate_profile().await)
        }
        "preview_current_profile" => {
            to_value(crate::commands::device::preview_current_profile().await)
        }
        "list_fingerprints" => to_value(crate::commands::device::list_fingerprints().await),
        "get_fingerprint" => {
            to_value(crate::commands::device::get_fingerprint(arg(args, "fingerprintId")?).await)
        }
        "generate_new_fingerprint" => {
            to_value(crate::commands::device::generate_new_fingerprint(arg(args, "name")?).await)
        }
        "capture_current_fingerprint" => {
            to_value(crate::commands::device::capture_current_fingerprint(arg(args, "name")?).await)
        }
        "create_fingerprint_with_profile" => to_value(
            crate::commands::device::create_fingerprint_with_profile(
                arg(args, "name")?,
                arg(args, "profile")?,
            )
            .await,
        ),
        "apply_fingerprint" => {
            to_value(crate::commands::device::apply_fingerprint(arg(args, "fingerprintId")?).await)
        }
        "delete_fingerprint" => {
            to_value(crate::commands::device::delete_fingerprint(arg(args, "fingerprintId")?).await)
        }
        "delete_unbound_fingerprints" => {
            to_value(crate::commands::device::delete_unbound_fingerprints().await)
        }
        "rename_fingerprint" => to_value(
            crate::commands::device::rename_fingerprint(
                arg(args, "fingerprintId")?,
                arg(args, "name")?,
            )
            .await,
        ),
        "get_current_fingerprint_id" => {
            to_value(crate::commands::device::get_current_fingerprint_id().await)
        }
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
