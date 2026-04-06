use gtk::{Window, prelude::IsA};
use gtk4_layer_shell::{Edge, LayerShell};

pub fn set_anchors<T: IsA<Window>>(widget: &T, anchors: Vec<Edge>) {
    for x in anchors {
        widget.set_anchor(x, true);
    }
}
