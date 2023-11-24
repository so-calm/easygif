use std::{
  alloc::{dealloc, Layout},
  borrow::Cow,
  ffi::c_void,
  io::{Read, Write},
  process::{ChildStdin, ChildStdout, Command, Stdio},
  ptr::{addr_of_mut, drop_in_place, null_mut},
};

use crate::{
  napi::native::{self, FromNapi, ToNapi},
  Result,
};

#[derive(Default, Debug)]
#[repr(u8)]
pub(crate) enum Repeat {
  #[default]
  Infinite = 0,
  Once = 1,
  Exact(u16),
}

impl FromNapi for Repeat {
  fn from_napi(env: crate::napi::native::env, v: crate::napi::native::value) -> Result<Self> {
    let v = native::get_value_uint32(env, v)
      .and_then(|v| u16::try_from(v).map_err(|v| Cow::Owned(v.to_string())))?;
    Ok(match v {
      0 => Self::Infinite,
      1 => Self::Once,
      v => Self::Exact(v),
    })
  }
}

impl ToNapi for Repeat {
  fn to_napi(&mut self, env: native::env) -> Result<native::value> {
    native::create_uint32(
      env,
      u32::from(match self {
        Self::Infinite => 0_u16,
        Self::Once => 1_u16,
        Self::Exact(v) => *v,
      }),
    )
  }
}

#[derive(Default)]
pub(crate) struct CombineOptions {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) fps: f64,
  pub(crate) scale: Option<(u32, u32)>,
  pub(crate) repeat: Repeat,
}

impl FromNapi for CombineOptions {
  fn from_napi(env: native::env, v: native::value) -> Result<Self> {
    if let native::valuetype::Object = native::value_type(env, v)? {
      let width = native::get_named_property(env, v, "width")
        .and_then(|v| native::get_value_uint32(env, v))?;
      let height = native::get_named_property(env, v, "height")
        .and_then(|v| native::get_value_uint32(env, v))?;
      let fps =
        native::get_named_property(env, v, "fps").and_then(|v| native::get_value_double(env, v))?;

      let vscale = native::get_named_property(env, v, "scale")?;
      let scale = match native::value_type(env, vscale)? {
        native::valuetype::Object
          if native::is_array(env, vscale)? && native::get_array_length(env, vscale)? == 2 =>
        {
          Some((
            native::get_property(env, vscale, 0_usize)
              .and_then(|v| native::get_value_uint32(env, v))?,
            native::get_property(env, vscale, 1_usize)
              .and_then(|v| native::get_value_uint32(env, v))?,
          ))
        }
        native::valuetype::Null | native::valuetype::Undefined => None,
        _ => return Err(Cow::Borrowed("Invalid `CombineOptions`.`scale` property")),
      };

      let vrepeat = native::get_named_property(env, v, "repeat")?;
      let repeat = match native::value_type(env, vrepeat)? {
        native::valuetype::Number => Repeat::from_napi(env, vrepeat)?,
        native::valuetype::Null | native::valuetype::Undefined => Repeat::default(),
        _ => return Err(Cow::Borrowed("Invalid `CombineOptions`.`repeat` property")),
      };

      Ok(Self {
        width,
        height,
        fps,
        scale,
        repeat,
      })
    } else {
      Err(Cow::Borrowed("Invalid CombineOptions"))
    }
  }
}

pub(crate) struct Combine {
  pub(crate) buf_size: usize,
  pub(crate) stdin: Option<ChildStdin>,
  pub(crate) stdout: ChildStdout,
}

impl ToNapi for &mut Combine {
  fn to_napi(&mut self, env: native::env) -> Result<native::value> {
    let combine_struct = native::get_reference_value(env, combine_struct(env)?)?;
    let instance = native::new_instance(env, combine_struct, &mut [])?;
    native::wrap(
      env,
      instance,
      addr_of_mut!(**self).cast(),
      combine_finalize,
      null_mut(),
      null_mut(),
    )?;
    Ok(instance)
  }
}

extern "C" fn combine_finalize(_env: native::env, data: *mut c_void, _hint: *mut c_void) {
  unsafe { drop_in_place(data) };
  unsafe {
    dealloc(data.cast(), Layout::new::<Combine>());
  };
}

