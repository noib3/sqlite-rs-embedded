extern crate alloc;

use core::ffi::{c_char, c_int, c_uchar, c_void};
use core::ptr;

pub use crate::bindings::{
    sqlite3, sqlite3_api_routines as api_routines, sqlite3_context as context,
    sqlite3_index_info as index_info, sqlite3_module as module, sqlite3_stmt as stmt,
    sqlite3_uint64 as uint64, sqlite3_value as value, sqlite_int64 as int64, SQLITE_UTF8 as UTF8,
};

mod aliased {
    #[cfg(feature = "omit_load_extension")]
    pub use crate::bindings::{
        sqlite3_bind_blob as bind_blob, sqlite3_bind_double as bind_double,
        sqlite3_bind_int as bind_int, sqlite3_bind_int64 as bind_int64,
        sqlite3_bind_null as bind_null, sqlite3_bind_parameter_count as bind_parameter_count,
        sqlite3_bind_parameter_index as bind_parameter_index,
        sqlite3_bind_parameter_name as bind_parameter_name, sqlite3_bind_pointer as bind_pointer,
        sqlite3_bind_text as bind_text, sqlite3_bind_value as bind_value,
        sqlite3_bind_zeroblob as bind_zeroblob, sqlite3_close as close,
        sqlite3_column_blob as column_blob, sqlite3_column_bytes as column_bytes,
        sqlite3_column_count as column_count, sqlite3_column_decltype as column_decltype,
        sqlite3_column_double as column_double, sqlite3_column_int as column_int,
        sqlite3_column_int64 as column_int64, sqlite3_column_name as column_name,
        sqlite3_column_origin_name as column_origin_name,
        sqlite3_column_table_name as column_table_name, sqlite3_column_text as column_text,
        sqlite3_column_type as column_type, sqlite3_column_value as column_value,
        sqlite3_commit_hook as commit_hook, sqlite3_context_db_handle as context_db_handle,
        sqlite3_create_function_v2 as create_function_v2,
        sqlite3_create_module_v2 as create_module_v2, sqlite3_declare_vtab as declare_vtab,
        sqlite3_exec as exec, sqlite3_finalize as finalize, sqlite3_free as free,
        sqlite3_get_auxdata as get_auxdata, sqlite3_malloc as malloc, sqlite3_malloc64 as malloc64,
        sqlite3_open as open, sqlite3_prepare_v2 as prepare_v2, sqlite3_reset as reset,
        sqlite3_result_blob as result_blob, sqlite3_result_double as result_double,
        sqlite3_result_error as result_error, sqlite3_result_error_code as result_error_code,
        sqlite3_result_int as result_int, sqlite3_result_int64 as result_int64,
        sqlite3_result_null as result_null, sqlite3_result_pointer as result_pointer,
        sqlite3_result_subtype as result_subtype, sqlite3_result_text as result_text,
        sqlite3_result_value as result_value, sqlite3_set_auxdata as set_auxdata,
        sqlite3_step as step, sqlite3_value_blob as value_blob, sqlite3_value_bytes as value_bytes,
        sqlite3_value_double as value_double, sqlite3_value_int as value_int,
        sqlite3_value_int64 as value_int64, sqlite3_value_pointer as value_pointer,
        sqlite3_value_subtype as value_subtype, sqlite3_value_text as value_text,
        sqlite3_value_type as value_type, sqlite3_vtab_collation as vtab_collation,
        sqlite3_vtab_config as vtab_config, sqlite3_vtab_distinct as vtab_distinct,
        sqlite3_vtab_nochange as vtab_nochange, sqlite3_vtab_on_conflict as vtab_on_conflict,
    };
}

pub enum Destructor {
    TRANSIENT,
    STATIC,
    CUSTOM(xDestroy),
}

