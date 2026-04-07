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

    use crate::ui::bar::workspace::Workspace;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::Workspaces)]
    pub struct Workspaces {}

    #[glib::object_subclass]
    impl ObjectSubclass for Workspaces {
        const NAME: &'static str = "Workspaces";
        type Type = super::Workspaces;
        type ParentType = gtk::Box;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Workspaces {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            for x in 1..6 {
                let wkspc = Workspace::new(x);
                obj.append(&wkspc);
            }
        }
    }

    impl WidgetImpl for Workspaces {}
    impl BoxImpl for Workspaces {}

    impl Workspaces {}
}

glib::wrapper! {
    pub struct Workspaces(ObjectSubclass<imp::Workspaces>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}
