use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        "codex_get_instance_defaults" => {
            to_value(crate::commands::codex_instance::codex_get_instance_defaults().await)
        }
        "codex_list_instances" => {
            to_value(crate::commands::codex_instance::codex_list_instances().await)
        }
        "github_copilot_get_instance_defaults" => to_value(
            crate::commands::github_copilot_instance::github_copilot_get_instance_defaults().await,
        ),
        "github_copilot_list_instances" => to_value(
            crate::commands::github_copilot_instance::github_copilot_list_instances().await,
        ),
        "windsurf_get_instance_defaults" => {
            to_value(crate::commands::windsurf_instance::windsurf_get_instance_defaults().await)
        }
        "windsurf_list_instances" => {
            to_value(crate::commands::windsurf_instance::windsurf_list_instances().await)
        }
        "kiro_get_instance_defaults" => {
            to_value(crate::commands::kiro_instance::kiro_get_instance_defaults().await)
        }
        "kiro_list_instances" => {
            to_value(crate::commands::kiro_instance::kiro_list_instances().await)
        }
        "cursor_get_instance_defaults" => {
            to_value(crate::commands::cursor_instance::cursor_get_instance_defaults().await)
        }
        "cursor_list_instances" => {
            to_value(crate::commands::cursor_instance::cursor_list_instances().await)
        }
        "gemini_get_instance_defaults" => {
            to_value(crate::commands::gemini_instance::gemini_get_instance_defaults().await)
        }
        "gemini_list_instances" => {
            to_value(crate::commands::gemini_instance::gemini_list_instances().await)
        }
        "codebuddy_get_instance_defaults" => {
            to_value(crate::commands::codebuddy_instance::codebuddy_get_instance_defaults().await)
        }
        "codebuddy_list_instances" => {
            to_value(crate::commands::codebuddy_instance::codebuddy_list_instances().await)
        }
        "codebuddy_cn_get_instance_defaults" => to_value(
            crate::commands::codebuddy_cn_instance::codebuddy_cn_get_instance_defaults().await,
        ),
        "codebuddy_cn_list_instances" => {
            to_value(crate::commands::codebuddy_cn_instance::codebuddy_cn_list_instances().await)
        }
        "qoder_get_instance_defaults" => {
            to_value(crate::commands::qoder_instance::qoder_get_instance_defaults().await)
        }
        "qoder_list_instances" => {
            to_value(crate::commands::qoder_instance::qoder_list_instances().await)
        }
        "trae_get_instance_defaults" => {
            to_value(crate::commands::trae_instance::trae_get_instance_defaults().await)
        }
        "trae_list_instances" => {
            to_value(crate::commands::trae_instance::trae_list_instances().await)
        }
        "workbuddy_get_instance_defaults" => {
            to_value(crate::commands::workbuddy_instance::workbuddy_get_instance_defaults().await)
        }
        "workbuddy_list_instances" => {
            to_value(crate::commands::workbuddy_instance::workbuddy_list_instances().await)
        }

        // github_copilot_instance
        "github_copilot_create_instance" => to_value(
            crate::commands::github_copilot_instance::github_copilot_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "github_copilot_update_instance" => to_value(
            crate::commands::github_copilot_instance::github_copilot_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "github_copilot_delete_instance" => to_value(
            crate::commands::github_copilot_instance::github_copilot_delete_instance(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "github_copilot_start_instance" => to_value(
            crate::commands::github_copilot_instance::github_copilot_start_instance(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "github_copilot_stop_instance" => to_value(
            crate::commands::github_copilot_instance::github_copilot_stop_instance(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "github_copilot_open_instance_window" => to_value(
            crate::commands::github_copilot_instance::github_copilot_open_instance_window(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "github_copilot_close_all_instances" => to_value(
            crate::commands::github_copilot_instance::github_copilot_close_all_instances().await,
        ),

        // workbuddy_instance
        "workbuddy_create_instance" => to_value(
            crate::commands::workbuddy_instance::workbuddy_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "workbuddy_update_instance" => to_value(
            crate::commands::workbuddy_instance::workbuddy_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "workbuddy_delete_instance" => to_value(
            crate::commands::workbuddy_instance::workbuddy_delete_instance(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "workbuddy_start_instance" => to_value(
            crate::commands::workbuddy_instance::workbuddy_start_instance(arg(args, "instanceId")?)
                .await,
        ),
        "workbuddy_stop_instance" => to_value(
            crate::commands::workbuddy_instance::workbuddy_stop_instance(arg(args, "instanceId")?)
                .await,
        ),
        "workbuddy_open_instance_window" => to_value(
            crate::commands::workbuddy_instance::workbuddy_open_instance_window(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "workbuddy_close_all_instances" => {
            to_value(crate::commands::workbuddy_instance::workbuddy_close_all_instances().await)
        }
        // codebuddy_instance
        "codebuddy_create_instance" => to_value(
            crate::commands::codebuddy_instance::codebuddy_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "codebuddy_update_instance" => to_value(
            crate::commands::codebuddy_instance::codebuddy_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "codebuddy_delete_instance" => to_value(
            crate::commands::codebuddy_instance::codebuddy_delete_instance(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "codebuddy_start_instance" => to_value(
            crate::commands::codebuddy_instance::codebuddy_start_instance(arg(args, "instanceId")?)
                .await,
        ),
        "codebuddy_stop_instance" => to_value(
            crate::commands::codebuddy_instance::codebuddy_stop_instance(arg(args, "instanceId")?)
                .await,
        ),
        "codebuddy_open_instance_window" => to_value(
            crate::commands::codebuddy_instance::codebuddy_open_instance_window(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "codebuddy_close_all_instances" => {
            to_value(crate::commands::codebuddy_instance::codebuddy_close_all_instances().await)
        }
        // codebuddy_cn_instance
        "codebuddy_cn_create_instance" => to_value(
            crate::commands::codebuddy_cn_instance::codebuddy_cn_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "codebuddy_cn_update_instance" => to_value(
            crate::commands::codebuddy_cn_instance::codebuddy_cn_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "codebuddy_cn_delete_instance" => to_value(
            crate::commands::codebuddy_cn_instance::codebuddy_cn_delete_instance(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "codebuddy_cn_start_instance" => to_value(
            crate::commands::codebuddy_cn_instance::codebuddy_cn_start_instance(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "codebuddy_cn_stop_instance" => to_value(
            crate::commands::codebuddy_cn_instance::codebuddy_cn_stop_instance(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "codebuddy_cn_open_instance_window" => to_value(
            crate::commands::codebuddy_cn_instance::codebuddy_cn_open_instance_window(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "codebuddy_cn_close_all_instances" => to_value(
            crate::commands::codebuddy_cn_instance::codebuddy_cn_close_all_instances().await,
        ),

        // qoder_instance
        "qoder_create_instance" => to_value(
            crate::commands::qoder_instance::qoder_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "qoder_update_instance" => to_value(
            crate::commands::qoder_instance::qoder_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "qoder_delete_instance" => to_value(
            crate::commands::qoder_instance::qoder_delete_instance(arg(args, "instanceId")?).await,
        ),
        "qoder_start_instance" => to_value(
            crate::commands::qoder_instance::qoder_start_instance(arg(args, "instanceId")?).await,
        ),
        "qoder_stop_instance" => to_value(
            crate::commands::qoder_instance::qoder_stop_instance(arg(args, "instanceId")?).await,
        ),
        "qoder_open_instance_window" => to_value(
            crate::commands::qoder_instance::qoder_open_instance_window(arg(args, "instanceId")?)
                .await,
        ),
        "qoder_close_all_instances" => {
            to_value(crate::commands::qoder_instance::qoder_close_all_instances().await)
        }

        // trae_instance
        "trae_create_instance" => to_value(
            crate::commands::trae_instance::trae_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "trae_update_instance" => to_value(
            crate::commands::trae_instance::trae_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "trae_delete_instance" => to_value(
            crate::commands::trae_instance::trae_delete_instance(arg(args, "instanceId")?).await,
        ),
        "trae_start_instance" => to_value(
            crate::commands::trae_instance::trae_start_instance(arg(args, "instanceId")?).await,
        ),
        "trae_stop_instance" => to_value(
            crate::commands::trae_instance::trae_stop_instance(arg(args, "instanceId")?).await,
        ),
        "trae_open_instance_window" => to_value(
            crate::commands::trae_instance::trae_open_instance_window(arg(args, "instanceId")?)
                .await,
        ),
        "trae_close_all_instances" => {
            to_value(crate::commands::trae_instance::trae_close_all_instances().await)
        }

        // gemini_instance
        "gemini_create_instance" => to_value(
            crate::commands::gemini_instance::gemini_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "workingDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "gemini_update_instance" => to_value(
            crate::commands::gemini_instance::gemini_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "workingDir")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "gemini_delete_instance" => to_value(
            crate::commands::gemini_instance::gemini_delete_instance(arg(args, "instanceId")?)
                .await,
        ),
        "gemini_start_instance" => to_value(
            crate::commands::gemini_instance::gemini_start_instance(arg(args, "instanceId")?).await,
        ),
        "gemini_stop_instance" => to_value(
            crate::commands::gemini_instance::gemini_stop_instance(arg(args, "instanceId")?).await,
        ),
        "gemini_open_instance_window" => to_value(
            crate::commands::gemini_instance::gemini_open_instance_window(arg(args, "instanceId")?)
                .await,
        ),
        "gemini_close_all_instances" => {
            to_value(crate::commands::gemini_instance::gemini_close_all_instances().await)
        }
        "gemini_get_instance_launch_command" => to_value(
            crate::commands::gemini_instance::gemini_get_instance_launch_command(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "gemini_execute_instance_launch_command" => to_value(
            crate::commands::gemini_instance::gemini_execute_instance_launch_command(
                arg(args, "instanceId")?,
                opt_arg(args, "terminal")?,
            )
            .await,
        ),
        // cursor_instance
        "cursor_create_instance" => to_value(
            crate::commands::cursor_instance::cursor_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "cursor_update_instance" => to_value(
            crate::commands::cursor_instance::cursor_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "cursor_delete_instance" => to_value(
            crate::commands::cursor_instance::cursor_delete_instance(arg(args, "instanceId")?)
                .await,
        ),
        "cursor_start_instance" => to_value(
            crate::commands::cursor_instance::cursor_start_instance(arg(args, "instanceId")?).await,
        ),
        "cursor_stop_instance" => to_value(
            crate::commands::cursor_instance::cursor_stop_instance(arg(args, "instanceId")?).await,
        ),
        "cursor_open_instance_window" => to_value(
            crate::commands::cursor_instance::cursor_open_instance_window(arg(args, "instanceId")?)
                .await,
        ),
        "cursor_close_all_instances" => {
            to_value(crate::commands::cursor_instance::cursor_close_all_instances().await)
        }
        // windsurf_instance
        "windsurf_create_instance" => to_value(
            crate::commands::windsurf_instance::windsurf_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "windsurf_update_instance" => to_value(
            crate::commands::windsurf_instance::windsurf_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "windsurf_delete_instance" => to_value(
            crate::commands::windsurf_instance::windsurf_delete_instance(arg(args, "instanceId")?)
                .await,
        ),
        "windsurf_start_instance" => to_value(
            crate::commands::windsurf_instance::windsurf_start_instance(arg(args, "instanceId")?)
                .await,
        ),
        "windsurf_stop_instance" => to_value(
            crate::commands::windsurf_instance::windsurf_stop_instance(arg(args, "instanceId")?)
                .await,
        ),
        "windsurf_open_instance_window" => to_value(
            crate::commands::windsurf_instance::windsurf_open_instance_window(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "windsurf_close_all_instances" => {
            to_value(crate::commands::windsurf_instance::windsurf_close_all_instances().await)
        }
        // kiro_instance
        "kiro_create_instance" => to_value(
            crate::commands::kiro_instance::kiro_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "kiro_update_instance" => to_value(
            crate::commands::kiro_instance::kiro_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "kiro_delete_instance" => to_value(
            crate::commands::kiro_instance::kiro_delete_instance(arg(args, "instanceId")?).await,
        ),
        "kiro_start_instance" => to_value(
            crate::commands::kiro_instance::kiro_start_instance(arg(args, "instanceId")?).await,
        ),
        "kiro_stop_instance" => to_value(
            crate::commands::kiro_instance::kiro_stop_instance(arg(args, "instanceId")?).await,
        ),
        "kiro_open_instance_window" => to_value(
            crate::commands::kiro_instance::kiro_open_instance_window(arg(args, "instanceId")?)
                .await,
        ),
        "kiro_close_all_instances" => {
            to_value(crate::commands::kiro_instance::kiro_close_all_instances().await)
        }
        // codex_instance
        "codex_get_instance_quick_config" => to_value(
            crate::commands::codex_instance::codex_get_instance_quick_config(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "codex_save_instance_quick_config" => to_value(
            crate::commands::codex_instance::codex_save_instance_quick_config(
                arg(args, "instanceId")?,
                opt_arg(args, "modelContextWindow")?,
                opt_arg(args, "autoCompactTokenLimit")?,
            )
            .await,
        ),
        "codex_open_instance_config_toml" => to_value(
            crate::commands::codex_instance::codex_open_instance_config_toml(
                app_handle()?,
                arg(args, "instanceId")?,
            )
            .await,
        ),
        "codex_sync_threads_across_instances" => {
            to_value(crate::commands::codex_instance::codex_sync_threads_across_instances().await)
        }
        "codex_sync_sessions_to_instance" => to_value(
            crate::commands::codex_instance::codex_sync_sessions_to_instance(
                arg(args, "sessionIds")?,
                arg(args, "targetInstanceId")?,
            )
            .await,
        ),
        "codex_repair_session_visibility_across_instances" => to_value(
            crate::commands::codex_instance::codex_repair_session_visibility_across_instances()
                .await,
        ),
        "codex_list_sessions_across_instances" => {
            to_value(crate::commands::codex_instance::codex_list_sessions_across_instances().await)
        }
        "codex_get_session_token_stats_across_instances" => to_value(
            crate::commands::codex_instance::codex_get_session_token_stats_across_instances(arg(
                args,
                "sessionIds",
            )?)
            .await,
        ),
        "codex_move_sessions_to_trash_across_instances" => to_value(
            crate::commands::codex_instance::codex_move_sessions_to_trash_across_instances(arg(
                args,
                "sessionIds",
            )?)
            .await,
        ),
        "codex_list_trashed_sessions_across_instances" => to_value(
            crate::commands::codex_instance::codex_list_trashed_sessions_across_instances().await,
        ),
        "codex_restore_sessions_from_trash_across_instances" => to_value(
            crate::commands::codex_instance::codex_restore_sessions_from_trash_across_instances(
                arg(args, "sessionIds")?,
            )
            .await,
        ),
        "codex_create_instance" => to_value(
            crate::commands::codex_instance::codex_create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "workingDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
                opt_arg(args, "launchMode")?,
                opt_arg(args, "appSpeed")?,
            )
            .await,
        ),
        "codex_update_instance" => to_value(
            crate::commands::codex_instance::codex_update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "workingDir")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
                opt_arg(args, "launchMode")?,
                opt_arg(args, "appSpeed")?,
                opt_arg(args, "autoSyncThreads")?,
            )
            .await,
        ),
        "codex_delete_instance" => to_value(
            crate::commands::codex_instance::codex_delete_instance(arg(args, "instanceId")?).await,
        ),
        "codex_start_instance" => to_value(
            crate::commands::codex_instance::codex_start_instance(arg(args, "instanceId")?).await,
        ),
        "codex_stop_instance" => to_value(
            crate::commands::codex_instance::codex_stop_instance(arg(args, "instanceId")?).await,
        ),
        "codex_open_instance_window" => to_value(
            crate::commands::codex_instance::codex_open_instance_window(arg(args, "instanceId")?)
                .await,
        ),
        "codex_close_all_instances" => {
            to_value(crate::commands::codex_instance::codex_close_all_instances().await)
        }
        "codex_get_instance_launch_command" => to_value(
            crate::commands::codex_instance::codex_get_instance_launch_command(arg(
                args,
                "instanceId",
            )?)
            .await,
        ),
        "codex_execute_instance_launch_command" => to_value(
            crate::commands::codex_instance::codex_execute_instance_launch_command(
                arg(args, "instanceId")?,
                opt_arg(args, "terminal")?,
            )
            .await,
        ),
        // instance
        "get_instance_defaults" => {
            to_value(crate::commands::instance::get_instance_defaults().await)
        }
        "list_instances" => to_value(crate::commands::instance::list_instances().await),
        "create_instance" => to_value(
            crate::commands::instance::create_instance(
                arg(args, "name")?,
                arg(args, "userDataDir")?,
                opt_arg(args, "extraArgs")?,
                opt_arg(args, "bindAccountId")?,
                opt_arg(args, "copySourceInstanceId")?,
                opt_arg(args, "initMode")?,
            )
            .await,
        ),
        "update_instance" => to_value(
            crate::commands::instance::update_instance(
                arg(args, "instanceId")?,
                opt_arg(args, "name")?,
                opt_arg(args, "extraArgs")?,
                opt_nullable_arg(args, "bindAccountId")?,
                opt_arg(args, "followLocalAccount")?,
            )
            .await,
        ),
        "delete_instance" => {
            to_value(crate::commands::instance::delete_instance(arg(args, "instanceId")?).await)
        }
        "start_instance" => {
            to_value(crate::commands::instance::start_instance(arg(args, "instanceId")?).await)
        }
        "stop_instance" => {
            to_value(crate::commands::instance::stop_instance(arg(args, "instanceId")?).await)
        }
        "open_instance_window" => to_value(
            crate::commands::instance::open_instance_window(arg(args, "instanceId")?).await,
        ),
        "close_all_instances" => to_value(crate::commands::instance::close_all_instances().await),
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
