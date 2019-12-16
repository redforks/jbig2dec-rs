#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Jbig2Severity {
    JBIG2_SEVERITY_DEBUG = 0,
    JBIG2_SEVERITY_INFO = 1,
    JBIG2_SEVERITY_WARNING = 2,
    JBIG2_SEVERITY_FATAL = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Jbig2Options {
    JBIG2_OPTIONS_EMBEDDED = 1,
}
pub type Jbig2Allocator = _Jbig2Allocator;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Jbig2Ctx {
    _unused: [u8; 0],
}
pub type Jbig2Ctx = _Jbig2Ctx;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Jbig2GlobalCtx {
    _unused: [u8; 0],
}
pub type Jbig2GlobalCtx = _Jbig2GlobalCtx;
pub type Jbig2Image = _Jbig2Image;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Jbig2Image {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub data: *mut u8,
    pub refcount: ::std::os::raw::c_int,
}
pub type Jbig2ErrorCallback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut ::std::os::raw::c_void,
        msg: *const ::std::os::raw::c_char,
        severity: Jbig2Severity,
        seg_idx: i32,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Jbig2Allocator {
    pub alloc: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut Jbig2Allocator,
            size: usize,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub free: ::std::option::Option<
        unsafe extern "C" fn(allocator: *mut Jbig2Allocator, p: *mut ::std::os::raw::c_void),
    >,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: *mut Jbig2Allocator,
            p: *mut ::std::os::raw::c_void,
            size: usize,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
extern "C" {
    pub fn jbig2_ctx_new_imp(
        allocator: *mut Jbig2Allocator,
        options: Jbig2Options,
        global_ctx: *mut Jbig2GlobalCtx,
        error_callback: Jbig2ErrorCallback,
        error_callback_data: *mut ::std::os::raw::c_void,
        jbig2_version_major: ::std::os::raw::c_int,
        jbig2_version_minor: ::std::os::raw::c_int,
    ) -> *mut Jbig2Ctx;
}
extern "C" {
    pub fn jbig2_ctx_free(ctx: *mut Jbig2Ctx) -> *mut Jbig2Allocator;
}
extern "C" {
    pub fn jbig2_make_global_ctx(ctx: *mut Jbig2Ctx) -> *mut Jbig2GlobalCtx;
}
extern "C" {
    pub fn jbig2_global_ctx_free(global_ctx: *mut Jbig2GlobalCtx) -> *mut Jbig2Allocator;
}
extern "C" {
    pub fn jbig2_data_in(
        ctx: *mut Jbig2Ctx,
        data: *const ::std::os::raw::c_uchar,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jbig2_page_out(ctx: *mut Jbig2Ctx) -> *mut Jbig2Image;
}
extern "C" {
    pub fn jbig2_release_page(ctx: *mut Jbig2Ctx, image: *mut Jbig2Image);
}
extern "C" {
    pub fn jbig2_complete_page(ctx: *mut Jbig2Ctx) -> ::std::os::raw::c_int;
}
