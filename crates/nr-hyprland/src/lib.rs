use hyprland::data::{Clients, Version};
use hyprland::dispatch;
use hyprland::dispatch::DispatchType::FocusWindow;
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::shared::{Address, HyprData};
use nr_core::window::{Window, WindowManager};

pub struct HyprlandWm {
    pub version: Version,
}

impl HyprlandWm {
    pub fn get() -> anyhow::Result<Self> {
        let version = Version::get()?;
        Ok(Self { version })
    }
}

impl WindowManager for HyprlandWm {
    fn list_windows(&self) -> anyhow::Result<Vec<Window>> {
        let clients = Clients::get()?;
        let mut windows = Vec::new();
        for client in clients.iter() {
            windows.push(Window {
                title: client.title.to_owned(),
                identifier: client.address.to_string(),
            });
        }
        Ok(windows)
    }

    fn jump_to_window(&self, window: Window) -> anyhow::Result<()> {
        let address = Address::new(window.identifier);

        dispatch!(FocusWindow, WindowIdentifier::Address(address))?;

        Ok(())
    }
}
