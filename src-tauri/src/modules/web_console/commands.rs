//! Command whitelist for the local web console bridge.
//!
//! Keep HTTP transport, event polling, and static file serving outside this file.
//! Browser-exposed commands are grouped by feature in `commands/*`.

use serde_json::Value;

mod account_management;
mod account_overview;
mod codex;
mod common;
mod device;
mod import_export;
mod instances;
mod plugin;
mod provider_accounts;
mod settings;
mod wakeup;

pub(super) async fn dispatch_invoke(cmd: &str, args: &Value) -> Result<Value, String> {
    if let Some(value) = plugin::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = account_overview::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = settings::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = wakeup::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = codex::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = account_management::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = device::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = import_export::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = provider_accounts::dispatch(cmd, args).await? {
        return Ok(value);
    }
    if let Some(value) = instances::dispatch(cmd, args).await? {
        return Ok(value);
    }

    Err(format!(
        "Command '{}' is not exposed through the local web console yet",
        cmd
    ))
}
