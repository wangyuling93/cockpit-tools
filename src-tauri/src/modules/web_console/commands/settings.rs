use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        "get_network_config" => to_value(crate::commands::system::get_network_config()),
        "save_network_config" => to_value(crate::commands::system::save_network_config(
            arg(args, "wsEnabled")?,
            arg(args, "wsPort")?,
            opt_arg(args, "reportEnabled")?,
            opt_arg(args, "reportPort")?,
            opt_arg(args, "reportToken")?,
            opt_arg(args, "globalProxyEnabled")?,
            opt_arg(args, "globalProxyUrl")?,
            opt_arg(args, "globalProxyNoProxy")?,
        )),
        "get_general_config" => {
            to_value(crate::commands::system::get_general_config(app_handle()?))
        }
        "save_general_config" => dispatch_save_general_config(args),
        "get_available_terminals" => {
            to_value(crate::commands::system::get_available_terminals().await)
        }
        "set_app_path" => to_value(crate::commands::system::set_app_path(
            arg(args, "app")?,
            arg(args, "path")?,
        )),
        "set_codex_launch_on_switch" => to_value(
            crate::commands::system::set_codex_launch_on_switch(arg(args, "enabled")?),
        ),
        "set_codex_local_access_entry_visible" => to_value(
            crate::commands::system::set_codex_local_access_entry_visible(arg(args, "enabled")?),
        ),
        "save_tray_platform_layout" => {
            to_value(crate::commands::system::save_tray_platform_layout(
                app_handle()?,
                arg(args, "sortMode")?,
                arg(args, "orderedPlatformIds")?,
                arg(args, "trayPlatformIds")?,
                opt_arg(args, "orderedEntryIds")?,
                opt_arg(args, "platformGroups")?,
            ))
        }
        "set_wakeup_override" => to_value(crate::commands::system::set_wakeup_override(arg(
            args, "enabled",
        )?)),
        "external_import_take_pending" => {
            serialize_value(crate::commands::system::external_import_take_pending())
        }
        "external_import_fetch_import_url" => to_value(
            crate::commands::system::external_import_fetch_import_url(arg(args, "importUrl")?)
                .await,
        ),
        "detect_app_path" => to_value(crate::commands::system::detect_app_path(
            arg(args, "app")?,
            opt_arg(args, "force")?,
        )),
        "get_antigravity_installed_version_info" => to_value(
            crate::commands::system::get_antigravity_installed_version_info(
                opt_arg(args, "runtimeTarget")?,
                opt_arg(args, "scanMode")?,
            )
            .await,
        ),
        "get_auto_backup_settings" => to_value(crate::commands::system::get_auto_backup_settings()),
        "save_auto_backup_settings" => {
            to_value(crate::commands::system::save_auto_backup_settings(
                arg(args, "enabled")?,
                arg(args, "includeAccounts")?,
                arg(args, "includeConfig")?,
                arg(args, "retentionDays")?,
            ))
        }
        "update_auto_backup_last_run" => to_value(
            crate::commands::system::update_auto_backup_last_run(opt_arg(args, "lastBackupAt")?),
        ),
        "write_auto_backup_file" => to_value(crate::commands::system::write_auto_backup_file(
            arg(args, "fileName")?,
            arg(args, "content")?,
        )),
        "read_auto_backup_file" => to_value(crate::commands::system::read_auto_backup_file(arg(
            args, "fileName",
        )?)),
        "copy_auto_backup_file" => to_value(crate::commands::system::copy_auto_backup_file(
            arg(args, "fileName")?,
            arg(args, "targetPath")?,
        )),
        "list_auto_backup_files" => to_value(crate::commands::system::list_auto_backup_files()),
        "delete_auto_backup_file" => to_value(crate::commands::system::delete_auto_backup_file(
            arg(args, "fileName")?,
        )),
        "cleanup_auto_backup_files" => to_value(
            crate::commands::system::cleanup_auto_backup_files(arg(args, "retentionDays")?),
        ),
        "open_auto_backup_dir" => to_value(crate::commands::system::open_auto_backup_dir()),
        "open_data_folder" => to_value(crate::commands::system::open_data_folder().await),
        "open_folder" => to_value(crate::commands::system::open_folder(arg(args, "path")?).await),
        "show_floating_card_window" => to_value(
            crate::commands::system::show_floating_card_window(app_handle()?),
        ),
        "show_instance_floating_card_window" => {
            to_value(crate::commands::system::show_instance_floating_card_window(
                app_handle()?,
                arg(args, "context")?,
            ))
        }
        "get_floating_card_context" => to_value(
            crate::commands::system::get_floating_card_context(arg(args, "windowLabel")?),
        ),
        "hide_floating_card_window" => to_value(
            crate::commands::system::hide_floating_card_window(app_handle()?),
        ),
        "hide_current_floating_card_window" => Ok(Value::Null),
        "set_floating_card_always_on_top" => {
            to_value(crate::commands::system::set_floating_card_always_on_top(
                app_handle()?,
                arg(args, "alwaysOnTop")?,
            ))
        }
        "set_current_floating_card_window_always_on_top" => Ok(Value::Null),
        "set_floating_card_confirm_on_close" => {
            to_value(crate::commands::system::set_floating_card_confirm_on_close(
                arg(args, "confirmOnClose")?,
            ))
        }
        "save_floating_card_position" => to_value(
            crate::commands::system::save_floating_card_position(arg(args, "x")?, arg(args, "y")?),
        ),
        "show_main_window_and_navigate" => {
            to_value(crate::commands::system::show_main_window_and_navigate(
                app_handle()?,
                arg(args, "page")?,
            ))
        }
        "logs_get_snapshot" => to_value(crate::commands::logs::logs_get_snapshot(
            opt_arg(args, "fileName")?,
            Some(arg_or(args, "lineLimit", 500usize)?),
        )),
        "logs_open_log_directory" => to_value(crate::commands::logs::logs_open_log_directory()),

        "get_update_settings" => to_value(crate::commands::update::get_update_settings()),
        "save_update_settings" => to_value(crate::commands::update::save_update_settings(arg(
            args, "settings",
        )?)),
        "should_check_updates" => to_value(crate::commands::update::should_check_updates()),
        "update_last_check_time" => to_value(crate::commands::update::update_last_check_time()),
        "check_version_jump" => to_value(crate::commands::update::check_version_jump()),
        "get_release_history" => to_value(crate::commands::update::get_release_history(
            opt_arg(args, "locale")?,
            opt_arg(args, "limit")?,
        )),
        "update_log" => to_value(crate::commands::update::update_log(
            arg(args, "level")?,
            arg(args, "message")?,
        )),
        "get_update_runtime_info" => to_value(crate::commands::update::get_update_runtime_info()),

        "announcement_get_state" => {
            to_value(crate::commands::announcement::announcement_get_state().await)
        }
        "announcement_mark_as_read" => to_value(
            crate::commands::announcement::announcement_mark_as_read(arg(args, "id")?).await,
        ),
        "announcement_mark_all_as_read" => {
            to_value(crate::commands::announcement::announcement_mark_all_as_read().await)
        }
        "announcement_force_refresh" => {
            to_value(crate::commands::announcement::announcement_force_refresh().await)
        }
        "announcement_get_top_right_ad" => {
            to_value(crate::commands::announcement::announcement_get_top_right_ad().await)
        }

        "get_group_settings" => to_value(crate::commands::group::get_group_settings()),
        "get_display_groups" => to_value(crate::commands::group::get_display_groups()),

        // system
        "save_text_file" => to_value(
            crate::commands::system::save_text_file(arg(args, "path")?, arg(args, "content")?)
                .await,
        ),
        "get_downloads_dir" => to_value(crate::commands::system::get_downloads_dir()),
        "handle_window_close" => Ok(Value::Null),
        "delete_corrupted_file" => {
            to_value(crate::commands::system::delete_corrupted_file(arg(args, "path")?).await)
        }
        // update
        "save_pending_update_notes" => {
            to_value(crate::commands::update::save_pending_update_notes(
                arg(args, "version")?,
                arg(args, "releaseNotes")?,
                arg(args, "releaseNotesZh")?,
            ))
        }
        "install_linux_update" => to_value(
            crate::commands::update::install_linux_update(
                app_handle()?,
                opt_arg(args, "expectedVersion")?,
            )
            .await,
        ),
        // group
        "save_group_settings" => to_value(crate::commands::group::save_group_settings(
            app_handle()?,
            arg(args, "groupMappings")?,
            arg(args, "groupNames")?,
            arg(args, "groupOrder")?,
        )),
        "set_model_group" => to_value(crate::commands::group::set_model_group(
            app_handle()?,
            arg(args, "modelId")?,
            arg(args, "groupId")?,
        )),
        "remove_model_group" => to_value(crate::commands::group::remove_model_group(
            app_handle()?,
            arg(args, "modelId")?,
        )),
        "set_group_name" => to_value(crate::commands::group::set_group_name(
            app_handle()?,
            arg(args, "groupId")?,
            arg(args, "name")?,
        )),
        "delete_group" => to_value(crate::commands::group::delete_group(
            app_handle()?,
            arg(args, "groupId")?,
        )),
        "update_group_order" => to_value(crate::commands::group::update_group_order(
            app_handle()?,
            arg(args, "order")?,
        )),
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}

