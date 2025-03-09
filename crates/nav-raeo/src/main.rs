use std::process::{ExitCode, Termination};
use std::rc::Rc;

use nr_gtk::run_app;
use nr_hyprland::HyprlandWm;

fn main() -> ExitCode {
    env_logger::init();

    let Ok(wm) = HyprlandWm::get() else {
        log::error!("Unable to retrieve Hyprland's version. Make sure it is running.");
        return ExitCode::FAILURE;
    };

    if let Some(version_str) = &wm.version.version {
        log::info!("Running on Hyprland {}", version_str);
    } else {
        log::info!("Running on Hyprland {}", wm.version.commit);
    }

    let wm_rc = Rc::new(wm);

    run_app(wm_rc).report()
}
