use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        // account
        "add_account" => {
            to_value(crate::commands::account::add_account(arg(args, "refreshToken")?).await)
        }
        "delete_account" => {
            to_value(crate::commands::account::delete_account(arg(args, "accountId")?).await)
        }
        "delete_accounts" => {
            to_value(crate::commands::account::delete_accounts(arg(args, "accountIds")?).await)
        }
        "reorder_accounts" => {
            to_value(crate::commands::account::reorder_accounts(arg(args, "accountIds")?).await)
        }
        "load_antigravity_switch_history" => {
            to_value(crate::commands::account::load_antigravity_switch_history())
        }
        "clear_antigravity_switch_history" => {
            to_value(crate::commands::account::clear_antigravity_switch_history())
        }
        "bind_account_fingerprint" => to_value(
            crate::commands::account::bind_account_fingerprint(
                arg(args, "accountId")?,
                arg(args, "fingerprintId")?,
            )
            .await,
        ),
        "get_bound_accounts" => to_value(
            crate::commands::account::get_bound_accounts(arg(args, "fingerprintId")?).await,
        ),
        "sync_current_from_client" => {
            to_value(crate::commands::account::sync_current_from_client(app_handle()?).await)
        }
        "sync_from_extension" => {
            to_value(crate::commands::account::sync_from_extension(app_handle()?).await)
        }
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
