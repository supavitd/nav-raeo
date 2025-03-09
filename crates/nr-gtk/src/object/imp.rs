use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

#[derive(glib::Properties, Default)]
#[properties(wrapper_type = super::WindowObject)]
pub struct WindowObject {
    #[property(get, set)]
    pub title: RefCell<String>,
    #[property(get, set)]
    pub identifier: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for WindowObject {
    const NAME: &'static str = "GtkNavRaeoWindow";
    type Type = super::WindowObject;
}

#[glib::derived_properties]
impl ObjectImpl for WindowObject {}
