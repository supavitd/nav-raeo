use hyprland::data::Clients;
use hyprland::dispatch;
use hyprland::dispatch::DispatchType::FocusWindow;
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::shared::{Address, HyprData};
use nr_core::window::Window;

pub fn list_windows() -> anyhow::Result<Vec<Window>> {
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

pub fn jump_to_window(window: Window) -> anyhow::Result<()> {
    let address = Address::new(window.identifier);

    dispatch!(FocusWindow, WindowIdentifier::Address(address))?;

    Ok(())
}
