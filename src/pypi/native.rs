#![allow(non_camel_case_types)]

use std::{
  ffi::{c_char, c_int, c_uint, c_ulong, c_void},
  ptr,
};

pub(crate) const API_VERSION: c_int = 1013;

pub(crate) type ssize_t = isize;
pub(crate) type hash_t = ssize_t;
// pub(crate) type MODINIT_FUNC = Option<extern "C" fn() -> *mut Object>;

pub(crate) type destructor = Option<extern "C" fn(*mut Object)>;
pub(crate) type getattrfunc = Option<extern "C" fn(*mut Object, *mut c_char) -> *mut Object>;
pub(crate) type setattrfunc = Option<extern "C" fn(*mut Object, *mut c_char, *mut Object) -> c_int>;
pub(crate) type unaryfunc = Option<extern "C" fn(*mut Object) -> *mut Object>;
pub(crate) type sendfunc = Option<
  extern "C" fn(iter: *mut Object, value: *mut Object, result: *mut *mut Object) -> SendResult,
>;
pub(crate) type reprfunc = Option<extern "C" fn(*mut Object) -> *mut Object>;
pub(crate) type binaryfunc = Option<extern "C" fn(*mut Object, *mut Object) -> *mut Object>;
pub(crate) type ternaryfunc =
  Option<extern "C" fn(*mut Object, *mut Object, *mut Object) -> *mut Object>;
pub(crate) type inquiry = Option<extern "C" fn(*mut Object) -> c_int>;
pub(crate) type hashfunc = Option<extern "C" fn(*mut Object) -> hash_t>;
pub(crate) type getattrofunc = Option<extern "C" fn(*mut Object, *mut Object) -> *mut Object>;
pub(crate) type setattrofunc =
  Option<extern "C" fn(*mut Object, *mut Object, *mut Object) -> c_int>;
pub(crate) type visitproc = Option<extern "C" fn(*mut Object, *mut c_void) -> c_int>;
pub(crate) type traverseproc = Option<extern "C" fn(*mut Object, visitproc, *mut c_void) -> c_int>;
pub(crate) type richcmpfunc = Option<extern "C" fn(*mut Object, *mut Object, c_int) -> *mut Object>;
pub(crate) type getiterfunc = Option<extern "C" fn(*mut Object) -> *mut Object>;
pub(crate) type iternextfunc = Option<extern "C" fn(*mut Object) -> *mut Object>;
pub(crate) type descrgetfunc =
  Option<extern "C" fn(*mut Object, *mut Object, *mut Object) -> *mut Object>;
pub(crate) type descrsetfunc =
  Option<extern "C" fn(*mut Object, *mut Object, *mut Object) -> c_int>;
pub(crate) type initproc = Option<extern "C" fn(*mut Object, *mut Object, *mut Object) -> c_int>;
pub(crate) type newfunc =
  Option<extern "C" fn(*mut TypeObject, *mut Object, *mut Object) -> *mut Object>;
pub(crate) type allocfunc = Option<extern "C" fn(*mut TypeObject, ssize_t) -> *mut Object>;
pub(crate) type freefunc = Option<extern "C" fn(*mut c_void) -> *mut c_void>;
pub(crate) type vectorcallfunc = Option<
  extern "C" fn(
    callable: *mut Object,
    args: *const *mut Object,
    nargsf: isize,
    kwnames: *mut Object,
  ) -> *mut Object,
>;
pub(crate) type lenfunc = Option<extern "C" fn(*mut Object) -> ssize_t>;
pub(crate) type ssizeargfunc = Option<extern "C" fn(*mut Object, ssize_t) -> *mut Object>;
pub(crate) type ssizeobjargproc = Option<extern "C" fn(*mut Object, ssize_t, *mut Object) -> c_int>;
pub(crate) type objobjproc = Option<extern "C" fn(*mut Object, *mut Object) -> c_int>;
pub(crate) type objobjargproc =
  Option<extern "C" fn(*mut Object, *mut Object, *mut Object) -> c_int>;
pub(crate) type getbufferproc = Option<extern "C" fn(*mut Object, *mut buffer, c_int) -> c_int>;
pub(crate) type releasebufferproc = Option<extern "C" fn(*mut Object, *mut buffer) -> c_int>;
pub(crate) type CFunction = Option<extern "C" fn(s: *mut Object, args: *mut Object) -> *mut Object>;
pub(crate) type getter = Option<extern "C" fn(*mut Object, *mut c_void) -> *mut Object>;
pub(crate) type setter = Option<extern "C" fn(*mut Object, *mut Object, *mut c_void) -> c_int>;

#[repr(C)]
pub(crate) struct ModuleDef_Base {
  pub(crate) ob_base: Object,
  pub(crate) m_init: Option<extern "C" fn() -> *mut Object>,
  pub(crate) m_index: ssize_t,
  pub(crate) m_copy: *mut Object,
}

