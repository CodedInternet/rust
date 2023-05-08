#[doc = " Mutex Structure\n @ingroup mutex_apis"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_mutex {
    #[doc = " Mutex wait queue"]
    pub wait_q: _wait_q_t,
    #[doc = " Mutex owner"]
    pub owner: *mut k_thread,
    #[doc = " Current lock count"]
    pub lock_count: u32,
    #[doc = " Original thread priority"]
    pub owner_orig_prio: c_types::c_int,
    pub _obj_track_next: *mut k_mutex,
}
impl Default for k_mutex {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
