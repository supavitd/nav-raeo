#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub identifier: String,
}

pub trait WindowManager {
    fn list_windows(&self) -> anyhow::Result<Vec<Window>>;
    fn jump_to_window(&self, window: Window) -> anyhow::Result<()>;
}

pub trait WindowOrdering {} // May want to use stdlib trait