#[repr(C)]
pub(crate) struct ModuleDef_Slot {
  pub(crate) slot: c_int,
  pub(crate) value: *mut c_void,
}

#[repr(C)]
pub(crate) struct ModuleDef {
  pub(crate) m_base: ModuleDef_Base,
  pub(crate) m_name: *const c_char,
  pub(crate) m_doc: *const c_char,
  pub(crate) m_size: ssize_t,
  pub(crate) m_methods: *mut MethodDef,
  pub(crate) m_slots: *mut ModuleDef_Slot,
  pub(crate) m_traverse: traverseproc,
  pub(crate) m_clear: inquiry,
  pub(crate) m_free: freefunc,
}

// unsafe impl Send for ModuleDef {}
unsafe impl Sync for ModuleDef {}

impl ModuleDef {
  pub(crate) const fn new(
    name: *const c_char,
    doc: *const c_char,
    methods: *const MethodDef,
  ) -> Self {
    Self {
      m_base: ModuleDef_Base {
        ob_base: Object {
          ob_refcnt: 1,
          ob_type: ptr::null_mut(),
        },
        m_init: None,
        m_index: 0,
        m_copy: ptr::null_mut(),
      },
      m_name: name,
      m_doc: doc,
      m_size: -1,
      m_methods: methods.cast_mut(),
      m_slots: ptr::null_mut(),
      m_traverse: None,
      m_clear: None,
      m_free: None,
    }
  }
}

#[repr(C)]
pub(crate) struct GetSetDef {
  pub(crate) name: *const c_char,
  pub(crate) get: getter,
  pub(crate) set: setter,
  pub(crate) doc: *const c_char,
  pub(crate) closure: *mut c_void,
}

#[repr(C)]
pub(crate) struct MemberDef {
  pub(crate) name: *const c_char,
  pub(crate) r#type: c_int,
  pub(crate) offset: ssize_t,
  pub(crate) flags: c_int,
  pub(crate) doc: *const c_char,
}

#[repr(C)]
pub(crate) struct buffer {
  pub(crate) buf: *mut c_void,
  /// owned reference
  pub(crate) obj: *mut Object,
  pub(crate) len: ssize_t,
  /// This is ssize_t so it can be
  /// pointed to by strides in simple case.
  pub(crate) itemsize: ssize_t,
  pub(crate) readonly: c_int,
  pub(crate) ndim: c_int,
  pub(crate) format: *mut c_char,
  pub(crate) shape: *mut ssize_t,
  pub(crate) strides: *mut ssize_t,
  pub(crate) suboffsets: *mut ssize_t,
  pub(crate) internal: *mut c_void,
}

#[repr(C)]
pub(crate) struct MethodDef {
  /// The name of the built-in function/method
  pub(crate) ml_name: *const c_char,
  /// The C function that implements it
  pub(crate) ml_meth: CFunction,
  /// Combination of METH_xxx flags, which mostly
  /// describe the args expected by the C func
  pub(crate) ml_flags: c_int,
  /// The __doc__ attribute, or NULL
  pub(crate) ml_doc: *const c_char,
}

#[derive(Clone, Copy)]
pub(crate) struct MethodDefFlags(c_int);

#[allow(dead_code)]
impl MethodDefFlags {
  pub(crate) const EMPTY: Self = Self(0);
  pub(crate) const VARARGS: Self = Self(0x0001);
  pub(crate) const KEYWORDS: Self = Self(0x0002);
  /// [`NOARGS`] and [`O`] must not be combined with the flags above.
  ///
  /// [`NOARGS`]: Self::NOARGS
  /// [`O`]: Self::O
  pub(crate) const NOARGS: Self = Self(0x0004);
  pub(crate) const O: Self = Self(0x0008);

  pub(crate) const NOARGS_O: Self = Self(Self::NOARGS.0 | Self::O.0);
  pub(crate) const VARARGS_KEYWORDS: Self = Self(Self::VARARGS.0 | Self::KEYWORDS.0);
}

impl MethodDef {
  pub(crate) const NULL: Self = Self::new(ptr::null(), None, MethodDefFlags::EMPTY, ptr::null());

  pub(crate) const fn new(
    name: *const c_char,
    meth: CFunction,
    flags: MethodDefFlags,
    doc: *const c_char,
  ) -> Self {
    Self {
      ml_name: name,
      ml_meth: meth,
      ml_flags: flags.0,
      ml_doc: doc,
    }
  }
}

unsafe impl Sync for MethodDef {}

#[repr(C)]
pub(crate) struct BufferProcs {
  pub(crate) bf_getbuffer: getbufferproc,
  pub(crate) bf_releasebuffer: releasebufferproc,
}

