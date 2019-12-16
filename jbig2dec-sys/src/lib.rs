mod binding;

pub use binding::*;

pub unsafe extern "C" fn jbig2_ctx_new(
    allocator: *mut Jbig2Allocator,
    options: Jbig2Options,
    global_ctx: *mut Jbig2GlobalCtx,
    error_callback: Jbig2ErrorCallback,
    error_callback_data: *mut ::std::os::raw::c_void,
) -> *mut Jbig2Ctx {
    jbig2_ctx_new_imp(
        allocator,
        options,
        global_ctx,
        error_callback,
        error_callback_data,
        0,
        17,
    )
}
