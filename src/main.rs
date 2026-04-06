mod application;
mod config;
mod services;
mod tools;
mod ui;

use gtk::{
    gio::{self, prelude::ApplicationExtManual},
    glib,
};

use crate::application::RShellApp;
use crate::config::PKGDATADIR;

fn main() -> glib::ExitCode {
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/rshell.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    let app = RShellApp::new("com.github.XtremeTHN.Lift");
    app.run()
}