pub(crate) fn combine_native(options: &CombineOptions) -> Result<Combine> {
  let scale = options.scale.unwrap_or((options.width, options.height));
  let mut child = Command::new("ffmpeg")
    .args([
      "-y", // Force replace output
      // Input parameters
      "-s",
      &format!("{w}x{h}", w = options.width, h = options.height),
      "-f",
      "rawvideo",
      "-pix_fmt",
      "rgba",
      "-r",
      &format!("{fps}", fps = options.fps),
      "-i",
      "-",
      // Output parameters
      "-f",
      "gif",
      "-loop",
      &match options.repeat {
        Repeat::Infinite => 0,
        Repeat::Once => 1,
        Repeat::Exact(v) => v,
      }
      .to_string(),
      "-filter_complex",
      &format!(
        concat![
          "scale={w}x{h}",
          ":flags=lanczos,split[s0][s1]",
          ";[s0]palettegen=max_colors=32[p];[s1][p]paletteuse=dither=bayer"
        ],
        w = scale.0,
        h = scale.1,
      ),
      "-",
    ])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::null())
    .spawn()
    .map_err(|_| Cow::Borrowed("Failed to spawn combine channel"))?;
  Ok(Combine {
    buf_size: scale.0 as usize * scale.1 as usize * 4_usize,
    stdin: Some(
      child
        .stdin
        .take()
        .ok_or(Cow::Borrowed("Failed to take the input handle"))?,
    ),
    stdout: child
      .stdout
      .take()
      .ok_or(Cow::Borrowed("Failed to take the input handle"))?,
  })
}

extern "C" fn combine_constructor(_env: native::env, _value: native::value) -> native::value {
  null_mut()
}

// extern "C" fn combine_inspect(env: native::env, info: native::value) -> native::value {
//   let (this, _, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
//   let probe =
//     unsafe { &mut *(native::unwrap_throw!(env, native::unwrap(env, this)) as *mut Probe) };
//   native::unwrap_throw!(
//     env,
//     format!(
//       "Combine [{w}x{h} {fps}fps] ({src:?})",
//       w = probe.width,
//       h = probe.height,
//       fps = probe.fps,
//       src = probe.src,
//     )
//     .as_str()
//     .to_napi(env)
//   )
// }

