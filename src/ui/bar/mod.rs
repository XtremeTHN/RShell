mod workspace;
mod workspaces;

use adw::subclass::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::*;

use crate::application::RShellApp;

use gtk4_layer_shell::LayerShell;

mod imp {
    use crate::{tools::layer::set_anchors, ui::bar::workspaces::Workspaces};
    use std::cell::RefCell;

    use gtk::glib::subclass::InitializingObject;
    use gtk4_layer_shell::{Edge, Layer};

    use crate::services::niri::Niri;

    use super::*;

    type OptionalRef<T> = RefCell<Option<T>>;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/XtremeTHN/RShell/bar.ui")]
    pub struct Bar {
        pub niri: OptionalRef<Niri>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Bar {
        const NAME: &'static str = "Bar";
        type Type = super::Bar;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Workspaces::ensure_type();

            klass.bind_template();
        }

        fn instance_init(klass: &InitializingObject<Self>) {
            klass.init_template();
        }
    }

    impl ObjectImpl for Bar {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.init_layer_shell();
            obj.set_layer(Layer::Top);
            obj.auto_exclusive_zone_enable();

            set_anchors(&*obj, vec![Edge::Bottom]);

            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                async move {
                    imp.setup().await;
                }
            ));
        }
    }
    impl WidgetImpl for Bar {}
    impl WindowImpl for Bar {}
    impl ApplicationWindowImpl for Bar {}
    impl AdwApplicationWindowImpl for Bar {}

    impl Bar {
        async fn setup(&self) {
            let n = Niri::instance();
            self.niri.replace(Some(n));
        }
    }
}

glib::wrapper! {
    pub struct Bar(ObjectSubclass<imp::Bar>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager,
                    adw::ApplicationWindow;
}

impl Bar {
    pub fn new(app: &RShellApp) -> Self {
        let obj: Bar = Object::builder().build();

        obj.set_application(Some(app));

        obj
    }
}
