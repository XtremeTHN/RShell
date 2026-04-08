use gtk::glib::{self, Object};

mod imp {
    use adw::subclass::prelude::{ObjectImplExt, ObjectSubclassExt};
    use astal_apps::prelude::{ApplicationExt, AppsExt};
    use gtk::{
        glib::{
            self,
            subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
        },
        prelude::BoxExt,
        subclass::{box_::BoxImpl, widget::WidgetImpl},
    };
    use niri_ipc::Window;

    use crate::services::niri::{Niri, types::OptionalRef};

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::ActiveWindow)]
    pub struct ActiveWindow {
        pub icon: gtk::Image,
        pub label: gtk::Label,
        pub previous_focused_id: OptionalRef<String>,
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
                        if *imp.previous_focused_id.borrow() != win.0.app_id {
                            imp.icon
                                .set_paintable(imp.app_icon_name(win.0.app_id.clone()).as_ref());
                        }

                        imp.previous_focused_id.replace(win.0.app_id);

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
        fn lookup_icon(&self, icon_name: &str) -> gtk::IconPaintable {
            gtk::IconTheme::default().lookup_icon(
                icon_name,
                &["application-x-executable-symbolic"],
                64,
                1,
                gtk::TextDirection::Ltr,
                gtk::IconLookupFlags::PRELOAD,
            )
        }

        fn icon_from_app(&self, app: astal_apps::Application) -> Option<gtk::IconPaintable> {
            let icon_name = app.icon_name();

            match std::fs::exists(&*icon_name) {
                Ok(true) => {
                    let file = gio::File::for_path(&*icon_name);
                    Some(gtk::IconPaintable::for_file(&file, 64, 1))
                }
                Ok(false) => Some(self.lookup_icon(&icon_name)),
                Err(e) => {
                    glib::g_warning!(
                        "ActiveWindow",
                        "Couldn't prove the existence of icon \"{}\": {}",
                        icon_name,
                        e
                    );
                    None
                }
            }
        }

        fn app_icon_name(&self, app_id: Option<String>) -> Option<gtk::IconPaintable> {
            let icon = app_id.as_deref().unwrap_or_default();
            let theme = gtk::IconTheme::default();

            if theme.has_icon(icon) {
                Some(self.lookup_icon(icon))
            } else {
                let apps = astal_apps::Apps::default();
                apps.fuzzy_query(app_id.as_deref())
                    .into_iter()
                    .find_map(|app| self.icon_from_app(app))
            }
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
