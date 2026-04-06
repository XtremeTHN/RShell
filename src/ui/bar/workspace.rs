use gtk::{
    glib::{self, Object},
    prelude::WidgetExt,
};

mod imp {
    use adw::subclass::prelude::{ObjectImplExt, ObjectSubclassExt};
    use gtk::{
        glib::{
            self,
            prelude::ObjectExt,
            subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectSubclass},
        },
        prelude::WidgetExt,
        subclass::{box_::BoxImpl, widget::WidgetImpl},
    };

    use std::cell::RefCell;

    use crate::services::niri::Niri;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::Workspace)]
    pub struct Workspace {
        #[property(get, set)]
        pub id: RefCell<u8>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Workspace {
        const NAME: &'static str = "Workspace";
        type Type = super::Workspace;
        type ParentType = gtk::Box;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Workspace {
        fn constructed(&self) {
            self.parent_constructed();

            let niri = Niri::instance();
            niri.connect_focused_workspace_notify(glib::clone!(
                #[weak(rename_to = imp)]
                self,
                move |n| {
                    if let Some(f) = n.focused_workspace()
                        && f.0.idx == *imp.id.borrow()
                    {
                        imp.obj().add_css_class("active");
                    } else {
                        imp.obj().remove_css_class("active");
                    }
                }
            ));
        }
    }

    impl WidgetImpl for Workspace {}
    impl BoxImpl for Workspace {}

    impl Workspace {}
}

glib::wrapper! {
    pub struct Workspace(ObjectSubclass<imp::Workspace>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Workspace {
    pub fn new(id: u8) -> Self {
        let obj: Workspace = Object::builder().property("id", id).build();
        obj.set_css_classes(&["workspace"]);
        obj
    }
}
