use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        // oauth
        "start_oauth_login" => {
            to_value(crate::commands::oauth::start_oauth_login(app_handle()?).await)
        }
        "prepare_oauth_url" => {
            to_value(crate::commands::oauth::prepare_oauth_url(app_handle()?).await)
        }
        "complete_oauth_login" => {
            to_value(crate::commands::oauth::complete_oauth_login(app_handle()?).await)
        }
        "submit_oauth_callback_url" => to_value(
            crate::commands::oauth::submit_oauth_callback_url(
                app_handle()?,
                arg(args, "callbackUrl")?,
            )
            .await,
        ),
        "cancel_oauth_login" => to_value(crate::commands::oauth::cancel_oauth_login().await),
        // import
        "import_from_old_tools" => to_value(crate::commands::import::import_from_old_tools().await),
        "import_fingerprints_from_old_tools" => {
            to_value(crate::commands::import::import_fingerprints_from_old_tools().await)
        }
        "import_fingerprints_from_json" => to_value(
            crate::commands::import::import_fingerprints_from_json(arg(args, "jsonContent")?).await,
        ),
        "import_from_local" => {
            to_value(crate::commands::import::import_from_local(app_handle()?).await)
        }
        "import_from_json" => {
            to_value(crate::commands::import::import_from_json(arg(args, "jsonContent")?).await)
        }
        "import_from_files" => {
            to_value(crate::commands::import::import_from_files(arg(args, "filePaths")?).await)
        }
        "export_accounts" => {
            to_value(crate::commands::import::export_accounts(arg(args, "accountIds")?).await)
        }
        // data_transfer
        "data_transfer_get_user_config" => {
            to_value(crate::commands::data_transfer::data_transfer_get_user_config())
        }
        "data_transfer_apply_user_config" => to_value(
            crate::commands::data_transfer::data_transfer_apply_user_config(
                app_handle()?,
                arg(args, "config")?,
            ),
        ),
        "data_transfer_get_instance_store" => to_value(
            crate::commands::data_transfer::data_transfer_get_instance_store(arg(
                args, "platform",
            )?),
        ),
        "data_transfer_replace_instance_store" => to_value(
            crate::commands::data_transfer::data_transfer_replace_instance_store(
                arg(args, "platform")?,
                arg(args, "store")?,
            ),
        ),
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
