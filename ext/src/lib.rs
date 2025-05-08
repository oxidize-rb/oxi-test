//! ⚠️ The `rb-sys` crate is a low level library. If you are looking to write a gem in
//! Rust, you should probably use https://github.com/matsadler/magnus instead. It is powered
//! by `rb-sys` and provides a higher level API, you get all of the
//! cross-platform compatibility goodnees of `rb-sys`.
//
//! If you do need to drop down into raw libruby, you can enable the
//! `rb-sys-interop` feature and add `rb-sys` to you Cargo dependencies.
//!
//! 👉 Read the docs: https://oxidize-rb.github.io/rb-sys/

use rb_sys::{
    rb_define_module, rb_define_module_under, rb_define_singleton_method, rb_str_buf_append,
    rb_utf8_str_new_cstr, VALUE,
};
use std::{mem::transmute, os::raw::c_char};

// Converts a static &str to a C string usable in foreign functions.
macro_rules! static_cstring {
    ($string:expr) => {{
        concat!($string, "\0").as_ptr() as *const c_char
    }};
}

unsafe extern "C" fn hello(_: VALUE, name: VALUE) -> VALUE {
    rb_str_buf_append(rb_utf8_str_new_cstr(static_cstring!("Hello, ")), name)
}

#[no_mangle]
unsafe extern "C" fn Init_oxi_test() {
    let oxi_module = rb_define_module(static_cstring!("Oxi"));
    let oxi_test_module = rb_define_module_under(oxi_module, static_cstring!("Test"));

    rb_define_singleton_method(
        oxi_test_module,
        static_cstring!("hello"),
        Some(transmute::<
            unsafe extern "C" fn(VALUE, VALUE) -> VALUE,
            unsafe extern "C" fn() -> VALUE,
        >(hello)),
        1,
    );
}

#[cfg(test)]
mod tests {
    use crate::Init_oxi_test;
    use std::os::raw::c_char;

    // By default, Cargo will run tests in parallel. This *will* segfault the
    // Ruby VM. In this simple example we are only writing a single test, but if
    // you want more than one you need to set `RUST_TEST_THREADS=1` in your
    // environment (or .cargo/config.toml).
    #[test]
    fn test_simple_hello() {
        let argv: [*mut c_char; 0] = [];
        let argv = argv.as_ptr();
        let mut argc = 1;

        unsafe {
            rb_sys::ruby_sysinit(&mut argc, argv as _);
            rb_sys::ruby_init();

            Init_oxi_test();

            let mut result = rb_sys::rb_eval_string(static_cstring!("Oxi::Test.hello('world')"));
            let result = rb_sys::rb_string_value_cstr(&mut result);
            let result = std::ffi::CStr::from_ptr(result).to_str().unwrap();

            assert_eq!("Hello, world", result);

            rb_sys::ruby_cleanup(0);
        }
    }
}
