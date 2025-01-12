use crate::window::Window;

pub struct AppState {
    pub favorites: Vec<Window>,
}

pub trait Application {
    fn set_favorite(&self, window: &Window);
    fn unset_favorite(&self, window: &Window);
}
