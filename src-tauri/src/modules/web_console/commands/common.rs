use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tauri::AppHandle;

pub(super) fn app_handle() -> Result<AppHandle, String> {
    super::super::app_handle()
}

pub(super) fn to_value<T: Serialize>(result: Result<T, String>) -> Result<Value, String> {
    serde_json::to_value(result?).map_err(|err| format!("serialize response failed: {}", err))
}

pub(super) fn serialize_value<T: Serialize>(value: T) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|err| format!("serialize response failed: {}", err))
}

pub(super) fn arg<T: DeserializeOwned>(args: &Value, key: &str) -> Result<T, String> {
    let value = args
        .get(key)
        .cloned()
        .ok_or_else(|| format!("missing argument '{}'", key))?;
    serde_json::from_value(value).map_err(|err| format!("invalid argument '{}': {}", key, err))
}

pub(super) fn optional_string(args: &Value, key: &str) -> Option<String> {
    args.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub(super) fn arg_or<T: DeserializeOwned>(
    args: &Value,
    key: &str,
    default: T,
) -> Result<T, String> {
    match args.get(key) {
        Some(value) => serde_json::from_value(value.clone())
            .map_err(|err| format!("invalid argument '{}': {}", key, err)),
        None => Ok(default),
    }
}

pub(super) fn opt_arg<T: DeserializeOwned>(args: &Value, key: &str) -> Result<Option<T>, String> {
    match args.get(key) {
        Some(Value::Null) | None => Ok(None),
        Some(value) => serde_json::from_value(value.clone())
            .map(Some)
            .map_err(|err| format!("invalid argument '{}': {}", key, err)),
    }
}

pub(super) fn opt_nullable_arg<T: DeserializeOwned>(
    args: &Value,
    key: &str,
) -> Result<Option<Option<T>>, String> {
    match args.get(key) {
        None => Ok(None),
        Some(Value::Null) => Ok(Some(None)),
        Some(value) => serde_json::from_value(value.clone())
            .map(|value| Some(Some(value)))
            .map_err(|err| format!("invalid argument '{}': {}", key, err)),
    }
}
