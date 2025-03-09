mod object;

use std::rc::Rc;

use crate::object::WindowObject;
use directories::BaseDirs;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gtk::{
    gdk, glib, Application, ApplicationWindow, Box as GtkBox, CssProvider, CustomFilter, ListItem,
    ListView, Orientation, SearchEntry, SignalListItemFactory, SingleSelection,
};
use gtk::{gio, FilterListModel};
use gtk::{prelude::*, Label, ScrolledWindow};
use nr_core::window::{Window as NrWindow, WindowManager};

const APP_ID: &str = "com.github.supavitd.nav_raeo";

pub fn run_app<T: WindowManager + 'static>(wm: Rc<T>) -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    log::debug!("Starting gtk...");

    app.connect_startup(|_| load_css());

    let wm_gtk = Rc::clone(&wm);
    app.connect_activate(move |app| {
        build_ui(app, Rc::clone(&wm_gtk)).expect("Failed to build UI");
    });

    app.run()
}

fn load_css() {
    let Some(config_dir) = BaseDirs::new().map(|dir| dir.config_dir().join("nav-raeo")) else {
        return;
    };

    log::debug!("{:?}", config_dir.join("style.css"));

    let provider = CssProvider::new();
    provider.load_from_path(config_dir.join("style.css"));

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui<T: WindowManager + 'static>(app: &Application, wm: Rc<T>) -> anyhow::Result<()> {
    let search_entry = SearchEntry::builder().placeholder_text("Search...").build();
    search_entry.set_search_delay(100);

    // TODO: Make sure the scrolled window still works but search_entry should be outside of list
    // so the search entry sticks to the top
    let list_view = build_window_list_ui(app, wm, &search_entry)?;

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(360)
        .vexpand(true)
        .child(&list_view)
        .build();

    let vbox = GtkBox::builder().orientation(Orientation::Vertical).build();

    vbox.append(&search_entry);
    vbox.append(&scrolled_window);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1000)
        .default_height(400)
        .resizable(false)
        .child(&vbox)
        .build();

    search_entry.set_key_capture_widget(Some(&window));
    search_entry.set_can_focus(false);

    let key_controller = gtk::EventControllerKey::new();
    key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
    key_controller.connect_key_pressed(glib::clone!(
        #[strong]
        app,
        move |_, keyval, _, _| {
            if keyval == gdk::Key::Escape {
                app.quit();
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        }
    ));
    window.add_controller(key_controller);
    window.present();

    Ok(())
}

fn build_window_list_ui<T: WindowManager + 'static>(
    app: &Application,
    wm: Rc<T>,
    search: &SearchEntry,
) -> anyhow::Result<ListView> {
    let model = gio::ListStore::new::<WindowObject>();
    let window_objs: Vec<WindowObject> = wm
        .list_windows()?
        .iter()
        .map(|w| WindowObject::from(&w))
        .collect();
    model.extend_from_slice(&window_objs);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));
    });
    factory.connect_bind(move |_, list_item| {
        let window_obj = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<WindowObject>()
            .expect("The item has to be a `WindowObject`.");
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<Label>()
            .expect("This child has to be a `Label`.");

        label.set_label(&window_obj.title());
    });

    let filter = CustomFilter::new(move |_| true);
    search.connect_search_changed(glib::clone!(
        #[strong]
        filter,
        move |entry| {
            let text = entry.text();
            let matcher = SkimMatcherV2::default();
            filter.set_filter_func(move |obj| {
                let window_obj = obj
                    .downcast_ref::<WindowObject>()
                    .expect("The item has to be a `WindowObject`.");
                let score = matcher.fuzzy_match(&window_obj.title(), &text);
                score.is_some()
            });
        }
    ));
    let filter_model = FilterListModel::new(Some(model), Some(filter.clone()));
    let selection_model = SingleSelection::new(Some(filter_model));

    let list_view = ListView::new(Some(selection_model), Some(factory));

    list_view.connect_activate(glib::clone!(
        #[strong]
        app,
        #[strong]
        wm,
        move |l, _| {
            if let Some(model) = l.model() {
                let single_select = model
                    .downcast_ref::<SingleSelection>()
                    .expect("Single selection model");

                if let Some(obj) = single_select.selected_item() {
                    let window_obj = obj.downcast_ref::<WindowObject>().expect("Window object");
                    log::debug!("Selected object {:?}", &window_obj);
                    wm.jump_to_window(NrWindow {
                        title: window_obj.title(),
                        identifier: window_obj.identifier(),
                    })
                    .unwrap();
                    app.quit();
                }
            }
        }
    ));

    Ok(list_view)
}
