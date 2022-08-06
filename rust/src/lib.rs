mod enum_test;
use gdnative::prelude::{godot_init, InitHandle};

fn init(handle: InitHandle) {
    handle.add_class::<enum_test::Move>();
}

godot_init!(init);
