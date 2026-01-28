use std::{collections::HashMap, fs::File, path::PathBuf};

use log::info;
use tauri::{path::BaseDirectory, AppHandle, Manager};

const DEFAULT_LOCALE: &str = "en-US";

struct I18nData {
    locale: String,
    localizations: HashMap<String, String>,
}

pub fn initialize(app_handle: &AppHandle) {
    let locale = get_locale(app_handle);
    info!("Use locale {locale}");
    let localize_data = load_localize_data(app_handle, &locale);
    app_handle.manage(I18nData {
        locale,
        localizations: localize_data,
    });
}

pub fn translate(app_handle: &AppHandle, key: &str) -> String {
    let i18n_data = app_handle.state::<I18nData>();
    let localizations = &i18n_data.localizations;
    let translated = localizations
        .get(&key.to_string())
        .expect(&format!(
            "Could not translate to language {} from {key}",
            i18n_data.locale
        ))
        .clone();
    translated
}

pub fn translate_format(app_handle: &AppHandle, key: &str, text: &str) -> String {
    translate(app_handle, key).replace("{}", text)
}

fn get_i18n_file_path(app_handle: &AppHandle, file_name: &str) -> PathBuf {
    app_handle
        .path()
        .resolve(format!("i18n/{file_name}"), BaseDirectory::Resource)
        .expect(&format!("Could not found i18n file \"{file_name}\""))
}

fn get_locale(app_handle: &AppHandle) -> String {
    let locale = tauri_plugin_os::locale().unwrap_or(DEFAULT_LOCALE.to_string());
    let tags: Vec<&str> = locale.split("-").filter(|tag| !tag.is_empty()).collect();
    let mut locale = DEFAULT_LOCALE.to_string();
    for i in (1..=tags.len()).rev() {
        let tmp_locale = tags[0..i].join("-");
        let path = get_i18n_file_path(app_handle, &format!("{tmp_locale}.json"));
        if path.exists() {
            locale = tmp_locale;
            break;
        }
    }
    locale
}

fn load_localize_data(app_handle: &AppHandle, locale: &str) -> HashMap<String, String> {
    let file_name = format!("{}.json", locale);
    let path = get_i18n_file_path(app_handle, &file_name);
    let file = File::open(path).expect(&format!("Could not open i18n file \"{file_name}\""));
    let lang: serde_json::Value = serde_json::from_reader(file).expect(&format!(
        "Could not parse as json i18n file \"{file_name}\""
    ));
    let lang = lang.as_object().expect("Could not get value as object");
    let lang: HashMap<String, String> = lang
        .into_iter()
        .map(|(key, translated)| {
            (
                key.clone(),
                translated
                    .as_str()
                    .expect(&format!("Could not get value as string"))
                    .to_string(),
            )
        })
        .collect();
    lang
}
