pub(super) mod native;

use std::{ffi::c_void, ptr::null_mut};

use crate::components::{combine, combine_struct, extract, extract_struct, probe, probe_struct};

use self::native::{Callback, ToNapi};

fn define_exports(
  env: native::env,
  exports: native::value,
  key: impl ToNapi,
  value: impl ToNapi,
) -> native::value {
  if let Ok(v) = native::set_property(env, exports, key, value) {
    v
  } else {
    exports
  }
}

extern "C" fn init(env: native::env, mut exports: native::value) -> native::value {
  if let Ok(v) = probe_struct(env).and_then(|v| native::get_reference_value(env, v)) {
    exports = define_exports(env, exports, "Probe", v);
  }
  if let Ok(v) = extract_struct(env).and_then(|v| native::get_reference_value(env, v)) {
    exports = define_exports(env, exports, "Extract", v);
  }
  if let Ok(v) = combine_struct(env).and_then(|v| native::get_reference_value(env, v)) {
    exports = define_exports(env, exports, "Combine", v);
  }
  exports = define_exports(env, exports, "probe", Callback("probe", null_mut(), probe));
  exports = define_exports(
    env,
    exports,
    "extract",
    Callback("extract", null_mut(), extract),
  );
  exports = define_exports(
    env,
    exports,
    "combine",
    Callback("combine", null_mut(), combine),
  );
  exports
}

static mut MODULE: native::module = native::module {
  nm_version: 3,
  nm_flags: 0,
  nm_filename: "libeasygif.node\0".as_ptr().cast(),
  nm_register_func: init,
  nm_modname: "EasyGIF 1.0.0\0".as_ptr().cast(),
  nm_priv: 0 as *mut c_void,
  reserved: [null_mut(); 4],
};

#[used]
#[cfg_attr(
  any(target_os = "linux", target_os = "android"),
  link_section = ".init_array"
)]
#[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
#[cfg_attr(target_os = "netbsd", link_section = ".init_array")]
#[cfg_attr(target_os = "openbsd", link_section = ".init_array")]
#[cfg_attr(target_os = "illumos", link_section = ".init_array")]
#[cfg_attr(
  any(target_os = "macos", target_os = "ios"),
  link_section = "__DATA_CONST,__mod_init_func"
)]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
static REGISTER: extern "C" fn() = {
  #[cfg_attr(
    any(target_os = "linux", target_os = "android"),
    link_section = ".text.startup"
  )]
  extern "C" fn register() {
    unsafe { native::module_register(&mut MODULE) };
  }
  register
};
