use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/keanukerr/semmel/window.ui")]
    pub struct SemmelWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SemmelWindow {
        const NAME: &'static str = "SemmelWindow";
        type Type = super::SemmelWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SemmelWindow {}
    impl WidgetImpl for SemmelWindow {}
    impl WindowImpl for SemmelWindow {}
    impl ApplicationWindowImpl for SemmelWindow {}
}

glib::wrapper! {
    pub struct SemmelWindow(ObjectSubclass<imp::SemmelWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl SemmelWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create SemmelWindow")
    }
}
