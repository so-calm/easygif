#![allow(non_camel_case_types)]

use std::{
  borrow::Cow,
  ffi::{c_char, c_int, c_uint, c_void, CStr, CString},
  ptr::{self, null_mut},
  slice::from_raw_parts,
};

use crate::Result;

pub(crate) type env = *mut c_void;
pub(crate) type deferred = *mut c_void;
pub(crate) type value = *mut c_void;
pub(crate) type callback_info = *mut c_void;
pub(crate) type async_work = *mut c_void;
pub(crate) type nref = value;

pub(crate) type addon_register_func = extern "C" fn(env: env, exports: value) -> value;
pub(crate) type callback = extern "C" fn(env: env, info: callback_info) -> value;
pub(crate) type async_execute_callback = extern "C" fn(env: env, data: *mut c_void);
pub(crate) type async_complete_callback =
  extern "C" fn(env: env, status: status, data: *mut c_void);

pub(crate) type finalize =
  extern "C" fn(env: env, finalize_data: *mut c_void, finalize_hint: *mut c_void);

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug)]
pub(crate) enum status {
  ok,
  invalid_arg,
  object_expected,
  string_expected,
  name_expected,
  function_expected,
  number_expected,
  boolean_expected,
  array_expected,
  generic_failure,
  pending_exception,
  cancelled,
  escape_called_twice,
  handle_scope_mismatch,
  callback_scope_mismatch,
  queue_full,
  closing,
  bigint_expected,
  date_expected,
  arraybuffer_expected,
  detachable_arraybuffer_expected,
  would_deadlock, // unused
}

#[repr(C)]
pub(crate) struct module {
  pub(crate) nm_version: c_int,
  pub(crate) nm_flags: c_uint,
  pub(crate) nm_filename: *const c_char,
  pub(crate) nm_register_func: addon_register_func,
  pub(crate) nm_modname: *const c_char,
  pub(crate) nm_priv: *mut c_void,
  pub(crate) reserved: [*mut c_void; 4],
}

unsafe impl Sync for module {}

#[repr(C)]
pub(crate) struct extended_error_info {
  pub(crate) error_message: *const c_char,
  pub(crate) engine_reserved: *mut c_void,
  pub(crate) engine_error_code: u32,
  pub(crate) error_code: status,
}

#[repr(C)]
#[allow(dead_code)]
pub(crate) enum property_attribute {
  Default = 0,
  Writable = 1 << 0,
  Enumerable = 1 << 1,
  Configurable = 1 << 2,

  /// Used with define_class to distinguish static properties
  /// from instance properties. Ignored by define_properties.
  Static = 1 << 10,

  /// Default for class methods.
  DefaultMethod = Self::Writable as isize | Self::Configurable as isize,

  /// Default for object properties, like in JS obj[prop].
  DefaultJsproperty =
    Self::Writable as isize | Self::Enumerable as isize | Self::Configurable as isize,
}

#[repr(C)]
pub(crate) struct property_descriptor {
  /// One of utf8name or name should be NULL.
  pub(crate) utf8name: *const c_char,
  /// One of utf8name or name should be NULL.
  pub(crate) name: value,

  pub(crate) method: Option<callback>,
  pub(crate) getter: Option<callback>,
  pub(crate) setter: Option<callback>,
  pub(crate) value: value,

  pub(crate) attributes: property_attribute,
  pub(crate) data: *mut c_void,
}

#[repr(C)]
#[derive(Default, Debug)]
#[allow(dead_code)]
pub(crate) enum valuetype {
  // ES6 types (corresponds to typeof)
  #[default]
  Undefined,
  Null,
  Boolean,
  Number,
  String,
  Symbol,
  Object,
  Function,
  External,
  Bigint,
}