#[macro_export]
macro_rules! strlit {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[cfg(feature = "omit_load_extension")]
macro_rules! invoke_sqlite {
    ($name:ident, $($arg:expr),*) => {
      aliased::$name($($arg),*)
    };
}

#[cfg(not(feature = "omit_load_extension"))]
macro_rules! invoke_sqlite {
  ($name:ident, $($arg:expr),*) => {
    ((*SQLITE3_API).$name.unwrap())($($arg),*)
  }
}

pub extern "C" fn droprust(ptr: *mut c_void) {
    unsafe {
        ptr.drop_in_place();
    }
}

#[macro_export]
macro_rules! args {
    ($argc:expr, $argv:expr) => {
        unsafe { slice::from_raw_parts($argv, $argc as usize) }
    };
}

static mut SQLITE3_API: *mut api_routines = ptr::null_mut();

pub fn EXTENSION_INIT2(api: *mut api_routines) {
    unsafe {
        SQLITE3_API = api;
    }
}

pub fn bind_int(stmt: *mut stmt, c: c_int, i: c_int) -> i32 {
    unsafe { invoke_sqlite!(bind_int, stmt, c, i) }
}

pub fn bind_int64(stmt: *mut stmt, c: c_int, i: int64) -> i32 {
    unsafe { invoke_sqlite!(bind_int64, stmt, c, i) }
}

pub fn bind_text(stmt: *mut stmt, c: c_int, s: *const c_char, n: c_int) -> i32 {
    unsafe { invoke_sqlite!(bind_text, stmt, c, s, n, None) }
}

pub fn bind_pointer(db: *mut stmt, i: c_int, p: *mut c_void, t: *const c_char) -> i32 {
    unsafe { invoke_sqlite!(bind_pointer, db, i, p, t, None) }
}

pub fn bind_value(stmt: *mut stmt, c: c_int, v: *mut value) -> i32 {
    unsafe { invoke_sqlite!(bind_value, stmt, c, v) }
}

pub fn close(db: *mut sqlite3) -> i32 {
    unsafe { invoke_sqlite!(close, db) }
}

pub type xCommitHook = unsafe extern "C" fn(*mut c_void) -> i32;
pub fn commit_hook(
    db: *mut sqlite3,
    callback: Option<xCommitHook>,
    user_data: *mut c_void,
) -> Option<xCommitHook> {
    unsafe {
        invoke_sqlite!(commit_hook, db, callback, user_data)
            .as_ref()
            .map(|p| core::mem::transmute(p))
    }
}

// pub fn mprintf(format: *const i8, ...) -> *mut c_char {
//     unsafe { ((*SQLITE3_API).mprintf.expect(EXPECT_MESSAGE))(format, args) }
// }

pub fn column_type(stmt: *mut stmt, c: c_int) -> i32 {
    unsafe { invoke_sqlite!(column_type, stmt, c) }
}

pub fn column_count(stmt: *mut stmt) -> i32 {
    unsafe { invoke_sqlite!(column_count, stmt) }
}

pub fn column_text(stmt: *mut stmt, c: c_int) -> *const c_uchar {
    unsafe { invoke_sqlite!(column_text, stmt, c) }
}

pub fn column_blob(stmt: *mut stmt, c: c_int) -> *const c_void {
    unsafe { invoke_sqlite!(column_blob, stmt, c) }
}

pub fn column_bytes(stmt: *mut stmt, c: c_int) -> i32 {
    unsafe { invoke_sqlite!(column_bytes, stmt, c) }
}

pub fn column_value(stmt: *mut stmt, c: c_int) -> *mut value {
    unsafe { invoke_sqlite!(column_value, stmt, c) }
}

pub fn column_double(stmt: *mut stmt, c: c_int) -> f64 {
    unsafe { invoke_sqlite!(column_double, stmt, c) }
}

pub fn column_int(stmt: *mut stmt, c: c_int) -> i32 {
    unsafe { invoke_sqlite!(column_int, stmt, c) }
}

pub fn column_int64(stmt: *mut stmt, c: c_int) -> int64 {
    unsafe { invoke_sqlite!(column_int64, stmt, c) }
}

pub fn column_name(stmt: *mut stmt, c: c_int) -> *const c_char {
    unsafe { invoke_sqlite!(column_name, stmt, c) }
}

pub fn context_db_handle(ctx: *mut context) -> *mut sqlite3 {
    unsafe { invoke_sqlite!(context_db_handle, ctx) }
}

pub type xFunc = unsafe extern "C" fn(*mut context, i32, *mut *mut value);
pub type xStep = unsafe extern "C" fn(*mut context, i32, *mut *mut value);
pub type xFinal = unsafe extern "C" fn(*mut context);
pub type xDestroy = unsafe extern "C" fn(*mut c_void);
pub fn create_function_v2(
    db: *mut sqlite3,
    s: *const c_char,
    argc: i32,
    flags: u32,
    p_app: *mut c_void,
    x_func: Option<xFunc>,
    x_step: Option<xStep>,
    x_final: Option<xFinal>,
    destroy: Option<xDestroy>,
) -> c_int {
    unsafe {
        invoke_sqlite!(
            create_function_v2,
            db,
            s,
            argc,
            i32::try_from(flags).expect("Invalid flags"),
            p_app,
            x_func,
            x_step,
            x_final,
            destroy
        )
    }
}

pub fn create_module_v2(
    db: *mut sqlite3,
    s: *const i8,
    module: *const module,
    p_app: *mut c_void,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
) -> i32 {
    unsafe { invoke_sqlite!(create_module_v2, db, s, module, p_app, destroy) }
}

pub fn declare_vtab(db: *mut sqlite3, s: *const i8) -> i32 {
    unsafe { invoke_sqlite!(declare_vtab, db, s) }
}

pub fn exec(db: *mut sqlite3, s: *const i8) -> i32 {
    unsafe { invoke_sqlite!(exec, db, s, None, ptr::null_mut(), ptr::null_mut()) }
}

pub fn finalize(stmt: *mut stmt) -> c_int {
    unsafe { invoke_sqlite!(finalize, stmt) }
}

pub fn free(ptr: *mut u8) {
    unsafe { invoke_sqlite!(free, ptr as *mut c_void) }
}

pub fn get_auxdata(context: *mut context, n: c_int) -> *mut c_void {
    unsafe { invoke_sqlite!(get_auxdata, context, n) }
}

pub fn malloc(size: usize) -> *mut u8 {
    unsafe {
        if usize::BITS == 64 {
            invoke_sqlite!(malloc64, size as uint64) as *mut u8
        } else {
            invoke_sqlite!(malloc, size as i32) as *mut u8
        }
    }
}

pub fn open(filename: *const c_char, db: *mut *mut sqlite3) -> i32 {
    unsafe { invoke_sqlite!(open, filename, db) }
}

pub fn prepare_v2(
    db: *mut sqlite3,
    sql: *const c_char,
    n: i32,
    stmt: *mut *mut stmt,
    leftover: *mut *const c_char,
) -> i32 {
    unsafe { invoke_sqlite!(prepare_v2, db, sql, n, stmt, leftover) }
}

pub fn result_int(context: *mut context, v: c_int) {
    unsafe { invoke_sqlite!(result_int, context, v) }
}

pub fn result_blob(context: *mut context, b: *const u8, n: i32, d: Destructor) {
    unsafe {
        invoke_sqlite!(
            result_blob,
            context,
            b as *const c_void,
            n,
            match d {
                Destructor::TRANSIENT => Some(core::mem::transmute(-1_isize)),
                Destructor::STATIC => None,
                Destructor::CUSTOM(f) => Some(f),
            }
        )
    }
}

pub fn result_int64(context: *mut context, v: int64) {
    unsafe { invoke_sqlite!(result_int64, context, v) }
}

pub fn result_double(context: *mut context, f: f64) {
    unsafe { invoke_sqlite!(result_double, context, f) }
}

pub fn result_null(context: *mut context) {
    unsafe { invoke_sqlite!(result_null, context) }
}
pub fn result_pointer(
    context: *mut context,
    pointer: *mut c_void,
    name: *mut c_char,
    destructor: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    unsafe { invoke_sqlite!(result_pointer, context, pointer, name, destructor) }
}

pub fn result_error(context: *mut context, text: &str) {
    unsafe {
        invoke_sqlite!(
            result_error,
            context,
            text.as_ptr() as *mut c_char,
            text.len() as i32
        )
    }
}

pub fn result_error_code(context: *mut context, code: i32) {
    unsafe { invoke_sqlite!(result_error_code, context, code) }
}

// d is our destructor function.
// -- https://dev.to/kgrech/7-ways-to-pass-a-string-between-rust-and-c-4ieb
pub fn result_text(context: *mut context, s: *const i8, n: i32, d: Destructor) {
    unsafe {
        invoke_sqlite!(
            result_text,
            context,
            s,
            n,
            match d {
                Destructor::TRANSIENT => Some(core::mem::transmute(-1_isize)),
                Destructor::STATIC => None,
                Destructor::CUSTOM(f) => Some(f),
            }
        )
    }
}

pub fn result_subtype(context: *mut context, subtype: u32) {
    unsafe { invoke_sqlite!(result_subtype, context, subtype) }
}

pub fn set_auxdata(
    context: *mut context,
    n: c_int,
    p: *mut c_void,
    d: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    unsafe { invoke_sqlite!(set_auxdata, context, n, p, d) }
}

pub fn reset(stmt: *mut stmt) -> c_int {
    unsafe { invoke_sqlite!(reset, stmt) }
}

pub fn step(stmt: *mut stmt) -> c_int {
    unsafe { invoke_sqlite!(step, stmt) }
}

pub fn value_text<'a>(arg1: *mut value) -> &'a str {
    unsafe {
        let len = value_bytes(arg1);
        let bytes = invoke_sqlite!(value_text, arg1);
        let slice = core::slice::from_raw_parts(bytes as *const u8, len as usize);
        core::str::from_utf8_unchecked(slice)
    }
}

pub fn value_type(value: *mut value) -> i32 {
    unsafe { invoke_sqlite!(value_type, value) }
}

pub fn value_bytes(arg1: *mut value) -> i32 {
    unsafe { invoke_sqlite!(value_bytes, arg1) }
}

pub fn value_blob<'a>(value: *mut value) -> &'a [u8] {
    unsafe {
        let n = value_bytes(value);
        let b = invoke_sqlite!(value_blob, value);
        core::slice::from_raw_parts(b.cast::<u8>(), n as usize)
    }
}

pub fn value_int(arg1: *mut value) -> i32 {
    unsafe { invoke_sqlite!(value_int, arg1) }
}

pub fn value_int64(arg1: *mut value) -> int64 {
    unsafe { invoke_sqlite!(value_int64, arg1) }
}

pub fn value_double(arg1: *mut value) -> f64 {
    unsafe { invoke_sqlite!(value_double, arg1) }
}

pub fn value_pointer(arg1: *mut value, p: *mut c_char) -> *mut c_void {
    unsafe { invoke_sqlite!(value_pointer, arg1, p) }
}

pub fn vtab_distinct(index_info: *mut index_info) -> i32 {
    unsafe { invoke_sqlite!(vtab_distinct, index_info) }
}
