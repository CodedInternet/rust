mod time;
mod objects;
mod memory;
mod mutex;
mod poll;

pub use time::*;
pub use objects::*;
pub use memory::*;
pub use mutex::*;
pub use poll::*;

extern "C" {
    pub fn printk(fmt: *const c_types::c_char, ...);
}

#[doc = " @cond INTERNAL_HIDDEN"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_sem {
    pub wait_q: _wait_q_t,
    pub count: c_types::c_uint,
    pub limit: c_types::c_uint,
    pub _obj_track_next: *mut k_sem,
}
impl Default for k_sem {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
