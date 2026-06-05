use serde_json::Value;

use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        // github_copilot
        "delete_github_copilot_account" => to_value(
            crate::commands::github_copilot::delete_github_copilot_account(arg(args, "accountId")?),
        ),
        "delete_github_copilot_accounts" => to_value(
            crate::commands::github_copilot::delete_github_copilot_accounts(arg(
                args,
                "accountIds",
            )?),
        ),
        "import_github_copilot_from_json" => to_value(
            crate::commands::github_copilot::import_github_copilot_from_json(arg(
                args,
                "jsonContent",
            )?),
        ),
        "import_github_copilot_from_local" => to_value(
            crate::commands::github_copilot::import_github_copilot_from_local(app_handle()?).await,
        ),
        "export_github_copilot_accounts" => to_value(
            crate::commands::github_copilot::export_github_copilot_accounts(arg(
                args,
                "accountIds",
            )?),
        ),
        "github_copilot_oauth_login_start" => {
            to_value(crate::commands::github_copilot::github_copilot_oauth_login_start().await)
        }
        "github_copilot_oauth_login_complete" => to_value(
            crate::commands::github_copilot::github_copilot_oauth_login_complete(
                app_handle()?,
                arg(args, "loginId")?,
            )
            .await,
        ),
        "github_copilot_oauth_login_cancel" => to_value(
            crate::commands::github_copilot::github_copilot_oauth_login_cancel(opt_arg(
                args, "loginId",
            )?),
        ),
        "add_github_copilot_account_with_token" => to_value(
            crate::commands::github_copilot::add_github_copilot_account_with_token(
                app_handle()?,
                arg(args, "githubAccessToken")?,
            )
            .await,
        ),
        "get_github_copilot_accounts_index_path" => {
            to_value(crate::commands::github_copilot::get_github_copilot_accounts_index_path())
        }

        // windsurf
        "delete_windsurf_account" => to_value(crate::commands::windsurf::delete_windsurf_account(
            arg(args, "accountId")?,
        )),
        "delete_windsurf_accounts" => to_value(
            crate::commands::windsurf::delete_windsurf_accounts(arg(args, "accountIds")?),
        ),
        "import_windsurf_from_json" => to_value(
            crate::commands::windsurf::import_windsurf_from_json(arg(args, "jsonContent")?),
        ),
        "import_windsurf_from_local" => {
            to_value(crate::commands::windsurf::import_windsurf_from_local(app_handle()?).await)
        }
        "export_windsurf_accounts" => to_value(
            crate::commands::windsurf::export_windsurf_accounts(arg(args, "accountIds")?),
        ),
        "windsurf_oauth_login_start" => {
            to_value(crate::commands::windsurf::windsurf_oauth_login_start().await)
        }
        "windsurf_oauth_login_complete" => to_value(
            crate::commands::windsurf::windsurf_oauth_login_complete(
                app_handle()?,
                arg(args, "loginId")?,
            )
            .await,
        ),
        "windsurf_oauth_submit_callback_url" => to_value(
            crate::commands::windsurf::windsurf_oauth_submit_callback_url(
                arg(args, "loginId")?,
                arg(args, "callbackUrl")?,
            ),
        ),
        "windsurf_oauth_login_cancel" => to_value(
            crate::commands::windsurf::windsurf_oauth_login_cancel(opt_arg(args, "loginId")?),
        ),
        "add_windsurf_account_with_token" => to_value(
            crate::commands::windsurf::add_windsurf_account_with_token(
                app_handle()?,
                arg(args, "githubAccessToken")?,
            )
            .await,
        ),
        "add_windsurf_account_with_password" => to_value(
            crate::commands::windsurf::add_windsurf_account_with_password(
                app_handle()?,
                arg(args, "email")?,
                arg(args, "password")?,
            )
            .await,
        ),
        "add_windsurf_accounts_with_password" => to_value(
            crate::commands::windsurf::add_windsurf_accounts_with_password(
                app_handle()?,
                arg(args, "credentials")?,
            )
            .await,
        ),
        "get_windsurf_accounts_index_path" => {
            to_value(crate::commands::windsurf::get_windsurf_accounts_index_path())
        }
        // kiro
        "delete_kiro_account" => to_value(crate::commands::kiro::delete_kiro_account(arg(
            args,
            "accountId",
        )?)),
        "delete_kiro_accounts" => to_value(crate::commands::kiro::delete_kiro_accounts(arg(
            args,
            "accountIds",
        )?)),
        "import_kiro_from_json" => to_value(crate::commands::kiro::import_kiro_from_json(arg(
            args,
            "jsonContent",
        )?)),
        "import_kiro_from_local" => {
            to_value(crate::commands::kiro::import_kiro_from_local(app_handle()?).await)
        }
        "export_kiro_accounts" => to_value(crate::commands::kiro::export_kiro_accounts(arg(
            args,
            "accountIds",
        )?)),
        "kiro_oauth_login_start" => to_value(crate::commands::kiro::kiro_oauth_login_start().await),
        "kiro_oauth_login_complete" => to_value(
            crate::commands::kiro::kiro_oauth_login_complete(app_handle()?, arg(args, "loginId")?)
                .await,
        ),
        "kiro_oauth_submit_callback_url" => {
            to_value(crate::commands::kiro::kiro_oauth_submit_callback_url(
                arg(args, "loginId")?,
                arg(args, "callbackUrl")?,
            ))
        }
        "kiro_oauth_login_cancel" => to_value(crate::commands::kiro::kiro_oauth_login_cancel(
            opt_arg(args, "loginId")?,
        )),
        "add_kiro_account_with_token" => to_value(
            crate::commands::kiro::add_kiro_account_with_token(
                app_handle()?,
                arg(args, "accessToken")?,
            )
            .await,
        ),
        "get_kiro_accounts_index_path" => {
            to_value(crate::commands::kiro::get_kiro_accounts_index_path())
        }
        // codebuddy
        "delete_codebuddy_account" => to_value(
            crate::commands::codebuddy::delete_codebuddy_account(arg(args, "accountId")?),
        ),
        "delete_codebuddy_accounts" => to_value(
            crate::commands::codebuddy::delete_codebuddy_accounts(arg(args, "accountIds")?),
        ),
        "import_codebuddy_from_json" => to_value(
            crate::commands::codebuddy::import_codebuddy_from_json(arg(args, "jsonContent")?),
        ),
        "import_codebuddy_from_local" => {
            to_value(crate::commands::codebuddy::import_codebuddy_from_local(app_handle()?).await)
        }
        "export_codebuddy_accounts" => to_value(
            crate::commands::codebuddy::export_codebuddy_accounts(arg(args, "accountIds")?),
        ),
        "codebuddy_oauth_login_start" => {
            to_value(crate::commands::codebuddy::codebuddy_oauth_login_start().await)
        }
        "codebuddy_oauth_login_complete" => to_value(
            crate::commands::codebuddy::codebuddy_oauth_login_complete(
                app_handle()?,
                arg(args, "loginId")?,
            )
            .await,
        ),
        "codebuddy_oauth_login_cancel" => to_value(
            crate::commands::codebuddy::codebuddy_oauth_login_cancel(opt_arg(args, "loginId")?),
        ),
        "add_codebuddy_account_with_token" => to_value(
            crate::commands::codebuddy::add_codebuddy_account_with_token(
                app_handle()?,
                arg(args, "accessToken")?,
            )
            .await,
        ),
        "get_codebuddy_accounts_index_path" => {
            to_value(crate::commands::codebuddy::get_codebuddy_accounts_index_path())
        }
        // codebuddy_cn
        "delete_codebuddy_cn_account" => to_value(
            crate::commands::codebuddy_cn::delete_codebuddy_cn_account(arg(args, "accountId")?),
        ),
        "delete_codebuddy_cn_accounts" => to_value(
            crate::commands::codebuddy_cn::delete_codebuddy_cn_accounts(arg(args, "accountIds")?),
        ),
        "import_codebuddy_cn_from_json" => to_value(
            crate::commands::codebuddy_cn::import_codebuddy_cn_from_json(arg(args, "jsonContent")?),
        ),
        "import_codebuddy_cn_from_local" => to_value(
            crate::commands::codebuddy_cn::import_codebuddy_cn_from_local(app_handle()?).await,
        ),
        "export_codebuddy_cn_accounts" => to_value(
            crate::commands::codebuddy_cn::export_codebuddy_cn_accounts(arg(args, "accountIds")?),
        ),
        "codebuddy_cn_oauth_login_start" => {
            to_value(crate::commands::codebuddy_cn::codebuddy_cn_oauth_login_start().await)
        }
        "codebuddy_cn_oauth_login_complete" => to_value(
            crate::commands::codebuddy_cn::codebuddy_cn_oauth_login_complete(
                app_handle()?,
                arg(args, "loginId")?,
            )
            .await,
        ),
        "codebuddy_cn_oauth_login_cancel" => to_value(
            crate::commands::codebuddy_cn::codebuddy_cn_oauth_login_cancel(opt_arg(
                args, "loginId",
            )?),
        ),
        "add_codebuddy_cn_account_with_token" => to_value(
            crate::commands::codebuddy_cn::add_codebuddy_cn_account_with_token(
                app_handle()?,
                arg(args, "accessToken")?,
            )
            .await,
        ),
        "get_codebuddy_cn_accounts_index_path" => {
            to_value(crate::commands::codebuddy_cn::get_codebuddy_cn_accounts_index_path())
        }
        "sync_codebuddy_cn_to_workbuddy" => to_value(
            crate::commands::codebuddy_cn::sync_codebuddy_cn_to_workbuddy(app_handle()?).await,
        ),
        // workbuddy
        "delete_workbuddy_account" => to_value(
            crate::commands::workbuddy::delete_workbuddy_account(arg(args, "accountId")?),
        ),
        "delete_workbuddy_accounts" => to_value(
            crate::commands::workbuddy::delete_workbuddy_accounts(arg(args, "accountIds")?),
        ),
        "import_workbuddy_from_json" => to_value(
            crate::commands::workbuddy::import_workbuddy_from_json(arg(args, "jsonContent")?),
        ),
        "import_workbuddy_from_local" => {
            to_value(crate::commands::workbuddy::import_workbuddy_from_local(app_handle()?).await)
        }
        "export_workbuddy_accounts" => to_value(
            crate::commands::workbuddy::export_workbuddy_accounts(arg(args, "accountIds")?),
        ),
        "workbuddy_oauth_login_start" => {
            to_value(crate::commands::workbuddy::workbuddy_oauth_login_start().await)
        }
        "workbuddy_oauth_login_complete" => to_value(
            crate::commands::workbuddy::workbuddy_oauth_login_complete(
                app_handle()?,
                arg(args, "loginId")?,
            )
            .await,
        ),
        "workbuddy_oauth_login_cancel" => to_value(
            crate::commands::workbuddy::workbuddy_oauth_login_cancel(opt_arg(args, "loginId")?),
        ),
        "add_workbuddy_account_with_token" => to_value(
            crate::commands::workbuddy::add_workbuddy_account_with_token(
                app_handle()?,
                arg(args, "accessToken")?,
            )
            .await,
        ),
        "get_workbuddy_accounts_index_path" => {
            to_value(crate::commands::workbuddy::get_workbuddy_accounts_index_path())
        }
        "sync_workbuddy_to_codebuddy_cn" => to_value(
            crate::commands::workbuddy::sync_workbuddy_to_codebuddy_cn(app_handle()?).await,
        ),
        "get_checkin_status_workbuddy" => to_value(
            crate::commands::workbuddy::get_checkin_status_workbuddy(arg(args, "accountId")?).await,
        ),
        "checkin_workbuddy" => to_value(
            crate::commands::workbuddy::checkin_workbuddy(app_handle()?, arg(args, "accountId")?)
                .await,
        ),

        // qoder
        "delete_qoder_account" => to_value(crate::commands::qoder::delete_qoder_account(arg(
            args,
            "accountId",
        )?)),
        "delete_qoder_accounts" => to_value(crate::commands::qoder::delete_qoder_accounts(arg(
            args,
            "accountIds",
        )?)),
        "import_qoder_from_json" => to_value(crate::commands::qoder::import_qoder_from_json(arg(
            args,
            "jsonContent",
        )?)),
        "import_qoder_from_local" => to_value(crate::commands::qoder::import_qoder_from_local(
            app_handle()?,
        )),
        "qoder_oauth_login_start" => {
            to_value(crate::commands::qoder::qoder_oauth_login_start().await)
        }
        "qoder_oauth_login_peek" => {
            serialize_value(crate::commands::qoder::qoder_oauth_login_peek())
        }
        "qoder_oauth_login_complete" => to_value(
            crate::commands::qoder::qoder_oauth_login_complete(
                app_handle()?,
                arg(args, "loginId")?,
            )
            .await,
        ),
        "qoder_oauth_login_cancel" => to_value(crate::commands::qoder::qoder_oauth_login_cancel(
            opt_arg(args, "loginId")?,
        )),
        "export_qoder_accounts" => to_value(crate::commands::qoder::export_qoder_accounts(arg(
            args,
            "accountIds",
        )?)),
        "get_qoder_accounts_index_path" => {
            to_value(crate::commands::qoder::get_qoder_accounts_index_path())
        }
        // zed
        "delete_zed_account" => to_value(crate::commands::zed::delete_zed_account(
            app_handle()?,
            arg(args, "accountId")?,
        )),
        "delete_zed_accounts" => to_value(crate::commands::zed::delete_zed_accounts(
            app_handle()?,
            arg(args, "accountIds")?,
        )),
        "import_zed_from_json" => to_value(crate::commands::zed::import_zed_from_json(
            app_handle()?,
            arg(args, "jsonContent")?,
        )),
        "import_zed_from_local" => {
            to_value(crate::commands::zed::import_zed_from_local(app_handle()?).await)
        }
        "export_zed_accounts" => to_value(crate::commands::zed::export_zed_accounts(arg(
            args,
            "accountIds",
        )?)),
        "zed_oauth_login_start" => to_value(crate::commands::zed::zed_oauth_login_start().await),
        "zed_oauth_login_peek" => serialize_value(crate::commands::zed::zed_oauth_login_peek()),
        "zed_oauth_login_complete" => to_value(
            crate::commands::zed::zed_oauth_login_complete(app_handle()?, arg(args, "loginId")?)
                .await,
        ),
        "zed_oauth_login_cancel" => to_value(crate::commands::zed::zed_oauth_login_cancel(
            opt_arg(args, "loginId")?,
        )),
        "zed_oauth_submit_callback_url" => {
            to_value(crate::commands::zed::zed_oauth_submit_callback_url(
                arg(args, "loginId")?,
                arg(args, "callbackUrl")?,
            ))
        }
        "zed_logout_current_account" => {
            to_value(crate::commands::zed::zed_logout_current_account(app_handle()?).await)
        }
        "zed_get_runtime_status" => to_value(crate::commands::zed::zed_get_runtime_status()),
        "zed_start_default_session" => to_value(crate::commands::zed::zed_start_default_session()),
        "zed_stop_default_session" => to_value(crate::commands::zed::zed_stop_default_session()),
        "zed_restart_default_session" => {
            to_value(crate::commands::zed::zed_restart_default_session())
        }
        "zed_focus_default_session" => to_value(crate::commands::zed::zed_focus_default_session()),

        // trae
        "delete_trae_account" => to_value(crate::commands::trae::delete_trae_account(arg(
            args,
            "accountId",
        )?)),
        "delete_trae_accounts" => to_value(crate::commands::trae::delete_trae_accounts(arg(
            args,
            "accountIds",
        )?)),
        "import_trae_from_json" => to_value(crate::commands::trae::import_trae_from_json(arg(
            args,
            "jsonContent",
        )?)),
        "import_trae_from_local" => {
            to_value(crate::commands::trae::import_trae_from_local(app_handle()?).await)
        }
        "trae_oauth_login_start" => to_value(crate::commands::trae::trae_oauth_login_start().await),
        "trae_oauth_login_complete" => to_value(
            crate::commands::trae::trae_oauth_login_complete(app_handle()?, arg(args, "loginId")?)
                .await,
        ),
        "trae_oauth_submit_callback_url" => {
            to_value(crate::commands::trae::trae_oauth_submit_callback_url(
                arg(args, "loginId")?,
                arg(args, "callbackUrl")?,
            ))
        }
        "trae_oauth_login_cancel" => to_value(crate::commands::trae::trae_oauth_login_cancel(
            opt_arg(args, "loginId")?,
        )),
        "export_trae_accounts" => to_value(crate::commands::trae::export_trae_accounts(arg(
            args,
            "accountIds",
        )?)),
        "add_trae_account_with_token" => {
            to_value(crate::commands::trae::add_trae_account_with_token(
                app_handle()?,
                arg(args, "accessToken")?,
            ))
        }
        "get_trae_accounts_index_path" => {
            to_value(crate::commands::trae::get_trae_accounts_index_path())
        }

        // cursor
        "delete_cursor_account" => to_value(crate::commands::cursor::delete_cursor_account(arg(
            args,
            "accountId",
        )?)),
        "delete_cursor_accounts" => to_value(crate::commands::cursor::delete_cursor_accounts(arg(
            args,
            "accountIds",
        )?)),
        "import_cursor_from_json" => to_value(crate::commands::cursor::import_cursor_from_json(
            arg(args, "jsonContent")?,
        )),
        "import_cursor_from_local" => to_value(crate::commands::cursor::import_cursor_from_local(
            app_handle()?,
        )),
        "export_cursor_accounts" => to_value(crate::commands::cursor::export_cursor_accounts(arg(
            args,
            "accountIds",
        )?)),
        "add_cursor_account_with_token" => {
            to_value(crate::commands::cursor::add_cursor_account_with_token(
                app_handle()?,
                arg(args, "accessToken")?,
            ))
        }
        "get_cursor_accounts_index_path" => {
            to_value(crate::commands::cursor::get_cursor_accounts_index_path())
        }
        "cursor_oauth_login_start" => to_value(crate::commands::cursor::cursor_oauth_login_start()),
        "cursor_oauth_login_complete" => to_value(
            crate::commands::cursor::cursor_oauth_login_complete(
                app_handle()?,
                arg(args, "loginId")?,
            )
            .await,
        ),
        "cursor_oauth_login_cancel" => to_value(
            crate::commands::cursor::cursor_oauth_login_cancel(opt_arg(args, "loginId")?),
        ),
        // gemini
        "delete_gemini_account" => to_value(crate::commands::gemini::delete_gemini_account(arg(
            args,
            "accountId",
        )?)),
        "delete_gemini_accounts" => to_value(crate::commands::gemini::delete_gemini_accounts(arg(
            args,
            "accountIds",
        )?)),
        "import_gemini_from_json" => to_value(
            crate::commands::gemini::import_gemini_from_json(
                app_handle()?,
                arg(args, "jsonContent")?,
            )
            .await,
        ),
        "import_gemini_from_local" => {
            to_value(crate::commands::gemini::import_gemini_from_local(app_handle()?).await)
        }
        "export_gemini_accounts" => to_value(crate::commands::gemini::export_gemini_accounts(arg(
            args,
            "accountIds",
        )?)),
        "gemini_oauth_login_start" => {
            to_value(crate::commands::gemini::gemini_oauth_login_start().await)
        }
        "gemini_oauth_login_complete" => to_value(
            crate::commands::gemini::gemini_oauth_login_complete(
                app_handle()?,
                arg(args, "loginId")?,
            )
            .await,
        ),
        "gemini_oauth_submit_callback_url" => {
            to_value(crate::commands::gemini::gemini_oauth_submit_callback_url(
                arg(args, "loginId")?,
                arg(args, "callbackUrl")?,
            ))
        }
        "gemini_oauth_login_cancel" => to_value(
            crate::commands::gemini::gemini_oauth_login_cancel(opt_arg(args, "loginId")?),
        ),
        "add_gemini_account_with_token" => to_value(
            crate::commands::gemini::add_gemini_account_with_token(
                app_handle()?,
                arg(args, "accessToken")?,
            )
            .await,
        ),
        "get_gemini_accounts_index_path" => {
            to_value(crate::commands::gemini::get_gemini_accounts_index_path())
        }
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
