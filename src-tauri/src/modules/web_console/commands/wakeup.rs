use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        "wakeup_ensure_runtime_ready" => {
            to_value(crate::commands::wakeup::wakeup_ensure_runtime_ready(
                opt_arg(args, "officialLsVersionMode")?,
            ))
        }
        "wakeup_set_official_ls_version_mode" => to_value(
            crate::commands::wakeup::wakeup_set_official_ls_version_mode(opt_arg(args, "mode")?),
        ),
        "trigger_wakeup" => to_value(
            crate::commands::wakeup::trigger_wakeup(
                arg(args, "accountId")?,
                arg(args, "model")?,
                opt_arg(args, "prompt")?,
                opt_arg(args, "maxOutputTokens")?,
                opt_arg(args, "cancelScopeId")?,
                opt_arg(args, "officialLsVersionMode")?,
            )
            .await,
        ),
        "fetch_available_models" => {
            to_value(crate::commands::wakeup::fetch_available_models().await)
        }
        "wakeup_validate_crontab" => to_value(crate::commands::wakeup::wakeup_validate_crontab(
            arg(args, "expr")?,
        )),
        "wakeup_sync_state" => to_value(
            crate::commands::wakeup::wakeup_sync_state(
                app_handle()?,
                arg(args, "enabled")?,
                arg(args, "tasks")?,
                opt_arg(args, "officialLsVersionMode")?,
                opt_arg(args, "runStartupTasks")?,
            )
            .await,
        ),
        "wakeup_run_enabled_tasks" => to_value(
            crate::commands::wakeup::wakeup_run_enabled_tasks(
                app_handle()?,
                opt_arg(args, "triggerSource")?,
                opt_arg(args, "officialLsVersionMode")?,
            )
            .await,
        ),
        "wakeup_load_history" => to_value(crate::commands::wakeup::wakeup_load_history()),
        "wakeup_add_history" => to_value(crate::commands::wakeup::wakeup_add_history(arg(
            args, "items",
        )?)),
        "wakeup_clear_history" => to_value(crate::commands::wakeup::wakeup_clear_history()),
        "wakeup_cancel_scope" => to_value(crate::commands::wakeup::wakeup_cancel_scope(arg(
            args,
            "cancelScopeId",
        )?)),
        "wakeup_release_scope" => to_value(crate::commands::wakeup::wakeup_release_scope(arg(
            args,
            "cancelScopeId",
        )?)),
        "wakeup_verification_load_state" => {
            to_value(crate::commands::wakeup::wakeup_verification_load_state())
        }
        "wakeup_verification_load_history" => {
            to_value(crate::commands::wakeup::wakeup_verification_load_history())
        }
        "wakeup_verification_delete_history" => to_value(
            crate::commands::wakeup::wakeup_verification_delete_history(arg(args, "batchIds")?),
        ),
        "wakeup_verification_run_batch" => to_value(
            crate::commands::wakeup::wakeup_verification_run_batch(
                app_handle()?,
                arg(args, "accountIds")?,
                arg(args, "model")?,
                opt_arg(args, "prompt")?,
                opt_arg(args, "maxOutputTokens")?,
                opt_arg(args, "officialLsVersionMode")?,
            )
            .await,
        ),
        "confirm_wakeup_task" => to_value(
            crate::commands::wakeup::confirm_wakeup_task(app_handle()?, arg(args, "taskId")?).await,
        ),
        "cancel_wakeup_task" => {
            to_value(crate::commands::wakeup::cancel_wakeup_task(arg(args, "taskId")?).await)
        }
        "check_wakeup_timeouts" => {
            to_value(crate::commands::wakeup::check_wakeup_timeouts(app_handle()?).await)
        }
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