#[cfg_attr(target_os = "windows", link(name = "lib\\node"))]
extern "C" {
  fn napi_module_register(module: *mut module);
  fn napi_create_function(
    env: env,
    utf8name: *const c_char,
    length: usize,
    cb: callback,
    data: *mut c_void,
    result: *mut value,
  ) -> status;
  fn napi_create_double(env: env, value: f64, result: *mut value) -> status;
  fn napi_get_last_error_info(env: env, result: *mut *const extended_error_info) -> status;
  fn napi_is_exception_pending(env: env, result: *mut bool) -> status;
  fn napi_throw_error(env: env, code: *const c_char, msg: *const c_char) -> status;
  fn napi_create_promise(env: env, deferred: *mut deferred, promise: *mut value) -> status;
  fn napi_resolve_deferred(env: env, deferred: deferred, resolution: value) -> status;
  fn napi_reject_deferred(env: env, deferred: deferred, rejection: value) -> status;
  fn napi_get_undefined(env: env, result: *mut value) -> status;
  fn napi_get_boolean(env: env, value: bool, result: *mut value) -> status;
  fn napi_get_null(env: env, result: *mut value) -> status;
  fn napi_create_async_work(
    env: env,
    async_resource: value,
    async_resource_name: value,
    execute: async_execute_callback,
    complete: async_complete_callback,
    data: *mut c_void,
    result: *mut async_work,
  ) -> status;
  fn napi_queue_async_work(env: env, work: async_work) -> status;
  fn napi_delete_async_work(env: env, work: async_work) -> status;
  fn napi_create_string_utf8(
    env: env,
    str: *const c_char,
    length: usize,
    result: *mut value,
  ) -> status;
  fn napi_get_cb_info(
    env: env,
    cbinfo: callback_info,
    argc: *mut usize,
    argv: *mut value,
    thisArg: *mut value,
    data: *mut *mut c_void,
  ) -> status;
  fn napi_get_value_uint32(env: env, value: value, result: *mut u32) -> status;
  fn napi_create_uint32(env: env, value: u32, result: *mut value) -> status;
  fn napi_get_value_string_utf8(
    env: env,
    value: value,
    buf: *mut c_char,
    bufsize: usize,
    result: *mut usize,
  ) -> status;
  fn napi_create_error(env: env, code: value, msg: value, result: *mut value) -> status;
  fn napi_create_buffer(
    env: env,
    size: usize,
    data: *mut *mut c_void,
    result: *mut value,
  ) -> status;
  fn napi_create_buffer_copy(
    env: env,
    length: usize,
    data: *const c_void,
    result_data: *mut *mut c_void,
    result: *mut value,
  ) -> status;
  fn napi_get_property(env: env, object: value, key: value, result: *mut value) -> status;
  fn napi_set_property(env: env, object: value, key: value, value: value) -> status;
  fn napi_define_class(
    env: env,
    utf8name: *const c_char,
    length: usize,
    constructor: callback,
    data: *mut c_void,
    property_count: usize,
    properties: *const property_descriptor,
    result: *mut value,
  ) -> status;
  fn napi_wrap(
    env: env,
    js_object: value,
    native_object: *mut c_void,
    finalize_cb: finalize,
    finalize_hint: *mut c_void,
    result: *mut nref,
  ) -> status;
  fn napi_unwrap(env: env, js_object: value, result: *mut *mut c_void) -> status;
  fn napi_create_reference(
    env: env,
    value: value,
    initial_refcount: u32,
    result: *mut nref,
  ) -> status;
  fn napi_get_global(env: env, result: *mut value) -> status;
  fn napi_get_named_property(
    env: env,
    object: value,
    utf8Name: *const c_char,
    result: *mut value,
  ) -> status;
  fn napi_set_named_property(
    env: env,
    object: value,
    utf8Name: *const c_char,
    value: value,
  ) -> status;
  fn napi_create_object(env: env, result: *mut value) -> status;
  fn napi_new_instance(
    env: env,
    cons: value,
    argc: usize,
    argv: *mut value,
    result: *mut value,
  ) -> status;
  fn napi_get_reference_value(env: env, nref: nref, result: *mut value) -> status;
  fn napi_get_value_double(env: env, value: value, result: *mut f64) -> status;
  // fn napi_call_function(
  //   env: env,
  //   recv: value,
  //   func: value,
  //   argc: usize,
  //   argv: *const value,
  //   result: *mut value,
  // ) -> status;
  fn napi_typeof(env: env, value: value, result: *mut valuetype) -> status;
  fn napi_is_array(env: env, value: value, result: *mut bool) -> status;
  fn napi_get_array_length(env: env, value: value, result: *mut u32) -> status;
  fn napi_is_buffer(env: env, value: value, result: *mut bool) -> status;
  fn napi_get_buffer_info(
    env: env,
    value: value,
    data: *mut *mut c_void,
    length: *mut usize,
  ) -> status;
}

macro_rules! unwrap_throw {
  ($env:expr, $expr:expr) => {
    match $expr {
      Ok(v) => v,
      Err(v) => {
        $crate::napi::native::throw_error($env, &v, &v);
        return ::std::ptr::null_mut();
      }
    }
  };
}

