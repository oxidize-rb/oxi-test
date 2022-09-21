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
