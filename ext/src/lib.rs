//! ⚠️ The `rb-sys` crate is a low level library. If you are looking to write a gem in
//! Rust, you should probably use https://github.com/matsadler/magnus instead. It is powered
//! by `rb-sys` and provides a higher level API, you get all of the
//! cross-platform compatibility goodnees of `rb-sys`.
//
//! If you do need to drop down into raw libruby, you can enable the
//! `rb-sys-interop` feature and add `rb-sys` to you Cargo dependencies.

#[cfg(feature = "mri")]
use rb_sys::{
    rb_define_module, rb_define_module_under, rb_define_singleton_method, rb_str_buf_append,
    rb_utf8_str_new_cstr, VALUE,
};
#[cfg(feature = "mri")]
use std::{ffi::CString, intrinsics::transmute, os::raw::c_char};

#[cfg(feature = "mri")]
trait AsCStr {
    fn to_cstring(&self) -> *const c_char;
}

#[cfg(feature = "mri")]
impl AsCStr for str {
    /// Convert a Rust string to a C string.
    fn to_cstring(&self) -> *const c_char {
        CString::new(self).unwrap().into_raw()
    }
}

#[cfg(feature = "mri")]
unsafe extern "C" fn hello(_: VALUE, name: VALUE) -> VALUE {
    rb_str_buf_append(rb_utf8_str_new_cstr("Hello, ".to_cstring()), name)
}

#[cfg(feature = "mri")]
#[no_mangle]
unsafe extern "C" fn Init_oxi_test() {
    let oxi_module = rb_define_module("Oxi".to_cstring());
    let oxi_test_module = rb_define_module_under(oxi_module, "Test".to_cstring());

    rb_define_singleton_method(
        oxi_test_module,
        "hello".to_cstring(),
        Some(transmute::<unsafe extern "C" fn(VALUE, VALUE) -> VALUE, _>(
            hello,
        )),
        1,
    );
}

#[cfg(feature = "mri")]
#[cfg(test)]
mod tests {
    use crate::{AsCStr, Init_oxi_test};
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

            let mut result = rb_sys::rb_eval_string("Oxi::Test.hello('world')".to_cstring());
            let result = rb_sys::rb_string_value_cstr(&mut result);
            let result = std::ffi::CStr::from_ptr(result).to_str().unwrap();

            assert_eq!("Hello, world", result);

            rb_sys::ruby_cleanup(0);
        }
    }
}

#[cfg(feature = "jruby")]
use std::os::raw::c_void;
#[cfg(feature = "jruby")]
use robusta_jni::convert::{Signature, TryFromJavaValue, TryIntoJavaValue};
#[cfg(feature = "jruby")]
use robusta_jni::jni::{
    JavaVM, JNIEnv, NativeMethod, objects::{JClass, JString}, strings::JNIString,
    sys::{jint, JNI_ERR, JNI_VERSION_1_4},
};

#[cfg(feature = "jruby")]
extern "system" fn hello<'local>(env: JNIEnv<'local>,
                                 _class: JClass<'local>,
                                 name: <String as TryFromJavaValue<'local, 'local>>::Source,
) -> <String as TryIntoJavaValue<'local>>::Target {
    let name_res: robusta_jni::jni::errors::Result<String> = TryFromJavaValue::try_from(name, &env);
    match name_res {
        Ok(name_conv) => {
            let res = format!("Hello, {}", name_conv);
            let res_res: robusta_jni::jni::errors::Result<<String as TryIntoJavaValue>::Target> = TryIntoJavaValue::try_into(res, &env);
            match res_res {
                Ok(conv_res) => { return conv_res; }
                Err(err) => {
                    // No need to handle err, ClassNotFoundException will be thrown implicitly
                    let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", err));
                }
            }
        }
        Err(err) => {
            // No need to handle err, ClassNotFoundException will be thrown implicitly
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", err));
        }
    }
    JString::from(std::ptr::null_mut())
}

/// This function is executed on loading native library by JVM.
/// It initializes the cache of method and class references.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad<'local>(vm: JavaVM, _: *mut c_void) -> jint {
    let Ok(env) = vm.get_env() else { return JNI_ERR; };
    let Ok(clazz) = env.find_class(
        "oxi/test/OxiTest"
    ) else { return JNI_ERR; };
    let hello_func = hello as unsafe extern "system" fn(env: JNIEnv<'local>, _class: JClass<'local>, name: JString<'local>) -> JString<'local>;
    let hello_ptr = hello_func as *mut c_void;
    let build_xml_method = NativeMethod {
        name: JNIString::from("helloNative"),
        sig: JNIString::from(format!("({}){}",
                                     <JString as Signature>::SIG_TYPE,
                                     <JString as Signature>::SIG_TYPE)),
        fn_ptr: hello_ptr,
    };
    let Ok(_) = env.register_native_methods(clazz, &[build_xml_method]) else { return JNI_ERR; };
    JNI_VERSION_1_4
}
