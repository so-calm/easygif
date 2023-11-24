use std::{
  alloc::{dealloc, Layout},
  borrow::Cow,
  ffi::c_void,
  mem::swap,
  process::Command,
  ptr::{addr_of_mut, drop_in_place, null_mut},
};

use crate::{
  napi::native::{self, FromNapi, ToNapi},
  Result,
};

#[derive(Debug)]
pub(crate) struct Probe {
  pub(crate) src: String,
  pub(crate) width: f64,
  pub(crate) height: f64,
  pub(crate) fps: f64,
}

impl FromNapi for &mut Probe {
  fn from_napi(env: crate::napi::native::env, v: crate::napi::native::value) -> Result<Self> {
    let probe = unsafe { &mut *(native::unwrap(env, v)? as *mut Probe) };
    Ok(probe)
  }
}

impl ToNapi for &'static mut Probe {
  fn to_napi(&mut self, env: native::env) -> Result<native::value> {
    let probe_struct = native::get_reference_value(env, probe_struct(env)?)?;
    let instance = native::new_instance(env, probe_struct, &mut [])?;
    native::wrap(
      env,
      instance,
      addr_of_mut!(**self).cast(),
      probe_finalize,
      null_mut(),
      null_mut(),
    )?;
    Ok(instance)
  }
}

pub(crate) fn probe_native(src: String) -> Result<Probe> {
  let metadata = Command::new("ffprobe")
    .args([
      "-of",
      "compact=p=0:s=,:nk=1",
      "-show_entries",
      "stream=width,height,r_frame_rate",
      &src,
    ])
    .output()
    .map_err(|v| Cow::Owned(format!("Failed to probe the asset: {v}")))?;

  if metadata.stdout.is_empty() {
    return Err(Cow::Borrowed("File not found"));
  }

  let mut w = 0_f64;
  let mut h = -1_f64;
  let mut fps_dividend = -1_f64;
  let mut fps_divisor = -1_f64;
  for b in metadata.stdout {
    match b {
      b'\r' | b'\n' => break,
      b',' | b'/' => {
        fps_divisor += f64::from(fps_dividend > -1_f64 && fps_divisor < 0_f64);
        fps_dividend += f64::from(h > -1_f64 && fps_dividend < 0_f64);
        h += f64::from(h < 0_f64);
        continue;
      }
      _ => {
        let n = f64::from(b) - 48_f64;
        fps_divisor = f64::from(fps_divisor > -1_f64) * (fps_divisor * 10_f64 + n + 1_f64) - 1_f64;
        fps_dividend = f64::from(fps_divisor > -1_f64) * (fps_dividend + 1_f64) - 1_f64
          + f64::from(fps_divisor < 0_f64 && fps_dividend > -1_f64)
            * (fps_dividend * 10_f64 + n + 1_f64);
        h = f64::from(fps_dividend > -1_f64) * (h + 1_f64) - 1_f64
          + f64::from(fps_dividend < 0_f64 && h > -1_f64) * (h * 10_f64 + n + 1_f64);
        w = f64::from(h > -1_f64) * w + f64::from(h < 0_f64) * (w * 10_f64 + n);
      }
    }
  }

  Ok(Probe {
    src,
    width: w,
    height: h,
    fps: fps_dividend / fps_divisor,
  })
}

extern "C" fn probe_finalize(_env: native::env, data: *mut c_void, _hint: *mut c_void) {
  unsafe { drop_in_place(data) };
  unsafe {
    dealloc(data.cast(), Layout::new::<Probe>());
  };
}

extern "C" fn probe_constructor(_env: native::env, _value: native::value) -> native::value {
  null_mut()
}

// extern "C" fn probe_inspect(env: native::env, info: native::value) -> native::value {
//   let (this, _, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
//   let probe =
//     unsafe { &mut *(native::unwrap_throw!(env, native::unwrap(env, this)) as *mut Probe) };
//   native::unwrap_throw!(
//     env,
//     format!(
//       "Probe [{w}x{h} {fps}fps] ({src:?})",
//       w = probe.width,
//       h = probe.height,
//       fps = probe.fps,
//       src = probe.src,
//     )
//     .as_str()
//     .to_napi(env)
//   )
// }

