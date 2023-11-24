use std::{
  alloc::{dealloc, Layout},
  borrow::Cow,
  ffi::c_void,
  io::{ErrorKind, Read},
  process::{ChildStdout, Command, Stdio},
  ptr::{addr_of_mut, drop_in_place, null, null_mut},
};

use crate::{
  napi::native::{self, FromNapi, ToNapi},
  Result,
};

use super::Probe;

pub(crate) struct Extract {
  pub(crate) stdout: ChildStdout,
  pub(crate) buf: Vec<u8>,
}

impl ToNapi for &mut Extract {
  fn to_napi(&mut self, env: native::env) -> Result<native::value> {
    let extract_struct = native::get_reference_value(env, extract_struct(env)?)?;
    let instance = native::new_instance(env, extract_struct, &mut [])?;
    native::wrap(
      env,
      instance,
      addr_of_mut!(**self).cast(),
      extract_finalize,
      null_mut(),
      null_mut(),
    )?;
    Ok(instance)
  }
}

pub(crate) fn extract_native(probe: &Probe) -> Result<Extract> {
  let args = [
    "-i", &probe.src, // Input parameters
    "-f", "rawvideo", "-pix_fmt", "rgba", "-", // Output parameters
  ];
  Ok(Extract {
    stdout: Command::new("./bin/ffmpeg")
      .args(args)
      .stdin(Stdio::null())
      .stdout(Stdio::piped())
      .stderr(Stdio::null())
      .spawn()
      .or(
        Command::new("ffmpeg")
          .args(args)
          .stdin(Stdio::null())
          .stdout(Stdio::piped())
          .stderr(Stdio::piped())
          .spawn(),
      )
      .map_err(|v| Cow::Owned(format!("Generic: {v}")))?
      .stdout
      .take()
      .ok_or(Cow::Borrowed("Failed to extract output handle"))?,
    buf: vec![0_u8; probe.width as usize * probe.height as usize * 4_usize],
  })
}

extern "C" fn extract_finalize(_env: native::env, data: *mut c_void, _hint: *mut c_void) {
  unsafe { drop_in_place(data) };
  unsafe {
    dealloc(data.cast(), Layout::new::<Extract>());
  };
}

extern "C" fn extract_constructor(
  _env: native::env,
  _info: native::callback_info,
) -> native::value {
  null_mut()
}

extern "C" fn extract_next(env: native::env, info: native::callback_info) -> native::value {
  let (_, _, data) = native::unwrap_throw!(env, native::get_cb_info(env, info));
  let extract = unsafe { &mut *(data as *mut Extract) };
  let result = native::unwrap_throw!(env, native::create_object(env));
  native::unwrap_throw!(
    env,
    native::set_named_property(env, result, "value", native::undefined(env))
  );
  native::unwrap_throw!(env, native::set_named_property(env, result, "done", false));
  if let Err(v) = extract.stdout.read_exact(&mut extract.buf) {
    if let ErrorKind::UnexpectedEof = v.kind() {
      native::unwrap_throw!(env, native::set_named_property(env, result, "done", true));
      return result;
    }
    native::throw_error(env, &v.to_string(), &v.to_string());
    null_mut()
  } else {
    native::unwrap_throw!(
      env,
      native::set_named_property(
        env,
        result,
        "value",
        native::unwrap_throw!(env, native::create_buffer_copy(env, &extract.buf))
      )
    );
    result
  }
}

extern "C" fn extract_iter(env: native::env, info: native::callback_info) -> native::value {
  let (this, _, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
  let inner = native::unwrap_throw!(env, native::unwrap(env, this));
  let object = native::unwrap_throw!(env, native::create_object(env));
  native::unwrap_throw!(
    env,
    native::set_named_property(
      env,
      object,
      "next",
      native::Callback("next", inner, extract_next)
    )
  );
  object
}

pub(crate) fn extract_struct(env: native::env) -> Result<native::value> {
  static mut EXTRACT_STRUCT: native::value = null_mut();
  if unsafe { EXTRACT_STRUCT }.is_null() {
    let global = native::global(env);
    let symbol = native::get_named_property(env, global, "Symbol")?;
    let symbol_iterator = native::get_named_property(env, symbol, "iterator")?;
    let extract_struct = native::define_class(
      env,
      "Extract",
      extract_constructor,
      null_mut(),
      &[native::property_descriptor {
        utf8name: null(),
        name: symbol_iterator,
        method: Some(extract_iter),
        getter: None,
        setter: None,
        value: null_mut(),
        attributes: native::property_attribute::Enumerable,
        data: null_mut(),
      }],
    )?;
    unsafe { EXTRACT_STRUCT = native::create_reference(env, extract_struct)? };
  }
  Ok(unsafe { EXTRACT_STRUCT })
}

struct ExtractContext {
  probe: &'static mut Probe,
  async_work: native::async_work,
  deferred: native::deferred,
  result: Result<&'static mut Extract>,
}

extern "C" fn extract_execute(_env: native::env, data: *mut c_void) {
  let ctx = unsafe { &mut *data.cast::<ExtractContext>() };
  ctx.result = extract_native(ctx.probe).map(Box::new).map(Box::leak);
}

extern "C" fn extract_complete(env: native::env, _status: native::status, data: *mut c_void) {
  let ctx = unsafe { &mut *data.cast::<ExtractContext>() };
  let _ = match &mut ctx.result {
    Ok(v) => v
      .to_napi(env)
      .and_then(|v| native::resolve_deferred(env, ctx.deferred, v)),
    Err(v) => {
      native::create_string_utf8(env, v).and_then(|v| native::reject_deferred(env, ctx.deferred, v))
    }
  };

  let _ = native::delete_async_work(env, ctx.async_work);
  unsafe { drop_in_place(data) };
  unsafe {
    dealloc(data.cast(), Layout::new::<ExtractContext>());
  };
}

fn extract_promise(
  env: native::env,
  info: native::callback_info,
  deferred: native::deferred,
) -> Result<()> {
  let (_, args, _) = native::get_cb_info(env, info)?;
  let mut args = args.into_iter();
  let probe = args
    .next()
    .ok_or(Cow::Borrowed("Function call expects exactly one argument"))
    .and_then(|v| {
      <&mut Probe>::from_napi(env, v)
        .map_err(|_| Cow::Borrowed("The first argument is expected to be of type `Probe`"))
    })?;

  let result = Box::leak(Box::new(ExtractContext {
    probe,
    async_work: null_mut(),
    deferred,
    result: Err(Cow::Borrowed("")),
  }));
  result.async_work = native::create_async_work(
    env,
    "Extract GIF frames from `src`",
    extract_execute,
    extract_complete,
    addr_of_mut!(*result).cast(),
  )
  .map_err(|_| Cow::Borrowed("Failed to create an async_work"))?;
  native::queue_async_work(env, result.async_work)
    .map_err(|_| Cow::Borrowed("Failed to queue the async_work"))?;
  Ok(())
}

pub(crate) extern "C" fn extract(env: native::env, info: native::callback_info) -> native::value {
  let (promise, deferred) = native::unwrap_throw!(env, native::create_promise(env));

  if !deferred.is_null() {
    if let Err(v) = extract_promise(env, info, deferred) {
      native::unwrap_throw!(
        env,
        native::create_string_utf8(env, &v).and_then(|v| native::reject_deferred(env, deferred, v))
      );
    }
  }

  promise
}