pub(crate) use unwrap_throw;

#[derive(Clone, Copy)]
pub(crate) struct Handle(pub(crate) value);

unsafe impl Send for Handle {}

unsafe impl Sync for Handle {}

pub(crate) trait FromNapi
where
  Self: Sized,
{
  fn from_napi(env: env, v: value) -> Result<Self>;
}

impl FromNapi for String {
  fn from_napi(env: env, v: value) -> Result<Self> {
    get_value_string(env, v)
  }
}

impl FromNapi for f64 {
  fn from_napi(env: env, v: value) -> Result<Self> {
    get_value_double(env, v)
  }
}

pub(crate) trait ToNapi {
  fn to_napi(&mut self, env: env) -> Result<value>;
}

impl ToNapi for usize {
  fn to_napi(&mut self, env: env) -> Result<value> {
    create_uint32(env, *self as u32)
  }
}

impl ToNapi for bool {
  fn to_napi(&mut self, env: env) -> Result<value> {
    Ok(boolean(env, *self))
  }
}

impl ToNapi for value {
  fn to_napi(&mut self, _env: env) -> Result<value> {
    Ok(*self)
  }
}

impl ToNapi for &str {
  fn to_napi(&mut self, env: env) -> Result<value> {
    create_string_utf8(env, self)
  }
}

impl ToNapi for f64 {
  fn to_napi(&mut self, env: env) -> Result<value> {
    create_double(env, *self)
  }
}

pub(crate) struct Callback<'a>(
  pub(crate) &'a str,
  pub(crate) *mut c_void,
  pub(crate) callback,
);

impl ToNapi for Callback<'_> {
  fn to_napi(&mut self, env: env) -> Result<value> {
    create_function(env, self.0, self.2, self.1)
  }
}

pub(crate) fn get_buffer_info(env: env, v: value) -> Result<&'static [u8]> {
  let mut data: *mut c_void = null_mut();
  let mut len = 0_usize;
  call(
    env,
    &unsafe { napi_get_buffer_info(env, v, &mut data, &mut len) },
    (),
  )?;
  Ok(unsafe { from_raw_parts(data.cast(), len) })
}

pub(crate) fn is_buffer(env: env, v: value) -> Result<bool> {
  let mut result = false;
  call(env, &unsafe { napi_is_buffer(env, v, &mut result) }, result)
}

pub(crate) fn is_array(env: env, v: value) -> Result<bool> {
  let mut result = false;
  call(env, &unsafe { napi_is_array(env, v, &mut result) }, result)
}

pub(crate) fn get_array_length(env: env, v: value) -> Result<usize> {
  let mut result = 0_u32;
  call(
    env,
    &unsafe { napi_get_array_length(env, v, &mut result) },
    result as usize,
  )
}

pub(crate) fn value_type(env: env, v: value) -> Result<valuetype> {
  let mut result: valuetype = valuetype::default();
  call(env, &unsafe { napi_typeof(env, v, &mut result) }, result)
}

// pub(crate) fn call_function(
//   env: env,
//   this: value,
//   func: value,
//   args: &mut [value],
// ) -> Result<value> {
//   let mut result: value = null_mut();
//   call(
//     env,
//     &unsafe { napi_call_function(env, this, func, args.len(), args.as_ptr(), &mut result) },
//     result,
//   )
// }

pub(crate) fn new_instance(env: env, cons: value, args: &mut [value]) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_new_instance(env, cons, args.len(), args.as_mut_ptr(), &mut result) },
    result,
  )
}

pub(crate) fn get_reference_value(env: env, nref: nref) -> Result<value> {
  let mut value: value = null_mut();
  call(
    env,
    &unsafe { napi_get_reference_value(env, nref, &mut value) },
    value,
  )
}

pub(crate) fn get_value_double(env: env, v: value) -> Result<f64> {
  let mut result = 0_f64;
  call(
    env,
    &unsafe { napi_get_value_double(env, v, &mut result) },
    result,
  )
}

pub(crate) fn get_property(env: env, object: value, mut key: impl ToNapi) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_get_property(env, object, key.to_napi(env)?, &mut result) },
    result,
  )
}

pub(crate) fn get_named_property(env: env, object: value, name: &str) -> Result<value> {
  let utf8name = CString::new(name).unwrap();
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_get_named_property(env, object, utf8name.as_ptr(), &mut result) },
    result,
  )
}