fn dispatch_save_general_config(args: &Value) -> Result<Value, String> {
    to_value(crate::commands::system::save_general_config(
        app_handle()?,
        arg(args, "language")?,
        opt_arg(args, "defaultTerminal")?,
        arg(args, "theme")?,
        opt_arg(args, "uiScale")?,
        arg(args, "autoRefreshMinutes")?,
        arg(args, "codexAutoRefreshMinutes")?,
        opt_arg(args, "zedAutoRefreshMinutes")?,
        opt_arg(args, "ghcpAutoRefreshMinutes")?,
        opt_arg(args, "windsurfAutoRefreshMinutes")?,
        opt_arg(args, "kiroAutoRefreshMinutes")?,
        opt_arg(args, "cursorAutoRefreshMinutes")?,
        opt_arg(args, "geminiAutoRefreshMinutes")?,
        opt_arg(args, "geminiSyncWsl")?,
        opt_arg(args, "codebuddyAutoRefreshMinutes")?,
        opt_arg(args, "codebuddyCnAutoRefreshMinutes")?,
        opt_arg(args, "workbuddyAutoRefreshMinutes")?,
        opt_arg(args, "qoderAutoRefreshMinutes")?,
        opt_arg(args, "traeAutoRefreshMinutes")?,
        arg(args, "closeBehavior")?,
        opt_arg(args, "minimizeBehavior")?,
        opt_arg(args, "hideDockIcon")?,
        opt_arg(args, "trayIconStyle")?,
        opt_arg(args, "floatingCardShowOnStartup")?,
        opt_arg(args, "floatingCardAlwaysOnTop")?,
        opt_arg(args, "appAutoLaunchEnabled")?,
        opt_arg(args, "antigravityStartupWakeupEnabled")?,
        opt_arg(args, "antigravityStartupWakeupDelaySeconds")?,
        opt_arg(args, "codexStartupWakeupEnabled")?,
        opt_arg(args, "codexStartupWakeupDelaySeconds")?,
        opt_arg(args, "floatingCardConfirmOnClose")?,
        arg(args, "opencodeAppPath")?,
        arg(args, "antigravityAppPath")?,
        arg(args, "codexAppPath")?,
        opt_arg(args, "codexSpecifiedAppPath")?,
        opt_arg(args, "zedAppPath")?,
        arg(args, "vscodeAppPath")?,
        opt_arg(args, "windsurfAppPath")?,
        opt_arg(args, "kiroAppPath")?,
        opt_arg(args, "cursorAppPath")?,
        opt_arg(args, "codebuddyAppPath")?,
        opt_arg(args, "codebuddyCnAppPath")?,
        opt_arg(args, "qoderAppPath")?,
        opt_arg(args, "traeAppPath")?,
        opt_arg(args, "workbuddyAppPath")?,
        arg(args, "opencodeSyncOnSwitch")?,
        opt_arg(args, "opencodeAuthOverwriteOnSwitch")?,
        opt_arg(args, "ghcpOpencodeSyncOnSwitch")?,
        opt_arg(args, "ghcpOpencodeAuthOverwriteOnSwitch")?,
        opt_arg(args, "ghcpLaunchOnSwitch")?,
        opt_arg(args, "openclawAuthOverwriteOnSwitch")?,
        arg(args, "codexLaunchOnSwitch")?,
        opt_arg(args, "codexRestartSpecifiedAppOnSwitch")?,
        opt_arg(args, "codexLocalAccessEntryVisible")?,
        opt_arg(args, "antigravityDualSwitchNoRestartEnabled")?,
        opt_arg(args, "autoSwitchEnabled")?,
        opt_arg(args, "autoSwitchThreshold")?,
        opt_arg(args, "autoSwitchCreditsEnabled")?,
        opt_arg(args, "autoSwitchCreditsThreshold")?,
        opt_arg(args, "autoSwitchScopeMode")?,
        opt_arg(args, "autoSwitchSelectedGroupIds")?,
        opt_arg(args, "autoSwitchAccountScopeMode")?,
        opt_arg(args, "autoSwitchSelectedAccountIds")?,
        opt_arg(args, "codexAutoSwitchEnabled")?,
        opt_arg(args, "codexAutoSwitchPrimaryThreshold")?,
        opt_arg(args, "codexAutoSwitchSecondaryThreshold")?,
        opt_arg(args, "codexAutoSwitchAccountScopeMode")?,
        opt_arg(args, "codexAutoSwitchSelectedAccountIds")?,
        opt_arg(args, "quotaAlertEnabled")?,
        opt_arg(args, "quotaAlertThreshold")?,
        opt_arg(args, "codexQuotaAlertEnabled")?,
        opt_arg(args, "codexQuotaAlertThreshold")?,
        opt_arg(args, "zedQuotaAlertEnabled")?,
        opt_arg(args, "zedQuotaAlertThreshold")?,
        opt_arg(args, "codexQuotaAlertPrimaryThreshold")?,
        opt_arg(args, "codexQuotaAlertSecondaryThreshold")?,
        opt_arg(args, "ghcpQuotaAlertEnabled")?,
        opt_arg(args, "ghcpQuotaAlertThreshold")?,
        opt_arg(args, "windsurfQuotaAlertEnabled")?,
        opt_arg(args, "windsurfQuotaAlertThreshold")?,
        opt_arg(args, "kiroQuotaAlertEnabled")?,
        opt_arg(args, "kiroQuotaAlertThreshold")?,
        opt_arg(args, "cursorQuotaAlertEnabled")?,
        opt_arg(args, "cursorQuotaAlertThreshold")?,
        opt_arg(args, "geminiQuotaAlertEnabled")?,
        opt_arg(args, "geminiQuotaAlertThreshold")?,
        opt_arg(args, "codebuddyQuotaAlertEnabled")?,
        opt_arg(args, "codebuddyQuotaAlertThreshold")?,
        opt_arg(args, "codebuddyCnQuotaAlertEnabled")?,
        opt_arg(args, "codebuddyCnQuotaAlertThreshold")?,
        opt_arg(args, "qoderQuotaAlertEnabled")?,
        opt_arg(args, "qoderQuotaAlertThreshold")?,
        opt_arg(args, "traeQuotaAlertEnabled")?,
        opt_arg(args, "traeQuotaAlertThreshold")?,
        opt_arg(args, "workbuddyQuotaAlertEnabled")?,
        opt_arg(args, "workbuddyQuotaAlertThreshold")?,
    ))
}
