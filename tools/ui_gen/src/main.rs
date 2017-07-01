#![recursion_limit = "200"]
#[macro_use]
extern crate pest;
extern crate heck;

pub mod qt;

//pub mod data;
pub mod api_parser;
mod rust_ffi_gen;
mod rust_gen;
//mod rust_widgets_gen;
//mod rust_traits_gen;
//mod rust_ui_gen;
pub mod c_api_gen;

//static INPUT_HEADER: &'static str = "../../src/prodbg/PluginUI/wrui.h";
static C_API_HEADER: &'static str = "../../src/prodbg/PluginUI/generated/c_api.h";
static QT_API_IMPL: &'static str = "../../src/prodbg/PluginUI/generated/qt_api_gen.cpp";
static QT_API_IMPL_HEADER: &'static str = "../../src/prodbg/PluginUI/generated/qt_api_gen.h";
static INPUT_API: &'static str = "src/api.def";

static RUST_FFI_FILE: &'static str = "../../api/rust/prodbg_ui/src/ffi_gen.rs";
static UI_FILE: &'static str = "../../api/rust/prodbg_ui/src/lib.rs";

fn main() {
    let api_def = api_parser::ApiDef::new(INPUT_API);

    if let Err(err) = c_api_gen::generate_c_api(C_API_HEADER, &api_def) {
        panic!("Unable to generate {} err {:?}", C_API_HEADER, err);
    }

    if let Err(err) = qt::generate_qt_bindings(QT_API_IMPL, QT_API_IMPL_HEADER, &api_def) {
        panic!("Unable to generate {} err {:?}", QT_API_IMPL, err);
    }

    if let Err(err) = rust_ffi_gen::generate_ffi_bindings(RUST_FFI_FILE, &api_def.entries) {
        panic!("Unable to generate {} err {:?}", RUST_FFI_FILE, err);
    }

    if let Err(err) = rust_gen::generate_rust_bindigs(UI_FILE, &api_def) {
        panic!("Unable to generate {} err {:?}", UI_FILE, err);
    }

    /*
    if let Err(err) = rust_traits_gen::generate_traits(TRAITS_FILE, &structs) {
        panic!("Unable to generate {} err {:?}", TRAITS_FILE, err);
    }

    if let Err(err) = rust_widgets_gen::generate_rust_binding(WIDGETS_FILE, &structs) {
        panic!("Unable to generate {} err {:?}", WIDGETS_FILE, err);
    }

    if let Err(err) = rust_widgets_gen::generate_rust_binding(WIDGETS_FILE, &structs) {
        panic!("Unable to generate {} err {:?}", WIDGETS_FILE, err);
    }

    if let Err(err) = rust_ui_gen::generate_ui(UI_FILE, &structs) {
        panic!("Unable to generate {} err {:?}", UI_FILE, err);
    }
    */
}
