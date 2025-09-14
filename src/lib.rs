use core::ffi::c_int;

pub fn get_number() -> i32 {
    42
}

#[unsafe(no_mangle)]
pub extern "C" fn gizmo_get_number() -> c_int {
    get_number() as c_int
}
