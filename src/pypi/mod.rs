pub(super) mod native;

use native::Topy;

extern "C" fn build(_s: *mut native::Object, args: *mut native::Object) -> *mut native::Object {
  native::parse_tuple!(args, "l\0", n: std::ffi::c_long);
  #[allow(clippy::cast_sign_loss)]
  match super::build_impl(n as u32) {
    Ok(v) => v.topy(),
    Err(v) => {
      let exc = format!("{v}\0");
      native::Exception::set_string(unsafe { native::PyExc_Exception }, exc.as_ptr().cast());
      std::ptr::null_mut()
    }
  }
}

static METHODS: &[native::MethodDef] = &[
  native::MethodDef::new(
    "\0".as_ptr().cast(),
    Some(build),
    native::MethodDefFlags::VARARGS,
    "\0".as_ptr().cast(),
  ),
  native::MethodDef::NULL,
];

static mut MODULE: native::ModuleDef =
  native::ModuleDef::new("\0".as_ptr().cast(), "\0".as_ptr().cast(), METHODS.as_ptr());

#[no_mangle]
extern "C" fn PyInit_meta_banner() -> *mut native::Object {
  native::Module::create(unsafe { &mut MODULE })
}
