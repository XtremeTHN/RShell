pub mod socket;
pub mod types;

use std::cell::OnceCell;

use gtk::glib::{self, Object};

mod imp {
    use adw::subclass::prelude::ObjectSubclassExt;
    use async_std::channel::Receiver;
    use gtk::glib::{
        self,
        object::ObjectExt,
        subclass::{
            Signal,
            prelude::{DerivedObjectProperties, ObjectImpl, ObjectImplExt, ObjectSubclass},
        },
    };
    use niri_ipc::{Event, Reply, Request, Response, Window, Workspace};

    use super::{socket::NiriSocket, types::*};
    use std::{cell::RefCell, collections::HashMap, sync::OnceLock};

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::Niri)]
    pub struct Niri {
        pub socket: Socket,

        #[property(get)]
        pub windows_hash: RefCell<NiriWindows>,

        #[property(get)]
        pub workspaces_hash: RefCell<NiriWorkspaces>,

        #[property(get)]
        pub focused_window: OptionalRef<NiriWindow>,

        #[property(get)]
        pub focused_workspace: OptionalRef<NiriWorkspace>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Niri {
        const NAME: &'static str = "Niri";
        type Type = super::Niri;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Niri {
        fn constructed(&self) {
            self.parent_constructed();

            self.setup();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("window-added").build(),
                    Signal::builder("window-removed").build(),
                ]
            })
        }
    }

    impl Niri {
        async fn send(&self, msg: Request) -> std::io::Result<Reply> {
            self.socket.borrow_mut().as_mut().unwrap().send(msg).await
        }

        async fn handle_events(&self, receiver: Receiver<Event>) {
            let obj = self.obj();
            while let Ok(e) = receiver.recv().await {
                match e {
                    Event::WindowClosed { id } => {
                        self.windows_hash.borrow_mut().0.remove(&id);
                    }
                    Event::WindowOpenedOrChanged { window } => {
                        self.windows_hash
                            .borrow_mut()
                            .0
                            .insert(window.id, window.clone());

                        if window.is_focused {
                            let win = NiriWindow(window);
                            self.focused_window.replace(Some(win));
                            obj.notify_focused_window();
                        }
                    }
                    Event::WindowFocusChanged { id } => {
                        if let Some(id) = id
                            && let Some(window) = self.windows_hash.borrow().0.get(&id)
                        {
                            let win = NiriWindow(window.clone());
                            self.focused_window.replace(Some(win));
                            obj.notify_focused_window();
                        }
                    }
                    Event::WindowLayoutsChanged { changes: _ } => {
                        // TODO: maybe implement this instead of querying the focused window
                    }
                    Event::WorkspaceActiveWindowChanged {
                        workspace_id: _,
                        active_window_id,
                    } => {
                        if let Some(id) = active_window_id
                            && let Some(win) = self.windows_hash.borrow().0.get(&id)
                        {
                            let win = NiriWindow(win.clone());
                            self.focused_window.replace(Some(win));
                            obj.notify_focused_window();
                        }
                    }
                    Event::WorkspacesChanged { workspaces } => {
                        let mut hash = HashMap::new();
                        for x in workspaces {
                            hash.insert(x.id, x);
                        }

                        self.workspaces_hash.replace(NiriWorkspaces(hash));
                        obj.notify_workspaces_hash();
                    }
                    Event::WorkspaceActivated { id, focused } => {
                        if !focused {
                            continue;
                        }

                        if let Some(workspace) = self.workspaces_hash.borrow().0.get(&id) {
                            let wkspc = NiriWorkspace(workspace.clone());
                            self.focused_workspace.replace(Some(wkspc));
                            obj.notify_focused_workspace();
                        } else {
                            glib::g_warning!("Niri", "Couldn't find workspace with id: {}", id);
                        }
                    }
                    _ => {
                        glib::g_debug!("Niri", "Unknown event: {:?}", e);
                    }
                }
            }
        }

        async fn setup_events_sock(&self) {
            let (sender, receiver) = async_std::channel::bounded(1);

            glib::spawn_future(async {
                let Ok(mut sock) = NiriSocket::new().await.inspect_err(|e| {
                    glib::g_critical!("Niri", "Couldn't connect to niri socket: {}", e)
                }) else {
                    return;
                };

                let res = sock.send(niri_ipc::Request::EventStream).await;

                if let Err(e) = res {
                    glib::g_critical!("Niri", "Failed to send request EventStream: {}", e);
                    return;
                }

                if let Err(e) = res.unwrap() {
                    glib::g_critical!("Niri", "Failed to parse reply: {}", e);
                    return;
                }

                sock.receive_events(sender).await;
            });

            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                async move {
                    imp.handle_events(receiver).await;
                }
            ));
        }

        async fn setup_props(&self) -> Result<(), Box<dyn std::error::Error>> {
            let mut sock = NiriSocket::new().await?;

            // TODO: make a helper func
            let Response::Windows(windows) = sock.send(Request::Windows).await?? else {
                return Ok(());
            };

            let map: Map<Window> = windows.into_iter().map(|w| (w.id, w)).collect();
            self.windows_hash.replace(NiriWindows(map));

            let Response::Workspaces(workspaces) = sock.send(Request::Workspaces).await?? else {
                return Ok(());
            };
            let map: Map<Workspace> = workspaces.into_iter().map(|w| (w.id, w)).collect();
            self.workspaces_hash.replace(NiriWorkspaces(map));

            self.socket.replace(Some(sock));

            Ok(())
        }

        fn setup(&self) {
            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                async move {
                    imp.setup_events_sock().await;
                    if let Err(e) = imp.setup_props().await {
                        glib::g_warning!("Niri", "Couldn't set main properties: {}", e);
                    };
                }
            ));
        }
    }
}

glib::wrapper! {
    pub struct Niri(ObjectSubclass<imp::Niri>);
}

thread_local! {
    static INSTANCE: OnceCell<Niri> = OnceCell::new();
}

impl Niri {
    fn new() -> Self {
        Object::builder().build()
    }

    pub fn instance() -> Self {
        INSTANCE.with(|cell| cell.get_or_init(|| Niri::new()).clone())
    }
}
