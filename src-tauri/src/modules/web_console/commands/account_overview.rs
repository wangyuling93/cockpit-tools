use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        "list_accounts" => to_value(crate::commands::account::list_accounts().await),
        "get_current_account" => to_value(crate::commands::account::get_current_account().await),
        "set_current_account" => to_value(
            crate::commands::account::set_current_account(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "fetch_account_quota" => to_value(
            crate::commands::account::fetch_account_quota(arg(args, "accountId")?)
                .await
                .map_err(|err| err.to_string()),
        ),
        "refresh_all_quotas" => {
            to_value(crate::commands::account::refresh_all_quotas(app_handle()?).await)
        }
        "refresh_current_quota" => {
            to_value(crate::commands::account::refresh_current_quota(app_handle()?).await)
        }
        "switch_account" => to_value(
            crate::commands::account::switch_account(
                app_handle()?,
                arg(args, "accountId")?,
                opt_arg(args, "runtimeTarget")?,
            )
            .await,
        ),
        "update_account_tags" => to_value(
            crate::commands::account::update_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "update_account_notes" => to_value(
            crate::commands::account::update_account_notes(
                arg(args, "accountId")?,
                arg(args, "notes")?,
            )
            .await,
        ),
        "load_account_groups" => to_value(crate::commands::account::load_account_groups().await),
        "save_account_groups" => {
            to_value(crate::commands::account::save_account_groups(arg(args, "data")?).await)
        }

        "list_github_copilot_accounts" => {
            to_value(crate::commands::github_copilot::list_github_copilot_accounts())
        }
        "refresh_github_copilot_token" => to_value(
            crate::commands::github_copilot::refresh_github_copilot_token(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "refresh_all_github_copilot_tokens" => to_value(
            crate::commands::github_copilot::refresh_all_github_copilot_tokens(app_handle()?).await,
        ),
        "inject_github_copilot_to_vscode" => to_value(
            crate::commands::github_copilot::inject_github_copilot_to_vscode(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "update_github_copilot_account_tags" => to_value(
            crate::commands::github_copilot::update_github_copilot_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "list_windsurf_accounts" => to_value(crate::commands::windsurf::list_windsurf_accounts()),
        "refresh_windsurf_token" => to_value(
            crate::commands::windsurf::refresh_windsurf_token(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "refresh_all_windsurf_tokens" => {
            to_value(crate::commands::windsurf::refresh_all_windsurf_tokens(app_handle()?).await)
        }
        "inject_windsurf_to_vscode" => to_value(
            crate::commands::windsurf::inject_windsurf_to_vscode(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "update_windsurf_account_tags" => to_value(
            crate::commands::windsurf::update_windsurf_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "list_kiro_accounts" => to_value(crate::commands::kiro::list_kiro_accounts()),
        "refresh_kiro_token" => to_value(
            crate::commands::kiro::refresh_kiro_token(app_handle()?, arg(args, "accountId")?).await,
        ),
        "refresh_all_kiro_tokens" => {
            to_value(crate::commands::kiro::refresh_all_kiro_tokens(app_handle()?).await)
        }
        "inject_kiro_to_vscode" => to_value(
            crate::commands::kiro::inject_kiro_to_vscode(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "update_kiro_account_tags" => to_value(
            crate::commands::kiro::update_kiro_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "list_cursor_accounts" => to_value(crate::commands::cursor::list_cursor_accounts()),
        "refresh_cursor_token" => to_value(
            crate::commands::cursor::refresh_cursor_token(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "refresh_all_cursor_tokens" => {
            to_value(crate::commands::cursor::refresh_all_cursor_tokens(app_handle()?).await)
        }
        "inject_cursor_account" => to_value(
            crate::commands::cursor::inject_cursor_account(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "update_cursor_account_tags" => to_value(
            crate::commands::cursor::update_cursor_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "list_gemini_accounts" => to_value(crate::commands::gemini::list_gemini_accounts()),
        "refresh_gemini_token" => to_value(
            crate::commands::gemini::refresh_gemini_token(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "refresh_all_gemini_tokens" => {
            to_value(crate::commands::gemini::refresh_all_gemini_tokens(app_handle()?).await)
        }
        "inject_gemini_account" => to_value(crate::commands::gemini::inject_gemini_account(
            app_handle()?,
            arg(args, "accountId")?,
        )),
        "update_gemini_account_tags" => {
            to_value(crate::commands::gemini::update_gemini_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            ))
        }
        "list_codebuddy_accounts" => {
            to_value(crate::commands::codebuddy::list_codebuddy_accounts())
        }
        "refresh_codebuddy_token" => to_value(
            crate::commands::codebuddy::refresh_codebuddy_token(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "refresh_all_codebuddy_tokens" => {
            to_value(crate::commands::codebuddy::refresh_all_codebuddy_tokens(app_handle()?).await)
        }
        "inject_codebuddy_to_vscode" => to_value(
            crate::commands::codebuddy::inject_codebuddy_to_vscode(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "update_codebuddy_account_tags" => to_value(
            crate::commands::codebuddy::update_codebuddy_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "list_codebuddy_cn_accounts" => {
            to_value(crate::commands::codebuddy_cn::list_codebuddy_cn_accounts())
        }
        "refresh_codebuddy_cn_token" => to_value(
            crate::commands::codebuddy_cn::refresh_codebuddy_cn_token(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "refresh_all_codebuddy_cn_tokens" => to_value(
            crate::commands::codebuddy_cn::refresh_all_codebuddy_cn_tokens(app_handle()?).await,
        ),
        "inject_codebuddy_cn_to_vscode" => to_value(
            crate::commands::codebuddy_cn::inject_codebuddy_cn_to_vscode(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "update_codebuddy_cn_account_tags" => to_value(
            crate::commands::codebuddy_cn::update_codebuddy_cn_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "list_qoder_accounts" => to_value(crate::commands::qoder::list_qoder_accounts()),
        "refresh_qoder_token" => to_value(
            crate::commands::qoder::refresh_qoder_token(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "refresh_all_qoder_tokens" => {
            to_value(crate::commands::qoder::refresh_all_qoder_tokens(app_handle()?).await)
        }
        "inject_qoder_account" => to_value(
            crate::commands::qoder::inject_qoder_account(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "update_qoder_account_tags" => to_value(crate::commands::qoder::update_qoder_account_tags(
            arg(args, "accountId")?,
            arg(args, "tags")?,
        )),
        "list_trae_accounts" => to_value(crate::commands::trae::list_trae_accounts()),
        "refresh_trae_token" => to_value(
            crate::commands::trae::refresh_trae_token(app_handle()?, arg(args, "accountId")?).await,
        ),
        "refresh_all_trae_tokens" => {
            to_value(crate::commands::trae::refresh_all_trae_tokens(app_handle()?).await)
        }
        "inject_trae_account" => to_value(
            crate::commands::trae::inject_trae_account(app_handle()?, arg(args, "accountId")?)
                .await,
        ),
        "update_trae_account_tags" => to_value(
            crate::commands::trae::update_trae_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "list_workbuddy_accounts" => {
            to_value(crate::commands::workbuddy::list_workbuddy_accounts())
        }
        "refresh_workbuddy_token" => to_value(
            crate::commands::workbuddy::refresh_workbuddy_token(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "refresh_all_workbuddy_tokens" => {
            to_value(crate::commands::workbuddy::refresh_all_workbuddy_tokens(app_handle()?).await)
        }
        "inject_workbuddy_to_vscode" => to_value(
            crate::commands::workbuddy::inject_workbuddy_to_vscode(
                app_handle()?,
                arg(args, "accountId")?,
            )
            .await,
        ),
        "update_workbuddy_account_tags" => to_value(
            crate::commands::workbuddy::update_workbuddy_account_tags(
                arg(args, "accountId")?,
                arg(args, "tags")?,
            )
            .await,
        ),
        "list_zed_accounts" => to_value(crate::commands::zed::list_zed_accounts()),
        "refresh_zed_token" => to_value(
            crate::commands::zed::refresh_zed_token(app_handle()?, arg(args, "accountId")?).await,
        ),
        "refresh_all_zed_tokens" => {
            to_value(crate::commands::zed::refresh_all_zed_tokens(app_handle()?).await)
        }
        "inject_zed_account" => to_value(
            crate::commands::zed::inject_zed_account(app_handle()?, arg(args, "accountId")?).await,
        ),
        "update_zed_account_tags" => to_value(crate::commands::zed::update_zed_account_tags(
            arg(args, "accountId")?,
            arg(args, "tags")?,
        )),

        "get_provider_current_account_id" => to_value(
            crate::commands::provider_current::get_provider_current_account_id(
                app_handle()?,
                arg(args, "platform")?,
            )
            .await,
        ),
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