extern "C" fn combine_alloc(env: native::env, info: native::callback_info) -> native::value {
  let (this, _, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
  let ctx =
    unsafe { &mut *(native::unwrap_throw!(env, native::unwrap(env, this)) as *mut Combine) };
  native::unwrap_throw!(env, native::create_buffer(env, ctx.buf_size))
}

extern "C" fn combine_write(env: native::env, info: native::callback_info) -> native::value {
  let (this, args, _) = native::unwrap_throw!(env, native::get_cb_info(env, info));
  let combine =
    unsafe { &mut *(native::unwrap_throw!(env, native::unwrap(env, this)) as *mut Combine) };

  let Some(v) = args.into_iter().next() else {
    let msg = "Function call expects exactly one argument";
    native::throw_error(env, msg, msg);
    return null_mut();
  };

  if !native::unwrap_throw!(env, native::is_buffer(env, v)) {
    let msg = "The first argument is expected to be of type `Buffer`";
    native::throw_error(env, msg, msg);
    return null_mut();
  }

  let buf = native::unwrap_throw!(env, native::get_buffer_info(env, v));
  if buf.len() != combine.buf_size {
    let msg = format!(
      "The buffer is expected to be of size {buf_size}",
      buf_size = combine.buf_size,
    );
    native::throw_error(env, &msg, &msg);
    return null_mut();
  }

  let Some(stdin) = &mut combine.stdin else {
    let msg = "The Combine instance has already finished";
    native::throw_error(env, msg, msg);
    return null_mut();
  };

  native::unwrap_throw!(
    env,
    stdin.write_all(buf).map_err(|v| Cow::Owned(v.to_string()))
  );
  native::null(env)
}

fn combine_finish_native(combine: &'static mut Combine) -> Result<Vec<u8>> {
  let stdin = combine
    .stdin
    .take()
    .ok_or(Cow::Borrowed("The Combine instance has already finished"))?;
  drop(stdin);

  let mut buf = Vec::<u8>::new();
  combine
    .stdout
    .read_to_end(&mut buf)
    .map_err(|v| Cow::Owned(v.to_string()))?;
  Ok(buf)
}

struct CombineFinishContext {
  combine: &'static mut Combine,
  async_work: native::async_work,
  deferred: native::deferred,
  result: Result<Vec<u8>>,
}

extern "C" fn combine_finish_execute(_env: native::env, data: *mut c_void) {
  let ctx = unsafe { &mut *data.cast::<CombineFinishContext>() };
  ctx.result = combine_finish_native(ctx.combine);
}

extern "C" fn combine_finish_complete(
  env: native::env,
  _status: native::status,
  data: *mut c_void,
) {
  let ctx = unsafe { &mut *data.cast::<CombineFinishContext>() };
  let _ = match &ctx.result {
    Ok(v) => native::create_buffer_copy(env, v)
      .and_then(|v| native::resolve_deferred(env, ctx.deferred, v)),
    Err(v) => {
      native::create_string_utf8(env, v).and_then(|v| native::reject_deferred(env, ctx.deferred, v))
    }
  };

  let _ = native::delete_async_work(env, ctx.async_work);
  unsafe { drop_in_place(data) };
  unsafe {
    dealloc(data.cast(), Layout::new::<CombineFinishContext>());
  };
}

fn combine_finish_promise(
  env: native::env,
  info: native::callback_info,
  deferred: native::deferred,
) -> Result<()> {
  let (this, _, _) = native::get_cb_info(env, info)?;
  let combine = unsafe { &mut *(native::unwrap(env, this)? as *mut Combine) };

  let result = Box::leak(Box::new(CombineFinishContext {
    combine,
    async_work: null_mut(),
    deferred,
    result: Err(Cow::Borrowed("")),
  }));
  result.async_work = native::create_async_work(
    env,
    "Render GIF using written RGBA frames",
    combine_finish_execute,
    combine_finish_complete,
    addr_of_mut!(*result).cast(),
  )
  .map_err(|_| Cow::Borrowed("Failed to create an async_work"))?;
  native::queue_async_work(env, result.async_work)
    .map_err(|_| Cow::Borrowed("Failed to queue the async_work"))?;
  Ok(())
}

pub(crate) extern "C" fn combine_finish(
  env: native::env,
  info: native::callback_info,
) -> native::value {
  let (promise, deferred) = native::unwrap_throw!(env, native::create_promise(env));

  if !deferred.is_null() {
    if let Err(v) = combine_finish_promise(env, info, deferred) {
      native::unwrap_throw!(
        env,
        native::create_string_utf8(env, &v).and_then(|v| native::reject_deferred(env, deferred, v))
      );
    }
  }

  promise
}

pub(crate) fn combine_struct(env: native::env) -> Result<native::value> {
  static mut COMBINE_STRUCT: native::value = null_mut();
  if unsafe { COMBINE_STRUCT }.is_null() {
    let combine_struct = native::define_class(
      env,
      "Combine",
      combine_constructor,
      null_mut(),
      &[
        native::property_descriptor {
          utf8name: null_mut(),
          name: native::create_string_utf8(env, "alloc")?,
          method: Some(combine_alloc),
          getter: None,
          setter: None,
          value: null_mut(),
          attributes: native::property_attribute::Enumerable,
          data: null_mut(),
        },
        native::property_descriptor {
          utf8name: null_mut(),
          name: native::create_string_utf8(env, "write")?,
          method: Some(combine_write),
          getter: None,
          setter: None,
          value: null_mut(),
          attributes: native::property_attribute::Enumerable,
          data: null_mut(),
        },
        native::property_descriptor {
          utf8name: null_mut(),
          name: native::create_string_utf8(env, "finish")?,
          method: Some(combine_finish),
          getter: None,
          setter: None,
          value: null_mut(),
          attributes: native::property_attribute::Enumerable,
          data: null_mut(),
        },
      ],
    )?;
    unsafe { COMBINE_STRUCT = native::create_reference(env, combine_struct)? };
  }
  Ok(unsafe { COMBINE_STRUCT })
}

struct CombineContext {
  options: CombineOptions,
  async_work: native::async_work,
  deferred: native::deferred,
  result: Result<&'static mut Combine>,
}

extern "C" fn combine_execute(_env: native::env, data: *mut c_void) {
  let ctx = unsafe { &mut *data.cast::<CombineContext>() };
  ctx.result = combine_native(&ctx.options).map(Box::new).map(Box::leak);
}

extern "C" fn combine_complete(env: native::env, _status: native::status, data: *mut c_void) {
  let ctx = unsafe { &mut *data.cast::<CombineContext>() };
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
    dealloc(data.cast(), Layout::new::<CombineContext>());
  };
}

fn combine_promise(
  env: native::env,
  info: native::callback_info,
  deferred: native::deferred,
) -> Result<()> {
  let (_, args, _) = native::get_cb_info(env, info)?;
  let mut args = args.into_iter();
  let options = args
    .next()
    .ok_or(Cow::Borrowed("Function call expects exactly one argument"))
    .and_then(|v| CombineOptions::from_napi(env, v))?;

  let result = Box::leak(Box::new(CombineContext {
    options,
    async_work: null_mut(),
    deferred,
    result: Err(Cow::Borrowed("")),
  }));
  result.async_work = native::create_async_work(
    env,
    "Combine RGBA frame buffers into a GIF file",
    combine_execute,
    combine_complete,
    addr_of_mut!(*result).cast(),
  )
  .map_err(|_| Cow::Borrowed("Failed to create an async_work"))?;
  native::queue_async_work(env, result.async_work)
    .map_err(|_| Cow::Borrowed("Failed to queue the async_work"))?;
  Ok(())
}

pub(crate) extern "C" fn combine(env: native::env, info: native::callback_info) -> native::value {
  let (promise, deferred) = native::unwrap_throw!(env, native::create_promise(env));

  if !deferred.is_null() {
    if let Err(v) = combine_promise(env, info, deferred) {
      native::unwrap_throw!(
        env,
        native::create_string_utf8(env, &v).and_then(|v| native::reject_deferred(env, deferred, v))
      );
    }
  }

  promise
}
