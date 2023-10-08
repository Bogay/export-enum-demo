mod enum_test;
use gdnative::prelude::*;

struct ExportEnumDemoLibrary;

#[gdnative::init::callbacks]
impl GDNativeCallbacks for ExportEnumDemoLibrary {
    fn nativescript_init(handle: InitHandle) {
        handle.add_class::<enum_test::Move>();
    }
}