pub(crate) fn create_object(env: env) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_create_object(env, &mut result) },
    result,
  )
}

pub(crate) fn set_property(
  env: env,
  object: value,
  mut key: impl ToNapi,
  mut value: impl ToNapi,
) -> Result<value> {
  call(
    env,
    &unsafe { napi_set_property(env, object, key.to_napi(env)?, value.to_napi(env)?) },
    object,
  )
}

pub(crate) fn set_named_property(
  env: env,
  object: value,
  name: &str,
  mut value: impl ToNapi,
) -> Result<()> {
  let utf8name = CString::new(name).unwrap();
  call(
    env,
    &unsafe { napi_set_named_property(env, object, utf8name.as_ptr(), value.to_napi(env)?) },
    (),
  )
}

pub(crate) fn create_reference(env: env, value: value) -> Result<nref> {
  let mut result: nref = null_mut();
  call(
    env,
    &unsafe { napi_create_reference(env, value, 1, &mut result) },
    result,
  )
}

pub(crate) fn wrap(
  env: env,
  js_object: value,
  native_object: *mut c_void,
  finalize_cb: finalize,
  finalize_hint: *mut c_void,
  result: *mut nref,
) -> Result<*mut nref> {
  call(
    env,
    &unsafe {
      napi_wrap(
        env,
        js_object,
        native_object,
        finalize_cb,
        finalize_hint,
        result,
      )
    },
    result,
  )
}

pub(crate) fn unwrap(env: env, js_object: value) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_unwrap(env, js_object, &mut result) },
    result,
  )
}

pub(crate) fn define_class(
  env: env,
  name: &str,
  constructor: callback,
  data: *mut c_void,
  properties: &[property_descriptor],
) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe {
      napi_define_class(
        env,
        name.as_ptr().cast(),
        name.len(),
        constructor,
        data,
        properties.len(),
        properties.as_ptr(),
        &mut result,
      )
    },
    result,
  )
}

pub(crate) fn get_value_uint32(env: env, v: value) -> Result<u32> {
  let mut result = 0_u32;
  call(
    env,
    &unsafe { napi_get_value_uint32(env, v, &mut result) },
    result,
  )
}

pub(crate) fn create_uint32(env: env, v: u32) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_create_uint32(env, v, &mut result) },
    result,
  )
}

pub(crate) fn get_value_string(env: env, value: value) -> Result<String> {
  let mut str_len = 0_usize;
  call(
    env,
    &unsafe { napi_get_value_string_utf8(env, value, null_mut(), 0, &mut str_len) },
    (),
  )?;
  let mut result = String::with_capacity(str_len);
  unsafe { result.as_mut_vec().set_len(str_len) };
  call(
    env,
    &unsafe {
      napi_get_value_string_utf8(
        env,
        value,
        result.as_mut_ptr().cast(),
        str_len + 1,
        null_mut(),
      )
    },
    result,
  )
}

pub(crate) fn get_cb_info(
  env: env,
  info: callback_info,
) -> Result<(value, Vec<value>, *mut c_void)> {
  let mut len = 255_usize;
  let mut args: [value; 255] = [null_mut(); 255];
  let mut this: value = null_mut();
  let mut data: *mut c_void = null_mut();
  call(
    env,
    &unsafe { napi_get_cb_info(env, info, &mut len, args.as_mut_ptr(), &mut this, &mut data) },
    (),
  )?;
  Ok((this, Vec::from(&args[..len]), data))
}

pub(crate) fn get_last_error_info(env: env) -> *const extended_error_info {
  let mut error_info: *const extended_error_info = ptr::null();
  unsafe { napi_get_last_error_info(env, &mut error_info) };
  error_info
}

pub(crate) fn is_exception_pending(env: env) -> bool {
  let mut pending = false;
  if let status::ok = unsafe { napi_is_exception_pending(env, &mut pending) } {
    pending
  } else {
    throw_error(
      env,
      "Failed to resolve last pending exception",
      "Failed to resolve last pending exception",
    );
    true
  }
}

pub(crate) fn throw_error(env: env, code: &str, msg: &str) {
  let code = CString::new(code).unwrap();
  let msg = CString::new(msg).unwrap();
  if !matches!(
    unsafe { napi_throw_error(env, code.as_ptr(), msg.as_ptr()) },
    status::ok
  ) {
    panic!("Failed to throw a napi error");
  }
}

