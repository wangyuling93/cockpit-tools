use serde_json::{json, Value};

use super::super::events::{
    emit_web_event_from_browser, register_web_event_listener, unregister_web_event_listener,
};
use super::common::*;

pub(super) async fn dispatch(cmd: &str, args: &Value) -> Result<Option<Value>, String> {
    let value = match cmd {
        "plugin:app|version" => Ok(Value::String(env!("CARGO_PKG_VERSION").to_string())),
        "plugin:app|name" => Ok(Value::String("Cockpit Tools".to_string())),
        "plugin:app|identifier" => Ok(Value::String("com.jlcodes.cockpit-tools".to_string())),
        "plugin:app|tauri_version" => Ok(Value::String("2".to_string())),
        "plugin:event|listen" => serialize_value(register_web_event_listener(
            arg(args, "event")?,
            optional_string(args, "clientId").unwrap_or_else(|| "default".to_string()),
        )?),
        "plugin:event|unlisten" => {
            unregister_web_event_listener(
                arg(args, "eventId")?,
                optional_string(args, "clientId").as_deref(),
            )?;
            Ok(Value::Null)
        }
        "plugin:event|emit" | "plugin:event|emit_to" => {
            emit_web_event_from_browser(arg(args, "event")?, args.get("payload").cloned())?;
            Ok(Value::Null)
        }
        "plugin:window|get_all_windows" => Ok(json!([{ "label": "main" }])),
        "plugin:webview|get_all_webviews" => {
            Ok(json!([{ "label": "main", "windowLabel": "main" }]))
        }
        "plugin:window|start_dragging"
        | "plugin:window|set_theme"
        | "plugin:webview|set_webview_zoom"
        | "plugin:webview|set_zoom" => Ok(Value::Null),
        _ => return Ok(None),
    }?;
    Ok(Some(value))
}
