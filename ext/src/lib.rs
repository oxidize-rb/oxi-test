use rb_sys::{
    rb_define_module, rb_define_module_under, rb_define_singleton_method, rb_str_buf_append,
    rb_utf8_str_new_cstr, VALUE,
};
use std::{ffi::CString, intrinsics::transmute};

trait AsCStr {
    fn as_cstr(&self) -> *const i8;
}

impl AsCStr for str {
    fn as_cstr(&self) -> *const std::os::raw::c_char {
        CString::new(self).unwrap().into_raw()
    }
}

unsafe extern "C" fn hello(_: VALUE, name: VALUE) -> VALUE {
    rb_str_buf_append(rb_utf8_str_new_cstr("Hello, ".as_cstr() as _), name)
}

#[no_mangle]
unsafe extern "C" fn Init_ext() {
    let oxi_module = rb_define_module("Oxi".as_cstr() as _);
    let oxi_test_module = rb_define_module_under(oxi_module, "Test".as_cstr() as _);

    rb_define_singleton_method(
        oxi_test_module,
        "hello".as_cstr() as _,
        Some(transmute::<unsafe extern "C" fn(VALUE, VALUE) -> VALUE, _>(
            hello,
        )),
        1,
    );
}