pub(crate) fn call<T>(env: env, status: &status, value: T) -> Result<T> {
  if !matches!(status, status::ok) {
    let error_info = get_last_error_info(env);
    let pending = is_exception_pending(env);
    if !pending {
      return Err(
        if error_info.is_null() || unsafe { &*error_info }.error_message.is_null() {
          Cow::Borrowed("Empty error message")
        } else {
          Cow::Owned(
            unsafe { CStr::from_ptr((*error_info).error_message) }
              .to_string_lossy()
              .to_string(),
          )
        },
      );
    }
  }
  Ok(value)
}

pub(crate) fn module_register(module: &mut module) {
  unsafe { napi_module_register(module) }
}

pub(crate) fn create_buffer(env: env, size: usize) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_create_buffer(env, size, null_mut(), &mut result) },
    result,
  )
}

pub(crate) fn create_buffer_copy(env: env, buffer: &[u8]) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe {
      napi_create_buffer_copy(
        env,
        buffer.len(),
        buffer.as_ptr().cast(),
        null_mut(),
        &mut result,
      )
    },
    result,
  )
}

pub(crate) fn create_function(
  env: env,
  name: &str,
  cb: callback,
  data: *mut c_void,
) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_create_function(env, name.as_ptr().cast(), name.len(), cb, data, &mut result) },
    result,
  )
}

pub(crate) fn create_double(env: env, value: f64) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_create_double(env, value, &mut result) },
    result,
  )
}

pub(crate) fn create_error(env: env, code: value, msg: value) -> Result<value> {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_create_error(env, code, msg, &mut result) },
    result,
  )
}

pub(crate) fn create_promise(env: env) -> Result<(value, deferred)> {
  let mut deferred: deferred = null_mut();
  let mut promise: value = null_mut();
  call(
    env,
    &unsafe { napi_create_promise(env, &mut deferred, &mut promise) },
    (promise, deferred),
  )
}

pub(crate) fn resolve_deferred(env: env, deferred: deferred, resolution: value) -> Result<()> {
  call(
    env,
    &unsafe { napi_resolve_deferred(env, deferred, resolution) },
    (),
  )
}

pub(crate) fn reject_deferred(env: env, deferred: deferred, rejection: value) -> Result<()> {
  let rejection = create_error(env, rejection, rejection)?;
  call(
    env,
    &unsafe { napi_reject_deferred(env, deferred, rejection) },
    (),
  )
}

pub(crate) fn global(env: env) -> value {
  let mut result: value = null_mut();
  call(env, &unsafe { napi_get_global(env, &mut result) }, result)
    .unwrap_or_else(|_| panic!("Failed to resolve napi global value"))
}

pub(crate) fn undefined(env: env) -> value {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_get_undefined(env, &mut result) },
    result,
  )
  .unwrap_or_else(|_| panic!("Failed to resolve napi undefined value"))
}

pub(crate) fn boolean(env: env, b: bool) -> value {
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_get_boolean(env, b, &mut result) },
    result,
  )
  .unwrap_or_else(|_| panic!("Failed to resolve napi undefined value"))
}

pub(crate) fn null(env: env) -> value {
  let mut result: value = null_mut();
  call(env, &unsafe { napi_get_null(env, &mut result) }, result)
    .unwrap_or_else(|_| panic!("Failed to resolve napi null value"))
}

pub(crate) fn create_string_utf8(env: env, str: &str) -> Result<value> {
  let len = str.len();
  let Ok(str) = CString::new(str) else {
    return Err(Cow::Borrowed("Failed to create string_utf8"));
  };
  let mut result: value = null_mut();
  call(
    env,
    &unsafe { napi_create_string_utf8(env, str.as_ptr(), len, &mut result) },
    result,
  )
}

pub(crate) fn create_async_work(
  env: env,
  resource: &str,
  execute: async_execute_callback,
  complete: async_complete_callback,
  data: *mut c_void,
) -> Result<value> {
  let resource = create_string_utf8(env, resource)?;
  let mut result: value = null_mut();
  call(
    env,
    &unsafe {
      napi_create_async_work(
        env,
        null_mut(),
        resource,
        execute,
        complete,
        data,
        &mut result,
      )
    },
    result,
  )
}

pub(crate) fn queue_async_work(env: env, work: async_work) -> Result<()> {
  call(env, &unsafe { napi_queue_async_work(env, work) }, ())
}

pub(crate) fn delete_async_work(env: env, work: async_work) -> Result<()> {
  call(env, &unsafe { napi_delete_async_work(env, work) }, ())
}
