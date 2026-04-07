use gtk::glib::{self, Object};

mod imp {
    use adw::subclass::prelude::{ObjectImplExt, ObjectSubclassExt};
    use gtk::{
        glib::{
            self,
            subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
        },
        prelude::BoxExt,
        subclass::{box_::BoxImpl, widget::WidgetImpl},
    };

    use crate::services::niri::Niri;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::ActiveWindow)]
    pub struct ActiveWindow {
        pub icon: gtk::Image,
        pub label: gtk::Label,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ActiveWindow {
        const NAME: &'static str = "ActiveWindow";
        type Type = super::ActiveWindow;
        type ParentType = gtk::Box;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ActiveWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.label.set_label("NixOS");
            self.icon.set_icon_name(Some("nix-snowflake"));

            let niri = Niri::instance();
            niri.connect_focused_window_notify(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                move |n| {
                    if let Some(win) = n.focused_window() {
                        imp.app_icon_name(win.0.app_id);
                        imp.label
                            .set_label(&win.0.title.unwrap_or(String::from("NixOS")));
                    };
                }
            ));

            let obj = self.obj();
            self.label.set_max_width_chars(30);
            self.label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            obj.append(&self.icon);
            obj.append(&self.label);

            obj.set_spacing(5);
        }
    }

    impl WidgetImpl for ActiveWindow {}
    impl BoxImpl for ActiveWindow {}

    impl ActiveWindow {
        fn app_icon_name(&self, app_id: Option<String>) -> gtk::IconPaintable {
            let theme = gtk::IconTheme::default();
            theme.lookup_icon(
                &app_id.unwrap_or(String::new()),
                &["application-x-executable-symbolic"],
                64,
                1,
                gtk::TextDirection::Ltr,
                gtk::IconLookupFlags::PRELOAD,
            )
        }
    }
}

glib::wrapper! {
    pub struct ActiveWindow(ObjectSubclass<imp::ActiveWindow>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ActiveWindow {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