#[repr(C)]
pub(crate) struct MappingMethods {
  pub(crate) mp_length: lenfunc,
  pub(crate) mp_subscript: binaryfunc,
  pub(crate) mp_ass_subscript: objobjargproc,
}

#[repr(C)]
pub(crate) struct SequenceMethods {
  pub(crate) sq_length: lenfunc,
  pub(crate) sq_concat: binaryfunc,
  pub(crate) sq_repeat: ssizeargfunc,
  pub(crate) sq_item: ssizeargfunc,
  pub(crate) was_sq_slice: *mut c_void,
  pub(crate) sq_ass_item: ssizeobjargproc,
  pub(crate) was_sq_ass_slice: *mut c_void,
  pub(crate) sq_contains: objobjproc,
  pub(crate) sq_inplace_concat: binaryfunc,
  pub(crate) sq_inplace_repeat: ssizeargfunc,
}

#[repr(C)]
#[allow(dead_code)]
pub(crate) enum SendResult {
  Return = 0,
  Error = -1,
  Next = 1,
}

#[repr(C)]
pub(crate) struct VarObject {
  pub(crate) ob_base: Object,
  /// Number of items in variable part
  pub(crate) ob_size: ssize_t,
}

#[repr(C)]
pub(crate) struct AsyncMethods {
  pub(crate) am_await: unaryfunc,
  pub(crate) am_aiter: unaryfunc,
  pub(crate) am_anext: unaryfunc,
  pub(crate) am_send: sendfunc,
}

#[repr(C)]
pub(crate) struct NumberMethods {
  /// Number implementations must check *both*
  /// arguments for proper type and implement the necessary conversions
  /// in the slot functions themselves.
  pub(crate) nb_add: binaryfunc,
  pub(crate) nb_subtract: binaryfunc,
  pub(crate) nb_multiply: binaryfunc,
  pub(crate) nb_remainder: binaryfunc,
  pub(crate) nb_divmod: binaryfunc,
  pub(crate) nb_power: ternaryfunc,
  pub(crate) nb_negative: unaryfunc,
  pub(crate) nb_positive: unaryfunc,
  pub(crate) nb_absolute: unaryfunc,
  pub(crate) nb_bool: inquiry,
  pub(crate) nb_invert: unaryfunc,
  pub(crate) nb_lshift: binaryfunc,
  pub(crate) nb_rshift: binaryfunc,
  pub(crate) nb_and: binaryfunc,
  pub(crate) nb_xor: binaryfunc,
  pub(crate) nb_or: binaryfunc,
  pub(crate) nb_int: unaryfunc,
  /// the slot formerly known as nb_long
  pub(crate) nb_reserved: *mut c_void,
  pub(crate) nb_float: unaryfunc,
  pub(crate) nb_inplace_add: binaryfunc,
  pub(crate) nb_inplace_subtract: binaryfunc,
  pub(crate) nb_inplace_multiply: binaryfunc,
  pub(crate) nb_inplace_remainder: binaryfunc,
  pub(crate) nb_inplace_power: ternaryfunc,
  pub(crate) nb_inplace_lshift: binaryfunc,
  pub(crate) nb_inplace_rshift: binaryfunc,
  pub(crate) nb_inplace_and: binaryfunc,
  pub(crate) nb_inplace_xor: binaryfunc,
  pub(crate) nb_inplace_or: binaryfunc,
  pub(crate) nb_floor_divide: binaryfunc,
  pub(crate) nb_true_divide: binaryfunc,
  pub(crate) nb_inplace_floor_divide: binaryfunc,
  pub(crate) nb_inplace_true_divide: binaryfunc,
  pub(crate) nb_index: unaryfunc,
  pub(crate) nb_matrix_multiply: binaryfunc,
  pub(crate) nb_inplace_matrix_multiply: binaryfunc,
}

