use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        "list_codex_accounts" => to_value(crate::commands::codex::list_codex_accounts()),
        "get_current_codex_account" => {
            to_value(crate::commands::codex::get_current_codex_account())
        }
        "refresh_current_codex_quota" => {
            to_value(crate::commands::codex::refresh_current_codex_quota(app_handle()?).await)
        }
        "refresh_codex_quota" => to_value(
            crate::commands::codex::refresh_codex_quota(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "refresh_codex_subscription_info" => to_value(
            crate::commands::codex::refresh_codex_subscription_info(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "refresh_codex_account_profile" => to_value(
            crate::commands::codex::refresh_codex_account_profile(arg(args, "accountId")?).await,
        ),
        "refresh_all_codex_quotas" => {
            to_value(crate::commands::codex::refresh_all_codex_quotas(app_handle()?).await)
        }
        "switch_codex_account" => to_value(
            crate::commands::codex::switch_codex_account(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "load_codex_account_groups" => {
            to_value(crate::commands::codex::load_codex_account_groups().await)
        }
        "save_codex_account_groups" => {
            to_value(crate::commands::codex::save_codex_account_groups(arg(args, "data")?).await)
        }
        "get_codex_quick_config" => to_value(crate::commands::codex::get_codex_quick_config()),
        "save_codex_quick_config" => to_value(crate::commands::codex::save_codex_quick_config(
            opt_arg(args, "modelContextWindow")?,
            opt_arg(args, "autoCompactTokenLimit")?,
        )),
        "get_codex_app_speed_config" => {
            to_value(crate::commands::codex::get_codex_app_speed_config())
        }
        "save_codex_app_speed" => to_value(crate::commands::codex::save_codex_app_speed(arg(
            args, "speed",
        )?)),
        "get_codex_api_service_app_speed_config" => {
            to_value(crate::commands::codex::get_codex_api_service_app_speed_config())
        }
        "save_codex_api_service_app_speed" => to_value(
            crate::commands::codex::save_codex_api_service_app_speed(arg(args, "speed")?),
        ),
        "codex_local_access_get_state" => {
            to_value(crate::commands::codex::codex_local_access_get_state().await)
        }

        "codex_wakeup_get_cli_status" => {
            to_value(crate::commands::codex::codex_wakeup_get_cli_status())
        }
        "codex_wakeup_update_runtime_config" => {
            to_value(crate::commands::codex::codex_wakeup_update_runtime_config(
                opt_arg(args, "codexCliPath")?,
                opt_arg(args, "nodePath")?,
            ))
        }
        "codex_wakeup_get_overview" => {
            to_value(crate::commands::codex::codex_wakeup_get_overview())
        }
        "codex_wakeup_get_state" => to_value(crate::commands::codex::codex_wakeup_get_state()),
        "codex_wakeup_save_state" => to_value(crate::commands::codex::codex_wakeup_save_state(
            arg(args, "enabled")?,
            arg(args, "tasks")?,
            arg(args, "modelPresets")?,
            arg(args, "modelPresetMigrations")?,
        )),
        "codex_wakeup_load_history" => {
            to_value(crate::commands::codex::codex_wakeup_load_history())
        }
        "codex_wakeup_clear_history" => {
            to_value(crate::commands::codex::codex_wakeup_clear_history())
        }
        "codex_wakeup_cancel_scope" => to_value(crate::commands::codex::codex_wakeup_cancel_scope(
            arg(args, "cancelScopeId")?,
        )),
        "codex_wakeup_release_scope" => to_value(
            crate::commands::codex::codex_wakeup_release_scope(arg(args, "cancelScopeId")?),
        ),
        "codex_wakeup_test" => to_value(
            crate::commands::codex::codex_wakeup_test(
                app_handle()?,
                arg(args, "accountIds")?,
                opt_arg(args, "prompt")?,
                opt_arg(args, "model")?,
                opt_arg(args, "modelDisplayName")?,
                opt_arg(args, "modelReasoningEffort")?,
                opt_arg(args, "runId")?,
                opt_arg(args, "cancelScopeId")?,
            )
            .await,
        ),
        "codex_wakeup_run_task" => to_value(
            crate::commands::codex::codex_wakeup_run_task(
                app_handle()?,
                arg(args, "taskId")?,
                opt_arg(args, "runId")?,
            )
            .await,
        ),
        "codex_wakeup_run_enabled_tasks" => to_value(
            crate::commands::codex::codex_wakeup_run_enabled_tasks(
                app_handle()?,
                opt_arg(args, "triggerType")?,
            )
            .await,
        ),

        // codex
        "get_codex_config_toml_path" => {
            to_value(crate::commands::codex::get_codex_config_toml_path())
        }
        "open_codex_config_toml" => {
            to_value(crate::commands::codex::open_codex_config_toml(app_handle()?))
        }
        "update_codex_account_app_speed" => {
            to_value(crate::commands::codex::update_codex_account_app_speed(
                arg(args, "accountId")?,
                arg(args, "speed")?,
            ))
        }
        "delete_codex_account" => to_value(crate::commands::codex::delete_codex_account(arg(
            args,
            "accountId",
        )?)),
        "delete_codex_accounts" => to_value(crate::commands::codex::delete_codex_accounts(arg(
            args,
            "accountIds",
        )?)),
        "import_codex_from_local" => {
            to_value(crate::commands::codex::import_codex_from_local(app_handle()?).await)
        }
        "import_codex_from_json" => to_value(
            crate::commands::codex::import_codex_from_json(
                app_handle()?,
                arg(args, "jsonContent")?,
            )
            .await,
        ),
        "export_codex_accounts" => to_value(crate::commands::codex::export_codex_accounts(arg(
            args,
            "accountIds",
        )?)),
        "import_codex_from_files" => to_value(
            crate::commands::codex::import_codex_from_files(app_handle()?, arg(args, "filePaths")?)
                .await,
        ),
        "codex_oauth_login_start" => {
            to_value(crate::commands::codex::codex_oauth_login_start(app_handle()?).await)
        }
        "codex_oauth_login_completed" => to_value(
            crate::commands::codex::codex_oauth_login_completed(arg(args, "loginId")?).await,
        ),
        "codex_oauth_submit_callback_url" => {
            to_value(crate::commands::codex::codex_oauth_submit_callback_url(
                app_handle()?,
                arg(args, "loginId")?,
                arg(args, "callbackUrl")?,
            ))
        }
        "codex_oauth_login_cancel" => to_value(crate::commands::codex::codex_oauth_login_cancel(
            opt_arg(args, "loginId")?,
        )),
        "add_codex_account_with_token" => to_value(
            crate::commands::codex::add_codex_account_with_token(
                arg(args, "idToken")?,
                arg(args, "accessToken")?,
                opt_arg(args, "refreshToken")?,
            )
            .await,
        ),
        "add_codex_account_with_api_key" => {
            to_value(crate::commands::codex::add_codex_account_with_api_key(
                arg(args, "apiKey")?,
                opt_arg(args, "apiBaseUrl")?,
                opt_arg(args, "apiProviderMode")?,
                opt_arg(args, "apiProviderId")?,
                opt_arg(args, "apiProviderName")?,
            ))
        }
        "update_codex_account_name" => to_value(crate::commands::codex::update_codex_account_name(
            arg(args, "accountId")?,
            arg(args, "name")?,
        )),
        "update_codex_api_key_credentials" => {
            to_value(crate::commands::codex::update_codex_api_key_credentials(
                arg(args, "accountId")?,
                arg(args, "apiKey")?,
                opt_arg(args, "apiBaseUrl")?,
                opt_arg(args, "apiProviderMode")?,
                opt_arg(args, "apiProviderId")?,
                opt_arg(args, "apiProviderName")?,
            ))
        }
        "update_codex_api_key_bound_oauth_account" => to_value(
            crate::commands::codex::update_codex_api_key_bound_oauth_account(
                arg(args, "accountId")?,
                opt_arg(args, "boundOauthAccountId")?,
            )
            .await,
        ),
        "is_codex_oauth_port_in_use" => {
            to_value(crate::commands::codex::is_codex_oauth_port_in_use())
        }
        "close_codex_oauth_port" => to_value(crate::commands::codex::close_codex_oauth_port()),
        "update_codex_account_tags" => to_value(
            crate::commands::codex::update_codex_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "update_codex_account_note" => to_value(
            crate::commands::codex::update_codex_account_note(
                arg(args, "accountId")?,
                arg(args, "note")?,
            )
            .await,
        ),
        "load_codex_model_providers" => {
            to_value(crate::commands::codex::load_codex_model_providers().await)
        }
        "save_codex_model_providers" => {
            to_value(crate::commands::codex::save_codex_model_providers(arg(args, "data")?).await)
        }
        "codex_local_access_save_accounts" => to_value(
            crate::commands::codex::codex_local_access_save_accounts(
                arg(args, "accountIds")?,
                opt_arg(args, "restrictFreeAccounts")?,
            )
            .await,
        ),
        "codex_local_access_remove_account" => to_value(
            crate::commands::codex::codex_local_access_remove_account(arg(args, "accountId")?)
                .await,
        ),
        "codex_local_access_rotate_api_key" => {
            to_value(crate::commands::codex::codex_local_access_rotate_api_key().await)
        }
        "codex_local_access_update_bound_oauth_account" => to_value(
            crate::commands::codex::codex_local_access_update_bound_oauth_account(opt_arg(
                args,
                "boundOauthAccountId",
            )?)
            .await,
        ),
        "codex_local_access_clear_stats" => {
            to_value(crate::commands::codex::codex_local_access_clear_stats().await)
        }
        "codex_local_access_query_request_logs" => to_value(
            crate::commands::codex::codex_local_access_query_request_logs(
                arg(args, "page")?,
                arg(args, "pageSize")?,
                opt_arg(args, "statsRange")?,
                opt_arg(args, "modelQuery")?,
                opt_arg(args, "accountQuery")?,
                opt_arg(args, "apiKeyQuery")?,
                opt_arg(args, "gatewayMode")?,
                opt_arg(args, "requestKind")?,
                opt_arg(args, "success")?,
                opt_arg(args, "errorCategory")?,
            )
            .await,
        ),
        "codex_local_access_prepare_restart" => {
            to_value(crate::commands::codex::codex_local_access_prepare_restart().await)
        }
        "codex_local_access_kill_port" => {
            to_value(crate::commands::codex::codex_local_access_kill_port().await)
        }
        "codex_local_access_update_port" => to_value(
            crate::commands::codex::codex_local_access_update_port(arg(args, "port")?).await,
        ),
        "codex_local_access_update_routing_strategy" => to_value(
            crate::commands::codex::codex_local_access_update_routing_strategy(arg(
                args, "strategy",
            )?)
            .await,
        ),
        "codex_local_access_update_custom_routing" => to_value(
            crate::commands::codex::codex_local_access_update_custom_routing(arg(args, "rules")?)
                .await,
        ),
        "codex_local_access_update_account_model_rules" => to_value(
            crate::commands::codex::codex_local_access_update_account_model_rules(arg(
                args, "rules",
            )?)
            .await,
        ),
        "codex_local_access_update_model_rules" => to_value(
            crate::commands::codex::codex_local_access_update_model_rules(
                arg(args, "modelAliases")?,
                arg(args, "excludedModels")?,
            )
            .await,
        ),
        "codex_local_access_update_model_pricings" => to_value(
            crate::commands::codex::codex_local_access_update_model_pricings(arg(
                args,
                "modelPricings",
            )?)
            .await,
        ),
        "codex_local_access_update_routing_options" => to_value(
            crate::commands::codex::codex_local_access_update_routing_options(
                arg(args, "sessionAffinity")?,
                arg(args, "sessionAffinityTtlMs")?,
                arg(args, "maxRetryCredentials")?,
                arg(args, "maxRetryIntervalMs")?,
                arg(args, "disableCooling")?,
            )
            .await,
        ),
        "codex_local_access_update_timeouts" => to_value(
            crate::commands::codex::codex_local_access_update_timeouts(
                arg(args, "timeouts")?,
                opt_arg(args, "activeTimeoutPresetId")?,
            )
            .await,
        ),
        "codex_local_access_update_timeout_presets" => to_value(
            crate::commands::codex::codex_local_access_update_timeout_presets(
                arg(args, "timeoutPresets")?,
                opt_arg(args, "activeTimeoutPresetId")?,
            )
            .await,
        ),
        "codex_local_access_update_upstream_proxy_config" => to_value(
            crate::commands::codex::codex_local_access_update_upstream_proxy_config(opt_arg(
                args,
                "upstreamProxyUrl",
            )?)
            .await,
        ),
        "codex_local_access_update_gateway_mode" => to_value(
            crate::commands::codex::codex_local_access_update_gateway_mode(arg(
                args,
                "gatewayMode",
            )?)
            .await,
        ),
        "codex_local_access_update_debug_logs" => to_value(
            crate::commands::codex::codex_local_access_update_debug_logs(arg(args, "debugLogs")?)
                .await,
        ),
        "codex_local_access_update_access_scope" => to_value(
            crate::commands::codex::codex_local_access_update_access_scope(arg(
                args,
                "accessScope",
            )?)
            .await,
        ),
        "codex_local_access_update_client_base_url_host" => to_value(
            crate::commands::codex::codex_local_access_update_client_base_url_host(arg(
                args,
                "clientBaseUrlHost",
            )?)
            .await,
        ),
        "codex_local_access_update_image_generation_mode" => to_value(
            crate::commands::codex::codex_local_access_update_image_generation_mode(arg(
                args,
                "imageGenerationMode",
            )?)
            .await,
        ),
        "codex_local_access_create_api_key" => to_value(
            crate::commands::codex::codex_local_access_create_api_key(opt_arg(args, "label")?)
                .await,
        ),
        "codex_local_access_update_api_key" => to_value(
            crate::commands::codex::codex_local_access_update_api_key(
                arg(args, "apiKeyId")?,
                opt_arg(args, "label")?,
                opt_arg(args, "enabled")?,
                opt_arg(args, "modelPrefix")?,
                opt_arg(args, "allowedModels")?,
                opt_arg(args, "excludedModels")?,
            )
            .await,
        ),
        "codex_local_access_rotate_named_api_key" => to_value(
            crate::commands::codex::codex_local_access_rotate_named_api_key(arg(args, "apiKeyId")?)
                .await,
        ),
        "codex_local_access_delete_api_key" => to_value(
            crate::commands::codex::codex_local_access_delete_api_key(arg(args, "apiKeyId")?).await,
        ),
        "codex_local_access_set_enabled" => to_value(
            crate::commands::codex::codex_local_access_set_enabled(arg(args, "enabled")?).await,
        ),
        "codex_local_access_activate" => {
            to_value(crate::commands::codex::codex_local_access_activate(app_handle()?).await)
        }
        "codex_local_access_test" => {
            to_value(crate::commands::codex::codex_local_access_test().await)
        }
        "codex_local_access_chat_test" => to_value(
            crate::commands::codex::codex_local_access_chat_test(
                arg(args, "modelId")?,
                arg(args, "messages")?,
            )
            .await,
        ),
        "codex_local_access_chat_test_stream" => to_value(
            crate::commands::codex::codex_local_access_chat_test_stream(
                app_handle()?,
                arg(args, "sessionId")?,
                arg(args, "modelId")?,
                arg(args, "messages")?,
            )
            .await,
        ),
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
