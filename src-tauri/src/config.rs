use std::any::Any;

use log::error;
use serde_json::Value as JsonValue;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigKey {
    ReadQrShortcut,
    OpenBrowser,
}

impl ToString for ConfigKey {
    fn to_string(&self) -> String {
        match self {
            ConfigKey::ReadQrShortcut => String::from("read_qr_shortcut"),
            ConfigKey::OpenBrowser => String::from("open_browser"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigValue {
    ReadQrShortcut(bool),
    OpenBrowser(bool),
}

impl ConfigValue {
    pub fn default_value(key: ConfigKey) -> Self {
        match key {
            ConfigKey::ReadQrShortcut => ConfigValue::ReadQrShortcut(false),
            ConfigKey::OpenBrowser => ConfigValue::OpenBrowser(true),
        }
    }

    pub fn from_json_value(key: ConfigKey, value: JsonValue) -> Option<Self> {
        match key {
            ConfigKey::ReadQrShortcut => value.as_bool().map(ConfigValue::ReadQrShortcut),
            ConfigKey::OpenBrowser => value.as_bool().map(ConfigValue::OpenBrowser),
        }
    }
}

impl From<&ConfigValue> for ConfigKey {
    fn from(value: &ConfigValue) -> Self {
        match value {
            ConfigValue::ReadQrShortcut(_) => ConfigKey::ReadQrShortcut,
            ConfigValue::OpenBrowser(_) => ConfigKey::OpenBrowser,
        }
    }
}

impl From<ConfigValue> for JsonValue {
    fn from(value: ConfigValue) -> Self {
        match value {
            ConfigValue::ReadQrShortcut(v) => JsonValue::Bool(v),
            ConfigValue::OpenBrowser(v) => JsonValue::Bool(v),
        }
    }
}

const CONFIG_PATH: &str = "config.json";

pub fn store_cfg(app_handle: &AppHandle, value: ConfigValue) {
    let store = app_handle.store(CONFIG_PATH);
    match store {
        Ok(store) => store.set(ConfigKey::from(&value).to_string(), value),
        Err(e) => error!("{e:?}"),
    }
}

pub fn load_cfg<T: 'static>(app_handle: &AppHandle, key: ConfigKey) -> T {
    let store = app_handle.store(CONFIG_PATH);
    let value = match store {
        Ok(store) => store
            .get(key.to_string())
            .map(|value| ConfigValue::from_json_value(key, value))
            .flatten(),
        Err(e) => {
            error!("{e:?}");
            None
        }
    }
    .unwrap_or(ConfigValue::default_value(key));
    let value: Box<dyn Any> = match value {
        ConfigValue::OpenBrowser(v) => Box::new(v),
        ConfigValue::ReadQrShortcut(v) => Box::new(v),
    };
    value.downcast::<T>().map(|v| *v).unwrap_or_else(|_| {
        error!(
            "Failed to downcast config value for key {}",
            key.to_string()
        );
        let value: Box<dyn Any> = Box::new(ConfigValue::default_value(key));
        value.downcast::<T>().map(|v| *v).expect(&format!(
            "Failed to downcast default config value for key {}",
            key.to_string()
        ))
    })
}
