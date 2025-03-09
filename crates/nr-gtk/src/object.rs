use gtk::glib;
use nr_core::window::Window;

mod imp;

glib::wrapper! {
    pub struct WindowObject(ObjectSubclass<imp::WindowObject>);
}

impl WindowObject {
    pub fn from(window: &Window) -> Self {
        glib::Object::builder()
            .property("title", window.title.clone())
            .property("identifier", window.identifier.clone())
            .build()
    }
}
