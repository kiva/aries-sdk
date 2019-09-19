use ffi_support::{FfiStr, ExternError};
use std::os::raw::c_char;

pub type OutString = *const c_char;
pub type Handle = u32;

#[no_mangle]
pub extern fn aries_create_wallet(config: FfiStr<'_>,
                                  credentials: FfiStr<'_>,
                                  err: &mut ExternError) -> i32 {
    //Convert to rust types
    let config = config.as_str();
    let credentials = credentials.as_str();
    *err = ExternError::success();

    // TODO: call the correct rust function
    // `err` should be set here and not in the corresponding function

    handle_result!(err)
}

#[no_mangle]
pub extern fn aries_open_wallet(config: FfiStr<'_>,
                                credentials: FfiStr<'_>,
                                err: &mut ExternError) -> Handle {
    //Convert to rust types
    let config = config.as_str();
    let credentials = credentials.as_str();
    *err = ExternError::success();

    // TODO: call the correct rust function
    // `err` should be set here and not in the corresponding function

    handle_result!(err)
}

#[no_mangle]
pub extern fn aries_export_wallet(wallet_handle: u32,
                                  export_config: FfiStr<'_>,
                                  err: &mut ExternError) -> i32 {
    //Convert to rust types
    let export_config = export_config.as_str();
    *err = ExternError::success();

    // TODO: call the correct rust function
    // `err` should be set here and not in the corresponding function

    handle_result!(err)
}

#[no_mangle]
pub extern fn aries_import_wallet(config: FfiStr<'_>,
                                  credentials: FfiStr<'_>,
                                  import_config: FfiStr<'_>,
                                  err: &mut ExternError) -> i32 {
    //Convert to rust types
    let config = config.as_str();
    let credentials = credentials.as_str();
    let import_config = import_config.as_str();
    *err = ExternError::success();

    // TODO: call the correct rust function
    // `err` should be set here and not in the corresponding function

    handle_result!(err)
}

#[no_mangle]
pub extern fn aries_close_wallet(wallet_handle: Handle) -> i32 {
    // TODO: call the correct rust function
    0
}

#[no_mangle]
pub extern fn aries_delete_wallet(config: FfiStr<'_>,
                                  credentials: FfiStr<'_>,
                                  err: &mut ExternError) -> i32 {
    //Convert to rust types
    let config = config.as_str();
    let credentials = credentials.as_str();
    *err = ExternError::success();

    // TODO: call the correct rust function
    // `err` should be set here and not in the corresponding function

    handle_result!(err)
}
