use hyprland::data::Clients;
use hyprland::dispatch;
use hyprland::dispatch::DispatchType::FocusWindow;
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::shared::HyprData;
use nr_core::window::Window;

pub fn list_windows() -> anyhow::Result<Vec<Window>> {
    let clients = Clients::get()?;
    let mut windows = Vec::new();
    for client in clients.iter() {
        windows.push(Window {
            title: client.title.to_owned(),
            identifier: client.pid.to_string(),
        });
    }
    Ok(windows)
}

pub fn jump_to_window(window: Window) -> anyhow::Result<()> {
    let pid = window.identifier.parse::<u32>()?;

    // TODO: Use Window address instead of PID. For some reason, different chrome windows return
    // the same pid.
    dispatch!(FocusWindow, WindowIdentifier::ProcessId(pid))?;

    Ok(())
}
