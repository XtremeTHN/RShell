use gtk::glib;
use niri_ipc::{Window, Workspace};

use super::socket::NiriSocket;
use std::cell::RefCell;

use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug, glib::Boxed)]
#[boxed_type(name = "NiriWindow", nullable)]
pub struct NiriWindow(pub niri_ipc::Window);

#[derive(Clone, Debug, glib::Boxed)]
#[boxed_type(name = "NiriWorkspace", nullable)]
pub struct NiriWorkspace(pub niri_ipc::Workspace);

pub type OptionalRef<T> = RefCell<Option<T>>;
pub type Socket = Rc<OptionalRef<NiriSocket>>;

pub type Map<T> = HashMap<u64, T>;
// pub type MapRef<T> = RefCell<Map<T>>;

#[derive(Clone, Debug, Default, glib::Boxed)]
#[boxed_type(name = "NiriWindows", nullable)]
pub struct NiriWindows(pub Map<Window>);

#[derive(Clone, Debug, Default, glib::Boxed)]
#[boxed_type(name = "NiriWorkspaces", nullable)]
pub struct NiriWorkspaces(pub Map<Workspace>);
