use std::collections::HashMap;

use anyhow::Result;
use log::info;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::qr_reader::process_qr;

macro_rules! hashmap {
    {} => {
        std::collections::HashMap::new()
    };
    {$($k:expr => $v:expr),+ $(,)?} => {
        {
            let mut hashmap = std::collections::HashMap::new();
            $(
                hashmap.insert($k, $v);
            )+
            hashmap
        }
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Hotkey {
    Capture,
}

pub fn init(app: &AppHandle) {
    let hotkeys = hashmap! {
        Hotkey::Capture => Shortcut::new(
            Some(Modifiers::union(Modifiers::META, Modifiers::SHIFT)),
            tauri_plugin_global_shortcut::Code::Digit1,
        )
    };
    app.manage(Hotkeys(hotkeys));
}

struct Hotkeys(HashMap<Hotkey, Shortcut>);

impl Hotkey {
    pub fn register(&self, app: &AppHandle) -> Result<()> {
        let hotkeys = &app.state::<Hotkeys>().0;
        let shortcut = *hotkeys
            .get(self)
            .expect("Shortcut not found! Did you forget to initialize hotkeys?");
        let global_shortcut = app.global_shortcut();
        match self {
            Hotkey::Capture => {
                global_shortcut.on_shortcut(shortcut, move |app, event_shortcut, event| {
                    if shortcut == *event_shortcut && event.state() == ShortcutState::Pressed {
                        process_qr(app);
                    }
                })?
            }
        };
        info!("Hotkey '{:?}' registered", self);
        Ok(())
    }

    pub fn unregister(&self, app: &AppHandle) -> Result<()> {
        let hotkeys = &app.state::<Hotkeys>().0;
        let shortcut = *hotkeys
            .get(self)
            .expect("Shortcut not found! Did you forget to initialize hotkeys?");
        let global_shortcut = app.global_shortcut();
        global_shortcut.unregister(shortcut)?;
        info!("Hotkey '{:?}' unregistered", self);
        Ok(())
    }
}