extern "C" fn probe_src(env: native::env, info: native::value) -> native::value {
  let (this, _, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
  let probe =
    unsafe { &mut *(native::unwrap_throw!(env, native::unwrap(env, this)) as *mut Probe) };
  native::unwrap_throw!(env, probe.src.as_str().to_napi(env))
}

extern "C" fn probe_width(env: native::env, info: native::value) -> native::value {
  let (this, _, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
  let probe =
    unsafe { &mut *(native::unwrap_throw!(env, native::unwrap(env, this)) as *mut Probe) };
  native::unwrap_throw!(env, probe.width.to_napi(env))
}

extern "C" fn probe_height(env: native::env, info: native::value) -> native::value {
  let (this, _, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
  let probe =
    unsafe { &mut *(native::unwrap_throw!(env, native::unwrap(env, this)) as *mut Probe) };
  native::unwrap_throw!(env, probe.height.to_napi(env))
}

extern "C" fn probe_fps(env: native::env, info: native::value) -> native::value {
  let (this, _, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
  let probe =
    unsafe { &mut *(native::unwrap_throw!(env, native::unwrap(env, this)) as *mut Probe) };
  native::unwrap_throw!(env, probe.fps.to_napi(env))
}

pub(crate) fn probe_struct(env: native::env) -> Result<native::value> {
  static mut PROBE_STRUCT: native::value = null_mut();
  if unsafe { PROBE_STRUCT }.is_null() {
    let probe_struct = native::define_class(
      env,
      "Probe",
      probe_constructor,
      null_mut(),
      &[
        native::property_descriptor {
          utf8name: null_mut(),
          name: native::create_string_utf8(env, "src")?,
          method: None,
          getter: Some(probe_src),
          setter: None,
          value: null_mut(),
          attributes: native::property_attribute::Enumerable,
          data: null_mut(),
        },
        native::property_descriptor {
          utf8name: null_mut(),
          name: native::create_string_utf8(env, "width")?,
          method: None,
          getter: Some(probe_width),
          setter: None,
          value: null_mut(),
          attributes: native::property_attribute::Enumerable,
          data: null_mut(),
        },
        native::property_descriptor {
          utf8name: null_mut(),
          name: native::create_string_utf8(env, "height")?,
          method: None,
          getter: Some(probe_height),
          setter: None,
          value: null_mut(),
          attributes: native::property_attribute::Enumerable,
          data: null_mut(),
        },
        native::property_descriptor {
          utf8name: null_mut(),
          name: native::create_string_utf8(env, "fps")?,
          method: None,
          getter: Some(probe_fps),
          setter: None,
          value: null_mut(),
          attributes: native::property_attribute::Enumerable,
          data: null_mut(),
        },
      ],
    )?;
    unsafe { PROBE_STRUCT = native::create_reference(env, probe_struct)? };
  }
  Ok(unsafe { PROBE_STRUCT })
}

struct ProbeContext {
  s: String,
  async_work: native::async_work,
  deferred: native::deferred,
  result: Result<&'static mut Probe>,
}

extern "C" fn probe_execute(_env: native::env, data: *mut c_void) {
  let ctx = unsafe { &mut *data.cast::<ProbeContext>() };
  let mut src = String::new();
  swap(&mut ctx.s, &mut src);
  ctx.result = probe_native(src).map(Box::new).map(Box::leak);
}

extern "C" fn probe_complete(env: native::env, _status: native::status, data: *mut c_void) {
  let ctx = unsafe { &mut *data.cast::<ProbeContext>() };
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
    dealloc(data.cast(), Layout::new::<ProbeContext>());
  };
}

fn probe_promise(
  env: native::env,
  info: native::callback_info,
  deferred: native::deferred,
) -> Result<()> {
  let (_, args, _) = native::get_cb_info(env, info)?;
  let mut args = args.into_iter();
  let s = args
    .next()
    .ok_or(Cow::Borrowed("Function call expects exactly one argument"))
    .and_then(|v| {
      String::from_napi(env, v)
        .map_err(|_| Cow::Borrowed("The first argument is expected to be of type `string`"))
    })?;

  let result = Box::leak(Box::new(ProbeContext {
    s,
    async_work: null_mut(),
    deferred,
    result: Err(Cow::Borrowed("")),
  }));
  result.async_work = native::create_async_work(
    env,
    "Probe GIF file from `src`",
    probe_execute,
    probe_complete,
    addr_of_mut!(*result).cast(),
  )
  .map_err(|_| Cow::Borrowed("Failed to create an async_work"))?;
  native::queue_async_work(env, result.async_work)
    .map_err(|_| Cow::Borrowed("Failed to queue the async_work"))?;
  Ok(())
}

pub(crate) extern "C" fn probe(env: native::env, info: native::callback_info) -> native::value {
  let (promise, deferred) = native::unwrap_throw!(env, native::create_promise(env));

  if !deferred.is_null() {
    if let Err(v) = probe_promise(env, info, deferred) {
      native::unwrap_throw!(
        env,
        native::create_string_utf8(env, &v).and_then(|v| native::reject_deferred(env, deferred, v))
      );
    }
  }

  promise
}
