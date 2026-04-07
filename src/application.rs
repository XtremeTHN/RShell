use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{
    // gio::{self, ApplicationFlags, prelude::*},
    glib,
};

use gio::{self, ApplicationFlags, prelude::ApplicationCommandLineExt};

use crate::ui::bar::Bar;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RShellApp {}

    #[glib::object_subclass]
    impl ObjectSubclass for RShellApp {
        const NAME: &'static str = "RShellApp";
        type Type = super::RShellApp;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for RShellApp {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl ApplicationImpl for RShellApp {
        fn command_line(&self, command_line: &gio::ApplicationCommandLine) -> glib::ExitCode {
            if command_line.is_remote() {
                command_line.print_literal(&format!(
                    "RShell is already running (on {})\n",
                    std::process::id()
                ));
                return glib::ExitCode::FAILURE;
            } else {
                self.activate();
            }

            glib::ExitCode::SUCCESS
        }
    }

    impl GtkApplicationImpl for RShellApp {}
    impl AdwApplicationImpl for RShellApp {}

    impl RShellApp {
        fn activate(&self) {
            let application = self.obj();
            let window = application.active_window().unwrap_or_else(|| {
                let window = Bar::new(&*application);
                window.upcast()
            });

            window.present();
        }
    }
}

glib::wrapper! {
    pub struct RShellApp(ObjectSubclass<imp::RShellApp>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl RShellApp {
    pub fn new(application_id: &str) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", ApplicationFlags::HANDLES_COMMAND_LINE)
            .property("resource-base-path", "/com/github/XtremeTHN/RShell")
            .build()
    }
}