#[repr(C)]
pub(crate) struct TypeObject {
  pub(crate) ob_base: VarObject,
  /// For printing, in format "<module>.<name>"
  pub(crate) tp_name: *const c_char,
  /// For allocation
  pub(crate) tp_basicsize: ssize_t,
  pub(crate) tp_itemsize: ssize_t,
  /// Methods to implement standard operations
  pub(crate) tp_dealloc: destructor,
  pub(crate) tp_vectorcall_offset: ssize_t,
  pub(crate) tp_getattr: getattrfunc,
  pub(crate) tp_setattr: setattrfunc,
  /// formerly known as tp_compare (Python 2)
  /// or tp_reserved (Python 3)
  pub(crate) tp_as_async: *mut AsyncMethods,
  pub(crate) tp_repr: reprfunc,
  /// Method suites for standard classes
  pub(crate) tp_as_number: *mut NumberMethods,
  pub(crate) tp_as_sequence: *mut SequenceMethods,
  pub(crate) tp_as_mapping: *mut MappingMethods,
  /// More standard operations (here for binary compatibility)
  pub(crate) tp_hash: hashfunc,
  pub(crate) tp_call: ternaryfunc,
  pub(crate) tp_str: reprfunc,
  pub(crate) tp_getattro: getattrofunc,
  pub(crate) tp_setattro: setattrofunc,
  /// Functions to access object as input/output buffer
  pub(crate) tp_as_buffer: *mut BufferProcs,
  /// Flags to define presence of optional/expanded features
  pub(crate) tp_flags: c_ulong,
  /// Documentation string
  pub(crate) tp_doc: *const c_char,
  /// Assigned meaning in release 2.0
  /// call function for all accessible objects
  pub(crate) tp_traverse: traverseproc,
  /// delete references to contained objects
  pub(crate) tp_clear: inquiry,
  /// Assigned meaning in release 2.1
  /// rich comparisons
  pub(crate) tp_richcompare: richcmpfunc,
  /// weak reference enabler
  pub(crate) tp_weaklistoffset: ssize_t,
  /// Iterators
  pub(crate) tp_iter: getiterfunc,
  pub(crate) tp_iternext: iternextfunc,
  /// Attribute descriptor and subclassing stuff
  pub(crate) tp_methods: *mut MethodDef,
  pub(crate) tp_members: *mut MemberDef,
  pub(crate) tp_getset: *mut GetSetDef,
  /// Strong reference on a heap type, borrowed reference on a static type
  pub(crate) tp_base: *mut TypeObject,
  pub(crate) tp_dict: *mut Object,
  pub(crate) tp_descr_get: descrgetfunc,
  pub(crate) tp_descr_set: descrsetfunc,
  pub(crate) tp_dictoffset: ssize_t,
  pub(crate) tp_init: initproc,
  pub(crate) tp_alloc: allocfunc,
  pub(crate) tp_new: newfunc,
  /// Low-level free-memory routine
  pub(crate) tp_free: freefunc,
  /// For PyObject_IS_GC
  pub(crate) tp_is_gc: inquiry,
  pub(crate) tp_bases: *mut Object,
  /// method resolution order
  pub(crate) tp_mro: *mut Object,
  pub(crate) tp_cache: *mut Object,
  pub(crate) tp_subclasses: *mut Object,
  pub(crate) tp_weaklist: *mut Object,
  pub(crate) tp_del: destructor,
  /// Type attribute cache version tag. Added in version 2.6
  pub(crate) tp_version_tag: c_uint,
  pub(crate) tp_finalize: destructor,
  pub(crate) tp_vectorcall: vectorcallfunc,
}

#[repr(C)]
pub(crate) struct Object {
  pub(crate) ob_refcnt: ssize_t,
  pub(crate) ob_type: *mut TypeObject,
}

#[cfg_attr(target_os = "windows", link(name = "lib\\python3"))]
extern "C" {
  pub(crate) static PyExc_Exception: *mut Object;
  pub(crate) fn PyModule_Create2(module: *mut ModuleDef, apiver: c_int) -> *mut Object;
  pub(crate) fn PyArg_ParseTuple(tuple: *mut Object, signature: *const c_char, ...) -> c_int;
  pub(crate) fn PyLong_FromLong(v: std::ffi::c_long) -> *mut Object;
  pub(crate) fn PyErr_SetString(exception: *mut Object, string: *const c_char);
  pub(crate) fn PyByteArray_FromStringAndSize(string: *const c_char, size: ssize_t) -> *mut Object;
}

macro_rules! parse_tuple {
  ($expr:expr, $sig:expr, $($ident:ident: $ty:ty),+$(,)?) => {
    $(let mut $ident: $ty = unsafe { std::mem::zeroed() };)+
    unsafe { $crate::pypi::native::PyArg_ParseTuple($expr, $sig.as_ptr().cast(), $(&mut $ident),*) };
  };
}

pub(crate) use parse_tuple;

pub(crate) struct Module;

impl Module {
  pub(crate) fn create(module: *mut ModuleDef) -> *mut Object {
    unsafe { PyModule_Create2(module, API_VERSION) }
  }
}

pub(crate) trait Topy {
  fn topy(self) -> *mut Object;
}

impl Topy for std::ffi::c_long {
  fn topy(self) -> *mut Object {
    unsafe { PyLong_FromLong(self) }
  }
}

impl Topy for &[u8] {
  fn topy(self) -> *mut Object {
    unsafe {
      #[allow(clippy::cast_possible_wrap)]
      PyByteArray_FromStringAndSize(self.as_ptr().cast(), self.len() as ssize_t)
    }
  }
}

pub(crate) struct Exception;

impl Exception {
  pub(crate) fn set_string(exception: *mut Object, string: *const c_char) {
    unsafe { PyErr_SetString(exception, string) }
  }
}
