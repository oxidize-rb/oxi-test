use rb_sys::{
    rb_define_module, rb_define_module_under, rb_define_singleton_method, rb_str_buf_append,
    rb_utf8_str_new_cstr, VALUE,
};
use std::{ffi::CString, intrinsics::transmute, os::raw::c_char};

trait AsCStr {
    fn as_cstr(&self) -> *const c_char;
}

impl AsCStr for str {
    /// Convert a Rust string to a C string.
    fn as_cstr(&self) -> *const c_char {
        CString::new(self).unwrap().into_raw()
    }
}

// ⚠️ The `rb-sys` crate is a low level library. If you are looking to write a gem in
// Rust, you should probably use https://github.com/matsadler/magnus instead. It is powered
// by `rb-sys` and provides a higher level API, you get all of the
// cross-platform compatibility goodnees of `rb-sys`.
//
// If you do need to drop down into raw libruby, you can enable the
// `rb-sys-interop` feature and add `rb-sys` to you Cargo dependencies.
unsafe extern "C" fn hello(_: VALUE, name: VALUE) -> VALUE {
    rb_str_buf_append(rb_utf8_str_new_cstr("Hello, ".as_cstr()), name)
}

#[no_mangle]
unsafe extern "C" fn Init_ext() {
    let oxi_module = rb_define_module("Oxi".as_cstr());
    let oxi_test_module = rb_define_module_under(oxi_module, "Test".as_cstr());

    rb_define_singleton_method(
        oxi_test_module,
        "hello".as_cstr(),
        Some(transmute::<unsafe extern "C" fn(VALUE, VALUE) -> VALUE, _>(
            hello,
        )),
        1,
    );
}
