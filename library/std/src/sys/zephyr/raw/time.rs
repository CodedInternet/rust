pub const Z_HZ_ticks: u32 = 100;

pub type k_ticks_t = i64;
#[doc = " @brief Kernel timeout type\n\n Timeout arguments presented to kernel APIs are stored in this\n opaque type, which is capable of representing times in various\n formats and units.  It should be constructed from application data\n using one of the macros defined for this purpose (e.g. `K_MSEC()`,\n `K_TIMEOUT_ABS_TICKS()`, etc...), or be one of the two constants\n K_NO_WAIT or K_FOREVER.  Applications should not inspect the\n internal data once constructed.  Timeout values may be compared for\n equality with the `K_TIMEOUT_EQ()` macro."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct k_timeout_t {
    pub ticks: k_ticks_t,
}

// Recreate what the K_FOREVER macro does
pub const K_FOREVER: k_timeout_t = k_timeout_t {
    ticks: -1 as k_ticks_t,
};

pub const K_NO_WAIT: k_timeout_t = k_timeout_t { ticks: 0 };
